//! Qaren CLI — Configuration comparison tool.
//!
//! Entry point for the `qaren` command-line application.
//! Dispatches to `diff` or `kvp` subcommands and handles
//! exit codes (0 = identical, 1 = different, 2 = error).

mod commands;
mod masking;
mod output;

use clap::Parser;
use commands::{Cli, Commands};
use qaren_core::{
    generate_patch, literal_diff, parse_file, semantic_diff, ParseOptions, PatchDirection,
    QarenError,
};
use std::path::Path;
use std::process;

fn main() {
    let exit_code = match run() {
        Ok(identical) => {
            if identical {
                0 // Files are identical
            } else {
                1 // Differences found
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            print_error_hints(&e);
            2 // Error occurred
        }
    };

    process::exit(exit_code);
}

/// Main application logic. Returns `Ok(true)` if files are identical,
/// `Ok(false)` if differences were found, or `Err` on failure.
fn run() -> Result<bool, QarenError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Diff { file1, file2 } => handle_diff_command(&file1, &file2),
        Commands::Kvp {
            file1,
            file2,
            delimiter,
            strip_quotes,
            generate_missing,
            direction,
            show_secrets,
        } => handle_kvp_command(
            &file1,
            &file2,
            &delimiter,
            strip_quotes,
            generate_missing.as_deref(),
            &direction,
            show_secrets,
        ),
    }
}

/// Handle the `qaren diff file1 file2` command — literal line-by-line comparison.
fn handle_diff_command(file1: &Path, file2: &Path) -> Result<bool, QarenError> {
    let content1 = std::fs::read_to_string(file1)
        .map_err(|e| QarenError::from_io_with_path(e, file1.to_path_buf()))?;
    let content2 = std::fs::read_to_string(file2)
        .map_err(|e| QarenError::from_io_with_path(e, file2.to_path_buf()))?;

    let result = literal_diff(&content1, &content2);
    output::print_literal_diff(&result);

    let identical = result.additions.is_empty()
        && result.deletions.is_empty()
        && result.modifications.is_empty();

    Ok(identical)
}

/// Handle the `qaren kvp file1 file2 [options]` command — semantic KV comparison.
fn handle_kvp_command(
    file1: &Path,
    file2: &Path,
    delimiter: &str,
    strip_quotes: bool,
    generate_missing: Option<&Path>,
    direction: &str,
    show_secrets: bool,
) -> Result<bool, QarenError> {
    // Validate delimiter
    let delim_char =
        commands::validate_delimiter(delimiter).map_err(QarenError::InvalidArguments)?;

    // Validate --direction without --generate-missing
    if generate_missing.is_none() && direction != "source-to-target" {
        return Err(QarenError::InvalidArguments(
            "--direction requires --generate-missing".to_string(),
        ));
    }

    // Build parse options
    let options = ParseOptions {
        delimiter: delim_char,
        strip_quotes,
        ..ParseOptions::default()
    };

    // Parse both files
    let config1 = parse_file(file1, &options)?;
    let config2 = parse_file(file2, &options)?;

    // Perform semantic diff
    let diff_result = semantic_diff(&config1, &config2);

    // Print results
    output::print_diff_result(&diff_result, show_secrets);

    // Generate patch if requested
    if let Some(output_path) = generate_missing {
        let patch_direction: PatchDirection = direction
            .parse()
            .map_err(QarenError::InvalidArguments)?;

        let created_files = generate_patch(&diff_result, output_path, &options, patch_direction)?;

        eprintln!();
        for path in &created_files {
            eprintln!(
                "✔ Patch file created: {}",
                path.display()
            );
        }
    }

    Ok(diff_result.is_identical())
}

/// Print additional hints for common error types.
fn print_error_hints(err: &QarenError) {
    match err {
        QarenError::PermissionDenied(_) => {
            eprintln!("  Hint: Try running with appropriate permissions or specify a different file");
        }
        QarenError::InvalidDelimiter(_) => {
            eprintln!("  Examples: '=', ':', ' '");
        }
        _ => {}
    }
}
