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
  <b>次世代の設定・システムバックアップ比較ツール</b><br>
  現代の DevOps 時代のために構築：セマンティック、セキュア、そして驚異的な速さ
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

## なぜ Qaren なのか？ [<img src="../icons/favicon.png" width="24" height="24">](https://qaren.me) &nbsp; [<img src="../icons/icons8-linkedin-48.png" width="24" height="24">](https://www.linkedin.com/in/alielesawy) &nbsp; [<img src="../icons/icons8-github-48.png" width="24" height="24">](https://github.com/alielesawy)

標準的な POSIX `diff` は 50 年もの間使われてきましたが、それはソースコードのために設計されたものであり、現代の複雑で順序を問わない設定ファイルや膨大なシステムバックアップには適していません。

Qaren（アラビア語で **「比較」**）は、データを理解するマルチパラダイムツールです。

- **セマンティック・キーバリュー解析**: 順序やフォーマットは関係ありません。データそのものに焦点を当てます。
- **ゼロトラスト・セキュリティ**: API キー、パスワード、接続文字列などの機密情報はデフォルトでマスクされます (`***MASKED***`)。
- **圧倒的なパフォーマンス**: Rust で最適化されており、GB 単位のシステムバックアップや 10 万個以上のキーを、従来の diff パイプラインより最大 **200 倍速く** 処理します。
- **ANSI 対応**: `pm2 env` の出力などの「汚染された」ファイルからターミナルカラーコードを自動的に除去し、クリーンな比較を実現します。
- **インテリジェントなパッチ生成**: 環境を数秒で同期させるための本番環境対応の `.env` パッチを生成します。

---

## <img src="../icons/icons8-doc-48.png" width="24" height="24"> ドキュメント
詳細なガイド、API リファレンス、および高度な設定については、ドキュメントサイトをご覧ください：
> **[https://qaren.me/docs](https://qaren.me/docs)**

---

## <img src="../icons/icons8-feature-48.png" width="24" height="24"> 主な機能

### 1. セマンティック KV モード
キーの順序に関係なく、`.env`、`.yaml`、`.ini` ファイルを理解します。
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="セマンティック KV モード">
</p>

### 2. 強化されたリテラル出力
Qaren は POSIX diff よりもはるかに明確な行単位の差分を提供し、特にシステムバックアップファイルの分析に最適化されています。

<p align="center">
  <b>従来の POSIX Diff</b><br>
  <img src="../icons/diff.gif" width="100%" alt="Traditional POSIX Diff">
</p>

<p align="center">
  <b>Qaren 強化 Diff</b><br>
  <img src="../icons/qaren-diff.gif" width="100%" alt="Qaren Enhanced Diff">
</p>

### 3. スマートなノイズ低減
KV モードで JSON バックアップを比較していますか？ Qaren はデフォルトで重複キーや権限の警告を自動的に抑制し、ターミナルをクリーンに保ちます。デバッグの際にこれらの警告が必要な場合は、`qaren config advisor toggle` を実行して有効にしてください。

---

## <img src="../icons/icons8-installation-48.png" width="24" height="24"> インストール

### クイックインストール (自動)

| プラットフォーム | コマンド |
| :--- | :--- |
| **Linux / macOS** | `curl -sSfL https://qaren.me/install | sh` |
| **Windows** | `irm https://qaren.me/install.ps1 | iex` |
| **Homebrew** | `brew tap qaren-cli/qaren && brew install qaren` |

### その他の方法
```bash
# Cargo を使用
cargo install qaren
```

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> 使い方と例

Qaren の `kv` モードは、実際の DevOps タスク向けに設計されています。以下のすべての例は、これら2つの環境ファイルに示されているデータを使用してテストされています：

<p align="center">
  <img src="../icons/dev_env.svg" width="45%" alt="開発環境">
  <img src="../icons/staging_env.svg" width="45%" alt="ステージング環境">
</p>

### 1. 基本的なセマンティック比較
行の順序を無視して、2つのファイルをセマンティックに比較します。
```bash
qaren kv -Q --d2 ":" dev.env staging.env
```
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="基本的なセマンティック比較">
</p>

### 2. サマリーモード
詳細な行の変更を表示せずに、差異の全体像を把握します。
```bash
qaren kv -Q --d2 ":" dev.env staging.env -s
```
<p align="center">
  <img src="../icons/Qd2s.gif" width="100%" alt="サマリーモード">
</p>

### 3. JSON エクスポート
自動化のために、マシンが読み取り可能な形式で結果をエクスポートします。
```bash
qaren kv -Q --d2 ":" dev.env staging.env -o json
```
<p align="center">
  <img src="../icons/Qd2o.gif" width="100%" alt="JSON エクスポート">
</p>

### 4. 機密情報の表示
自動マスキングをバイパスして、生の機密値を表示します。
```bash
qaren kv -Q --d2 ":" dev.env staging.env -S
```
<p align="center">
  <img src="../icons/Qd2S.gif" width="100%" alt="機密情報の表示">
</p>

### 5. 特定のキーを無視
既知の動的なキーや無関係なキーを比較から除外します。
```bash
qaren kv -Q --d2 ":" dev.env staging.env -x API_KEY
```
<p align="center">
  <img src="../icons/Qd2x.gif" width="100%" alt="キーの無視">
</p>

### 6. キーワードで無視
特定の文字列を含むすべてのキーを除外します。
```bash
qaren kv --ignore-keyword MAX ...
```
<p align="center">
  <img src="../icons/Qd2-ignore-keyword.gif" width="100%" alt="キーワードで無視">
</p>

### 7. クワイエットモード
終了コードのみを使用して、スクリプト内での互換性を確認します。
```bash
qaren kv -Q --d2 ":" dev.env staging.env -q
```
<p align="center">
  <img src="../icons/Qd2q.gif" width="100%" alt="クワイエットモード">
</p>

### 8. パッチ生成
欠落しているキーを同期するためのパッチファイルを作成します。
```bash
qaren kv ... -g missing.env
```
<p align="center">
  <img src="../icons/Qd2g.gif" width="100%" alt="パッチ生成">
</p>

### 9. セキュアパッチ
機密データを自動的にマスクした状態でパッチを生成します。
```bash
qaren kv ... -g missing.env --mask-patches
```
<p align="center">
  <img src="../icons/Qd2g-masked.gif" width="100%" alt="セキュアパッチ">
</p>

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> リテラル比較 (Diff)

### 1. 基本的な比較
標準的な行単位の比較。
```bash
qaren diff file1.txt file2.txt
```

### 2. 統合フォーマット
POSIX 準拠の統合 diff 出力。
```bash
qaren diff file1.txt file2.txt -u
```

### 3. ディレクトリの再帰的比較
ディレクトリ構造全体を比較し、孤立したファイルや既存ファイルの差異を特定します。
```bash
qaren diff -r old-backup/ new-backup/
```
<p align="center">
  <img src="../icons/qaren-diff-R.gif" width="100%" alt="ディレクトリの再帰的比較">
</p>

### 4. 高度なオプション
```bash
# 比較前にバックアップファイルから ANSI カラーを除去
qaren diff backup_polluted.txt backup_clean.txt -A

# 空白と空行を無視
qaren diff f1.txt f2.txt -w -B

# 差異のあるファイル名のみを表示（再帰モード）
qaren diff -r old-backup/ new-backup/ --files-only
```

---

## <img src="../icons/icons8-configuration-48.png" width="24" height="24"> 設定

Qaren は設定を記憶します。
<p align="center">
  <img src="../icons/config-color.gif" width="100%" alt="カラー切り替え">
</p>

```bash
# パイプラインフレンドリーモード（常に 0 で終了）を切り替え
qaren config exit toggle

# カラー出力を切り替え
qaren config color toggle

# アドバイザー（警告）を切り替え
qaren config advisor toggle

# マスキングを切り替え
qaren config masking toggle

# 現在の設定を表示
qaren config show
```

---

## <img src="../icons/icons8-performance-48.png" width="24" height="24"> パフォーマンスベンチマーク
| シナリオ | 勝者 | 差 |
| :--- | :--- | :--- |
| **巨大バックアップ (100MB)** | **Qaren** | **200倍以上** |
| **再帰ディレクトリ** | **Qaren** | **3倍** |
| **大量の変更 (100万行)** | **Qaren** | **50倍以上** |

---

## <img src="../icons/icons8-contribution-64.png" width="24" height="24"> 貢献とサポート

私たちは **コントリビューションを歓迎します！** プルリクエストを送信する前に **[貢献ガイド](CONTRIBUTING.md)** をお読みください。

- [ ] リポジトリを **フォーク** する
- [ ] 機能を **改善** または **追加** する（削除は避けてください）
- [ ] **警告ゼロ** を確認する (`clippy` & `tests`)
- [ ] 新しいフラグに合わせて **ドキュメント** と **--help** を更新する

<img src="../icons/icons8-star-.gif" width="20" height="20"> **便利だと思ったら、ぜひスターをお願いします！**

- **公式サイト**: [https://qaren.me/](https://qaren.me/)
- **詳細ドキュメント**: [https://qaren.me/docs](https://qaren.me/docs)
- **バグ報告**: [https://qaren.me/community](https://qaren.me/community) にアクセスし、 **"Open Issue"** をクリックしてください。

---

## <img src="../icons/icons8-licence-48.png" width="24" height="24"> ライセンス
このプロジェクトは **MIT ライセンス** の下で公開されています。詳細は `LICENSE` ファイルをご覧ください。

---

<p align="right">(قارن) — エンジニアのために誇りを持って制作</p>
