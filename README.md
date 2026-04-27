<p align="center">
  <img src="icon.png" width="160" alt="Qaren Logo">
</p>

<h1 align="center">Qaren (قارن)</h1>

<p align="center">
  <a href="README.md">English</a> | 
  <a href="docs/README.zh.md">中文</a> | 
  <a href="docs/README.ru.md">Русский</a> | 
  <a href="docs/README.ar.md">العربية</a> | 
  <a href="docs/README.fa.md">فارسی</a> | 
  <a href="docs/README.ja.md">日本語</a> | 
  <a href="docs/README.de.md">Deutsch</a> | 
  <a href="docs/README.fr.md">Français</a>
</p>

<p align="center">
  <b>The Next Generation of Configuration and System Backup Comparison.</b><br>
  Built for the modern DevOps era: Semantic, Secure, and Blazingly Fast.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-brightgreen.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/version-1.0.0-orange.svg" alt="Version">
  <img src="https://img.shields.io/badge/PRs-welcome-cyan.svg" alt="PRs Welcome">
  <a href="https://github.com/qaren-cli/qaren/actions/workflows/release.yml">
    <img src="https://github.com/qaren-cli/qaren/actions/workflows/release.yml/badge.svg?branch=master" alt="Release">
  </a>
</p>

---

## Why Qaren? <img src="icons/favicon.png" width="24" height="24"> &nbsp; [<img src="icons/icons8-linkedin-48.png" width="24" height="24">](https://www.linkedin.com/in/alielesawy) &nbsp; [<img src="icons/icons8-github-48.png" width="24" height="24">](https://github.com/alielesawy)

Standard POSIX `diff` has served us for 50 years, but it was designed for source code, not for the complex, order-agnostic configuration files and massive system backups of today.

Qaren (Arabic for **"Compare"**) is a multi-paradigm tool that understands your data.

- **Semantic Key-Value Parsing**: Order doesn't matter. Formatting doesn't matter. Only the data matters.
- **Zero-Trust Security**: Secrets like API keys, passwords, and connection strings are masked by default (`***MASKED***`).
- **Blazingly Fast**: Optimized in Rust to handle GB-scale backups and 100k+ keys up to **200x faster** than traditional diff pipelines.
- **ANSI-Aware**: Automatically cleans terminal color codes from "polluted" files (like `pm2 env` output) for clean comparison.
- **Intelligent Patching**: Generate production-ready `.env` patches to synchronize environments in seconds.

---

## <img src="icons/icons8-doc-48.png" width="24" height="24"> Documentation
For detailed guides, API reference, and advanced configuration, visit our documentation:
> **[https://qaren.me/docs](https://qaren.me/docs)**

---

## <img src="icons/icons8-feature-48.png" width="24" height="24"> Key Features

### 1. Enhanced Literal Output
Qaren provides much clearer line-by-line diffs than POSIX diff, specifically optimized for system backup analysis.
```bash
$ qaren diff backup-old.txt backup-new.txt -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### 2. Semantic KV mode
Understand `.env`, `.yaml`, and `.ini` files regardless of key order.
```bash
$ qaren kv prod.env staging.env
── Modified (1 keys) ──
  ~ PORT: 5000 → 4040
```

### 3. Smart Noise Reduction
Comparing JSON-based backups in KV mode? Use `-D` to suppress duplicate key warnings and `-P` to silence permission alerts. Qaren automatically caps warnings at 5 per file to keep your terminal clean.

---

## <img src="icons/icons8-installation-48.png" width="24" height="24"> Installation

### Quick Install (Automated)

| Platform | Command |
| :--- | :--- |
| **Linux / macOS** | `curl -sSfL https://qaren.me/install | sh` |
| **Windows** | `irm https://qaren.me/install.ps1 | iex` |
| **Homebrew** | `brew tap qaren-cli/qaren && brew install qaren` |

### Alternative Methods
```bash
# Via Cargo
cargo install qaren
```

---

## <img src="icons/icons8-rust-48.png" width="24" height="24"> Usage & Examples

### Semantic Comparison (KV)
```bash
# Basic comparison (auto-detects = or :)
qaren kv file1.env file2.env

# Compare different formats (e.g. .env vs .yaml)
qaren kv file1.env file2.yaml --d2 ':'

# Generate a patch file for missing keys
qaren kv prod.env local.env -g patch.env

# Ignore specific keys or keywords
qaren kv a.env b.env -x HOSTNAME --ignore-keyword AWS

# Output as machine-readable JSON
qaren kv a.env b.env --output json
```

### Literal Comparison (Diff)
```bash
# Unified diff format (POSIX compliant)
qaren diff file1.txt file2.txt -u

# Recursive directory diff
qaren diff -r ./backup-old ./backup-new

# Strip ANSI colors from system snapshots before diffing
qaren diff backup_polluted.txt backup_clean.txt -A

# Ignore whitespace and blank lines
qaren diff f1.txt f2.txt -w -B
```

---

## <img src="icons/icons8-configuration-48.png" width="24" height="24"> Configuration

Qaren remembers your preferences.
```bash
# Toggle pipeline-friendly mode (always exit 0)
qaren config exit toggle

# Toggle color output
qaren config color toggle

# View current settings
qaren config show
```

---

## <img src="icons/icons8-performance-48.png" width="24" height="24"> Performance Benchmarks
| Scenario | Winner | Margin |
| :--- | :--- | :--- |
| **Large Backups (100MB)** | **Qaren** | **200x+** |
| **Recursive Directory** | **Qaren** | **3x** |
| **Massive Changes (1M Lines)** | **Qaren** | **50x+** |

---

## <img src="icons/icons8-contribution-64.png" width="24" height="24"> Contributing & Support

We are **Open for Contributions!** Please read our **[Contributing Guide](CONTRIBUTING.md)** before submitting a Pull Request.

- [ ] **Fork** the repo.
- [ ] **Improve** or **Add** features (avoid deletions).
- [ ] Ensure **Zero Warnings** (`clippy` & `tests`).
- [ ] Update **Docs** and **--help** for new flags.

<img src="icons/icons8-star-.gif" width="20" height="20"> **Please star the project if you find it useful!**

- **Official Website**: [https://qaren.me/](https://qaren.me/)
- **Full Documentation**: [https://qaren.me/docs](https://qaren.me/docs)
- **Bug Reports**: Go to [https://qaren.me/community](https://qaren.me/community) and click **"Open Issue"**.

---

## <img src="icons/icons8-licence-48.png" width="24" height="24"> License
This project is licensed under the **MIT License**. See the `LICENSE` file for details.

---