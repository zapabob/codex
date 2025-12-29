/// Orchestrator server for multi-instance coordination
///
/// Provides repository-level locking, single-writer queue, and RPC API
/// for coordinating multiple CLI/GUI/agent instances.
pub mod audit;
pub mod auth;
pub mod rate_limit;
pub mod replay_protection;
pub mod rpc;
pub mod security_headers;
pub mod server;
pub mod transport;

pub use auth::AuthHeader;
pub use auth::AuthManager;
pub use auth::CodexAuth;
pub use auth::HmacAuthenticator;
pub use rpc::*;
pub use server::OrchestratorConfig;
pub use server::OrchestratorServer;
pub use transport::Connection;
pub use transport::Transport;
pub use transport::TransportConfig;
pub use transport::TransportInfo;
pub use transport::TransportPreference;
pub use transport::create_transport;
