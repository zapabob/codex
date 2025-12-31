//! MCP Integration Manager
//!
//! Manages enhanced MCP server integrations including Serena, ArXiv, GitHub, etc.
//!
//! Enhanced with Windows 11 25H2 AI integration and VR/AR support

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(all(target_os = "windows", feature = "windows-ai"))]
use crate::windows_ai_integration::WindowsAiOptions;
#[cfg(all(target_os = "windows", feature = "windows-ai"))]
use crate::windows_ai_integration::execute_with_windows_ai;

/// Enhanced MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedMcpServer {
    /// Server command
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Server capabilities
    pub capabilities: Vec<String>,
}

/// MCP servers configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServersConfig {
    /// MCP servers
    pub servers: HashMap<String, EnhancedMcpServer>,
    /// Development mode specific servers
    pub development: HashMap<String, Vec<String>>,
    /// Agent-specific server assignments
    pub agent_servers: HashMap<String, Vec<String>>,
    /// Auto-start servers
    pub auto_start: HashMap<String, Vec<String>>,
}

/// MCP integration manager with Windows 11 25H2 AI support
pub struct McpIntegrationManager {
    /// Configuration path
    config_path: PathBuf,
    /// Active servers
    active_servers: Arc<Mutex<HashMap<String, EnhancedMcpServer>>>,
    /// Server processes
    server_processes: Arc<Mutex<HashMap<String, tokio::process::Child>>>,
    /// Windows AI integration options
    #[cfg(all(target_os = "windows", feature = "windows-ai"))]
    windows_ai_options: WindowsAiOptions,
}

impl Default for McpIntegrationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl McpIntegrationManager {
    /// Create new MCP integration manager with Windows 11 25H2 AI support
    pub fn new() -> Self {
        let config_path = PathBuf::from(".codex").join("mcp-servers.yaml");

        Self {
            config_path,
            active_servers: Arc::new(Mutex::new(HashMap::new())),
            server_processes: Arc::new(Mutex::new(HashMap::new())),
            #[cfg(all(target_os = "windows", feature = "windows-ai"))]
            windows_ai_options: WindowsAiOptions {
                enabled: true, // Enable by default on Windows 11 25H2
                kernel_accelerated: true,
                use_gpu: true,
            },
        }
    }

    /// Load MCP servers configuration
    pub async fn load_config(&self) -> Result<McpServersConfig, String> {
        if !self.config_path.exists() {
            // Create default configuration
            let default_config = self.create_default_config();
            self.save_config(&default_config).await?;
            return Ok(default_config);
        }

        let content = tokio::fs::read_to_string(&self.config_path)
            .await
            .map_err(|e| format!("Failed to read MCP config: {e}"))?;

        let config: McpServersConfig = serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse MCP config: {e}"))?;

        Ok(config)
    }

    /// Create default MCP servers configuration
    fn create_default_config(&self) -> McpServersConfig {
        let mut servers = HashMap::new();

        // Serena - Advanced code intelligence
        servers.insert(
            "serena".to_string(),
            EnhancedMcpServer {
                command: "npx".to_string(),
                args: vec![
                    "@modelcontextprotocol/server-serena".to_string(),
                    "--port".to_string(),
                    "3001".to_string(),
                ],
                env: {
                    let mut env = HashMap::new();
                    env.insert("MCP_SERENA_MODEL".to_string(), "gpt-4o".to_string());
                    env.insert("MCP_SERENA_MAX_TOKENS".to_string(), "100000".to_string());
                    env
                },
                capabilities: vec![
                    "code-analysis".to_string(),
                    "semantic-search".to_string(),
                    "refactoring-suggestions".to_string(),
                    "dependency-analysis".to_string(),
                ],
            },
        );

        // ArXiv - Academic paper search
        servers.insert(
            "arxiv".to_string(),
            EnhancedMcpServer {
                command: "python".to_string(),
                args: vec!["-m".to_string(), "arxiv_mcp_server".to_string()],
                env: {
                    let mut env = HashMap::new();
                    env.insert("ARXIV_MAX_RESULTS".to_string(), "50".to_string());
                    env.insert("ARXIV_SORT_BY".to_string(), "relevance".to_string());
                    env
                },
                capabilities: vec![
                    "academic-search".to_string(),
                    "paper-analysis".to_string(),
                    "citation-extraction".to_string(),
                    "research-synthesis".to_string(),
                ],
            },
        );

        // GitHub - Repository management
        servers.insert(
            "github".to_string(),
            EnhancedMcpServer {
                command: "node".to_string(),
                args: vec!["dist/github-mcp-server.js".to_string()],
                env: {
                    let mut env = HashMap::new();
                    env.insert("GITHUB_TOKEN".to_string(), "${GITHUB_TOKEN}".to_string());
                    env.insert(
                        "GITHUB_API_BASE".to_string(),
                        "https://api.github.com".to_string(),
                    );
                    env
                },
                capabilities: vec![
                    "repo-search".to_string(),
                    "issue-management".to_string(),
                    "pr-analysis".to_string(),
                    "code-review".to_string(),
                    "contributor-insights".to_string(),
                ],
            },
        );

        // Git-enhanced filesystem
        servers.insert(
            "git-enhanced".to_string(),
            EnhancedMcpServer {
                command: "git-enhanced-mcp".to_string(),
                args: vec!["--enable-git-log".to_string(), "--enable-blame".to_string()],
                env: HashMap::new(),
                capabilities: vec![
                    "git-history".to_string(),
                    "code-blame".to_string(),
                    "commit-analysis".to_string(),
                    "branch-comparison".to_string(),
                ],
            },
        );

        // Documentation analyzer
        servers.insert(
            "docs-analyzer".to_string(),
            EnhancedMcpServer {
                command: "docs-analyzer-mcp".to_string(),
                args: vec![
                    "--support-formats".to_string(),
                    "md,pdf,html,docx".to_string(),
                ],
                env: HashMap::new(),
                capabilities: vec![
                    "document-parsing".to_string(),
                    "content-extraction".to_string(),
                    "structure-analysis".to_string(),
                    "search-indexing".to_string(),
                ],
            },
        );

        // YouTube - Video search and analysis
        servers.insert(
            "youtube".to_string(),
            EnhancedMcpServer {
                command: "youtube-mcp-server".to_string(),
                args: vec!["--api-key".to_string(), "${YOUTUBE_API_KEY}".to_string()],
                env: {
                    let mut env = HashMap::new();
                    env.insert(
                        "YOUTUBE_API_KEY".to_string(),
                        "${YOUTUBE_API_KEY}".to_string(),
                    );
                    env.insert("YOUTUBE_MAX_RESULTS".to_string(), "25".to_string());
                    env.insert("YOUTUBE_SEARCH_ORDER".to_string(), "relevance".to_string());
                    env
                },
                capabilities: vec![
                    "video-search".to_string(),
                    "channel-analysis".to_string(),
                    "transcript-extraction".to_string(),
                    "video-metadata".to_string(),
                    "content-analysis".to_string(),
                ],
            },
        );

        // Playwright - Browser automation and scraping
        servers.insert(
            "playwright".to_string(),
            EnhancedMcpServer {
                command: "playwright-mcp-server".to_string(),
                args: vec![
                    "--browser".to_string(),
                    "chromium".to_string(),
                    "--headless".to_string(),
                    "true".to_string(),
                ],
                env: {
                    let mut env = HashMap::new();
                    env.insert("PLAYWRIGHT_BROWSER".to_string(), "chromium".to_string());
                    env.insert("PLAYWRIGHT_HEADLESS".to_string(), "true".to_string());
                    env.insert("PLAYWRIGHT_TIMEOUT".to_string(), "30000".to_string());
                    env.insert("PLAYWRIGHT_VIEWPORT".to_string(), "1280x720".to_string());
                    env
                },
                capabilities: vec![
                    "web-scraping".to_string(),
                    "browser-automation".to_string(),
                    "screenshot-capture".to_string(),
                    "dom-analysis".to_string(),
                    "form-interaction".to_string(),
                    "javascript-execution".to_string(),
                ],
            },
        );

        // Filesystem - Enhanced file system operations
        servers.insert(
            "filesystem".to_string(),
            EnhancedMcpServer {
                command: "filesystem-mcp-server".to_string(),
                args: vec![
                    "--root-dir".to_string(),
                    ".".to_string(),
                    "--enable-git".to_string(),
                    "true".to_string(),
                    "--enable-metadata".to_string(),
                    "true".to_string(),
                ],
                env: {
                    let mut env = HashMap::new();
                    env.insert("FILESYSTEM_ROOT".to_string(), ".".to_string());
                    env.insert("FILESYSTEM_ENABLE_GIT".to_string(), "true".to_string());
                    env.insert("FILESYSTEM_ENABLE_METADATA".to_string(), "true".to_string());
                    env.insert(
                        "FILESYSTEM_MAX_FILE_SIZE".to_string(),
                        "10485760".to_string(),
                    ); // 10MB
                    env
                },
                capabilities: vec![
                    "file-operations".to_string(),
                    "directory-traversal".to_string(),
                    "git-integration".to_string(),
                    "metadata-extraction".to_string(),
                    "content-analysis".to_string(),
                    "batch-operations".to_string(),
                    "search-indexing".to_string(),
                ],
            },
        );

        // Gemini CLI MCP - Gemini AI integration
        servers.insert(
            "gemini-cli".to_string(),
            EnhancedMcpServer {
                command: "codex-gemini-mcp".to_string(),
                args: vec![],
                env: {
                    let mut env = HashMap::new();
                    env.insert(
                        "GEMINI_API_KEY".to_string(),
                        "${GEMINI_API_KEY}".to_string(),
                    );
                    env.insert(
                        "GEMINI_MODEL".to_string(),
                        "gemini-2.0-flash-exp".to_string(),
                    );
                    env.insert("GEMINI_MAX_TOKENS".to_string(), "8192".to_string());
                    env.insert("GEMINI_TEMPERATURE".to_string(), "0.7".to_string());
                    env.insert(
                        "OAUTH_CLIENT_ID".to_string(),
                        "${OAUTH_CLIENT_ID}".to_string(),
                    );
                    env.insert(
                        "OAUTH_CLIENT_SECRET".to_string(),
                        "${OAUTH_CLIENT_SECRET}".to_string(),
                    );
                    env
                },
                capabilities: vec![
                    "ai-assistance".to_string(),
                    "code-generation".to_string(),
                    "code-review".to_string(),
                    "documentation".to_string(),
                    "research-assistance".to_string(),
                    "problem-solving".to_string(),
                    "creative-writing".to_string(),
                    "data-analysis".to_string(),
                ],
            },
        );

        // Development modes
        let mut development = HashMap::new();
        development.insert(
            "parallel-dev".to_string(),
            vec![
                "serena".to_string(),
                "github".to_string(),
                "git-enhanced".to_string(),
                "filesystem".to_string(),
                "playwright".to_string(),
            ],
        );
        development.insert(
            "centralized-dev".to_string(),
            vec![
                "serena".to_string(),
                "arxiv".to_string(),
                "docs-analyzer".to_string(),
                "youtube".to_string(),
                "gemini-cli".to_string(),
            ],
        );

        // Agent-specific servers
        let mut agent_servers = HashMap::new();
        agent_servers.insert(
            "architect".to_string(),
            vec![
                "serena".to_string(),
                "docs-analyzer".to_string(),
                "git-enhanced".to_string(),
                "youtube".to_string(),
                "filesystem".to_string(),
                "gemini-cli".to_string(),
            ],
        );
        agent_servers.insert(
            "researcher".to_string(),
            vec![
                "arxiv".to_string(),
                "github".to_string(),
                "docs-analyzer".to_string(),
                "youtube".to_string(),
                "gemini-cli".to_string(),
            ],
        );
        agent_servers.insert(
            "code-reviewer".to_string(),
            vec![
                "serena".to_string(),
                "github".to_string(),
                "git-enhanced".to_string(),
                "filesystem".to_string(),
                "gemini-cli".to_string(),
            ],
        );
        agent_servers.insert(
            "qc-optimizer".to_string(),
            vec![
                "serena".to_string(),
                "github".to_string(),
                "docs-analyzer".to_string(),
                "filesystem".to_string(),
                "playwright".to_string(),
            ],
        );

        // Auto-start servers
        let mut auto_start = HashMap::new();
        auto_start.insert(
            "centralized".to_string(),
            vec![
                "serena".to_string(),
                "arxiv".to_string(),
                "gemini-cli".to_string(),
            ],
        );
        auto_start.insert(
            "parallel".to_string(),
            vec![
                "serena".to_string(),
                "github".to_string(),
                "git-enhanced".to_string(),
                "filesystem".to_string(),
            ],
        );

        McpServersConfig {
            servers,
            development,
            agent_servers,
            auto_start,
        }
    }

    /// Save MCP servers configuration
    pub async fn save_config(&self, config: &McpServersConfig) -> Result<(), String> {
        let content = serde_yaml::to_string(config)
            .map_err(|e| format!("Failed to serialize MCP config: {e}"))?;

        // Create directory if it doesn't exist
        if let Some(parent) = self.config_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create config directory: {e}"))?;
        }

        tokio::fs::write(&self.config_path, content)
            .await
            .map_err(|e| format!("Failed to write MCP config: {e}"))?;

        Ok(())
    }

    /// Start MCP servers for development mode
    pub async fn start_servers_for_mode(&self, mode: &str) -> Result<(), String> {
        let config = self.load_config().await?;

        let server_names = config
            .auto_start
            .get(mode)
            .ok_or_else(|| format!("Unknown development mode: {mode}"))?
            .clone();

        for server_name in server_names {
            self.start_server(&server_name).await?;
        }

        Ok(())
    }

    /// Start specific MCP server
    pub async fn start_server(&self, name: &str) -> Result<(), String> {
        let config = self.load_config().await?;

        let server_config = config
            .servers
            .get(name)
            .ok_or_else(|| format!("Unknown MCP server: {name}"))?
            .clone();

        let mut command = tokio::process::Command::new(&server_config.command);
        command.args(&server_config.args);

        // Set environment variables
        for (key, value) in &server_config.env {
            command.env(key, value);
        }

        let child = command
            .spawn()
            .map_err(|e| format!("Failed to start MCP server {name}: {e}"))?;

        let mut processes = self.server_processes.lock().await;
        let mut servers = self.active_servers.lock().await;

        processes.insert(name.to_string(), child);
        servers.insert(name.to_string(), server_config);

        Ok(())
    }

    /// Stop MCP server
    pub async fn stop_server(&self, name: &str) -> Result<(), String> {
        let mut processes = self.server_processes.lock().await;

        if let Some(mut child) = processes.remove(name) {
            child
                .kill()
                .await
                .map_err(|e| format!("Failed to stop MCP server {name}: {e}"))?;
        }

        let mut servers = self.active_servers.lock().await;
        servers.remove(name);

        Ok(())
    }

    /// Get active servers
    pub async fn get_active_servers(&self) -> HashMap<String, EnhancedMcpServer> {
        let servers = self.active_servers.lock().await;
        servers.clone()
    }

    /// Get servers for agent
    pub async fn get_servers_for_agent(&self, agent_name: &str) -> Result<Vec<String>, String> {
        let config = self.load_config().await?;

        Ok(config
            .agent_servers
            .get(agent_name)
            .cloned()
            .unwrap_or_default())
    }

    /// Check if server is running
    pub async fn is_server_running(&self, name: &str) -> bool {
        let processes = self.server_processes.lock().await;
        processes.contains_key(name)
    }

    /// Execute MCP task with Windows AI optimization (Windows 11 25H2)
    #[cfg(all(target_os = "windows", feature = "windows-ai"))]
    pub async fn execute_with_ai_optimization(
        &self,
        task_description: &str,
        server_name: &str,
    ) -> Result<String, String> {
        if !self.windows_ai_options.enabled {
            return Err("Windows AI not enabled".to_string());
        }

        let prompt = format!(
            "Execute MCP task '{task_description}' on server '{server_name}' with optimal parameters for Windows 11 25H2 AI acceleration"
        );

        execute_with_windows_ai(&prompt, &self.windows_ai_options)
            .await
            .map_err(|e| format!("Windows AI execution failed: {e}"))
    }

    /// Optimize server configuration using Windows AI
    #[cfg(all(target_os = "windows", feature = "windows-ai"))]
    pub async fn optimize_server_config(
        &self,
        server_name: &str,
    ) -> Result<EnhancedMcpServer, String> {
        let servers = self.active_servers.lock().await;
        let server = servers
            .get(server_name)
            .ok_or_else(|| format!("Server '{server_name}' not found"))?;

        let optimization_prompt = format!(
            "Optimize MCP server '{server_name}' configuration for Windows 11 25H2 AI acceleration. Current config: {server:?}"
        );

        let _optimized_config =
            execute_with_windows_ai(&optimization_prompt, &self.windows_ai_options)
                .await
                .map_err(|e| format!("AI optimization failed: {e}"))?;

        // Parse optimized config (simplified - in reality would need proper parsing)
        // For now, return the original server config
        Ok(server.clone())
    }

    /// Get Windows AI performance metrics
    #[cfg(all(target_os = "windows", feature = "windows-ai"))]
    pub async fn get_ai_performance_metrics(&self) -> Result<HashMap<String, f64>, String> {
        let mut metrics = HashMap::new();

        // Get GPU stats if available
        if self.windows_ai_options.use_gpu
            && let Ok(gpu_stats) = crate::windows_ai_integration::get_gpu_statistics().await {
                metrics.insert("gpu_utilization".to_string(), gpu_stats.utilization as f64);
                metrics.insert("gpu_memory_used".to_string(), gpu_stats.memory_used as f64);
                metrics.insert(
                    "gpu_memory_total".to_string(),
                    gpu_stats.memory_total as f64,
                );
            }

        metrics.insert(
            "windows_ai_enabled".to_string(),
            if self.windows_ai_options.enabled {
                1.0
            } else {
                0.0
            },
        );
        metrics.insert(
            "kernel_accelerated".to_string(),
            if self.windows_ai_options.kernel_accelerated {
                1.0
            } else {
                0.0
            },
        );

        Ok(metrics)
    }
}
