//! Terminal output formatting for comparison results.
//!
//! Handles coloured output for diff results and summary lines.
//! Identical keys are hidden by default; pass `verbose = true` to show them.
//! Pass `brief = true` to suppress per-key details and only print the summary.

use colored::Colorize;
use qaren_core::{DiffResult, LiteralDiffResult};

use crate::masking::mask_value;

/// Print the result of a semantic key-value comparison to stdout.
///
/// - `show_secrets` — when `true`, skip masking and show raw values
/// - `verbose`      — when `true`, also display identical keys
/// - `brief`        — when `true`, print summary line only (no per-key output)
/// - `label1`       — display name for file1 (e.g. the filename stem)
/// - `label2`       — display name for file2
pub fn print_diff_result(
    result: &DiffResult,
    show_secrets: bool,
    verbose: bool,
    brief: bool,
    label1: &str,
    label2: &str,
) {
    if result.is_identical() {
        println!("{}", "✔ Files are identical".green().bold());
        return;
    }

    if !brief {
        // ── Only in file1 (missing from file2) ──────────────────────
        if !result.missing_in_file2.is_empty() {
            println!(
                "\n{}",
                format!(
                    "── Only in {} ({} keys) ──",
                    label1,
                    result.missing_in_file2.len()
                )
                .red()
                .bold()
            );
            for pair in &result.missing_in_file2 {
                let display_val = mask_value(&pair.key, &pair.value, show_secrets);
                println!("  {} {}={}", "-".red(), pair.key.red(), display_val.red());
            }
        }

        // ── Only in file2 (missing from file1) ──────────────────────
        if !result.missing_in_file1.is_empty() {
            println!(
                "\n{}",
                format!(
                    "── Only in {} ({} keys) ──",
                    label2,
                    result.missing_in_file1.len()
                )
                .green()
                .bold()
            );
            for pair in &result.missing_in_file1 {
                let display_val = mask_value(&pair.key, &pair.value, show_secrets);
                println!("  {} {}={}", "+".green(), pair.key.green(), display_val.green());
            }
        }

        // ── Modified keys ────────────────────────────────────────────
        if !result.modified.is_empty() {
            println!(
                "\n{}",
                format!("── Modified ({} keys) ──", result.modified.len())
                    .yellow()
                    .bold()
            );
            for m in &result.modified {
                let val1 = mask_value(&m.key, &m.value_file1, show_secrets);
                let val2 = mask_value(&m.key, &m.value_file2, show_secrets);
                println!(
                    "  {} {}: {} → {}",
                    "~".yellow(),
                    m.key.yellow(),
                    val1.red(),
                    val2.green()
                );
            }
        }

        // ── Identical keys (only when --verbose / -v) ────────────────
        if verbose && !result.identical.is_empty() {
            println!(
                "\n{}",
                format!("── Identical ({} keys) ──", result.identical.len())
                    .dimmed()
                    .bold()
            );
            let mut sorted = result.identical.clone();
            sorted.sort();
            for key in &sorted {
                println!("  {} {}", "=".dimmed(), key.dimmed());
            }
        }
    }

    // ── Summary ──────────────────────────────────────────────────────
    println!();
    println!(
        "{}",
        format!(
            "Summary: {} only in {}, {} only in {}, {} modified{}",
            result.missing_in_file2.len(),
            label1,
            result.missing_in_file1.len(),
            label2,
            result.modified.len(),
            if result.identical.is_empty() {
                String::new()
            } else if verbose {
                format!(", {} identical", result.identical.len())
            } else {
                format!(", {} identical (use -v to show)", result.identical.len())
            }
        )
        .bold()
    );
}

/// Print the result of a literal line-by-line comparison to stdout.
pub fn print_literal_diff(result: &LiteralDiffResult) {
    let has_changes = !result.additions.is_empty()
        || !result.deletions.is_empty()
        || !result.modifications.is_empty();

    if !has_changes {
        println!("{}", "✔ Files are identical".green().bold());
        return;
    }

    // ── Deletions ────────────────────────────────────────────────────
    for line in &result.deletions {
        let content = line.content.trim_end();
        println!(
            "{}",
            format!("-[L{}] {}", line.line_number, content).red()
        );
    }

    // ── Additions ────────────────────────────────────────────────────
    for line in &result.additions {
        let content = line.content.trim_end();
        println!(
            "{}",
            format!("+[L{}] {}", line.line_number, content).green()
        );
    }

    // ── Modifications ────────────────────────────────────────────────
    for (old, new) in &result.modifications {
        let old_content = old.content.trim_end();
        let new_content = new.content.trim_end();
        println!(
            "{}",
            format!("~[L{}] {} → {}", old.line_number, old_content, new_content).yellow()
        );
    }

    // ── Summary ──────────────────────────────────────────────────────
    println!();
    println!(
        "{}",
        format!(
            "Summary: {} additions, {} deletions, {} modifications",
            result.additions.len(),
            result.deletions.len(),
            result.modifications.len()
        )
        .bold()
    );
}
