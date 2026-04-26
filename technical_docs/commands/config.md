# Command: `qaren config`

The `qaren config` command manages global, persistent settings for Qaren. These settings allow you to customize Qaren's behavior (such as color output and exit code semantics) without needing to pass flags on every execution.

## Usage

```bash
qaren config [WHAT] [ACTION]
```

---

## Detailed Options Reference

### `show` (The Default Command)
**Use Case:** View the entire current configuration state, including the path to the configuration file on your system.
**Command Example:**
```bash
qaren config show
```
**Output Example:**
```text
Config file: /home/user/.config/qaren/config

  exit nonzero-on-diff : enabled  (exit 1 when differences found)
  color output         : enabled
```

### `exit`
**Use Case:** Control how Qaren exits when differences are found. Essential for making Qaren "pipeline-friendly" in strict CI/CD environments.
**Actions:**
- `show`: View the current exit code behavior.
- `toggle`: Switch between standard POSIX mode (exit 1 on diff) and Pipeline-Friendly mode (always exit 0).

**Command Example (Toggle):**
```bash
qaren config exit toggle
```
**Output Example:**
```text
✔ exit nonzero-on-diff: disabled — always exit 0 on success
```

### `color`
**Use Case:** Globally enable or disable ANSI color output. Useful for legacy terminals, text-only environments, or strict logging aggregators.
**Actions:**
- `show`: View current color settings.
- `toggle`: Enable or disable color output.

**Command Example (Toggle):**
```bash
qaren config color toggle
```

### `path`
**Use Case:** Quickly retrieve the absolute path to the Qaren configuration file for manual editing or backup.
**Command Example:**
```bash
qaren config path
```
**Output Example:**
```text
/home/user/.config/qaren/config
```

---

## Configuration File Locations

Qaren follows platform-native standards for storing configuration:

- **Linux / macOS:** `$XDG_CONFIG_HOME/qaren/config` (typically `~/.config/qaren/config`)
- **Windows:** `%APPDATA%\qaren\config`

---

## Automation Best Practices

In automated environments, you can pre-configure Qaren before running audits to ensure predictable behavior:

```bash
# Ensure the pipeline doesn't fail due to exit codes
qaren config exit toggle

# Run the audit and capture results
qaren kv prod.env staging.env -o json > report.json
```

---
[Return to Index](../README.md) | [See `diff` Command](diff.md) | [See `kv` Command](kv.md)
