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
  <a href="README.ja.md">日本語</a> | 
  <a href="README.de.md">Deutsch</a> | 
  <a href="README.fr.md">Français</a>
</p>

<p align="center">
  <b>下一代配置与系统备份对比工具。</b><br>
  专为现代 DevOps 时代打造：语义化、安全且极速。
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

## 为什么选择 Qaren？ <img src="../icons/favicon.png" width="24" height="24"> &nbsp; [<img src="../icons/icons8-linkedin-48.png" width="24" height="24">](https://www.linkedin.com/in/alielesawy) &nbsp; [<img src="../icons/icons8-github-48.png" width="24" height="24">](https://github.com/alielesawy)

标准的 POSIX `diff` 已经服务了我们 50 年，但它是为源代码设计的，而不是为了当今复杂、与顺序无关的配置文件和海量 system backups。

Qaren（阿拉伯语意为 **“比较”**）是一个理解您数据的多范式工具。

- **语义化键值解析**：顺序不重要。格式不重要。只有数据重要。
- **零信任安全**：API 密钥、密码和连接字符串等敏感信息默认被遮盖 (`***MASKED***`)。
- **极速性能**：使用 Rust 优化，处理 GB 级系统备份和 10 万级键值对的速度比传统 diff 流水线快达 **200 倍**。
- **ANSI 感知**：自动清除“污染”文件（如 `pm2 env` 输出）中的终端颜色代码，进行纯净对比。
- **智能补丁**：在几秒钟内生成生产级 `.env` 补丁，同步不同环境。

---

## <img src="../icons/icons8-doc-48.png" width="24" height="24"> 文档
有关详细指南、API 参考和高级配置，请访问我们的文档：
> **[https://qaren.me/docs](https://qaren.me/docs)**

---

## <img src="../icons/icons8-feature-48.png" width="24" height="24"> 核心功能

### 1. 语义化 KV 模式
理解 `.env`, `.yaml`, 和 `.ini` 文件，无论键的顺序如何。
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="语义化 KV 模式">
</p>

### 2. 增强的字面量输出
Qaren 提供比 POSIX diff 更清晰的逐行对比，特别针对系统备份文件分析进行了优化。
```bash
$ qaren diff backup-old backup-new -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### 3. 智能降噪
在 KV 模式下对比 JSON 备份？Qaren 默认会自动抑制重复键和权限警告，保持终端整洁。如果您需要辅助调试，运行 `qaren config advisor toggle` 即可开启相关警报。

---

## <img src="../icons/icons8-installation-48.png" width="24" height="24"> 安装

### 快速安装（自动）

| 平台 | 命令 |
| :--- | :--- |
| **Linux / macOS** | `curl -sSfL https://qaren.me/install | sh` |
| **Windows** | `irm https://qaren.me/install.ps1 | iex` |
| **Homebrew** | `brew tap qaren-cli/qaren && brew install qaren` |

### 替代方法
```bash
# 通过 Cargo
cargo install qaren
```

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> 使用方法与示例

Qaren 的 `kv` 模式专为真实世界的 DevOps 任务设计。以下是对比环境文件的常用模式。

### 1. 基础语义对比
语义化对比两个文件，忽略行顺序。
```bash
qaren kv -Q --d2 ":" dev.env staging.env
```
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="Basic Semantic Diff">
</p>

### 2. 概要模式
获取差异的高级概览，不显示详细的行更改。
```bash
qaren kv -Q --d2 ":" dev.env staging.env -s
```
<p align="center">
  <img src="../icons/Qd2s.gif" width="100%" alt="Summary Mode">
</p>

### 3. 导出 JSON
以机器可读格式导出结果，以便实现自动化。
```bash
qaren kv -Q --d2 ":" dev.env staging.env -o json
```
<p align="center">
  <img src="../icons/Qd2o.gif" width="100%" alt="JSON Export">
</p>

### 4. 显示敏感信息
绕过自动遮盖以查看原始敏感值。
```bash
qaren kv -Q --d2 ":" dev.env staging.env -S
```
<p align="center">
  <img src="../icons/Qd2S.gif" width="100%" alt="Show Secrets">
</p>

### 5. 忽略特定键
从对比中排除已知的动态或不相关键。
```bash
qaren kv -Q --d2 ":" dev.env staging.env -x API_KEY
```
<p align="center">
  <img src="../icons/Qd2x.gif" width="100%" alt="Ignore Keys">
</p>

### 6. 按关键词忽略
排除所有包含特定子字符串的键。
```bash
qaren kv --ignore-keyword MAX ...
```
<p align="center">
  <img src="../icons/Qd2-ignore-keyword.gif" width="100%" alt="Ignore Keyword">
</p>

### 7. 静默模式
仅通过退出代码在脚本中检查兼容性。
```bash
qaren kv -Q --d2 ":" dev.env staging.env -q
```
<p align="center">
  <img src="../icons/Qd2q.gif" width="100%" alt="Quiet Mode">
</p>

### 8. 补丁生成
创建补丁文件以同步缺失的键。
```bash
qaren kv ... -g missing.env
```
<p align="center">
  <img src="../icons/Qd2g.gif" width="100%" alt="Patch Generation">
</p>

### 9. 安全补丁
生成敏感数据自动遮盖的补丁。
```bash
qaren kv ... -g missing.env --mask-patches
```
<p align="center">
  <img src="../icons/Qd2g-masked.gif" width="100%" alt="Secure Patches">
</p>

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> 字面量对比 (Diff)
```bash
# 统一 diff 格式（符合 POSIX 标准）
qaren diff file1.txt file2.txt -u

# 递归目录对比
qaren diff -r ./backup-old ./backup-new

# 对比前清除备份文件中的 ANSI 颜色
qaren diff backup_polluted.txt backup_clean.txt -A

# 忽略空格和空行
qaren diff f1.txt f2.txt -w -B
```

---

## <img src="../icons/icons8-configuration-48.png" width="24" height="24"> 配置

Qaren 会记住您的偏好。
<p align="center">
  <img src="../icons/config-color.gif" width="100%" alt="配置颜色切换">
</p>

```bash
# 切换流水线友好模式（始终以 0 退出）
qaren config exit toggle

# 切换彩色输出
qaren config color toggle

# 切换 Advisor（警告）
qaren config advisor toggle

# 切换敏感信息遮盖
qaren config masking toggle

# 查看当前设置
qaren config show
```

---

## <img src="../icons/icons8-performance-48.png" width="24" height="24"> 性能基准
| 场景 | 获胜者 | 领先幅度 |
| :--- | :--- | :--- |
| **大型备份 (100MB)** | **Qaren** | **200x+** |
| **递归目录** | **Qaren** | **3x** |
| **海量变更 (100万行)** | **Qaren** | **50x+** |

---

## <img src="../icons/icons8-contribution-64.png" width="24" height="24"> 贡献与支持

我们 **欢迎贡献！** 在提交 Pull Request 之前，请阅读我们的 **[贡献指南](CONTRIBUTING.md)**。

- [ ] **Fork** 仓库。
- [ ] **改进**或**添加**功能（避免删除已有功能）。
- [ ] 确保 **零警告** (`clippy` & `tests`)。
- [ ] 为新标志更新 **文档** 和 **--help**。

<img src="../icons/icons8-star-.gif" width="20" height="20"> **如果觉得有用，请为本项目点亮星标！**

- **官方网站**: [https://qaren.me/](https://qaren.me/)
- **完整文档**: [https://qaren.me/docs](https://qaren.me/docs)
- **错误报告**: 前往 [https://qaren.me/community](https://qaren.me/community) 并点击 **"Open Issue"**。

---

## <img src="../icons/icons8-licence-48.png" width="24" height="24"> 许可证
本项目采用 **MIT 许可证**。详情请参阅 `LICENSE` 文件。

---

<p align="right">(قارن) — 专为工程师自豪打造</p>
