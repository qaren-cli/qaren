//! Diff engine for semantic and literal comparison of configuration files.
//!
//! Provides two comparison modes:
//!
//! - **Semantic diff** (`semantic_diff`): Order-agnostic, key-value aware
//!   comparison using HashMap for O(n+m) time complexity.
//! - **Literal diff** (`literal_diff`): Line-by-line text comparison using
//!   the Myers diff algorithm via the `similar` crate.

use crate::types::{ConfigFile, DiffLine, DiffResult, KvPair, LiteralDiffResult, ModifiedPair};
use similar::{ChangeTag, TextDiff};

/// Perform semantic key-value comparison between two configuration files.
///
/// This is an order-agnostic comparison that categorizes keys into four groups:
/// - **missing_in_file2**: keys in file1 not present in file2
/// - **missing_in_file1**: keys in file2 not present in file1
/// - **modified**: keys in both files with different values
/// - **identical**: keys in both files with the same value
///
/// # Complexity
///
/// - **Time**: O(n + m) where n and m are the number of pairs in each file
/// - **Space**: O(n + m) for storing results
pub fn semantic_diff(file1: &ConfigFile, file2: &ConfigFile) -> DiffResult {
    let mut missing_in_file2 = Vec::new();
    let mut missing_in_file1 = Vec::new();
    let mut modified = Vec::new();
    let mut identical = Vec::new();

    // Check all keys in file1 against file2
    for (key, (value1, line_num1)) in &file1.pairs {
        match file2.pairs.get(key) {
            Some((value2, line_num2)) => {
                if value1 == value2 {
                    identical.push(key.clone());
                } else {
                    modified.push(ModifiedPair {
                        key: key.clone(),
                        value_file1: value1.clone(),
                        value_file2: value2.clone(),
                        line_number_file1: *line_num1,
                        line_number_file2: *line_num2,
                    });
                }
            }
            None => {
                missing_in_file2.push(KvPair {
                    key: key.clone(),
                    value: value1.clone(),
                    line_number: *line_num1,
                });
            }
        }
    }

    // Check for keys in file2 that are not in file1
    for (key, (value2, line_num2)) in &file2.pairs {
        if !file1.pairs.contains_key(key) {
            missing_in_file1.push(KvPair {
                key: key.clone(),
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
pub fn literal_diff(content1: &str, content2: &str) -> LiteralDiffResult {
    let diff = TextDiff::from_lines(content1, content2);

    let mut additions = Vec::new();
    let mut deletions = Vec::new();
    let modifications = Vec::new();

    let mut old_line_num: usize = 1;
    let mut new_line_num: usize = 1;

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                deletions.push(DiffLine {
                    content: change.value().to_string(),
                    line_number: old_line_num,
                });
                old_line_num += 1;
            }
            ChangeTag::Insert => {
                additions.push(DiffLine {
                    content: change.value().to_string(),
                    line_number: new_line_num,
                });
                new_line_num += 1;
            }
            ChangeTag::Equal => {
                old_line_num += 1;
                new_line_num += 1;
            }
        }
    }

    LiteralDiffResult {
        additions,
        deletions,
        modifications,
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

    /// Helper to build a ConfigFile from a list of (key, value) pairs.
    fn make_config(pairs: &[(&str, &str)]) -> ConfigFile {
        let mut map = HashMap::new();
        for (i, (k, v)) in pairs.iter().enumerate() {
            map.insert(k.to_string(), (v.to_string(), i + 1));
        }
        ConfigFile {
            pairs: map,
            file_path: PathBuf::from("test.env"),
        }
    }

    // ── Semantic diff tests ─────────────────────────────────────────

    #[test]
    fn test_identical_files() {
        let file1 = make_config(&[("A", "1"), ("B", "2"), ("C", "3")]);
        let file2 = make_config(&[("A", "1"), ("B", "2"), ("C", "3")]);
        let result = semantic_diff(&file1, &file2);

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
        let result = semantic_diff(&file1, &file2);

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
        let result = semantic_diff(&file1, &file2);

        assert!(!result.is_identical());
        assert!(result.missing_in_file2.is_empty());
        assert_eq!(result.missing_in_file1.len(), 2);
        assert_eq!(result.identical.len(), 1);
    }

    #[test]
    fn test_only_deletions() {
        let file1 = make_config(&[("A", "1"), ("B", "2"), ("C", "3")]);
        let file2 = make_config(&[("A", "1")]);
        let result = semantic_diff(&file1, &file2);

        assert!(!result.is_identical());
        assert_eq!(result.missing_in_file2.len(), 2);
        assert!(result.missing_in_file1.is_empty());
        assert_eq!(result.identical.len(), 1);
    }

    #[test]
    fn test_only_modifications() {
        let file1 = make_config(&[("A", "1"), ("B", "2")]);
        let file2 = make_config(&[("A", "10"), ("B", "20")]);
        let result = semantic_diff(&file1, &file2);

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
        let result = semantic_diff(&file1, &file2);

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
        let result = semantic_diff(&file1, &file2);

        assert!(result.is_identical());
        assert_eq!(result.difference_count(), 0);
    }

    #[test]
    fn test_one_empty_file() {
        let file1 = make_config(&[("A", "1"), ("B", "2")]);
        let file2 = make_config(&[]);
        let result = semantic_diff(&file1, &file2);

        assert!(!result.is_identical());
        assert_eq!(result.missing_in_file2.len(), 2);
        assert!(result.missing_in_file1.is_empty());
    }

    #[test]
    fn test_exit_code_logic_identical() {
        let file1 = make_config(&[("A", "1")]);
        let file2 = make_config(&[("A", "1")]);
        let result = semantic_diff(&file1, &file2);

        // Exit code 0 for identical
        assert!(result.is_identical());
    }

    #[test]
    fn test_exit_code_logic_different() {
        let file1 = make_config(&[("A", "1")]);
        let file2 = make_config(&[("A", "2")]);
        let result = semantic_diff(&file1, &file2);

        // Exit code 1 for different
        assert!(!result.is_identical());
    }

    #[test]
    fn test_modified_pair_tracks_line_numbers() {
        let file1 = make_config(&[("A", "old_value")]);
        let file2 = make_config(&[("A", "new_value")]);
        let result = semantic_diff(&file1, &file2);

        assert_eq!(result.modified.len(), 1);
        assert_eq!(result.modified[0].line_number_file1, 1);
        assert_eq!(result.modified[0].line_number_file2, 1);
    }

    // ── Literal diff tests ──────────────────────────────────────────

    #[test]
    fn test_literal_identical() {
        let result = literal_diff("line1\nline2\n", "line1\nline2\n");
        assert!(result.additions.is_empty());
        assert!(result.deletions.is_empty());
    }

    #[test]
    fn test_literal_additions() {
        let result = literal_diff("line1\n", "line1\nline2\n");
        assert_eq!(result.additions.len(), 1);
        assert!(result.additions[0].content.contains("line2"));
        assert!(result.deletions.is_empty());
    }

    #[test]
    fn test_literal_deletions() {
        let result = literal_diff("line1\nline2\n", "line1\n");
        assert!(result.additions.is_empty());
        assert_eq!(result.deletions.len(), 1);
        assert!(result.deletions[0].content.contains("line2"));
    }

    #[test]
    fn test_literal_mixed_changes() {
        let content1 = "line1\nline2\nline3\n";
        let content2 = "line1\nmodified\nline3\nline4\n";
        let result = literal_diff(content1, content2);

        // line2 deleted, "modified" and "line4" added
        assert!(!result.deletions.is_empty());
        assert!(!result.additions.is_empty());
    }

    #[test]
    fn test_literal_empty_files() {
        let result = literal_diff("", "");
        assert!(result.additions.is_empty());
        assert!(result.deletions.is_empty());
    }

    #[test]
    fn test_literal_empty_vs_content() {
        let result = literal_diff("", "line1\nline2\n");
        assert_eq!(result.additions.len(), 2);
        assert!(result.deletions.is_empty());
    }

    #[test]
    fn test_literal_tracks_line_numbers() {
        let result = literal_diff("a\nb\n", "a\nc\n");
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
    fn sanitize_value(s: &str) -> String {
        s.chars()
            .filter(|c| is_safe_char(*c))
            .collect::<String>()
            .trim()
            .to_string()
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
                file_path: PathBuf::from("test.env"),
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

        let diff_ab = semantic_diff(&file_a, &file_b);
        let diff_ba = semantic_diff(&file_b, &file_a);

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
                file_path: PathBuf::from("bench.env"),
            }
        }

        let sizes = [100, 1000, 10000];
        let mut times = Vec::new();

        for &size in &sizes {
            let file1 = generate_config(size);
            let file2 = generate_config(size);

            let start = std::time::Instant::now();
            for _ in 0..10 {
                let _ = semantic_diff(&file1, &file2);
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
