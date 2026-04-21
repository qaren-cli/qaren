//! CLI command definitions using `clap` derive macros.
//!
//! Defines the top-level `Cli` struct and its `Commands` subcommands:
//! - `diff`   — literal line-by-line comparison
//! - `kv`     — semantic key-value comparison
//! - `config` — persistent user configuration

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(clap::Args, Debug, Default, Clone)]
pub struct SharedDiffOptions {
    /// Ignore case differences in file contents
    #[arg(short = 'i', long)]
    pub ignore_case: bool,

    /// Ignore all white space (for KV: strips spaces inside values)
    #[arg(short = 'w', long)]
    pub ignore_all_space: bool,
}

// ─────────────────────────────────────────────────────────────────────
// Top-level CLI
// ─────────────────────────────────────────────────────────────────────

/// Qaren — The next generation of diff.
///
/// A blazingly fast configuration comparison tool built specifically 
/// for DevOps engineers and system administrators.
#[derive(Parser)]
#[command(name = "qaren")]
#[command(version)]
#[command(about = "Semantic and literal diffing for configuration files")]
#[command(long_about = "\
Qaren is the next generation of diff. A blazingly fast, multi-paradigm 
configuration comparison tool built specially for DevOps engineers and 
system administrators.

It understands KEY=VALUE and KEY: VALUE formats natively, masks secrets
automatically (zero-trust by default), generates intelligent patch files,
and integrates cleanly into CI/CD pipelines via POSIX-standard exit codes.")]
#[command(after_help = "\
EXIT CODES:
  0   Files are identical (or pipeline-friendly mode enabled)
  1   Differences found (default POSIX diff behaviour)
  2   Error (file not found, permission denied, invalid arguments)

  Default behaviour: exit 1 on differences (POSIX standard).
  Run 'qaren config exit toggle' to switch to pipeline-friendly mode (always exit 0).

  To see rich examples and use-cases, run:
  $ qaren --example
")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Print detailed usage examples
    #[arg(long = "example", global = true)]
    pub example: bool,
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
        file1: Option<PathBuf>,
        /// Second file to compare
        file2: Option<PathBuf>,

        /// Output unified diff  [short: -u]
        #[arg(short = 'u', long)]
        unified: bool,

        /// Ignore changes in the amount of white space
        #[arg(short = 'b', long)]
        ignore_space_change: bool,

        /// Ignore white space at line end
        #[arg(short = 'Z', long)]
        ignore_trailing_space: bool,

        /// Ignore changes where lines are all blank
        #[arg(short = 'B', long)]
        ignore_blank_lines: bool,

        /// Report only when files differ
        #[arg(short = 'q', long)]
        brief: bool,

        /// Report when two files are the same
        #[arg(short = 's', long)]
        report_identical_files: bool,

        #[command(flatten)]
        shared: SharedDiffOptions,
    },

    /// Perform semantic key-value comparison [alias: kvp]
    ///
    /// Order-agnostic comparison that understands KEY=VALUE and KEY: VALUE
    /// formats. Automatically detects the delimiter used in each file.
    #[command(alias = "kvp")]
    Kv {
        /// First file to compare (source / reference)
        file1: Option<PathBuf>,
        /// Second file to compare (target)
        file2: Option<PathBuf>,

        /// Delimiter for BOTH files when they share the same format.
        /// Auto-detected if omitted. Use --d1/--d2 for cross-format comparisons.
        /// Examples: '=', ':', ' '
        #[arg(short = 'd', long, value_name = "DELIMITER", conflicts_with_all = ["d1", "d2"])]
        delimiter: Option<String>,

        /// Delimiter override for file1 only (overrides --delimiter for file1)
        #[arg(long, value_name = "DELIMITER")]
        d1: Option<String>,

        /// Delimiter override for file2 only (overrides --delimiter for file2)
        #[arg(long, value_name = "DELIMITER")]
        d2: Option<String>,

        /// Strip surrounding quotes from keys and values  [short: -Q]
        #[arg(short = 'Q', long)]
        strip_quotes: bool,

        #[command(flatten)]
        shared: SharedDiffOptions,
        /// Output format (text or json)
        #[arg(short = 'o', long, default_value = "text", value_name = "FORMAT")]
        output: String,

        /// Ignore a specific key (exact match). Can be passed multiple times.  [short: -x]
        #[arg(short = 'x', long = "ignore-key", value_name = "KEY")]
        ignore_keys: Vec<String>,

        /// Ignore keys containing this keyword (case-insensitive substring).
        #[arg(long = "ignore-keyword", value_name = "KEYWORD")]
        ignore_keywords: Vec<String>,

        /// Quiet mode - no stdout/stderr output. Return exit code only.  [short: -q]
        #[arg(short = 'q', long, conflicts_with_all = ["verbose", "summary"])]
        quiet: bool,

        /// Summary mode - minimize output, aggregate warnings  [short: -s]
        #[arg(short = 's', long, conflicts_with_all = ["verbose", "quiet"])]
        summary: bool,

        /// Show secret values in plain text (disables masking)  [short: -S]
        #[arg(short = 'S', long)]
        show_secrets: bool,

        /// Show identical keys in output as well (hidden by default)  [short: -v]
        #[arg(short = 'v', long, conflicts_with_all = ["quiet", "summary"])]
        verbose: bool,
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
