# Implementation Plan: Qaren Configuration Comparison Tool

## Overview

Qaren (قارن) is a high-performance configuration comparison tool built in Rust with three crates: `qaren-core` (library), `qaren-cli` (CLI binary), and `qaren-gui` (GUI binary — not yet started). This task list reflects the **actual implementation status** as of 2026-04-24, audited against the codebase.

---

## Phase 1: Core Foundation ✅

- [x] 1. Set up Cargo workspace and project structure
  - [x] Workspace root `Cargo.toml` with three member crates (`qaren-core`, `qaren-cli`, `qaren-gui`)
  - [x] Release profile configured (LTO, strip symbols, codegen-units=1, panic=abort)
  - [x] `.gitignore` for Rust projects

- [x] 2. Implement core data structures and error handling
  - [x] 2.1 Core types in `qaren-core/src/types.rs`
    - [x] `KvPair` struct (key, value, line_number)
    - [x] `ParseWarning` struct (key, message) — **bonus: not in original plan**
    - [x] `ConfigFile` struct (HashMap pairs, file_path, warnings)
    - [x] `ParseOptions` struct (delimiter, strip_quotes, comment_prefixes, ignore_case)
    - [x] `DiffOptions` struct (ignore_case, ignore_all_space, ignore_space_change, ignore_trailing_space, ignore_blank_lines, ignore_keys, ignore_keywords) — **exceeded spec significantly**
    - [x] `DiffResult` struct with missing/modified/identical
    - [x] `ModifiedPair` struct with line numbers for both files
    - [x] `LiteralDiffResult` and `DiffLine` structs
    - [x] `PatchDirection` enum with `Default` and `FromStr` traits
    - [x] Helper methods: `is_identical()`, `difference_count()`, `is_ignored()`
    - [x] Unit tests for all types
  - [x] 2.2 Error types in `qaren-core/src/error.rs`
    - [x] `thiserror` dependency
    - [x] `QarenError` enum (FileRead, FileWrite, FileNotFound, PermissionDenied, InvalidEncoding, InvalidDelimiter, ParseError, InvalidArguments, Io)
    - [x] `from_io_with_path` helper method
    - [x] `QarenResult<T>` type alias
    - [x] Unit tests for all error variants

---

## Phase 2: Parser Engine ✅

- [x] 3. Implement parser module
  - [x] 3.1 Parser core in `qaren-core/src/parser.rs`
    - [x] `parse_file` — reads file and delegates to `parse_content`
    - [x] `parse_content` — line-by-line iteration with BOM stripping
    - [x] `should_skip_line` — empty lines, comments, empty-prefix guard
    - [x] `parse_line` — `split_once` for safe delimiter splitting
    - [x] `process_key` — trim, `export` prefix stripping, optional quote stripping
    - [x] `process_value` — trim, comment-starts-value detection, inline comment stripping, optional quote stripping
    - [x] `strip_inline_comment` — space-prefixed markers to avoid URL false positives
    - [x] `strip_surrounding_quotes` — panic-free, byte-level, matching quotes only
    - [x] `detect_delimiter` — heuristic auto-detection (`=` vs `:`) — **bonus feature**
    - [x] Duplicate key detection with warnings — **bonus feature**
    - [x] UTF-8 BOM stripping — **bonus feature**
  - [x] 3.2 Property test: first-delimiter-only splitting (quickcheck)
  - [x] 3.3 Property test: quote preservation without stripping (quickcheck)
  - [x] 3.4 Property test: inline comment removal (quickcheck)
  - [x] 3.5 Property test: empty value handling (quickcheck)
  - [x] 3.6 Property test: parser robustness and error recovery (quickcheck)
  - [x] 3.7 Unit tests for parser edge cases (30+ tests covering: empty file, comments-only, whitespace-only, basic KV, URL preservation, quote stripping, PM2 format, shell export, unbalanced quotes, custom delimiters, CRLF, BOM, chaos audit findings)

- [x] 4. ✅ Checkpoint — Parser tests pass

---

## Phase 3: Diff Engine ✅

- [x] 5. Implement diff engine
  - [x] 5.1 Semantic diff in `qaren-core/src/diff.rs`
    - [x] O(n+m) HashMap-based algorithm
    - [x] Value normalisation (`normalise()`) for ignore_case, ignore_all_space, ignore_space_change, ignore_trailing_space
    - [x] Key ignore support (exact match and keyword substring)
    - [x] Case-insensitive key matching via lookup maps
  - [x] 5.2 Literal diff in `qaren-core/src/diff.rs`
    - [x] `similar` crate (Myers diff algorithm)
    - [x] Normalisation-aware comparison
    - [x] `ignore_blank_lines` support
    - [x] Line number tracking for old and new content
  - [x] 5.3 Property test: comparison symmetry (quickcheck)
  - [x] 5.4 Property test: linear time complexity (benchmark-style)
  - [x] 5.5 Unit tests (identical, completely different, only additions, only deletions, only modifications, mixed, empty files, one empty, exit code logic, line numbers, ignore_case, ignore_whitespace, ignore_keys, ignore_keywords, original values preserved)

---

## Phase 4: Patch Generator ✅

- [x] 6. Implement patch generator
  - [x] 6.1 Patch generator in `qaren-core/src/patch.rs`
    - [x] `generate_patch` with direction parameter
    - [x] `generate_single_patch` helper
    - [x] `append_suffix` for bidirectional file naming
    - [x] `format_kv_pair` with delimiter preservation
    - [x] SourceToTarget, TargetToSource, Bidirectional directions
    - [x] Empty-patch guard (returns empty vec, no file created)
    - [x] Separate `options1`/`options2` for cross-format patches — **exceeded spec**
  - [x] 6.2 Property test: patch completeness (quickcheck)
  - [x] 6.3 Property test: patch round-trip (quickcheck)
  - [x] 6.4 Property test: bidirectional patch symmetry (quickcheck)
  - [x] 6.5 Unit tests (empty patch, multiple missing keys, source-to-target, target-to-source, bidirectional, file naming, creation failure, delimiter preservation)

- [x] 7. ✅ Checkpoint — Core library tests pass

---

## Phase 5: Core Public API ✅

- [x] 8. Library public API in `qaren-core/src/lib.rs`
  - [x] Module-level documentation with examples
  - [x] Re-exports: `semantic_diff`, `literal_diff`, `parse_file`, `parse_content`, `detect_delimiter`, `generate_patch`
  - [x] Re-exports: all public types (`ConfigFile`, `DiffResult`, `DiffOptions`, `ParseOptions`, `PatchDirection`, etc.)
  - [x] Re-exports: error types (`QarenError`, `QarenResult`)

---

## Phase 6: CLI Application ✅

- [x] 9. CLI application structure
  - [x] 9.1 Command structure in `qaren-cli/src/commands.rs`
    - [x] `Cli` struct with `clap` derive — includes `--example` and `--generate-completions`
    - [x] `Commands` enum: `Diff`, `Kv` (alias `kvp`), `Config` — **Config is a bonus subcommand**
    - [x] `SharedDiffOptions` (ignore_case, ignore_all_space) flattened into both commands
    - [x] `Diff` command: file1, file2, unified, ignore_space_change, ignore_trailing_space, ignore_blank_lines, brief, report_identical_files
    - [x] `Kv` command: file1, file2, delimiter, d1, d2 (per-file delimiters), strip_quotes, output format (text/json), ignore_keys, ignore_keywords, quiet, summary, show_secrets, verbose, generate_missing, direction
    - [x] `validate_delimiter` with unit tests
    - [x] After-help text with exit codes
  - [x] 9.2 Secret masking in `qaren-cli/src/masking.rs`
    - [x] PRD keywords + defense-in-depth additions (credential, cert, private, signing, connection_string, conn_str, url, dsn, redis, rabbit, amqp, postgres, mongo, db)
    - [x] `should_mask` — case-insensitive substring matching
    - [x] `mask_value` — returns `***MASKED***` or actual value
    - [x] Comprehensive unit tests
  - [x] 9.3 Output formatting in `qaren-cli/src/output.rs`
    - [x] `print_diff_result` — semantic diff with color (red/green/yellow/dimmed)
    - [x] `print_literal_diff` — literal diff with color
    - [x] `print_json_diff` — JSON output format — **bonus feature**
    - [x] Secret masking integrated in all output paths
    - [x] Verbose mode (show identical keys)
    - [x] Summary line with counts

- [x] 10. CLI main entry point and command handlers
  - [x] 10.1 Main entry point in `qaren-cli/src/main.rs`
    - [x] `run()` function with proper exit codes (0, 1, 2)
    - [x] `handle_diff_command` — literal diff with unified, brief, report_identical_files, ignore flags
    - [x] `handle_kv_command` — semantic KV with all options
    - [x] `resolve_delimiter` — per-file > shared > auto-detect priority chain
    - [x] `print_error_hints` — contextual hints for common errors
    - [x] Config-aware exit code behavior (POSIX vs pipeline-friendly)
    - [x] `NO_COLOR` environment variable support
    - [x] Auto-disable color for JSON output
    - [x] Duplicate key warnings (per-warning and summary modes)
    - [x] Cross-format delimiter auto-detection notification
  - [x] 10.2 Patch generation in CLI
    - [x] `--generate-missing` / `-g` flag
    - [x] `--direction` validation (requires `--generate-missing`)
    - [x] Per-file delimiter preservation in patches
    - [x] Success/info messages
  - [x] 10.3 Persistent config in `qaren-cli/src/config.rs` — **bonus module**
    - [x] `QarenConfig` struct (exit_nonzero_on_diff, color)
    - [x] XDG/APPDATA-aware config path
    - [x] Load/save TOML-style config without extra crates
    - [x] `qaren config show/exit toggle/color toggle/path`
  - [x] 10.4 Usage examples in `qaren-cli/src/examples.rs` — **bonus module**
    - [x] Rich colored examples for `kv`, `diff`, and general usage
    - [x] Accessible via `--example` flag
  - [ ] 10.5 CLI integration tests — **NOT YET WRITTEN**
    - [ ] Test `qaren diff` with identical files (exit 0)
    - [ ] Test `qaren diff` with different files (exit 1)
    - [ ] Test `qaren kv` with custom delimiter
    - [ ] Test `qaren kv` with --strip-quotes
    - [ ] Test `qaren kv` with --generate-missing
    - [ ] Test `qaren kv` with all --direction variants
    - [ ] Test file not found (exit 2)
    - [ ] Test invalid delimiter (exit 2)
    - [ ] Test --direction without --generate-missing
    - [ ] Test --help and --version
    - [ ] Verify secret masking in output
    - [ ] Test JSON output format
    - [ ] Test quiet mode
    - [ ] Test summary mode

- [x] 11. ✅ Checkpoint — CLI compiles and runs (integration tests pending)

---

## Phase 7: Release v0.1.0 (CLI-Only) — PARTIALLY DONE

- [x] Release binaries built
  - [x] `qaren-linux-amd64` (1.0 MB)
  - [x] `qaren-windows-amd64.exe` (830 KB)
  - [ ] macOS Intel binary
  - [ ] macOS Apple Silicon binary

---

## Phase 8: GUI Application — NOT STARTED

- [ ] 12. GUI application structure
  - [ ] 12.1 Add `egui`, `eframe`, `serde` dependencies to `qaren-gui/Cargo.toml`
  - [ ] 12.2 Create `QarenApp` struct with application state
  - [ ] 12.3 Create GUI main entry point (window title, size, persistence)
  - [ ] 12.4 Create secret masking for GUI

- [ ] 13. GUI file loading and comparison display
  - [ ] 13.1 File drag-and-drop with drop zones
  - [ ] 13.2 Comparison execution on button click
  - [ ] 13.3 Color-coded results display (green/red/yellow)
  - [ ] 13.4 Summary with counts

- [ ] 14. GUI settings panel
  - [ ] 14.1 Delimiter dropdown + custom input, strip quotes checkbox, show secrets checkbox
  - [ ] 14.2 Settings persistence via eframe

- [ ] 15. GUI export functionality
  - [ ] 15.1 Export UI with direction options and key counts
  - [ ] 15.2 Native file save dialog + patch generation
  - [ ] 15.3 GUI integration tests

- [ ] 16. ✅ Checkpoint — GUI tests pass

---

## Phase 9: Polish & Quality — NOT STARTED

- [ ] 17. Performance benchmarks
  - [ ] 17.1 Criterion benchmarks (parse 1K/10K lines, diff 1K/10K pairs)
  - [ ] Verify sub-100ms for 1000-line files

- [ ] 18. Cross-platform testing
  - [ ] 18.1 Linux (verified for CLI)
  - [ ] 18.2 macOS (Intel + Apple Silicon)
  - [ ] 18.3 Windows (CRLF handling verified in unit tests)

- [ ] 19. Documentation
  - [ ] 19.1 README.md with installation, usage, examples, screenshots
  - [ ] 19.2 API documentation (`cargo doc`)
  - [ ] 19.3 Example configuration files

- [ ] 20. Build and release preparation
  - [ ] 20.1 CI/CD pipeline (GitHub Actions)
  - [ ] 20.2 Binary size optimization verification
  - [ ] 20.3 Release artifacts with checksums

- [ ] 21. Final checkpoint — Comprehensive testing

---

## Summary

| Component | Status | Details |
|-----------|--------|---------|
| **qaren-core types** | ✅ Done | Exceeded spec (DiffOptions, ParseWarning, ignore_keys/keywords) |
| **qaren-core error** | ✅ Done | Full `thiserror` implementation with unit tests |
| **qaren-core parser** | ✅ Done | 30+ unit tests, 5 property tests, BOM/CRLF/export/auto-detect |
| **qaren-core diff** | ✅ Done | Semantic + literal, ignore flags, 15+ unit tests, 2 property tests |
| **qaren-core patch** | ✅ Done | Bidirectional, cross-format, 8+ unit tests, 3 property tests |
| **qaren-core lib API** | ✅ Done | Full re-exports with documentation |
| **qaren-cli commands** | ✅ Done | `diff`, `kv`, `config` subcommands with rich options |
| **qaren-cli masking** | ✅ Done | 20+ keywords, comprehensive tests |
| **qaren-cli output** | ✅ Done | Text + JSON, verbose/summary/quiet modes |
| **qaren-cli main** | ✅ Done | Exit codes, error hints, config-aware, auto-detect |
| **qaren-cli config** | ✅ Done | Persistent XDG config (bonus) |
| **qaren-cli examples** | ✅ Done | Rich colored examples (bonus) |
| **CLI integration tests** | ❌ Pending | `assert_cmd`/`predicates` in dev-deps but no tests written |
| **Release binaries** | 🟡 Partial | Linux + Windows built; macOS pending |
| **qaren-gui** | ❌ Not started | Placeholder `main.rs` only |
| **Benchmarks** | ❌ Not started | |
| **CI/CD** | ❌ Not started | |
| **Documentation** | ❌ Not started | |

### Bonus features implemented (beyond original spec):
- `qaren config` subcommand with persistent settings
- `--example` flag with rich colored usage examples
- `--generate-completions` for shell completion scripts
- Per-file delimiter overrides (`--d1`, `--d2`)
- Auto-delimiter detection (`detect_delimiter`)
- JSON output format (`-o json`)
- Quiet mode (`-q`) and Summary mode (`-s`)
- Ignore keys (`-x`) and ignore keywords (`--ignore-keyword`)
- Unified diff output (`-u`)
- `--brief` and `--report-identical-files` flags
- Duplicate key detection with warnings
- UTF-8 BOM stripping
- `NO_COLOR` environment variable support
- Defense-in-depth secret keywords (20+ keywords vs 5 in PRD)
