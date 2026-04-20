//! CLI command definitions using `clap` derive macros.
//!
//! Defines the top-level `Cli` struct and its `Commands` subcommands:
//! - `diff`   — literal line-by-line comparison
//! - `kv`     — semantic key-value comparison
//! - `config` — persistent user configuration

use clap::{Parser, Subcommand};
use std::path::PathBuf;

// ─────────────────────────────────────────────────────────────────────
// Top-level CLI
// ─────────────────────────────────────────────────────────────────────

/// Qaren (قارن) — A blazingly fast configuration comparison tool.
///
/// Compare .env files, PM2 configs, YAML keys, and any key-value format
/// with semantic awareness, secret masking, and Linux-native exit codes.
#[derive(Parser)]
#[command(name = "qaren")]
#[command(version)]
#[command(about = "Semantic and literal diffing for configuration files")]
#[command(long_about = "\
Qaren (قارن — Arabic for 'compare') is a blazingly fast configuration
comparison tool built for DevOps engineers and system administrators.

It understands KEY=VALUE and KEY: VALUE formats natively, masks secrets
automatically, generates patch files for missing keys, and integrates
cleanly into CI/CD pipelines via standard exit codes.")]
#[command(after_help = "\
EXAMPLES:
  # Compare two .env files (auto-detects delimiter)
  qaren kv prod.env staging.env

  # Compare PM2 ecosystem.json exports (colon delimiter)
  qaren kv ecosystem.prod.txt ecosystem.staging.txt -d ':'

  # Cross-format comparison (prod uses '=', staging uses ':')
  qaren kv prod.env staging.yaml --d1 '=' --d2 ':'

  # Show differences only, hide identical keys (default)
  qaren kv prod.env staging.env

  # Show everything including identical keys
  qaren kv prod.env staging.env --verbose

  # Case-insensitive comparison
  qaren kv prod.env staging.env -i

  # Ignore whitespace differences in values
  qaren kv prod.env staging.env -w

  # Show secret values instead of masking them
  qaren kv prod.env staging.env -S

  # Generate a patch file with missing keys
  qaren kv prod.env staging.env -g missing.env

  # Generate bidirectional patch files
  qaren kv prod.env staging.env -g sync.env --direction bidirectional

  # Literal line-by-line diff
  qaren diff file1.txt file2.txt

  # View and toggle persistent settings
  qaren config show
  qaren config exit toggle   # toggle exit-1-on-diff behaviour
  qaren config color toggle  # toggle ANSI colour output

EXIT CODES:
  0   Files are identical (or pipeline-friendly mode enabled)
  1   Differences found (default POSIX diff behaviour)
  2   Error (file not found, permission denied, invalid arguments)

  Default behaviour: exit 1 on differences (POSIX standard).
  Run 'qaren config exit toggle' to switch to pipeline-friendly mode (always exit 0).")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// ─────────────────────────────────────────────────────────────────────
// Subcommands
// ─────────────────────────────────────────────────────────────────────

/// Available commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Perform literal line-by-line comparison (like POSIX diff)
    ///
    /// Compares two files line-by-line using the Myers diff algorithm.
    /// Useful for detecting formatting changes or non-KV config files.
    Diff {
        /// First file to compare
        file1: PathBuf,
        /// Second file to compare
        file2: PathBuf,
    },

    /// Perform semantic key-value comparison [alias: kvp]
    ///
    /// Order-agnostic comparison that understands KEY=VALUE and KEY: VALUE
    /// formats. Automatically detects the delimiter used in each file.
    #[command(alias = "kvp")]
    Kv {
        /// First file to compare (source / reference)
        file1: PathBuf,
        /// Second file to compare (target)
        file2: PathBuf,

        /// Delimiter for BOTH files when they share the same format.
        /// Auto-detected if omitted. Use --d1/--d2 for cross-format comparisons.
        /// Examples: '=', ':', ' '
        #[arg(short = 'd', long, value_name = "CHAR")]
        delimiter: Option<String>,

        /// Delimiter override for file1 only (overrides --delimiter for file1)
        #[arg(long, value_name = "CHAR")]
        d1: Option<String>,

        /// Delimiter override for file2 only (overrides --delimiter for file2)
        #[arg(long, value_name = "CHAR")]
        d2: Option<String>,

        /// Strip surrounding quotes from keys and values  [short: -s]
        #[arg(short = 's', long)]
        strip_quotes: bool,

        /// Compare values case-insensitively  [short: -i]
        #[arg(short = 'i', long)]
        ignore_case: bool,

        /// Ignore whitespace differences in values  [short: -w]
        #[arg(short = 'w', long)]
        ignore_whitespace: bool,

        /// Show secret values in plain text (disables masking)  [short: -S]
        #[arg(short = 'S', long)]
        show_secrets: bool,

        /// Show identical keys in output as well (hidden by default)  [short: -v]
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Print summary line only — no per-key details  [short: -q]
        #[arg(short = 'q', long)]
        brief: bool,

        /// Generate a patch file with missing keys  [short: -g]
        #[arg(short = 'g', long, value_name = "FILE")]
        generate_missing: Option<PathBuf>,

        /// Patch direction: source-to-target (default), target-to-source, bidirectional
        /// Only valid when --generate-missing / -g is specified.
        #[arg(long, default_value = "source-to-target", value_name = "DIR")]
        direction: String,
    },

    /// View and modify persistent Qaren settings
    ///
    /// Settings are stored in the platform config directory:
    ///   Linux/macOS : $XDG_CONFIG_HOME/qaren/config  (~/.config/qaren/config)
    ///   Windows     : %APPDATA%\qaren\config
    Config {
        /// Setting to configure: 'exit', 'color', 'show', or 'path'
        #[arg(default_value = "show")]
        what: String,

        /// Action: 'show' (default) or 'toggle'
        #[arg(default_value = "show")]
        action: String,
    },
}

// ─────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────

/// Validate the delimiter string — must be exactly one character.
///
/// Returns the character on success, or a descriptive error string.
pub fn validate_delimiter(delimiter: &str) -> Result<char, String> {
    let mut chars = delimiter.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Ok(c),
        _ => Err(format!(
            "Invalid delimiter: '{}' (must be a single character). Examples: '=', ':', ' '",
            delimiter
        )),
    }
}

// ─────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_delimiter_single_char() {
        assert_eq!(validate_delimiter("="), Ok('='));
        assert_eq!(validate_delimiter(":"), Ok(':'));
        assert_eq!(validate_delimiter(" "), Ok(' '));
    }

    #[test]
    fn test_validate_delimiter_multi_char() {
        assert!(validate_delimiter("==").is_err());
        assert!(validate_delimiter("abc").is_err());
    }

    #[test]
    fn test_validate_delimiter_empty() {
        assert!(validate_delimiter("").is_err());
    }
}
