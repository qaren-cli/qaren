# Command: `qaren diff`

The `qaren diff` command performs **literal, line-by-line comparison** using the Myers diff algorithm. Unlike the `kv` command, `diff` does not attempt to understand key-value relationships. It treats files as raw text where order, indentation, and exact line placement matter.

It is the ideal tool for auditing system manifests, raw API payloads (JSON/XML), and recursive directory snapshots where structural integrity must be preserved.

## Usage

```bash
qaren diff [OPTIONS] [FILE1] [FILE2]
```

---

## Real-World Scenarios

### 1. API Payload Audit (JSON/XML)
When comparing JSON or XML payloads where you need to ensure that the formatting, indentation, and property order haven't changed (even if the data is semantically similar), use `diff`.
```bash
# Compare two JSON snapshots literally
qaren diff response_v1.json response_v2.json -u
```

### 2. System Manifests & Scripts
For files like `hosts`, `resolv.conf`, or shell scripts where the exact line order is functional logic, `qaren diff` provides the necessary literal precision.
```bash
# Check for changes in system host files
qaren diff /etc/hosts /backup/etc/hosts -s
```

### 3. High-Speed Directory Snapshots
Use recursive mode to audit entire directory structures (e.g., `/etc` or `/var/www`) to detect any modified, added, or deleted files literally.
```bash
# Audit a web directory against a known clean backup
qaren diff -r /var/www/html /backup/www/html
```

---

## Detailed Options Reference

### `-u, --unified`
**Use Case:** Generate standard patches or review changes in the standard format used by DevOps teams and Git.
**Command Example:**
```bash
qaren diff -u payload_old.json payload_new.json
```

### `-b, --ignore-space-change`
**Use Case:** Ignore changes in the *amount* of whitespace, but still treat whitespace as a separator. Useful for comparing code where indentation might have shifted from 2 to 4 spaces.

### `-w, --ignore-all-space`
**Use Case:** Completely ignore all whitespace. Useful for comparing minified JSON or CSS files where characters are identical but formatting differs.

### `-B, --ignore-blank-lines`
**Use Case:** Ignore changes that only add or remove empty lines between blocks of code or data.

### `-i, --ignore-case`
**Use Case:** Compare text while disregarding uppercase/lowercase differences.

### `-A, --strip-ansi`
**Use Case:** Strip terminal color codes from files (like captured `stdout` from a build tool) before comparing them against a plain-text reference.

### `-r, --recursive`
**Use Case:** Perform a literal comparison across entire directory trees.
**Output Example:**
```text
▶ Differences in: config/settings.conf
-[L12] timeout=30
+[L12] timeout=60

▶ Orphan: old_backup.tar.gz (exists only in ./dir1)
```

### `--files-only` (requires `-r`)
**Use Case:** Compare directory structures only. Find missing or extra files without reading their contents.

### `-q, --brief`
**Use Case:** Only report if files differ (Exit 1) or are identical (Exit 0), without showing content changes.

---

## Advanced: Combining Flags for Payloads

### 1. Minified vs Prettified Payload
Compare a minified JSON file against a formatted one by ignoring all whitespace.
```bash
qaren diff -w production.min.json local.pretty.json -s
```

### 2. Case-Insensitive Manifest Audit
Audit system files where casing might vary but content should remain identical.
```bash
qaren diff -i /etc/group /backup/etc/group
```

---
[Return to Index](../README.md) | [See `kv` Command (for Key-Value Data)](kv.md) | [Persistent Configuration](config.md)
