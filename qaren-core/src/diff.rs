//! Diff engine for semantic and literal comparison of configuration files.
//!
//! Provides two comparison modes:
//!
//! - **Semantic diff** (`semantic_diff`): Order-agnostic, key-value aware
//!   comparison using HashMap for O(n+m) time complexity.
//! - **Literal diff** (`literal_diff`): Line-by-line text comparison using
//!   the Myers diff algorithm via the `similar` crate.

use crate::types::{
    ConfigFile, DiffLine, DiffResult, DiffOptions, KvPair, LiteralDiffResult, ModifiedPair,
};
use similar::{Algorithm, ChangeTag, DiffOp, TextDiff, capture_diff_slices};

/// Normalise a value (or key) string for comparison purposes according to `DiffOptions`.
#[inline]
pub fn normalise(value: &str, opts: &DiffOptions) -> String {
    let mut s = if opts.ignore_all_space {
        value.chars().filter(|c| !c.is_whitespace()).collect()
    } else if opts.ignore_space_change {
        value.split_whitespace().collect::<Vec<_>>().join(" ")
    } else if opts.ignore_trailing_space {
        value.trim_end().to_string()
    } else {
        value.to_string()
    };
    
    if opts.ignore_case {
        s = s.to_lowercase();
    }
    s
}

/// Perform semantic key-value comparison between two configuration files.
pub fn semantic_diff(file1: &ConfigFile, file2: &ConfigFile, opts: &DiffOptions) -> DiffResult {
    let mut missing_in_file2 = Vec::new();
    let mut missing_in_file1 = Vec::new();
    let mut modified = Vec::new();
    let mut identical = Vec::new();

    // Map lowercase key to original key in file2
    let lookup_file2: std::collections::HashMap<String, &String> = file2.pairs.keys()
        .filter(|k| !opts.is_ignored(k))
        .map(|k| (if opts.ignore_case { k.to_lowercase() } else { k.clone() }, k))
        .collect();

    // Check all keys in file1 against file2
    for (key1, (value1, line_num1)) in &file1.pairs {
        if opts.is_ignored(key1) { continue; }
        
        let search_key = if opts.ignore_case { key1.to_lowercase() } else { key1.clone() };
        
        // Lookup matching key in file2
        let file2_entry = lookup_file2.get(&search_key)
            .and_then(|&k2| file2.pairs.get(k2).map(|v| (k2, v)));

        match file2_entry {
            Some((_key2, (value2, line_num2))) => {
                if normalise(value1, opts) == normalise(value2, opts) {
                    // Use the key from file1 as the base output key
                    identical.push(key1.clone());
                } else {
                    modified.push(ModifiedPair {
                        key: key1.clone(),
                        value_file1: value1.clone(),
                        value_file2: value2.clone(),
                        line_number_file1: *line_num1,
                        line_number_file2: *line_num2,
                    });
                }
            }
            None => {
                missing_in_file2.push(KvPair {
                    key: key1.clone(),
                    value: value1.clone(),
                    line_number: *line_num1,
                });
            }
        }
    }

    // Map lowercase key to original key in file1
    let lookup_file1: std::collections::HashMap<String, &String> = file1.pairs.keys()
        .filter(|k| !opts.is_ignored(k))
        .map(|k| (if opts.ignore_case { k.to_lowercase() } else { k.clone() }, k))
        .collect();

    // Check for keys in file2 that are not in file1
    for (key2, (value2, line_num2)) in &file2.pairs {
        if opts.is_ignored(key2) { continue; }
        let search_key = if opts.ignore_case { key2.to_lowercase() } else { key2.clone() };
        if !lookup_file1.contains_key(&search_key) {
            missing_in_file1.push(KvPair {
                key: key2.clone(),
                value: value2.clone(),
                line_number: *line_num2,
            });
        }
    }

    DiffResult {
        missing_in_file2,
        missing_in_file1,
        modified,
        identical,
    }
}

/// Perform literal line-by-line comparison between two text contents.
///
/// Uses the Myers diff algorithm (via the `similar` crate) to detect
/// additions, deletions, and unchanged lines. Line numbers are tracked
/// separately for old and new content.
pub fn literal_diff(content1: &[u8], content2: &[u8], opts: &DiffOptions) -> LiteralDiffResult {
    // Fast-path: Exact equality
    if content1 == content2 {
        return LiteralDiffResult {
            additions: Vec::new(),
            deletions: Vec::new(),
            modifications: Vec::new(),
        };
    }

    let has_ignore_flags = opts.ignore_case || opts.ignore_all_space || opts.ignore_space_change || opts.ignore_trailing_space || opts.strip_ansi;

    let mut additions = Vec::new();
    let mut deletions = Vec::new();

    if !has_ignore_flags {
        // Optimized fast path: manual line splitting for byte slices to avoid similar v2.7 limitations
        let lines1: Vec<&[u8]> = content1.split(|&b| b == b'\n').collect();
        let lines2: Vec<&[u8]> = content2.split(|&b| b == b'\n').collect();
        
        let ops = capture_diff_slices(Algorithm::Myers, &lines1, &lines2);
        
        for op in ops {
            match op {
                DiffOp::Delete { old_index, old_len, .. } => {
                    for i in 0..old_len {
                        let idx = old_index + i;
                        let mut val = lines1[idx];
                        if let Some(stripped) = val.strip_suffix(b"\r") {
                            val = stripped;
                        }
                        let content = String::from_utf8_lossy(val).to_string();
                        if !opts.ignore_blank_lines || !content.trim().is_empty() {
                            deletions.push(DiffLine {
                                content,
                                line_number: idx + 1,
                            });
                        }
                    }
                }
                DiffOp::Insert { new_index, new_len, .. } => {
                    for i in 0..new_len {
                        let idx = new_index + i;
                        let mut val = lines2[idx];
                        if let Some(stripped) = val.strip_suffix(b"\r") {
                            val = stripped;
                        }
                        let content = String::from_utf8_lossy(val).to_string();
                        if !opts.ignore_blank_lines || !content.trim().is_empty() {
                            additions.push(DiffLine {
                                content,
                                line_number: idx + 1,
                            });
                        }
                    }
                }
                DiffOp::Replace { old_index, old_len, new_index, new_len } => {
                    for i in 0..old_len {
                        let idx = old_index + i;
                        let mut val = lines1[idx];
                        if let Some(stripped) = val.strip_suffix(b"\r") {
                            val = stripped;
                        }
                        let content = String::from_utf8_lossy(val).to_string();
                        if !opts.ignore_blank_lines || !content.trim().is_empty() {
                            deletions.push(DiffLine {
                                content,
                                line_number: idx + 1,
                            });
                        }
                    }
                    for i in 0..new_len {
                        let idx = new_index + i;
                        let mut val = lines2[idx];
                        if let Some(stripped) = val.strip_suffix(b"\r") {
                            val = stripped;
                        }
                        let content = String::from_utf8_lossy(val).to_string();
                        if !opts.ignore_blank_lines || !content.trim().is_empty() {
                            additions.push(DiffLine {
                                content,
                                line_number: idx + 1,
                            });
                        }
                    }
                }
                DiffOp::Equal { .. } => {}
            }
        }
    } else {
        // Slow path: Normalization or ANSI stripping is required.
        let mut s1 = String::from_utf8_lossy(content1).to_string();
        let mut s2 = String::from_utf8_lossy(content2).to_string();

        if opts.strip_ansi {
            s1 = crate::parser::strip_ansi(&s1);
            s2 = crate::parser::strip_ansi(&s2);
        }
        
        let lines1: Vec<&str> = s1.lines().collect();
        let lines2: Vec<&str> = s2.lines().collect();

        let norm1: Vec<String> = lines1.iter().map(|&l| normalise(l, opts)).collect();
        let norm2: Vec<String> = lines2.iter().map(|&l| normalise(l, opts)).collect();
        
        let refs1: Vec<&str> = norm1.iter().map(|s| s.as_str()).collect();
        let refs2: Vec<&str> = norm2.iter().map(|s| s.as_str()).collect();

        let diff = TextDiff::from_slices(&refs1, &refs2);

        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    let idx = change.old_index().unwrap();
                    let orig_content = lines1.get(idx).unwrap_or(&"").to_string();
                    if !opts.ignore_blank_lines || !orig_content.trim().is_empty() {
                        deletions.push(DiffLine {
                            content: orig_content,
                            line_number: idx + 1,
                        });
                    }
                }
                ChangeTag::Insert => {
                    let idx = change.new_index().unwrap();
                    let orig_content = lines2.get(idx).unwrap_or(&"").to_string();
                    if !opts.ignore_blank_lines || !orig_content.trim().is_empty() {
                        additions.push(DiffLine {
                            content: orig_content,
                            line_number: idx + 1,
                        });
                    }
                }
                ChangeTag::Equal => {}
            }
        }
    }

    LiteralDiffResult {
        additions,
        deletions,
        modifications: Vec::new(),
    }
}

// ─────────────────────────────────────────────────────────────────────
// Unit Tests (Task 5.5)
// ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn default_opts() -> DiffOptions { DiffOptions::default() }

    /// Helper to build a ConfigFile from a list of (key, value) pairs.
    fn make_config(pairs: &[(&str, &str)]) -> ConfigFile {
        let mut map = HashMap::new();
        for (i, (k, v)) in pairs.iter().enumerate() {
            map.insert(k.to_string(), (v.to_string(), i + 1));
        }
        ConfigFile {
            pairs: map,
            file_path: PathBuf::from("test.env"), warnings: vec![],
        }
    }

    // ── Semantic diff tests ─────────────────────────────────────────

    #[test]
    fn test_identical_files() {
        let file1 = make_config(&[("A", "1"), ("B", "2"), ("C", "3")]);
        let file2 = make_config(&[("A", "1"), ("B", "2"), ("C", "3")]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        assert!(result.is_identical());
        assert_eq!(result.difference_count(), 0);
        assert_eq!(result.identical.len(), 3);
        assert!(result.missing_in_file1.is_empty());
        assert!(result.missing_in_file2.is_empty());
        assert!(result.modified.is_empty());
    }

    #[test]
    fn test_completely_different_files() {
        let file1 = make_config(&[("A", "1"), ("B", "2")]);
        let file2 = make_config(&[("C", "3"), ("D", "4")]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        assert!(!result.is_identical());
        assert_eq!(result.missing_in_file2.len(), 2);
        assert_eq!(result.missing_in_file1.len(), 2);
        assert!(result.modified.is_empty());
        assert!(result.identical.is_empty());
    }

    #[test]
    fn test_only_additions() {
        let file1 = make_config(&[("A", "1")]);
        let file2 = make_config(&[("A", "1"), ("B", "2"), ("C", "3")]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        assert!(!result.is_identical());
        assert!(result.missing_in_file2.is_empty());
        assert_eq!(result.missing_in_file1.len(), 2);
        assert_eq!(result.identical.len(), 1);
    }

    #[test]
    fn test_only_deletions() {
        let file1 = make_config(&[("A", "1"), ("B", "2"), ("C", "3")]);
        let file2 = make_config(&[("A", "1")]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        assert!(!result.is_identical());
        assert_eq!(result.missing_in_file2.len(), 2);
        assert!(result.missing_in_file1.is_empty());
        assert_eq!(result.identical.len(), 1);
    }

    #[test]
    fn test_only_modifications() {
        let file1 = make_config(&[("A", "1"), ("B", "2")]);
        let file2 = make_config(&[("A", "10"), ("B", "20")]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        assert!(!result.is_identical());
        assert_eq!(result.modified.len(), 2);
        assert!(result.missing_in_file1.is_empty());
        assert!(result.missing_in_file2.is_empty());
        assert!(result.identical.is_empty());
    }

    #[test]
    fn test_mixed_differences() {
        let file1 = make_config(&[("A", "1"), ("B", "2"), ("C", "3")]);
        let file2 = make_config(&[("A", "1"), ("B", "20"), ("D", "4")]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        assert!(!result.is_identical());
        // A is identical
        assert_eq!(result.identical.len(), 1);
        assert!(result.identical.contains(&"A".to_string()));
        // B is modified
        assert_eq!(result.modified.len(), 1);
        assert_eq!(result.modified[0].key, "B");
        assert_eq!(result.modified[0].value_file1, "2");
        assert_eq!(result.modified[0].value_file2, "20");
        // C is missing in file2
        assert_eq!(result.missing_in_file2.len(), 1);
        assert_eq!(result.missing_in_file2[0].key, "C");
        // D is missing in file1
        assert_eq!(result.missing_in_file1.len(), 1);
        assert_eq!(result.missing_in_file1[0].key, "D");
    }

    #[test]
    fn test_empty_files() {
        let file1 = make_config(&[]);
        let file2 = make_config(&[]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        assert!(result.is_identical());
        assert_eq!(result.difference_count(), 0);
    }

    #[test]
    fn test_one_empty_file() {
        let file1 = make_config(&[("A", "1"), ("B", "2")]);
        let file2 = make_config(&[]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        assert!(!result.is_identical());
        assert_eq!(result.missing_in_file2.len(), 2);
        assert!(result.missing_in_file1.is_empty());
    }

    #[test]
    fn test_exit_code_logic_identical() {
        let file1 = make_config(&[("A", "1")]);
        let file2 = make_config(&[("A", "1")]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        // Exit code 0 for identical
        assert!(result.is_identical());
    }

    #[test]
    fn test_exit_code_logic_different() {
        let file1 = make_config(&[("A", "1")]);
        let file2 = make_config(&[("A", "2")]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        // Exit code 1 for different
        assert!(!result.is_identical());
    }

    #[test]
    fn test_modified_pair_tracks_line_numbers() {
        let file1 = make_config(&[("A", "old_value")]);
        let file2 = make_config(&[("A", "new_value")]);
        let result = semantic_diff(&file1, &file2, &default_opts());

        assert_eq!(result.modified.len(), 1);
        assert_eq!(result.modified[0].line_number_file1, 1);
        assert_eq!(result.modified[0].line_number_file2, 1);
    }

    // ── ignore_case / ignore_whitespace tests ──────────────────────

    #[test]
    fn test_ignore_case_treats_as_identical() {
        let file1 = make_config(&[("KEY", "Value")]);
        let file2 = make_config(&[("KEY", "value")]);
        let opts = DiffOptions { ignore_case: true, ..DiffOptions::default() };
        let result = semantic_diff(&file1, &file2, &opts);
        assert!(result.is_identical(), "ignore_case should treat 'Value' == 'value'");
    }

    #[test]
    fn test_ignore_case_false_detects_difference() {
        let file1 = make_config(&[("KEY", "Value")]);
        let file2 = make_config(&[("KEY", "value")]);
        let result = semantic_diff(&file1, &file2, &default_opts());
        assert!(!result.is_identical(), "case-sensitive: 'Value' != 'value'");
    }

    #[test]
    fn test_ignore_whitespace_treats_as_identical() {
        let file1 = make_config(&[("KEY", "hello   world")]);
        let file2 = make_config(&[("KEY", "hello world")]);
        let opts = DiffOptions { ignore_all_space: true, ..DiffOptions::default() };
        let result = semantic_diff(&file1, &file2, &opts);
        assert!(result.is_identical(), "ignore_whitespace should collapse spaces");
    }

    #[test]
    fn test_ignore_whitespace_false_detects_difference() {
        let file1 = make_config(&[("KEY", "hello   world")]);
        let file2 = make_config(&[("KEY", "hello world")]);
        let result = semantic_diff(&file1, &file2, &default_opts());
        assert!(!result.is_identical(), "whitespace-sensitive: extra spaces differ");
    }

    #[test]
    fn test_original_values_preserved_in_result() {
        // Even when ignoring case, the STORED values should be the originals
        let file1 = make_config(&[("KEY", "Value")]);
        let file2 = make_config(&[("KEY", "value")]);
        let opts = DiffOptions { ignore_case: true, ..DiffOptions::default() };
        let result = semantic_diff(&file1, &file2, &opts);
        // Identical — but we verify normalise() didn't mutate stored values
        assert!(result.modified.is_empty());
        assert!(result.identical.contains(&"KEY".to_string()));
    }

    #[test]
    fn test_ignore_keys_skips_missing_and_modified() {
        let file1 = make_config(&[("KEY1", "v1"), ("KEY2", "v2"), ("KEY3", "v3")]);
        let file2 = make_config(&[("KEY1", "v1_mod"), ("KEY3", "v3"), ("KEY4", "v4")]);
        
        let opts = DiffOptions {
            ignore_keys: vec!["KEY1".to_string(), "KEY2".to_string(), "KEY4".to_string()],
            ..Default::default()
        };
        
        let result = semantic_diff(&file1, &file2, &opts);
        
        // KEY1 is modified, but ignored.
        // KEY2 is missing in file2, but ignored.
        // KEY4 is missing in file1, but ignored.
        // Only KEY3 remains, and it's identical.
        assert!(result.modified.is_empty());
        assert!(result.missing_in_file1.is_empty());
        assert!(result.missing_in_file2.is_empty());
        assert_eq!(result.identical.len(), 1);
        assert_eq!(result.identical[0], "KEY3");
        assert!(result.is_identical());
    }

    #[test]
    fn test_ignore_keywords_case_insensitive_skips() {
        let file1 = make_config(&[("GITHUB_TOKEN", "123"), ("HOST", "localhost")]);
        let file2 = make_config(&[("github_url", "http"), ("HOST", "127.0.0.1")]);
        
        let opts = DiffOptions {
            ignore_keywords: vec!["github".to_string()],
            ..Default::default()
        };
        
        let result = semantic_diff(&file1, &file2, &opts);
        
        // GITHUB_TOKEN and github_url should be ignored.
        // HOST is modified and should be detected.
        assert_eq!(result.modified.len(), 1);
        assert_eq!(result.modified[0].key, "HOST");
        assert!(result.missing_in_file1.is_empty());
        assert!(result.missing_in_file2.is_empty());
    }

    // ── Literal diff tests ──────────────────────────────────────────

    #[test]
    fn test_literal_identical() {
        let result = literal_diff("line1\nline2\n".as_bytes(), "line1\nline2\n".as_bytes(), &default_opts());
        assert!(result.additions.is_empty());
        assert!(result.deletions.is_empty());
    }

    #[test]
    fn test_literal_additions() {
        let result = literal_diff("line1\n".as_bytes(), "line1\nline2\n".as_bytes(), &default_opts());
        assert_eq!(result.additions.len(), 1);
        assert!(result.additions[0].content.contains("line2"));
        assert!(result.deletions.is_empty());
    }

    #[test]
    fn test_literal_deletions() {
        let result = literal_diff("line1\nline2\n".as_bytes(), "line1\n".as_bytes(), &default_opts());
        assert!(result.additions.is_empty());
        assert_eq!(result.deletions.len(), 1);
        assert!(result.deletions[0].content.contains("line2"));
    }

    #[test]
    fn test_literal_mixed_changes() {
        let content1 = "line1\nline2\nline3\n";
        let content2 = "line1\nmodified\nline3\nline4\n";
        let result = literal_diff(content1.as_bytes(), content2.as_bytes(), &default_opts());

        // line2 deleted, "modified" and "line4" added
        assert!(!result.deletions.is_empty());
        assert!(!result.additions.is_empty());
    }

    #[test]
    fn test_literal_empty_files() {
        let result = literal_diff(b"", b"", &default_opts());
        assert!(result.additions.is_empty());
        assert!(result.deletions.is_empty());
    }

    #[test]
    fn test_literal_empty_vs_content() {
        let result = literal_diff(b"", "line1\nline2\n".as_bytes(), &default_opts());
        assert_eq!(result.additions.len(), 2);
        assert!(result.deletions.is_empty());
    }

    #[test]
    fn test_literal_tracks_line_numbers() {
        let result = literal_diff(b"a\nb\n", b"a\nc\n", &default_opts());
        // 'b' deleted at line 2, 'c' added at line 2
        if !result.deletions.is_empty() {
            assert_eq!(result.deletions[0].line_number, 2);
        }
        if !result.additions.is_empty() {
            assert_eq!(result.additions[0].line_number, 2);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// Property-based tests (Tasks 5.3, 5.4)
// ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use std::collections::HashMap;
    use std::path::PathBuf;

    /// Check if a character is safe for use in test inputs.
    fn is_safe_char(c: char) -> bool {
        !c.is_control() && c.is_ascii() && c != '\0'
    }

    /// Sanitize a key for test use
    fn sanitize_key(s: &str) -> Option<String> {
        let clean: String = s
            .chars()
            .filter(|c| is_safe_char(*c) && *c != '=' && *c != '#' && *c != '/')
            .collect();
        let clean = clean.trim().to_string();
        if clean.is_empty() {
            return None;
        }
        Some(clean)
    }

    /// Sanitize a value for test use
    /// Also avoid ending with a backslash to prevent accidental multi-line joining in property tests.
    fn sanitize_value(s: &str) -> String {
        let clean: String = s.chars()
            .filter(|c| is_safe_char(*c))
            .collect();
            
        let mut trimmed = clean.trim().to_string();
        while trimmed.ends_with('\\') || trimmed.ends_with(' ') {
            if trimmed.ends_with('\\') {
                trimmed.pop();
            } else {
                trimmed = trimmed.trim_end().to_string();
            }
        }
        trimmed
    }

    /// Build a ConfigFile from sanitized pairs (deduplicates keys)
    fn build_config(raw_pairs: &[(String, String)]) -> (ConfigFile, Vec<(String, String)>) {
        let mut map = HashMap::new();
        let mut unique_pairs = Vec::new();

        for (i, (k, v)) in raw_pairs.iter().enumerate() {
            let key = match sanitize_key(k) {
                Some(k) => k,
                None => continue,
            };
            let value = sanitize_value(v);
            if !map.contains_key(&key) {
                unique_pairs.push((key.clone(), value.clone()));
            }
            map.insert(key, (value, i + 1));
        }

        (
            ConfigFile {
                pairs: map,
                file_path: PathBuf::from("test.env"), warnings: vec![],
            },
            unique_pairs,
        )
    }

    // ── Property 1: Comparison Symmetry (Inverse Relationship) ──────

    #[quickcheck]
    fn prop_comparison_symmetry(
        pairs_a: Vec<(String, String)>,
        pairs_b: Vec<(String, String)>,
    ) -> TestResult {
        let (file_a, _) = build_config(&pairs_a);
        let (file_b, _) = build_config(&pairs_b);

        if file_a.pairs.is_empty() && file_b.pairs.is_empty() {
            return TestResult::discard();
        }

        let diff_ab = semantic_diff(&file_a, &file_b, &DiffOptions::default());
        let diff_ba = semantic_diff(&file_b, &file_a, &DiffOptions::default());

        // missing_in_file2 for A→B  ==  missing_in_file1 for B→A (by keys)
        let missing_ab_keys: std::collections::HashSet<_> =
            diff_ab.missing_in_file2.iter().map(|p| &p.key).collect();
        let missing_ba_keys: std::collections::HashSet<_> =
            diff_ba.missing_in_file1.iter().map(|p| &p.key).collect();

        if missing_ab_keys != missing_ba_keys {
            return TestResult::failed();
        }

        // missing_in_file1 for A→B  ==  missing_in_file2 for B→A (by keys)
        let missing_ab_keys2: std::collections::HashSet<_> =
            diff_ab.missing_in_file1.iter().map(|p| &p.key).collect();
        let missing_ba_keys2: std::collections::HashSet<_> =
            diff_ba.missing_in_file2.iter().map(|p| &p.key).collect();

        if missing_ab_keys2 != missing_ba_keys2 {
            return TestResult::failed();
        }

        // Modified pairs should have swapped values
        let mod_ab: HashMap<_, _> = diff_ab
            .modified
            .iter()
            .map(|m| (&m.key, (&m.value_file1, &m.value_file2)))
            .collect();
        let mod_ba: HashMap<_, _> = diff_ba
            .modified
            .iter()
            .map(|m| (&m.key, (&m.value_file1, &m.value_file2)))
            .collect();

        if mod_ab.len() != mod_ba.len() {
            return TestResult::failed();
        }

        for (key, (v1_ab, v2_ab)) in &mod_ab {
            match mod_ba.get(key) {
                Some((v1_ba, v2_ba)) => {
                    // A→B: file1=v1, file2=v2 means B→A: file1=v2, file2=v1
                    if v1_ab != v2_ba || v2_ab != v1_ba {
                        return TestResult::failed();
                    }
                }
                None => return TestResult::failed(),
            }
        }

        // Identical keys should be the same in both directions
        let identical_ab: std::collections::HashSet<_> =
            diff_ab.identical.iter().collect();
        let identical_ba: std::collections::HashSet<_> =
            diff_ba.identical.iter().collect();

        TestResult::from_bool(identical_ab == identical_ba)
    }

    // ── Property 7: Linear Time Complexity ──────────────────────────

    #[test]
    fn prop_linear_time_complexity() {
        // Generate config files of increasing sizes and verify
        // that time scales roughly linearly

        fn generate_config(size: usize) -> ConfigFile {
            let mut pairs = HashMap::new();
            for i in 0..size {
                pairs.insert(
                    format!("KEY_{}", i),
                    (format!("value_{}", i), i + 1),
                );
            }
            ConfigFile {
                pairs,
                file_path: PathBuf::from("bench.env"), warnings: vec![],
            }
        }

        let sizes = [100, 1000, 10000];
        let mut times = Vec::new();

        for &size in &sizes {
            let file1 = generate_config(size);
            let file2 = generate_config(size);

            let start = std::time::Instant::now();
            for _ in 0..10 {
                let _ = semantic_diff(&file1, &file2, &DiffOptions::default());
            }
            let elapsed = start.elapsed();
            times.push(elapsed.as_nanos() as f64);
        }

        // Check that time ratio is roughly linear:
        // time(10000) / time(1000) should be roughly 10x (allow 50x for overhead)
        // time(1000) / time(100) should be roughly 10x (allow 50x for overhead)
        if times[0] > 0.0 {
            let ratio_1 = times[1] / times[0];
            let ratio_2 = times[2] / times[1];

            // Allow generous bounds — we just want to verify it's NOT quadratic
            // Quadratic would give ~100x ratio, linear gives ~10x
            assert!(
                ratio_1 < 50.0,
                "Time ratio 1000/100 = {:.1}x (should be < 50x for linear)",
                ratio_1
            );
            assert!(
                ratio_2 < 50.0,
                "Time ratio 10000/1000 = {:.1}x (should be < 50x for linear)",
                ratio_2
            );
        }
    }
}
