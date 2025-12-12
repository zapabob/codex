use std::path::Path;
use std::sync::Arc;

use codex_otel::otel_event_manager::OtelEventManager;
use codex_protocol::ConversationId;
use codex_protocol::protocol::SandboxCommandAssessment;
use codex_protocol::protocol::SandboxPolicy;
use codex_protocol::protocol::SandboxRiskLevel;
use codex_protocol::protocol::SessionSource;

use crate::AuthManager;
use crate::ModelProviderInfo;
use crate::command_safety::is_dangerous_command::command_might_be_dangerous;
use crate::config::Config;
use crate::features::Feature;

pub async fn assess_command(
    config: Arc<Config>,
    _provider: ModelProviderInfo,
    _auth_manager: Arc<AuthManager>,
    _otel: &OtelEventManager,
    _conversation_id: ConversationId,
    _session_source: SessionSource,
    _call_id: &str,
    command: &[String],
    _sandbox_policy: &SandboxPolicy,
    _cwd: &Path,
    failure_message: Option<&str>,
) -> Option<SandboxCommandAssessment> {
    if !config.features.enabled(Feature::SandboxCommandAssessment) {
        return None;
    }

    if !command_might_be_dangerous(command) {
        return None;
    }

    let mut description = "command may be destructive".to_string();
    if let Some(msg) = failure_message {
        description.push_str(&format!(" (sandbox failure: {msg})"));
    }

    Some(SandboxCommandAssessment {
        description,
        risk_level: SandboxRiskLevel::High,
    })
}


