use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod secret_masking;

pub use crate::audit_log::AuditLogger;
pub use secret_masking::mask_debug_output;
pub use secret_masking::mask_error_message;
pub use secret_masking::mask_secrets;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityContext {
    pub session_id: Option<String>,
    pub user_id: Option<String>,
    pub tags: HashMap<String, String>,
}
