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
mod examples;
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

    if cli.example {
        let subcmd = match &cli.command {
            Some(Commands::Diff { .. }) => Some("diff"),
            Some(Commands::Kv { .. }) => Some("kv"),
            Some(Commands::Config { .. }) => Some("config"),
            None => None,
        };
        crate::examples::print_examples(subcmd);
        return Ok((true, cfg));
    }

    match cli.command {
        Some(Commands::Diff { file1, file2, unified, ignore_space_change, ignore_trailing_space, ignore_blank_lines, brief, report_identical_files, shared }) => {
            let (f1, f2) = match (file1, file2) {
                (Some(f1), Some(f2)) => (f1, f2),
                _ => {
                    use clap::CommandFactory;
                    let mut cmd = crate::commands::Cli::command();
                    if let Some(sub) = cmd.find_subcommand_mut("diff") {
                        sub.print_help().unwrap();
                    }
                    std::process::exit(2);
                }
            };
            let identical = handle_diff_command(&f1, &f2, unified, ignore_space_change, ignore_trailing_space, ignore_blank_lines, brief, report_identical_files, &shared)?;
            Ok((identical, cfg))
        }
        Some(Commands::Kv {
            file1,
            file2,
            delimiter,
            d1,
            d2,
            strip_quotes,
            shared,
            output,
            ignore_keys,
            ignore_keywords,
            quiet,
            summary,
            show_secrets,
            verbose,
            generate_missing,
            direction,
        }) => {
            let (f1, f2) = match (file1, file2) {
                (Some(f1), Some(f2)) => (f1, f2),
                _ => {
                    use clap::CommandFactory;
                    let mut cmd = crate::commands::Cli::command();
                    if let Some(sub) = cmd.find_subcommand_mut("kv") {
                        sub.print_help().unwrap();
                    }
                    std::process::exit(2);
                }
            };
            let identical = handle_kv_command(
                &f1,
                &f2,
                delimiter.as_deref(),
                d1.as_deref(),
                d2.as_deref(),
                strip_quotes,
                &shared,
                &output,
                ignore_keys,
                ignore_keywords,
                quiet,
                summary,
                show_secrets,
                verbose,
                generate_missing.as_deref(),
                &direction,
            )?;
            Ok((identical, cfg))
        }
        Some(Commands::Config { what, action }) => {
            config::handle_config_command(&what, &action);
            // Config commands always succeed and return "identical" = true
            // (so the exit code is always 0)
            Ok((true, cfg))
        }
        None => {
            use clap::CommandFactory;
            let mut cmd = crate::commands::Cli::command();
            cmd.print_help().unwrap();
            std::process::exit(2);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// Command handlers
// ─────────────────────────────────────────────────────────────────────

/// Handle `qaren diff file1 file2` — literal line-by-line comparison.
#[allow(clippy::too_many_arguments)]
fn handle_diff_command(
    file1: &Path,
    file2: &Path,
    unified: bool,
    ignore_space_change: bool,
    ignore_trailing_space: bool,
    ignore_blank_lines: bool,
    brief: bool,
    report_identical_files: bool,
    shared: &crate::commands::SharedDiffOptions,
) -> Result<bool, QarenError> {
    let content1 = std::fs::read_to_string(file1)
        .map_err(|e| QarenError::from_io_with_path(e, file1.to_path_buf()))?;
    let content2 = std::fs::read_to_string(file2)
        .map_err(|e| QarenError::from_io_with_path(e, file2.to_path_buf()))?;

    let opts = DiffOptions {
        ignore_case: shared.ignore_case,
        ignore_all_space: shared.ignore_all_space,
        ignore_space_change,
        ignore_trailing_space,
        ignore_blank_lines,
        ignore_keys: vec![],
        ignore_keywords: vec![],
    };

    let result = literal_diff(&content1, &content2, &opts);

    let identical = result.additions.is_empty()
        && result.deletions.is_empty()
        && result.modifications.is_empty();

    let l1 = file1.file_name().and_then(|n| n.to_str()).unwrap_or("file1");
    let l2 = file2.file_name().and_then(|n| n.to_str()).unwrap_or("file2");

    if identical {
        if report_identical_files {
            println!("Files {} and {} are identical", l1, l2);
        }
        // POSIX diff stays silent if identical and -s is NOT provided
    } else {
        if brief {
            println!("Files {} and {} differ", l1, l2);
        } else if unified {
            let diff = similar::TextDiff::from_lines(&content1, &content2);
            let diff_str = diff.unified_diff().context_radius(3).header(l1, l2).to_string();
            use colored::Colorize;
            for line in diff_str.lines() {
                if line.starts_with('+') && !line.starts_with("+++") {
                    println!("{}", line.green());
                } else if line.starts_with('-') && !line.starts_with("---") {
                    println!("{}", line.red());
                } else if line.starts_with("@@") {
                    println!("{}", line.cyan());
                } else if line.starts_with("+++") || line.starts_with("---") {
                    println!("{}", line.bold());
                } else {
                    println!("{}", line);
                }
            }
        } else {
            output::print_literal_diff(&result);
        }
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
    shared: &crate::commands::SharedDiffOptions,
    output_format: &str,
    ignore_keys: Vec<String>,
    ignore_keywords: Vec<String>,
    quiet: bool,
    summary: bool,
    show_secrets: bool,
    verbose: bool,
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
        ..ParseOptions::default()
    };
    let opts2 = ParseOptions {
        delimiter: delim2,
        strip_quotes,
        ignore_case: false,
        ..ParseOptions::default()
    };

    // Parse both files using their respective options
    let config1 = parse_file(file1, &opts1)?;
    let config2 = parse_file(file2, &opts2)?;

    if verbose && !quiet {
        println!("[INFO] Parsing file1: {} (Found {} keys)", file1.display(), config1.pairs.len());
        println!("[INFO] Parsing file2: {} (Found {} keys)", file2.display(), config2.pairs.len());
    }

    // Build diff options
    let diff_opts = DiffOptions {
        ignore_case: shared.ignore_case,
        ignore_all_space: shared.ignore_all_space,
        ignore_space_change: false,
        ignore_trailing_space: false,
        ignore_blank_lines: false,
        ignore_keys,
        ignore_keywords,
    };

    if !quiet {
        use colored::Colorize;
        if summary {
            let warn_count1 = config1.warnings.iter().filter(|w| w.key.as_ref().is_none_or(|k| !diff_opts.is_ignored(k))).count();
            if warn_count1 > 0 {
                eprintln!("{} {}: {} duplicate keys detected (auto-overwritten).", "⚠".yellow(), file1.display(), warn_count1);
            }
            let warn_count2 = config2.warnings.iter().filter(|w| w.key.as_ref().is_none_or(|k| !diff_opts.is_ignored(k))).count();
            if warn_count2 > 0 {
                eprintln!("{} {}: {} duplicate keys detected (auto-overwritten).", "⚠".yellow(), file2.display(), warn_count2);
            }
        } else {
            for w in config1.warnings.iter().chain(config2.warnings.iter()) {
                let should_show = w.key.as_ref().is_none_or(|k| !diff_opts.is_ignored(k));
                if should_show {
                    eprintln!("{}: {}", "⚠ Warning".yellow().bold(), w.message);
                }
            }
        }
    }

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

    let identical = diff_result.is_identical();

    // Print results
    if quiet {
        // Absolute silence for exit-code checking
    } else if output_format == "json" {
        output::print_json_diff(&diff_result, show_secrets);
    } else {
        use colored::Colorize;
        if identical {
            if summary {
                println!("✔ Files are identical.");
            }
        } else {
            if summary {
                let l1 = label1.red();
                let l2 = label2.green();
                println!("\n── {} vs {} ──", l1, l2);
                println!(
                    "Summary: {} only in {}, {} only in {}, {} modified",
                    diff_result.missing_in_file2.len(), label1.red(),
                    diff_result.missing_in_file1.len(), label2.green(),
                    diff_result.modified.len()
                );
                println!();
            } else {
                output::print_diff_result(&diff_result, show_secrets, verbose, label1, label2);
            }
        }

        if !summary && d1.is_none() && d2.is_none() && delimiter.is_none() && delim1 != delim2 {
            eprintln!(
                "  ℹ auto-detected delimiters: '{}' for {}, '{}' for {}",
                delim1, label1, delim2, label2
            );
        }
    }

    // Generate patch if requested
    if let Some(output_path) = generate_missing {
        let patch_direction: PatchDirection = direction
            .parse()
            .map_err(QarenError::InvalidArguments)?;

        let patch_opts1 = ParseOptions {
            delimiter: delim1,
            strip_quotes,
            ..ParseOptions::default()
        };
        let patch_opts2 = ParseOptions {
            delimiter: delim2,
            strip_quotes,
            ..ParseOptions::default()
        };

        let created_files = generate_patch(&diff_result, output_path, &patch_opts1, &patch_opts2, patch_direction)?;

        if !quiet {
            eprintln!();
            if created_files.is_empty() {
                eprintln!("  ℹ No missing keys found. No patch file created.");
            } else {
                for path in &created_files {
                    eprintln!("✔ Patch file created: {}", path.display());
                }
            }
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
            eprintln!("  hint: to see usage examples run 'qaren --example' or 'qaren --help'");
        }
        _ => {
            eprintln!("  hint: check your arguments or run 'qaren --help' or 'qaren --example' for syntax help");
        }
    }
}


