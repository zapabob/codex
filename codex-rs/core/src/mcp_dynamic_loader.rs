//! Dynamic MCP server loader for runtime addition, removal, and reloading of MCP servers.
//!
//! This module provides functionality to dynamically manage MCP servers at runtime,
//! including adding new servers, removing existing ones, and reloading server configurations.

use anyhow::Result;
use async_channel::Sender;
use codex_protocol::protocol::Event;
use codex_rmcp_client::OAuthCredentialsStoreMode;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(test)]
mod tests;
use std::time::SystemTime;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::config::types::McpServerConfig;
use crate::mcp::auth::McpAuthStatusEntry;
use crate::mcp_connection_manager::McpConnectionManager;
use crate::mcp_connection_manager::SandboxState;

/// Server state tracking for dynamic management
#[derive(Debug, Clone)]
pub struct ServerState {
    pub name: String,
    pub config: McpServerConfig,
    pub status: ServerStatus,
    pub last_updated: SystemTime,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerStatus {
    Initializing,
    Running,
    Stopped,
    Error(String),
}

/// Dynamic MCP loader for runtime server management
pub struct DynamicMcpLoader {
    connection_manager: Arc<tokio::sync::RwLock<McpConnectionManager>>,
    server_states: Arc<Mutex<HashMap<String, ServerState>>>,
    store_mode: OAuthCredentialsStoreMode,
    auth_entries: Arc<Mutex<HashMap<String, McpAuthStatusEntry>>>,
    tx_event: Sender<Event>,
    cancel_token: CancellationToken,
    sandbox_state: SandboxState,
}

impl DynamicMcpLoader {
    /// Create a new dynamic MCP loader
    pub fn new(
        connection_manager: Arc<tokio::sync::RwLock<McpConnectionManager>>,
        store_mode: OAuthCredentialsStoreMode,
        auth_entries: HashMap<String, McpAuthStatusEntry>,
        tx_event: Sender<Event>,
        cancel_token: CancellationToken,
        sandbox_state: SandboxState,
    ) -> Self {
        Self {
            connection_manager,
            server_states: Arc::new(Mutex::new(HashMap::new())),
            store_mode,
            auth_entries: Arc::new(Mutex::new(auth_entries)),
            tx_event,
            cancel_token,
            sandbox_state,
        }
    }

    /// Add a new MCP server dynamically
    pub async fn add_server(&self, server_name: String, config: McpServerConfig) -> Result<String> {
        // Check if server already exists
        let mut states = self.server_states.lock().await;
        if states.contains_key(&server_name) {
            return Err(anyhow::anyhow!("Server '{}' already exists", server_name));
        }

        info!("Adding MCP server dynamically: {}", server_name);

        // Create server state
        let state = ServerState {
            name: server_name.clone(),
            config: config.clone(),
            status: ServerStatus::Initializing,
            last_updated: SystemTime::now(),
        };
        states.insert(server_name.clone(), state);

        // Add to connection manager using dynamic add method
        let auth_entry = self.auth_entries.lock().await.get(&server_name).cloned();
        let mut manager = self.connection_manager.write().await;
        manager
            .add_server_dynamic(
                server_name.clone(),
                config,
                self.store_mode,
                auth_entry,
                self.tx_event.clone(),
                self.cancel_token.child_token(),
                self.sandbox_state.clone(),
            )
            .await;

        // Update state to Running after initialization
        let mut states = self.server_states.lock().await;
        if let Some(state) = states.get_mut(&server_name) {
            state.status = ServerStatus::Running;
            state.last_updated = SystemTime::now();
        }

        info!("MCP server '{}' added successfully", server_name);
        Ok(server_name)
    }

    /// Remove an MCP server dynamically
    pub async fn remove_server(&self, server_name: &str) -> Result<()> {
        let mut states = self.server_states.lock().await;
        if !states.contains_key(server_name) {
            return Err(anyhow::anyhow!("Server '{}' not found", server_name));
        }

        info!("Removing MCP server: {}", server_name);

        // Remove from connection manager
        let mut manager = self.connection_manager.write().await;
        manager.remove_server_dynamic(server_name).await;

        // Update state to Stopped and remove
        if let Some(state) = states.get_mut(server_name) {
            state.status = ServerStatus::Stopped;
            state.last_updated = SystemTime::now();
        }
        states.remove(server_name);

        info!("MCP server '{}' removed successfully", server_name);
        Ok(())
    }

    /// Reload an MCP server with new configuration
    pub async fn reload_server(
        &self,
        server_name: &str,
        new_config: McpServerConfig,
    ) -> Result<()> {
        // Remove existing server
        self.remove_server(server_name).await?;

        // Add with new configuration
        self.add_server(server_name.to_string(), new_config).await?;

        info!("MCP server '{}' reloaded successfully", server_name);
        Ok(())
    }

    /// List all dynamically managed servers
    pub async fn list_servers(&self) -> Vec<String> {
        let states = self.server_states.lock().await;
        states.keys().cloned().collect()
    }

    /// Get server state
    pub async fn get_server_state(&self, server_name: &str) -> Option<ServerState> {
        let states = self.server_states.lock().await;
        states.get(server_name).cloned()
    }

    /// Get all server states
    pub async fn get_all_server_states(&self) -> HashMap<String, ServerState> {
        let states = self.server_states.lock().await;
        states.clone()
    }
}
