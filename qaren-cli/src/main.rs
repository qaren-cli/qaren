//! Qaren CLI — Configuration comparison tool.
//!
//! Entry point for the `qaren` command-line application.
//! Dispatches to `diff`, `kv`, or `config` subcommands.
//!
//! # Exit Codes
//!
//! The default behaviour follows **POSIX diff**:
//!   - `0`  — files are identical
//!   - `1`  — differences found
//!   - `2`  — a real error occurred (file not found, permission denied, etc.)
//!
//! Run `qaren config exit toggle` to switch to pipeline-friendly mode
//! (always exit `0` on success).

mod commands;
mod config;
mod masking;
mod output;

use clap::Parser;
use commands::{validate_delimiter, Cli, Commands};
use config::load_config;
use qaren_core::{
    detect_delimiter, generate_patch, literal_diff, parse_file, semantic_diff, DiffOptions,
    ParseOptions, PatchDirection, QarenError,
};
use std::path::Path;
use std::process;

fn main() {
    let exit_code = match run() {
        Ok((identical, cfg)) => {
            if identical {
                0 // Files are identical
            } else if cfg.exit_nonzero_on_diff {
                1 // Differences found (POSIX diff mode)
            } else {
                0 // User opted to always exit 0 on success
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
            print_error_hints(&e);
            2 // Real error
        }
    };

    process::exit(exit_code);
}

/// Main application logic.
///
/// Returns `Ok((identical, config))` where `identical` is `true` when the two
/// files are semantically equivalent.  Returns `Err` on any failure.
fn run() -> Result<(bool, config::QarenConfig), QarenError> {
    let cli = Cli::parse();
    let cfg = load_config();

    // --color respects the persistent config (and NO_COLOR env convention)
    let use_color = cfg.color && std::env::var("NO_COLOR").is_err();
    if !use_color {
        // Disable colored output globally
        colored::control::set_override(false);
    }

    match cli.command {
        Commands::Diff { file1, file2, ignore_case, ignore_whitespace, brief } => {
            let identical = handle_diff_command(&file1, &file2, ignore_case, ignore_whitespace, brief)?;
            Ok((identical, cfg))
        }
        Commands::Kv {
            file1,
            file2,
            delimiter,
            d1,
            d2,
            strip_quotes,
            ignore_case,
            ignore_whitespace,
            show_secrets,
            verbose,
            brief,
            generate_missing,
            direction,
        } => {
            let identical = handle_kv_command(
                &file1,
                &file2,
                delimiter.as_deref(),
                d1.as_deref(),
                d2.as_deref(),
                strip_quotes,
                ignore_case,
                ignore_whitespace,
                show_secrets,
                verbose,
                brief,
                generate_missing.as_deref(),
                &direction,
            )?;
            Ok((identical, cfg))
        }
        Commands::Config { what, action } => {
            config::handle_config_command(&what, &action);
            // Config commands always succeed and return "identical" = true
            // (so the exit code is always 0)
            Ok((true, cfg))
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// Command handlers
// ─────────────────────────────────────────────────────────────────────

/// Handle `qaren diff file1 file2` — literal line-by-line comparison.
fn handle_diff_command(
    file1: &Path,
    file2: &Path,
    ignore_case: bool,
    ignore_whitespace: bool,
    brief: bool,
) -> Result<bool, QarenError> {
    let content1 = std::fs::read_to_string(file1)
        .map_err(|e| QarenError::from_io_with_path(e, file1.to_path_buf()))?;
    let content2 = std::fs::read_to_string(file2)
        .map_err(|e| QarenError::from_io_with_path(e, file2.to_path_buf()))?;

    let opts = DiffOptions {
        ignore_case,
        ignore_whitespace,
    };

    let result = literal_diff(&content1, &content2, &opts);

    let identical = result.additions.is_empty()
        && result.deletions.is_empty()
        && result.modifications.is_empty();

    if brief {
        let l1 = file1.file_name().and_then(|n| n.to_str()).unwrap_or("file1");
        let l2 = file2.file_name().and_then(|n| n.to_str()).unwrap_or("file2");
        if identical {
            println!("Summary: {} and {} are identical", l1, l2);
        } else {
            println!("Summary: {} and {} differ ({} additions, {} deletions, {} modifications)", 
                l1, l2, result.additions.len(), result.deletions.len(), result.modifications.len());
        }
    } else {
        output::print_literal_diff(&result);
    }

    Ok(identical)
}

/// Resolve the delimiter for a file: explicit flag → shared flag → auto-detect.
///
/// Priority: `per_file` > `shared` > auto-detect from `content`.
fn resolve_delimiter(
    per_file: Option<&str>,
    shared: Option<&str>,
    content: &str,
) -> Result<char, QarenError> {
    if let Some(d) = per_file.or(shared) {
        validate_delimiter(d).map_err(QarenError::InvalidArguments)
    } else {
        Ok(detect_delimiter(content))
    }
}

/// Handle `qaren kv file1 file2 [options]` — semantic KV comparison.
#[allow(clippy::too_many_arguments)]
fn handle_kv_command(
    file1: &Path,
    file2: &Path,
    delimiter: Option<&str>,
    d1: Option<&str>,
    d2: Option<&str>,
    strip_quotes: bool,
    ignore_case: bool,
    ignore_whitespace: bool,
    show_secrets: bool,
    verbose: bool,
    brief: bool,
    generate_missing: Option<&Path>,
    direction: &str,
) -> Result<bool, QarenError> {
    // Validate --direction without --generate-missing
    if generate_missing.is_none() && direction != "source-to-target" {
        return Err(QarenError::InvalidArguments(
            "--direction requires --generate-missing (-g)".to_string(),
        ));
    }

    // Read raw file content for auto-detection
    let content1 = std::fs::read_to_string(file1)
        .map_err(|e| QarenError::from_io_with_path(e, file1.to_path_buf()))?;
    let content2 = std::fs::read_to_string(file2)
        .map_err(|e| QarenError::from_io_with_path(e, file2.to_path_buf()))?;

    // Resolve delimiter independently for each file
    let delim1 = resolve_delimiter(d1, delimiter, &content1)?;
    let delim2 = resolve_delimiter(d2, delimiter, &content2)?;

    // Build per-file parse options
    let opts1 = ParseOptions {
        delimiter: delim1,
        strip_quotes,
        ignore_case: false,    // parsing is case-preserving; diff handles case
        ignore_whitespace: false,
        ..ParseOptions::default()
    };
    let opts2 = ParseOptions {
        delimiter: delim2,
        strip_quotes,
        ignore_case: false,
        ignore_whitespace: false,
        ..ParseOptions::default()
    };

    // Parse both files using their respective options
    let config1 = parse_file(file1, &opts1)?;
    let config2 = parse_file(file2, &opts2)?;

    use colored::Colorize;
    for w in config1.warnings.iter().chain(config2.warnings.iter()) {
        eprintln!("{}: {}", "⚠ Warning".yellow().bold(), w);
    }

    // Build diff options
    let diff_opts = DiffOptions {
        ignore_case,
        ignore_whitespace,
    };

    // Perform semantic diff
    let diff_result = semantic_diff(&config1, &config2, &diff_opts);

    // Determine display labels from actual file stems
    let label1 = file1
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file1");
    let label2 = file2
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file2");

    // Print results
    output::print_diff_result(&diff_result, show_secrets, verbose, brief, label1, label2);

    // Annotate delimiter info when auto-detected and different
    if d1.is_none() && d2.is_none() && delimiter.is_none() && delim1 != delim2 {
        eprintln!(
            "  ℹ auto-detected delimiters: '{}' for {}, '{}' for {}",
            delim1, label1, delim2, label2
        );
    }

    // Generate patch if requested
    if let Some(output_path) = generate_missing {
        let patch_direction: PatchDirection = direction
            .parse()
            .map_err(QarenError::InvalidArguments)?;

        // Use file1's delimiter as the canonical output format
        let patch_opts = ParseOptions {
            delimiter: delim1,
            strip_quotes,
            ..ParseOptions::default()
        };

        let created_files = generate_patch(&diff_result, output_path, &patch_opts, patch_direction)?;

        eprintln!();
        for path in &created_files {
            eprintln!("✔ Patch file created: {}", path.display());
        }
    }

    Ok(diff_result.is_identical())
}

// ─────────────────────────────────────────────────────────────────────
// Error hints
// ─────────────────────────────────────────────────────────────────────

/// Print additional contextual hints for common error types.
fn print_error_hints(err: &QarenError) {
    match err {
        QarenError::PermissionDenied(_) => {
            eprintln!("  hint: try running with appropriate permissions");
        }
        QarenError::InvalidDelimiter(_) => {
            eprintln!("  hint: delimiter must be a single character, e.g. '=', ':', ' '");
        }
        QarenError::FileNotFound(path) => {
            eprintln!("  hint: check that '{}' exists and is readable", path.display());
        }
        _ => {}
    }
}
