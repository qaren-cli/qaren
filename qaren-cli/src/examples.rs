use colored::Colorize;

/// Display rich usage examples. 
pub fn print_examples(subcmd: Option<&str>) {
    match subcmd {
        Some("kv") => {
            println!("\n  {} {} {}\n", "Qaren KV".bold().cyan(), "—".bright_black(), "Semantic Configuration Comparison".bold().white());
            println!("  {}\n", "Compare configurations intelligently (ignoring order and formatting differences).".bright_black());
            
            println!("    {} {} {}", "$".bright_black(), "qaren kv".green(), "prod.env staging.env");
            println!("    {}\n", "Compare two files showing only keys that differ or are missing.".bright_black());

            println!("    {} {} {}", "$".bright_black(), "qaren kv".green(), "prod.env staging.yaml --d2 ':'");
            println!("    {}\n", "Cross-format comparison (.env vs .yaml) using explicit delimiter logic.".bright_black());

            println!("    {} {} {}", "$".bright_black(), "qaren kv".green(), "prod.env staging.env -i -w");
            println!("    {}\n", "Compare ignoring case (-i) and stripping all values of white spaces (-w).".bright_black());

            println!("    {} {} {}", "$".bright_black(), "qaren kv".green(), "prod.env staging.env -g missing.env");
            println!("    {}\n", "Generate an intelligent patch file with all keys missing from prod.env.".bright_black());
            println!();
        }
        Some("diff") => {
            println!("\n  {} {} {}\n", "Qaren Diff".bold().cyan(), "—".bright_black(), "Literal Line-by-Line Comparison".bold().white());
            println!("  {}\n", "Strict POSIX-compliant literal comparison with formatting preservation.".bright_black());

            println!("    {} {} {}", "$".bright_black(), "qaren diff".green(), "file1.txt file2.txt -u");
            println!("    {}\n", "Show differences in the highly-readable Unified Diff (-u) format.".bright_black());

            println!("    {} {} {}", "$".bright_black(), "qaren diff".green(), "file1.txt file2.txt -w -B -i");
            println!("    {}\n", "Compare ignoring white space (-w), blank lines (-B), and case (-i).".bright_black());

            println!("    {} {} {}", "$".bright_black(), "qaren diff".green(), "file1.txt file2.txt -q");
            println!("    {}\n", "Brief output; only exit 1 and report if files differ, suppress all output.".bright_black());
            println!();
        }
        _ => {
            println!("\n  {} {} {}\n", "Qaren (قارن)".bold().magenta(), "—".bright_black(), "Blazingly fast configuration comparison tool.".bold().white());
            
            println!("  {}\n", "CORE EXAMPLES:".bold().cyan());
            println!("    {} {} {}", "$".bright_black(), "qaren kv".green(), "prod.env staging.env -g patch.env");
            println!("    {}\n", "Compare two dot-env configurations and safely patch missing keys.".bright_black());

            println!("    {} {} {}", "$".bright_black(), "qaren diff".green(), "config1.ini config2.ini -u");
            println!("    {}\n", "Compute literal unified diffs between typical structural files.".bright_black());

            println!("    {} {} {}", "$".bright_black(), "qaren config".green(), "exit toggle");
            println!("    {}\n", "Toggle pipeline-friendly exit codes automatically.".bright_black());

            println!("  {} {}\n  {}\n", "➔".bright_black(), "Want format-specific examples?".bold().cyan(), 
                     "Run `qaren <command> --example` (e.g. `qaren kv --example`)".bright_black());
        }
    }
}
