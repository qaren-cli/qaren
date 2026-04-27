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
  <b>La prochaine génération de comparaison de configurations et de sauvegardes système.</b><br>
  Conçu pour l'ère moderne du DevOps : Sémantique, Sécurisé et Incroyablement Rapide.
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

## Pourquoi Qaren ? <img src="../icons/favicon.png" width="24" height="24"> &nbsp; [<img src="../icons/icons8-linkedin-48.png" width="24" height="24">](https://www.linkedin.com/in/alielesawy) &nbsp; [<img src="../icons/icons8-github-48.png" width="24" height="24">](https://github.com/alielesawy)

L'outil standard POSIX `diff` nous sert depuis 50 years, mais il a été conçu pour le code source, et non pour les fichiers de configuration complexes et indépendants de l'ordre, ni pour les sauvegardes système massives d'aujourd'hui.

Qaren (arabe pour **"Comparer"**) est un outil multi-paradigme qui comprend vos données.

- **Analyse sémantique Clé-Valeur** : L'ordre n'a pas d'importance. Le formatage n'a pas d'importance. Seules les données comptent.
- **Sécurité Zero-Trust** : Les secrets tels que les clés API, les mots de passe et les chaînes de connexion sont masqués par défaut (`***MASKED***`).
- **Incroyablement Rapide** : Optimisé en Rust pour traiter des sauvegardes système à l'échelle du Go et plus de 100k clés jusqu'à **200x plus vite** que les pipelines diff traditionnels.
- **Compatible ANSI** : Nettoie automatiquement les codes de couleur du terminal des fichiers "pollués" (comme la sortie de `pm2 env`) pour une comparaison propre.
- **Patching Intelligent** : Générez des correctifs `.env` prêts pour la production afin de synchroniser les environnements en quelques secondes.

---

## <img src="../icons/icons8-doc-48.png" width="24" height="24"> Documentation
Pour des guides détaillés, la référence API et la configuration avancée, visitez notre site de documentation :
> **[https://qaren.me/docs](https://qaren.me/docs)**

---

## <img src="../icons/icons8-feature-48.png" width="24" height="24"> Caractéristiques principales

### 1. Mode KV sémantique
Comprend les fichiers `.env`, `.yaml` et `.ini` quel que soit l'ordre des clés.
<p align="center">
  <img src="../icons/Qd2.gif" width="100%" alt="Mode KV Sémantique">
</p>

### 2. Sortie littérale améliorée
Qaren fournit des diffs ligne par ligne beaucoup plus clairs que POSIX diff, spécifiquement optimisés pour l'analyse des fichiers de sauvegarde système.
```bash
$ qaren diff backup-old backup-new -w
-[L47] TimeoutOverflowWarning: does not fit into a 32-bit integer.
+[L47] TimeoutOverflowWarning: 3000010000 does not fit into a 32-bit integer.
```

### 3. Réduction intelligente du bruit
Vous comparez des sauvegardes JSON en mode KV ? Qaren supprime automatiquement les avertissements de clés dupliquées et de permissions par défaut pour garder votre terminal propre. Si vous avez besoin d'aide pour le débogage, lancez `qaren config advisor toggle` pour activer les alertes utiles.

---

## <img src="../icons/icons8-installation-48.png" width="24" height="24"> Installation

### Installation Rapide (Automatisée)

| Plateforme | Commande |
| :--- | :--- |
| **Linux / macOS** | `curl -sSfL https://qaren.me/install | sh` |
| **Windows** | `irm https://qaren.me/install.ps1 | iex` |
| **Homebrew** | `brew tap qaren-cli/qaren && brew install qaren` |

### Méthodes Alternatives
```bash
# Via Cargo
cargo install qaren
```

---

## <img src="../icons/icons8-rust-48.png" width="24" height="24"> Utilisation & Exemples

### Comparaison Sémantique (KV)
Le mode `kv` de Qaren est conçu pour les tâches DevOps réelles. Voici les modèles courants pour comparer des fichiers d'environnement :

| Tâche | Commande | Visuel |
| :--- | :--- | :--- |
| **Diff Sémantique de Base** | `qaren kv -Q --d2 ":" dev.env staging.env` | <img src="../icons/Qd2.gif" width="400"> |
| **Mode Résumé** | `qaren kv -Q --d2 ":" dev.env staging.env -s` | <img src="../icons/Qd2s.gif" width="400"> |
| **Export JSON** | `qaren kv -Q --d2 ":" dev.env staging.env -o json` | <img src="../icons/Qd2o.gif" width="400"> |
| **Afficher les Secrets** | `qaren kv -Q --d2 ":" dev.env staging.env -S` | <img src="../icons/Qd2S.gif" width="400"> |
| **Ignorer des Clés** | `qaren kv -Q --d2 ":" dev.env staging.env -x API_KEY` | <img src="../icons/Qd2x.gif" width="400"> |
| **Ignorer des Mots-clés**| `qaren kv --ignore-keyword MAX ...` | <img src="../icons/Qd2-ignore-keyword.gif" width="400"> |
| **Mode Silencieux** | `qaren kv -Q --d2 ":" dev.env staging.env -q` | <img src="../icons/Qd2q.gif" width="400"> |
| **Génération de Patch**| `qaren kv ... -g missing.env` | <img src="../icons/Qd2g.gif" width="400"> |
| **Patches Sécurisés** | `qaren kv ... -g missing.env --mask-patches` | <img src="../icons/Qd2g-masked.gif" width="400"> |

### Comparaison Littérale (Diff)
```bash
# Format unified diff (conforme POSIX)
qaren diff file1.txt file2.txt -u

# Diff de répertoire récursif
qaren diff -r ./backup-old ./backup-new

# Supprimer les couleurs ANSI des fichiers de sauvegarde avant le diff
qaren diff backup_polluted.txt backup_clean.txt -A

# Ignorer les espaces blancs et les lignes vides
qaren diff f1.txt f2.txt -w -B
```

---

## <img src="../icons/icons8-configuration-48.png" width="24" height="24"> Configuration

Qaren se souvient de vos préférences.
<p align="center">
  <img src="../icons/config-color.gif" width="100%" alt="Bascule Couleur Config">
</p>

```bash
# Basculer en mode compatible pipeline (quitte toujours avec 0)
qaren config exit toggle

# Basculer la sortie couleur
qaren config color toggle

# Basculer l'Advisor (avertissements)
qaren config advisor toggle

# Basculer le masquage des secrets
qaren config masking toggle

# Afficher les paramètres actuels
qaren config show
```

---

## <img src="../icons/icons8-performance-48.png" width="24" height="24"> Benchmarks de Performance
| Scénario | Gagnant | Marge |
| :--- | :--- | :--- |
| **Grosses Sauvegardes (100Mo)** | **Qaren** | **200x+** |
| **Répertoire Récursif** | **Qaren** | **3x** |
| **Changements Massifs (1M de lignes)** | **Qaren** | **50x+** |

---

## <img src="../icons/icons8-contribution-64.png" width="24" height="24"> Contribution & Support

Nous sommes **Ouverts aux Contributions !** Veuillez lire notre **[Guide de Contribution](CONTRIBUTING.md)** avant de soumettre une Pull Request.

- [ ] **Fork** le dépôt.
- [ ] **Améliorer** ou **Ajouter** des fonctionnalités (éviter les suppressions).
- [ ] Assurer **Zéro Avertissement** (`clippy` & `tests`).
- [ ] Mettre à jour les **Docs** et le **--help** pour les nouveaux flags.

<img src="../icons/icons8-star-.gif" width="20" height="20"> **N'hésitez pas à donner une étoile au projet si vous le trouvez utile !**

- **Site officiel** : [https://qaren.me/](https://qaren.me/)
- **Documentation complète** : [https://qaren.me/docs](https://qaren.me/docs)
- **Rapports de bugs** : Allez sur [https://qaren.me/community](https://qaren.me/community) et cliquez sur **"Open Issue"**.

---

## <img src="../icons/icons8-licence-48.png" width="24" height="24"> Licence
Ce projet est sous **licence MIT**. Voir le fichier `LICENSE` pour plus de détails.

---

<p align="right">(قارن) — Créé avec fierté pour les ingénieurs</p>
