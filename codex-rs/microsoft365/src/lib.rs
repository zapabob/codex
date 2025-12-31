//! Microsoft 365 integration for Codex
//!
//! Provides Office 365 API integration including Word, Excel, PowerPoint, and Outlook

pub mod auth;
pub mod client;

pub use auth::AuthManager as Microsoft365AuthManager;
pub use client::Microsoft365Client;
