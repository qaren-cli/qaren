# Qaren Technical Documentation

[![Release](https://github.com/qaren-cli/qaren/actions/workflows/release.yml/badge.svg?branch=master)](https://github.com/qaren-cli/qaren/actions/workflows/release.yml)

Welcome to the official documentation for **Qaren**, a blazingly fast, multi-paradigm configuration comparison tool built specifically for DevOps engineers, system administrators, and security-conscious developers.

## Overview

Standard `diff` is a literal tool. It sees characters and lines. It was designed decades ago for comparing source code logic. But in modern infrastructure, configurations (`.env`, `.yaml`, `.ini`) and system backups are *semantic*. 

Qaren re-imagines what a comparison tool should be for modern infrastructure:
- **Semantic Awareness:** Understands data formats, not just line order.
- **Zero-Trust Security:** Automatic masking of sensitive data.
- **Blazing Speed:** Optimized in Rust for GB-scale backups.
- **Automation Ready:** Native JSON output and POSIX exit codes.

## Table of Contents

- [Installation](installation.md)
- [<img src="../icons/icons8-doc-48.png" width="20" height="20"> CLI Reference & Global Options](cli-reference.md)
- **Core Commands**
  - [`kv` - Semantic Key-Value Comparison](commands/kv.md)
  - [`diff` - Literal line-by-line Comparison](commands/diff.md)
  - [`config` - Persistent Settings](commands/config.md)
- **Deep Dives**
  - [Qaren vs POSIX Diff](concepts/qaren-vs-posix-diff.md)
  - [Automation and CI/CD Guide](guides/automation-and-cicd.md)

## Dual Paradigms

Qaren provides two distinct comparison engines to cover all DevOps use cases:

1. **`qaren diff` (Literal):** An enhanced, high-speed line-by-line comparison using the Myers diff algorithm. Ideal for system backups, unstructured text, and recursive directory audits.
2. **`qaren kv` (Semantic):** An order-agnostic, format-aware Key-Value comparison engine. Ideal for configurations and structured environment variables.

---
[Return to GitHub Repository](../README.md)
