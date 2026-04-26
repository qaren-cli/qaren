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
  <b>La prochaine génération de comparaison de configurations et de journaux.</b><br>
  Conçu pour l'ère moderne du DevOps : Sémantique, Sécurisé et Incroyablement Rapide.
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

## 🚀 Pourquoi Qaren ?

L'outil standard POSIX `diff` nous sert depuis 50 ans, mais il a été conçu pour le code source, et non pour les fichiers de configuration complexes et indépendants de l'ordre, ni pour les journaux massifs d'aujourd'hui.

Qaren (arabe pour **"Comparer"**) est un outil multi-paradigme qui comprend vos données.

- **Analyse sémantique Clé-Valeur** : L'ordre n'a pas d'importance. Le formatage n'a pas d'importance. Seules les données comptent.
- **Sécurité Zero-Trust** : Les secrets tels que les clés API, les mots de passe et les chaînes de connexion sont masqués par défaut (`***MASKED***`).
- **Incroyablement Rapide** : Optimisé en Rust pour traiter des journaux à l'échelle du Go et plus de 100k clés jusqu'à **200x plus vite** que les pipelines diff traditionnels.
- **Compatible ANSI** : Nettoie automatiquement les codes de couleur du terminal des fichiers "pollués" (comme la sortie de `pm2 env`) pour une comparaison propre.
- **Patching Intelligent** : Générez des correctifs `.env` prêts pour la production afin de synchroniser les environnements en quelques secondes.

---

## 📚 Documentation
Pour des guides détaillés, la référence API et la configuration avancée, visitez notre site de documentation :
👉 **[https://qaren.me/docs](https://qaren.me/docs)**

---

## 🛠️ Caractéristiques principales

### 1. Sortie littérale améliorée
Qaren fournit des diffs ligne par ligne beaucoup plus clairs que POSIX diff, spécifiquement optimisés pour l'analyse des fichiers journaux (logs).
```bash
$ qaren diff old.log new.log -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### 2. Mode KV sémantique
Comprend les fichiers `.env`, `.yaml` et `.ini` quel que soit l'ordre des clés.
```bash
$ qaren kv prod.env staging.env
── Modified (1 keys) ──
  ~ PORT: 5000 → 4040
```

### 3. Réduction intelligente du bruit
Vous comparez des journaux JSON en mode KV ? Utilisez `-D` pour supprimer les avertissements de clés dupliquées et `-P` pour masquer les alertes de permissions. Qaren limite automatiquement les avertissements à 5 par fichier pour garder votre terminal propre.

---

## 📥 Installation

```bash
# Cloner le dépôt
git clone https://github.com/yourusername/qaren.git
cd qaren

# Compiler le binaire de version
cargo build --release

# Le binaire sera disponible à l'adresse ./target/release/qaren
```

---

## 📖 Utilisation & Exemples

### Comparaison Sémantique (KV)
```bash
# Comparaison de base (détection automatique = ou :)
qaren kv file1.env file2.env

# Comparer différents formats (ex: .env vs .yaml)
qaren kv file1.env file2.yaml --d2 ':'

# Générer un fichier de patch pour les clés manquantes
qaren kv prod.env local.env -g patch.env

# Ignorer des clés ou des mots-clés spécifiques
qaren kv a.env b.env -x HOSTNAME --ignore-keyword AWS

# Sortie au format JSON lisible par machine
qaren kv a.env b.env --output json
```

### Comparaison Littérale (Diff)
```bash
# Format unified diff (conforme POSIX)
qaren diff file1.txt file2.txt -u

# Diff de répertoire récursif
qaren diff -r ./logs-old ./logs-new

# Supprimer les couleurs ANSI des fichiers journaux avant le diff
qaren diff logs_polluted.txt logs_clean.txt -A

# Ignorer les espaces blancs et les lignes vides
qaren diff f1.txt f2.txt -w -B
```

---

## ⚙️ Configuration

Qaren se souvient de vos préférences.
```bash
# Basculer en mode compatible pipeline (quitte toujours avec 0)
qaren config exit toggle

# Basculer la sortie couleur
qaren config color toggle

# Afficher les paramètres actuels
qaren config show
```

---

## 📊 Benchmarks de Performance
| Scénario | Gagnant | Marge |
| :--- | :--- | :--- |
| **Gros Logs (100Mo)** | **Qaren** | **200x+** |
| **Répertoire Récursif** | **Qaren** | **3x** |
| **Changements Massifs (1M de lignes)** | **Qaren** | **50x+** |

---

## 🤝 Contribution & Support

Nous sommes **Ouverts aux Contributions !** Qu'il s'agisse d'une correction de bug, d'un nouveau parseur ou d'une amélioration des performances, vos PR sont les bienvenues.

⭐ **N'hésitez pas à donner une étoile au projet si vous le trouvez utile !**

- **Site officiel** : [https://qaren.me/](https://qaren.me/)
- **Documentation complète** : [https://qaren.me/docs](https://qaren.me/docs)
- **Rapports de bugs** : Allez sur [https://qaren.me/community](https://qaren.me/community) et cliquez sur **"Open Issue"**.

---

## 📜 Licence
Ce projet est sous **licence MIT**. Voir le fichier `LICENSE` pour plus de détails.

---

<p align="right">(قارن) — Créé avec fierté pour les ingénieurs</p>
