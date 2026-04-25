# diff - Literal Comparison

The `diff` command provides a byte-for-byte line comparison using the Myers diff algorithm. It is highly analogous to the standard POSIX `diff`, but includes features tailored for DevOps workflows.

## Usage

```bash
qaren diff [OPTIONS] [FILE1] [FILE2]
```

## Options and Flags

| Short | Long Flag | Description |
|-------|-----------|-------------|
| `-u` | `--unified` | Output unified diff |
| `-b` | `--ignore-space-change` | Ignore changes in the amount of white space |
| `-Z` | `--ignore-trailing-space` | Ignore white space at line end |
| `-B` | `--ignore-blank-lines` | Ignore changes where lines are all blank |
| `-r` | `--recursive` | Recursively compare directories |
| | `--files-only` | Instead of comparing file contents, only compare file existence (requires `-r`) |
| `-q` | `--brief` | Report only when files differ |
| `-s` | `--report-identical-files` | Report when two files are the same |
| `-i` | `--ignore-case` | Ignore case differences in file contents |
| `-w` | `--ignore-all-space` | Ignore all white space |

## Examples

**Basic unified diff:**
```bash
qaren diff -u config.yml config_new.yml
```

**Directory comparison checking file existence only:**
```bash
qaren diff -r --files-only ./staging ./production
```

---
[Return to Index](../README.md) | [Next: kv Command](kv.md)
