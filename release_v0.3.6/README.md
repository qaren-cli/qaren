<p align="center">
  <img src="icon.png" width="160" alt="Qaren Logo">
</p>

<h1 align="center">Qaren (قارن)</h1>

<p align="center">
  <b>The Next Generation of Configuration and Log Comparison.</b><br>
  Built for the modern DevOps era: Semantic, Secure, and Blazingly Fast.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-brightgreen.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/version-0.3.6-orange.svg" alt="Version">
  <img src="https://img.shields.io/badge/PRs-welcome-cyan.svg" alt="PRs Welcome">
</p>

---

## 🚀 Why Qaren?

Standard POSIX `diff` has served us for 50 years, but it was designed for source code, not for the complex, order-agnostic configuration files and massive logs of today.

Qaren (Arabic for **"Compare"**) is a multi-paradigm tool that understands your data.

- **Semantic Key-Value Parsing**: Order doesn't matter. Formatting doesn't matter. Only the data matters.
- **Zero-Trust Security**: Secrets like API keys, passwords, and connection strings are masked by default (`***MASKED***`).
- **Blazingly Fast**: Optimized in Rust to handle GB-scale logs and 100k+ keys up to **200x faster** than traditional diff pipelines.
- **ANSI-Aware**: Automatically cleans terminal color codes from "polluted" files (like `pm2 env` output) for clean comparison.
- **Intelligent Patching**: Generate production-ready `.env` patches to synchronize environments in seconds.

---

## 🛠️ Key Features

### 1. Enhanced Literal Output
Qaren provides much clearer line-by-line diffs than POSIX diff, specifically optimized for log file analysis.
```bash
$ qaren diff old.log new.log -w
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
Comparing JSON logs in KV mode? Use `-D` to suppress duplicate key warnings and `-P` to silence permission alerts. Qaren automatically caps warnings at 5 per file to keep your terminal clean.

---

## 📥 Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/qaren.git
cd qaren

# Build the release binary
cargo build --release

# The binary will be available at ./target/release/qaren
```
*(Coming soon to Cargo and Homebrew)*

---

## 📖 Usage & Examples

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
qaren diff -r ./logs-old ./logs-new

# Strip ANSI colors from log files before diffing
qaren diff logs_polluted.txt logs_clean.txt -A

# Ignore whitespace and blank lines
qaren diff f1.txt f2.txt -w -B
```

---

## ⚙️ Configuration

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

## 📊 Performance Benchmarks
| Scenario | Winner | Margin |
| :--- | :--- | :--- |
| **Large Logs (100MB)** | **Qaren** | **200x+** |
| **Recursive Directory** | **Qaren** | **3x** |
| **Massive Changes (1M Lines)** | **Qaren** | **50x+** |

---

## 🤝 Contributing & Support

We are **Open for Contributions!** Whether it's a bug fix, a new parser, or a performance tweak, your PRs are welcome.

⭐ **Please star the project if you find it useful!**

- **Official Website**: [https://qaren.me/](https://qaren.me/)
- **Bug Reports**: Go to [https://qaren.me/community](https://qaren.me/community) and click **"Open Issue"**.

---

## 📜 License
This project is licensed under the **MIT License**. See the `LICENSE` file for details.

---

<p align="right">(قارن) — صنع بكل فخر للمهندسين</p>

## (Arabic / بالعربية)

**قارن (Qaren)** هو الجيل القادم من أدوات مقارنة الإعدادات (Configuration) والسجلات (Logs). 

- **مقارنة ذكية**: يفهم صيغ KEY=VALUE ولا يكترث بترتيب الأسطر.
- **أمان عالٍ**: يقوم بإخفاء الأسرار (Secrets) تلقائياً.
- **سرعة فائقة**: مبرمج بلغة Rust للتعامل مع ملفات ضخمة بسرعة مذهلة.
- **دعم ANSI**: تنظيف الأكواد البرمجية الخاصة بالألوان من الملفات للمقارنة بوضوح.

الموقع الرسمي: [https://qaren.me/](https://qaren.me/)
