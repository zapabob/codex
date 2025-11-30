//! Codex Core Library
//!
//! Core functionality for the Codex AI-Native OS

// Rust 2024 best practices: Use impl Trait in type aliases where possible
pub type Result<T> = anyhow::Result<T>;

// Common collection type aliases for better readability
pub type HashMap<K, V> = std::collections::HashMap<K, V>;
pub type HashSet<T> = std::collections::HashSet<T>;
pub type VecDeque<T> = std::collections::VecDeque<T>;

pub mod agent_interpreter;
pub mod agents;
pub mod ai_orchestrator;
pub mod api_bridge;
pub mod apply_patch;
pub mod async_subagent_integration;
pub mod audit_log;
pub mod auth;
pub use auth::AuthManager;
pub mod bash;
pub mod client;
pub mod client_common;
pub mod codex;
pub mod codex_conversation;
pub mod codex_delegate;
pub mod command_safety;
pub mod compact;
pub mod compact_remote;
pub mod config;
pub mod config_loader;
pub mod context_manager;
pub mod conversation_history;
pub mod conversation_manager;
pub mod custom_commands;
pub mod custom_prompts;
pub mod default_client;
pub mod environment_context;
pub mod error;
pub mod event_mapping;

// Re-export protocol types and functions
pub use codex_protocol::*;

// Re-export protocol types directly
pub use codex_protocol::*;

// Re-export protocol submodule for crate::protocol access
pub use codex_protocol::protocol;
pub use event_mapping::parse_turn_item;
pub mod exec;
pub mod exec_env;
pub mod exec_policy;
pub mod execution;
pub mod features;
pub mod flags;
pub mod function_tool;
pub mod git_info;
pub mod hooks;
pub mod hybrid_acceleration;
pub mod landlock;
pub mod line_communicator;
pub mod lock;
pub mod macos_virtual_os;
pub mod mcp;
pub mod mcp_connection_manager;
pub mod mcp_tool_call;
pub mod message_history;
pub mod model_family;
pub mod model_provider_info;

// Re-export commonly used types
pub use model_provider_info::ModelProviderInfo;
pub use safety::get_platform_sandbox;

// RMCP client stubs when rmcp feature is disabled
#[cfg(not(feature = "rmcp"))]
pub mod rmcp_client_stub {
    use std::fmt;

    #[derive(Debug)]
    pub struct RmcpClient;

    #[derive(Debug, Clone)]
    pub enum OAuthCredentialsStoreMode {
        Keyring,
        Env,
    }

    #[derive(Debug)]
    pub struct ElicitationResponse;

    #[derive(Debug)]
    pub struct SendElicitation;

    pub enum ElicitationAction {
        Accept,
        Decline,
        Cancel,
    }

    pub fn determine_streamable_http_auth_status() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    impl fmt::Display for RmcpClient {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RmcpClient(stub)")
        }
    }
}

#[cfg(not(feature = "rmcp"))]
pub use rmcp_client_stub::*;

// ExecPolicy stubs when execpolicy feature is disabled
#[cfg(not(feature = "execpolicy"))]
pub mod execpolicy_stub {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Policy;

    #[derive(Debug, Clone)]
    pub enum Decision {
        Allow,
        Deny,
        Prompt,
    }

    #[derive(Debug, Clone)]
    pub struct Evaluation {
        pub decision: Decision,
        pub reason: String,
    }

    #[derive(Debug)]
    pub struct PolicyParser;

    impl Policy {
        pub fn evaluate(&self, _command: &[String], _env: &HashMap<String, String>) -> Evaluation {
            Evaluation {
                decision: Decision::Allow,
                reason: "Policy disabled".to_string(),
            }
        }
    }

    impl PolicyParser {
        pub fn new() -> Self {
            Self
        }

        pub fn parse(&self, _content: &str) -> Result<Policy, Box<dyn std::error::Error>> {
            Ok(Policy)
        }
    }
}

#[cfg(not(feature = "execpolicy"))]
pub use execpolicy_stub::*;
pub mod natural_language_parser;
pub mod openai_model_info;
pub mod optimization_engine;
pub mod orchestration;
pub mod otel_init;
pub mod parse_command;
pub mod plan;
pub mod powershell;
pub mod project_doc;
pub mod qc;
pub mod resource_monitor;
pub mod response_processing;
pub mod review_format;
pub mod rollout;
pub mod safety;
pub mod sandboxing;
pub mod seatbelt;
pub mod security;
pub mod security_monitor;
pub mod shell;
pub mod spawn;
pub mod state;
pub mod tasks;
pub mod telemetry;
pub mod terminal;
pub mod token_budget;
pub mod token_data;
pub mod tools;
pub mod truncate;
pub mod turn_diff_tracker;
pub mod unified_exec;
pub mod user_instructions;
pub mod user_notification;
pub mod user_shell_command;
pub mod util;
pub mod virtualization;
pub mod webhook_integrator;
pub mod webhooks;
pub mod windows_ai_integration;
