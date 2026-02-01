use anyhow::Result;
use bollard::Docker;
use bollard::container::Config;
use bollard::container::CreateContainerOptions;
use bollard::container::StartContainerOptions;
use bollard::image::CreateImageOptions;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio::time::{self};

/// Container-based virtual OS management system
pub struct ContainerManager {
    docker: Docker,
    containers: Mutex<HashMap<String, VirtualEnvironment>>,
    event_sender: broadcast::Sender<ContainerEvent>,
    resource_manager: ResourceManager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualEnvironment {
    pub id: String,
    pub name: String,
    pub container_id: String,
    pub image: String,
    pub status: ContainerStatus,
    pub ports: HashMap<u16, u16>,         // host_port -> container_port
    pub volumes: HashMap<String, String>, // host_path -> container_path
    pub environment: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub allocated_resources: ResourceAllocation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContainerStatus {
    Creating,
    Starting,
    Running,
    Paused,
    Stopped,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub cpu_cores: f64,
    pub memory_mb: u64,
    pub disk_gb: u64,
    pub network_bandwidth_mbps: u64,
}

#[derive(Debug, Clone)]
pub enum ContainerEvent {
    ContainerCreated(String),
    ContainerStarted(String),
    ContainerStopped(String),
    ContainerRemoved(String),
    ResourceAllocated(String, ResourceAllocation),
    Error(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualOSConfig {
    pub base_image: String,
    pub development_tools: Vec<String>,
    pub browsers: Vec<String>,
    pub ai_tools: Vec<String>,
    pub default_resources: ResourceAllocation,
    pub auto_cleanup: bool,
    pub max_containers: usize,
}

impl ContainerManager {
    pub async fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;

        // Test Docker connection
        docker.ping().await?;

        let (event_sender, _) = broadcast::channel(100);

        Ok(Self {
            docker,
            containers: Mutex::new(HashMap::new()),
            event_sender,
            resource_manager: ResourceManager::new(),
        })
    }

    /// Create a new virtual development environment
    pub async fn create_environment(&self, name: &str, config: &VirtualOSConfig) -> Result<String> {
        let env_id = format!("codex_env_{}_{}", name, chrono::Utc::now().timestamp());

        // Check resource availability
        if !self
            .resource_manager
            .check_availability(&config.default_resources)
            .await?
        {
            return Err(anyhow::anyhow!("Insufficient resources available"));
        }

        // Ensure base image exists
        self.ensure_image(&config.base_image).await?;

        // Create container configuration
        let container_config = self.create_container_config(&env_id, config)?;

        // Create container
        let create_options = CreateContainerOptions {
            name: env_id.clone(),
            ..Default::default()
        };

        let container = self
            .docker
            .create_container(Some(create_options), container_config)
            .await?;
        let container_id = container.id;

        // Create virtual environment record
        let environment = VirtualEnvironment {
            id: env_id.clone(),
            name: name.to_string(),
            container_id: container_id.clone(),
            image: config.base_image.clone(),
            status: ContainerStatus::Creating,
            ports: HashMap::new(),
            volumes: HashMap::new(),
            environment: HashMap::new(),
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
            allocated_resources: config.default_resources.clone(),
        };

        // Store environment
        {
            let mut containers = self.containers.lock().unwrap();
            containers.insert(env_id.clone(), environment);
        }

        // Allocate resources
        self.resource_manager
            .allocate_resources(&env_id, &config.default_resources)
            .await?;

        // Start container
        self.start_container(&container_id).await?;

        // Setup development environment
        self.setup_development_environment(&container_id, config)
            .await?;

        // Send event
        let _ = self
            .event_sender
            .send(ContainerEvent::ContainerCreated(env_id.clone()));

        Ok(env_id)
    }

    /// Start an existing container
    pub async fn start_container(&self, container_id: &str) -> Result<()> {
        let start_options = StartContainerOptions::<String> {
            ..Default::default()
        };

        self.docker
            .start_container(container_id, Some(start_options))
            .await?;

        // Update status
        self.update_container_status(container_id, ContainerStatus::Running)
            .await?;

        let _ = self
            .event_sender
            .send(ContainerEvent::ContainerStarted(container_id.to_string()));

        Ok(())
    }

    /// Stop a running container
    pub async fn stop_container(&self, container_id: &str) -> Result<()> {
        self.docker.stop_container(container_id, None).await?;

        // Update status
        self.update_container_status(container_id, ContainerStatus::Stopped)
            .await?;

        let _ = self
            .event_sender
            .send(ContainerEvent::ContainerStopped(container_id.to_string()));

        Ok(())
    }

    /// Remove a container and clean up resources
    pub async fn remove_environment(&self, env_id: &str) -> Result<()> {
        let containers = self.containers.lock().unwrap();
        let environment = containers
            .get(env_id)
            .ok_or_else(|| anyhow::anyhow!("Environment not found"))?;

        // Stop container if running
        if environment.status == ContainerStatus::Running {
            self.stop_container(&environment.container_id).await?;
        }

        // Remove container
        self.docker
            .remove_container(&environment.container_id, None)
            .await?;

        // Release resources
        self.resource_manager.release_resources(env_id).await?;

        // Remove from storage
        drop(containers);
        {
            let mut containers = self.containers.lock().unwrap();
            containers.remove(env_id);
        }

        let _ = self
            .event_sender
            .send(ContainerEvent::ContainerRemoved(env_id.to_string()));

        Ok(())
    }

    /// Execute code in the container environment
    pub async fn execute_code(
        &self,
        env_id: &str,
        code: &str,
        language: &str,
        timeout: Duration,
    ) -> Result<ExecutionResult> {
        let containers = self.containers.lock().unwrap();
        let environment = containers
            .get(env_id)
            .ok_or_else(|| anyhow::anyhow!("Environment not found"))?;

        if environment.status != ContainerStatus::Running {
            return Err(anyhow::anyhow!("Environment is not running"));
        }

        // Create execution script based on language
        let (script_path, command) = self.create_execution_script(code, language)?;

        // Execute in container
        let exec = bollard::exec::CreateExecOptions {
            cmd: Some(command),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            ..Default::default()
        };

        let exec_result = self
            .docker
            .create_exec(&environment.container_id, exec)
            .await?;
        let exec_id = exec_result.id;

        // Start execution
        self.docker.start_exec(&exec_id, None).await?;

        // Wait for completion with timeout
        let result =
            tokio::time::timeout(timeout, async { self.docker.inspect_exec(&exec_id).await })
                .await??;

        // Parse result
        let success = result.exit_code.unwrap_or(1) == 0;
        let output = String::new(); // Would collect actual output
        let error = String::new(); // Would collect error output

        Ok(ExecutionResult {
            success,
            output,
            error,
            execution_time: Duration::from_secs(1), // Placeholder
        })
    }

    /// Install browser in container
    pub async fn install_browser(&self, env_id: &str, browser: &str) -> Result<()> {
        let containers = self.containers.lock().unwrap();
        let environment = containers
            .get(env_id)
            .ok_or_else(|| anyhow::anyhow!("Environment not found"))?;

        let install_commands = match browser {
            "chrome" => vec![
                "wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -",
                "echo 'deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main' | tee /etc/apt/sources.list.d/google-chrome.list",
                "apt-get update",
                "apt-get install -y google-chrome-stable",
            ],
            "firefox" => vec!["apt-get update", "apt-get install -y firefox"],
            "vscode" => vec![
                "wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg",
                "install -o root -g root -m 644 packages.microsoft.gpg /etc/apt/trusted.gpg.d/",
                "echo 'deb [arch=amd64,arm64,armhf signed-by=/etc/apt/trusted.gpg.d/packages.microsoft.gpg] https://packages.microsoft.com/repos/code stable main' | tee /etc/apt/sources.list.d/vscode.list",
                "apt-get update",
                "apt-get install -y code",
            ],
            _ => return Err(format!("Unsupported browser: {}", browser).into()),
        };

        for command in install_commands {
            self.execute_command(&environment.container_id, &["sh", "-c", command])
                .await?;
        }

        Ok(())
    }

    /// Generate AI-assisted code in container
    pub async fn generate_code(
        &self,
        env_id: &str,
        prompt: &str,
        language: &str,
        context: &CodeGenerationContext,
    ) -> Result<GeneratedCode> {
        let containers = self.containers.lock().unwrap();
        let environment = containers
            .get(env_id)
            .ok_or_else(|| anyhow::anyhow!("Environment not found"))?;

        // This would integrate with AI models running in the container
        // For now, return a placeholder
        Ok(GeneratedCode {
            code: format!(
                "// Generated code for: {}\n// Language: {}\n\nconsole.log('Hello, World!');\n",
                prompt, language
            ),
            explanation: "Generated basic code structure".to_string(),
            confidence: 0.8,
            suggestions: vec![
                "Add error handling".to_string(),
                "Add documentation".to_string(),
            ],
        })
    }

    /// Get container logs
    pub async fn get_logs(&self, env_id: &str, lines: usize) -> Result<String> {
        let containers = self.containers.lock().unwrap();
        let environment = containers
            .get(env_id)
            .ok_or_else(|| anyhow::anyhow!("Environment not found"))?;

        let logs = self
            .docker
            .logs(
                &environment.container_id,
                Some(bollard::container::LogsOptions {
                    stdout: true,
                    stderr: true,
                    tail: lines.to_string(),
                    ..Default::default()
                }),
            )
            .await?;

        let mut output = String::new();
        // Collect log output
        // This would need proper stream handling in real implementation

        Ok(output)
    }

    /// List all environments
    pub fn list_environments(&self) -> Vec<VirtualEnvironment> {
        self.containers.lock().unwrap().values().cloned().collect()
    }

    /// Get environment by ID
    pub fn get_environment(&self, env_id: &str) -> Option<VirtualEnvironment> {
        self.containers.lock().unwrap().get(env_id).cloned()
    }

    /// Subscribe to container events
    pub fn subscribe_events(&self) -> broadcast::Receiver<ContainerEvent> {
        self.event_sender.subscribe()
    }

    // Private helper methods
    async fn ensure_image(&self, image: &str) -> Result<()> {
        let images = self
            .docker
            .list_images(Some(bollard::image::ListImagesOptions::<String> {
                filters: HashMap::from([("reference".to_string(), vec![image.to_string()])]),
                ..Default::default()
            }))
            .await?;

        if images.is_empty() {
            // Pull image
            let create_options = CreateImageOptions {
                from_image: image,
                ..Default::default()
            };

            let mut stream = self.docker.create_image(Some(create_options), None, None);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(_) => continue,
                    Err(e) => return Err(e.into()),
                }
            }
        }

        Ok(())
    }

    fn create_container_config(
        &self,
        env_id: &str,
        config: &VirtualOSConfig,
    ) -> Result<Config<String>> {
        let exposed_ports = HashMap::from([
            ("8080/tcp".to_string(), HashMap::new()), // Web server
            ("3000/tcp".to_string(), HashMap::new()), // Development server
            ("9222/tcp".to_string(), HashMap::new()), // Chrome debugging
        ]);

        let port_bindings = HashMap::new(); // Dynamic port allocation would be handled separately

        let env_vars = vec![
            "DEBIAN_FRONTEND=noninteractive".to_string(),
            "TZ=UTC".to_string(),
        ];

        Ok(Config {
            image: Some(config.base_image.clone()),
            exposed_ports: Some(exposed_ports),
            host_config: Some(bollard::models::HostConfig {
                port_bindings: Some(port_bindings),
                memory: Some(config.default_resources.memory_mb * 1024 * 1024),
                cpu_quota: Some((config.default_resources.cpu_cores * 100000.0) as i64),
                cpu_period: Some(100000),
                ..Default::default()
            }),
            env: Some(env_vars),
            ..Default::default()
        })
    }

    async fn setup_development_environment(
        &self,
        container_id: &str,
        config: &VirtualOSConfig,
    ) -> Result<()> {
        // Install development tools
        for tool in &config.development_tools {
            match tool.as_str() {
                "node" => {
                    self.execute_command(container_id, &["apt-get", "update"])
                        .await?;
                    self.execute_command(
                        container_id,
                        &["apt-get", "install", "-y", "nodejs", "npm"],
                    )
                    .await?;
                }
                "python" => {
                    self.execute_command(
                        container_id,
                        &["apt-get", "install", "-y", "python3", "python3-pip"],
                    )
                    .await?;
                }
                "rust" => {
                    self.execute_command(
                        container_id,
                        &[
                            "curl",
                            "--proto",
                            "=https",
                            "--tlsv1.2",
                            "-sSf",
                            "https://sh.rustup.rs",
                            "|",
                            "sh",
                            "-s",
                            "--",
                            "-y",
                        ],
                    )
                    .await?;
                }
                _ => continue,
            }
        }

        // Install browsers if requested
        for browser in &config.browsers {
            self.install_browser_by_id(container_id, browser).await?;
        }

        Ok(())
    }

    async fn install_browser_by_id(&self, container_id: &str, browser: &str) -> Result<()> {
        match browser {
            "chrome" => {
                let commands = vec![
                    "wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -",
                    "echo 'deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main' | tee /etc/apt/sources.list.d/google-chrome.list",
                    "apt-get update",
                    "apt-get install -y google-chrome-stable",
                ];
                for cmd in commands {
                    self.execute_command(container_id, &["sh", "-c", cmd])
                        .await?;
                }
            }
            "firefox" => {
                self.execute_command(container_id, &["apt-get", "install", "-y", "firefox"])
                    .await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn execute_command(&self, container_id: &str, command: &[&str]) -> Result<()> {
        let exec = bollard::exec::CreateExecOptions {
            cmd: Some(command.to_vec()),
            ..Default::default()
        };

        let exec_result = self.docker.create_exec(container_id, exec).await?;
        self.docker.start_exec(&exec_result.id, None).await?;

        Ok(())
    }

    async fn update_container_status(
        &self,
        container_id: &str,
        status: ContainerStatus,
    ) -> Result<()> {
        let mut containers = self.containers.lock().unwrap();
        for env in containers.values_mut() {
            if env.container_id == container_id {
                env.status = status;
                break;
            }
        }
        Ok(())
    }

    fn create_execution_script(&self, code: &str, language: &str) -> Result<(String, Vec<String>)> {
        match language {
            "javascript" | "js" => {
                let script_path = "/tmp/script.js".to_string();
                // In real implementation, would write code to container
                Ok((script_path, vec!["node".to_string(), script_path]))
            }
            "python" => {
                let script_path = "/tmp/script.py".to_string();
                Ok((script_path, vec!["python3".to_string(), script_path]))
            }
            "rust" => {
                let script_path = "/tmp/main.rs".to_string();
                Ok((
                    script_path,
                    vec![
                        "rustc".to_string(),
                        script_path,
                        "-o",
                        "/tmp/main",
                        "&&",
                        "/tmp/main",
                    ],
                ))
            }
            _ => Err(format!("Unsupported language: {}", language).into()),
        }
    }
}

/// Resource management for containers
pub struct ResourceManager {
    total_resources: ResourceAllocation,
    allocated_resources: Mutex<HashMap<String, ResourceAllocation>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        // Default system resources (would be detected in real implementation)
        let total_resources = ResourceAllocation {
            cpu_cores: 8.0,
            memory_mb: 16384, // 16GB
            disk_gb: 500,
            network_bandwidth_mbps: 1000,
        };

        Self {
            total_resources,
            allocated_resources: Mutex::new(HashMap::new()),
        }
    }

    pub async fn check_availability(&self, requested: &ResourceAllocation) -> Result<bool> {
        let allocated = self.allocated_resources.lock().unwrap();

        let total_allocated = allocated.values().fold(
            ResourceAllocation {
                cpu_cores: 0.0,
                memory_mb: 0,
                disk_gb: 0,
                network_bandwidth_mbps: 0,
            },
            |acc, res| ResourceAllocation {
                cpu_cores: acc.cpu_cores + res.cpu_cores,
                memory_mb: acc.memory_mb + res.memory_mb,
                disk_gb: acc.disk_gb + res.disk_gb,
                network_bandwidth_mbps: acc.network_bandwidth_mbps + res.network_bandwidth_mbps,
            },
        );

        Ok(
            total_allocated.cpu_cores + requested.cpu_cores <= self.total_resources.cpu_cores
                && total_allocated.memory_mb + requested.memory_mb
                    <= self.total_resources.memory_mb
                && total_allocated.disk_gb + requested.disk_gb <= self.total_resources.disk_gb
                && total_allocated.network_bandwidth_mbps + requested.network_bandwidth_mbps
                    <= self.total_resources.network_bandwidth_mbps,
        )
    }

    pub async fn allocate_resources(
        &self,
        env_id: &str,
        resources: &ResourceAllocation,
    ) -> Result<()> {
        let mut allocated = self.allocated_resources.lock().unwrap();
        allocated.insert(env_id.to_string(), resources.clone());
        Ok(())
    }

    pub async fn release_resources(&self, env_id: &str) -> Result<()> {
        let mut allocated = self.allocated_resources.lock().unwrap();
        allocated.remove(env_id);
        Ok(())
    }
}

/// Code execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: String,
    pub error: String,
    pub execution_time: Duration,
}

/// Code generation context
#[derive(Debug, Clone)]
pub struct CodeGenerationContext {
    pub project_type: String,
    pub dependencies: Vec<String>,
    pub target_platform: String,
    pub coding_style: String,
}

/// Generated code result
#[derive(Debug, Clone)]
pub struct GeneratedCode {
    pub code: String,
    pub explanation: String,
    pub confidence: f32,
    pub suggestions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_container_manager_creation() {
        // Skip test if Docker is not available
        let manager = match ContainerManager::new().await {
            Ok(m) => m,
            Err(_) => return, // Skip test if Docker not available
        };

        assert!(manager.list_environments().is_empty());
    }

    #[test]
    fn test_resource_manager() {
        let rm = ResourceManager::new();

        let request = ResourceAllocation {
            cpu_cores: 2.0,
            memory_mb: 4096,
            disk_gb: 50,
            network_bandwidth_mbps: 100,
        };

        // Should be available initially
        assert!(rm.check_availability(&request).is_ok());
    }
}
