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
mod examples;
mod output;

use clap::Parser;
use commands::{validate_delimiter, Cli, Commands};
use config::load_config;
use qaren_core::{
    collect_files_recursive, detect_delimiter, literal_diff, parse_file, semantic_diff,
    semantic_diff_dir, DiffOptions, DirParseOptions, FileDiffStatus, LiteralDiffResult, ParseOptions, PatchDirection,
    QarenError,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
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
    let mut use_color = cfg.color && std::env::var("NO_COLOR").is_err();

    // Disable color automatically for JSON output to prevent ANSI codes in strings
    if let Some(Commands::Kv { output, .. }) = &cli.command {
        if output == "json" {
            use_color = false;
        }
    }

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

    if let Some(shell) = cli.generate_completions {
        use clap::CommandFactory;
        let mut cmd = Cli::command();
        clap_complete::generate(shell, &mut cmd, "qaren", &mut std::io::stdout());
        return Ok((true, cfg));
    }

    match cli.command {
        Some(Commands::Diff { file1, file2, unified, ignore_space_change, ignore_trailing_space, ignore_blank_lines, recursive, files_only, brief, quiet, report_identical_files, shared }) => {
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
            let identical = handle_diff_command(&f1, &f2, unified, ignore_space_change, ignore_trailing_space, ignore_blank_lines, recursive, files_only, brief, quiet, report_identical_files, &shared)?;
            Ok((identical, cfg))
        }
        Some(Commands::Kv {
            file1, file2, delimiter, d1, d2, strip_quotes, shared, recursive, output, ignore_keys, ignore_keywords, quiet, summary, show_secrets, verbose, missing_only, generate_patch, mask_patches, direction
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
                &f1, &f2, delimiter.as_deref(), d1.as_deref(), d2.as_deref(), strip_quotes, recursive, &output,
                ignore_keys, ignore_keywords, quiet, summary, show_secrets, verbose, missing_only,
                generate_patch.as_deref(), mask_patches, &direction, &shared,
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
    recursive: bool,
    files_only: bool,
    brief: bool,
    quiet: bool,
    report_identical_files: bool,
    shared: &crate::commands::SharedDiffOptions,
) -> Result<bool, QarenError> {
    if recursive {
        let mut files1 = HashSet::new();
        let mut files2 = HashSet::new();
        let mut warnings = Vec::new();
        
        collect_files_recursive(file1, file1, &mut files1, &mut warnings);
        collect_files_recursive(file2, file2, &mut files2, &mut warnings);

        let all_files: HashSet<&PathBuf> = files1.union(&files2).collect();
        let mut sorted_files: Vec<_> = all_files.into_iter().collect();
        sorted_files.sort();

        let opts = DiffOptions {
            ignore_case: shared.ignore_case,
            ignore_all_space: shared.ignore_all_space,
            ignore_space_change,
            ignore_trailing_space,
            ignore_blank_lines,
            ignore_keys: vec![],
            ignore_keywords: vec![],
        };

        use rayon::prelude::*;
        
        let results: Vec<(bool, Option<LiteralDiffResult>, String, String)> = sorted_files.par_iter().map(|rel_path| {
            let in_1 = files1.contains(*rel_path);
            let in_2 = files2.contains(*rel_path);
            
            let p1 = file1.join(*rel_path);
            let p2 = file2.join(*rel_path);
            
            let label1 = format!("{}/{}", file1.display(), rel_path.display());
            let label2 = format!("{}/{}", file2.display(), rel_path.display());

            if in_1 && in_2 {
                if files_only {
                    return (true, None, String::new(), String::new());
                }

                // Metadata check short-circuit
                let m1 = std::fs::metadata(&p1).ok();
                let m2 = std::fs::metadata(&p2).ok();
                
                let can_short_circuit_on_size = !opts.ignore_case && !opts.ignore_all_space && !opts.ignore_space_change 
                    && !opts.ignore_trailing_space && !opts.ignore_blank_lines;

                if can_short_circuit_on_size {
                    if let (Some(m1), Some(m2)) = (m1, m2) {
                        if m1.len() != m2.len() {
                            if brief && !quiet {
                                return (false, None, format!("Files {} and {} differ", label1, label2), String::new());
                            } else if quiet {
                                return (false, None, String::new(), String::new());
                            }
                        }
                    }
                }

                let c1 = std::fs::read_to_string(&p1).unwrap_or_default();
                let c2 = std::fs::read_to_string(&p2).unwrap_or_default();
                
                if c1 == c2 {
                    if report_identical_files && !quiet {
                        return (true, None, format!("Files {} and {} are identical", label1, label2), String::new());
                    }
                    return (true, None, String::new(), String::new());
                }

                let res = literal_diff(&c1, &c2, &opts);
                let identical = res.additions.is_empty() && res.deletions.is_empty() && res.modifications.is_empty();
                
                if !identical {
                    if quiet {
                        return (false, None, String::new(), String::new());
                    } else if brief {
                        return (false, None, format!("Files {} and {} differ", label1, label2), String::new());
                    } else if unified {
                        return (false, None, format!("Files {} and {} differ (unified diff not shown in recursive mode)", label1, label2), String::new());
                    } else {
                        return (false, Some(res), String::new(), rel_path.display().to_string());
                    }
                } else if report_identical_files && !quiet {
                    return (true, None, format!("Files {} and {} are identical", label1, label2), String::new());
                }
            } else if in_1 {
                let msg = if quiet { String::new() } else { format!("▶ Orphan: {} (exists only in {})", rel_path.display(), file1.display()) };
                return (false, None, msg, String::new());
            } else {
                let msg = if quiet { String::new() } else { format!("▶ Orphan: {} (exists only in {})", rel_path.display(), file2.display()) };
                return (false, None, msg, String::new());
            }
            (true, None, String::new(), String::new())
        }).collect();

        let mut all_identical = true;
        for (identical, diff_res, msg, rel_path_str) in results {
            if !identical {
                all_identical = false;
            }
            if !msg.is_empty() {
                println!("{}", msg);
            }
            if let Some(res) = diff_res {
                println!("\n▶ Differences in: {}", rel_path_str);
                output::print_literal_diff(&res);
            }
        }
        
        for w in warnings {
            eprintln!("Warning: {}", w);
        }

        return Ok(all_identical);
    }

    let metadata1 = std::fs::metadata(file1).ok();
    let metadata2 = std::fs::metadata(file2).ok();

    let opts = DiffOptions {
        ignore_case: shared.ignore_case,
        ignore_all_space: shared.ignore_all_space,
        ignore_space_change,
        ignore_trailing_space,
        ignore_blank_lines,
        ignore_keys: vec![],
        ignore_keywords: vec![],
    };

    // Fast-path: If no "ignore" flags are set and sizes differ, they must be different.
    if !opts.ignore_case && !opts.ignore_all_space && !opts.ignore_space_change 
       && !opts.ignore_trailing_space && !opts.ignore_blank_lines 
    {
        if let (Some(m1), Some(m2)) = (&metadata1, &metadata2) {
            if m1.len() != m2.len() {
                if !quiet && brief {
                    let l1 = file1.file_name().and_then(|n| n.to_str()).unwrap_or("file1");
                    let l2 = file2.file_name().and_then(|n| n.to_str()).unwrap_or("file2");
                    println!("Files {} and {} differ", l1, l2);
                }
                return Ok(false);
            }
        }
    }

    let content1 = std::fs::read_to_string(file1)
        .map_err(|e| QarenError::from_io_with_path(e, file1.to_path_buf()))?;
    let content2 = std::fs::read_to_string(file2)
        .map_err(|e| QarenError::from_io_with_path(e, file2.to_path_buf()))?;

    // Fast-path: Exact string equality check
    if content1 == content2 {
        if !quiet && report_identical_files {
            let l1 = file1.file_name().and_then(|n| n.to_str()).unwrap_or("file1");
            let l2 = file2.file_name().and_then(|n| n.to_str()).unwrap_or("file2");
            println!("Files {} and {} are identical", l1, l2);
        }
        return Ok(true);
    }

    let result = literal_diff(&content1, &content2, &opts);

    let identical = result.additions.is_empty()
        && result.deletions.is_empty()
        && result.modifications.is_empty();

    let l1 = file1.file_name().and_then(|n| n.to_str()).unwrap_or("file1");
    let l2 = file2.file_name().and_then(|n| n.to_str()).unwrap_or("file2");

    if !quiet {
        if identical && report_identical_files {
            println!("Files {} and {} are identical", l1, l2);
        } else if !identical {
            if brief {
                println!("Files {} and {} differ", l1, l2);
            } else if unified {
                let lines1: Vec<&str> = content1.lines().collect();
                let lines2: Vec<&str> = content2.lines().collect();

                let (norm1, norm2): (Vec<String>, Vec<String>);
                let (refs1, refs2) = if !opts.ignore_case && !opts.ignore_all_space && !opts.ignore_space_change && !opts.ignore_trailing_space {
                    (lines1.clone(), lines2.clone())
                } else {
                    norm1 = lines1.iter().map(|&l| qaren_core::normalise(l, &opts)).collect();
                    norm2 = lines2.iter().map(|&l| qaren_core::normalise(l, &opts)).collect();
                    (
                        norm1.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                        norm2.iter().map(|s| s.as_str()).collect::<Vec<_>>()
                    )
                };

                let diff = similar::TextDiff::from_slices(&refs1, &refs2);
                
                use colored::Colorize;
                println!("--- {}", l1);
                println!("+++ {}", l2);

                for hunk in diff.grouped_ops(3) {
                    let mut old_range = 0..0;
                    let mut new_range = 0..0;
                    if let Some(first) = hunk.first() {
                        old_range.start = first.old_range().start;
                        new_range.start = first.new_range().start;
                    }
                    if let Some(last) = hunk.last() {
                        old_range.end = last.old_range().end;
                        new_range.end = last.new_range().end;
                    }
                    
                    println!("{}", format!("@@ -{},{} +{},{} @@", 
                        old_range.start + 1, old_range.end - old_range.start,
                        new_range.start + 1, new_range.end - new_range.start
                    ).cyan());

                    for op in hunk {
                        match op {
                            similar::DiffOp::Equal { old_index, len, .. } => {
                                for i in 0..len {
                                    println!(" {}", lines1[old_index + i]);
                                }
                            }
                            similar::DiffOp::Delete { old_index, old_len, .. } => {
                                for i in 0..old_len {
                                    if !opts.ignore_blank_lines || !lines1[old_index + i].trim().is_empty() {
                                        println!("{}", format!("-{}", lines1[old_index + i]).red());
                                    }
                                }
                            }
                            similar::DiffOp::Insert { new_index, new_len, .. } => {
                                for i in 0..new_len {
                                    if !opts.ignore_blank_lines || !lines2[new_index + i].trim().is_empty() {
                                        println!("{}", format!("+{}", lines2[new_index + i]).green());
                                    }
                                }
                            }
                            similar::DiffOp::Replace { old_index, old_len, new_index, new_len } => {
                                for i in 0..old_len {
                                    if !opts.ignore_blank_lines || !lines1[old_index + i].trim().is_empty() {
                                        println!("{}", format!("-{}", lines1[old_index + i]).red());
                                    }
                                }
                                for i in 0..new_len {
                                    if !opts.ignore_blank_lines || !lines2[new_index + i].trim().is_empty() {
                                        println!("{}", format!("+{}", lines2[new_index + i]).green());
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                output::print_literal_diff(&result);
            }
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
    recursive: bool,
    output_format: &str,
    ignore_keys: Vec<String>,
    ignore_keywords: Vec<String>,
    quiet: bool,
    summary: bool,
    show_secrets: bool,
    verbose: bool,
    missing_only: bool,
    generate_patch: Option<&Path>,
    mask_patches: bool,
    direction: &str,
    shared: &crate::commands::SharedDiffOptions,
) -> Result<bool, QarenError> {
    // Validate --direction without --generate-missing
    if generate_patch.is_none() && direction != "source-to-target" {
        return Err(QarenError::InvalidArguments(
            "--direction requires --generate-patch (-g)".to_string(),
        ));
    }

    // Read raw file content for auto-detection (only if not recursive, or wait, if recursive we don't read yet)
    let content1 = if !recursive { std::fs::read_to_string(file1).map_err(|e| QarenError::from_io_with_path(e, file1.to_path_buf()))? } else { String::new() };
    let content2 = if !recursive { std::fs::read_to_string(file2).map_err(|e| QarenError::from_io_with_path(e, file2.to_path_buf()))? } else { String::new() };

    let delim1_opt = if !recursive { Some(resolve_delimiter(d1, delimiter, &content1)?) } else { resolve_delimiter(d1, delimiter, "").ok() };
    let delim2_opt = if !recursive { Some(resolve_delimiter(d2, delimiter, &content2)?) } else { resolve_delimiter(d2, delimiter, "").ok() };


    // Build diff options
    let diff_opts = DiffOptions {
        ignore_case: shared.ignore_case,
        ignore_all_space: shared.ignore_all_space,
        ignore_space_change: false,
        ignore_trailing_space: false,
        ignore_blank_lines: false,
        ignore_keys: ignore_keys.clone(),
        ignore_keywords: ignore_keywords.clone(),
    };

    if recursive {
        let dir_opts1 = DirParseOptions {
            delimiter: delim1_opt,
            strip_quotes,
            ..DirParseOptions::default()
        };
        let dir_opts2 = DirParseOptions {
            delimiter: delim2_opt,
            strip_quotes,
            ..DirParseOptions::default()
        };

        if verbose && !quiet {
            println!("[INFO] Recursive scan of file1: {}", file1.display());
            println!("[INFO] Recursive scan of file2: {}", file2.display());
        }

        let mut dir_diff = semantic_diff_dir(file1, file2, &dir_opts1, &dir_opts2, &diff_opts);

        if missing_only {
            let keys_to_remove: Vec<_> = dir_diff.files.iter()
                .filter_map(|(k, v)| if matches!(v, qaren_core::FileDiffStatus::OrphanInTarget(_)) { Some(k.clone()) } else { None })
                .collect();
            for k in keys_to_remove {
                dir_diff.files.remove(&k);
            }

            for status in dir_diff.files.values_mut() {
                if let qaren_core::FileDiffStatus::Modified(res) = status {
                    res.missing_in_file1.clear();
                    res.modified.clear();
                    if res.is_identical() {
                        *status = qaren_core::FileDiffStatus::Identical;
                    }
                }
            }
        }

        if output_format == "json" {
            println!("{}", serde_json::to_string_pretty(&dir_diff).unwrap());
        } else if !quiet {
            use colored::Colorize;
            
            for warning in &dir_diff.traversal_warnings {
                eprintln!("{}: {}", "⚠ Warning".yellow().bold(), warning);
            }

            if summary {
                let l1 = file1.display().to_string().red();
                let l2 = file2.display().to_string().green();
                println!("\n── {} vs {} (Recursive) ──", l1, l2);
                
                let mut identical_count = 0;
                let mut modified_count = 0;
                let mut orphan_source = 0;
                let mut orphan_target = 0;
                
                for status in dir_diff.files.values() {
                    match status {
                        FileDiffStatus::Identical => identical_count += 1,
                        FileDiffStatus::Modified(_) => modified_count += 1,
                        FileDiffStatus::OrphanInSource(_) => orphan_source += 1,
                        FileDiffStatus::OrphanInTarget(_) => orphan_target += 1,
                        _ => {}
                    }
                }
                
                println!(
                    "Summary: {} identical files, {} files only in {}, {} files only in {}, {} modified files",
                    identical_count,
                    orphan_source, file1.display().to_string().red(),
                    orphan_target, file2.display().to_string().green(),
                    modified_count
                );
                println!();
            } else {
                let mut sorted_paths: Vec<_> = dir_diff.files.keys().collect();
                sorted_paths.sort();
                
                for path in sorted_paths {
                    let status = &dir_diff.files[path];
                    match status {
                        FileDiffStatus::Modified(diff) => {
                            let label1 = format!("{}/{}", file1.display(), path.display());
                            let label2 = format!("{}/{}", file2.display(), path.display());
                            println!("\n▶ Modified File: {}", path.display());
                            output::print_diff_result(diff, show_secrets, verbose, &label1, &label2);
                        }
                        FileDiffStatus::OrphanInSource(_) => {
                            println!("▶ Orphan: {} (exists only in {})", path.display().to_string().red(), file1.display());
                        }
                        FileDiffStatus::OrphanInTarget(_) => {
                            println!("▶ Orphan: {} (exists only in {})", path.display().to_string().green(), file2.display());
                        }
                        FileDiffStatus::Error(e) => {
                            eprintln!("▶ Error processing {}: {}", path.display(), e);
                        }
                        _ => {} // Skip Identical and NotAKvFile in non-verbose, or handle them
                    }
                }
            }
        }

        if let Some(output_path) = generate_patch {
            let patch_direction: PatchDirection = direction.parse().map_err(QarenError::InvalidArguments)?;
            let patch_opts1 = ParseOptions { delimiter: delim1_opt.unwrap_or('='), strip_quotes, ..ParseOptions::default() };
            let patch_opts2 = ParseOptions { delimiter: delim2_opt.unwrap_or('='), strip_quotes, ..ParseOptions::default() };
            
            let created_files = qaren_core::generate_recursive_patch(&dir_diff, output_path, &patch_opts1, &patch_opts2, patch_direction, mask_patches)?;
            if !quiet {
                println!("\nGenerated {} recursive patch files under {}", created_files.len(), output_path.display());
            }
        }
        
        if !dir_diff.traversal_warnings.is_empty() {
            return Err(QarenError::Io(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Directory traversal encountered permission or I/O errors",
            )));
        }

        return Ok(dir_diff.is_identical());
    }

    // --- Non-recursive mode (original logic) ---

    // Build per-file parse options
    let delim1 = delim1_opt.unwrap();
    let delim2 = delim2_opt.unwrap();

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
    let mut diff_result = semantic_diff(&config1, &config2, &diff_opts);

    if missing_only {
        diff_result.missing_in_file1.clear();
        diff_result.modified.clear();
    }

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

        if let Some(output_path) = generate_patch {
            let patch_direction = direction.parse::<PatchDirection>()
                .map_err(QarenError::InvalidArguments)?;
            
            let patch_opts1 = ParseOptions {
                delimiter: opts1.delimiter,
                strip_quotes: opts1.strip_quotes,
                comment_prefixes: opts1.comment_prefixes.clone(),
                ignore_case: opts1.ignore_case,
            };
            let patch_opts2 = ParseOptions {
                delimiter: opts2.delimiter,
                strip_quotes: opts2.strip_quotes,
                comment_prefixes: opts2.comment_prefixes.clone(),
                ignore_case: opts2.ignore_case,
            };

            let created_files = qaren_core::generate_patch(&diff_result, output_path, &patch_opts1, &patch_opts2, patch_direction, mask_patches)?;
            if !quiet {
                println!("\nGenerated {} patch file(s) for missing keys.", created_files.len());
            }
        }

        if !summary && d1.is_none() && d2.is_none() && delimiter.is_none() && delim1 != delim2 {
            eprintln!(
                "  ℹ auto-detected delimiters: '{}' for {}, '{}' for {}",
                delim1, label1, delim2, label2
            );
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
