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
  <b>次世代の設定・ログ比較ツール</b><br>
  現代の DevOps 時代のために構築：セマンティック、セキュア、そして驚異的な速さ
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-brightgreen.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/version-0.3.6-orange.svg" alt="Version">
  <img src="https://img.shields.io/badge/PRs-welcome-cyan.svg" alt="PRs Welcome">
</p>

---

## 🚀 なぜ Qaren なのか？

標準的な POSIX `diff` は 50 年もの間使われてきましたが、それはソースコードのために設計されたものであり、現代の複雑で順序を問わない設定ファイルや膨大なログには適していません。

Qaren（アラビア語で **「比較」**）は、データを理解するマルチパラダイムツールです。

- **セマンティック・キーバリュー解析**: 順序やフォーマットは関係ありません。データそのものに焦点を当てます。
- **ゼロトラスト・セキュリティ**: API キー、パスワード、接続文字列などの機密情報はデフォルトでマスクされます (`***MASKED***`)。
- **圧倒的なパフォーマンス**: Rust で最適化されており、GB 単位のログや 10 万個以上のキーを、従来の diff パイプラインより最大 **200 倍速く** 処理します。
- **ANSI 対応**: `pm2 env` の出力などの「汚染された」ファイルからターミナルカラーコードを自動的に除去し、クリーンな比較を実現します。
- **インテリジェントなパッチ生成**: 環境を数秒で同期させるための本番環境対応の `.env` パッチを生成します。

---

## 📚 ドキュメント
詳細なガイド、API リファレンス、および高度な設定については、ドキュメントサイトをご覧ください：
👉 **[https://qaren.me/docs](https://qaren.me/docs)**

---

## 🛠️ 主な機能

### 1. 強化されたリテラル出力
Qaren は POSIX diff よりもはるかに明確な行単位の差分を提供し、特にログファイルの分析に最適化されています。
```bash
$ qaren diff old.log new.log -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### 2. セマンティック KV モード
キーの順序に関係なく、`.env`、`.yaml`、`.ini` ファイルを理解します。
```bash
$ qaren kv prod.env staging.env
── Modified (1 keys) ──
  ~ PORT: 5000 → 4040
```

### 3. スマートなノイズ低減
KV モードで JSON ログを比較していますか？ `-D` で重複キーの警告を抑制し、 `-P` で権限の警告を消音できます。Qaren は自動的にファイルごとの警告を 5 つまでに制限し、ターミナルをクリーンに保ちます。

---

## 📥 インストール

```bash
# リポジトリをクローン
git clone https://github.com/yourusername/qaren.git
cd qaren

# リリースバイナリをビルド
cargo build --release

# バイナリは ./target/release/qaren に生成されます
```

---

## 📖 使い方と例

### セマンティック比較 (KV)
```bash
# 基本的な比較 (= または : を自動検出)
qaren kv file1.env file2.env

# 異なるフォーマットの比較 (例: .env vs .yaml)
qaren kv file1.env file2.yaml --d2 ':'

# 欠落しているキーのパッチファイルを生成
qaren kv prod.env local.env -g patch.env

# 特定のキーまたはキーワードを無視
qaren kv a.env b.env -x HOSTNAME --ignore-keyword AWS

# 機械読み取り可能な JSON として出力
qaren kv a.env b.env --output json
```

### リテラル比較 (Diff)
```bash
# 統合 diff 形式 (POSIX 準拠)
qaren diff file1.txt file2.txt -u

# ディレクトリの再帰的比較
qaren diff -r ./logs-old ./logs-new

# 比較前にログファイルから ANSI カラーを除去
qaren diff logs_polluted.txt logs_clean.txt -A

# 空白と空行を無視
qaren diff f1.txt f2.txt -w -B
```

---

## ⚙️ 設定

Qaren は設定を記憶します。
```bash
# パイプラインフレンドリーモード（常に 0 で終了）を切り替え
qaren config exit toggle

# カラー出力を切り替え
qaren config color toggle

# 現在の設定を表示
qaren config show
```

---

## 📊 パフォーマンスベンチマーク
| シナリオ | 勝者 | 差 |
| :--- | :--- | :--- |
| **巨大ログ (100MB)** | **Qaren** | **200倍以上** |
| **再帰ディレクトリ** | **Qaren** | **3倍** |
| **大量の変更 (100万行)** | **Qaren** | **50倍以上** |

---

## 🤝 貢献とサポート

私たちは **コントリビューションを歓迎します！** バグ修正、新しいパーサー、パフォーマンスの改善など、PR をお待ちしています。

⭐ **便利だと思ったら、ぜひスターをお願いします！**

- **公式サイト**: [https://qaren.me/](https://qaren.me/)
- **詳細ドキュメント**: [https://qaren.me/docs](https://qaren.me/docs)
- **バグ報告**: [https://qaren.me/community](https://qaren.me/community) にアクセスし、 **"Open Issue"** をクリックしてください。

---

## 📜 ライセンス
このプロジェクトは **MIT ライセンス** の下で公開されています。詳細は `LICENSE` ファイルをご覧ください。

---

<p align="right">(قارن) — エンジニアのために誇りを持って制作</p>
