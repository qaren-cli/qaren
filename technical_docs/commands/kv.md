# Command: `qaren kv`

The `qaren kv` (alias: `kvp`) command is the core of Qaren's capabilities. It performs a semantic, order-agnostic comparison of Key-Value configurations. It natively understands formats like `KEY=VALUE` and `KEY: VALUE` and automatically detects delimiters.

If you manage `.env`, `.yaml`, `.ini`, or unstructured log key-values, this command will save you hours of manual auditing.

## Usage

```bash
qaren kv [OPTIONS] [FILE1] [FILE2]
```
*(Note: FILE1 is treated as the Source/Reference, FILE2 is the Target).*

## Core Features

### 1. Semantic Awareness & Auto-Detection
Qaren automatically detects delimiters. It doesn't matter if `file1.env` uses `=` and `file2.yaml` uses `:`. Qaren parses the data, not the syntax.

### 2. Zero-Trust Secrets Masking
By default, Qaren identifies value differences but redacts the actual sensitive strings in the terminal output with `***MASKED***`.

*   **`-S, --show-secrets`**
    Disables masking and shows the actual plain text values that differ.
    ```bash
    qaren kv -S source.env target.env
    ```

## Options and Flags

| Short | Long Flag | Description |
|-------|-----------|-------------|
| `-d` | `--delimiter <DELIMITER>` | Delimiter for BOTH files. Auto-detected if omitted. |
| | `--d1 <DELIMITER>` | Delimiter override for file1 only |
| | `--d2 <DELIMITER>` | Delimiter override for file2 only |
| `-Q` | `--strip-quotes` | Strip surrounding quotes from keys and values |
| `-i` | `--ignore-case` | Ignore case differences in file contents |
| `-w` | `--ignore-all-space` | Ignore all white space (strips spaces inside values) |
| `-r` | `--recursive` | Recursively compare directories containing KV files |
| `-o` | `--output <FORMAT>` | Output format (`text` or `json`). Default: `text` |
| `-x` | `--ignore-key <KEY>` | Ignore a specific key (exact match). Can be repeated. |
| | `--ignore-keyword <KWD>` | Ignore keys containing this keyword (substring) |
| `-q` | `--quiet` | Quiet mode - no stdout/stderr output. Return exit code only. |
| `-s` | `--summary` | Summary mode - minimize output, aggregate warnings |
| `-P` | `--no-perm-warn` | Silences warnings about insecure file permissions |
| `-D` | `--no-duplicate-warn` | Silences warnings about duplicate keys |
| | `--missing-only` | Show *only* keys present in source but missing in target |
| `-g` | `--generate-patch <FILE>` | Generate a patch file containing missing keys |
| | `--mask-patches` | Mask secrets in generated patch files |
| | `--direction <DIR>` | Patch direction: `source-to-target`, `target-to-source`, `bidirectional` |

---

## Intelligent Patching (`-g, --generate-patch`)

Qaren doesn't just find problems; it fixes them. 

*   **`-g, --generate-patch <FILE>`**
    Generates a patch file containing keys that are missing.
*   **`--direction <DIR>`**
    *   `source-to-target` (default): Extracts keys in FILE1 missing from FILE2.
    *   `target-to-source`: Extracts keys in FILE2 missing from FILE1.
    *   `bidirectional`: Creates two patch files concurrently to sync both environments.
*   **`--mask-patches`**
    By default, generated patches contain the *real* values to allow instant syncing. If you are sharing a patch file securely over Slack/Email and want to redact the secrets, use this flag.

**Example: Safe Synchronization**
```bash
# Find what's in prod that staging lacks, and create a sync file
qaren kv prod.env staging.env -g staging-sync.env
```

## More Examples

**Cross-format comparison (YAML-like to ENV-like):**
```bash
qaren kv config.yml variables.env --d1 ":" --d2 "="
```

**Output as machine-readable JSON:**
```bash
qaren kv prod.env staging.env -o json
```

---
[Return to Index](../README.md) | [See `diff` Command](diff.md) | [Persistent Configuration](config.md)
