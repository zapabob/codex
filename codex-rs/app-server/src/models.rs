//! Models for app server

<<<<<<< HEAD
use serde::{Deserialize, Serialize};
use codex_app_server_protocol::AuthMode;
use codex_protocol::config_types::ReasoningEffort;

/// Request model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestModel {
    pub id: String,
    pub data: serde_json::Value,
=======
use codex_app_server_protocol::Model;
use codex_app_server_protocol::ReasoningEffortOption;
use codex_core::ConversationManager;
use codex_core::config::Config;
use codex_protocol::openai_models::ModelPreset;
use codex_protocol::openai_models::ReasoningEffortPreset;

pub async fn supported_models(
    conversation_manager: Arc<ConversationManager>,
    config: &Config,
) -> Vec<Model> {
    conversation_manager
        .list_models(config)
        .await
        .into_iter()
        .map(model_from_preset)
        .collect()
>>>>>>> upstream/main
}

/// Response model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseModel {
    pub id: String,
    pub result: serde_json::Value,
}

/// Supported models list
pub fn supported_models(_auth_mode: Option<AuthMode>) -> Vec<codex_app_server_protocol::Model> {
    vec![
        codex_app_server_protocol::Model {
            id: "gpt-4".to_string(),
            model: "gpt-4".to_string(),
            display_name: "GPT-4".to_string(),
            description: "".to_string(),
            supported_reasoning_efforts: vec![],
            default_reasoning_effort: ReasoningEffort::Medium,
            is_default: false,
        },
        codex_app_server_protocol::Model {
            id: "gpt-3.5-turbo".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            display_name: "GPT-3.5 Turbo".to_string(),
            description: "".to_string(),
            supported_reasoning_efforts: vec![],
            default_reasoning_effort: ReasoningEffort::Medium,
            is_default: false,
        },
        codex_app_server_protocol::Model {
            id: "claude-3".to_string(),
            model: "claude-3".to_string(),
            display_name: "Claude 3".to_string(),
            description: "".to_string(),
            supported_reasoning_efforts: vec![],
            default_reasoning_effort: ReasoningEffort::Medium,
            is_default: false,
        },
    ]
}
