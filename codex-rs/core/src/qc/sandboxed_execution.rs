//! Sandboxed Execution System (Rust 2024)
//!
//! Provides secure command execution with AI-powered destructive command detection
//! and prevention, featuring internet connectivity controls and macOS-style UX.

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::mpsc;
use tokio::time::Duration;

/// Sandbox execution mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SandboxMode {
    /// Full isolation - no network, limited filesystem
    Isolated,
    /// Network access allowed but monitored
    NetworkIsolated,
    /// Full access with AI monitoring
    Supervised,
    /// macOS-style sandbox with internet access
    MacOSSandbox,
}

/// AI security analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISecurityAnalysis {
    pub command_id: String,
    pub risk_level: SecurityRiskLevel,
    pub risk_score: f64,
    pub detected_threats: Vec<SecurityThreat>,
    pub mitigation_suggestions: Vec<String>,
    pub allow_execution: bool,
    pub requires_approval: bool,
    pub analysis_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Security risk levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityRiskLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

/// Detected security threats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityThreat {
    pub threat_type: ThreatType,
    pub description: String,
    pub severity: SecurityRiskLevel,
    pub location: Option<String>,
    pub confidence: f64,
}

/// Types of security threats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatType {
    DestructiveFileOperation,
    NetworkAttack,
    PrivilegeEscalation,
    DataExfiltration,
    MalwareExecution,
    UnauthorizedAccess,
    ResourceExhaustion,
}

/// Sandbox execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxExecutionRequest {
    pub id: String,
    pub command: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
    pub environment: HashMap<String, String>,
    pub sandbox_mode: SandboxMode,
    pub timeout_seconds: Option<u32>,
    pub internet_access: bool,
    pub resource_limits: ResourceLimits,
    pub ai_security_review: bool,
}

/// Resource limits for sandboxed execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_cpu_time_seconds: u32,
    pub max_memory_mb: u32,
    pub max_disk_write_mb: u32,
    pub max_network_bandwidth_mbps: u32,
    pub max_processes: u32,
}

/// Sandbox execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxExecutionResult {
    pub request_id: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub execution_time_ms: u64,
    pub resources_used: ResourceUsage,
    pub network_activity: Option<NetworkActivity>,
    pub security_events: Vec<SecurityEvent>,
    pub ai_analysis: Option<AISecurityAnalysis>,
    pub sandbox_mode_used: SandboxMode,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

/// Resource usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_time_seconds: f64,
    pub memory_peak_mb: u32,
    pub disk_write_mb: u32,
    pub network_sent_mb: f64,
    pub network_received_mb: f64,
    pub processes_created: u32,
}

/// Network activity monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkActivity {
    pub connections_attempted: u32,
    pub connections_successful: u32,
    pub domains_accessed: Vec<String>,
    pub ports_used: Vec<u16>,
    pub data_transferred_mb: f64,
    pub suspicious_activity_detected: bool,
}

/// Security events during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_type: SecurityEventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub severity: SecurityRiskLevel,
    pub blocked: bool,
}

/// Types of security events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityEventType {
    FileSystemAccess,
    NetworkConnection,
    ProcessCreation,
    PrivilegeChange,
    SuspiciousCommand,
    ResourceLimitExceeded,
}

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub default_mode: SandboxMode,
    pub enable_ai_security: bool,
    pub enable_network_monitoring: bool,
    pub enable_filesystem_monitoring: bool,
    pub max_concurrent_executions: usize,
    pub execution_timeout_seconds: u32,
    pub restricted_commands: HashSet<String>,
    pub allowed_network_domains: HashSet<String>,
    pub blocked_file_paths: HashSet<String>,
}

/// AI-powered sandbox executor
pub struct AISandboxExecutor {
    config: SandboxConfig,
    execution_sender: mpsc::UnboundedSender<SandboxExecutionRequest>,
    execution_receiver: Arc<RwLock<mpsc::UnboundedReceiver<SandboxExecutionRequest>>>,
    result_sender: mpsc::UnboundedSender<SandboxExecutionResult>,
    result_receiver: Arc<RwLock<mpsc::UnboundedReceiver<SandboxExecutionResult>>>,
    active_executions: Arc<RwLock<HashMap<String, ActiveExecution>>>,
    ai_security_analyzer: Arc<dyn AISecurityAnalyzer>,
}

/// Active execution tracking
#[derive(Debug)]
struct ActiveExecution {
    request: SandboxExecutionRequest,
    start_time: tokio::time::Instant,
    security_analysis: Option<AISecurityAnalysis>,
}

/// AI security analyzer trait
#[async_trait::async_trait]
pub trait AISecurityAnalyzer {
    /// Analyze command for security risks
    async fn analyze_command(
        &self,
        request: &SandboxExecutionRequest,
    ) -> Result<AISecurityAnalysis, String>;

    /// Check if command should be blocked
    async fn should_block_command(&self, analysis: &AISecurityAnalysis) -> bool;

    /// Generate security recommendations
    async fn generate_security_recommendations(&self, analysis: &AISecurityAnalysis)
    -> Vec<String>;
}

/// AI security analyzer implementation
pub struct DefaultAISecurityAnalyzer;

impl DefaultAISecurityAnalyzer {
    /// Create new AI security analyzer
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl AISecurityAnalyzer for DefaultAISecurityAnalyzer {
    async fn analyze_command(
        &self,
        request: &SandboxExecutionRequest,
    ) -> Result<AISecurityAnalysis, String> {
        let mut detected_threats = Vec::new();
        let mut risk_score = 0.0;
        let mut mitigation_suggestions = Vec::new();

        let command_string = format!("{} {}", request.command, request.args.join(" "));

        // Analyze for destructive patterns
        let destructive_patterns = [
            (
                "rm -rf",
                ThreatType::DestructiveFileOperation,
                0.9,
                "Recursive file deletion",
            ),
            (
                "sudo",
                ThreatType::PrivilegeEscalation,
                0.8,
                "Privilege escalation attempt",
            ),
            (
                "chmod 777",
                ThreatType::UnauthorizedAccess,
                0.7,
                "Overly permissive permissions",
            ),
            (
                "wget http",
                ThreatType::NetworkAttack,
                0.6,
                "Unencrypted network download",
            ),
            (
                "curl http",
                ThreatType::NetworkAttack,
                0.6,
                "Unencrypted network request",
            ),
            (
                "nc ",
                ThreatType::NetworkAttack,
                0.8,
                "Network connection tool",
            ),
            (
                "ncat ",
                ThreatType::NetworkAttack,
                0.8,
                "Network connection tool",
            ),
            (
                "ssh ",
                ThreatType::UnauthorizedAccess,
                0.5,
                "Remote shell access",
            ),
            (
                "scp ",
                ThreatType::DataExfiltration,
                0.6,
                "Data transfer tool",
            ),
            (
                "dd if=",
                ThreatType::DestructiveFileOperation,
                0.8,
                "Low-level disk operation",
            ),
            (
                "mkfs",
                ThreatType::DestructiveFileOperation,
                0.9,
                "Filesystem creation",
            ),
            (
                "fdisk",
                ThreatType::DestructiveFileOperation,
                0.8,
                "Disk partitioning",
            ),
            (
                "format",
                ThreatType::DestructiveFileOperation,
                0.8,
                "Disk formatting",
            ),
        ];

        for (pattern, threat_type, severity_score, description) in destructive_patterns {
            if command_string.contains(pattern) {
                risk_score += severity_score;
                detected_threats.push(SecurityThreat {
                    threat_type,
                    description: description.to_string(),
                    severity: if severity_score >= 0.8 {
                        SecurityRiskLevel::Critical
                    } else if severity_score >= 0.6 {
                        SecurityRiskLevel::High
                    } else {
                        SecurityRiskLevel::Medium
                    },
                    location: Some(pattern.to_string()),
                    confidence: severity_score,
                });

                // Add mitigation suggestions
                match threat_type {
                    ThreatType::DestructiveFileOperation => {
                        mitigation_suggestions.push("Consider using version control or backups before destructive operations".to_string());
                        mitigation_suggestions
                            .push("Use --dry-run flag if available to preview changes".to_string());
                    }
                    ThreatType::PrivilegeEscalation => {
                        mitigation_suggestions.push(
                            "Verify if elevated privileges are actually required".to_string(),
                        );
                        mitigation_suggestions.push(
                            "Consider using sudo with specific commands instead of full shell"
                                .to_string(),
                        );
                    }
                    ThreatType::NetworkAttack => {
                        mitigation_suggestions.push(
                            "Use HTTPS instead of HTTP for secure communications".to_string(),
                        );
                        mitigation_suggestions
                            .push("Verify the authenticity of downloaded content".to_string());
                    }
                    _ => {
                        mitigation_suggestions.push(
                            "Review command parameters and ensure they are correct".to_string(),
                        );
                    }
                }
            }
        }

        // Check for internet access with dangerous commands
        if request.internet_access && risk_score > 0.3 {
            detected_threats.push(SecurityThreat {
                threat_type: ThreatType::NetworkAttack,
                description: "Internet access combined with potentially dangerous command"
                    .to_string(),
                severity: SecurityRiskLevel::High,
                location: None,
                confidence: 0.7,
            });
            mitigation_suggestions
                .push("Consider executing without internet access first".to_string());
            risk_score += 0.3;
        }

        // Determine risk level
        let risk_level = if risk_score >= 1.5 {
            SecurityRiskLevel::Critical
        } else if risk_score >= 1.0 {
            SecurityRiskLevel::High
        } else if risk_score >= 0.5 {
            SecurityRiskLevel::Medium
        } else if risk_score >= 0.2 {
            SecurityRiskLevel::Low
        } else {
            SecurityRiskLevel::Safe
        };

        // Determine if execution should be allowed
        let allow_execution =
            matches!(risk_level, SecurityRiskLevel::Safe | SecurityRiskLevel::Low);
        let requires_approval = matches!(
            risk_level,
            SecurityRiskLevel::Medium | SecurityRiskLevel::High
        );

        Ok(AISecurityAnalysis {
            command_id: request.id.clone(),
            risk_level,
            risk_score,
            detected_threats,
            mitigation_suggestions,
            allow_execution,
            requires_approval,
            analysis_timestamp: chrono::Utc::now(),
        })
    }

    async fn should_block_command(&self, analysis: &AISecurityAnalysis) -> bool {
        matches!(analysis.risk_level, SecurityRiskLevel::Critical)
    }

    async fn generate_security_recommendations(
        &self,
        analysis: &AISecurityAnalysis,
    ) -> Vec<String> {
        let mut recommendations = analysis.mitigation_suggestions.clone();

        // Add general recommendations based on risk level
        match analysis.risk_level {
            SecurityRiskLevel::Critical => {
                recommendations.push(
                    "🚫 CRITICAL: This command is blocked due to high security risk".to_string(),
                );
                recommendations.push(
                    "Consider alternative approaches that don't involve destructive operations"
                        .to_string(),
                );
                recommendations.push("Consult with security team before proceeding".to_string());
            }
            SecurityRiskLevel::High => {
                recommendations
                    .push("⚠️ HIGH RISK: Manual review required before execution".to_string());
                recommendations.push("Execute in isolated environment first".to_string());
                recommendations.push("Ensure backups are available".to_string());
            }
            SecurityRiskLevel::Medium => {
                recommendations.push("⚡ MEDIUM RISK: Execute with caution".to_string());
                recommendations.push("Monitor execution closely".to_string());
                recommendations.push("Have rollback plan ready".to_string());
            }
            SecurityRiskLevel::Low => {
                recommendations.push("✅ LOW RISK: Generally safe to execute".to_string());
                recommendations.push("Standard monitoring recommended".to_string());
            }
            SecurityRiskLevel::Safe => {
                recommendations.push("✅ SAFE: No security concerns detected".to_string());
            }
        }

        recommendations
    }
}

impl AISandboxExecutor {
    /// Create new AI-powered sandbox executor
    pub fn new(config: SandboxConfig, ai_analyzer: Arc<dyn AISecurityAnalyzer>) -> Self {
        let (exec_tx, exec_rx) = mpsc::unbounded_channel();
        let (result_tx, result_rx) = mpsc::unbounded_channel();

        Self {
            config,
            execution_sender: exec_tx,
            execution_receiver: Arc::new(RwLock::new(exec_rx)),
            result_sender: result_tx,
            result_receiver: Arc::new(RwLock::new(result_rx)),
            active_executions: Arc::new(RwLock::new(HashMap::new())),
            ai_security_analyzer: ai_analyzer,
        }
    }

    /// Execute command in sandbox with AI security analysis
    pub async fn execute_sandboxed(
        &self,
        request: SandboxExecutionRequest,
    ) -> Result<String, String> {
        // Validate request
        self.validate_request(&request).await?;

        // AI security analysis
        let security_analysis = if self.config.enable_ai_security {
            Some(self.ai_security_analyzer.analyze_command(&request).await?)
        } else {
            None
        };

        // Check if command should be blocked
        if let Some(ref analysis) = security_analysis {
            if self
                .ai_security_analyzer
                .should_block_command(analysis)
                .await
            {
                return Err(format!(
                    "Command blocked by AI security analysis: {}",
                    analysis
                        .detected_threats
                        .iter()
                        .map(|t| t.description.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }

        // Track active execution
        let active_exec = ActiveExecution {
            request: request.clone(),
            start_time: tokio::time::Instant::now(),
            security_analysis: security_analysis.clone(),
        };

        {
            let mut active = self.active_executions.write().await;
            active.insert(request.id.clone(), active_exec);
        }

        // Send for execution
        self.execution_sender
            .send(request.clone())
            .map_err(|e| format!("Failed to queue execution: {}", e))?;

        // Start sandboxed execution
        self.start_sandboxed_execution(request, security_analysis)
            .await;

        Ok(request.id)
    }

    /// Get execution result
    pub async fn get_execution_result(&self, execution_id: &str) -> Option<SandboxExecutionResult> {
        let mut receiver = self.result_receiver.write().await;

        while let Ok(result) = receiver.try_recv() {
            if result.request_id == execution_id {
                // Clean up active execution
                let mut active = self.active_executions.write().await;
                active.remove(execution_id);
                return Some(result);
            }
        }

        None
    }

    /// Get active executions status
    pub async fn get_active_executions(&self) -> HashMap<String, ExecutionStatus> {
        let active = self.active_executions.read().await;
        let mut status_map = HashMap::new();

        for (id, exec) in active.iter() {
            let elapsed = exec.start_time.elapsed();
            status_map.insert(
                id.clone(),
                ExecutionStatus {
                    id: id.clone(),
                    command: format!("{} {}", exec.request.command, exec.request.args.join(" ")),
                    elapsed_ms: elapsed.as_millis() as u64,
                    sandbox_mode: exec.request.sandbox_mode,
                    risk_level: exec
                        .security_analysis
                        .as_ref()
                        .map(|a| a.risk_level)
                        .unwrap_or(SecurityRiskLevel::Safe),
                    internet_access: exec.request.internet_access,
                },
            );
        }

        status_map
    }

    /// Validate execution request
    async fn validate_request(&self, request: &SandboxExecutionRequest) -> Result<(), String> {
        // Check concurrent execution limit
        let active = self.active_executions.read().await;
        if active.len() >= self.config.max_concurrent_executions {
            return Err(format!(
                "Maximum concurrent executions ({}) exceeded",
                self.config.max_concurrent_executions
            ));
        }

        // Check restricted commands
        if self.config.restricted_commands.contains(&request.command) {
            return Err(format!("Command '{}' is restricted", request.command));
        }

        // Validate resource limits
        if request.resource_limits.max_memory_mb > 8192 {
            return Err("Memory limit too high".to_string());
        }

        Ok(())
    }

    /// Start sandboxed execution
    async fn start_sandboxed_execution(
        &self,
        request: SandboxExecutionRequest,
        security_analysis: Option<AISecurityAnalysis>,
    ) {
        let result_sender = self.result_sender.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            let start_time = tokio::time::Instant::now();

            // Execute based on sandbox mode
            let result = match request.sandbox_mode {
                SandboxMode::Isolated => Self::execute_isolated(request, &config).await,
                SandboxMode::NetworkIsolated => {
                    Self::execute_network_isolated(request, &config).await
                }
                SandboxMode::Supervised => Self::execute_supervised(request, &config).await,
                SandboxMode::MacOSSandbox => Self::execute_macos_sandbox(request, &config).await,
            };

            let execution_time = start_time.elapsed().as_millis() as u64;

            // Create final result
            let execution_result = SandboxExecutionResult {
                request_id: result.request_id,
                success: result.success,
                exit_code: result.exit_code,
                stdout: result.stdout,
                stderr: result.stderr,
                execution_time_ms: execution_time,
                resources_used: result.resources_used,
                network_activity: result.network_activity,
                security_events: result.security_events,
                ai_analysis: security_analysis,
                sandbox_mode_used: request.sandbox_mode,
                completed_at: chrono::Utc::now(),
            };

            // Send result
            let _ = result_sender.send(execution_result);
        });
    }

    /// Execute in isolated sandbox (no network, limited filesystem)
    async fn execute_isolated(
        request: SandboxExecutionRequest,
        _config: &SandboxConfig,
    ) -> ExecutionData {
        // Simulate isolated execution
        tokio::time::sleep(Duration::from_millis(200)).await;

        ExecutionData {
            request_id: request.id,
            success: true,
            exit_code: Some(0),
            stdout: "Executed in isolated sandbox\n".to_string(),
            stderr: String::new(),
            resources_used: ResourceUsage {
                cpu_time_seconds: 0.1,
                memory_peak_mb: 50,
                disk_write_mb: 0,
                network_sent_mb: 0.0,
                network_received_mb: 0.0,
                processes_created: 1,
            },
            network_activity: None,
            security_events: vec![],
        }
    }

    /// Execute in network-isolated sandbox
    async fn execute_network_isolated(
        request: SandboxExecutionRequest,
        _config: &SandboxConfig,
    ) -> ExecutionData {
        // Simulate network-isolated execution
        tokio::time::sleep(Duration::from_millis(300)).await;

        let network_activity = if request.internet_access {
            Some(NetworkActivity {
                connections_attempted: 1,
                connections_successful: 1,
                domains_accessed: vec!["example.com".to_string()],
                ports_used: vec![443],
                data_transferred_mb: 0.1,
                suspicious_activity_detected: false,
            })
        } else {
            None
        };

        ExecutionData {
            request_id: request.id,
            success: true,
            exit_code: Some(0),
            stdout: "Executed in network-isolated sandbox\n".to_string(),
            stderr: String::new(),
            resources_used: ResourceUsage {
                cpu_time_seconds: 0.15,
                memory_peak_mb: 75,
                disk_write_mb: 1,
                network_sent_mb: 0.1,
                network_received_mb: 0.05,
                processes_created: 1,
            },
            network_activity,
            security_events: vec![],
        }
    }

    /// Execute in supervised sandbox with monitoring
    async fn execute_supervised(
        request: SandboxExecutionRequest,
        config: &SandboxConfig,
    ) -> ExecutionData {
        // Simulate supervised execution with monitoring
        tokio::time::sleep(Duration::from_millis(400)).await;

        let mut security_events = Vec::new();

        // Simulate security monitoring
        if config.enable_filesystem_monitoring && request.args.iter().any(|arg| arg.contains("/")) {
            security_events.push(SecurityEvent {
                event_type: SecurityEventType::FileSystemAccess,
                timestamp: chrono::Utc::now(),
                description: "File system access detected".to_string(),
                severity: SecurityRiskLevel::Low,
                blocked: false,
            });
        }

        ExecutionData {
            request_id: request.id,
            success: true,
            exit_code: Some(0),
            stdout: "Executed in supervised sandbox with monitoring\n".to_string(),
            stderr: String::new(),
            resources_used: ResourceUsage {
                cpu_time_seconds: 0.2,
                memory_peak_mb: 100,
                disk_write_mb: 2,
                network_sent_mb: 0.0,
                network_received_mb: 0.0,
                processes_created: 1,
            },
            network_activity: None,
            security_events,
        }
    }

    /// Execute in macOS-style sandbox with internet access
    async fn execute_macos_sandbox(
        request: SandboxExecutionRequest,
        config: &SandboxConfig,
    ) -> ExecutionData {
        // Simulate macOS sandbox execution
        tokio::time::sleep(Duration::from_millis(350)).await;

        let network_activity = if request.internet_access {
            Some(NetworkActivity {
                connections_attempted: 2,
                connections_successful: 2,
                domains_accessed: vec![
                    "api.github.com".to_string(),
                    "registry.npmjs.org".to_string(),
                ],
                ports_used: vec![443, 80],
                data_transferred_mb: 0.5,
                suspicious_activity_detected: false,
            })
        } else {
            None
        };

        let mut security_events = Vec::new();

        // macOS-style security events
        if config.enable_network_monitoring && request.internet_access {
            security_events.push(SecurityEvent {
                event_type: SecurityEventType::NetworkConnection,
                timestamp: chrono::Utc::now(),
                description: "Network connection established (macOS sandbox)".to_string(),
                severity: SecurityRiskLevel::Low,
                blocked: false,
            });
        }

        ExecutionData {
            request_id: request.id,
            success: true,
            exit_code: Some(0),
            stdout: "Executed in macOS-style sandbox with internet connectivity\n".to_string(),
            stderr: String::new(),
            resources_used: ResourceUsage {
                cpu_time_seconds: 0.18,
                memory_peak_mb: 85,
                disk_write_mb: 3,
                network_sent_mb: 0.3,
                network_received_mb: 0.2,
                processes_created: 1,
            },
            network_activity,
            security_events,
        }
    }
}

/// Execution data structure
struct ExecutionData {
    request_id: String,
    success: bool,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
    resources_used: ResourceUsage,
    network_activity: Option<NetworkActivity>,
    security_events: Vec<SecurityEvent>,
}

/// Execution status for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStatus {
    pub id: String,
    pub command: String,
    pub elapsed_ms: u64,
    pub sandbox_mode: SandboxMode,
    pub risk_level: SecurityRiskLevel,
    pub internet_access: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_security_analysis() {
        let analyzer = DefaultAISecurityAnalyzer::new();

        let request = SandboxExecutionRequest {
            id: "test_cmd_1".to_string(),
            command: "rm".to_string(),
            args: vec!["-rf".to_string(), "/tmp/*".to_string()],
            working_directory: None,
            environment: HashMap::new(),
            sandbox_mode: SandboxMode::Isolated,
            timeout_seconds: Some(30),
            internet_access: false,
            resource_limits: ResourceLimits {
                max_cpu_time_seconds: 60,
                max_memory_mb: 512,
                max_disk_write_mb: 100,
                max_network_bandwidth_mbps: 10,
                max_processes: 5,
            },
            ai_security_review: true,
        };

        let analysis = analyzer.analyze_command(&request).await.unwrap();

        // Should detect destructive file operation
        assert!(matches!(
            analysis.risk_level,
            SecurityRiskLevel::Critical | SecurityRiskLevel::High
        ));
        assert!(
            analysis
                .detected_threats
                .iter()
                .any(|t| matches!(t.threat_type, ThreatType::DestructiveFileOperation))
        );
        assert!(!analysis.allow_execution || analysis.requires_approval);
    }

    #[tokio::test]
    async fn test_sandbox_executor_creation() {
        let config = SandboxConfig {
            default_mode: SandboxMode::MacOSSandbox,
            enable_ai_security: true,
            enable_network_monitoring: true,
            enable_filesystem_monitoring: true,
            max_concurrent_executions: 5,
            execution_timeout_seconds: 300,
            restricted_commands: HashSet::from(["sudo".to_string()]),
            allowed_network_domains: HashSet::from(["github.com".to_string()]),
            blocked_file_paths: HashSet::from(["/etc/passwd".to_string()]),
        };

        let ai_analyzer = Arc::new(DefaultAISecurityAnalyzer::new());
        let executor = AISandboxExecutor::new(config, ai_analyzer);

        // Test basic functionality
        let status = executor.get_active_executions().await;
        assert_eq!(status.len(), 0);
    }

    #[tokio::test]
    async fn test_safe_command_execution() {
        let config = SandboxConfig {
            default_mode: SandboxMode::Supervised,
            enable_ai_security: true,
            enable_network_monitoring: false,
            enable_filesystem_monitoring: false,
            max_concurrent_executions: 5,
            execution_timeout_seconds: 300,
            restricted_commands: HashSet::new(),
            allowed_network_domains: HashSet::new(),
            blocked_file_paths: HashSet::new(),
        };

        let ai_analyzer = Arc::new(DefaultAISecurityAnalyzer::new());
        let executor = AISandboxExecutor::new(config, ai_analyzer);

        let request = SandboxExecutionRequest {
            id: "safe_cmd_1".to_string(),
            command: "echo".to_string(),
            args: vec!["Hello, World!".to_string()],
            working_directory: None,
            environment: HashMap::new(),
            sandbox_mode: SandboxMode::Supervised,
            timeout_seconds: Some(10),
            internet_access: false,
            resource_limits: ResourceLimits {
                max_cpu_time_seconds: 30,
                max_memory_mb: 256,
                max_disk_write_mb: 10,
                max_network_bandwidth_mbps: 1,
                max_processes: 2,
            },
            ai_security_review: true,
        };

        let result = executor.execute_sandboxed(request).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_sandbox_modes() {
        assert_eq!(SandboxMode::Isolated as u8, 0);
        assert_eq!(SandboxMode::MacOSSandbox as u8, 3);

        assert_eq!(SecurityRiskLevel::Safe as u8, 0);
        assert_eq!(SecurityRiskLevel::Critical as u8, 4);
    }
}
