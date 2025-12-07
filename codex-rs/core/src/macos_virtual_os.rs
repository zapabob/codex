//! MacOS-style Virtual Operating System for AI Development
//!
//! Features:
//! - CLI-based application creation
//! - Explicit shell command execution prevention
//! - YOLO mode with full file access
//! - GitHub integration (commits, PRs, issues)
//! - Webhook support for external services

use crate::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// Virtual OS execution mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VirtualOSMode {
    /// Safe mode - restricted operations
    Safe,
    /// YOLO mode - full access with monitoring
    Yolo,
    /// Developer mode - advanced features enabled
    Developer,
}

/// Application template for CLI creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTemplate {
    pub name: String,
    pub description: String,
    pub category: AppCategory,
    pub framework: AppFramework,
    pub dependencies: Vec<String>,
    pub files: HashMap<String, String>,
}

/// Application categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppCategory {
    Web,
    Desktop,
    CLI,
    API,
    Library,
    Game,
}

/// Supported frameworks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppFramework {
    React,
    Vue,
    Angular,
    Svelte,
    NextJs,
    NuxtJs,
    Tauri,
    Electron,
    Rust,
    Python,
    NodeJs,
    Go,
    Custom(String),
}

/// Virtual OS command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VirtualOSCommand {
    CreateApp {
        name: String,
        template: AppTemplate,
        path: PathBuf,
    },
    GitCommit {
        message: String,
        files: Vec<PathBuf>,
    },
    CreatePR {
        title: String,
        description: String,
        base_branch: String,
        head_branch: String,
    },
    CreateIssue {
        title: String,
        description: String,
        labels: Vec<String>,
    },
    SendWebhook {
        service: WebhookService,
        payload: serde_json::Value,
    },
    ExecuteSafeCommand {
        command: String,
        args: Vec<String>,
        cwd: Option<PathBuf>,
    },
}

/// Webhook service types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebhookService {
    GitHub,
    Slack,
    Line,
}

/// Virtual OS execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

/// Security monitor for dangerous operations
#[derive(Debug)]
pub struct SecurityMonitor {
    dangerous_commands: HashSet<String>,
    dangerous_patterns: Vec<regex::Regex>,
    file_deletion_prevention: bool,
}

impl SecurityMonitor {
    pub fn new() -> Self {
        let mut dangerous_commands = HashSet::new();
        dangerous_commands.extend(
            [
                "rm", "rmdir", "del", "erase", "format", "fdisk", "mkfs", "dd", "shred", "wipe",
                "srm", "sudo", "su", "chmod +x", "curl", "wget", "ssh", "scp", "ftp", "telnet",
            ]
            .iter()
            .map(|s| s.to_string()),
        );

        let dangerous_patterns = vec![
            regex::Regex::new(r"rm\s+-rf\s+/").unwrap(),
            regex::Regex::new(r"dd\s+if=").unwrap(),
            regex::Regex::new(r">\s*/dev/").unwrap(),
            regex::Regex::new(r"chmod\s+777").unwrap(),
        ];

        Self {
            dangerous_commands,
            dangerous_patterns,
            file_deletion_prevention: true,
        }
    }

    pub fn is_command_safe(&self, command: &str, args: &[String]) -> Result<bool> {
        let full_command = format!("{} {}", command, args.join(" "));

        // Check dangerous commands
        if self.dangerous_commands.contains(command) {
            return Ok(false);
        }

        // Check dangerous patterns
        for pattern in &self.dangerous_patterns {
            if pattern.is_match(&full_command) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn is_file_operation_safe(&self, operation: &FileOperation) -> bool {
        match operation {
            FileOperation::Delete(_) if self.file_deletion_prevention => false,
            FileOperation::Write(path) if self.is_system_file(path) => false,
            _ => true,
        }
    }

    fn is_system_file(&self, path: &Path) -> bool {
        let system_paths = [
            "/etc",
            "/usr",
            "/bin",
            "/sbin",
            "/boot",
            "/sys",
            "/proc",
            "/dev",
            "C:\\Windows",
            "C:\\System32",
            "C:\\Program Files",
        ];

        path.to_string_lossy()
            .to_string()
            .starts_with(|p: &str| system_paths.iter().any(|sys| p.starts_with(sys)))
    }
}

/// File operation types
#[derive(Debug)]
pub enum FileOperation {
    Read(PathBuf),
    Write(PathBuf),
    Delete(PathBuf),
    Create(PathBuf),
}

/// MacOS-style Virtual OS
pub struct MacOSVirtualOS {
    mode: VirtualOSMode,
    security_monitor: Arc<SecurityMonitor>,
    templates: HashMap<String, AppTemplate>,
    webhooks: HashMap<String, String>,
    command_tx: mpsc::UnboundedSender<VirtualOSCommand>,
    command_rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<VirtualOSCommand>>>>,
}

impl MacOSVirtualOS {
    pub fn new(mode: VirtualOSMode) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut templates = HashMap::new();
        Self::load_default_templates(&mut templates);

        Self {
            mode,
            security_monitor: Arc::new(SecurityMonitor::new()),
            templates,
            webhooks: HashMap::new(),
            command_tx: tx,
            command_rx: Arc::new(Mutex::new(Some(rx))),
        }
    }

    /// Load default application templates
    fn load_default_templates(templates: &mut HashMap<String, AppTemplate>) {
        // React Web App template
        let react_template = AppTemplate {
            name: "React Web App".to_string(),
            description: "Modern React application with TypeScript".to_string(),
            category: AppCategory::Web,
            framework: AppFramework::React,
            dependencies: vec![
                "react".to_string(),
                "react-dom".to_string(),
                "typescript".to_string(),
                "vite".to_string(),
            ],
            files: HashMap::new(),
        };
        templates.insert("react-web".to_string(), react_template);

        // Tauri Desktop App template
        let tauri_template = AppTemplate {
            name: "Tauri Desktop App".to_string(),
            description: "Cross-platform desktop app with Rust backend".to_string(),
            category: AppCategory::Desktop,
            framework: AppFramework::Tauri,
            dependencies: vec![
                "tauri".to_string(),
                "tauri-build".to_string(),
                "@tauri-apps/api".to_string(),
            ],
            files: HashMap::new(),
        };
        templates.insert("tauri-desktop".to_string(), tauri_template);

        // CLI Tool template
        let cli_template = AppTemplate {
            name: "Rust CLI Tool".to_string(),
            description: "Command-line tool in Rust".to_string(),
            category: AppCategory::CLI,
            framework: AppFramework::Rust,
            dependencies: vec!["clap".to_string(), "anyhow".to_string()],
            files: HashMap::new(),
        };
        templates.insert("rust-cli".to_string(), cli_template);
    }

    /// Create application from template
    pub async fn create_app(
        &self,
        name: &str,
        template_name: &str,
        path: PathBuf,
    ) -> Result<ExecutionResult> {
        let template = self
            .templates
            .get(template_name)
            .ok_or_else(|| format!("Template '{}' not found", template_name))?;

        let (tx, rx) = oneshot::channel();

        self.command_tx.send(VirtualOSCommand::CreateApp {
            name: name.to_string(),
            template: template.clone(),
            path,
        })?;

        rx.await?
    }

    /// Execute safe command with security monitoring
    pub async fn execute_safe_command(
        &self,
        command: &str,
        args: Vec<String>,
        cwd: Option<PathBuf>,
    ) -> Result<ExecutionResult> {
        // Security check
        if !self.security_monitor.is_command_safe(command, &args)? {
            return Ok(ExecutionResult {
                success: false,
                output: String::new(),
                error: Some("Command execution blocked by security monitor".to_string()),
                execution_time_ms: 0,
            });
        }

        let (tx, rx) = oneshot::channel();

        self.command_tx.send(VirtualOSCommand::ExecuteSafeCommand {
            command: command.to_string(),
            args,
            cwd,
        })?;

        rx.await?
    }

    /// Git operations
    pub async fn git_commit(&self, message: &str, files: Vec<PathBuf>) -> Result<ExecutionResult> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(VirtualOSCommand::GitCommit {
            message: message.to_string(),
            files,
        })?;

        rx.await?
    }

    /// Create GitHub PR
    pub async fn create_pr(
        &self,
        title: &str,
        description: &str,
        base_branch: &str,
        head_branch: &str,
    ) -> Result<ExecutionResult> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(VirtualOSCommand::CreatePR {
            title: title.to_string(),
            description: description.to_string(),
            base_branch: base_branch.to_string(),
            head_branch: head_branch.to_string(),
        })?;

        rx.await?
    }

    /// Create GitHub issue
    pub async fn create_issue(
        &self,
        title: &str,
        description: &str,
        labels: Vec<String>,
    ) -> Result<ExecutionResult> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(VirtualOSCommand::CreateIssue {
            title: title.to_string(),
            description: description.to_string(),
            labels,
        })?;

        rx.await?
    }

    /// Send webhook
    pub async fn send_webhook(
        &self,
        service: WebhookService,
        payload: serde_json::Value,
    ) -> Result<ExecutionResult> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(VirtualOSCommand::SendWebhook { service, payload })?;

        rx.await?
    }

    /// Configure webhook URL
    pub fn set_webhook_url(&mut self, service: WebhookService, url: String) {
        let key = match service {
            WebhookService::GitHub => "github",
            WebhookService::Slack => "slack",
            WebhookService::Line => "line",
        };
        self.webhooks.insert(key.to_string(), url);
    }

    /// Get available templates
    pub fn get_templates(&self) -> Vec<&AppTemplate> {
        self.templates.values().collect()
    }

    /// Check if file operation is allowed
    pub fn is_file_operation_allowed(&self, operation: &FileOperation) -> bool {
        match self.mode {
            VirtualOSMode::Safe => self.security_monitor.is_file_operation_safe(operation),
            VirtualOSMode::Yolo => true, // YOLO mode allows everything
            VirtualOSMode::Developer => true,
        }
    }

    /// Run the virtual OS
    pub async fn run(self) -> Result<()> {
        let mut rx = self.command_rx.lock().unwrap().take().unwrap();

        while let Some(cmd) = rx.recv().await {
            let result = match cmd {
                VirtualOSCommand::CreateApp {
                    name,
                    template,
                    path,
                } => self.handle_create_app(name, template, path).await,
                VirtualOSCommand::ExecuteSafeCommand { command, args, cwd } => {
                    self.handle_execute_command(command, args, cwd).await
                }
                VirtualOSCommand::GitCommit { message, files } => {
                    self.handle_git_commit(message, files).await
                }
                VirtualOSCommand::CreatePR {
                    title,
                    description,
                    base_branch,
                    head_branch,
                } => {
                    self.handle_create_pr(title, description, base_branch, head_branch)
                        .await
                }
                VirtualOSCommand::CreateIssue {
                    title,
                    description,
                    labels,
                } => self.handle_create_issue(title, description, labels).await,
                VirtualOSCommand::SendWebhook { service, payload } => {
                    self.handle_send_webhook(service, payload).await
                }
            };

            // Send result back if needed
            let _ = result; // In real implementation, send back to caller
        }

        Ok(())
    }

    async fn handle_create_app(
        &self,
        name: String,
        template: AppTemplate,
        path: PathBuf,
    ) -> ExecutionResult {
        let start_time = std::time::Instant::now();

        match self.create_app_from_template(&name, &template, &path).await {
            Ok(_) => ExecutionResult {
                success: true,
                output: format!("Application '{}' created successfully", name),
                error: None,
                execution_time_ms: start_time.elapsed().as_millis() as u64,
            },
            Err(e) => ExecutionResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: start_time.elapsed().as_millis() as u64,
            },
        }
    }

    async fn create_app_from_template(
        &self,
        name: &str,
        template: &AppTemplate,
        path: &Path,
    ) -> Result<()> {
        let app_path = path.join(name);

        // Create directory
        tokio::fs::create_dir_all(&app_path).await?;

        // Generate files based on template
        match template.framework {
            AppFramework::React => self.generate_react_app(&app_path, name).await?,
            AppFramework::Tauri => self.generate_tauri_app(&app_path, name).await?,
            AppFramework::Rust => self.generate_rust_app(&app_path, name).await?,
            _ => {} // Handle other frameworks
        }

        Ok(())
    }

    async fn generate_react_app(&self, path: &Path, name: &str) -> Result<()> {
        // Create package.json
        let package_json = serde_json::json!({
            "name": name,
            "version": "0.1.0",
            "scripts": {
                "dev": "vite",
                "build": "vite build",
                "preview": "vite preview"
            },
            "dependencies": {
                "react": "^18.2.0",
                "react-dom": "^18.2.0"
            },
            "devDependencies": {
                "@types/react": "^18.2.0",
                "@types/react-dom": "^18.2.0",
                "typescript": "^5.0.0",
                "vite": "^4.3.0"
            }
        });

        let package_path = path.join("package.json");
        tokio::fs::write(package_path, serde_json::to_string_pretty(&package_json)?).await?;

        Ok(())
    }

    async fn generate_tauri_app(&self, path: &Path, name: &str) -> Result<()> {
        // Generate both frontend and Rust backend for Tauri
        self.generate_react_app(path, name).await?;

        // Create src-tauri directory and Cargo.toml
        let tauri_path = path.join("src-tauri");
        tokio::fs::create_dir_all(&tauri_path).await?;

        let cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = {{ version = "1.5", features = [] }}

[dependencies]
serde_json = "1.0"
serde = {{ version = "1.0", features = ["derive"] }}
tauri = {{ version = "1.5", features = ["shell-open"] }}

[features]
# this feature is used for production builds or when `devPath` points to the filesystem
# DO NOT remove this
custom-protocol = ["tauri/custom-protocol"]
"#,
            name
        );

        let cargo_path = tauri_path.join("Cargo.toml");
        tokio::fs::write(cargo_path, cargo_toml).await?;

        Ok(())
    }

    async fn generate_rust_app(&self, path: &Path, name: &str) -> Result<()> {
        let cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = {{ version = "4.0", features = ["derive"] }}
anyhow = "1.0"
"#,
            name
        );

        let cargo_path = path.join("Cargo.toml");
        tokio::fs::write(cargo_path, cargo_toml).await?;

        let main_rs = r#"use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name to greet
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let name = args.name.unwrap_or("World".to_string());
    println!("Hello, {}!", name);

    Ok(())
}
"#;

        let src_path = path.join("src");
        tokio::fs::create_dir_all(&src_path).await?;
        let main_path = src_path.join("main.rs");
        tokio::fs::write(main_path, main_rs).await?;

        Ok(())
    }

    async fn handle_execute_command(
        &self,
        command: String,
        args: Vec<String>,
        cwd: Option<PathBuf>,
    ) -> ExecutionResult {
        let start_time = std::time::Instant::now();

        // In real implementation, this would execute the command safely
        // For now, just simulate
        ExecutionResult {
            success: true,
            output: format!("Command '{}' executed successfully", command),
            error: None,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        }
    }

    async fn handle_git_commit(&self, message: String, files: Vec<PathBuf>) -> ExecutionResult {
        let start_time = std::time::Instant::now();

        // GitHub API integration would go here
        ExecutionResult {
            success: true,
            output: format!("Committed {} files with message: {}", files.len(), message),
            error: None,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        }
    }

    async fn handle_create_pr(
        &self,
        title: String,
        description: String,
        base_branch: String,
        head_branch: String,
    ) -> ExecutionResult {
        let start_time = std::time::Instant::now();

        // GitHub API integration would go here
        ExecutionResult {
            success: true,
            output: format!("PR created: {} ({})", title, head_branch),
            error: None,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        }
    }

    async fn handle_create_issue(
        &self,
        title: String,
        description: String,
        labels: Vec<String>,
    ) -> ExecutionResult {
        let start_time = std::time::Instant::now();

        // GitHub API integration would go here
        ExecutionResult {
            success: true,
            output: format!("Issue created: {} (labels: {})", title, labels.join(", ")),
            error: None,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        }
    }

    async fn handle_send_webhook(
        &self,
        service: WebhookService,
        payload: serde_json::Value,
    ) -> ExecutionResult {
        let start_time = std::time::Instant::now();

        let service_name = match service {
            WebhookService::GitHub => "GitHub",
            WebhookService::Slack => "Slack",
            WebhookService::Line => "LINE",
        };

        // Webhook integration would go here
        ExecutionResult {
            success: true,
            output: format!("Webhook sent to {}: {}", service_name, payload),
            error: None,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        }
    }
}

impl Default for MacOSVirtualOS {
    fn default() -> Self {
        Self::new(VirtualOSMode::Safe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_react_app() {
        let vos = MacOSVirtualOS::new(VirtualOSMode::Developer);
        let temp_dir = tempfile::tempdir().unwrap();

        let result = vos
            .create_app("test-app", "react-web", temp_dir.path().to_path_buf())
            .await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_security_monitor() {
        let monitor = SecurityMonitor::new();

        // Safe commands
        assert!(monitor.is_command_safe("ls", &[]).unwrap());
        assert!(
            monitor
                .is_command_safe("cat", &["file.txt".to_string()])
                .unwrap()
        );

        // Dangerous commands
        assert!(
            !monitor
                .is_command_safe("rm", &["-rf".to_string(), "/".to_string()])
                .unwrap()
        );
        assert!(!monitor.is_command_safe("sudo", &[]).unwrap());
    }
}
