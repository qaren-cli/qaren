# Qaren (قارن)

[![Release](https://github.com/qaren-cli/qaren/actions/workflows/release.yml/badge.svg)](https://github.com/qaren-cli/qaren/actions/workflows/release.yml)

A blazingly fast, secure, offline configuration comparison tool.

## Features
- **Semantic Comparison**: Intelligent key-value parsing (supports `.env`, `.yaml`, `.ini`, etc.)
- **POSIX-Compliant Diff**: Standard line-by-line comparison with `-u`, `-w`, `-i` support.
- **Fast & Parallel**: Built in Rust with Rayon for multi-core performance.
- **Zero Trust**: Completely offline, no data leaves your machine.
- **Secret Masking**: Automatically mask sensitive data in diffs and patches.

## Installation
```bash
cargo install qaren
```

## Usage
### Semantic Comparison (KV)
```bash
qaren kv prod.env staging.env
```

### Literal Diff
```bash
qaren diff -u file1.txt file2.txt
```

### Recursive Directory Diff
```bash
qaren diff -r dir1/ dir2/
```

## License
MIT
