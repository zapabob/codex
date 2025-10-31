//! Agent orchestrator module for coordinating write operations across multiple clients.
//!
//! This module provides:
//! - Versioned protocol with JSON Lines over UDS/Pipe/TCP
//! - HMAC-based authentication
//! - Single-writer queue for serializing write operations
//! - Idempotency cache for request deduplication
//! - Event pub/sub for real-time updates

pub mod auth;
pub mod idempotency;
pub mod protocol;
pub mod queue;
pub mod server;
pub mod transport;

pub use auth::{AuthConfig, AuthManager};
pub use idempotency::IdempotencyCache;
pub use protocol::{Envelope, MessageType, ResponseStatus};
pub use queue::{QueueConfig, SingleWriterQueue, Task, TaskExecutor, TaskStatus};
pub use server::{OrchestratorServer, ServerConfig};
pub use transport::{TransportConfig, TransportServer, TransportConnection};

