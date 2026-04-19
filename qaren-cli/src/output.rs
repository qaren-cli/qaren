//! Terminal output formatting for comparison results.
//!
//! Handles colored output for diff results (green = additions,
//! red = deletions, yellow = modifications) and summary lines.

use colored::Colorize;
use qaren_core::{DiffResult, LiteralDiffResult};

use crate::masking::mask_value;

/// Print the result of a semantic key-value comparison to stdout.
///
/// Output is grouped into sections: Missing in file2, Missing in file1,
/// Modified, and Identical. Values are masked according to the
/// `show_secrets` flag.
pub fn print_diff_result(result: &DiffResult, show_secrets: bool) {
    if result.is_identical() {
        println!("{}", "✔ Files are identical".green().bold());
        return;
    }

    // ── Missing in file2 (present in source, absent in target) ──
    if !result.missing_in_file2.is_empty() {
        println!(
            "\n{}",
            format!(
                "── Missing in file2 ({} keys) ──",
                result.missing_in_file2.len()
            )
            .red()
            .bold()
        );
        for pair in &result.missing_in_file2 {
            let display_val = mask_value(&pair.key, &pair.value, show_secrets);
            println!("  {} {}: {}", "-".red(), pair.key.red(), display_val.red());
        }
    }

    // ── Missing in file1 (present in target, absent in source) ──
    if !result.missing_in_file1.is_empty() {
        println!(
            "\n{}",
            format!(
                "── Missing in file1 ({} keys) ──",
                result.missing_in_file1.len()
            )
            .green()
            .bold()
        );
        for pair in &result.missing_in_file1 {
            let display_val = mask_value(&pair.key, &pair.value, show_secrets);
            println!(
                "  {} {}: {}",
                "+".green(),
                pair.key.green(),
                display_val.green()
            );
        }
    }

    // ── Modified keys ──
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

    // ── Identical keys ──
    if !result.identical.is_empty() {
        println!(
            "\n{}",
            format!("── Identical ({} keys) ──", result.identical.len())
                .dimmed()
                .bold()
        );
        for key in &result.identical {
            println!("  {} {}", "=".dimmed(), key.dimmed());
        }
    }

    // ── Summary ──
    println!();
    println!(
        "{}",
        format!(
            "Summary: {} missing in file2, {} missing in file1, {} modified, {} identical",
            result.missing_in_file2.len(),
            result.missing_in_file1.len(),
            result.modified.len(),
            result.identical.len()
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

    // ── Deletions ──
    for line in &result.deletions {
        let content = line.content.trim_end();
        println!(
            "{}",
            format!("-[L{}] {}", line.line_number, content).red()
        );
    }

    // ── Additions ──
    for line in &result.additions {
        let content = line.content.trim_end();
        println!(
            "{}",
            format!("+[L{}] {}", line.line_number, content).green()
        );
    }

    // ── Modifications ──
    for (old, new) in &result.modifications {
        let old_content = old.content.trim_end();
        let new_content = new.content.trim_end();
        println!(
            "{}",
            format!("~[L{}] {} → {}", old.line_number, old_content, new_content).yellow()
        );
    }

    // ── Summary ──
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
