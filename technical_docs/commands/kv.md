# Command: `qaren kv`

The `qaren kv` (alias: `kvp`) command is the semantic core of Qaren. It performs order-agnostic comparison of Key-Value configuration files. It natively understands data structures, automatically masks secrets, and provides intelligent environment synchronization tools.

## Usage

```bash
qaren kv [OPTIONS] [FILE1] [FILE2]
```
*(Note: FILE1 is treated as the Source/Reference, FILE2 is the Target).*

---

## Detailed Options Reference

### `-d, --delimiter <DELIMITER>`
**Use Case:** Explicitly set the delimiter for both files (e.g., if auto-detection fails or you want to force a specific parsing logic).
**Command Example:**
```bash
qaren kv -d "=" prod.env staging.env
```

### `--d1`, `--d2 <DELIMITER>`
**Use Case:** Compare files with different formats (e.g., a `.env` file using `=` and a `.yaml` file using `:`).
**Command Example:**
```bash
qaren kv config.yaml variables.env --d1 ":" --d2 "="
```

### `-Q, --strip-quotes`
**Use Case:** Remove surrounding single or double quotes from both keys and values. Essential when comparing files where some keys are quoted and others are not.
**Command Example:**
```bash
qaren kv -Q file1.env file2.env
```

### `-i, --ignore-case`
**Use Case:** Disregard case differences in both keys and values.
**Command Example:**
```bash
qaren kv -i dev.env PROD.env
```

### `-w, --ignore-all-space`
**Use Case:** Remove all whitespace characters from keys and values before comparison. Useful for comparing messy files with inconsistent internal spacing.
**Command Example:**
```bash
qaren kv -w messy.env clean.env
```

### `-A, --strip-ansi`
**Use Case:** Strip terminal color codes from values. Crucial when comparing environments where values were captured from colored terminal outputs.
**Command Example:**
```bash
qaren kv -A env_output.txt expected.env
```

### `-r, --recursive`
**Use Case:** Recursively compare directories containing Key-Value files. Qaren will match files by relative path and perform semantic KV comparison on each pair.
**Command Example:**
```bash
qaren kv -r ./config_v1 ./config_v2
```

### `-o, --output <FORMAT>`
**Use Case:** Switch between standard terminal output and machine-readable JSON. JSON output is ideal for integration with other tools (e.g., `jq`).
**Command Example:**
```bash
qaren kv prod.env staging.env -o json
```
**Output Example:**
```json
{
  "missing_in_source": [{"key": "DEBUG", "value": "true"}],
  "modified": {
    "PORT": {"old": "8080", "new": "9090"}
  }
}
```

### `-x, --ignore-key <KEY>`
**Use Case:** Exclude a specific key from the comparison. Can be passed multiple times to ignore several keys.
**Command Example:**
```bash
qaren kv a.env b.env -x TIMESTAMP -x RUNTIME_ID
```

### `--ignore-keyword <KEYWORD>`
**Use Case:** Exclude any keys that contain the specified keyword (case-insensitive substring). Great for ignoring dynamic cloud provider keys (e.g., `AWS_`, `GOOGLE_`).
**Command Example:**
```bash
qaren kv dev.env prod.env --ignore-keyword AWS
```

### `-q, --quiet`
**Use Case:** Absolute silence. Returns exit code `0` if identical and `1` if drift is detected.
**Command Example:**
```bash
qaren kv --quiet .env .env.backup || echo "Drift detected!"
```

### `-s, --summary`
**Use Case:** Minimize terminal output to a high-level summary. Ideal for broad audits where you don't need to see every individual value change.
**Command Example:**
```bash
qaren kv prod.env staging.env -s
```
**Output Example:**
```text
── prod.env vs staging.env ──
Summary: 2 only in prod.env, 1 only in staging.env, 4 modified
```

### `-S, --show-secrets`
**Use Case:** Disable the default "Zero-Trust" masking. By default, Qaren hides values; use this flag to see exactly what the differences are.
**Command Example:**
```bash
qaren kv prod.env local.env -S
```
**Output Example:**
```text
── Modified (1 keys) ──
  ~ DB_PASS: s3cret → p@ssw0rd
```

### `-v, --verbose`
**Use Case:** Show all keys, including those that are identical (which are hidden by default to reduce noise). Also shows parsing metadata.
**Command Example:**
```bash
qaren kv a.env b.env -v
```

### `--missing-only`
**Use Case:** Filter the output to ONLY show keys that exist in the source file but are missing in the target. Useful for identifying what needs to be added to a new environment.
**Command Example:**
```bash
qaren kv prod.env .env.example --missing-only
```

### `-g, --generate-patch <FILE>`
**Use Case:** Automatically generate a new configuration file containing the differences. This is the ultimate tool for environment synchronization.
**Command Example:**
```bash
qaren kv prod.env local.env -g missing_keys.env
```

### `--mask-patches` (requires `-g`)
**Use Case:** Redact secret values in the generated patch file. Use this if you need to share a patch file over insecure channels (Slack, Email) for someone else to fill in the secrets.
**Command Example:**
```bash
qaren kv prod.env local.env -g patch.env --mask-patches
```

### `--direction <DIR>` (requires `-g`)
**Use Case:** Control which differences are included in the patch.
- `source-to-target` (default): Keys in FILE1 missing from FILE2.
- `target-to-source`: Keys in FILE2 missing from FILE1.
- `bidirectional`: Generates two patch files to sync both ways.
**Command Example:**
```bash
qaren kv prod.env dev.env -g sync --direction bidirectional
```

---

## Advanced: Combining Flags

### 1. Secure Environment Sync
Identify what staging is missing from production, mask the secrets, and output as JSON for a custom dashboard.
```bash
qaren kv prod.env staging.env --missing-only --mask-patches -o json
```

### 2. Aggressive Cross-Format Comparison
Compare a quoted `.env` file with a colon-delimited YAML file, ignoring all whitespace and case differences.
```bash
qaren kv config.yaml .env -Q -i -w --d1 ":" --d2 "="
```

### 3. CI/CD Drift Guard
Check for configuration drift in a script, ignoring dynamic AWS keys and permission warnings, with absolute silence.
```bash
qaren kv --quiet prod.env staging.env --ignore-keyword AWS -P
```

---
[Return to Index](../README.md) | [See `diff` Command](diff.md) | [Persistent Configuration](config.md)
