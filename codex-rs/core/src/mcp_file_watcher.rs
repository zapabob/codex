//! File system watcher for MCP server configuration files.
//!
//! This module provides functionality to monitor MCP server configuration files
//! and automatically reload servers when changes are detected.

use anyhow::Context;
use anyhow::Result;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

use crate::mcp_dynamic_loader::DynamicMcpLoader;

/// File watcher for MCP configuration files
pub struct McpFileWatcher {
    config_path: PathBuf,
    plugin_dir: PathBuf,
    loader: Option<Arc<DynamicMcpLoader>>,
    watch_interval: Duration,
    running: Arc<tokio::sync::Mutex<bool>>,
}

impl McpFileWatcher {
    /// Create a new file watcher
    pub fn new(
        config_path: PathBuf,
        plugin_dir: PathBuf,
        watch_interval: Option<Duration>,
    ) -> Self {
        Self {
            config_path,
            plugin_dir,
            loader: None,
            watch_interval: watch_interval.unwrap_or(Duration::from_secs(5)),
            running: Arc::new(tokio::sync::Mutex::new(false)),
        }
    }

    /// Set the dynamic loader to use for reloading
    pub fn set_loader(&mut self, loader: Arc<DynamicMcpLoader>) {
        self.loader = Some(loader);
    }

    /// Start watching for file changes
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        if *running {
            return Err(anyhow::anyhow!("File watcher is already running"));
        }
        *running = true;
        drop(running);

        let config_path = self.config_path.clone();
        let plugin_dir = self.plugin_dir.clone();
        let watch_interval = self.watch_interval;
        let loader = self.loader.clone();
        let running = self.running.clone();

        tokio::spawn(async move {
            let mut last_config_mtime = Self::get_file_mtime(&config_path).ok();
            let mut last_plugin_check = std::time::SystemTime::now();

            info!("Started MCP file watcher for: {:?}", config_path);

            loop {
                // Check if we should stop
                {
                    let running_guard = running.lock().await;
                    if !*running_guard {
                        info!("MCP file watcher stopped");
                        break;
                    }
                }

                // Check config file changes
                if let Ok(current_mtime) = Self::get_file_mtime(&config_path) {
                    if last_config_mtime.as_ref() != Some(&current_mtime) {
                        info!("MCP config file changed, reloading...");
                        if let Some(loader) = &loader {
                            // Reload configuration
                            if let Err(e) = Self::reload_config_file(loader, &config_path).await {
                                error!("Failed to reload config file: {}", e);
                            }
                        }
                        last_config_mtime = Some(current_mtime);
                    }
                }

                // Check plugin directory changes (less frequently)
                let now = std::time::SystemTime::now();
                if now
                    .duration_since(last_plugin_check)
                    .unwrap_or(Duration::ZERO)
                    > Duration::from_secs(30)
                {
                    if let Some(loader) = &loader {
                        if let Err(e) = Self::check_plugin_directory(loader, &plugin_dir).await {
                            warn!("Failed to check plugin directory: {}", e);
                        }
                    }
                    last_plugin_check = now;
                }

                sleep(watch_interval).await;
            }
        });

        Ok(())
    }

    /// Stop watching for file changes
    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        *running = false;
        info!("Stopping MCP file watcher");
    }

    /// Get file modification time
    fn get_file_mtime(path: &Path) -> Result<std::time::SystemTime> {
        let metadata = std::fs::metadata(path)
            .with_context(|| format!("Failed to get metadata for {:?}", path))?;
        metadata
            .modified()
            .with_context(|| format!("Failed to get modification time for {:?}", path))
    }

    /// Reload configuration file
    async fn reload_config_file(
        loader: &DynamicMcpLoader,
        config_path: &Path,
    ) -> Result<()> {
        // Read and parse config file
        // This is a simplified version - actual implementation would need to
        // parse the config file format (YAML/TOML) and reload servers
        debug!("Reloading config file: {:?}", config_path);
        // TODO: Implement actual config file parsing and reloading
        Ok(())
    }

    /// Check plugin directory for new/removed plugins
    async fn check_plugin_directory(
        loader: &DynamicMcpLoader,
        plugin_dir: &Path,
    ) -> Result<()> {
        if !plugin_dir.exists() {
            return Ok(());
        }

        debug!("Checking plugin directory: {:?}", plugin_dir);
        // TODO: Implement plugin directory scanning and loading
        Ok(())
    }
}

use std::sync::Arc;
