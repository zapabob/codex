//! Web Search Provider - Real web search integration
//! Conforms to OpenAI/codex official web_search implementation
//!
//! This crate provides web search functionality that can be used independently
//! or as a provider for deep research systems.

pub mod types;
pub mod url_decoder;
pub mod web_search_provider;

pub use types::Source;
pub use web_search_provider::{ResearchProvider, WebSearchProvider};
