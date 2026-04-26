//! Configuration file parser for key-value pair formats.
//!
//! Supports custom delimiters, quote stripping, comment filtering,
//! and the `export` prefix used by shell environments. All parsing
//! is done in-memory with zero temporary files.
//!
//! # Key Design Decisions
//!
//! - **`split_once`** is used instead of `split` to preserve URLs and
//!   values containing the delimiter character.
//! - **No regex** — all parsing uses standard `&str` methods for speed.
//! - **No `.unwrap()` / `.expect()`** — all code paths are panic-free.

use crate::error::{QarenError, QarenResult};
use crate::types::{ConfigFile, ParseOptions, ParseWarning};
use std::collections::HashMap;
use std::path::Path;

/// Heuristically detect the delimiter used in a configuration file.
///
/// Scans non-empty, non-comment lines and counts occurrences of `=` and
/// `: ` (colon + space, to avoid false positives from URLs like
/// `postgres://host:5432`).  Returns whichever delimiter appears in more
/// lines; falls back to `'='` on a tie or empty input.
///
/// # Examples
///
/// ```
/// # use qaren_core::detect_delimiter;
/// assert_eq!(detect_delimiter("KEY=value\nDB=host"), '=');
/// assert_eq!(detect_delimiter("key: value\ndb: host"), ':');
/// ```
pub fn detect_delimiter(content: &str) -> char {
    let mut eq_count: usize = 0;
    let mut colon_count: usize = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        // Skip empty lines and comment lines
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("//")
        {
            continue;
        }
        if trimmed.contains('=') {
            eq_count += 1;
        }
        // `: ` (colon-space) distinguishes YAML/PM2 format from URLs
        if trimmed.contains(": ") {
            colon_count += 1;
        }
    }

    if colon_count > eq_count {
        ':'
    } else {
        '='
    }
}

const MAX_FILE_SIZE: u64 = 512 * 1024 * 1024; // 512MB

/// Parse a configuration file from disk with the given options.
///
/// Reads the file into memory and delegates to [`parse_content`].
/// Returns a contextual error if the file cannot be read or is too large.
pub fn parse_file(
    file_path: &Path,
    options: &ParseOptions,
) -> QarenResult<ConfigFile> {
    let metadata = std::fs::metadata(file_path)
        .map_err(|e| QarenError::from_io_with_path(e, file_path.to_path_buf()))?;
        
    if metadata.len() > MAX_FILE_SIZE {
        return Err(QarenError::ParseError {
            path: file_path.to_path_buf(),
            line: 0,
            reason: format!("File size exceeds {}MB limit", MAX_FILE_SIZE / 1_000_000),
        });
    }

    let content = std::fs::read_to_string(file_path)
        .map_err(|e| QarenError::from_io_with_path(e, file_path.to_path_buf()))?;
        
    #[allow(unused_mut)]
    let mut config = parse_content(&content, file_path, options)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        if mode & 0o077 != 0 {
            config.warnings.push(ParseWarning {
                key: None,
                message: format!("Insecure file permissions ({:o})", mode & 0o777),
            });
        }
    }

    Ok(config)
}


/// Parse configuration content from an in-memory string.
///
/// Iterates line-by-line, skipping empty lines and comments, and
/// extracts key-value pairs using the first occurrence of the delimiter.
pub fn parse_content(
    content: &str,
    file_path: &Path,
    options: &ParseOptions,
) -> QarenResult<ConfigFile> {
    // Strip UTF-8 BOM if present (Finding 7: Windows editors add BOM)
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(content);
    let mut pairs = HashMap::new();
    let mut warnings = Vec::new();
    let file_label = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("file");

    let lines: Vec<_> = content.lines().enumerate().collect();
    let mut it = lines.into_iter().peekable();

    while let Some((line_idx, line)) = it.next() {
        let line_number = line_idx + 1; // 1-indexed for user display

        // Skip empty lines and full-line comments
        if should_skip_line(line, options) {
            continue;
        }

        // Parse key-value pair; malformed lines are silently skipped
        if let Some((key, mut value)) = parse_line(line, options) {
            // Multi-line support: if value ends with \, join with next line(s)
            while value.ends_with('\\') {
                if let Some((_, next_line)) = it.peek() {
                    // Peeked line exists, join it
                    value.pop(); // Remove the continuation backslash
                    value.push_str(next_line.trim());
                    it.next(); // Consume the joined line
                } else {
                    // No next line, keep the backslash as a literal
                    break;
                }
            }

            if let Some((_, old_line)) = pairs.get(&key) {
                let msg = format!("duplicate key '{}' detected in {} (overwriting line {} with line {})", key, file_label, old_line, line_number);
                warnings.push(ParseWarning {
                    key: Some(key.clone()),
                    message: msg,
                });
            }
            pairs.insert(key, (value, line_number));
        }
    }

    Ok(ConfigFile {
        pairs,
        file_path: file_path.to_path_buf(),
        warnings,
    })
}

/// Check if a line should be skipped entirely (empty or full-line comment).
fn should_skip_line(line: &str, options: &ParseOptions) -> bool {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return true;
    }

    for prefix in &options.comment_prefixes {
        // Finding 6: skip empty prefixes to prevent matching all lines
        if prefix.is_empty() {
            continue;
        }
        if trimmed.starts_with(prefix.as_str()) {
            return true;
        }
    }

    false
}

/// Parse a single line into a key-value pair.
///
/// Uses `split_once` to split at the **first** delimiter only, which is
/// critical for preserving URLs like `https://api.com?id=1&key=value`.
///
/// Returns `None` if the line has no delimiter or the key is empty after
/// processing (e.g., a line that is just `=`).
fn parse_line(
    line: &str,
    options: &ParseOptions,
) -> Option<(String, String)> {
    // Split at FIRST delimiter only
    let (key_raw, value_raw) = line.split_once(options.delimiter)?;

    // Process the key: trim, strip `export ` prefix, optionally strip quotes
    let key = process_key(key_raw, options);

    // Skip if key is empty after processing (e.g., line was just `=`)
    if key.is_empty() {
        return None;
    }

    // Process the value: trim, strip inline comments, optionally strip quotes
    let value = process_value(value_raw, options);

    Some((key, value))
}

/// Process a key token: trim whitespace, strip `export ` prefix, optionally
/// strip surrounding quotes.
fn process_key(token: &str, options: &ParseOptions) -> String {
    let trimmed = token.trim();

    // Strip the shell `export ` prefix (lowercase only, POSIX convention)
    let without_export = trimmed.strip_prefix("export ").unwrap_or(trimmed);
    let without_export = without_export.trim();

    // Finding 1: A bare `export` with no actual key name is not valid
    if without_export == "export" || without_export.is_empty() {
        return String::new();
    }

    if options.strip_quotes {
        strip_surrounding_quotes(without_export)
    } else {
        without_export.to_string()
    }
}

/// Process a value token: trim whitespace, strip inline comments, optionally
/// strip surrounding quotes.
fn process_value(token: &str, options: &ParseOptions) -> String {
    let trimmed = token.trim();

    // Finding 2: If the entire value starts with a comment marker, treat as empty
    for prefix in &options.comment_prefixes {
        if !prefix.is_empty() && trimmed.starts_with(prefix.as_str()) {
            return String::new();
        }
    }

    // Strip inline comments first (before quote stripping)
    let without_comment = strip_inline_comment(trimmed, options);
    let without_comment = without_comment.trim();

    if options.strip_quotes {
        strip_surrounding_quotes(without_comment)
    } else {
        without_comment.to_string()
    }
}

/// Strip an inline comment from a value string.
///
/// Uses ` # ` (space-hash) and ` // ` (space-double-slash) as delimiters
/// to avoid false positives with URLs containing `#` fragments
/// (e.g., `https://example.com/#section`).
///
/// Only the **first** matching comment marker is used; everything after
/// it (inclusive) is removed.
fn strip_inline_comment<'a>(value: &'a str, options: &ParseOptions) -> &'a str {
    let mut earliest_pos: Option<usize> = None;

    for prefix in &options.comment_prefixes {
        // Look for ` #` or ` //` (space before the comment marker)
        let marker = format!(" {}", prefix);
        if let Some(pos) = value.find(&marker) {
            match earliest_pos {
                Some(current) if pos < current => {
                    earliest_pos = Some(pos);
                }
                None => {
                    earliest_pos = Some(pos);
                }
                _ => {}
            }
        }
    }

    match earliest_pos {
        Some(pos) => &value[..pos],
        None => value,
    }
}

/// Strip surrounding quotes from a string.
///
/// Only removes quotes that **surround the entire string** — internal
/// quotes are preserved. For example:
///
/// - `"hello"` → `hello`
/// - `'hello'` → `hello`
/// - `'John "The Boss" Doe'` → `John "The Boss" Doe`
/// - `"unbalanced` → `"unbalanced` (no stripping)
/// - `""` → `` (empty string)
///
/// This function is **panic-free**: no `.unwrap()` or `.expect()`.
fn strip_surrounding_quotes(s: &str) -> String {
    let bytes = s.as_bytes();
    let len = bytes.len();

    if len < 2 {
        return s.to_string();
    }

    let first = bytes[0];
    let last = bytes[len - 1];

    // Check for matching surrounding quotes (double or single)
    if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
        s[1..len - 1].to_string()
    } else {
        s.to_string()
    }
}

// ─────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Helper to parse a content string with default options.
    fn parse_default(content: &str) -> ConfigFile {
        parse_content(content, &PathBuf::from("test.env"), &ParseOptions::default())
            .expect("parse_content should not fail on valid input")
    }

    /// Helper to parse with custom options.
    fn parse_with(content: &str, options: &ParseOptions) -> ConfigFile {
        parse_content(content, &PathBuf::from("test.env"), options)
            .expect("parse_content should not fail on valid input")
    }

    // ── 3.7: Unit tests for parser edge cases ───────────────────────

    #[test]
    fn test_empty_file() {
        let config = parse_default("");
        assert!(config.pairs.is_empty());
    }

    #[test]
    fn test_only_comments() {
        let config = parse_default("# comment\n// another comment");
        assert!(config.pairs.is_empty());
    }

    #[test]
    fn test_only_whitespace() {
        let config = parse_default("   \n  \n\t\n");
        assert!(config.pairs.is_empty());
    }

    #[test]
    fn test_basic_kv_pair() {
        let config = parse_default("KEY=value");
        assert_eq!(config.pairs.get("KEY").map(|(v, _)| v.as_str()), Some("value"));
    }

    #[test]
    fn test_multiple_kv_pairs() {
        let config = parse_default("A=1\nB=2\nC=3");
        assert_eq!(config.pairs.len(), 3);
        assert_eq!(config.pairs.get("A").map(|(v, _)| v.as_str()), Some("1"));
        assert_eq!(config.pairs.get("B").map(|(v, _)| v.as_str()), Some("2"));
        assert_eq!(config.pairs.get("C").map(|(v, _)| v.as_str()), Some("3"));
    }

    #[test]
    fn test_url_preservation() {
        let config = parse_default("URL=https://api.com?id=1&key=value");
        assert_eq!(
            config.pairs.get("URL").map(|(v, _)| v.as_str()),
            Some("https://api.com?id=1&key=value")
        );
    }

    #[test]
    fn test_quote_stripping_basic() {
        let opts = ParseOptions {
            strip_quotes: true,
            ..ParseOptions::default()
        };
        let config = parse_with("\"API_KEY\"=\"abc123\"", &opts);
        assert_eq!(config.pairs.get("API_KEY").map(|(v, _)| v.as_str()), Some("abc123"));
    }

    #[test]
    fn test_quote_stripping_internal_quotes_preserved() {
        let opts = ParseOptions {
            strip_quotes: true,
            ..ParseOptions::default()
        };
        let config = parse_with("NAME='John \"The Boss\" Doe'", &opts);
        assert_eq!(
            config.pairs.get("NAME").map(|(v, _)| v.as_str()),
            Some("John \"The Boss\" Doe")
        );
    }

    #[test]
    fn test_pm2_format() {
        let opts = ParseOptions {
            delimiter: ':',
            strip_quotes: true,
            ..ParseOptions::default()
        };
        let config = parse_with("\"DATABASE_URL\":\"postgres://host:5432/db\"", &opts);
        assert_eq!(
            config.pairs.get("DATABASE_URL").map(|(v, _)| v.as_str()),
            Some("postgres://host:5432/db")
        );
    }

    #[test]
    fn test_shell_export() {
        let config = parse_default("export KEY=value");
        assert_eq!(config.pairs.get("KEY").map(|(v, _)| v.as_str()), Some("value"));
    }

    #[test]
    fn test_shell_export_with_quotes() {
        let opts = ParseOptions {
            strip_quotes: true,
            ..ParseOptions::default()
        };
        let config = parse_with("export KEY=\"value with spaces\"", &opts);
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("value with spaces")
        );
    }

    #[test]
    fn test_unbalanced_quotes() {
        let opts = ParseOptions {
            strip_quotes: true,
            ..ParseOptions::default()
        };
        let config = parse_with("KEY=\"unbalanced", &opts);
        // Unbalanced quotes should NOT be stripped
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("\"unbalanced")
        );
    }

    #[test]
    fn test_only_delimiter() {
        let config = parse_default("=");
        // Key is empty after processing → line should be skipped
        assert!(config.pairs.is_empty());
    }

    #[test]
    fn test_delimiter_with_value_only() {
        let config = parse_default("=value");
        // Key is empty → skipped
        assert!(config.pairs.is_empty());
    }

    #[test]
    fn test_custom_delimiter_colon() {
        let opts = ParseOptions {
            delimiter: ':',
            ..ParseOptions::default()
        };
        let config = parse_with("KEY:value", &opts);
        assert_eq!(config.pairs.get("KEY").map(|(v, _)| v.as_str()), Some("value"));
    }

    #[test]
    fn test_custom_delimiter_space() {
        let opts = ParseOptions {
            delimiter: ' ',
            ..ParseOptions::default()
        };
        let config = parse_with("KEY value", &opts);
        assert_eq!(config.pairs.get("KEY").map(|(v, _)| v.as_str()), Some("value"));
    }

    #[test]
    fn test_empty_value() {
        let config = parse_default("KEY=");
        assert_eq!(config.pairs.get("KEY").map(|(v, _)| v.as_str()), Some(""));
    }

    #[test]
    fn test_line_without_delimiter_is_skipped() {
        let config = parse_default("no_delimiter_here\nKEY=value");
        assert_eq!(config.pairs.len(), 1);
        assert_eq!(config.pairs.get("KEY").map(|(v, _)| v.as_str()), Some("value"));
    }

    #[test]
    fn test_whitespace_around_key_and_value() {
        let config = parse_default("  KEY  =  value  ");
        assert_eq!(config.pairs.get("KEY").map(|(v, _)| v.as_str()), Some("value"));
    }

    #[test]
    fn test_inline_comment_hash() {
        let config = parse_default("KEY=value # this is a comment");
        assert_eq!(config.pairs.get("KEY").map(|(v, _)| v.as_str()), Some("value"));
    }

    #[test]
    fn test_inline_comment_double_slash() {
        let config = parse_default("KEY=value // this is a comment");
        assert_eq!(config.pairs.get("KEY").map(|(v, _)| v.as_str()), Some("value"));
    }

    #[test]
    fn test_value_with_hash_in_url_no_space() {
        // Hash without preceding space should NOT be treated as comment
        let config = parse_default("URL=https://example.com/#section");
        assert_eq!(
            config.pairs.get("URL").map(|(v, _)| v.as_str()),
            Some("https://example.com/#section")
        );
    }

    #[test]
    fn test_comment_line_with_leading_whitespace() {
        let config = parse_default("  # indented comment\nKEY=value");
        assert_eq!(config.pairs.len(), 1);
    }

    #[test]
    fn test_line_numbers_are_1_indexed() {
        let config = parse_default("A=1\n# comment\nB=2");
        assert_eq!(config.pairs.get("A").map(|(_, ln)| *ln), Some(1));
        assert_eq!(config.pairs.get("B").map(|(_, ln)| *ln), Some(3));
    }

    #[test]
    fn test_duplicate_keys_last_wins() {
        let config = parse_default("KEY=first\nKEY=second");
        assert_eq!(config.pairs.get("KEY").map(|(v, _)| v.as_str()), Some("second"));
    }

    #[test]
    fn test_value_with_spaces() {
        let config = parse_default("GREETING=hello world");
        assert_eq!(
            config.pairs.get("GREETING").map(|(v, _)| v.as_str()),
            Some("hello world")
        );
    }

    #[test]
    fn test_value_with_equals_in_base64() {
        let config = parse_default("TOKEN=eyJhbGciOi==");
        assert_eq!(
            config.pairs.get("TOKEN").map(|(v, _)| v.as_str()),
            Some("eyJhbGciOi==")
        );
    }

    #[test]
    fn test_file_path_preserved() {
        let path = PathBuf::from("/custom/path.env");
        let config = parse_content("KEY=value", &path, &ParseOptions::default())
            .expect("should not fail");
        assert_eq!(config.file_path, path);
    }

    #[test]
    fn test_mixed_valid_and_invalid_lines() {
        let content = "VALID1=one\nmalformed\n\n# comment\nVALID2=two\n=empty_key\nVALID3=three";
        let config = parse_default(content);
        assert_eq!(config.pairs.len(), 3);
        assert!(config.pairs.contains_key("VALID1"));
        assert!(config.pairs.contains_key("VALID2"));
        assert!(config.pairs.contains_key("VALID3"));
    }

    #[test]
    fn test_crlf_line_endings() {
        let config = parse_default("A=1\r\nB=2\r\n");
        assert_eq!(config.pairs.len(), 2);
        assert_eq!(config.pairs.get("A").map(|(v, _)| v.as_str()), Some("1"));
        assert_eq!(config.pairs.get("B").map(|(v, _)| v.as_str()), Some("2"));
    }

    // ── strip_surrounding_quotes unit tests ─────────────────────────

    #[test]
    fn test_strip_quotes_double() {
        assert_eq!(strip_surrounding_quotes("\"hello\""), "hello");
    }

    #[test]
    fn test_strip_quotes_single() {
        assert_eq!(strip_surrounding_quotes("'hello'"), "hello");
    }

    #[test]
    fn test_strip_quotes_empty_quoted() {
        assert_eq!(strip_surrounding_quotes("\"\""), "");
    }

    #[test]
    fn test_strip_quotes_mismatched() {
        assert_eq!(strip_surrounding_quotes("\"hello'"), "\"hello'");
    }

    #[test]
    fn test_strip_quotes_single_char() {
        assert_eq!(strip_surrounding_quotes("\""), "\"");
    }

    #[test]
    fn test_strip_quotes_empty() {
        assert_eq!(strip_surrounding_quotes(""), "");
    }

    #[test]
    fn test_strip_quotes_no_quotes() {
        assert_eq!(strip_surrounding_quotes("hello"), "hello");
    }

    // ── strip_inline_comment unit tests ─────────────────────────────

    #[test]
    fn test_strip_inline_comment_basic_hash() {
        let opts = ParseOptions::default();
        assert_eq!(strip_inline_comment("value # comment", &opts), "value");
    }

    #[test]
    fn test_strip_inline_comment_basic_double_slash() {
        let opts = ParseOptions::default();
        assert_eq!(strip_inline_comment("value // comment", &opts), "value");
    }

    #[test]
    fn test_strip_inline_comment_no_comment() {
        let opts = ParseOptions::default();
        assert_eq!(strip_inline_comment("value", &opts), "value");
    }

    #[test]
    fn test_strip_inline_comment_hash_without_space() {
        let opts = ParseOptions::default();
        // Hash without space before it — not treated as comment
        assert_eq!(strip_inline_comment("value#notacomment", &opts), "value#notacomment");
    }

    #[test]
    fn test_strip_inline_comment_url_fragment() {
        let opts = ParseOptions::default();
        assert_eq!(
            strip_inline_comment("https://example.com/#section", &opts),
            "https://example.com/#section"
        );
    }

    // ── Chaos audit finding tests ───────────────────────────────────

    #[test]
    fn test_finding1_export_only_line() {
        // Finding 1: `export =value` should skip — bare "export" is not a valid key
        let config = parse_default("export =value");
        assert!(
            config.pairs.is_empty(),
            "export =value should be skipped (bare 'export' is not a key)"
        );
    }

    #[test]
    fn test_finding2_value_starts_with_comment_marker() {
        // Finding 2: `KEY= # comment` → after trim, value is `# comment` → empty
        let config = parse_default("KEY= # this is a comment");
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some(""),
            "Value starting with comment marker should be treated as empty"
        );
    }

    #[test]
    fn test_finding2_value_is_only_hash() {
        let config = parse_default("KEY=#FF0000");
        // A value like #FF0000 starts with # so it's treated as a comment → empty
        assert_eq!(
            config.pairs.get("KEY").map(|(v, _)| v.as_str()),
            Some("")
        );
    }

    #[test]
    fn test_finding6_empty_comment_prefix() {
        // Finding 6: empty string in comment_prefixes should not skip all lines
        let opts = ParseOptions {
            comment_prefixes: vec!["".to_string(), "#".to_string()],
            ..ParseOptions::default()
        };
        let config = parse_with("KEY=value\n# comment\nKEY2=value2", &opts);
        assert_eq!(config.pairs.len(), 2, "Empty prefix should not skip all lines");
        assert!(config.pairs.contains_key("KEY"));
        assert!(config.pairs.contains_key("KEY2"));
    }

    #[test]
    fn test_finding7_bom_stripped() {
        // Finding 7: UTF-8 BOM at start of file should be stripped
        let content = "\u{FEFF}KEY=value";
        let config = parse_default(content);
        assert!(
            config.pairs.contains_key("KEY"),
            "BOM should be stripped, key should be 'KEY' not '\\u{{FEFF}}KEY'"
        );
    }

    #[test]
    fn test_finding7_bom_with_multiple_lines() {
        let content = "\u{FEFF}FIRST=one\nSECOND=two";
        let config = parse_default(content);
        assert_eq!(config.pairs.len(), 2);
        assert!(config.pairs.contains_key("FIRST"));
        assert!(config.pairs.contains_key("SECOND"));
    }
}

// ─────────────────────────────────────────────────────────────────────
// Property-based tests
// ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use std::path::PathBuf;

    /// Check if a character is safe for use in test inputs.
    /// Filters out control chars, null bytes, non-ASCII whitespace, etc.
    fn is_safe_char(c: char) -> bool {
        !c.is_control() && c.is_ascii() && c != '\0'
    }

    /// A valid key: non-empty, ASCII-only, no delimiter, no newlines, no comment prefixes
    fn sanitize_key(s: &str, delimiter: char) -> Option<String> {
        let clean: String = s
            .chars()
            .filter(|c| is_safe_char(*c) && *c != delimiter && *c != '#' && *c != '/' && *c != '"' && *c != '\'')
            .collect();
        let clean = clean.trim().to_string();
        if clean.is_empty() || clean.starts_with("export ") || clean == "export" {
            return None;
        }
        Some(clean)
    }

    /// A value that doesn't contain newlines, control chars, or the inline comment pattern.
    /// Also avoid ending with a backslash to prevent accidental multi-line joining in property tests.
    fn sanitize_value(s: &str) -> String {
        let clean: String = s.chars()
            .filter(|c| is_safe_char(*c) && *c != '#' && *c != '/')
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

    // ── Property 2: First-Delimiter-Only Splitting ──────────────────

    #[quickcheck]
    fn prop_first_delimiter_only(key: String, extra_parts: Vec<String>) -> TestResult {
        let key = match sanitize_key(&key, '=') {
            Some(k) => k,
            None => return TestResult::discard(),
        };

        if extra_parts.is_empty() {
            return TestResult::discard();
        }

        // Build a value that contains additional '=' characters
        let value_parts: Vec<String> = extra_parts
            .iter()
            .map(|p| sanitize_value(p))
            .collect();
        let value = value_parts.join("=");
        let line = format!("{}={}", key, value);

        // Count expected delimiters in value
        let total_delimiters = line.matches('=').count();
        let expected_value_delimiters = total_delimiters - 1; // first one is the split point

        let config = parse_content(&line, &PathBuf::from("test"), &ParseOptions::default())
            .expect("should not fail");

        match config.pairs.get(&key) {
            Some((parsed_value, _)) => {
                let actual_delimiters = parsed_value.matches('=').count();
                TestResult::from_bool(actual_delimiters == expected_value_delimiters)
            }
            None => TestResult::failed(),
        }
    }

    // ── Property 3: Quote Preservation Without Stripping ────────────

    #[quickcheck]
    fn prop_quote_preservation(key: String, value: String) -> TestResult {
        let key = match sanitize_key(&key, '=') {
            Some(k) => k,
            None => return TestResult::discard(),
        };
        let value = sanitize_value(&value);
        // The parser trims values, so we need to compare against trimmed
        let expected = value.trim().to_string();
        if expected.is_empty() {
            return TestResult::discard();
        }

        // Parse WITHOUT strip_quotes — all quotes should be preserved
        let line = format!("{}={}", key, value);
        let opts = ParseOptions {
            strip_quotes: false,
            ..ParseOptions::default()
        };
        let config = parse_content(&line, &PathBuf::from("test"), &opts)
            .expect("should not fail");

        match config.pairs.get(&key) {
            Some((parsed_value, _)) => {
                TestResult::from_bool(parsed_value == &expected)
            }
            None => TestResult::failed(),
        }
    }

    // ── Property 4: Inline Comment Removal ──────────────────────────

    #[quickcheck]
    fn prop_inline_comment_removal(key: String, value: String, comment: String) -> TestResult {
        let key = match sanitize_key(&key, '=') {
            Some(k) => k,
            None => return TestResult::discard(),
        };

        // Clean up value to not contain comment markers, control chars, or newlines
        let value = sanitize_value(&value);

        // The parser trims values, so compare against trimmed
        let expected = value.trim().to_string();
        if expected.is_empty() {
            return TestResult::discard();
        }

        let comment: String = comment
            .chars()
            .filter(|c| is_safe_char(*c))
            .collect();

        // Build line with inline comment: KEY=value # comment
        let line = format!("{}={} # {}", key, expected, comment);
        let config = parse_content(&line, &PathBuf::from("test"), &ParseOptions::default())
            .expect("should not fail");

        match config.pairs.get(&key) {
            Some((parsed_value, _)) => {
                // The parsed value should NOT contain the comment text
                // and should equal the original value
                TestResult::from_bool(parsed_value == &expected)
            }
            None => TestResult::failed(),
        }
    }

    // ── Property 8: Empty Value Handling ─────────────────────────────

    #[quickcheck]
    fn prop_empty_value(key: String) -> TestResult {
        let key = match sanitize_key(&key, '=') {
            Some(k) => k,
            None => return TestResult::discard(),
        };

        // Build line: KEY= (delimiter but no value)
        let line = format!("{}=", key);
        let config = parse_content(&line, &PathBuf::from("test"), &ParseOptions::default())
            .expect("should not fail");

        match config.pairs.get(&key) {
            Some((parsed_value, _)) => {
                TestResult::from_bool(parsed_value.is_empty())
            }
            None => TestResult::failed(),
        }
    }

    // ── Property 9: Parser Robustness and Error Recovery ────────────

    #[quickcheck]
    fn prop_parser_robustness(
        valid_pairs: Vec<(String, String)>,
        malformed: Vec<String>,
    ) -> TestResult {
        // Build valid lines
        let mut valid_keys = Vec::new();
        let mut lines = Vec::new();

        for (k, v) in &valid_pairs {
            let key = match sanitize_key(k, '=') {
                Some(k) => k,
                None => continue,
            };
            let value = sanitize_value(v);
            valid_keys.push(key.clone());
            lines.push(format!("{}={}", key, value));
        }

        // Interleave malformed lines (no delimiter)
        for m in &malformed {
            let clean: String = m
                .chars()
                .filter(|c| is_safe_char(*c) && *c != '=')
                .collect();
            if !clean.trim().is_empty() {
                lines.push(clean);
            }
        }

        if valid_keys.is_empty() {
            return TestResult::discard();
        }

        let content = lines.join("\n");
        let result = parse_content(&content, &PathBuf::from("test"), &ParseOptions::default());

        // Parser should NEVER error out
        match result {
            Ok(config) => {
                // All valid keys should be present (last-wins for duplicates)
                let mut unique_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
                for key in &valid_keys {
                    unique_keys.insert(key.clone());
                }
                let all_present = unique_keys.iter().all(|k| config.pairs.contains_key(k));
                TestResult::from_bool(all_present)
            }
            Err(_) => TestResult::failed(),
        }
    }
}
