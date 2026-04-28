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
  <img src="https://img.shields.io/badge/version-1.0.1-orange.svg" alt="Version">
  <img src="https://img.shields.io/badge/PRs-welcome-cyan.svg" alt="PRs Welcome">
  <a href="https://github.com/qaren-cli/qaren/actions/workflows/release.yml">
    <img src="https://github.com/qaren-cli/qaren/actions/workflows/release.yml/badge.svg?branch=master" alt="Release">
  </a>
</p>

---

## Why Qaren? [<img src="icons/favicon.png" width="24" height="24">](https://qaren.me) &nbsp; [<img src="icons/icons8-linkedin-48.png" width="24" height="24">](https://www.linkedin.com/in/alielesawy) &nbsp; [<img src="icons/icons8-github-48.png" width="24" height="24">](https://github.com/alielesawy)

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

### 1. Semantic KV mode
Understand `.env`, `.yaml`, and `.ini` files regardless of key order.
<p align="center">
  <img src="icons/Qd2.gif" width="100%" alt="Semantic KV Mode">
</p>

### 2. Enhanced Literal Output
Qaren provides much clearer line-by-line diffs than POSIX diff, specifically optimized for system backup analysis.

<p align="center">
  <b>Traditional POSIX Diff</b><br>
  <img src="icons/diff.gif" width="100%" alt="Traditional POSIX Diff">
</p>

<p align="center">
  <b>Qaren Enhanced Diff</b><br>
  <img src="icons/qaren-diff.gif" width="100%" alt="Qaren Enhanced Diff">
</p>

### 3. Smart Noise Reduction
Comparing JSON-based backups in KV mode? Qaren automatically suppresses duplicate key and permission warnings by default to keep your terminal clean. If you need assistance debugging, run `qaren config advisor toggle` to enable helpful alerts.

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

Qaren's `kv` mode is designed for real-world DevOps tasks. All the following examples are tested using the data shown in these two environment files:

<p align="center">
  <img src="icons/dev_env.svg" width="45%" alt="Dev Environment">
  <img src="icons/staging_env.svg" width="45%" alt="Staging Environment">
</p>

### 1. Basic Semantic Diff
Compare two files semantically, ignoring line order.
```bash
qaren kv -Q --d2 ":" dev.env staging.env
```
<p align="center">
  <img src="icons/Qd2.gif" width="100%" alt="Basic Semantic Diff">
</p>

### 2. Summary Mode
Get a high-level overview of differences without detailed line changes.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -s
```
<p align="center">
  <img src="icons/Qd2s.gif" width="100%" alt="Summary Mode">
</p>

### 3. JSON Export
Export results in machine-readable format for automation.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -o json
```
<p align="center">
  <img src="icons/Qd2o.gif" width="100%" alt="JSON Export">
</p>

### 4. Show Secrets
Bypass automatic masking to see raw sensitive values.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -S
```
<p align="center">
  <img src="icons/Qd2S.gif" width="100%" alt="Show Secrets">
</p>

### 5. Ignore Specific Keys
Exclude known dynamic or irrelevant keys from comparison.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -x API_KEY
```
<p align="center">
  <img src="icons/Qd2x.gif" width="100%" alt="Ignore Keys">
</p>

### 6. Ignore by Keyword
Exclude all keys containing a specific substring.
```bash
qaren kv --ignore-keyword MAX ...
```
<p align="center">
  <img src="icons/Qd2-ignore-keyword.gif" width="100%" alt="Ignore Keyword">
</p>

### 7. Quiet Mode
Check compatibility in scripts via exit codes only.
```bash
qaren kv -Q --d2 ":" dev.env staging.env -q
```
<p align="center">
  <img src="icons/Qd2q.gif" width="100%" alt="Quiet Mode">
</p>

### 8. Patch Generation
Create a patch file to synchronize missing keys.
```bash
qaren kv ... -g missing.env
```
<p align="center">
  <img src="icons/Qd2g.gif" width="100%" alt="Patch Generation">
</p>

### 9. Secure Patching
Generate patches with sensitive data automatically masked.
```bash
qaren kv ... -g missing.env --mask-patches
```
<p align="center">
  <img src="icons/Qd2g-masked.gif" width="100%" alt="Secure Patches">
</p>

---

## <img src="icons/icons8-rust-48.png" width="24" height="24"> Literal Comparison (Diff)

### 1. Basic Diff
Standard line-by-line comparison with enhanced readability.
```bash
qaren diff file1.txt file2.txt
```

### 2. Unified format
POSIX-compliant unified diff output.
```bash
qaren diff file1.txt file2.txt -u
```

### 3. Recursive Directory Diff
Compare entire directory structures, identifying orphan files and differences in existing ones.
```bash
qaren diff -r old-backup/ new-backup/
```
<p align="center">
  <img src="icons/qaren-diff-R.gif" width="100%" alt="Recursive Directory Diff">
</p>

### 4. Advanced Options
```bash
# Strip ANSI colors from system snapshots before diffing
qaren diff backup_polluted.txt backup_clean.txt -A

# Ignore whitespace and blank lines
qaren diff f1.txt f2.txt -w -B

# Only show which files differ (recursive mode)
qaren diff -r old-backup/ new-backup/ --files-only
```

---

## <img src="icons/icons8-configuration-48.png" width="24" height="24"> Configuration

Qaren remembers your preferences.
<p align="center">
  <img src="icons/config-color.gif" width="100%" alt="Config Color Toggle">
</p>

```bash
# Toggle pipeline-friendly mode (always exit 0)
qaren config exit toggle

# Toggle color output
qaren config color toggle

# Toggle advisor (warnings)
qaren config advisor toggle

# Toggle secret masking
qaren config masking toggle

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