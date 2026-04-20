//! Persistent user configuration for Qaren.
//!
//! Configuration is stored in a plain TOML-style key=value file at:
//!   - Linux/macOS: `$XDG_CONFIG_HOME/qaren/config` (default `~/.config/qaren/config`)
//!   - Windows:     `%APPDATA%\qaren\config`
//!
//! Parsed manually — no extra crates, compliant with the strict-dependency rule.

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// The runtime configuration read from the config file.
#[derive(Debug, Clone)]
pub struct QarenConfig {
    /// When `true` (default), exit with code 1 when differences are found (like POSIX `diff`).
    /// When `false`, exit 0 on any successful run, 2 on error.
    pub exit_nonzero_on_diff: bool,
    /// When `true` (default), output uses ANSI colour codes.
    pub color: bool,
}

impl Default for QarenConfig {
    fn default() -> Self {
        Self {
            exit_nonzero_on_diff: true, // POSIX default
            color: true,
        }
    }
}

/// Return the path to the config directory, creating it if necessary.
fn config_dir() -> Option<PathBuf> {
    // XDG / Platform-specific config home
    let base = if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg)
    } else if let Ok(appdata) = std::env::var("APPDATA") {
        // Windows fallback
        PathBuf::from(appdata)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".config")
    } else {
        return None;
    };

    Some(base.join("qaren"))
}

/// Return the full path to the config file.
pub fn config_path() -> Option<PathBuf> {
    config_dir().map(|d| d.join("config"))
}

/// Load configuration from disk. Returns [`QarenConfig::default()`] if the
/// file does not exist or cannot be parsed — never fails.
pub fn load_config() -> QarenConfig {
    let path = match config_path() {
        Some(p) => p,
        None => return QarenConfig::default(),
    };

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return QarenConfig::default(),
    };

    let mut cfg = QarenConfig::default();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "exit_nonzero_on_diff" => {
                    cfg.exit_nonzero_on_diff = value.trim() == "true";
                }
                "color" => {
                    cfg.color = value.trim() != "false";
                }
                _ => {} // Unknown keys are silently ignored (forward-compatible)
            }
        }
    }
    cfg
}

/// Save the configuration to disk. Creates the directory if needed.
fn save_config(cfg: &QarenConfig) -> Result<(), io::Error> {
    let dir = match config_dir() {
        Some(d) => d,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Cannot determine config directory (HOME or APPDATA not set)",
            ))
        }
    };

    fs::create_dir_all(&dir)?;
    let path = dir.join("config");

    let mut file = fs::File::create(&path)?;
    writeln!(file, "# Qaren configuration")?;
    writeln!(file, "# exit_nonzero_on_diff: true = exit 1 when diffs found (like POSIX diff)")?;
    writeln!(file, "exit_nonzero_on_diff={}", cfg.exit_nonzero_on_diff)?;
    writeln!(file, "")?;
    writeln!(file, "# color: false = disable ANSI color output")?;
    writeln!(file, "color={}", cfg.color)?;
    Ok(())
}

/// Display the current configuration values.
pub fn cmd_config_show(cfg: &QarenConfig) {
    let path_display = config_path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "<unknown>".to_string());

    println!("Config file: {}", path_display);
    println!();

    let exit_status = if cfg.exit_nonzero_on_diff {
        "enabled  (exit 1 when differences found)"
    } else {
        "disabled (always exit 0 on success)"
    };
    println!("  exit nonzero-on-diff : {}", exit_status);

    let color_status = if cfg.color { "enabled" } else { "disabled" };
    println!("  color output         : {}", color_status);
}

/// Toggle the exit-on-diff behaviour and save.
pub fn cmd_exit_toggle(cfg: &mut QarenConfig) {
    cfg.exit_nonzero_on_diff = !cfg.exit_nonzero_on_diff;
    match save_config(cfg) {
        Ok(_) => {
            let state = if cfg.exit_nonzero_on_diff {
                "enabled  — exit 1 when differences found"
            } else {
                "disabled — always exit 0 on success"
            };
            println!("✔ exit nonzero-on-diff: {}", state);
        }
        Err(e) => eprintln!("Error saving config: {}", e),
    }
}

/// Toggle the color output and save.
pub fn cmd_color_toggle(cfg: &mut QarenConfig) {
    cfg.color = !cfg.color;
    match save_config(cfg) {
        Ok(_) => {
            let state = if cfg.color { "enabled" } else { "disabled" };
            println!("✔ color output: {}", state);
        }
        Err(e) => eprintln!("Error saving config: {}", e),
    }
}

/// Show the path to the config file (useful for scripting).
pub fn cmd_config_path() {
    match config_path() {
        Some(p) => println!("{}", p.display()),
        None => eprintln!("Cannot determine config path"),
    }
}

/// Handle `qaren config <subcommand>` dispatch.
pub fn handle_config_command(what: &str, action: &str) {
    let mut cfg = load_config();
    match (what, action) {
        ("exit", "show") => {
            let state = if cfg.exit_nonzero_on_diff {
                "enabled  (exit 1 when differences found)"
            } else {
                "disabled (always exit 0 on success)"
            };
            println!("exit nonzero-on-diff: {}", state);
        }
        ("exit", "toggle") => cmd_exit_toggle(&mut cfg),
        ("color", "show") => {
            let state = if cfg.color { "enabled" } else { "disabled" };
            println!("color output: {}", state);
        }
        ("color", "toggle") => cmd_color_toggle(&mut cfg),
        ("show", _) | ("", "") => cmd_config_show(&cfg),
        ("path", _) => cmd_config_path(),
        _ => {
            eprintln!("Unknown config command: {} {}", what, action);
            eprintln!("Usage: qaren config <exit|color> <show|toggle>");
            eprintln!("       qaren config show");
        }
    }
}
