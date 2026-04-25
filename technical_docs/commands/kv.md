# kv - Semantic Key-Value Comparison

The `kv` command (alias: `kvp`) is the core of Qaren's capabilities. It performs a semantic, order-agnostic comparison of Key-Value configurations. It natively understands formats like `KEY=VALUE` and `KEY: VALUE` and automatically detects delimiters.

## Usage

```bash
qaren kv [OPTIONS] [FILE1] [FILE2]
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
| `-S` | `--show-secrets` | Show secret values in plain text (disables masking) |
| `-v` | `--verbose` | Show identical keys in output |
| | `--missing-only` | Show only keys present in source but missing in target |
| `-g` | `--generate-patch <FILE>` | Generate a patch file containing missing keys |
| | `--mask-patches` | Mask secrets in generated patch files |
| | `--direction <DIR>` | Patch direction: `source-to-target`, `target-to-source`, `bidirectional` |

## Examples

**Basic comparison with auto-detection:**
```bash
qaren kv staging.env production.env
```

**Checking for missing keys only:**
```bash
qaren kv dev.env staging.env --missing-only
```

**Cross-format comparison (YAML-like to ENV-like):**
```bash
qaren kv config.yml variables.env --d1 ":" --d2 "="
```

**Generating a masked patch file:**
```bash
qaren kv staging.env production.env -g update.patch --mask-patches
```

---
[Return to Index](../README.md) | [See diff Command](diff.md) | [See Config Management](config.md)
