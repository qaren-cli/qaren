# config - Settings Management

The `config` command allows you to view and modify persistent Qaren settings, stored locally in your user configuration directory.

**Storage Paths:**
- Linux/macOS: `$XDG_CONFIG_HOME/qaren/config` (`~/.config/qaren/config`)
- Windows: `%APPDATA%\qaren\config`

## Usage

```bash
qaren config [OPTIONS] [WHAT] [ACTION]
```

## Options and Flags

| Argument | Description | Options |
|----------|-------------|---------|
| `WHAT` | The setting to configure | `exit`, `color`, `show`, `path` |
| `ACTION` | The action to perform | `show` (default), `toggle` |

## Modifying CI/CD Exit Behaviors

By default, Qaren mimics POSIX `diff` and exits with code `1` if differences are found. In some pipeline setups, you may prefer it to exit `0` and parse the output instead. You can toggle this behavior persistently:

```bash
# Toggle to pipeline-friendly mode (always exit 0 on diffs)
qaren config exit toggle

# Check current exit mode
qaren config exit show
```

## Other Settings

```bash
# View configuration paths
qaren config path show

# Toggle colorized output globally
qaren config color toggle
```

---
[Return to Index](../README.md) | [Review CI/CD Automation](../guides/automation-and-cicd.md)
