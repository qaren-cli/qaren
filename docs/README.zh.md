<p align="center">
  <img src="../icon.png" width="160" alt="Qaren Logo">
</p>

<h1 align="center">Qaren (قارن)</h1>

<p align="center">
  <a href="../README.md">English</a> | 
  <a href="README.zh.md">中文</a> | 
  <a href="README.ru.md">Русский</a> | 
  <a href="README.ar.md">العربية</a> | 
  <a href="README.fa.md">فارسی</a> | 
  <a href="README.ja.md">日本語</a>
</p>

<p align="center">
  <b>下一代配置与日志对比工具。</b><br>
  专为现代 DevOps 时代打造：语义化、安全且极速。
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-brightgreen.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/version-0.3.6-orange.svg" alt="Version">
  <img src="https://img.shields.io/badge/PRs-welcome-cyan.svg" alt="PRs Welcome">
  <a href="https://github.com/qaren-cli/qaren/actions/workflows/release.yml">
    <img src="https://github.com/qaren-cli/qaren/actions/workflows/release.yml/badge.svg" alt="Release">
  </a>
</p>

---

## 🚀 为什么选择 Qaren？

标准的 POSIX `diff` 已经服务了我们 50 年，但它是为源代码设计的，而不是为了当今复杂、与顺序无关的配置文件和海量日志。

Qaren（阿拉伯语意为 **“比较”**）是一个理解您数据的多范式工具。

- **语义化键值解析**：顺序不重要。格式不重要。只有数据重要。
- **零信任安全**：API 密钥、密码和连接字符串等敏感信息默认被遮盖 (`***MASKED***`)。
- **极速性能**：使用 Rust 优化，处理 GB 级日志和 10 万级键值对的速度比传统 diff 流水线快达 **200 倍**。
- **ANSI 感知**：自动清除“污染”文件（如 `pm2 env` 输出）中的终端颜色代码，进行纯净对比。
- **智能补丁**：在几秒钟内生成生产级 `.env` 补丁，同步不同环境。

---

## 📚 文档
有关详细指南、API 参考和高级配置，请访问我们的文档：
👉 **[https://qaren.me/docs](https://qaren.me/docs)**

---

## 🛠️ 核心功能

### 1. 增强的字面量输出
Qaren 提供比 POSIX diff 更清晰的逐行对比，特别针对日志文件分析进行了优化。
```bash
$ qaren diff old.log new.log -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### 2. 语义化 KV 模式
理解 `.env`, `.yaml`, 和 `.ini` 文件，无论键的顺序如何。
```bash
$ qaren kv prod.env staging.env
── Modified (1 keys) ──
  ~ PORT: 5000 → 4040
```

### 3. 智能降噪
在 KV 模式下对比 JSON 日志？使用 `-D` 抑制重复键警告，使用 `-P` 沉默权限警报。Qaren 会自动将每个文件的警告限制在 5 个以内，保持终端整洁。

---

## 📥 安装

```bash
# 克隆仓库
git clone https://github.com/yourusername/qaren.git
cd qaren

# 构建发布版本
cargo build --release

# 二进制文件位于 ./target/release/qaren
```

---

## 📖 使用方法与示例

### 语义化对比 (KV)
```bash
# 基础对比（自动检测 = 或 :）
qaren kv file1.env file2.env

# 对比不同格式（例如 .env 对比 .yaml）
qaren kv file1.env file2.yaml --d2 ':'

# 为缺失的键生成补丁文件
qaren kv prod.env local.env -g patch.env

# 忽略特定的键或关键词
qaren kv a.env b.env -x HOSTNAME --ignore-keyword AWS

# 输出为机器可读的 JSON
qaren kv a.env b.env --output json
```

### 字面量对比 (Diff)
```bash
# 统一 diff 格式（符合 POSIX 标准）
qaren diff file1.txt file2.txt -u

# 递归目录对比
qaren diff -r ./logs-old ./logs-new

# 对比前清除日志文件中的 ANSI 颜色
qaren diff logs_polluted.txt logs_clean.txt -A

# 忽略空格和空行
qaren diff f1.txt f2.txt -w -B
```

---

## ⚙️ 配置

Qaren 会记住您的偏好。
```bash
# 切换流水线友好模式（始终以 0 退出）
qaren config exit toggle

# 切换彩色输出
qaren config color toggle

# 查看当前设置
qaren config show
```

---

## 📊 性能基准
| 场景 | 获胜者 | 领先幅度 |
| :--- | :--- | :--- |
| **大型日志 (100MB)** | **Qaren** | **200x+** |
| **递归目录** | **Qaren** | **3x** |
| **海量变更 (100万行)** | **Qaren** | **50x+** |

---

## 🤝 贡献与支持

我们**欢迎贡献！** 无论是错误修复、新的解析器还是性能优化，都欢迎提交 PR。

⭐ **如果觉得有用，请为本项目点亮星标！**

- **官方网站**: [https://qaren.me/](https://qaren.me/)
- **完整文档**: [https://qaren.me/docs](https://qaren.me/docs)
- **错误报告**: 前往 [https://qaren.me/community](https://qaren.me/community) 并点击 **"Open Issue"**。

---

## 📜 许可证
本项目采用 **MIT 许可证**。详情请参阅 `LICENSE` 文件。

---

<p align="right">(قارن) — 专为工程师自豪打造</p>
