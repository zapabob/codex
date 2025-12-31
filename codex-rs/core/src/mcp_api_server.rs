//! REST API server for dynamic MCP server management.
//!
//! This module provides HTTP endpoints to manage MCP servers dynamically.
//! The API server is optional and can be enabled via configuration.

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use tracing::info;
use tracing::warn;

use crate::config::types::McpServerConfig;
use crate::mcp_dynamic_loader::DynamicMcpLoader;

/// API server for MCP dynamic management
pub struct McpApiServer {
    loader: Arc<DynamicMcpLoader>,
    port: u16,
    running: Arc<tokio::sync::Mutex<bool>>,
}

/// Request/Response types for API
#[derive(Debug, Serialize, Deserialize)]
pub struct AddServerRequest {
    pub name: String,
    pub config: McpServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReloadServerRequest {
    pub config: McpServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerListResponse {
    pub servers: Vec<ServerInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub status: String,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl McpApiServer {
    /// Create a new API server
    pub fn new(loader: Arc<DynamicMcpLoader>, port: u16) -> Self {
        Self {
            loader,
            port,
            running: Arc::new(tokio::sync::Mutex::new(false)),
        }
    }

    /// Start the API server
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        if *running {
            return Err(anyhow::anyhow!("API server is already running"));
        }
        *running = true;
        drop(running);

        info!("Starting MCP API server on port {}", self.port);

        // Note: Actual HTTP server implementation would require axum or similar.
        // For now, this is a placeholder that can be extended.
        // The server would handle:
        // - POST /api/mcp/servers - Add server
        // - DELETE /api/mcp/servers/{name} - Remove server
        // - PUT /api/mcp/servers/{name}/reload - Reload server
        // - GET /api/mcp/servers - List servers
        // - GET /api/mcp/servers/{name}/tools - List tools

        warn!("MCP API server is not fully implemented. HTTP server requires axum dependency.");
        warn!("To enable: add axum to Cargo.toml and implement HTTP handlers.");

        Ok(())
    }

    /// Stop the API server
    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        *running = false;
        info!("Stopping MCP API server");
    }

    /// Handle add server request (internal method, called by HTTP handler when implemented)
    pub async fn handle_add_server(&self, request: AddServerRequest) -> Result<String> {
        self.loader
            .add_server(request.name.clone(), request.config)
            .await
            .with_context(|| format!("Failed to add server: {}", request.name))
    }

    /// Handle remove server request
    pub async fn handle_remove_server(&self, name: &str) -> Result<()> {
        self.loader
            .remove_server(name)
            .await
            .with_context(|| format!("Failed to remove server: {}", name))
    }

    /// Handle reload server request
    pub async fn handle_reload_server(
        &self,
        name: &str,
        request: ReloadServerRequest,
    ) -> Result<()> {
        self.loader
            .reload_server(name, request.config)
            .await
            .with_context(|| format!("Failed to reload server: {}", name))
    }

    /// Handle list servers request
    pub async fn handle_list_servers(&self) -> Result<ServerListResponse> {
        let server_names = self.loader.list_servers().await;
        let mut servers = Vec::new();

        for name in server_names {
            if let Some(state) = self.loader.get_server_state(&name).await {
                servers.push(ServerInfo {
                    name: state.name,
                    status: format!("{:?}", state.status),
                    last_updated: format!("{:?}", state.last_updated),
                });
            }
        }

        Ok(ServerListResponse { servers })
    }

    /// Check if server is running
    pub async fn is_running(&self) -> bool {
        let running = self.running.lock().await;
        *running
    }
}

// Future implementation note:
// To fully implement the HTTP server, add axum to Cargo.toml:
// ```toml
// [dependencies]
// axum = { version = "0.7", features = ["macros"] }
// ```
//
// Then implement handlers like:
// ```rust
// use axum::{Router, routing::{get, post, delete, put}, Json, extract::Path};
//
// pub fn create_router(server: Arc<McpApiServer>) -> Router {
//     Router::new()
//         .route("/api/mcp/servers", post(add_server))
//         .route("/api/mcp/servers/:name", delete(remove_server))
//         .route("/api/mcp/servers/:name/reload", put(reload_server))
//         .route("/api/mcp/servers", get(list_servers))
//         .with_state(server)
// }
// ```
