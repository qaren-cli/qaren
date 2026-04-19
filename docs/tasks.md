# Implementation Plan: Qaren Configuration Comparison Tool

## Overview

This implementation plan breaks down the Qaren (قارن) configuration comparison tool into discrete, incremental coding tasks. The tool is built in Rust with three main components: a core library (qaren-core), a CLI application (qaren-cli), and a GUI application (qaren-gui). Each task builds on previous work, with property-based tests integrated throughout to validate correctness properties from the design document.

## Tasks

- [ ] 1. Set up Cargo workspace and project structure
  - Create workspace root `Cargo.toml` with three member crates
  - Create `qaren-core/` directory with library crate structure
  - Create `qaren-cli/` directory with binary crate structure
  - Create `qaren-gui/` directory with binary crate structure
  - Configure release profile with optimization settings (LTO, strip symbols)
  - Add workspace-level `.gitignore` for Rust projects
  - _Requirements: 19.1, 19.2, 19.3_

- [ ] 2. Implement core data structures and error handling
  - [ ] 2.1 Create core type definitions in `qaren-core/src/types.rs`
    - Define `KvPair` struct with key, value, and line_number fields
    - Define `ConfigFile` struct with HashMap and file_path
    - Define `ParseOptions` struct with delimiter, strip_quotes, and comment_prefixes
    - Implement `Default` trait for `ParseOptions`
    - Define `DiffResult` struct with missing/modified/identical vectors
    - Define `ModifiedPair` struct for value differences
    - Define `LiteralDiffResult` and `DiffLine` structs
    - Define `PatchDirection` enum (SourceToTarget, TargetToSource, Bidirectional)
    - Implement `FromStr` trait for `PatchDirection` with aliases
    - Implement helper methods: `DiffResult::is_identical()`, `DiffResult::difference_count()`
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 21.1, 21.2, 21.3_

  - [ ] 2.2 Create error types in `qaren-core/src/error.rs`
    - Add `thiserror` dependency to `qaren-core/Cargo.toml`
    - Define `QarenError` enum with variants: FileRead, FileWrite, FileNotFound, PermissionDenied, InvalidEncoding, InvalidDelimiter, ParseError, InvalidArguments, Io
    - Implement `from_io_with_path` helper method
    - Define `QarenResult<T>` type alias
    - _Requirements: 14.1, 14.2, 14.3, 14.4_

- [ ] 3. Implement parser module with safe splitting and quote stripping
  - [ ] 3.1 Create parser core in `qaren-core/src/parser.rs`
    - Implement `parse_file` function that reads file and calls `parse_content`
    - Implement `parse_content` function with line-by-line iteration
    - Implement `should_skip_line` function for empty lines and comments
    - Implement `parse_line` function using `split_once` for safe delimiter splitting
    - Implement `process_token` function for trimming and quote handling
    - Implement `strip_surrounding_quotes` function (preserve internal quotes)
    - Handle malformed lines gracefully (skip and continue)
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 5.1, 5.2, 5.3, 5.4, 5.5, 5.6, 6.1, 6.2, 6.3, 6.4, 20.1, 20.2, 20.3, 20.4, 20.5, 20.6, 20.7_

  - [ ]* 3.2 Write property test for first-delimiter-only splitting
    - **Property 2: First-Delimiter-Only Splitting**
    - **Validates: Requirements 4.5**
    - Add `quickcheck` and `quickcheck_macros` to dev-dependencies
    - Generate random lines with 1-10 delimiters
    - Parse each line and verify value contains exactly N-1 delimiters
    - Run 100+ iterations
    - _Requirements: 4.5_

  - [ ]* 3.3 Write property test for quote preservation without stripping
    - **Property 3: Quote Preservation Without Stripping**
    - **Validates: Requirements 5.6**
    - Generate random strings with various quote patterns
    - Parse with `strip_quotes=false`
    - Verify all quotes are preserved exactly (round-trip identity)
    - Run 100+ iterations
    - _Requirements: 5.6_

  - [ ]* 3.4 Write property test for inline comment removal
    - **Property 4: Inline Comment Removal**
    - **Validates: Requirements 6.4**
    - Generate random KV pairs followed by comment characters and text
    - Parse and verify comment content is excluded from value
    - Run 100+ iterations
    - _Requirements: 6.4_

  - [ ]* 3.5 Write property test for empty value handling
    - **Property 8: Empty Value Handling**
    - **Validates: Requirements 20.2**
    - Generate keys followed by delimiter but no value (e.g., `KEY=`)
    - Verify parser produces empty string as value
    - Run 100+ iterations
    - _Requirements: 20.2_

  - [ ]* 3.6 Write property test for parser robustness
    - **Property 9: Parser Robustness and Error Recovery**
    - **Validates: Requirements 20.1, 20.7**
    - Generate files with random mix of valid and malformed lines
    - Verify all valid lines are successfully parsed
    - Verify parser continues processing without terminating
    - Run 100+ iterations
    - _Requirements: 20.1, 20.7_

  - [ ]* 3.7 Write unit tests for parser edge cases
    - Test empty file handling
    - Test file with only comments
    - Test file with only whitespace
    - Test quote stripping with internal quotes: `'John "The Boss" Doe'`
    - Test URL preservation: `URL=https://api.com?id=1&key=value`
    - Test PM2 format: `"DATABASE_URL":"postgres://host:5432/db"`
    - Test shell export format: `export KEY=value`
    - Test unbalanced quotes
    - Test line with only delimiter
    - Test custom delimiters (`:`, space)
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 17.1, 17.2, 17.3, 17.4, 17.5, 20.1, 20.2, 20.3, 20.4_

- [ ] 4. Checkpoint - Ensure parser tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 5. Implement diff engine for semantic and literal comparison
  - [ ] 5.1 Create semantic diff in `qaren-core/src/diff.rs`
    - Implement `semantic_diff` function with O(n+m) HashMap-based algorithm
    - Iterate through file1 pairs and check against file2 HashMap
    - Identify missing_in_file2, modified, and identical keys
    - Iterate through file2 pairs to find missing_in_file1
    - Return `DiffResult` with all categories populated
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 13.4_

  - [ ] 5.2 Create literal diff in `qaren-core/src/diff.rs`
    - Add `similar` crate dependency to `qaren-core/Cargo.toml`
    - Implement `literal_diff` function using `TextDiff::from_lines`
    - Process diff changes and categorize as additions, deletions, modifications
    - Track line numbers for each change
    - Return `LiteralDiffResult`
    - _Requirements: 1.1, 1.2, 1.3, 1.4_

  - [ ]* 5.3 Write property test for comparison symmetry
    - **Property 1: Comparison Symmetry (Inverse Relationship)**
    - **Validates: Requirements 2.7**
    - Generate random configuration file pairs A and B
    - Compare A→B and B→A
    - Verify missing_in_file2 for A→B equals missing_in_file1 for B→A
    - Verify modified pairs have swapped file1/file2 values
    - Run 100+ iterations
    - _Requirements: 2.7_

  - [ ]* 5.4 Write property test for linear time complexity
    - **Property 7: Linear Time Complexity**
    - **Validates: Requirements 13.4**
    - Generate config files of sizes 100, 1000, 10000 pairs
    - Measure comparison time for each size
    - Verify time scales linearly (time ratio ≈ size ratio)
    - Run multiple iterations for statistical significance
    - _Requirements: 13.4_

  - [ ]* 5.5 Write unit tests for diff engine
    - Test identical files (all keys in identical category)
    - Test completely different files (all keys in missing categories)
    - Test files with only additions
    - Test files with only deletions
    - Test files with only modifications
    - Test mixed differences (additions, deletions, modifications)
    - Test empty files
    - Verify exit code logic (0 for identical, 1 for different)
    - _Requirements: 1.5, 1.6, 2.1, 2.2, 2.3, 2.4, 2.5_

- [ ] 6. Implement patch generator with bidirectional support
  - [ ] 6.1 Create patch generator in `qaren-core/src/patch.rs`
    - Implement `generate_patch` function with direction parameter
    - Implement `generate_single_patch` helper for one direction
    - Implement `append_suffix` helper for bidirectional file naming
    - Implement `format_kv_pair` helper to format output lines
    - Handle SourceToTarget direction (missing_in_file2)
    - Handle TargetToSource direction (missing_in_file1)
    - Handle Bidirectional direction (create two files with suffixes)
    - Return vector of created file paths
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 21.1, 21.2, 21.3, 21.4, 21.5_

  - [ ]* 6.2 Write property test for patch completeness
    - **Property 5: Patch Completeness and Correctness**
    - **Validates: Requirements 7.2, 21.1, 21.2**
    - Generate random file pairs A and B
    - Generate patch with source-to-target direction
    - Verify patch contains exactly keys in A missing from B
    - Generate patch with target-to-source direction
    - Verify patch contains exactly keys in B missing from A
    - Run 100+ iterations
    - _Requirements: 7.2, 21.1, 21.2_

  - [ ]* 6.3 Write property test for patch round-trip
    - **Property 6: Patch Formatting Preservation (Round-Trip)**
    - **Validates: Requirements 7.3**
    - Generate config file A and file B where A has keys missing in B
    - Generate patch from diff
    - Parse the generated patch file
    - Verify parsed values match original values from A
    - Run 100+ iterations
    - _Requirements: 7.3_

  - [ ]* 6.4 Write property test for bidirectional patch symmetry
    - **Property 10: Bidirectional Patch Symmetry**
    - **Validates: Requirements 21.3**
    - Generate random file pairs A and B
    - Generate bidirectional patches
    - Verify source-to-target patch contains exactly keys in A missing from B
    - Verify target-to-source patch contains exactly keys in B missing from A
    - Verify no overlap between the two patch files
    - Run 100+ iterations
    - _Requirements: 21.3_

  - [ ]* 6.5 Write unit tests for patch generation
    - Test empty patch (no missing keys)
    - Test patch with multiple missing keys
    - Test source-to-target direction
    - Test target-to-source direction
    - Test bidirectional patch generation
    - Verify correct file naming: `output.source-to-target.env`, `output.target-to-source.env`
    - Test patch file creation failure (invalid path)
    - Test delimiter preservation in patch output
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6, 21.1, 21.2, 21.3, 21.4, 21.5_

- [ ] 7. Checkpoint - Ensure core library tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 8. Create library public API in `qaren-core/src/lib.rs`
  - Export all public types from `types` module
  - Export all public functions from `parser` module
  - Export all public functions from `diff` module
  - Export all public functions from `patch` module
  - Export error types from `error` module
  - Add module-level documentation with examples
  - _Requirements: 19.1_

- [ ] 9. Implement CLI application structure
  - [ ] 9.1 Create CLI command structure in `qaren-cli/src/commands.rs`
    - Add `clap` dependency with derive features to `qaren-cli/Cargo.toml`
    - Add `qaren-core` dependency to `qaren-cli/Cargo.toml`
    - Define `Cli` struct with clap derive macros
    - Define `Commands` enum with `Diff` and `Kvp` variants
    - Add fields for `Diff` command: file1, file2
    - Add fields for `Kvp` command: file1, file2, delimiter, strip_quotes, generate_missing, direction, show_secrets
    - Add validation for delimiter (must be single character)
    - Add validation for direction flag (requires generate_missing)
    - _Requirements: 16.1, 16.2, 16.3, 16.4, 16.5, 16.6, 16.7, 3.1, 3.2, 3.3, 3.4, 3.5, 7.1, 21.6, 21.7, 21.8_

  - [ ] 9.2 Create secret masking module in `qaren-cli/src/masking.rs`
    - Define `SECRET_KEYWORDS` constant array: ["key", "password", "secret", "token", "auth"]
    - Implement `should_mask` function with case-insensitive keyword matching
    - Implement `mask_value` function that returns "***MASKED***" or actual value
    - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5, 8.6, 8.7, 8.8_

  - [ ] 9.3 Create output formatting module in `qaren-cli/src/output.rs`
    - Add `colored` crate dependency to `qaren-cli/Cargo.toml`
    - Implement `print_diff_result` function for semantic diff output
    - Implement `print_literal_diff` function for literal diff output
    - Use green color for additions
    - Use red color for deletions
    - Use yellow color for modifications
    - Group output into sections: Missing in file2, Missing in file1, Modified, Identical
    - Display summary line with counts
    - Apply secret masking to values based on show_secrets flag
    - _Requirements: 1.2, 1.3, 1.4, 18.1, 18.2, 18.3, 18.4, 18.5, 18.6, 18.7, 8.6, 8.7_

  - [ ]* 9.4 Write unit tests for secret masking
    - Test keys containing "key", "password", "secret", "token", "auth"
    - Test case-insensitive matching (API_KEY, api_key, Api_Key)
    - Test masking disabled with show_secrets=true
    - Test keys without sensitive keywords (not masked)
    - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5, 8.6, 8.7_

- [ ] 10. Implement CLI main entry point and command handlers
  - [ ] 10.1 Create main entry point in `qaren-cli/src/main.rs`
    - Parse command-line arguments with clap
    - Implement `run` function that dispatches to command handlers
    - Implement error handling with proper exit codes (0, 1, 2)
    - Write errors to stderr
    - Implement `handle_diff_command` function
    - Implement `handle_kvp_command` function
    - Handle file not found errors with descriptive messages
    - Handle permission denied errors with hints
    - Handle invalid UTF-8 errors with line numbers
    - Handle invalid delimiter errors with examples
    - _Requirements: 1.5, 1.6, 1.7, 14.1, 14.2, 14.3, 14.4, 14.5, 14.6, 16.1, 16.2, 16.3_

  - [ ] 10.2 Implement patch generation in CLI
    - Check if `--generate-missing` flag is provided
    - Validate that `--direction` is only used with `--generate-missing`
    - Parse direction string to `PatchDirection` enum
    - Call `generate_patch` from qaren-core
    - Display success message with created file path(s)
    - Handle file write errors with descriptive messages
    - _Requirements: 7.1, 7.6, 7.7, 21.1, 21.2, 21.3, 21.4, 21.5, 21.6, 21.7, 21.8_

  - [ ]* 10.3 Write CLI integration tests
    - Add `tempfile`, `assert_cmd`, and `predicates` to dev-dependencies
    - Test `qaren diff` command with identical files (exit code 0)
    - Test `qaren diff` command with different files (exit code 1)
    - Test `qaren kvp` command with custom delimiter
    - Test `qaren kvp` command with --strip-quotes flag
    - Test `qaren kvp` command with --generate-missing flag
    - Test `qaren kvp` command with --direction source-to-target
    - Test `qaren kvp` command with --direction target-to-source
    - Test `qaren kvp` command with --direction bidirectional
    - Test file not found error (exit code 2)
    - Test invalid delimiter error (exit code 2)
    - Test --direction without --generate-missing error
    - Test --help flag
    - Test --version flag
    - Verify colored output in terminal
    - Verify secret masking in output
    - _Requirements: 1.5, 1.6, 1.7, 3.1, 3.2, 3.3, 5.1, 7.1, 14.1, 14.2, 14.5, 16.4, 16.5, 21.6, 21.7_

- [ ] 11. Checkpoint - Ensure CLI tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 12. Implement GUI application structure
  - [ ] 12.1 Create GUI application state in `qaren-gui/src/app.rs`
    - Add `egui`, `eframe`, and `serde` dependencies to `qaren-gui/Cargo.toml`
    - Add `qaren-core` dependency to `qaren-gui/Cargo.toml`
    - Define `QarenApp` struct with file paths, diff result, settings, and UI state
    - Define `AppSettings` struct with delimiter, strip_quotes, show_secrets
    - Implement `Default` trait for `AppSettings`
    - Define `UiState` struct with show_settings, custom_delimiter_input, error_message, success_message, show_export_menu
    - Implement `eframe::App` trait for `QarenApp`
    - _Requirements: 10.1, 10.2, 10.3, 10.4, 11.1, 11.2, 11.3, 11.4, 11.5, 11.6, 11.7_

  - [ ] 12.2 Create GUI main entry point in `qaren-gui/src/main.rs`
    - Implement `main` function with eframe native options
    - Set window title to "Qaren - Configuration Comparison"
    - Set initial window size (800x600)
    - Enable settings persistence with eframe
    - Run the eframe application
    - _Requirements: 10.1, 10.8, 11.6_

  - [ ] 12.3 Create secret masking for GUI in `qaren-gui/src/masking.rs`
    - Copy masking logic from CLI (same keywords and logic)
    - Implement `should_mask` and `mask_value` functions
    - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5, 8.6, 8.7_

- [ ] 13. Implement GUI file loading and comparison display
  - [ ] 13.1 Implement file drag-and-drop in `qaren-gui/src/app.rs`
    - Create two drop zones for file1 and file2
    - Handle file drop events from eframe
    - Display file paths when files are loaded
    - Enable "Compare" button when both files are loaded
    - _Requirements: 10.1, 10.2, 10.3_

  - [ ] 13.2 Implement comparison execution in GUI
    - Handle "Compare" button click
    - Read files using qaren-core parser
    - Execute semantic diff using qaren-core
    - Store diff result in application state
    - Display error dialog if comparison fails
    - _Requirements: 10.4_

  - [ ] 13.3 Implement comparison results display
    - Display additions in green color
    - Display deletions in red color
    - Display modifications in yellow color
    - Group results into sections (Missing in file2, Missing in file1, Modified, Identical)
    - Apply secret masking based on show_secrets setting
    - Display summary with counts
    - _Requirements: 10.5, 10.6, 10.7, 10.8, 18.1, 18.2, 18.3, 18.4_

- [ ] 14. Implement GUI settings panel
  - [ ] 14.1 Create settings panel UI in `qaren-gui/src/app.rs`
    - Add "Settings" button to main interface
    - Create collapsible settings panel
    - Add dropdown for common delimiters (=, :, space)
    - Add text input for custom delimiter
    - Add checkbox for "Strip Quotes"
    - Add checkbox for "Show Secrets"
    - _Requirements: 11.1, 11.2, 11.3, 11.4_

  - [ ] 14.2 Implement settings persistence
    - Use eframe's persistence feature to save settings
    - Load settings on application startup
    - Save settings when changed
    - _Requirements: 11.6, 11.7_

- [ ] 15. Implement GUI export functionality with bidirectional support
  - [ ] 15.1 Create export UI in `qaren-gui/src/app.rs`
    - Display "Export Options" dropdown button when differences exist
    - Add three dropdown options: "Export Source → Target", "Export Target → Source", "Export Both Directions"
    - Display count of missing keys for each direction in dropdown labels
    - _Requirements: 12.1, 12.2, 22.1, 22.2, 22.6_

  - [ ] 15.2 Implement export functionality
    - Handle "Export Source → Target" selection (missing_in_file2)
    - Handle "Export Target → Source" selection (missing_in_file1)
    - Handle "Export Both Directions" selection (create two files)
    - Open native file save dialog with suggested filename
    - Call qaren-core patch generator with appropriate direction
    - Display success notification with file path(s)
    - Display error dialog if export fails
    - _Requirements: 12.2, 12.3, 12.4, 12.5, 12.6, 22.3, 22.4, 22.5, 22.7_

  - [ ]* 15.3 Write GUI integration tests
    - Test file drag-and-drop functionality
    - Test comparison execution
    - Test settings persistence
    - Test export with source-to-target direction
    - Test export with target-to-source direction
    - Test export with bidirectional direction
    - Test error dialog display
    - Test success notification display
    - _Requirements: 10.1, 10.2, 10.3, 10.4, 11.6, 12.1, 12.2, 12.5, 12.6, 22.1, 22.2, 22.3, 22.4, 22.5_

- [ ] 16. Checkpoint - Ensure GUI tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 17. Add performance benchmarks
  - [ ]* 17.1 Create benchmark suite with criterion
    - Add `criterion` to dev-dependencies in workspace
    - Create `benches/` directory in qaren-core
    - Implement benchmark for parsing 1000-line file
    - Implement benchmark for parsing 10,000-line file
    - Implement benchmark for semantic diff with 1000 pairs
    - Implement benchmark for semantic diff with 10,000 pairs
    - Verify parsing 1000 lines completes in < 10ms
    - Verify parsing 10,000 lines completes in < 100ms
    - Verify diff 10,000 pairs completes in < 50ms
    - _Requirements: 13.1, 13.2, 13.3, 13.4_

- [ ] 18. Cross-platform testing and compatibility
  - [ ]* 18.1 Test on Linux
    - Build CLI and GUI for x86_64-unknown-linux-gnu
    - Run all tests on Linux
    - Verify colored terminal output
    - Verify GUI window management
    - Test file dialogs
    - _Requirements: 15.1, 15.4_

  - [ ]* 18.2 Test on macOS
    - Build CLI and GUI for x86_64-apple-darwin and aarch64-apple-darwin
    - Run all tests on macOS
    - Verify colored terminal output
    - Verify GUI window management
    - Test file dialogs
    - _Requirements: 15.2, 15.5_

  - [ ]* 18.3 Test on Windows
    - Build CLI and GUI for x86_64-pc-windows-msvc
    - Run all tests on Windows
    - Verify colored terminal output (Windows console API)
    - Verify GUI window management
    - Test file dialogs
    - Test CRLF line ending handling
    - _Requirements: 15.3, 15.6, 15.7_

- [ ] 19. Documentation and examples
  - [ ] 19.1 Write README.md
    - Add project overview and features
    - Add installation instructions for all platforms
    - Add CLI usage examples with screenshots
    - Add GUI usage guide with screenshots
    - Add comparison of literal vs semantic diff
    - Add examples of custom delimiters and quote stripping
    - Add examples of bidirectional patch generation
    - Add troubleshooting section
    - _Requirements: 16.1, 16.2, 16.3, 3.1, 5.1, 7.1, 21.1, 21.2, 21.3_

  - [ ] 19.2 Write API documentation for qaren-core
    - Add module-level documentation with examples
    - Add doc comments for all public types
    - Add doc comments for all public functions
    - Add usage examples in doc comments
    - Generate rustdoc and verify completeness
    - _Requirements: 19.1_

  - [ ] 19.3 Create example files for testing
    - Create example `.env` files with various formats
    - Create example AWS SSM output files
    - Create example PM2 output files
    - Create example files with URLs and complex values
    - Add examples to repository for users to test with
    - _Requirements: 17.1, 17.2, 17.3_

- [ ] 20. Build and release preparation
  - [ ] 20.1 Set up CI/CD pipeline
    - Create GitHub Actions workflow for automated testing
    - Add jobs for Linux, macOS, and Windows
    - Run all tests (unit, property, integration) on each platform
    - Run benchmarks and verify performance targets
    - Build release binaries for all platforms
    - _Requirements: 15.1, 15.2, 15.3, 15.4, 15.5, 15.6_

  - [ ] 20.2 Optimize binary size
    - Verify release profile settings (LTO, strip, codegen-units)
    - Measure binary sizes for CLI and GUI
    - Verify CLI binary < 10 MB
    - Verify GUI binary < 20 MB
    - Apply additional optimizations if needed
    - _Requirements: 19.4, 19.5_

  - [ ] 20.3 Create release artifacts
    - Build static binaries for Linux (musl target)
    - Build binaries for macOS (Intel and Apple Silicon)
    - Build binaries for Windows
    - Create compressed archives for each platform
    - Generate checksums for all artifacts
    - _Requirements: 19.1, 19.2, 19.3, 19.5, 19.6_

- [ ] 21. Final checkpoint - Comprehensive testing
  - Run all unit tests across all crates
  - Run all property-based tests (1000+ total iterations)
  - Run all integration tests
  - Run all benchmarks and verify performance targets
  - Test on all supported platforms
  - Verify binary sizes meet requirements
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Property-based tests validate universal correctness properties from the design document
- Unit tests validate specific examples and edge cases
- Integration tests validate end-to-end workflows
- Checkpoints ensure incremental validation at key milestones
- The implementation uses Rust throughout (as specified in the design document)
- Bidirectional patch generation is a new feature that requires careful testing
- Secret masking is implemented only in presentation layers (CLI and GUI), not in core library or patch files
