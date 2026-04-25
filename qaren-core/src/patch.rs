//! Patch file generator for missing configuration keys.
//!
//! Generates patch files containing key-value pairs that are missing
//! from one configuration file relative to another. Supports three
//! directions:
//!
//! - **SourceToTarget**: keys in file1 missing from file2 (default)
//! - **TargetToSource**: keys in file2 missing from file1
//! - **Bidirectional**: generates both files with suffixed names

use crate::error::{QarenError, QarenResult};
use crate::types::{DiffResult, DirDiffResult, FileDiffStatus, KvPair, ParseOptions, PatchDirection};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Generate patch file(s) based on the specified direction.
///
/// Returns a vector of paths to the created files. For `SourceToTarget`
/// and `TargetToSource`, this is a single path. For `Bidirectional`,
/// this contains two paths.
pub fn generate_patch(
    diff_result: &DiffResult,
    output_path: &Path,
    options1: &ParseOptions,
    options2: &ParseOptions,
    direction: PatchDirection,
    mask_secrets: bool,
) -> QarenResult<Vec<PathBuf>> {
    let mut created_files = Vec::new();
    match direction {
        PatchDirection::SourceToTarget => {
            if !diff_result.missing_in_file2.is_empty() {
                generate_single_patch(&diff_result.missing_in_file2, output_path, options2, mask_secrets)?;
                created_files.push(output_path.to_path_buf());
            }
        }
        PatchDirection::TargetToSource => {
            if !diff_result.missing_in_file1.is_empty() {
                generate_single_patch(&diff_result.missing_in_file1, output_path, options1, mask_secrets)?;
                created_files.push(output_path.to_path_buf());
            }
        }
        PatchDirection::Bidirectional => {
            // Generate source-to-target patch
            if !diff_result.missing_in_file2.is_empty() {
                let s2t_path = append_suffix(output_path, "source-to-target");
                generate_single_patch(&diff_result.missing_in_file2, &s2t_path, options2, mask_secrets)?;
                created_files.push(s2t_path);
            }

            // Generate target-to-source patch
            if !diff_result.missing_in_file1.is_empty() {
                let t2s_path = append_suffix(output_path, "target-to-source");
                generate_single_patch(&diff_result.missing_in_file1, &t2s_path, options1, mask_secrets)?;
                created_files.push(t2s_path);
            }
        }
    }
    Ok(created_files)
}

/// Generate patch files for an entire directory based on the recursive diff result.
/// 
/// Recreates the directory structure in `output_dir`.
pub fn generate_recursive_patch(
    dir_diff: &DirDiffResult,
    output_dir: &Path,
    options1: &ParseOptions,
    options2: &ParseOptions,
    direction: PatchDirection,
    mask_secrets: bool,
) -> QarenResult<Vec<PathBuf>> {
    let mut created_files = Vec::new();

    if direction == PatchDirection::Bidirectional {
        let s2t_dir = output_dir.join("source-to-target");
        let t2s_dir = output_dir.join("target-to-source");

        let mut s2t_files = generate_recursive_patch(dir_diff, &s2t_dir, options1, options2, PatchDirection::SourceToTarget, mask_secrets)?;
        let mut t2s_files = generate_recursive_patch(dir_diff, &t2s_dir, options1, options2, PatchDirection::TargetToSource, mask_secrets)?;

        created_files.append(&mut s2t_files);
        created_files.append(&mut t2s_files);
        return Ok(created_files);
    }

    for (rel_path, status) in &dir_diff.files {
        let file_out = output_dir.join(rel_path);

        match direction {
            PatchDirection::SourceToTarget => match status {
                FileDiffStatus::Modified(diff) if !diff.missing_in_file2.is_empty() => {
                    if let Some(p) = file_out.parent() {
                        std::fs::create_dir_all(p).map_err(|e| QarenError::FileWrite { path: p.to_path_buf(), source: e })?;
                    }
                    generate_single_patch(&diff.missing_in_file2, &file_out, options2, mask_secrets)?;
                    created_files.push(file_out);
                }
                FileDiffStatus::OrphanInSource(config) if !config.pairs.is_empty() => {
                    if let Some(p) = file_out.parent() {
                        std::fs::create_dir_all(p).map_err(|e| QarenError::FileWrite { path: p.to_path_buf(), source: e })?;
                    }
                    let mut pairs: Vec<_> = config.pairs.iter().map(|(k, (v, line))| KvPair {
                        key: k.clone(),
                        value: v.clone(),
                        line_number: *line,
                    }).collect();
                    // Sort by line number to keep original file order
                    pairs.sort_by_key(|p| p.line_number);
                    generate_single_patch(&pairs, &file_out, options2, mask_secrets)?;
                    created_files.push(file_out);
                }
                _ => {}
            },
            PatchDirection::TargetToSource => match status {
                FileDiffStatus::Modified(diff) if !diff.missing_in_file1.is_empty() => {
                    if let Some(p) = file_out.parent() {
                        std::fs::create_dir_all(p).map_err(|e| QarenError::FileWrite { path: p.to_path_buf(), source: e })?;
                    }
                    generate_single_patch(&diff.missing_in_file1, &file_out, options1, mask_secrets)?;
                    created_files.push(file_out);
                }
                FileDiffStatus::OrphanInTarget(config) if !config.pairs.is_empty() => {
                    if let Some(p) = file_out.parent() {
                        std::fs::create_dir_all(p).map_err(|e| QarenError::FileWrite { path: p.to_path_buf(), source: e })?;
                    }
                    let mut pairs: Vec<_> = config.pairs.iter().map(|(k, (v, line))| KvPair {
                        key: k.clone(),
                        value: v.clone(),
                        line_number: *line,
                    }).collect();
                    // Sort by line number to keep original file order
                    pairs.sort_by_key(|p| p.line_number);
                    generate_single_patch(&pairs, &file_out, options1, mask_secrets)?;
                    created_files.push(file_out);
                }
                _ => {}
            },
            _ => {} // Handled at the top
        }
    }

    Ok(created_files)
}

/// Generate a single patch file containing the specified missing key-value pairs.
fn generate_single_patch(
    missing_pairs: &[KvPair],
    output_path: &Path,
    options: &ParseOptions,
    mask_secrets: bool,
) -> QarenResult<()> {
    let mut file = File::create(output_path).map_err(|e| QarenError::FileWrite {
        path: output_path.to_path_buf(),
        source: e,
    })?;

    for pair in missing_pairs {
        let value = if mask_secrets && crate::masking::should_mask(&pair.key) {
            "***MASKED***"
        } else {
            &pair.value
        };
        let line = format!("{}{}{}", pair.key, options.delimiter, value);
        writeln!(file, "{}", line).map_err(|e| QarenError::FileWrite {
            path: output_path.to_path_buf(),
            source: e,
        })?;
    }

    Ok(())
}

/// Append a suffix to a file path before the extension.
///
/// # Examples
///
/// - `"output.env"` + `"source-to-target"` → `"output.source-to-target.env"`
/// - `"output"` + `"source-to-target"` → `"output.source-to-target.env"`
fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("env");

    let new_filename = format!("{}.{}.{}", stem, suffix, extension);

    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.join(new_filename),
        _ => PathBuf::from(new_filename),
    }
}

// ─────────────────────────────────────────────────────────────────────
// Unit Tests (Task 6.5)
// ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Helper: build a DiffResult with specified missing/modified keys.
    fn make_diff(
        missing_in_file2: Vec<(&str, &str)>,
        missing_in_file1: Vec<(&str, &str)>,
    ) -> DiffResult {
        DiffResult {
            missing_in_file2: missing_in_file2
                .into_iter()
                .enumerate()
                .map(|(i, (k, v))| KvPair {
                    key: k.to_string(),
                    value: v.to_string(),
                    line_number: i + 1,
                })
                .collect(),
            missing_in_file1: missing_in_file1
                .into_iter()
                .enumerate()
                .map(|(i, (k, v))| KvPair {
                    key: k.to_string(),
                    value: v.to_string(),
                    line_number: i + 1,
                })
                .collect(),
            modified: vec![],
            identical: vec![],
        }
    }

    #[test]
    fn test_empty_patch() {
        let tmp = TempDir::new().expect("temp dir");
        let output = tmp.path().join("empty.env");
        let diff = make_diff(vec![], vec![]);
        let opts = ParseOptions::default();

        let paths = generate_patch(&diff, &output, &opts, &opts, PatchDirection::SourceToTarget, false)
            .expect("should succeed");

        assert_eq!(paths.len(), 0);
    }

    #[test]
    fn test_patch_with_multiple_missing_keys() {
        let tmp = TempDir::new().expect("temp dir");
        let output = tmp.path().join("missing.env");
        let diff = make_diff(
            vec![("API_KEY", "abc123"), ("DB_HOST", "localhost"), ("PORT", "3000")],
            vec![],
        );
        let opts = ParseOptions::default();

        let paths = generate_patch(&diff, &output, &opts, &opts, PatchDirection::SourceToTarget, false)
            .expect("should succeed");

        assert_eq!(paths.len(), 1);
        let content = std::fs::read_to_string(&paths[0]).expect("read");

        assert!(content.contains("API_KEY=abc123"));
        assert!(content.contains("DB_HOST=localhost"));
        assert!(content.contains("PORT=3000"));
    }

    #[test]
    fn test_source_to_target_direction() {
        let tmp = TempDir::new().expect("temp dir");
        let output = tmp.path().join("patch.env");
        let diff = make_diff(
            vec![("FROM_SOURCE", "val1")],
            vec![("FROM_TARGET", "val2")],
        );
        let opts = ParseOptions::default();

        let paths = generate_patch(&diff, &output, &opts, &opts, PatchDirection::SourceToTarget, false)
            .expect("should succeed");

        let content = std::fs::read_to_string(&paths[0]).expect("read");
        assert!(content.contains("FROM_SOURCE=val1"));
        assert!(!content.contains("FROM_TARGET"));
    }

    #[test]
    fn test_target_to_source_direction() {
        let tmp = TempDir::new().expect("temp dir");
        let output = tmp.path().join("patch.env");
        let diff = make_diff(
            vec![("FROM_SOURCE", "val1")],
            vec![("FROM_TARGET", "val2")],
        );
        let opts = ParseOptions::default();

        let paths = generate_patch(&diff, &output, &opts, &opts, PatchDirection::TargetToSource, false)
            .expect("should succeed");

        let content = std::fs::read_to_string(&paths[0]).expect("read");
        assert!(content.contains("FROM_TARGET=val2"));
        assert!(!content.contains("FROM_SOURCE"));
    }

    #[test]
    fn test_bidirectional_patch_generation() {
        let tmp = TempDir::new().expect("temp dir");
        let output = tmp.path().join("sync.env");
        let diff = make_diff(
            vec![("ONLY_IN_A", "val_a")],
            vec![("ONLY_IN_B", "val_b")],
        );
        let opts = ParseOptions::default();

        let paths = generate_patch(&diff, &output, &opts, &opts, PatchDirection::Bidirectional, false)
            .expect("should succeed");

        assert_eq!(paths.len(), 2);

        // Verify filenames
        let filenames: Vec<String> = paths
            .iter()
            .map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default())
            .collect();
        assert!(filenames.iter().any(|f| f.contains("source-to-target")));
        assert!(filenames.iter().any(|f| f.contains("target-to-source")));

        // Verify contents
        let s2t_path = paths.iter().find(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().contains("source-to-target"))
                .unwrap_or(false)
        }).expect("s2t path");
        let t2s_path = paths.iter().find(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().contains("target-to-source"))
                .unwrap_or(false)
        }).expect("t2s path");

        let s2t_content = std::fs::read_to_string(s2t_path).expect("read s2t");
        let t2s_content = std::fs::read_to_string(t2s_path).expect("read t2s");

        assert!(s2t_content.contains("ONLY_IN_A=val_a"));
        assert!(t2s_content.contains("ONLY_IN_B=val_b"));
    }

    #[test]
    fn test_correct_file_naming() {
        assert_eq!(
            append_suffix(Path::new("output.env"), "source-to-target"),
            PathBuf::from("output.source-to-target.env")
        );
        assert_eq!(
            append_suffix(Path::new("output.env"), "target-to-source"),
            PathBuf::from("output.target-to-source.env")
        );
    }

    #[test]
    fn test_correct_file_naming_with_directory() {
        let path = Path::new("/tmp/patches/sync.env");
        let result = append_suffix(path, "source-to-target");
        assert_eq!(
            result,
            PathBuf::from("/tmp/patches/sync.source-to-target.env")
        );
    }

    #[test]
    fn test_correct_file_naming_no_extension() {
        let result = append_suffix(Path::new("output"), "source-to-target");
        // When no extension, default to "env"
        assert!(result.to_string_lossy().contains("source-to-target"));
    }

    #[test]
    fn test_patch_file_creation_failure() {
        let bad_path = PathBuf::from("/nonexistent/directory/should/fail/patch.env");
        let diff = make_diff(vec![("KEY", "val")], vec![]);
        let opts = ParseOptions::default();

        let result = generate_patch(&diff, &bad_path, &opts, &opts, PatchDirection::SourceToTarget, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_delimiter_preservation_in_patch() {
        let tmp = TempDir::new().expect("temp dir");
        let output = tmp.path().join("patch.env");
        let diff = make_diff(vec![("KEY", "value")], vec![]);
        let opts = ParseOptions {
            delimiter: ':',
            ..ParseOptions::default()
        };

        let paths = generate_patch(&diff, &output, &opts, &opts, PatchDirection::SourceToTarget, false)
            .expect("should succeed");

        let content = std::fs::read_to_string(&paths[0]).expect("read");
        assert!(content.contains("KEY:value"));
    }



    #[test]
    fn test_generate_recursive_patch() {
        use crate::types::{DirDiffResult, FileDiffStatus, ConfigFile};
        use std::collections::HashMap;

        let tmp = TempDir::new().expect("temp dir");
        let output_dir = tmp.path().join("patches");

        let mut files = HashMap::new();
        
        // Setup a mock DirDiffResult
        let mock_diff = make_diff(
            vec![("MISSING_IN_TARGET", "value1")],
            vec![]
        );
        files.insert(PathBuf::from("app1/config.env"), FileDiffStatus::Modified(mock_diff));

        // Mock orphan in source
        let mut orphan_pairs = HashMap::new();
        orphan_pairs.insert("ORPHAN_KEY".to_string(), ("orphan_value".to_string(), 1));
        let orphan_config = ConfigFile {
            pairs: orphan_pairs,
            file_path: PathBuf::from("orphan.env"),
            warnings: vec![],
        };
        files.insert(PathBuf::from("app2/orphan.env"), FileDiffStatus::OrphanInSource(orphan_config));

        let dir_diff = DirDiffResult {
            files,
            traversal_warnings: vec![],
        };

        let opts = ParseOptions::default();

        let paths = generate_recursive_patch(
            &dir_diff,
            &output_dir,
            &opts,
            &opts,
            PatchDirection::SourceToTarget, false,
        ).expect("should generate patches");

        assert_eq!(paths.len(), 2);

        // Verify the directory structure was created
        let app1_patch = output_dir.join("app1/config.env");
        let app2_patch = output_dir.join("app2/orphan.env");

        assert!(app1_patch.exists());
        assert!(app2_patch.exists());

        let content1 = std::fs::read_to_string(&app1_patch).unwrap();
        assert!(content1.contains("MISSING_IN_TARGET=value1"));

        let content2 = std::fs::read_to_string(&app2_patch).unwrap();
        assert!(content2.contains("ORPHAN_KEY=orphan_value"));
    }
}

// ─────────────────────────────────────────────────────────────────────
// Property-based tests (Tasks 6.2, 6.3, 6.4)
// ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::diff::semantic_diff;
    use crate::parser;
    use crate::types::{ConfigFile, DiffOptions};
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use std::collections::{HashMap, HashSet};
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Check if a character is safe for use in test inputs.
    fn is_safe_char(c: char) -> bool {
        !c.is_control() && c.is_ascii() && c != '\0'
    }

    /// Sanitize key
    fn sanitize_key(s: &str) -> Option<String> {
        let clean: String = s
            .chars()
            .filter(|c| {
                is_safe_char(*c)
                    && *c != '='
                    && *c != '#'
                    && *c != '/'
                    && *c != '"'
                    && *c != '\''
            })
            .collect();
        let clean = clean.trim().to_string();
        if clean.is_empty() || clean.starts_with("export ") || clean == "export" {
            return None;
        }
        Some(clean)
    }

    /// Sanitize value
    /// Sanitize value
    fn sanitize_value(s: &str) -> String {
        let clean: String = s.chars()
            .filter(|c| is_safe_char(*c) && *c != '#' && *c != '/')
            .collect();
            
        clean.trim().to_string()
    }

    /// Build a ConfigFile from raw pairs (deduplicates)
    fn build_config(raw_pairs: &[(String, String)]) -> ConfigFile {
        let mut map = HashMap::new();
        for (i, (k, v)) in raw_pairs.iter().enumerate() {
            let key = match sanitize_key(k) {
                Some(k) => k,
                None => continue,
            };
            let value = sanitize_value(v);
            map.insert(key, (value, i + 1));
        }
        ConfigFile {
            pairs: map,
            file_path: PathBuf::from("test.env"), warnings: vec![],
        }
    }

    // ── Property 5: Patch Completeness and Correctness ──────────────

    #[quickcheck]
    fn prop_patch_completeness(
        pairs_a: Vec<(String, String)>,
        pairs_b: Vec<(String, String)>,
    ) -> TestResult {
        let file_a = build_config(&pairs_a);
        let file_b = build_config(&pairs_b);

        if file_a.pairs.is_empty() && file_b.pairs.is_empty() {
            return TestResult::discard();
        }

        let diff = semantic_diff(&file_a, &file_b, &DiffOptions::default());
        let opts = ParseOptions::default();
        let tmp = match TempDir::new() {
            Ok(t) => t,
            Err(_) => return TestResult::discard(),
        };

        // Source-to-target: should contain exactly keys in A missing from B
        let s2t_path = tmp.path().join("s2t.env");
        if let Ok(paths) =
            generate_patch(&diff, &s2t_path, &opts, &opts, PatchDirection::SourceToTarget, false)
        {
            let content = paths.first().map(|p| std::fs::read_to_string(p).unwrap_or_default()).unwrap_or_default();
            let expected_keys: HashSet<_> =
                diff.missing_in_file2.iter().map(|p| &p.key).collect();

            // Every expected key should appear in the patch
            for key in &expected_keys {
                if !content.contains(key.as_str()) {
                    return TestResult::failed();
                }
            }
        }

        // Target-to-source: should contain exactly keys in B missing from A
        let t2s_path = tmp.path().join("t2s.env");
        if let Ok(paths) =
            generate_patch(&diff, &t2s_path, &opts, &opts, PatchDirection::TargetToSource, false)
        {
            let content = paths.first().map(|p| std::fs::read_to_string(p).unwrap_or_default()).unwrap_or_default();
            let expected_keys: HashSet<_> =
                diff.missing_in_file1.iter().map(|p| &p.key).collect();

            for key in &expected_keys {
                if !content.contains(key.as_str()) {
                    return TestResult::failed();
                }
            }
        }

        TestResult::passed()
    }

    // ── Property 6: Patch Formatting Preservation (Round-Trip) ──────

    #[quickcheck]
    fn prop_patch_round_trip(
        pairs_a: Vec<(String, String)>,
        pairs_b: Vec<(String, String)>,
    ) -> TestResult {
        let file_a = build_config(&pairs_a);
        let file_b = build_config(&pairs_b);

        let diff = semantic_diff(&file_a, &file_b, &DiffOptions::default());

        if diff.missing_in_file2.is_empty() {
            return TestResult::discard();
        }

        let opts = ParseOptions::default();
        let tmp = match TempDir::new() {
            Ok(t) => t,
            Err(_) => return TestResult::discard(),
        };

        // Generate patch
        let patch_path = tmp.path().join("roundtrip.env");
        if let Ok(paths) =
            generate_patch(&diff, &patch_path, &opts, &opts, PatchDirection::SourceToTarget, false)
        {
            // Parse the generated patch file
            let parsed = match parser::parse_file(&paths[0], &opts) {
                Ok(p) => p,
                Err(_) => return TestResult::discard(),
            };

            // Verify parsed values match originals from file_a
            for missing_pair in &diff.missing_in_file2 {
                match parsed.pairs.get(&missing_pair.key) {
                    Some((parsed_value, _)) => {
                        if parsed_value != &missing_pair.value {
                            return TestResult::failed();
                        }
                    }
                    None => return TestResult::failed(),
                }
            }
        }

        TestResult::passed()
    }

    // ── Property 10: Bidirectional Patch Symmetry ────────────────────

    #[quickcheck]
    fn prop_bidirectional_patch_symmetry(
        pairs_a: Vec<(String, String)>,
        pairs_b: Vec<(String, String)>,
    ) -> TestResult {
        let file_a = build_config(&pairs_a);
        let file_b = build_config(&pairs_b);

        let diff = semantic_diff(&file_a, &file_b, &DiffOptions::default());

        if diff.missing_in_file2.is_empty() && diff.missing_in_file1.is_empty() {
            return TestResult::discard();
        }

        let opts = ParseOptions::default();
        let tmp = match TempDir::new() {
            Ok(t) => t,
            Err(_) => return TestResult::discard(),
        };

        let output_path = tmp.path().join("bidir.env");
        let paths = match generate_patch(&diff, &output_path, &opts, &opts, PatchDirection::Bidirectional, false)
        {
            Ok(p) => p,
            Err(_) => return TestResult::discard(),
        };

        let expected_len = (if diff.missing_in_file2.is_empty() { 0 } else { 1 })
            + (if diff.missing_in_file1.is_empty() { 0 } else { 1 });
        assert_eq!(paths.len(), expected_len);

        if paths.len() < 2 {
            return TestResult::passed();
        }

        // Parse both patch files
        let s2t_parsed = match parser::parse_file(&paths[0], &opts) {
            Ok(p) => p,
            Err(_) => return TestResult::discard(),
        };
        let t2s_parsed = match parser::parse_file(&paths[1], &opts) {
            Ok(p) => p,
            Err(_) => return TestResult::discard(),
        };

        let s2t_keys: HashSet<_> = s2t_parsed.pairs.keys().collect();
        let t2s_keys: HashSet<_> = t2s_parsed.pairs.keys().collect();

        // Verify no overlap between the two patches
        let overlap: HashSet<_> = s2t_keys.intersection(&t2s_keys).collect();
        if !overlap.is_empty() {
            return TestResult::failed();
        }

        // Verify s2t contains exactly the keys missing in file2
        let expected_s2t: HashSet<_> =
            diff.missing_in_file2.iter().map(|p| &p.key).collect();
        if s2t_keys != expected_s2t {
            return TestResult::failed();
        }

        // Verify t2s contains exactly the keys missing in file1
        let expected_t2s: HashSet<_> =
            diff.missing_in_file1.iter().map(|p| &p.key).collect();
        if t2s_keys != expected_t2s {
            return TestResult::failed();
        }

        TestResult::passed()
    }
}

