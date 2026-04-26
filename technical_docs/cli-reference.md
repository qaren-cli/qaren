# CLI Reference & Global Options

Qaren integrates cleanly into standard Unix toolchains and CI/CD pipelines via POSIX-standard exit codes and global shell compatibility.

## Usage Structure

```bash
qaren [OPTIONS] [COMMAND]
```

**Commands:**
*   [`diff`](commands/diff.md): Perform literal line-by-line comparison.
*   [`kv`](commands/kv.md) (alias `kvp`): Perform semantic key-value comparison.
*   [`config`](commands/config.md): View and modify persistent Qaren settings.
*   `help`: Print help information.

---

## Global Options

The following options apply to the Qaren CLI binary as a whole:

### `--example`
Prints detailed, rich usage examples for Qaren or specific subcommands.
```bash
qaren --example
qaren kv --example
```

### `--generate-completions <SHELL>`
Generates shell autocompletion scripts for your terminal environment. 
Supported shells: `bash`, `elvish`, `fish`, `powershell`, `zsh`.

**Example: Bash Installation**
```bash
qaren --generate-completions bash > ~/.local/share/bash-completion/completions/qaren
```

**Example: Zsh Installation**
```bash
qaren --generate-completions zsh > ~/.zfunc/_qaren
```

### `-h, --help`
Prints the standard help menu. Use `-h` for a brief summary and `--help` for extended documentation.

### `-V, --version`
Prints the current Qaren version.

---

## Exit Codes & Automation

Qaren is designed for scripting and pipeline automation. It adheres strictly to standard Unix exit codes:

| Code | Meaning | Description |
| :--- | :--- | :--- |
| **`0`** | **Identical / Success** | The files are identical (or pipeline-friendly mode is enabled via `qaren config`). |
| **`1`** | **Differences Found** | Differences exist between the source and target files. (Standard POSIX diff behavior). |
| **`2`** | **Error** | A system or input error occurred (e.g., file not found, permission denied, invalid arguments). |

### Handling Exit Codes in Scripts

You can easily build conditional logic around Qaren's exit codes:

```bash
if qaren kv prod.env staging.env --quiet; then
    echo "Environments are synchronized."
else
    echo "Environments have drifted!"
    exit 1
fi
```

---
[Return to Index](README.md)
