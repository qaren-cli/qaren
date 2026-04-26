//! # Qaren Core
//!
//! Core library for the **Qaren** (قارن — "compare" in Arabic) configuration
//! comparison tool. This crate provides parsing, diffing, and patch generation
//! logic used by both the CLI and GUI applications.
//!
//! It has **zero UI dependencies** and can be used standalone as a Rust library.
//!
//! ## Quick Start
//!
//! ```rust
//! use qaren_core::{parse_content, semantic_diff, ParseOptions, DiffOptions};
//! use std::path::Path;
//!
//! let opts = ParseOptions::default();
//! let diff_opts = DiffOptions::default();
//! let file1 = parse_content("KEY=value1\nDB=postgres", Path::new("a.env"), &opts).unwrap();
//! let file2 = parse_content("KEY=value2\nNEW=added", Path::new("b.env"), &opts).unwrap();
//!
//! let diff = semantic_diff(&file1, &file2, &diff_opts);
//! assert!(!diff.is_identical());
//! ```
//!
//! ## Modules
//!
//! - [`types`] — Core data structures (`KvPair`, `ConfigFile`, `DiffResult`, etc.)
//! - [`error`] — Custom error types (`QarenError`, `QarenResult`)
//! - [`parser`] — Configuration file parser with safe splitting and quote stripping
//! - [`diff`] — Semantic and literal diff engines
//! - [`patch`] — Patch file generator with bidirectional support

pub mod diff;
pub mod directory;
pub mod error;
pub mod masking;
pub mod parser;
pub mod patch;
pub mod types;

// Re-export key types for ergonomic imports:
//   use qaren_core::{parse_file, semantic_diff, generate_patch, ...};

pub use diff::{literal_diff, normalise, semantic_diff};
pub use directory::{collect_files_recursive, semantic_diff_dir, DirParseOptions};
pub use error::{QarenError, QarenResult};
pub use masking::{mask_value, should_mask};
pub use parser::{detect_delimiter, parse_content, parse_file, strip_ansi};
pub use patch::{generate_patch, generate_recursive_patch};
pub use types::{
    ConfigFile, DiffLine, DiffOptions, DiffResult, DirDiffResult, FileDiffStatus, KvPair, LiteralDiffResult, ModifiedPair,
    ParseOptions, PatchDirection,
};
