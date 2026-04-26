# Contributing to Qaren (قارن)

Thank you for your interest in contributing! To maintain Qaren's standards for speed, security, and reliability, we have a strict contribution process.

## <img src="icons/icons8-licence-48.png" width="24" height="24"> Core Principles

1.  **Security First**: Never expose secrets. Mask sensitive data by default.
2.  **Performance**: Qaren is built for speed. Any PR that significantly degrades performance without a critical reason will be rejected.
3.  **Additive Integrity**: Focus on **adding** and **improving**. Do not delete existing features or break backward compatibility without prior discussion.
4.  **Zero Warnings**: We maintain a "Zero Warning" policy for Clippy and Tests.

## <img src="icons/icons8-feature-48.png" width="24" height="24"> The Workflow

1.  **Fork & Clone**: Create your own fork of the repository and clone it locally.
2.  **Branch**: Create a feature branch (e.g., `feat/new-parser` or `fix/issue-123`).
3.  **Implement**: Write your code following existing patterns.
4.  **Document**:
    *   If you add a new flag or command, you **must** update the `--help` output in `src/commands.rs`.
    *   Update `--example` outputs and technical documentation in `technical_docs/`.
5.  **Test**: Write reviewed, comprehensive tests in the `tests/` directory of the relevant crate.
6.  **Validate**: Your code must pass the following locally before submission:
    ```bash
    cargo test --workspace
    cargo clippy --workspace -- -D warnings
    cargo build --release
    ```
7.  **Submit**: Open a Pull Request against the `main` branch.

## <img src="icons/icons8-configuration-48.png" width="24" height="24"> PR Requirements

Your Pull Request will only be merged if it meets these criteria:
- [ ] **Tests**: Includes new test cases for added functionality.
- [ ] **Zero Warnings**: Passes `cargo clippy` with no warnings.
- [ ] **Build**: Successfully builds on all platforms (handled by CI).
- [ ] **Help & Examples**: All new CLI features are reflected in `--help` and documentation.
- [ ] **Performance**: Benchmarks (if applicable) show no regression.

## 📝 Code Ownership
Note that critical files (Workflows, Install Scripts, Cargo.toml) are protected via `CODEOWNERS`. Changes to these files require explicit approval from the lead maintainer.

---
*By contributing to Qaren, you agree that your contributions will be licensed under the project's MIT License.*
