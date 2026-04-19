//! CLI command definitions using `clap` derive macros.
//!
//! Defines the top-level `Cli` struct and its `Commands` subcommands:
//! `diff` for literal comparison and `kvp` for semantic key-value comparison.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Qaren (قارن) — A blazingly fast configuration comparison tool.
#[derive(Parser)]
#[command(name = "qaren")]
#[command(about = "Configuration comparison tool — semantic and literal diffing for config files")]
#[command(version)]
#[command(after_help = "Examples:\n  qaren diff file1.txt file2.txt\n  qaren kvp .env.prod .env.staging\n  qaren kvp pm2.txt local.env -d \":\" --strip-quotes\n  qaren kvp .env.prod .env.staging --generate-missing missing.env\n  qaren kvp .env.prod .env.staging --generate-missing sync.env --direction bidirectional")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available comparison commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Perform literal line-by-line comparison
    Diff {
        /// First file to compare
        file1: PathBuf,
        /// Second file to compare
        file2: PathBuf,
    },

    /// Perform semantic key-value pair comparison
    Kvp {
        /// First file to compare (source)
        file1: PathBuf,
        /// Second file to compare (target)
        file2: PathBuf,

        /// Custom delimiter character (default: '=')
        #[arg(short, long, default_value = "=")]
        delimiter: String,

        /// Strip surrounding quotes from keys and values
        #[arg(long)]
        strip_quotes: bool,

        /// Generate a patch file with missing keys
        #[arg(long)]
        generate_missing: Option<PathBuf>,

        /// Patch direction: source-to-target (default), target-to-source, or bidirectional.
        /// Only valid when --generate-missing is specified.
        #[arg(long, default_value = "source-to-target")]
        direction: String,

        /// Show secret values instead of masking them
        #[arg(long)]
        show_secrets: bool,
    },
}

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
