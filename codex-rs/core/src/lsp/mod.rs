//! LSP (Language Server Protocol) integration for real-time diagnostics
//!
//! Provides integration with existing LSP servers (rust-analyzer, TypeScript Server, etc.)
//! for real-time code diagnostics, completion, and symbol search.

pub mod client;
pub mod diagnostics;

pub use client::LspClient;
pub use diagnostics::DiagnosticsManager;
pub use lsp_types::Url;
