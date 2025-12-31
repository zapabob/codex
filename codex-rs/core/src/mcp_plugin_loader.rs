//! Plugin loader for MCP server plugins.
//!
//! This module provides functionality to discover, load, and manage MCP server plugins
//! from a plugin directory.

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

use crate::config::types::McpServerConfig;
use crate::mcp_dynamic_loader::DynamicMcpLoader;

/// Plugin metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub enabled: bool,
}

/// Plugin structure containing metadata and server configuration
#[derive(Debug, Clone)]
pub struct McpPlugin {
    pub metadata: PluginMetadata,
    pub server_config: McpServerConfig,
    pub plugin_path: PathBuf,
}

/// Plugin loader for discovering and loading MCP plugins
pub struct McpPluginLoader {
    plugin_dir: PathBuf,
    loaded_plugins: Arc<tokio::sync::Mutex<HashMap<String, McpPlugin>>>,
}

impl McpPluginLoader {
    /// Create a new plugin loader
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self {
            plugin_dir,
            loaded_plugins: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Scan plugin directory and discover plugins
    pub async fn scan_plugins(&self) -> Result<Vec<McpPlugin>> {
        let mut plugins = Vec::new();

        if !self.plugin_dir.exists() {
            debug!("Plugin directory does not exist: {:?}", self.plugin_dir);
            return Ok(plugins);
        }

        let mut entries = fs::read_dir(&self.plugin_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let plugin_path = entry.path();
            if !plugin_path.is_dir() {
                continue;
            }

            match Self::load_plugin(&plugin_path).await {
                Ok(plugin) => {
                    info!(
                        "Discovered plugin: {} ({})",
                        plugin.metadata.name, plugin.metadata.version
                    );
                    plugins.push(plugin);
                }
                Err(e) => {
                    warn!("Failed to load plugin from {:?}: {}", plugin_path, e);
                }
            }
        }

        Ok(plugins)
    }

    /// Load a single plugin from a directory
    async fn load_plugin(plugin_path: &Path) -> Result<McpPlugin> {
        let plugin_toml_path = plugin_path.join("plugin.toml");
        let server_toml_path = plugin_path.join("server.toml");

        // Load plugin metadata
        let metadata = if plugin_toml_path.exists() {
            let content = fs::read_to_string(&plugin_toml_path).await?;
            let mut metadata: PluginMetadata = toml::from_str(&content).with_context(|| {
                format!("Failed to parse plugin.toml at {:?}", plugin_toml_path)
            })?;

            // Ensure plugin name matches directory name if not set
            if metadata.name.is_empty() {
                metadata.name = plugin_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
            }
            metadata
        } else {
            // Default metadata if plugin.toml doesn't exist
            PluginMetadata {
                name: plugin_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                version: "1.0.0".to_string(),
                description: None,
                author: None,
                enabled: true,
            }
        };

        // Load server configuration
        let server_config = if server_toml_path.exists() {
            let content = fs::read_to_string(&server_toml_path).await?;
            toml::from_str(&content)
                .with_context(|| format!("Failed to parse server.toml at {:?}", server_toml_path))?
        } else {
            return Err(anyhow::anyhow!(
                "server.toml not found in plugin directory: {:?}",
                plugin_path
            ));
        };

        Ok(McpPlugin {
            metadata,
            server_config,
            plugin_path: plugin_path.to_path_buf(),
        })
    }

    /// Load all enabled plugins into the dynamic loader
    pub async fn load_enabled_plugins(&self, loader: &DynamicMcpLoader) -> Result<Vec<String>> {
        let plugins = self.scan_plugins().await?;
        let mut loaded_names = Vec::new();

        for plugin in plugins {
            if !plugin.metadata.enabled {
                debug!("Skipping disabled plugin: {}", plugin.metadata.name);
                continue;
            }

            let server_name = format!("plugin-{}", plugin.metadata.name);
            match loader
                .add_server(server_name.clone(), plugin.server_config.clone())
                .await
            {
                Ok(_) => {
                    info!("Loaded plugin: {}", plugin.metadata.name);
                    loaded_names.push(server_name.clone());

                    // Store loaded plugin
                    let mut loaded = self.loaded_plugins.lock().await;
                    loaded.insert(server_name, plugin);
                }
                Err(e) => {
                    error!("Failed to load plugin {}: {}", plugin.metadata.name, e);
                }
            }
        }

        Ok(loaded_names)
    }

    /// Unload a plugin
    pub async fn unload_plugin(&self, plugin_name: &str, loader: &DynamicMcpLoader) -> Result<()> {
        let server_name = format!("plugin-{}", plugin_name);
        loader.remove_server(&server_name).await?;

        let mut loaded = self.loaded_plugins.lock().await;
        loaded.remove(&server_name);

        info!("Unloaded plugin: {}", plugin_name);
        Ok(())
    }

    /// Get list of loaded plugins
    pub async fn list_loaded_plugins(&self) -> Vec<String> {
        let loaded = self.loaded_plugins.lock().await;
        loaded.keys().cloned().collect()
    }

    /// Get plugin information
    pub async fn get_plugin(&self, plugin_name: &str) -> Option<McpPlugin> {
        let server_name = format!("plugin-{}", plugin_name);
        let loaded = self.loaded_plugins.lock().await;
        loaded.get(&server_name).cloned()
    }
}

use std::sync::Arc;
