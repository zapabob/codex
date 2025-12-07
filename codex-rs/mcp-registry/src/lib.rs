//! MCP Registry for Windows 11 25H2
//!
//! Provides a secure, standardized registry for AI agents to discover
//! and connect to MCP servers on Windows 11.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use crate::Result;

/// MCP Server metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerInfo {
    /// Unique server identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Server description
    pub description: String,
    /// Version information
    pub version: String,
    /// Capabilities provided by this server
    pub capabilities: Vec<String>,
    /// Connection endpoint
    pub endpoint: String,
    /// Authentication requirements
    pub auth_required: bool,
    /// Security level
    pub security_level: SecurityLevel,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    /// Last seen timestamp
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

/// Security levels for MCP servers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// High trust - system services
    System,
    /// Medium trust - verified third-party
    Verified,
    /// Low trust - unverified
    Unverified,
}

/// Resource requirements for MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Memory requirement in MB
    pub memory_mb: u32,
    /// CPU cores required
    pub cpu_cores: f32,
    /// Network access required
    pub network_access: bool,
    /// File system access level
    pub filesystem_access: FilesystemAccess,
}

/// Filesystem access levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilesystemAccess {
    /// No file system access
    None,
    /// Read-only access to user directories
    ReadOnly,
    /// Full read-write access
    ReadWrite,
}

/// Agent registration info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistration {
    pub agent_id: String,
    pub name: String,
    pub capabilities: Vec<String>,
    pub trust_level: TrustLevel,
    pub permissions: Vec<String>,
}

/// Trust levels for agents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustLevel {
    System,
    Verified,
    User,
}

/// Registry commands
#[derive(Debug)]
pub enum RegistryCommand {
    RegisterServer {
        info: MCPServerInfo,
        response: oneshot::Sender<Result<String>>,
    },
    UnregisterServer {
        server_id: String,
        response: oneshot::Sender<Result<()>>,
    },
    DiscoverServers {
        capabilities: Option<Vec<String>>,
        security_level: Option<SecurityLevel>,
        response: oneshot::Sender<Result<Vec<MCPServerInfo>>>,
    },
    RegisterAgent {
        registration: AgentRegistration,
        response: oneshot::Sender<Result<String>>,
    },
    GetAgentPermissions {
        agent_id: String,
        response: oneshot::Sender<Result<Vec<String>>>,
    },
    HealthCheck {
        response: oneshot::Sender<Result<RegistryHealth>>,
    },
}

/// Registry health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryHealth {
    pub total_servers: usize,
    pub active_servers: usize,
    pub registered_agents: usize,
    pub uptime_seconds: u64,
}

/// Windows 11 25H2 MCP Registry
pub struct MCPRegistry {
    servers: Arc<Mutex<HashMap<String, MCPServerInfo>>>,
    agents: Arc<Mutex<HashMap<String, AgentRegistration>>>,
    command_tx: mpsc::UnboundedSender<RegistryCommand>,
    command_rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<RegistryCommand>>>>,
    start_time: chrono::DateTime<chrono::Utc>,
}

impl MCPRegistry {
    /// Create new MCP Registry instance
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Self {
            servers: Arc::new(Mutex::new(HashMap::new())),
            agents: Arc::new(Mutex::new(HashMap::new())),
            command_tx: tx,
            command_rx: Arc::new(Mutex::new(Some(rx))),
            start_time: chrono::Utc::now(),
        }
    }

    /// Register a Windows system MCP server
    pub async fn register_system_server(&self, server_type: SystemServerType) -> Result<String> {
        let server_info = self.create_system_server_info(server_type);
        self.register_server(server_info).await
    }

    /// Register an MCP server
    pub async fn register_server(&self, info: MCPServerInfo) -> Result<String> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(RegistryCommand::RegisterServer {
            info,
            response: tx,
        })?;

        rx.await?
    }

    /// Discover available MCP servers
    pub async fn discover_servers(
        &self,
        capabilities: Option<Vec<String>>,
        security_level: Option<SecurityLevel>,
    ) -> Result<Vec<MCPServerInfo>> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(RegistryCommand::DiscoverServers {
            capabilities,
            security_level,
            response: tx,
        })?;

        rx.await?
    }

    /// Register an AI agent
    pub async fn register_agent(&self, registration: AgentRegistration) -> Result<String> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(RegistryCommand::RegisterAgent {
            registration,
            response: tx,
        })?;

        rx.await?
    }

    /// Get agent permissions
    pub async fn get_agent_permissions(&self, agent_id: &str) -> Result<Vec<String>> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(RegistryCommand::GetAgentPermissions {
            agent_id: agent_id.to_string(),
            response: tx,
        })?;

        rx.await?
    }

    /// Get registry health
    pub async fn health_check(&self) -> Result<RegistryHealth> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(RegistryCommand::HealthCheck {
            response: tx,
        })?;

        rx.await?
    }

    /// Create system server info for Windows services
    fn create_system_server_info(&self, server_type: SystemServerType) -> MCPServerInfo {
        match server_type {
            SystemServerType::FileSystem => MCPServerInfo {
                id: "windows-filesystem".to_string(),
                name: "Windows File System".to_string(),
                description: "Provides secure access to Windows file system operations".to_string(),
                version: "1.0.0".to_string(),
                capabilities: vec![
                    "file.read".to_string(),
                    "file.write".to_string(),
                    "directory.list".to_string(),
                    "file.search".to_string(),
                ],
                endpoint: "mcp://windows/filesystem".to_string(),
                auth_required: true,
                security_level: SecurityLevel::System,
                resource_requirements: ResourceRequirements {
                    memory_mb: 50,
                    cpu_cores: 0.1,
                    network_access: false,
                    filesystem_access: FilesystemAccess::ReadWrite,
                },
                last_seen: chrono::Utc::now(),
            },
            SystemServerType::Windowing => MCPServerInfo {
                id: "windows-windowing".to_string(),
                name: "Windows Windowing System".to_string(),
                description: "Manages Windows UI and window operations".to_string(),
                version: "1.0.0".to_string(),
                capabilities: vec![
                    "window.create".to_string(),
                    "window.manage".to_string(),
                    "ui.interact".to_string(),
                ],
                endpoint: "mcp://windows/windowing".to_string(),
                auth_required: true,
                security_level: SecurityLevel::System,
                resource_requirements: ResourceRequirements {
                    memory_mb: 100,
                    cpu_cores: 0.2,
                    network_access: false,
                    filesystem_access: FilesystemAccess::None,
                },
                last_seen: chrono::Utc::now(),
            },
            SystemServerType::WSL => MCPServerInfo {
                id: "windows-wsl".to_string(),
                name: "Windows Subsystem for Linux".to_string(),
                description: "Provides access to WSL environments and operations".to_string(),
                version: "1.0.0".to_string(),
                capabilities: vec![
                    "wsl.execute".to_string(),
                    "wsl.manage".to_string(),
                    "linux.command".to_string(),
                ],
                endpoint: "mcp://windows/wsl".to_string(),
                auth_required: true,
                security_level: SecurityLevel::System,
                resource_requirements: ResourceRequirements {
                    memory_mb: 200,
                    cpu_cores: 0.5,
                    network_access: true,
                    filesystem_access: FilesystemAccess::ReadWrite,
                },
                last_seen: chrono::Utc::now(),
            },
        }
    }

    /// Run the registry service
    pub async fn run(mut self) -> Result<()> {
        let mut rx = self.command_rx.lock().unwrap().take().unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                RegistryCommand::RegisterServer { info, response } => {
                    let server_id = info.id.clone();
                    self.servers.lock().unwrap().insert(info.id.clone(), info);
                    let _ = response.send(Ok(server_id));
                }
                RegistryCommand::UnregisterServer { server_id, response } => {
                    self.servers.lock().unwrap().remove(&server_id);
                    let _ = response.send(Ok(()));
                }
                RegistryCommand::DiscoverServers { capabilities, security_level, response } => {
                    let servers = self.servers.lock().unwrap();
                    let filtered_servers: Vec<MCPServerInfo> = servers.values()
                        .filter(|server| {
                            // Filter by capabilities
                            if let Some(ref caps) = capabilities {
                                if !caps.iter().any(|cap| server.capabilities.contains(cap)) {
                                    return false;
                                }
                            }

                            // Filter by security level
                            if let Some(level) = security_level {
                                if server.security_level != level {
                                    return false;
                                }
                            }

                            true
                        })
                        .cloned()
                        .collect();

                    let _ = response.send(Ok(filtered_servers));
                }
                RegistryCommand::RegisterAgent { registration, response } => {
                    let agent_id = registration.agent_id.clone();
                    self.agents.lock().unwrap().insert(registration.agent_id.clone(), registration);
                    let _ = response.send(Ok(agent_id));
                }
                RegistryCommand::GetAgentPermissions { agent_id, response } => {
                    let permissions = self.agents.lock().unwrap()
                        .get(&agent_id)
                        .map(|agent| agent.permissions.clone())
                        .unwrap_or_default();
                    let _ = response.send(Ok(permissions));
                }
                RegistryCommand::HealthCheck { response } => {
                    let servers = self.servers.lock().unwrap();
                    let agents = self.agents.lock().unwrap();

                    let health = RegistryHealth {
                        total_servers: servers.len(),
                        active_servers: servers.values()
                            .filter(|s| s.last_seen > chrono::Utc::now() - chrono::Duration::minutes(5))
                            .count(),
                        registered_agents: agents.len(),
                        uptime_seconds: (chrono::Utc::now() - self.start_time).num_seconds() as u64,
                    };

                    let _ = response.send(Ok(health));
                }
            }
        }

        Ok(())
    }
}

/// Windows system server types
#[derive(Debug, Clone, Copy)]
pub enum SystemServerType {
    FileSystem,
    Windowing,
    WSL,
}

impl Default for MCPRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registry_operations() {
        let registry = MCPRegistry::new();

        // Register a system server
        let result = registry.register_system_server(SystemServerType::FileSystem).await;
        assert!(result.is_ok());

        // Discover servers
        let servers = registry.discover_servers(None, None).await;
        assert!(servers.is_ok());
        assert_eq!(servers.unwrap().len(), 1);

        // Register an agent
        let agent = AgentRegistration {
            agent_id: "test-agent".to_string(),
            name: "Test Agent".to_string(),
            capabilities: vec!["file.read".to_string()],
            trust_level: TrustLevel::User,
            permissions: vec!["read".to_string()],
        };

        let result = registry.register_agent(agent).await;
        assert!(result.is_ok());

        // Health check
        let health = registry.health_check().await;
        assert!(health.is_ok());
    }
}
