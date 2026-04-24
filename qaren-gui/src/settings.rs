//! Application settings for the Qaren GUI.
//!
//! Persisted via `eframe`'s built-in storage (backed by `ron` serialization).
//! Settings control parsing behaviour, display preferences, and comparison
//! options. All fields have sensible defaults.

use serde::{Deserialize, Serialize};

/// Comparison mode: Semantic KV or Literal line-by-line diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonMode {
    /// Order-agnostic key-value comparison
    SemanticKv,
    /// Traditional line-by-line diff
    LiteralDiff,
}

impl Default for ComparisonMode {
    fn default() -> Self {
        Self::SemanticKv
    }
}

impl ComparisonMode {
    /// Human-readable label for UI display.
    pub fn label(&self) -> &'static str {
        match self {
            Self::SemanticKv => "Semantic KV",
            Self::LiteralDiff => "Literal Diff",
        }
    }
}

/// Delimiter choice for the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelimiterChoice {
    /// Equals sign `=` (default for .env files)
    Equals,
    /// Colon `:` (YAML / PM2 style)
    Colon,
    /// Space ` ` (some config formats)
    Space,
    /// Auto-detect from file content
    Auto,
}

impl Default for DelimiterChoice {
    fn default() -> Self {
        Self::Equals
    }
}

impl DelimiterChoice {
    /// Human-readable label for UI display.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Equals => "= (equals)",
            Self::Colon => ": (colon)",
            Self::Space => "  (space)",
            Self::Auto => "Auto-detect",
        }
    }

    /// Convert to the delimiter char for parsing.
    /// Returns `None` for Auto (caller must use `detect_delimiter`).
    pub fn to_char(&self) -> Option<char> {
        match self {
            Self::Equals => Some('='),
            Self::Colon => Some(':'),
            Self::Space => Some(' '),
            Self::Auto => None,
        }
    }

    /// All variants for iteration in UI dropdowns.
    pub const ALL: &'static [Self] = &[
        Self::Equals,
        Self::Colon,
        Self::Space,
        Self::Auto,
    ];
}

/// Persistent application settings.
///
/// Serialized to disk by `eframe` between sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Delimiter choice for parsing
    pub delimiter: DelimiterChoice,

    /// Whether to strip surrounding quotes from keys/values
    pub strip_quotes: bool,

    /// Whether to show secret values in plain text
    pub show_secrets: bool,

    /// Compare values case-insensitively
    pub ignore_case: bool,

    /// Ignore all whitespace differences
    pub ignore_all_space: bool,

    /// Ignore changes in amount of whitespace
    pub ignore_space_change: bool,

    /// Ignore trailing whitespace
    pub ignore_trailing_space: bool,

    /// Ignore blank line changes (literal diff)
    pub ignore_blank_lines: bool,

    /// Comparison mode
    pub comparison_mode: ComparisonMode,

    /// Use dark mode
    pub dark_mode: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            delimiter: DelimiterChoice::default(),
            strip_quotes: false,
            show_secrets: false,
            ignore_case: false,
            ignore_all_space: false,
            ignore_space_change: false,
            ignore_trailing_space: false,
            ignore_blank_lines: false,
            comparison_mode: ComparisonMode::default(),
            dark_mode: false,
        }
    }
}

impl AppSettings {
    /// Build `qaren_core::ParseOptions` from these settings.
    ///
    /// If delimiter is `Auto`, the caller must resolve it via
    /// `qaren_core::detect_delimiter` before calling this.
    pub fn to_parse_options(&self, resolved_delimiter: char) -> qaren_core::ParseOptions {
        qaren_core::ParseOptions {
            delimiter: resolved_delimiter,
            strip_quotes: self.strip_quotes,
            comment_prefixes: vec!["#".to_string(), "//".to_string()],
            ignore_case: self.ignore_case,
        }
    }

    /// Build `qaren_core::DiffOptions` from these settings.
    pub fn to_diff_options(&self) -> qaren_core::DiffOptions {
        qaren_core::DiffOptions {
            ignore_case: self.ignore_case,
            ignore_all_space: self.ignore_all_space,
            ignore_space_change: self.ignore_space_change,
            ignore_trailing_space: self.ignore_trailing_space,
            ignore_blank_lines: self.ignore_blank_lines,
            ignore_keys: Vec::new(),
            ignore_keywords: Vec::new(),
        }
    }

    /// Resolve the delimiter character, using auto-detection if needed.
    pub fn resolve_delimiter(&self, content: &str) -> char {
        match self.delimiter.to_char() {
            Some(ch) => ch,
            None => qaren_core::detect_delimiter(content),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();
        assert_eq!(settings.delimiter, DelimiterChoice::Equals);
        assert!(!settings.strip_quotes);
        assert!(!settings.show_secrets);
        assert!(!settings.ignore_case);
        assert!(!settings.ignore_all_space);
        assert!(!settings.dark_mode);
        assert_eq!(settings.comparison_mode, ComparisonMode::SemanticKv);
    }

    #[test]
    fn test_comparison_mode_labels() {
        assert_eq!(ComparisonMode::SemanticKv.label(), "Semantic KV");
        assert_eq!(ComparisonMode::LiteralDiff.label(), "Literal Diff");
    }

    #[test]
    fn test_delimiter_choice_labels() {
        assert_eq!(DelimiterChoice::Equals.label(), "= (equals)");
        assert_eq!(DelimiterChoice::Colon.label(), ": (colon)");
        assert_eq!(DelimiterChoice::Space.label(), "  (space)");
        assert_eq!(DelimiterChoice::Auto.label(), "Auto-detect");
    }

    #[test]
    fn test_delimiter_choice_to_char() {
        assert_eq!(DelimiterChoice::Equals.to_char(), Some('='));
        assert_eq!(DelimiterChoice::Colon.to_char(), Some(':'));
        assert_eq!(DelimiterChoice::Space.to_char(), Some(' '));
        assert_eq!(DelimiterChoice::Auto.to_char(), None);
    }

    #[test]
    fn test_to_parse_options() {
        let settings = AppSettings {
            strip_quotes: true,
            ignore_case: true,
            ..AppSettings::default()
        };
        let opts = settings.to_parse_options('=');
        assert_eq!(opts.delimiter, '=');
        assert!(opts.strip_quotes);
        assert!(opts.ignore_case);
        assert_eq!(opts.comment_prefixes, vec!["#", "//"]);
    }

    #[test]
    fn test_to_diff_options() {
        let settings = AppSettings {
            ignore_case: true,
            ignore_all_space: true,
            ignore_blank_lines: true,
            ..AppSettings::default()
        };
        let opts = settings.to_diff_options();
        assert!(opts.ignore_case);
        assert!(opts.ignore_all_space);
        assert!(opts.ignore_blank_lines);
        assert!(!opts.ignore_space_change);
        assert!(!opts.ignore_trailing_space);
    }

    #[test]
    fn test_resolve_delimiter_explicit() {
        let settings = AppSettings {
            delimiter: DelimiterChoice::Colon,
            ..AppSettings::default()
        };
        assert_eq!(settings.resolve_delimiter("KEY=value"), ':');
    }

    #[test]
    fn test_resolve_delimiter_auto() {
        let settings = AppSettings {
            delimiter: DelimiterChoice::Auto,
            ..AppSettings::default()
        };
        assert_eq!(settings.resolve_delimiter("KEY=value\nDB=host"), '=');
        assert_eq!(settings.resolve_delimiter("key: value\ndb: host"), ':');
    }

    #[test]
    fn test_delimiter_all_variants() {
        assert_eq!(DelimiterChoice::ALL.len(), 4);
    }

    #[test]
    fn test_settings_serde_roundtrip() {
        let settings = AppSettings {
            delimiter: DelimiterChoice::Colon,
            strip_quotes: true,
            show_secrets: true,
            ignore_case: true,
            dark_mode: true,
            ..AppSettings::default()
        };
        let json = serde_json::to_string(&settings);
        assert!(json.is_ok(), "Settings should serialize to JSON");
        let deserialized: Result<AppSettings, _> = serde_json::from_str(&json.unwrap_or_default());
        assert!(deserialized.is_ok(), "Settings should deserialize from JSON");
        let restored = deserialized.unwrap_or_default();
        assert_eq!(restored.delimiter, DelimiterChoice::Colon);
        assert!(restored.strip_quotes);
        assert!(restored.show_secrets);
        assert!(restored.dark_mode);
    }
}
