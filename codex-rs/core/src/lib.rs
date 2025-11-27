//! Codex Core Library
//!
//! Core functionality for the Codex AI-Native OS

// Rust 2024 best practices: Use impl Trait in type aliases where possible
pub type Result<T> = anyhow::Result<T>;

// Common collection type aliases for better readability
pub type HashMap<K, V> = std::collections::HashMap<K, V>;
pub type HashSet<T> = std::collections::HashSet<T>;
pub type VecDeque<T> = std::collections::VecDeque<T>;

pub mod qc;
pub mod virtualization;
pub mod ai_orchestrator;
pub mod optimization_engine;
pub mod mcp_registry;
pub mod agent_runtime;
pub mod macos_virtual_os;
pub mod security_monitor;
pub mod webhook_integrator;
pub mod line_communicator;
pub mod resource_monitor;
pub mod agents;
pub mod plan;
