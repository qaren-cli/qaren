# Technical Design Document: Qaren Configuration Comparison Tool

## Overview

Qaren (قارن - "compare" in Arabic) is a high-performance, memory-safe configuration comparison tool built in Rust. The tool provides both CLI and GUI interfaces for comparing configuration files with advanced parsing capabilities, semantic diffing, secret masking, and patch generation.

### Design Philosophy

The design prioritizes:
- **Memory Safety**: Leveraging Rust's ownership system to guarantee no memory leaks, buffer overflows, or data races
- **Zero-Copy Parsing**: Using string slicing (`&str`) instead of allocations wherever possible
- **Offline-First**: All operations occur in RAM with no network access or temporary files
- **Separation of Concerns**: Core parsing/diffing logic completely decoupled from UI layers
- **Single Binary Distribution**: Static linking with no runtime dependencies

### Key Technical Challenges Addressed

1. **Safe Delimiter Splitting**: Parsing `URL=https://api.com?id=1&key=value` requires splitting only at the FIRST `=` to preserve the URL integrity
2. **Quote Stripping with Internal Quotes**: Handling `NAME='John "The Boss" Doe'` requires removing outer quotes while preserving internal ones
3. **O(n) Semantic Comparison**: Using HashMap for constant-time lookups to achieve linear complexity for large config files
4. **Cross-Platform GUI**: Using egui/eframe for immediate-mode GUI that compiles to native binaries on Linux, macOS, and Windows
5. **Secret Masking as Presentation Layer**: Masking occurs only during output rendering, not in the core data structures or patch files

## Architecture

### System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Qaren Application                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐              ┌──────────────┐            │
│  │  Qaren CLI   │              │  Qaren GUI   │            │
│  │   (clap)     │              │ (egui/eframe)│            │
│  └──────┬───────┘              └──────┬───────┘            │
│         │                              │                     │
│         └──────────────┬───────────────┘                     │
│                        │                                     │
│         ┌──────────────▼──────────────┐                     │
│         │   Presentation Layer        │                     │
│         │  (Secret Masking, Colors)   │                     │
│         └──────────────┬──────────────┘                     │
│                        │                                     │
│         ┌──────────────▼──────────────┐                     │
│         │      Qaren Core Engine      │                     │
│         │                              │                     │
│         │  ┌────────────────────────┐ │                     │
│         │  │   Parser Module        │ │                     │
│         │  │  - Safe Splitting      │ │                     │
│         │  │  - Quote Stripping     │ │                     │
│         │  │  - Comment Filtering   │ │                     │
│         │  └────────────────────────┘ │                     │
│         │                              │                     │
│         │  ┌────────────────────────┐ │                     │
│         │  │   Diff Engine          │ │                     │
│         │  │  - Literal Diff        │ │                     │
│         │  │  - Semantic KV Diff    │ │                     │
│         │  └────────────────────────┘ │                     │
│         │                              │                     │
│         │  ┌────────────────────────┐ │                     │
│         │  │   Patch Generator      │ │                     │
│         │  └────────────────────────┘ │                     │
│         └──────────────────────────────┘                     │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Project Structure (Cargo Workspace)

```
qaren/
├── Cargo.toml                 # Workspace root
├── qaren-core/                # Core library (no UI dependencies)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs             # Public API
│       ├── parser.rs          # Parsing logic
│       ├── diff.rs            # Diffing algorithms
│       ├── patch.rs           # Patch generation
│       ├── types.rs           # Core data structures
│       └── error.rs           # Error types
├── qaren-cli/                 # CLI application
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs            # CLI entry point
│       ├── commands.rs        # Command handlers
│       ├── output.rs          # Terminal output formatting
│       └── masking.rs         # Secret masking for CLI
└── qaren-gui/                 # GUI application
    ├── Cargo.toml
    └── src/
        ├── main.rs            # GUI entry point
        ├── app.rs             # Main application state
        ├── widgets.rs         # Custom UI widgets
        └── masking.rs         # Secret masking for GUI
```

### Module Responsibilities

**qaren-core** (Library Crate):
- Pure Rust library with no UI dependencies
- Exposes public API for parsing, diffing, and patch generation
- Contains all core business logic
- Can be used as a library by other Rust projects
- Zero dependencies on CLI or GUI frameworks

**qaren-cli** (Binary Crate):
- Depends on `qaren-core` and `clap`
- Handles command-line argument parsing
- Formats output for terminal (colors, exit codes)
- Implements secret masking for stdout

**qaren-gui** (Binary Crate):
- Depends on `qaren-core`, `egui`, and `eframe`
- Implements immediate-mode GUI
- Handles file drag-and-drop
- Implements secret masking for GUI display
- Persists user settings (delimiter, quote stripping)

## Components and Interfaces

### Core Data Structures

#### Configuration Representation

```rust
// types.rs

/// Represents a single key-value pair from a configuration file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvPair {
    /// The key (left side of delimiter)
    pub key: String,
    /// The value (right side of delimiter)
    pub value: String,
    /// Original line number in source file (for error reporting)
    pub line_number: usize,
}

/// Represents a parsed configuration file
#[derive(Debug, Clone)]
pub struct ConfigFile {
    /// Key-value pairs stored in a HashMap for O(1) lookup
    /// Key: configuration key, Value: (value, line_number)
    pub pairs: HashMap<String, (String, usize)>,
    /// Original file path (for error messages)
    pub file_path: PathBuf,
}

/// Parsing configuration options
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// Delimiter character (default: '=')
    pub delimiter: char,
    /// Whether to strip surrounding quotes from keys and values
    pub strip_quotes: bool,
    /// Comment prefixes to ignore (default: ["#", "//"])
    pub comment_prefixes: Vec<String>,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            delimiter: '=',
            strip_quotes: false,
            comment_prefixes: vec!["#".to_string(), "//".to_string()],
        }
    }
}
```

#### Diff Results

```rust
// types.rs

/// Represents the result of comparing two configuration files
#[derive(Debug, Clone)]
pub struct DiffResult {
    /// Keys present in file1 but missing in file2
    pub missing_in_file2: Vec<KvPair>,
    /// Keys present in file2 but missing in file1
    pub missing_in_file1: Vec<KvPair>,
    /// Keys present in both files with different values
    pub modified: Vec<ModifiedPair>,
    /// Keys present in both files with identical values
    pub identical: Vec<String>,
}

/// Represents a key-value pair that differs between files
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifiedPair {
    pub key: String,
    pub value_file1: String,
    pub value_file2: String,
    pub line_number_file1: usize,
    pub line_number_file2: usize,
}

impl DiffResult {
    /// Returns true if files are identical
    pub fn is_identical(&self) -> bool {
        self.missing_in_file1.is_empty()
            && self.missing_in_file2.is_empty()
            && self.modified.is_empty()
    }

    /// Returns total count of differences
    pub fn difference_count(&self) -> usize {
        self.missing_in_file1.len()
            + self.missing_in_file2.len()
            + self.modified.len()
    }
}

/// Direction for patch generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatchDirection {
    /// Generate patch with keys from file1 missing in file2 (default)
    SourceToTarget,
    /// Generate patch with keys from file2 missing in file1
    TargetToSource,
    /// Generate both patches
    Bidirectional,
}

impl Default for PatchDirection {
    fn default() -> Self {
        Self::SourceToTarget
    }
}

impl std::str::FromStr for PatchDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "source-to-target" | "source" | "s2t" => Ok(Self::SourceToTarget),
            "target-to-source" | "target" | "t2s" => Ok(Self::TargetToSource),
            "bidirectional" | "both" | "bi" => Ok(Self::Bidirectional),
            _ => Err(format!(
                "Invalid direction '{}'. Valid options: source-to-target, target-to-source, bidirectional",
                s
            )),
        }
    }
}
```

#### Literal Diff Results

```rust
// types.rs

/// Represents a line-by-line diff result
#[derive(Debug, Clone)]
pub struct LiteralDiffResult {
    /// Lines added in file2
    pub additions: Vec<DiffLine>,
    /// Lines removed from file1
    pub deletions: Vec<DiffLine>,
    /// Lines modified between files
    pub modifications: Vec<(DiffLine, DiffLine)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffLine {
    pub content: String,
    pub line_number: usize,
}
```

### Parser Module

#### Core Parsing Algorithm

```rust
// parser.rs

use crate::types::{ConfigFile, KvPair, ParseOptions};
use crate::error::QarenResult;
use std::collections::HashMap;
use std::path::Path;

/// Parse a configuration file with given options
pub fn parse_file(
    file_path: &Path,
    options: &ParseOptions,
) -> QarenResult<ConfigFile> {
    let content = std::fs::read_to_string(file_path)?;
    parse_content(&content, file_path, options)
}

/// Parse configuration content from a string
pub fn parse_content(
    content: &str,
    file_path: &Path,
    options: &ParseOptions,
) -> QarenResult<ConfigFile> {
    let mut pairs = HashMap::new();

    for (line_number, line) in content.lines().enumerate() {
        let line_number = line_number + 1; // 1-indexed for user display

        // Skip empty lines and comments
        if should_skip_line(line, options) {
            continue;
        }

        // Parse key-value pair
        if let Some((key, value)) = parse_line(line, options)? {
            pairs.insert(key, (value, line_number));
        }
    }

    Ok(ConfigFile {
        pairs,
        file_path: file_path.to_path_buf(),
    })
}

/// Check if a line should be skipped (empty or comment)
fn should_skip_line(line: &str, options: &ParseOptions) -> bool {
    let trimmed = line.trim();

    // Empty line
    if trimmed.is_empty() {
        return true;
    }

    // Comment line
    for prefix in &options.comment_prefixes {
        if trimmed.starts_with(prefix) {
            return true;
        }
    }

    false
}

/// Parse a single line into a key-value pair
/// Returns None if line has no delimiter or is malformed
fn parse_line(
    line: &str,
    options: &ParseOptions,
) -> QarenResult<Option<(String, String)>> {
    // Use split_once to split at FIRST delimiter only
    // This is critical for URLs: "URL=https://api.com?id=1&key=value"
    let Some((key_raw, value_raw)) = line.split_once(options.delimiter) else {
        // No delimiter found - skip this line
        return Ok(None);
    };

    // Process key and value
    let key = process_token(key_raw, options);
    let value = process_token(value_raw, options);

    // Skip if key is empty after processing
    if key.is_empty() {
        return Ok(None);
    }

    Ok(Some((key, value)))
}

/// Process a token (key or value) by trimming and optionally stripping quotes
fn process_token(token: &str, options: &ParseOptions) -> String {
    let trimmed = token.trim();

    if options.strip_quotes {
        strip_surrounding_quotes(trimmed)
    } else {
        trimmed.to_string()
    }
}

/// Strip surrounding quotes from a string
/// Only removes quotes that surround the ENTIRE string
/// Preserves internal quotes: 'John "The Boss" Doe' -> John "The Boss" Doe
fn strip_surrounding_quotes(s: &str) -> String {
    let len = s.len();

    if len < 2 {
        return s.to_string();
    }

    let first = s.chars().next().unwrap();
    let last = s.chars().last().unwrap();

    // Check if surrounded by matching quotes
    if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
        // Remove first and last character
        s[1..len - 1].to_string()
    } else {
        s.to_string()
    }
}
```

**Algorithm Complexity**:
- **Time**: O(n) where n is the number of lines in the file
- **Space**: O(m) where m is the number of key-value pairs
- **Key Optimization**: Using `split_once` instead of `split` avoids allocating a Vec and stops at the first delimiter

**Why `split_once` is Critical**:
```rust
// WRONG: Using split() would break URLs
let parts: Vec<&str> = "URL=https://api.com?id=1&key=value".split('=').collect();
// parts = ["URL", "https://api.com?id", "1&key", "value"] ❌

// CORRECT: Using split_once() preserves the URL
let (key, value) = "URL=https://api.com?id=1&key=value".split_once('=').unwrap();
// key = "URL", value = "https://api.com?id=1&key=value" ✅
```

### Diff Engine

#### Semantic Key-Value Diffing

```rust
// diff.rs

use crate::types::{ConfigFile, DiffResult, KvPair, ModifiedPair};

/// Perform semantic key-value comparison between two configuration files
/// Complexity: O(n + m) where n and m are the number of pairs in each file
pub fn semantic_diff(file1: &ConfigFile, file2: &ConfigFile) -> DiffResult {
    let mut missing_in_file2 = Vec::new();
    let mut missing_in_file1 = Vec::new();
    let mut modified = Vec::new();
    let mut identical = Vec::new();

    // Check all keys in file1
    for (key, (value1, line_num1)) in &file1.pairs {
        match file2.pairs.get(key) {
            Some((value2, line_num2)) => {
                if value1 == value2 {
                    // Identical
                    identical.push(key.clone());
                } else {
                    // Modified
                    modified.push(ModifiedPair {
                        key: key.clone(),
                        value_file1: value1.clone(),
                        value_file2: value2.clone(),
                        line_number_file1: *line_num1,
                        line_number_file2: *line_num2,
                    });
                }
            }
            None => {
                // Missing in file2
                missing_in_file2.push(KvPair {
                    key: key.clone(),
                    value: value1.clone(),
                    line_number: *line_num1,
                });
            }
        }
    }

    // Check for keys in file2 that are not in file1
    for (key, (value2, line_num2)) in &file2.pairs {
        if !file1.pairs.contains_key(key) {
            missing_in_file1.push(KvPair {
                key: key.clone(),
                value: value2.clone(),
                line_number: *line_num2,
            });
        }
    }

    DiffResult {
        missing_in_file2,
        missing_in_file1,
        modified,
        identical,
    }
}
```

**Algorithm Analysis**:
- **Time Complexity**: O(n + m) where n and m are the number of pairs
  - First loop: O(n) iterations with O(1) HashMap lookups
  - Second loop: O(m) iterations with O(1) HashMap lookups
- **Space Complexity**: O(n + m) for storing results
- **Why HashMap**: Provides O(1) average-case lookup vs O(log n) for BTreeMap

#### Literal Line-by-Line Diffing

```rust
// diff.rs

use crate::types::{LiteralDiffResult, DiffLine};
use similar::{ChangeTag, TextDiff};

/// Perform literal line-by-line comparison
/// Uses the 'similar' crate for Myers diff algorithm
pub fn literal_diff(content1: &str, content2: &str) -> LiteralDiffResult {
    let diff = TextDiff::from_lines(content1, content2);

    let mut additions = Vec::new();
    let mut deletions = Vec::new();
    let mut modifications = Vec::new();

    let mut old_line_num = 1;
    let mut new_line_num = 1;

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                deletions.push(DiffLine {
                    content: change.value().to_string(),
                    line_number: old_line_num,
                });
                old_line_num += 1;
            }
            ChangeTag::Insert => {
                additions.push(DiffLine {
                    content: change.value().to_string(),
                    line_number: new_line_num,
                });
                new_line_num += 1;
            }
            ChangeTag::Equal => {
                old_line_num += 1;
                new_line_num += 1;
            }
        }
    }

    LiteralDiffResult {
        additions,
        deletions,
        modifications,
    }
}
```

### Patch Generator

```rust
// patch.rs

use crate::types::{DiffResult, ParseOptions, PatchDirection};
use crate::error::QarenResult;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Generate patch file(s) based on the specified direction
pub fn generate_patch(
    diff_result: &DiffResult,
    output_path: &Path,
    options: &ParseOptions,
    direction: PatchDirection,
) -> QarenResult<Vec<PathBuf>> {
    match direction {
        PatchDirection::SourceToTarget => {
            generate_single_patch(
                &diff_result.missing_in_file2,
                output_path,
                options,
            )?;
            Ok(vec![output_path.to_path_buf()])
        }
        PatchDirection::TargetToSource => {
            generate_single_patch(
                &diff_result.missing_in_file1,
                output_path,
                options,
            )?;
            Ok(vec![output_path.to_path_buf()])
        }
        PatchDirection::Bidirectional => {
            let mut created_files = Vec::new();

            // Generate source-to-target patch
            let s2t_path = append_suffix(output_path, "source-to-target");
            generate_single_patch(
                &diff_result.missing_in_file2,
                &s2t_path,
                options,
            )?;
            created_files.push(s2t_path);

            // Generate target-to-source patch
            let t2s_path = append_suffix(output_path, "target-to-source");
            generate_single_patch(
                &diff_result.missing_in_file1,
                &t2s_path,
                options,
            )?;
            created_files.push(t2s_path);

            Ok(created_files)
        }
    }
}

/// Generate a single patch file containing the specified missing keys
fn generate_single_patch(
    missing_pairs: &[KvPair],
    output_path: &Path,
    options: &ParseOptions,
) -> QarenResult<()> {
    let mut file = File::create(output_path)?;

    // Write missing keys in original format
    for pair in missing_pairs {
        let line = format_kv_pair(&pair.key, &pair.value, options);
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

/// Append a suffix to a file path before the extension
/// Example: "output.env" + "source-to-target" -> "output.source-to-target.env"
fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("env");
    
    let new_filename = format!("{}.{}.{}", stem, suffix, extension);
    
    if let Some(parent) = path.parent() {
        parent.join(new_filename)
    } else {
        PathBuf::from(new_filename)
    }
}

/// Format a key-value pair using the specified delimiter
fn format_kv_pair(key: &str, value: &str, options: &ParseOptions) -> String {
    format!("{}{}{}", key, options.delimiter, value)
}
```

### Secret Masking Layer

```rust
// qaren-cli/src/masking.rs

/// Keywords that trigger secret masking (case-insensitive)
const SECRET_KEYWORDS: &[&str] = &["key", "password", "secret", "token", "auth"];

/// Check if a key should have its value masked
pub fn should_mask(key: &str) -> bool {
    let key_lower = key.to_lowercase();
    SECRET_KEYWORDS.iter().any(|keyword| key_lower.contains(keyword))
}

/// Mask a value if needed
pub fn mask_value(key: &str, value: &str, show_secrets: bool) -> String {
    if show_secrets || !should_mask(key) {
        value.to_string()
    } else {
        "***MASKED***".to_string()
    }
}
```

**Design Decision**: Secret masking is implemented in the presentation layer (CLI and GUI modules) rather than in the core library. This ensures:
- Patch files contain actual values (not masked)
- Core library remains pure and testable
- Different UIs can implement different masking strategies

## Data Models

### Error Handling

```rust
// error.rs

use thiserror::Error;
use std::io;
use std::path::PathBuf;

/// Result type alias for Qaren operations
pub type QarenResult<T> = Result<T, QarenError>;

/// Comprehensive error type for all Qaren operations
#[derive(Error, Debug)]
pub enum QarenError {
    #[error("Failed to read file '{path}': {source}")]
    FileRead {
        path: PathBuf,
        source: io::Error,
    },

    #[error("Failed to write file '{path}': {source}")]
    FileWrite {
        path: PathBuf,
        source: io::Error,
    },

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Permission denied for file: {0}")]
    PermissionDenied(PathBuf),

    #[error("Invalid UTF-8 encoding in file '{path}' at line {line}")]
    InvalidEncoding {
        path: PathBuf,
        line: usize,
    },

    #[error("Invalid delimiter: '{0}' (must be a single character)")]
    InvalidDelimiter(String),

    #[error("Parsing failed at line {line} in file '{path}': {reason}")]
    ParseError {
        path: PathBuf,
        line: usize,
        reason: String,
    },

    #[error("Invalid command-line arguments: {0}")]
    InvalidArguments(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

impl QarenError {
    /// Convert io::Error to QarenError with context
    pub fn from_io_with_path(err: io::Error, path: PathBuf) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => QarenError::FileNotFound(path),
            io::ErrorKind::PermissionDenied => QarenError::PermissionDenied(path),
            _ => QarenError::FileRead { path, source: err },
        }
    }
}
```

**Why `thiserror`**:
- Reduces boilerplate for custom error types
- Automatic `Display` and `Error` trait implementations
- Supports error chaining with `#[from]` attribute
- Better than `anyhow` for library code (provides typed errors for API consumers)

### CLI Data Model

```rust
// qaren-cli/src/commands.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "qaren")]
#[command(about = "Configuration comparison tool", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform literal line-by-line comparison
    Diff {
        /// First file to compare
        file1: PathBuf,
        /// Second file to compare
        file2: PathBuf,
    },

    /// Perform semantic key-value pair comparison
    Kvp {
        /// First file to compare
        file1: PathBuf,
        /// Second file to compare
        file2: PathBuf,

        /// Custom delimiter (default: '=')
        #[arg(short, long, default_value = "=")]
        delimiter: String,

        /// Strip surrounding quotes from keys and values
        #[arg(long)]
        strip_quotes: bool,

        /// Generate a patch file with missing keys
        #[arg(long)]
        generate_missing: Option<PathBuf>,

        /// Patch direction: source-to-target (default), target-to-source, or bidirectional
        /// Only valid when --generate-missing is specified
        #[arg(long, default_value = "source-to-target")]
        direction: String,

        /// Show secret values instead of masking them
        #[arg(long)]
        show_secrets: bool,
    },
}
```

### GUI Data Model

```rust
// qaren-gui/src/app.rs

use qaren_core::{ParseOptions, DiffResult, PatchDirection};
use std::path::PathBuf;

/// Main application state
pub struct QarenApp {
    /// Path to first file
    pub file1_path: Option<PathBuf>,
    /// Path to second file
    pub file2_path: Option<PathBuf>,
    /// Comparison result
    pub diff_result: Option<DiffResult>,
    /// User settings
    pub settings: AppSettings,
    /// UI state
    pub ui_state: UiState,
}

/// User-configurable settings
#[derive(Clone)]
pub struct AppSettings {
    pub delimiter: char,
    pub strip_quotes: bool,
    pub show_secrets: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            delimiter: '=',
            strip_quotes: false,
            show_secrets: false,
        }
    }
}

/// UI state
pub struct UiState {
    pub show_settings: bool,
    pub custom_delimiter_input: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub show_export_menu: bool,
}
```

## Dependencies

### Core Dependencies (qaren-core)

```toml
[dependencies]
# Error handling
thiserror = "2.0"

# Literal diffing algorithm (Myers diff)
similar = "2.6"
```

### CLI Dependencies (qaren-cli)

```toml
[dependencies]
qaren-core = { path = "../qaren-core" }

# Command-line argument parsing
clap = { version = "4.5", features = ["derive"] }

# Terminal colors
colored = "2.1"
```

### GUI Dependencies (qaren-gui)

```toml
[dependencies]
qaren-core = { path = "../qaren-core" }

# GUI framework
egui = "0.29"
eframe = { version = "0.29", default-features = false, features = [
    "default_fonts",
    "glow",
    "persistence",
] }

# Settings persistence
serde = { version = "1.0", features = ["derive"] }
```

**Dependency Rationale**:

- **thiserror**: Ergonomic custom error types for library code
- **similar**: Battle-tested Myers diff algorithm for literal comparison
- **clap**: Industry-standard CLI parsing with derive macros
- **colored**: Simple terminal color output
- **egui/eframe**: Immediate-mode GUI that compiles to native binaries with no runtime dependencies
- **serde**: Settings serialization for GUI persistence

### Build Configuration

```toml
# Cargo.toml (workspace root)

[profile.release]
opt-level = 3           # Maximum optimization
lto = true              # Link-time optimization for smaller binary
codegen-units = 1       # Better optimization (slower compile)
strip = true            # Strip symbols for smaller binary
panic = "abort"         # Smaller binary, no unwinding
```

**Expected Binary Sizes**:
- CLI: ~3-5 MB (static binary)
- GUI: ~8-12 MB (includes egui rendering)


## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Comparison Symmetry (Inverse Relationship)

*For any* two valid configuration files A and B, when comparing A to B and then B to A, the difference reports SHALL be inverse: keys missing in B when comparing A→B SHALL equal keys missing in A when comparing B→A, and modified pairs SHALL have their file1/file2 values swapped.

**Validates: Requirements 2.7**

### Property 2: First-Delimiter-Only Splitting

*For any* line containing N delimiters where N ≥ 1, the parser SHALL split at the first delimiter only, producing exactly one key-value pair where the value contains exactly N-1 delimiter characters.

**Validates: Requirements 4.5**

### Property 3: Quote Preservation Without Stripping

*For any* string containing quotation marks, when parsing with `strip_quotes=false`, the parser SHALL preserve all quotation marks exactly as they appear in the input (round-trip identity for quotes).

**Validates: Requirements 5.6**

### Property 4: Inline Comment Removal

*For any* valid key-value pair followed by a comment character and comment text, the parser SHALL extract only the key-value pair and exclude all comment content from the parsed value.

**Validates: Requirements 6.4**

### Property 5: Patch Completeness and Correctness

*For any* two configuration files A and B, the generated patch file SHALL contain exactly the set of key-value pairs present in A but missing in B when using `source-to-target` direction, or exactly the set present in B but missing in A when using `target-to-source` direction, with no additional or missing entries.

**Validates: Requirements 7.2, 21.1, 21.2**

### Property 6: Patch Formatting Preservation (Round-Trip)

*For any* configuration file A and any file B where A has keys missing in B, generating a patch from the diff and then parsing that patch SHALL produce key-value pairs with values identical to the original values in A for those missing keys.

**Validates: Requirements 7.3**

### Property 10: Bidirectional Patch Symmetry

*For any* two configuration files A and B, when generating bidirectional patches, the source-to-target patch SHALL contain exactly the keys in A missing from B, and the target-to-source patch SHALL contain exactly the keys in B missing from A, with no overlap between the two patch files.

**Validates: Requirements 21.3**

### Property 7: Linear Time Complexity

*For any* two configuration files with n and m key-value pairs respectively, the semantic diff operation SHALL complete in O(n + m) time, demonstrating linear scaling when measured across varying input sizes (100, 1000, 10000 pairs).

**Validates: Requirements 13.4**

### Property 8: Empty Value Handling

*For any* key followed by a delimiter but no value (e.g., `KEY=`), the parser SHALL produce a key-value pair with the key and an empty string as the value.

**Validates: Requirements 20.2**

### Property 9: Parser Robustness and Error Recovery

*For any* configuration file containing a mix of valid and malformed lines (lines without delimiters, lines with only delimiters, lines with invalid syntax), the parser SHALL successfully extract all valid key-value pairs and continue processing without terminating, regardless of the number or position of malformed lines.

**Validates: Requirements 20.1, 20.7**

## Error Handling

### Error Categories

The application handles errors in three categories:

1. **File System Errors**
   - File not found
   - Permission denied
   - Invalid path
   - Disk full (during patch generation)

2. **Parsing Errors**
   - Invalid UTF-8 encoding
   - Malformed lines (handled gracefully, not fatal)
   - Invalid delimiter specification

3. **User Input Errors**
   - Invalid command-line arguments
   - Invalid delimiter (not a single character)
   - Invalid output path for patch generation

### Error Handling Strategy

**Fail Fast for Fatal Errors**:
- File system errors (file not found, permission denied) immediately return an error
- Invalid UTF-8 encoding stops parsing and returns an error with line number
- Invalid command-line arguments display help and exit with code 2

**Graceful Degradation for Non-Fatal Errors**:
- Malformed lines (no delimiter, empty key) are skipped with a warning
- Parser continues processing remaining lines
- Final result includes count of skipped lines

**User-Friendly Error Messages**:
```rust
// Example error messages

// File not found
Error: Failed to read file '/path/to/config.env': No such file or directory

// Permission denied
Error: Permission denied for file '/etc/secure/config.env'
  Hint: Try running with appropriate permissions or specify a different file

// Invalid UTF-8
Error: Invalid UTF-8 encoding in file 'config.env' at line 42
  The file contains non-UTF-8 bytes that cannot be processed

// Invalid delimiter
Error: Invalid delimiter: 'abc' (must be a single character)
  Examples: '=', ':', ' '

// Parsing warning (non-fatal)
Warning: Skipped 3 malformed lines in 'config.env'
  Lines without delimiters are ignored during parsing
```

### Exit Codes

The CLI uses standard exit codes:

- **0**: Success (files are identical or operation completed successfully)
- **1**: Differences found (files are not identical)
- **2**: Error occurred (file not found, permission denied, invalid arguments, etc.)

```rust
// qaren-cli/src/main.rs

fn main() {
    let exit_code = match run() {
        Ok(diff_result) => {
            if diff_result.is_identical() {
                0 // Success - files identical
            } else {
                1 // Differences found
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            2 // Error occurred
        }
    };

    std::process::exit(exit_code);
}
```

### Error Context Propagation

Using `thiserror`, errors automatically include context:

```rust
// Example: File read error with context
match std::fs::read_to_string(&path) {
    Ok(content) => content,
    Err(e) => return Err(QarenError::from_io_with_path(e, path)),
}

// Results in error message:
// "Failed to read file '/path/to/file': Permission denied"
```

## Testing Strategy

### Dual Testing Approach

The testing strategy combines **property-based testing** for universal correctness guarantees with **example-based unit tests** for specific scenarios and edge cases.

#### Property-Based Testing

Property-based tests validate universal properties across randomly generated inputs. Each property test will:
- Run a minimum of **100 iterations** with randomized inputs
- Use the **quickcheck** crate for Rust property-based testing
- Tag each test with a comment referencing the design property
- Test the core parsing and diffing logic in `qaren-core`

**Property Test Configuration**:
```rust
// Example property test structure

#[cfg(test)]
mod property_tests {
    use quickcheck::{quickcheck, TestResult};
    use quickcheck_macros::quickcheck;

    // Feature: qaren-config-comparison-tool, Property 2: First-Delimiter-Only Splitting
    #[quickcheck]
    fn prop_first_delimiter_only(key: String, values: Vec<String>) -> TestResult {
        // Test implementation
    }
}
```

**Property Tests to Implement**:

1. **Comparison Symmetry** (Property 1)
   - Generate random configuration pairs
   - Compare A→B and B→A
   - Verify inverse relationship

2. **First-Delimiter-Only Splitting** (Property 2)
   - Generate lines with 1-10 delimiters
   - Parse and verify value contains N-1 delimiters

3. **Quote Preservation** (Property 3)
   - Generate random strings with quotes
   - Parse without stripping
   - Verify round-trip identity

4. **Inline Comment Removal** (Property 4)
   - Generate KV pairs with inline comments
   - Verify comments are excluded from values

5. **Patch Completeness** (Property 5)
   - Generate random file pairs
   - Create patch with each direction
   - Verify patch contains exactly missing keys for that direction

6. **Patch Round-Trip** (Property 6)
   - Generate configs, create patch, parse patch
   - Verify values match original

7. **Linear Time Complexity** (Property 7)
   - Generate configs of sizes 100, 1000, 10000
   - Measure comparison time
   - Verify linear scaling (time ratio ≈ size ratio)

8. **Empty Value Handling** (Property 8)
   - Generate keys with delimiter but no value
   - Verify value is empty string

9. **Parser Robustness** (Property 9)
   - Generate files with random mix of valid/invalid lines
   - Verify all valid lines are parsed
   - Verify parser doesn't crash

10. **Bidirectional Patch Symmetry** (Property 10)
    - Generate random file pairs
    - Create bidirectional patches
    - Verify no overlap between patches
    - Verify union of patches equals all differences

#### Example-Based Unit Tests

Unit tests validate specific scenarios, edge cases, and integration points:

**Core Parsing Tests**:
- Empty file handling
- File with only comments
- File with only whitespace
- Quote stripping with internal quotes: `'John "The Boss" Doe'`
- URL preservation: `URL=https://api.com?id=1&key=value`
- PM2 format: `"DATABASE_URL":"postgres://host:5432/db"`
- Shell export format: `export KEY=value`

**Diff Engine Tests**:
- Identical files
- Completely different files
- Files with only additions
- Files with only deletions
- Files with only modifications
- Mixed differences

**Patch Generation Tests**:
- Empty patch (no missing keys)
- Patch with multiple missing keys
- Patch file creation failure (invalid path)
- Source-to-target direction
- Target-to-source direction
- Bidirectional patch generation
- Verify correct file naming for bidirectional patches

**Secret Masking Tests**:
- Keys containing "key", "password", "secret", "token", "auth"
- Case-insensitive matching
- Masking disabled with `--show-secrets`

**Error Handling Tests**:
- File not found
- Permission denied
- Invalid UTF-8 encoding
- Invalid delimiter (multi-character)
- Invalid command-line arguments

#### Integration Tests

Integration tests validate end-to-end workflows:

**CLI Integration Tests**:
```rust
// Test CLI commands with real files
#[test]
fn test_cli_kvp_comparison() {
    // Create temporary test files
    // Run CLI command
    // Verify output and exit code
}
```

**GUI Integration Tests**:
- File drag-and-drop
- Settings persistence
- Export functionality (all three directions)
- Export menu display with key counts
- Error dialog display

#### Performance Benchmarks

Using **criterion** crate for performance benchmarking:

```rust
// Benchmark parsing performance
fn bench_parse_large_file(c: &mut Criterion) {
    let content = generate_config_file(10_000); // 10k lines
    c.bench_function("parse 10k lines", |b| {
        b.iter(|| parse_content(&content, &ParseOptions::default()))
    });
}

// Benchmark diff performance
fn bench_semantic_diff(c: &mut Criterion) {
    let file1 = generate_config_file(10_000);
    let file2 = generate_config_file(10_000);
    c.bench_function("diff 10k pairs", |b| {
        b.iter(|| semantic_diff(&file1, &file2))
    });
}
```

**Performance Targets**:
- Parse 1000 lines: < 10ms
- Parse 10,000 lines: < 100ms
- Diff 10,000 pairs: < 50ms
- Memory usage: < 50 MB for 10,000 pairs

### Test Coverage Goals

- **Line Coverage**: > 90% for `qaren-core`
- **Branch Coverage**: > 85% for `qaren-core`
- **Property Tests**: 10 properties with 100+ iterations each
- **Unit Tests**: 60+ example-based tests
- **Integration Tests**: 25+ end-to-end scenarios

### Testing Dependencies

```toml
[dev-dependencies]
# Property-based testing
quickcheck = "1.0"
quickcheck_macros = "1.0"

# Performance benchmarking
criterion = "0.5"

# Test utilities
tempfile = "3.8"  # Temporary files for integration tests
assert_cmd = "2.0"  # CLI testing
predicates = "3.0"  # Assertion helpers
```

## Security Considerations

### Memory Safety

Rust's ownership system provides compile-time guarantees:
- **No buffer overflows**: Array bounds are checked at runtime
- **No use-after-free**: Ownership prevents dangling pointers
- **No data races**: Borrow checker ensures thread safety
- **No null pointer dereferences**: `Option<T>` makes nullability explicit

### Offline-First Architecture

- **No network access**: Application never initiates network connections
- **No telemetry**: No data is sent to external servers
- **No temporary files**: All processing occurs in RAM
- **No disk writes** except:
  - Explicit patch file generation (user-initiated)
  - GUI settings persistence (local only)

### Secret Masking

**Masking Strategy**:
- Keyword-based detection (case-insensitive)
- Applied only in presentation layer (CLI output, GUI display)
- **Not applied** to:
  - Patch files (contain actual values for remediation)
  - Internal data structures
  - Comparison logic

**Masking Keywords**:
- `key` (API_KEY, SECRET_KEY, etc.)
- `password` (PASSWORD, DB_PASSWORD, etc.)
- `secret` (CLIENT_SECRET, etc.)
- `token` (AUTH_TOKEN, ACCESS_TOKEN, etc.)
- `auth` (AUTH_HEADER, etc.)

**Bypass Mechanism**:
- CLI: `--show-secrets` flag
- GUI: Checkbox in settings panel

### Input Validation

**File Path Validation**:
- Reject paths with null bytes
- Reject paths outside allowed directories (if sandboxing is enabled)
- Validate file extensions (optional, configurable)

**Delimiter Validation**:
- Must be exactly one character
- Reject control characters
- Reject newline characters

**Content Validation**:
- Validate UTF-8 encoding
- Handle malformed lines gracefully
- Limit line length to prevent memory exhaustion (configurable, default 10KB per line)

### Denial of Service Prevention

**Resource Limits**:
```rust
// parser.rs

const MAX_LINE_LENGTH: usize = 10 * 1024; // 10 KB
const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100 MB

pub fn parse_file(path: &Path, options: &ParseOptions) -> QarenResult<ConfigFile> {
    let metadata = std::fs::metadata(path)?;
    
    if metadata.len() > MAX_FILE_SIZE as u64 {
        return Err(QarenError::FileTooLarge {
            path: path.to_path_buf(),
            size: metadata.len(),
            max_size: MAX_FILE_SIZE as u64,
        });
    }
    
    // Continue with parsing...
}
```

**Memory Management**:
- Use streaming parsing for large files (future enhancement)
- Limit HashMap size to prevent memory exhaustion
- Use `String` instead of `&str` only when necessary

## Cross-Platform Considerations

### Platform-Specific Handling

**Line Endings**:
```rust
// Handle both LF (Unix) and CRLF (Windows) transparently
// Rust's .lines() iterator handles this automatically
for line in content.lines() {
    // Works on all platforms
}
```

**File Paths**:
```rust
// Use PathBuf for platform-independent path handling
use std::path::PathBuf;

// Automatically uses correct separator (/ on Unix, \ on Windows)
let path = PathBuf::from("config").join("app.env");
```

**Terminal Colors**:
```rust
// Use 'colored' crate which handles Windows console API
use colored::*;

println!("{}", "Added".green());  // Works on Windows, macOS, Linux
```

### GUI Platform Integration

**Native Window Decorations**:
- eframe provides native window decorations on all platforms
- Respects platform-specific window management (minimize, maximize, close)

**File Dialogs**:
```rust
// Use rfd (Rusty File Dialogs) for native file pickers
use rfd::FileDialog;

let file = FileDialog::new()
    .add_filter("Environment Files", &["env", "conf"])
    .pick_file();
```

**Settings Persistence**:
```rust
// eframe handles platform-specific config directories
// Linux: ~/.config/qaren/
// macOS: ~/Library/Application Support/qaren/
// Windows: %APPDATA%\qaren\
```

### Build Targets

**Supported Platforms**:
- Linux x86_64 (GNU and musl)
- macOS x86_64 (Intel)
- macOS aarch64 (Apple Silicon)
- Windows x86_64 (MSVC)

**Build Commands**:
```bash
# Linux (GNU)
cargo build --release --target x86_64-unknown-linux-gnu

# Linux (musl - fully static)
cargo build --release --target x86_64-unknown-linux-musl

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-msvc
```

## Implementation Roadmap

### Phase 1: Core Library (Week 1-2)

**Milestone 1.1: Parser Implementation**
- [ ] Implement `ParseOptions` struct
- [ ] Implement `parse_file` and `parse_content` functions
- [ ] Implement safe delimiter splitting with `split_once`
- [ ] Implement quote stripping algorithm
- [ ] Implement comment and empty line filtering
- [ ] Write unit tests for parser
- [ ] Write property tests for parser

**Milestone 1.2: Diff Engine**
- [ ] Implement `semantic_diff` function with HashMap
- [ ] Implement `literal_diff` function using `similar` crate
- [ ] Write unit tests for diff engine
- [ ] Write property tests for diff engine

**Milestone 1.3: Patch Generator**
- [ ] Implement `generate_patch` function
- [ ] Write unit tests for patch generation
- [ ] Write property tests for patch generation

**Milestone 1.4: Error Handling**
- [ ] Define `QarenError` enum with `thiserror`
- [ ] Implement error context propagation
- [ ] Write error handling tests

### Phase 2: CLI Application (Week 3)

**Milestone 2.1: CLI Structure**
- [ ] Set up `clap` command structure
- [ ] Implement `diff` command
- [ ] Implement `kvp` command
- [ ] Implement argument validation

**Milestone 2.2: Output Formatting**
- [ ] Implement colored terminal output
- [ ] Implement secret masking for CLI
- [ ] Implement exit code handling
- [ ] Write CLI integration tests

**Milestone 2.3: CLI Polish**
- [ ] Add progress indicators for large files
- [ ] Add verbose mode for debugging
- [ ] Improve error messages
- [ ] Write CLI documentation

### Phase 3: GUI Application (Week 4-5)

**Milestone 3.1: GUI Foundation**
- [ ] Set up eframe application structure
- [ ] Implement main application state
- [ ] Implement file drag-and-drop
- [ ] Implement basic comparison display

**Milestone 3.2: Settings Panel**
- [ ] Implement settings UI
- [ ] Implement delimiter selection
- [ ] Implement quote stripping toggle
- [ ] Implement settings persistence

**Milestone 3.3: Advanced Features**
- [ ] Implement export functionality
- [ ] Implement secret masking for GUI
- [ ] Implement error dialogs
- [ ] Implement success notifications

**Milestone 3.4: GUI Polish**
- [ ] Improve visual design
- [ ] Add keyboard shortcuts
- [ ] Add tooltips and help text
- [ ] Write GUI integration tests

### Phase 4: Testing and Optimization (Week 6)

**Milestone 4.1: Comprehensive Testing**
- [ ] Achieve 90%+ line coverage
- [ ] Run all property tests (900+ iterations total)
- [ ] Run all integration tests
- [ ] Fix any discovered bugs

**Milestone 4.2: Performance Optimization**
- [ ] Run criterion benchmarks
- [ ] Optimize hot paths if needed
- [ ] Verify O(n) complexity
- [ ] Verify memory usage targets

**Milestone 4.3: Cross-Platform Testing**
- [ ] Test on Linux
- [ ] Test on macOS (Intel and Apple Silicon)
- [ ] Test on Windows
- [ ] Fix platform-specific issues

### Phase 5: Documentation and Release (Week 7)

**Milestone 5.1: Documentation**
- [ ] Write README with examples
- [ ] Write CLI usage guide
- [ ] Write GUI user guide
- [ ] Write API documentation for `qaren-core`

**Milestone 5.2: Release Preparation**
- [ ] Set up CI/CD pipeline
- [ ] Build release binaries for all platforms
- [ ] Create release notes
- [ ] Publish to GitHub releases

**Milestone 5.3: Distribution**
- [ ] Publish `qaren-core` to crates.io
- [ ] Create installation instructions
- [ ] Create demo video
- [ ] Announce release

## Future Enhancements (Phase 2)

### JSON Support
- Parse and compare JSON configuration files
- Semantic comparison of nested structures
- Pretty-print JSON diffs

### YAML Support
- Parse and compare YAML configuration files
- Handle YAML-specific features (anchors, aliases)
- Semantic comparison of nested structures

### Advanced Diffing
- Three-way merge support
- Conflict resolution UI
- Diff visualization with side-by-side view

### Performance Enhancements
- Streaming parser for very large files (> 100 MB)
- Parallel processing for multi-file comparisons
- Incremental diffing for real-time updates

### Integration Features
- Git integration (compare configs across branches)
- CI/CD integration (exit codes for pipeline failures)
- API server mode for remote comparisons

## Conclusion

This design provides a comprehensive blueprint for implementing Qaren, a high-performance, memory-safe configuration comparison tool. The architecture prioritizes:

1. **Correctness**: Property-based testing ensures universal correctness guarantees
2. **Performance**: O(n) algorithms and efficient data structures
3. **Security**: Offline-first, memory-safe, with secret masking
4. **Usability**: Both CLI and GUI interfaces for different workflows
5. **Maintainability**: Clean separation of concerns, comprehensive testing

The Rust implementation leverages the language's strengths:
- Memory safety without garbage collection
- Zero-cost abstractions for performance
- Strong type system for correctness
- Excellent cross-platform support

With this design, Qaren will solve real-world DevOps pain points while maintaining the highest standards of software quality.
