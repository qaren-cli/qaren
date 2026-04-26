# Configuration & Persistent Settings

The `qaren config` command manages global, persistent settings. This allows you to tailor Qaren to your terminal environment or CI/CD runner without needing to pass the same flags on every execution.

## Usage

```bash
qaren config [WHAT] [ACTION]
```

## Options and Flags

| Argument | Description | Options |
|----------|-------------|---------|
| `WHAT` | The setting to configure | `exit`, `color`, `show`, `path` |
| `ACTION` | The action to perform | `show` (default), `toggle` |

## Configuration State File

Settings are stored in your platform's native config directory:
- **Linux/macOS:** `$XDG_CONFIG_HOME/qaren/config` (typically `~/.config/qaren/config`)
- **Windows:** `%APPDATA%\qaren\config`

## Viewing Configuration

To see your current configuration state:
```bash
qaren config show
```

*Example Output:*
```text
Config file: /home/user/.config/qaren/config

  exit nonzero-on-diff : enabled  (exit 1 when differences found)
  color output         : enabled
```

## Managing Exit Codes (Pipeline Friendly Mode)

By default, Qaren follows the POSIX standard: it exits with `0` if files are identical, and exits with `1` if differences are found.

In some automated deployment pipelines (like Jenkins, GitLab CI, or GitHub Actions), a non-zero exit code will immediately fail the pipeline. If you are using Qaren purely for auditing or generating JSON reports in a CI step, you might want it to always return `0` on successful execution, regardless of whether differences exist.

**Toggle Pipeline-Friendly Mode:**
```bash
qaren config exit toggle
```
*Output:* `✔ exit nonzero-on-diff: disabled — always exit 0 on success`

*(Running the command again will toggle it back to standard behavior).*

## Managing Colors

If you want to globally disable ANSI color output from Qaren (for legacy terminals or strict text logging):

```bash
qaren config color toggle
```

## Path Management

To see the exact path Qaren is using for its configuration:
```bash
qaren config path show
```

---

## Best Practices for Automation

If you are deploying Qaren inside Docker containers or ephemeral CI runners, you can configure it on the fly:

```yaml
steps:
  - name: Install Qaren
    run: cargo install qaren
    
  - name: Configure Qaren for CI
    run: qaren config exit toggle
    
  - name: Audit Configuration Drift
    run: qaren kv ./production.env ./staging.env -o json > drift_report.json
```

---
[Return to Index](../README.md) | [See `diff` Command](diff.md) | [See `kv` Command](kv.md)
