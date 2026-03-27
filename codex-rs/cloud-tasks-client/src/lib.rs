//! Cloud Tasks Client

pub mod client;
pub mod types;

#[cfg(feature = "mock")]
mod mock;

#[cfg(feature = "online")]
mod http;

#[cfg(feature = "mock")]
pub use mock::MockClient;

#[cfg(feature = "online")]
pub use http::HttpClient;

// Reusable apply engine now lives in the shared crate `codex-git-utils`.
pub use client::*;
pub use types::*;

