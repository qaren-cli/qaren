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
  <b>Die nächste Generation des Konfigurations- und System-Backup-Vergleichs.</b><br>
  Entwickelt für die moderne DevOps-Ära: Semantisch, Sicher und Blitzschnell.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-brightgreen.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/version-1.0.0-orange.svg" alt="Version">
  <img src="https://img.shields.io/badge/PRs-welcome-cyan.svg" alt="PRs Welcome">
  <a href="https://github.com/qaren-cli/qaren/actions/workflows/release.yml">
    <img src="https://github.com/qaren-cli/qaren/actions/workflows/release.yml/badge.svg" alt="Release">
  </a>
</p>

---

## Warum Qaren? <img src="../icons/favicon.png" width="24" height="24"> &nbsp; [<img src="../icons/icons8-linkedin-48.png" width="24" height="24">](https://www.linkedin.com/in/alielesawy) &nbsp; [<img src="../icons/icons8-github-48.png" width="24" height="24">](https://github.com/alielesawy)

Das standardmäßige POSIX `diff` dient uns seit 50 Jahren, wurde jedoch für Quellcode entwickelt – nicht für die komplexen, reihenfolgeunabhängigen Konfigurationsdateien und massiven System-Backups von heute.

Qaren (arabisch für **„Vergleichen“**) ist ein Multi-Paradigma-Tool, das Ihre Daten versteht.

- **Semantisches Key-Value-Parsing**: Die Reihenfolge spielt keine Rolle. Die Formatierung spielt keine Rolle. Nur die Daten zählen.
- **Zero-Trust-Sicherheit**: Geheimnisse wie API-Schlüssel, Passwörter und Verbindungszeichenfolgen werden standardmäßig maskiert (`***MASKED***`).
- **Blitzschnell**: In Rust optimiert, um System-Backups im GB-Bereich und über 100k Schlüssel bis zu **200-mal schneller** zu verarbeiten als herkömmliche Diff-Pipelines.
- **ANSI-Unterstützung**: Bereinigt Terminal-Farbcodes automatisch aus „verunreinigten“ Dateien (wie `pm2 env`-Ausgaben) für einen sauberen Vergleich.
- **Intelligentes Patching**: Erstellen Sie produktionsbereite `.env`-Patches, um Umgebungen in Sekundenschnelle zu synchronisieren.

---

## <img src="../icons/icons8-doc-48.png" width="24" height="24"> Dokumentation
Detaillierte Anleitungen, API-Referenzen und erweiterte Konfigurationen finden Sie in unserer Dokumentation:
> **[https://qaren.me/docs](https://qaren.me/docs)**

---

## <img src="../icons/icons8-feature-48.png" width="24" height="24"> Hauptmerkmale

### 1. Verbesserte Textausgabe
Qaren bietet viel klarere zeilenweise Diffs als POSIX-Diff, speziell optimiert für die Analyse von System-Backups.
```bash
$ qaren diff backup-old backup-new -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### 2. Semantischer KV-Modus
Versteht `.env`-, `.yaml`- und `.ini`-Dateien unabhängig von der Schlüsselreihenfolge.
```bash
$ qaren kv prod.env staging.env
── Modified (1 keys) ──
  ~ PORT: 5000 → 4040
```

### 3. Intelligente Rauschunterdrückung
Vergleichen Sie JSON-Backups im KV-Modus? Verwenden Sie `-D`, um Warnungen zu doppelten Schlüsseln zu unterdrücken, und `-P`, um Berechtigungswarnungen stummzuschalten. Qaren begrenzt Warnungen automatisch auf 5 pro Datei, um Ihr Terminal sauber zu halten.

---

## <img src="../icons/icons8-installation-48.png" width="24" height="24"> Installation

```bash
# Repository klonen
git clone https://github.com/qaren-cli/qaren.git
cd qaren

# Release-Binary erstellen
cargo build --release

# Das Binary befindet sich unter ./target/release/qaren
```

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> Nutzung & Beispiele

### Semantischer Vergleich (KV)
```bash
# Basis-Vergleich (erkennt automatisch = oder :)
qaren kv file1.env file2.env

# Vergleich verschiedener Formate (z. B. .env vs. .yaml)
qaren kv file1.env file2.yaml --d2 ':'

# Patch-Datei für fehlende Schlüssel generieren
qaren kv prod.env local.env -g patch.env

# Bestimmte Schlüssel oder Schlüsselwörter ignorieren
qaren kv a.env b.env -x HOSTNAME --ignore-keyword AWS

# Ausgabe als maschinenlesbares JSON
qaren kv a.env b.env --output json
```

### Wörtlicher Vergleich (Diff)
```bash
# Unified-Diff-Format (POSIX-konform)
qaren diff file1.txt file2.txt -u

# Rekursiver Verzeichnis-Diff
qaren diff -r ./backup-old ./backup-new

# ANSI-Farben vor dem Diff aus Backup-Dateien entfernen
qaren diff backup_polluted.txt backup_clean.txt -A

# Leerzeichen und Leerzeilen ignorieren
qaren diff f1.txt f2.txt -w -B
```

---

## <img src="../icons/icons8-configuration-48.png" width="24" height="24"> Konfiguration

Qaren merkt sich Ihre Einstellungen.
```bash
# Pipeline-freundlichen Modus umschalten (beendet immer mit 0)
qaren config exit toggle

# Farbausgabe umschalten
qaren config color toggle

# Aktuelle Einstellungen anzeigen
qaren config show
```

---

## <img src="../icons/icons8-performance-48.png" width="24" height="24"> Performance-Benchmarks
| Szenario | Gewinner | Vorsprung |
| :--- | :--- | :--- |
| **Große Backups (100MB)** | **Qaren** | **200x+** |
| **Rekursives Verzeichnis** | **Qaren** | **3x** |
| **Massive Änderungen (1M Zeilen)** | **Qaren** | **50x+** |

---

## <img src="../icons/icons8-contribution-64.png" width="24" height="24"> Mitwirken & Support

Wir sind **offen für Beiträge!** Ob Fehlerbehebung, neuer Parser oder Leistungsoptimierung – Ihre Pull-Requests sind willkommen.

<img src="../icons/icons8-star-.gif" width="20" height="20"> **Bitte geben Sie dem Projekt einen Stern, wenn Sie es nützlich finden!**

- **Offizielle Webseite**: [https://qaren.me/](https://qaren.me/)
- **Vollständige Dokumentation**: [https://qaren.me/docs](https://qaren.me/docs)
- **Fehlerberichte**: Gehen Sie zu [https://qaren.me/community](https://qaren.me/community) und klicken Sie auf **„Open Issue“**.

---

## <img src="../icons/icons8-licence-48.png" width="24" height="24"> Lizenz
Dieses Projekt steht unter der **MIT-Lizenz**. Weitere Details finden Sie in der Datei `LICENSE`.

---

<p align="right">(قارن) — Mit Stolz für Ingenieure entwickelt</p>

