use colored::Colorize;

/// Display rich usage examples. 
pub fn print_examples(subcmd: Option<&str>) {
    match subcmd {
        Some("kv") => {
            println!("\n  {} {} {}\n", "Qaren KV".bold().cyan(), "—".bright_black(), "Semantic Configuration Comparison".bold().white());
            println!("  {}\n", "Compare configurations intelligently (ignoring order and formatting differences).".bright_black());
            
            println!("    {} {} prod.env staging.env", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Default behavior: compare two files showing only keys that differ or are missing.".bright_black());

            println!("    {} {} prod.env staging.env -g patch.env", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Generate an intelligent patch file containing keys missing from prod.env.".bright_black());
            
            println!("    {} {} prod.env staging.env -g sync.env --direction bidirectional", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Generate two patch files (one for each direction) to synchronize both files.".bright_black());

            println!("    {} {} app.env ci.env -x SECRET_KEY -x DB_PASSWORD", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Ignore specific sensitive or dynamic keys from the comparison.".bright_black());

            println!("    {} {} build.env staging.env --ignore-keyword GITHUB", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Exclude all keys containing 'GITHUB' (case-insensitive substring match).".bright_black());

            println!("    {} {} api.env local.env --output json", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Export comparison results in machine-readable JSON format for automation.".bright_black());

            println!("    {} {} s1.env s2.env -s", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Summary mode: Aggregated warnings and compact diff headers (Red/Green).".bright_black());

            println!("    {} {} p1.env p2.env -q", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Quiet mode: Absolute silence. Use for shell script conditionals (check exit code).".bright_black());

            println!("    {} {} prod.env staging.yaml --d2 ':'", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Cross-format comparison (.env vs .yaml) using explicit delimiter logic.".bright_black());

            println!("    {} {} ./prod_configs ./staging_configs", "$".bright_black(), "qaren kv -r".green());
            println!("    {}\n", "Recursively compare entire directories, intelligently detecting missing files and keys.".bright_black());
            println!();
        }
        Some("diff") => {
            println!("\n  {} {} {}\n", "Qaren Diff".bold().cyan(), "—".bright_black(), "Literal Line-by-Line Comparison".bold().white());
            println!("  {}\n", "Strict POSIX-compliant literal comparison with formatting preservation.".bright_black());

            println!("    {} {} file1.txt file2.txt -u", "$".bright_black(), "qaren diff".green());
            println!("    {}\n", "Show differences in the highly-readable Unified Diff (-u) format.".bright_black());

            println!("    {} {} file1.txt file2.txt -w -B -i", "$".bright_black(), "qaren diff".green());
            println!("    {}\n", "Compare ignoring white space (-w), blank lines (-B), and case (-i).".bright_black());

            println!("    {} {} file1.txt file2.txt -q", "$".bright_black(), "qaren diff".green());
            println!("    {}\n", "Quiet mode: Suppress all output. Exit 0 if identical, 1 if different.".bright_black());

            println!("    {} {} ./dir1 ./dir2", "$".bright_black(), "qaren diff -r".green());
            println!("    {}\n", "Recursively compare directories (Literal diff recursively).".bright_black());
            println!();
        }
        _ => {
            println!("\n  {} {} {}\n", "Qaren".bold().magenta(), "—".bright_black(), "Blazingly fast configuration comparison tool.".bold().white());
            
            println!("  {}\n", "CORE EXAMPLES:".bold().cyan());
            println!("    {} {} prod.env staging.env -x DYNAMIC_URL --output json", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Ignore non-static keys and pipe the results as JSON into other tools.".bright_black());

            println!("    {} {} config.dev config.prod -s", "$".bright_black(), "qaren kv".green());
            println!("    {}\n", "Get a high-level summary of configuration differences.".bright_black());

            println!("    {} {} exit toggle", "$".bright_black(), "qaren config".green());
            println!("    {}\n", "Toggle pipeline-friendly exit code behavior.".bright_black());

            println!("    {} {}", "$".bright_black(), "qaren --generate-completions bash > ~/.local/share/bash-completion/completions/qaren".green());
            println!("    {}\n", "Generate shell autocompletion script (supports bash, zsh, fish, powershell, elvish).".bright_black());

            println!("  {} {}\n  {}\n", "➔".bright_black(), "Want format-specific examples?".bold().cyan(), 
                     "Run `qaren <command> --example` (e.g. `qaren kv --example`)".bright_black());
        }
    }
}
