# Command: `qaren diff`

The `qaren diff` command performs literal, line-by-line comparison using the Myers diff algorithm. It acts as a hyper-optimized drop-in replacement for POSIX `diff`, making it the perfect tool for detecting formatting changes, analyzing unstructured logs, or comparing non-KV configuration files.

## Usage

```bash
qaren diff [OPTIONS] [FILE1] [FILE2]
```

## Options and Flags

### Diff Formatting Options

*   **`-u, --unified`**
    Output changes in the standard unified diff format. This is highly recommended when piping output to `patch` or sharing diffs in code review.
    ```bash
    qaren diff -u file1.txt file2.txt
    ```

### Whitespace & Formatting Ignorance

*   **`-b, --ignore-space-change`**
    Ignore changes in the amount of white space. Multiple spaces are treated as a single space.
*   **`-Z, --ignore-trailing-space`**
    Ignore white space at the end of a line. Useful for files edited on different operating systems.
*   **`-B, --ignore-blank-lines`**
    Ignore changes where lines are all blank.
*   **`-w, --ignore-all-space`**
    Ignore all white space completely.
*   **`-i, --ignore-case`**
    Ignore case differences in file contents (`A` equals `a`).

### Output Control

*   **`-q, --brief`**
    Report only whether the files differ, outputting a single sentence (e.g., `Files file1.txt and file2.txt differ`). Returns standard exit codes.
*   **`--quiet`**
    Absolute silence. Outputs nothing to `stdout` or `stderr`. Only returns the exit code. Essential for shell script conditionals.
*   **`-s, --report-identical-files`**
    Explicitly report when two files are the exact same (standard `diff` returns nothing on success).
*   **`-A, --strip-ansi`**
    **Advanced Feature:** Automatically strips terminal ANSI escape color codes from input files before comparing them. This is crucial for comparing raw `stdout` captures or CI logs that are polluted with color formatting.

### Directory Comparison

*   **`-r, --recursive`**
    Recursively compare directories. Unlike standard `diff`, Qaren provides an easily readable summary of changed files and clearly identifies "Orphans" (files that exist in only one directory).
    ```bash
    qaren diff -r ./v1_source ./v2_source
    ```
*   **`--files-only`**
    Requires `-r`. Instead of comparing file contents, Qaren will only compare file *existence*. It will immediately list files missing from either directory without wasting time hashing or comparing contents.
    ```bash
    qaren diff -r --files-only ./dir1 ./dir2
    ```

---

## Detailed Examples

**Ignoring formatting noise:**
```bash
qaren diff -b -B -Z old_config.txt new_config.txt
```

**Benchmarking massive directories:**
Qaren's `diff -r` is heavily optimized. When comparing 100MB+ payload directories, Qaren processes files significantly faster than traditional `diff -r`, providing real-time summaries of additions, deletions, and modifications.

---
[Return to Index](../README.md) | [See `kv` Command](kv.md) | [Persistent Configuration](config.md)
