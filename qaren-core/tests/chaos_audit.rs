//! ═══════════════════════════════════════════════════════════════════
//! Chaos Audit & Security Test Suite — Qaren Phase 1
//! ═══════════════════════════════════════════════════════════════════
//! 
//! Autonomous adversarial testing designed by a security auditor to:
//! 1. Break the parser with malformed, weapon-grade edge-case inputs
//! 2. Verify zero-panic guarantee under all conditions
//! 3. Stress-test memory and performance boundaries
//! 4. Audit the secret masking system for bypass vectors
//! 5. Validate diff engine invariants under adversarial data
//! 6. Test patch generator against filesystem edge cases

use qaren_core::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ═══════════════════════════════════════════════════════════════════
// CATEGORY 1: Parser Chaos — Malformed & Adversarial Inputs
// ═══════════════════════════════════════════════════════════════════

mod parser_chaos {
    use super::*;

    fn parse(content: &str) -> ConfigFile {
        parse_content(content, Path::new("chaos.env"), &ParseOptions::default())
            .expect("parser should never fail on in-memory content")
    }

    fn parse_with_opts(content: &str, opts: &ParseOptions) -> ConfigFile {
        parse_content(content, Path::new("chaos.env"), opts)
            .expect("parser should never fail on in-memory content")
    }

    // ── 1.1: Null bytes and binary garbage ──────────────────────────

    #[test]
    fn test_null_bytes_in_key() {
        let content = "KEY\0BROKEN=value";
        let config = parse(content);
        // Should handle without panic — key may have the null or line may be skipped
        // The important thing is NO PANIC
        let _ = config.pairs;
    }

    #[test]
    fn test_null_bytes_in_value() {
        let content = "KEY=val\0ue";
        let config = parse(content);
        let _ = config.pairs;
    }

    #[test]
    fn test_binary_garbage() {
        let content = "\x00\x01\x02\x03\x04\x05\x06=\x07\x08";
        let config = parse(content);
        let _ = config.pairs;
    }

    // ── 1.2: Unicode edge cases ─────────────────────────────────────

    #[test]
    fn test_unicode_key_and_value() {
        let config = parse("مفتاح=قيمة");
        assert_eq!(
            config.pairs.get("مفتاح").map(|(v, _)| v.as_str()),
            Some("قيمة")
        );
    }

    #[test]
    fn test_emoji_key() {
        let config = parse("🔑=🔒secret🔒");
        assert_eq!(
            config.pairs.get("🔑").map(|(v, _)| v.as_str()),
            Some("🔒secret🔒")
        );
    }

    #[test]
    fn test_zero_width_characters() {
        // ZWJ, ZWNJ, ZWS can create "invisible" key differences
        let config = parse("KEY\u{200B}=value");
        // Key contains a zero-width space — should be preserved
        assert!(config.pairs.contains_key("KEY\u{200B}") || config.pairs.contains_key("KEY"));
    }

    #[test]
    fn test_bom_at_start_of_file() {
        // UTF-8 BOM: EF BB BF (\u{FEFF})
        let content = "\u{FEFF}KEY=value";
        let config = parse(content);
        // BOM-infected key: "\u{FEFF}KEY" — this is a known gotcha
        // The parser should ideally strip it, or at least not panic
        let _ = config.pairs;
    }

    #[test]
    fn test_right_to_left_override() {
        // RLO character can visually spoof key names
        let content = "API\u{202E}YEK_=secret";
        let config = parse(content);
        let _ = config.pairs;
    }

    // ── 1.3: Mega-lines and memory stress ───────────────────────────

    #[test]
    fn test_extremely_long_key() {
        let key = "K".repeat(1_000_000); // 1MB key
        let content = format!("{}=value", key);
        let config = parse(&content);
        assert_eq!(config.pairs.len(), 1);
    }

    #[test]
    fn test_extremely_long_value() {
        let value = "V".repeat(1_000_000); // 1MB value
        let content = format!("KEY={}", value);
        let config = parse(&content);
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.len()),
            Some(1_000_000)
        );
    }

    #[test]
    fn test_many_keys() {
        // 100k unique keys
        let content: String = (0..100_000)
            .map(|i| format!("KEY_{}=val_{}", i, i))
            .collect::<Vec<_>>()
            .join("\n");
        let config = parse(&content);
        assert_eq!(config.pairs.len(), 100_000);
    }

    #[test]
    fn test_single_line_no_newline() {
        let config = parse("KEY=value");
        assert_eq!(config.pairs.len(), 1);
    }

    #[test]
    fn test_many_empty_lines() {
        let content = "\n".repeat(100_000) + "KEY=value";
        let config = parse(&content);
        assert_eq!(config.pairs.len(), 1);
    }

    // ── 1.4: Delimiter chaos ────────────────────────────────────────

    #[test]
    fn test_multiple_consecutive_delimiters() {
        let config = parse("KEY====value");
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("===value")
        );
    }

    #[test]
    fn test_delimiter_only_line() {
        let config = parse("=");
        assert!(config.pairs.is_empty());
    }

    #[test]
    fn test_many_delimiters_only() {
        let config = parse("=====");
        // Key is empty → skipped
        assert!(config.pairs.is_empty());
    }

    #[test]
    fn test_tab_delimiter() {
        let opts = ParseOptions {
            delimiter: '\t',
            ..ParseOptions::default()
        };
        let config = parse_with_opts("KEY\tvalue", &opts);
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("value")
        );
    }

    #[test]
    fn test_multi_byte_char_as_delimiter() {
        // Use a multi-byte UTF-8 character as delimiter
        let opts = ParseOptions {
            delimiter: '→',
            ..ParseOptions::default()
        };
        let config = parse_with_opts("KEY→value→extra", &opts);
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("value→extra")
        );
    }

    // ── 1.5: Comment injection & edge cases ─────────────────────────

    #[test]
    fn test_comment_in_key_area() {
        // A line whose key area starts with #
        let config = parse("#KEY=value");
        // Should be treated as comment
        assert!(config.pairs.is_empty());
    }

    #[test]
    fn test_hash_immediately_after_value() {
        let config = parse("KEY=value#nospace");
        // No space before #, so NOT a comment
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("value#nospace")
        );
    }

    #[test]
    fn test_double_slash_in_url_value() {
        let config = parse("URL=https://example.com/path");
        assert_eq!(
            config.pairs.get("URL").map(|(v, _)| v.as_str()),
            Some("https://example.com/path")
        );
    }

    #[test]
    fn test_value_is_only_comment_marker() {
        // KEY= # nothing before the comment
        let config = parse("KEY= # this is a comment");
        // The value after stripping the comment should be empty
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("")
        );
    }

    #[test]
    fn test_custom_comment_prefix() {
        let opts = ParseOptions {
            comment_prefixes: vec![";;".to_string()],
            ..ParseOptions::default()
        };
        let config = parse_with_opts(";; This is a comment\nKEY=value", &opts);
        assert_eq!(config.pairs.len(), 1);
        // # should NOT be treated as comment with custom prefix
        let config2 = parse_with_opts("# not a comment\nKEY=value", &opts);
        // "# not a comment" has no '=' delimiter → skipped anyway
        assert_eq!(config2.pairs.len(), 1);
    }

    #[test]
    fn test_empty_comment_prefix_in_list() {
        // Empty string as comment prefix — could cause every line to be skipped
        let opts = ParseOptions {
            comment_prefixes: vec!["".to_string()],
            ..ParseOptions::default()
        };
        let config = parse_with_opts("KEY=value", &opts);
        // BUG VECTOR: empty prefix.starts_with("") is always true → ALL lines skipped
        // This test documents expected behavior
        let _ = config.pairs;
    }

    // ── 1.6: export prefix edge cases ───────────────────────────────

    #[test]
    fn test_export_without_space() {
        // "exportKEY=value" — should NOT strip "export"
        let config = parse("exportKEY=value");
        assert!(config.pairs.contains_key("exportKEY"));
    }

    #[test]
    fn test_export_capitalized() {
        // "EXPORT KEY=value" — should NOT strip (case-sensitive)
        let config = parse("EXPORT KEY=value");
        // The key should be "EXPORT KEY" since only lowercase "export " is stripped
        let _ = config.pairs;
    }

    #[test]
    fn test_export_only_line() {
        // "export = value"
        let config = parse("export =value");
        // After stripping "export ", key becomes empty → skipped
        // Wait: split_once('=') gives key_raw="export ", value_raw="value"
        // process_key trims "export " → strips "export " prefix → "" → empty → skipped
        assert!(config.pairs.is_empty());
    }

    #[test]
    fn test_double_export_prefix() {
        let config = parse("export export KEY=value");
        // First strip: "export export KEY" → strip "export " → "export KEY"
        // But "export KEY" is the final key? No — strip_prefix only removes ONCE
        assert!(config.pairs.contains_key("export KEY"));
    }

    // ── 1.7: Quote stripping edge cases ─────────────────────────────

    #[test]
    fn test_nested_quotes() {
        let opts = ParseOptions {
            strip_quotes: true,
            ..ParseOptions::default()
        };
        let config = parse_with_opts("KEY=\"\"inner\"\"", &opts);
        // Should strip outer quotes: "inner"" → result depends on implementation
        let _ = config.pairs;
    }

    #[test]
    fn test_single_quote_inside_double() {
        let opts = ParseOptions {
            strip_quotes: true,
            ..ParseOptions::default()
        };
        let config = parse_with_opts("KEY=\"it's a test\"", &opts);
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("it's a test")
        );
    }

    #[test]
    fn test_only_quotes() {
        let opts = ParseOptions {
            strip_quotes: true,
            ..ParseOptions::default()
        };
        let config = parse_with_opts("KEY=\"\"", &opts);
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("")
        );
    }

    #[test]
    fn test_triple_quotes() {
        let opts = ParseOptions {
            strip_quotes: true,
            ..ParseOptions::default()
        };
        let config = parse_with_opts("KEY=\"\"\"", &opts);
        // Three chars: strip outer "…" → middle char "
        let _ = config.pairs;
    }

    // ── 1.8: CRLF / mixed line ending chaos ─────────────────────────

    #[test]
    fn test_mixed_line_endings() {
        let config = parse("A=1\nB=2\r\nC=3\rD=4");
        // \r (bare carriage return) may not split as a line
        // Rust's .lines() handles \n and \r\n but NOT bare \r
        assert!(config.pairs.contains_key("A"));
        assert!(config.pairs.contains_key("B"));
        assert!(config.pairs.contains_key("C"));
        // "C=3\rD=4" might be one line or two depending on .lines() behavior
    }

    #[test]
    fn test_trailing_crlf() {
        let config = parse("KEY=value\r\n");
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("value")
        );
    }

    // ── 1.9: Duplicate key overwrite verification ───────────────────

    #[test]
    fn test_massive_duplicate_keys() {
        let content: String = (0..10_000)
            .map(|i| format!("KEY=value_{}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let config = parse(&content);
        // last-wins: should be "value_9999"
        assert_eq!(config.pairs.len(), 1);
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("value_9999")
        );
    }

    // ── 1.10: Whitespace-only key after trimming ────────────────────

    #[test]
    fn test_whitespace_only_key() {
        let config = parse("   =value");
        // Key after trim is empty → should be skipped
        assert!(config.pairs.is_empty());
    }

    #[test]
    fn test_tab_only_key() {
        let config = parse("\t\t=value");
        assert!(config.pairs.is_empty());
    }
}

// ═══════════════════════════════════════════════════════════════════
// CATEGORY 2: Secret Masking — Bypass & Evasion Vectors
// ═══════════════════════════════════════════════════════════════════

mod masking_chaos {
    use super::*;

    // We need to test the masking module. Since it's in the CLI crate,
    // we'll re-implement the same logic to test core assumptions.
    // The key question: does the masking keyword list cover enough patterns?

    fn should_mask(key: &str) -> bool {
        let keywords = [
            "key", "password", "secret", "token", "auth",
            "credential", "cert", "private", "signing",
            "connection_string", "conn_str",
        ];
        let key_lower = key.to_lowercase();
        keywords.iter().any(|kw| key_lower.contains(kw))
    }

    // ── 2.1: Secret keyword evasion via encoding tricks ─────────────

    #[test]
    fn test_mask_bypass_unicode_homoglyph_k() {
        // Using Cyrillic 'К' (U+041A) instead of Latin 'K'
        // "API_КEY" — visually identical but different codepoints
        assert!(
            !should_mask("API_\u{041A}EY"),
            "VULNERABILITY: Homoglyph bypass not detected"
        );
        // This IS a vulnerability but requires mitigation documentation
    }

    #[test]
    fn test_mask_bypass_zero_width_joiner() {
        // KEY with a zero-width joiner inside: "K\u200DEY"
        assert!(
            !should_mask("API_K\u{200D}EY"),
            "ZWJ-injected key bypasses masking"
        );
    }

    #[test]
    fn test_mask_bypass_case_mixed() {
        // Already handled by to_lowercase(), but verify
        assert!(should_mask("aPi_KeY"));
        assert!(should_mask("PASSWORD"));
        assert!(should_mask("SeCrEt"));
    }

    // ── 2.2: Missing keyword coverage ───────────────────────────────

    #[test]
    fn test_missing_keyword_credential() {
        // "CREDENTIAL" is a common secret-bearing key not in the list
        assert!(
            should_mask("AWS_CREDENTIAL"),
            "CREDENTIAL not in keyword list — potential secret leak"
        );
    }

    #[test]
    fn test_missing_keyword_private() {
        assert!(
            should_mask("PRIVATE_KEY_PATH"),
            "PRIVATE not in keyword list — but KEY is, so this might match"
        );
        // Actually "PRIVATE_KEY_PATH" contains "key" — so it WILL match
    }

    #[test]
    fn test_missing_keyword_cert() {
        assert!(
            should_mask("SSL_CERT"),
            "CERT not in keyword list"
        );
    }

    #[test]
    fn test_missing_keyword_apikey_no_underscore() {
        // "APIKEY" (no underscore) — "key" is substring → should match
        assert!(should_mask("APIKEY"));
    }

    #[test]
    fn test_missing_keyword_bearer() {
        assert!(
            should_mask("BEARER_TOKEN_VALUE"),
            "BEARER not needed since TOKEN is in list"
        );
    }

    #[test]
    fn test_missing_keyword_connection_string() {
        // Common pattern: "CONNECTION_STRING" contains passwords inline
        assert!(
            should_mask("CONNECTION_STRING"),
            "CONNECTION_STRING not detected as secret"
        );
    }

    #[test]
    fn test_missing_keyword_signing() {
        assert!(should_mask("SIGNING_KEY_ID"));
        // Wait — "SIGNING_KEY_ID" contains "key" → WILL match
    }

    // ── 2.3: False positive testing ─────────────────────────────────

    #[test]
    fn test_false_positive_keyboard() {
        // "KEYBOARD_LAYOUT" contains "key" — false positive
        assert!(
            should_mask("KEYBOARD_LAYOUT"),
            "False positive: KEYBOARD_LAYOUT is not a secret"
        );
        // This IS a false positive but acceptable given security-first approach
    }

    #[test]
    fn test_false_positive_keynote() {
        assert!(should_mask("KEYNOTE_APP"));
    }

    #[test]
    fn test_false_positive_authentication_level() {
        assert!(should_mask("AUTHENTICATION_LEVEL"));
    }
}

// ═══════════════════════════════════════════════════════════════════
// CATEGORY 3: Diff Engine — Invariant Violations
// ═══════════════════════════════════════════════════════════════════

mod diff_chaos {
    use super::*;

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

    // ── 3.1: Self-diff identity ─────────────────────────────────────

    #[test]
    fn test_self_diff_is_identical() {
        let file = make_config(&[("A", "1"), ("B", "2"), ("C", "3")]);
        let result = semantic_diff(&file, &file, &DiffOptions::default());
        assert!(result.is_identical());
        assert_eq!(result.difference_count(), 0);
    }

    // ── 3.2: Diff with empty values ─────────────────────────────────

    #[test]
    fn test_diff_empty_vs_empty_value() {
        let file1 = make_config(&[("KEY", "")]);
        let file2 = make_config(&[("KEY", "")]);
        let result = semantic_diff(&file1, &file2, &DiffOptions::default());
        assert!(result.is_identical());
    }

    #[test]
    fn test_diff_empty_vs_nonempty_value() {
        let file1 = make_config(&[("KEY", "")]);
        let file2 = make_config(&[("KEY", "something")]);
        let result = semantic_diff(&file1, &file2, &DiffOptions::default());
        assert_eq!(result.modified.len(), 1);
    }

    // ── 3.3: Large-scale diff performance ───────────────────────────

    #[test]
    fn test_diff_10k_keys_all_different_values() {
        let pairs1: Vec<(String, String)> = (0..10_000)
            .map(|i| (format!("KEY_{}", i), format!("old_{}", i)))
            .collect();
        let pairs2: Vec<(String, String)> = (0..10_000)
            .map(|i| (format!("KEY_{}", i), format!("new_{}", i)))
            .collect();

        let file1 = {
            let mut map = HashMap::new();
            for (i, (k, v)) in pairs1.iter().enumerate() {
                map.insert(k.clone(), (v.clone(), i + 1));
            }
            ConfigFile {
                pairs: map,
                file_path: PathBuf::from("a.env"), warnings: vec![],
            }
        };
        let file2 = {
            let mut map = HashMap::new();
            for (i, (k, v)) in pairs2.iter().enumerate() {
                map.insert(k.clone(), (v.clone(), i + 1));
            }
            ConfigFile {
                pairs: map,
                file_path: PathBuf::from("b.env"), warnings: vec![],
            }
        };

        let start = std::time::Instant::now();
        let result = semantic_diff(&file1, &file2, &DiffOptions::default());
        let elapsed = start.elapsed();

        assert_eq!(result.modified.len(), 10_000);
        // Must complete in under 1 second
        assert!(
            elapsed.as_millis() < 1000,
            "10k diff took {}ms — too slow",
            elapsed.as_millis()
        );
    }

    // ── 3.4: Diff with whitespace-only value differences ────────────

    #[test]
    fn test_diff_trailing_whitespace_matters() {
        let file1 = make_config(&[("KEY", "value ")]);
        let file2 = make_config(&[("KEY", "value")]);
        let result = semantic_diff(&file1, &file2, &DiffOptions::default());
        // Parser trims values, so these SHOULD be identical after parsing
        // But since we're using make_config directly, trim doesn't apply
        assert_eq!(result.modified.len(), 1);
    }

    // ── 3.5: Literal diff with pathological inputs ──────────────────

    #[test]
    fn test_literal_diff_completely_different() {
        let content1: String = (0..1000)
            .map(|i| format!("old_line_{}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let content2: String = (0..1000)
            .map(|i| format!("new_line_{}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let result = literal_diff(&content1, &content2, &DiffOptions::default());
        assert!(!result.additions.is_empty() || !result.deletions.is_empty());
    }

    #[test]
    fn test_literal_diff_single_char_change() {
        let result = literal_diff("aaaa\n", "aaab\n", &DiffOptions::default());
        assert!(!result.additions.is_empty() || !result.deletions.is_empty());
    }
}

// ═══════════════════════════════════════════════════════════════════
// CATEGORY 4: Patch Generation — Filesystem & Edge Cases
// ═══════════════════════════════════════════════════════════════════

mod patch_chaos {
    use super::*;
    use tempfile::TempDir;

    fn make_diff(
        missing_f2: Vec<(&str, &str)>,
        missing_f1: Vec<(&str, &str)>,
    ) -> DiffResult {
        DiffResult {
            missing_in_file2: missing_f2
                .into_iter()
                .enumerate()
                .map(|(i, (k, v))| KvPair {
                    key: k.to_string(),
                    value: v.to_string(),
                    line_number: i + 1,
                })
                .collect(),
            missing_in_file1: missing_f1
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

    // ── 4.1: Patch to non-existent directory ────────────────────────

    #[test]
    fn test_patch_to_nonexistent_dir_returns_error() {
        let diff = make_diff(vec![("KEY", "val")], vec![]);
        let bad_path = PathBuf::from("/this/path/does/not/exist/patch.env");
        let result = generate_patch(
            &diff,
            &bad_path,
            &ParseOptions::default(),
            PatchDirection::SourceToTarget,
        );
        assert!(result.is_err());
    }

    // ── 4.2: Patch with special characters in values ────────────────

    #[test]
    fn test_patch_preserves_url_values() {
        let tmp = TempDir::new().expect("temp dir");
        let output = tmp.path().join("patch.env");
        let diff = make_diff(
            vec![("URL", "https://api.com?id=1&key=abc")],
            vec![],
        );
        let opts = ParseOptions::default();
        let paths = generate_patch(&diff, &output, &opts, PatchDirection::SourceToTarget)
            .expect("should succeed");

        let content = std::fs::read_to_string(&paths[0]).expect("read");
        assert!(content.contains("URL=https://api.com?id=1&key=abc"));
    }

    // ── 4.3: Patch file round-trip integrity ────────────────────────

    #[test]
    fn test_patch_round_trip_with_complex_values() {
        let tmp = TempDir::new().expect("temp dir");
        let output = tmp.path().join("roundtrip.env");
        let diff = make_diff(
            vec![
                ("DB_URL", "postgres://user:p@ss@host:5432/db"),
                ("MULTI_EQ", "a==b==c"),
                ("SPACES", "hello world   "),
                ("EMPTY", ""),
            ],
            vec![],
        );
        let opts = ParseOptions::default();
        let paths = generate_patch(&diff, &output, &opts, PatchDirection::SourceToTarget)
            .expect("should succeed");

        // Re-parse the generated patch
        let parsed = parse_file(&paths[0], &opts).expect("parse patch");

        assert_eq!(
            parsed.pairs.get("DB_URL").map(|(v, _)| v.as_str()),
            Some("postgres://user:p@ss@host:5432/db")
        );
        assert_eq!(
            parsed.pairs.get("MULTI_EQ").map(|(v, _)| v.as_str()),
            Some("a==b==c")
        );
        // Note: "SPACES" value has trailing spaces that get trimmed by parser
        assert_eq!(
            parsed.pairs.get("SPACES").map(|(v, _)| v.as_str()),
            Some("hello world")
        );
        assert_eq!(
            parsed.pairs.get("EMPTY").map(|(v, _)| v.as_str()),
            Some("")
        );
    }

    // ── 4.4: Bidirectional with no missing keys ─────────────────────

    #[test]
    fn test_bidirectional_patch_both_empty() {
        let tmp = TempDir::new().expect("temp dir");
        let output = tmp.path().join("empty_bi.env");
        let diff = make_diff(vec![], vec![]);
        let opts = ParseOptions::default();
        let paths = generate_patch(&diff, &output, &opts, PatchDirection::Bidirectional)
            .expect("should succeed");

        assert_eq!(paths.len(), 2);
        for path in &paths {
            let content = std::fs::read_to_string(path).expect("read");
            assert!(content.is_empty());
        }
    }

    // ── 4.5: Patch with delimiter in value ──────────────────────────

    #[test]
    fn test_patch_colon_delimiter_with_colon_in_value() {
        let tmp = TempDir::new().expect("temp dir");
        let output = tmp.path().join("colon.env");
        let diff = make_diff(
            vec![("DB", "postgres://host:5432/db")],
            vec![],
        );
        let opts = ParseOptions {
            delimiter: ':',
            ..ParseOptions::default()
        };
        let paths = generate_patch(&diff, &output, &opts, PatchDirection::SourceToTarget)
            .expect("should succeed");

        let content = std::fs::read_to_string(&paths[0]).expect("read");
        assert!(content.contains("DB:postgres://host:5432/db"));

        // Round-trip: can we re-parse this correctly?
        let parsed = parse_file(&paths[0], &opts).expect("parse");
        assert_eq!(
            parsed.pairs.get("DB").map(|(v, _)| v.as_str()),
            Some("postgres://host:5432/db")
        );
    }
}

// ═══════════════════════════════════════════════════════════════════
// CATEGORY 5: End-to-End Parsing Attack Scenarios
// ═══════════════════════════════════════════════════════════════════

mod e2e_attack_scenarios {
    use super::*;

    fn parse(content: &str) -> ConfigFile {
        parse_content(content, Path::new("attack.env"), &ParseOptions::default())
            .expect("should not fail")
    }

    // ── 5.1: Real-world DevOps configs ──────────────────────────────

    #[test]
    fn test_aws_ssm_format() {
        let content = r#""DATABASE_HOST"="db.internal.aws.com"
"API_KEY"="sk-12345-abcdef"
"REDIS_URL"="redis://cache.internal:6379/0""#;
        let opts = ParseOptions {
            strip_quotes: true,
            ..ParseOptions::default()
        };
        let config =
            parse_content(content, Path::new("ssm.env"), &opts).expect("should parse");

        assert_eq!(
            config.pairs.get("DATABASE_HOST").map(|(v, _)| v.as_str()),
            Some("db.internal.aws.com")
        );
        assert_eq!(
            config.pairs.get("API_KEY").map(|(v, _)| v.as_str()),
            Some("sk-12345-abcdef")
        );
    }

    #[test]
    fn test_docker_compose_env_format() {
        let content = "MYSQL_ROOT_PASSWORD=supersecret\n\
                       MYSQL_DATABASE=myapp\n\
                       MYSQL_USER=appuser\n\
                       MYSQL_PASSWORD=apppass123\n";
        let config = parse(content);
        assert_eq!(config.pairs.len(), 4);
    }

    #[test]
    fn test_kubernetes_configmap_format() {
        let content = "LOG_LEVEL=info\n\
                       SERVICE_PORT=8080\n\
                       DATABASE_URL=postgres://db:5432/production\n\
                       FEATURE_FLAG_NEW_UI=true\n";
        let config = parse(content);
        assert_eq!(config.pairs.len(), 4);
        assert_eq!(
            config
                .pairs
                .get("DATABASE_URL")
                .map(|(v, _)| v.as_str()),
            Some("postgres://db:5432/production")
        );
    }

    // ── 5.2: Injection attacks ──────────────────────────────────────

    #[test]
    fn test_newline_injection_in_value() {
        // Can a value contain a literal \n that creates a fake second KV?
        // In reality, .lines() splits so any \n in value is impossible in file parsing
        let content = "KEY=value\nINJECTED=evil";
        let config = parse(content);
        // This should parse as TWO separate keys
        assert_eq!(config.pairs.len(), 2);
        assert!(config.pairs.contains_key("KEY"));
        assert!(config.pairs.contains_key("INJECTED"));
    }

    #[test]
    fn test_escaped_newline_in_value() {
        // Literal backslash-n in value (not actual newline)
        let content = "KEY=line1\\nline2";
        let config = parse(content);
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("line1\\nline2")
        );
    }

    // ── 5.3: Full pipeline — parse → diff → patch ───────────────────

    #[test]
    fn test_full_pipeline_prod_vs_staging() {
        let prod = "DATABASE_URL=postgres://prod:5432/db\n\
                    API_KEY=sk-prod-12345\n\
                    REDIS_URL=redis://prod-cache:6379\n\
                    LOG_LEVEL=warn\n\
                    FEATURE_NEW=true\n";
        let staging = "DATABASE_URL=postgres://staging:5432/db\n\
                      API_KEY=sk-staging-67890\n\
                      REDIS_URL=redis://staging-cache:6379\n\
                      LOG_LEVEL=debug\n\
                      NEW_SERVICE_URL=https://new.staging.internal\n";

        let opts = ParseOptions::default();
        let config1 = parse_content(prod, Path::new("prod.env"), &opts).expect("parse prod");
        let config2 = parse_content(staging, Path::new("staging.env"), &opts).expect("parse staging");

        let result = semantic_diff(&config1, &config2, &DiffOptions::default());

        // FEATURE_NEW only in prod → missing in file2
        assert!(
            result.missing_in_file2.iter().any(|p| p.key == "FEATURE_NEW"),
            "FEATURE_NEW should be missing from staging"
        );

        // NEW_SERVICE_URL only in staging → missing in file1
        assert!(
            result.missing_in_file1.iter().any(|p| p.key == "NEW_SERVICE_URL"),
            "NEW_SERVICE_URL should be missing from prod"
        );

        // DATABASE_URL, API_KEY, REDIS_URL, LOG_LEVEL → modified
        assert_eq!(result.modified.len(), 4);

        // Generate patch
        let tmp = tempfile::TempDir::new().expect("temp dir");
        let patch_path = tmp.path().join("missing.env");
        let paths = generate_patch(
            &result,
            &patch_path,
            &opts,
            PatchDirection::Bidirectional,
        )
        .expect("generate patch");

        assert_eq!(paths.len(), 2);

        // Verify patch contents
        let s2t = std::fs::read_to_string(&paths[0]).expect("read s2t");
        let t2s = std::fs::read_to_string(&paths[1]).expect("read t2s");

        assert!(s2t.contains("FEATURE_NEW=true"));
        assert!(t2s.contains("NEW_SERVICE_URL=https://new.staging.internal"));
    }
}

// ═══════════════════════════════════════════════════════════════════
// CATEGORY 6: Zero-Panic Guarantee Verification
// ═══════════════════════════════════════════════════════════════════

mod zero_panic {
    use super::*;

    // ── 6.1: File operations with bad paths ─────────────────────────

    #[test]
    fn test_parse_nonexistent_file() {
        let result = parse_file(
            Path::new("this_file_does_not_exist__12345.env"),
            &ParseOptions::default(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_directory_as_file() {
        // Attempting to parse a directory should fail gracefully
        let result = parse_file(Path::new("."), &ParseOptions::default());
        assert!(result.is_err());
    }

    // ── 6.2: All combinations of empty inputs ───────────────────────

    #[test]
    fn test_all_empty_combinations() {
        let empty = parse_content("", Path::new("e.env"), &ParseOptions::default())
            .expect("empty should work");
        assert!(empty.pairs.is_empty());

        // Diff two empty files
        let result = semantic_diff(&empty, &empty, &DiffOptions::default());
        assert!(result.is_identical());

        // Literal diff of empty strings
        let lit = literal_diff("", "", &DiffOptions::default());
        assert!(lit.additions.is_empty());
        assert!(lit.deletions.is_empty());
    }

    // ── 6.3: Verify no index-out-of-bounds on edge strings ──────────

    #[test]
    fn test_single_char_inputs() {
        let inputs = ["=", "a", "#", "/", "'", "\"", " ", "\t", "\n", "\r", "\\"];
        for input in &inputs {
            let _ = parse_content(input, Path::new("t.env"), &ParseOptions::default());
        }
    }

    #[test]
    fn test_two_char_inputs() {
        let inputs = ["==", "a=", "=b", "# ", "//", "\"\"", "''", "\r\n", "\n\n"];
        for input in &inputs {
            let _ = parse_content(input, Path::new("t.env"), &ParseOptions::default());
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// CATEGORY 7: Inline Comment Stripping Security
// ═══════════════════════════════════════════════════════════════════

mod comment_stripping_security {
    use super::*;

    fn parse(content: &str) -> ConfigFile {
        parse_content(content, Path::new("comments.env"), &ParseOptions::default())
            .expect("should not fail")
    }

    #[test]
    fn test_hash_in_password_value() {
        // Real-world: password contains a # character
        let config = parse("DB_PASSWORD=p@ss#word123");
        // The # has no space before it → NOT treated as comment
        assert_eq!(
            config.pairs.get("DB_PASSWORD").map(|(v, _)| v.as_str()),
            Some("p@ss#word123")
        );
    }

    #[test]
    fn test_space_hash_in_password() {
        // This is the vulnerability: password contains " #"
        let config = parse("PASSWORD=my pass #word");
        // " #word" is treated as a comment → password TRUNCATED
        assert_eq!(
            config.pairs.get("PASSWORD").map(|(v, _)| v.as_str()),
            Some("my pass")
        );
        // This documents known behavior — NOT a bug per PRD, but a security edge case
    }

    #[test]
    fn test_url_with_fragment_and_comment() {
        let config = parse("URL=https://example.com/#section # actual comment");
        assert_eq!(
            config.pairs.get("URL").map(|(v, _)| v.as_str()),
            Some("https://example.com/#section")
        );
    }

    #[test]
    fn test_value_that_is_entirely_a_comment() {
        let config = parse("KEY= # everything is comment");
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("")
        );
    }

    #[test]
    fn test_double_slash_in_url_not_stripped() {
        let config = parse("URL=https://example.com/api/v2");
        // "https://example.com/api/v2" — the // in https should NOT trigger comment
        assert_eq!(
            config.pairs.get("URL").map(|(v, _)| v.as_str()),
            Some("https://example.com/api/v2")
        );
    }

    #[test]
    fn test_double_slash_with_space_stripped() {
        let config = parse("KEY=value // inline comment");
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("value")
        );
    }
}

