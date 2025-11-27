// Module declarations for the app-server protocol namespace.
// Exposes protocol pieces used by `lib.rs` via `pub use protocol::common::*;`.

pub(crate) mod common;
pub(crate) mod thread_history;
pub(crate) mod v1;
pub(crate) mod v2;
