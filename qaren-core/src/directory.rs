//! Recursive directory traversal and semantic diffing.
//!
//! This module provides functions to traverse two directories and compare
//! corresponding key-value configuration files.

use crate::diff::semantic_diff;
use crate::parser::detect_delimiter;
use crate::types::{ConfigFile, DiffOptions, DirDiffResult, FileDiffStatus, ParseOptions};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Options for parsing files during a directory traversal.
#[derive(Debug, Clone)]
pub struct DirParseOptions {
    /// Delimiter to use. If `None`, it will be auto-detected per file.
    pub delimiter: Option<char>,
    /// Whether to strip surrounding quotes from keys and values
    pub strip_quotes: bool,
    /// Comment prefixes to ignore
    pub comment_prefixes: Vec<String>,
    /// Compare values case-insensitively
    pub ignore_case: bool,
}

impl Default for DirParseOptions {
    fn default() -> Self {
        Self {
            delimiter: None,
            strip_quotes: false,
            comment_prefixes: vec!["#".to_string(), "//".to_string()],
            ignore_case: false,
        }
    }
}

/// Recursively collect all files in a directory, returning paths relative to the base directory.
pub fn collect_files_recursive(
    dir: &Path,
    base: &Path,
    files: &mut HashSet<PathBuf>,
    warnings: &mut Vec<String>,
) {
    match std::fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_symlink() {
                                continue;
                            }
                        }
                        let path = entry.path();
                        if path.is_dir() {
                            collect_files_recursive(&path, base, files, warnings);
                        } else if path.is_file() {
                            if let Ok(rel_path) = path.strip_prefix(base) {
                                files.insert(rel_path.to_path_buf());
                            }
                        }
                    }
                    Err(e) => warnings.push(format!("Error reading entry in {}: {}", dir.display(), e)),
                }
            }
        }
        Err(e) => warnings.push(format!("Error reading directory {}: {}", dir.display(), e)),
    }
}
const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024; // 50MB

fn parse_file_with_opts(path: &Path, dir_opts: &DirParseOptions) -> Result<ConfigFile, String> {
    if let Ok(metadata) = std::fs::metadata(path) {
        if metadata.len() > MAX_FILE_SIZE {
            return Err(format!("File size exceeds {}MB limit", MAX_FILE_SIZE / 1_000_000));
        }
    }
    
    let bytes = std::fs::read(path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    let content = match String::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => {
            // Not valid UTF-8, treat as an empty config so it gets categorized as NotAKvFile
            return Ok(ConfigFile {
                pairs: std::collections::HashMap::new(),
                file_path: path.to_path_buf(),
                warnings: vec![],
            });
        }
    };

    let delimiter = dir_opts.delimiter.unwrap_or_else(|| detect_delimiter(&content));

    let opts = ParseOptions {
        delimiter,
        strip_quotes: dir_opts.strip_quotes,
        comment_prefixes: dir_opts.comment_prefixes.clone(),
        ignore_case: dir_opts.ignore_case,
    };

    #[allow(unused_mut)]
    let mut config = crate::parser::parse_content(&content, path, &opts)
        .map_err(|e| format!("Parse error in {}: {}", path.display(), e))?;

    #[cfg(unix)]
    if let Ok(metadata) = std::fs::metadata(path) {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        if mode & 0o077 != 0 {
            config.warnings.push(crate::types::ParseWarning {
                key: None,
                message: format!("Insecure file permissions ({:o})", mode & 0o777),
            });
        }
    }

    Ok(config)
}

/// Recursively compares two directories of configuration files.
pub fn semantic_diff_dir(
    dir1: &Path,
    dir2: &Path,
    opts1: &DirParseOptions,
    opts2: &DirParseOptions,
    diff_opts: &DiffOptions,
) -> DirDiffResult {
    let mut files1 = HashSet::new();
    let mut files2 = HashSet::new();
    let mut traversal_warnings = Vec::new();

    collect_files_recursive(dir1, dir1, &mut files1, &mut traversal_warnings);
    collect_files_recursive(dir2, dir2, &mut files2, &mut traversal_warnings);

    let all_files: HashSet<&PathBuf> = files1.union(&files2).collect();
    let mut result_files = HashMap::new();

    for rel_path in all_files {
        let in_dir1 = files1.contains(rel_path);
        let in_dir2 = files2.contains(rel_path);

        let path1 = dir1.join(rel_path);
        let path2 = dir2.join(rel_path);

        if in_dir1 && in_dir2 {
            match (parse_file_with_opts(&path1, opts1), parse_file_with_opts(&path2, opts2)) {
                (Ok(c1), Ok(c2)) => {
                    if c1.pairs.is_empty() && c2.pairs.is_empty() {
                        result_files.insert(rel_path.clone(), FileDiffStatus::NotAKvFile(rel_path.clone()));
                    } else {
                        let diff = semantic_diff(&c1, &c2, diff_opts);
                        if diff.is_identical() {
                            result_files.insert(rel_path.clone(), FileDiffStatus::Identical);
                        } else {
                            result_files.insert(rel_path.clone(), FileDiffStatus::Modified(diff));
                        }
                    }
                }
                (Err(e), _) | (_, Err(e)) => {
                    result_files.insert(rel_path.clone(), FileDiffStatus::Error(e));
                }
            }
        } else if in_dir1 {
            match parse_file_with_opts(&path1, opts1) {
                Ok(c1) => {
                    if c1.pairs.is_empty() {
                        result_files.insert(rel_path.clone(), FileDiffStatus::NotAKvFile(rel_path.clone()));
                    } else {
                        result_files.insert(rel_path.clone(), FileDiffStatus::OrphanInSource(c1));
                    }
                }
                Err(e) => {
                    result_files.insert(rel_path.clone(), FileDiffStatus::Error(e));
                }
            }
        } else if in_dir2 {
            match parse_file_with_opts(&path2, opts2) {
                Ok(c2) => {
                    if c2.pairs.is_empty() {
                        result_files.insert(rel_path.clone(), FileDiffStatus::NotAKvFile(rel_path.clone()));
                    } else {
                        result_files.insert(rel_path.clone(), FileDiffStatus::OrphanInTarget(c2));
                    }
                }
                Err(e) => {
                    result_files.insert(rel_path.clone(), FileDiffStatus::Error(e));
                }
            }
        }
    }

    DirDiffResult {
        files: result_files,
        traversal_warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_recursive_directory_diff() {
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();

        // Setup Source Directory
        let app1_dir1 = dir1.path().join("app1");
        std::fs::create_dir(&app1_dir1).unwrap();
        let mut f = File::create(app1_dir1.join(".env")).unwrap();
        writeln!(f, "KEY1=value1\nKEY2=value2").unwrap();

        let mut f = File::create(dir1.path().join("orphan1.env")).unwrap();
        writeln!(f, "ORPHAN=1").unwrap();

        // Setup Target Directory
        let app1_dir2 = dir2.path().join("app1");
        std::fs::create_dir(&app1_dir2).unwrap();
        let mut f = File::create(app1_dir2.join(".env")).unwrap();
        writeln!(f, "KEY1=value1\nKEY2=modified").unwrap();

        let mut f = File::create(dir2.path().join("orphan2.env")).unwrap();
        writeln!(f, "ORPHAN=2").unwrap();

        let opts1 = DirParseOptions::default();
        let opts2 = DirParseOptions::default();
        let diff_opts = DiffOptions::default();

        let result = semantic_diff_dir(dir1.path(), dir2.path(), &opts1, &opts2, &diff_opts);

        assert_eq!(result.files.len(), 3); // app1/.env, orphan1.env, orphan2.env
        
        let app1_env = PathBuf::from("app1").join(".env");
        match result.files.get(&app1_env).unwrap() {
            FileDiffStatus::Modified(diff) => {
                assert_eq!(diff.modified.len(), 1);
                assert_eq!(diff.modified[0].key, "KEY2");
            }
            _ => panic!("Expected app1/.env to be Modified"),
        }

        let orphan1 = PathBuf::from("orphan1.env");
        assert!(matches!(result.files.get(&orphan1).unwrap(), FileDiffStatus::OrphanInSource(_)));

        let orphan2 = PathBuf::from("orphan2.env");
        assert!(matches!(result.files.get(&orphan2).unwrap(), FileDiffStatus::OrphanInTarget(_)));
    }
}
