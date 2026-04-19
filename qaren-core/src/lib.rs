//! # Qaren Core
//!
//! Core library for the Qaren (قارن) configuration comparison tool.
//!
//! This crate provides the parsing, diffing, and patch generation logic
//! used by both the CLI and GUI applications. It has **zero UI dependencies**
//! and can be used standalone as a library.
//!
//! ## Modules
//!
//! - [`types`] — Core data structures (`KvPair`, `ConfigFile`, `DiffResult`, etc.)
//! - [`error`] — Custom error types (`QarenError`, `QarenResult`)
//! - [`parser`] — Configuration file parser with safe splitting and quote stripping

pub mod error;
pub mod parser;
pub mod types;

// Re-export key types for ergonomic imports
pub use error::{QarenError, QarenResult};
pub use parser::{parse_content, parse_file};
pub use types::{
    ConfigFile, DiffLine, DiffResult, KvPair, LiteralDiffResult, ModifiedPair, ParseOptions,
    PatchDirection,
};
