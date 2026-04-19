//! Core data structures for the Qaren configuration comparison tool.
//!
//! This module defines all the types used across the parser, diff engine,
//! and patch generator. These types are UI-agnostic and form the public API
//! of `qaren-core`.

use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a single key-value pair from a configuration file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvPair {
    /// The key (left side of delimiter)
    pub key: String,
    /// The value (right side of delimiter)
    pub value: String,
    /// Original line number in source file (1-indexed, for error reporting)
    pub line_number: usize,
}

/// Represents a parsed configuration file as a HashMap for O(1) lookups.
#[derive(Debug, Clone)]
pub struct ConfigFile {
    /// Key-value pairs stored in a HashMap for O(1) lookup.
    /// Key: configuration key, Value: (value, line_number)
    pub pairs: HashMap<String, (String, usize)>,
    /// Original file path (for error messages)
    pub file_path: PathBuf,
}

/// Parsing configuration options.
///
/// Controls how the parser interprets configuration file content:
/// delimiter selection, quote handling, and comment detection.
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// Delimiter character between key and value (default: `'='`)
    pub delimiter: char,
    /// Whether to strip surrounding quotes from keys and values
    pub strip_quotes: bool,
    /// Comment prefixes to ignore (default: `["#", "//"]`)
    pub comment_prefixes: Vec<String>,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            delimiter: '=',
            strip_quotes: false,
            comment_prefixes: vec!["#".to_string(), "//".to_string()],
        }
    }
}

/// Represents the result of a semantic comparison between two configuration files.
#[derive(Debug, Clone)]
pub struct DiffResult {
    /// Keys present in file1 but missing in file2
    pub missing_in_file2: Vec<KvPair>,
    /// Keys present in file2 but missing in file1
    pub missing_in_file1: Vec<KvPair>,
    /// Keys present in both files with different values
    pub modified: Vec<ModifiedPair>,
    /// Keys present in both files with identical values
    pub identical: Vec<String>,
}

impl DiffResult {
    /// Returns `true` if the two files are semantically identical.
    pub fn is_identical(&self) -> bool {
        self.missing_in_file1.is_empty()
            && self.missing_in_file2.is_empty()
            && self.modified.is_empty()
    }

    /// Returns the total count of differences (missing + modified).
    pub fn difference_count(&self) -> usize {
        self.missing_in_file1.len()
            + self.missing_in_file2.len()
            + self.modified.len()
    }
}

/// Represents a key-value pair that differs between two files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifiedPair {
    /// The shared key
    pub key: String,
    /// The value in file1
    pub value_file1: String,
    /// The value in file2
    pub value_file2: String,
    /// Line number in file1
    pub line_number_file1: usize,
    /// Line number in file2
    pub line_number_file2: usize,
}

/// Represents the result of a literal (line-by-line) diff.
#[derive(Debug, Clone)]
pub struct LiteralDiffResult {
    /// Lines added in file2
    pub additions: Vec<DiffLine>,
    /// Lines removed from file1
    pub deletions: Vec<DiffLine>,
    /// Lines modified between files (old, new)
    pub modifications: Vec<(DiffLine, DiffLine)>,
}

/// A single line from a diff result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffLine {
    /// The content of the line
    pub content: String,
    /// The line number in the source file (1-indexed)
    pub line_number: usize,
}

/// Direction for patch file generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatchDirection {
    /// Generate patch with keys from file1 missing in file2 (default)
    #[default]
    SourceToTarget,
    /// Generate patch with keys from file2 missing in file1
    TargetToSource,
    /// Generate both patches as separate files
    Bidirectional,
}

impl std::str::FromStr for PatchDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "source-to-target" | "source" | "s2t" => Ok(Self::SourceToTarget),
            "target-to-source" | "target" | "t2s" => Ok(Self::TargetToSource),
            "bidirectional" | "both" | "bi" => Ok(Self::Bidirectional),
            _ => Err(format!(
                "Invalid direction '{}'. Valid options: source-to-target, target-to-source, bidirectional",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_options_default() {
        let opts = ParseOptions::default();
        assert_eq!(opts.delimiter, '=');
        assert!(!opts.strip_quotes);
        assert_eq!(opts.comment_prefixes, vec!["#", "//"]);
    }

    #[test]
    fn test_diff_result_is_identical_when_empty() {
        let result = DiffResult {
            missing_in_file2: vec![],
            missing_in_file1: vec![],
            modified: vec![],
            identical: vec!["KEY".to_string()],
        };
        assert!(result.is_identical());
        assert_eq!(result.difference_count(), 0);
    }

    #[test]
    fn test_diff_result_is_not_identical_with_missing() {
        let result = DiffResult {
            missing_in_file2: vec![KvPair {
                key: "A".to_string(),
                value: "1".to_string(),
                line_number: 1,
            }],
            missing_in_file1: vec![],
            modified: vec![],
            identical: vec![],
        };
        assert!(!result.is_identical());
        assert_eq!(result.difference_count(), 1);
    }

    #[test]
    fn test_diff_result_difference_count_all_categories() {
        let result = DiffResult {
            missing_in_file2: vec![KvPair {
                key: "A".to_string(),
                value: "1".to_string(),
                line_number: 1,
            }],
            missing_in_file1: vec![
                KvPair { key: "B".to_string(), value: "2".to_string(), line_number: 2 },
                KvPair { key: "C".to_string(), value: "3".to_string(), line_number: 3 },
            ],
            modified: vec![ModifiedPair {
                key: "D".to_string(),
                value_file1: "old".to_string(),
                value_file2: "new".to_string(),
                line_number_file1: 4,
                line_number_file2: 4,
            }],
            identical: vec!["E".to_string()],
        };
        assert_eq!(result.difference_count(), 4);
    }

    #[test]
    fn test_patch_direction_default() {
        assert_eq!(PatchDirection::default(), PatchDirection::SourceToTarget);
    }

    #[test]
    fn test_patch_direction_from_str() {
        assert_eq!("source-to-target".parse::<PatchDirection>(), Ok(PatchDirection::SourceToTarget));
        assert_eq!("source".parse::<PatchDirection>(), Ok(PatchDirection::SourceToTarget));
        assert_eq!("s2t".parse::<PatchDirection>(), Ok(PatchDirection::SourceToTarget));
        assert_eq!("target-to-source".parse::<PatchDirection>(), Ok(PatchDirection::TargetToSource));
        assert_eq!("target".parse::<PatchDirection>(), Ok(PatchDirection::TargetToSource));
        assert_eq!("t2s".parse::<PatchDirection>(), Ok(PatchDirection::TargetToSource));
        assert_eq!("bidirectional".parse::<PatchDirection>(), Ok(PatchDirection::Bidirectional));
        assert_eq!("both".parse::<PatchDirection>(), Ok(PatchDirection::Bidirectional));
        assert_eq!("bi".parse::<PatchDirection>(), Ok(PatchDirection::Bidirectional));
    }

    #[test]
    fn test_patch_direction_from_str_case_insensitive() {
        assert_eq!("SOURCE-TO-TARGET".parse::<PatchDirection>(), Ok(PatchDirection::SourceToTarget));
        assert_eq!("Bidirectional".parse::<PatchDirection>(), Ok(PatchDirection::Bidirectional));
    }

    #[test]
    fn test_patch_direction_from_str_invalid() {
        let result = "invalid".parse::<PatchDirection>();
        assert!(result.is_err());
        assert!(result.err().map_or(false, |e| e.contains("Invalid direction")));
    }
}
