//! Models for app server

use codex_app_server_protocol::AuthMode;
use codex_protocol::config_types::ReasoningEffort;
use serde::Deserialize;
use serde::Serialize;

/// Request model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestModel {
    pub id: String,
    pub data: serde_json::Value,
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
