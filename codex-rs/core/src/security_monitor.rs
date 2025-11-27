//! Security Monitor for AI Development Environment
//!
//! Prevents dangerous operations and monitors for malware:
//! - Blocks explicit shell command execution by AI
//! - Prevents file deletion operations
//! - Malware detection and isolation
//! - Behavioral analysis and anomaly detection

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::Result;

/// Security threat levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Security event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEvent {
    BlockedShellCommand {
        command: String,
        args: Vec<String>,
        source: String,
        reason: String,
    },
    BlockedFileDeletion {
        path: PathBuf,
        source: String,
        reason: String,
    },
    MalwareDetected {
        file_path: PathBuf,
        malware_type: String,
        confidence: f64,
        actions_taken: Vec<String>,
    },
    SuspiciousBehavior {
        description: String,
        source: String,
        indicators: Vec<String>,
    },
    AnomalyDetected {
        metric: String,
        value: f64,
        threshold: f64,
        description: String,
    },
}

/// Security rule configuration
#[derive(Debug, Clone)]
pub struct SecurityRule {
    pub name: String,
    pub pattern: Regex,
    pub threat_level: ThreatLevel,
    pub action: SecurityAction,
    pub enabled: bool,
}

/// Security actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityAction {
    Block,
    Warn,
    Isolate,
    Report,
}

/// Malware signature database
#[derive(Debug, Clone)]
pub struct MalwareSignature {
    pub signature_type: String,
    pub pattern: String,
    pub description: String,
    pub threat_level: ThreatLevel,
}

/// Security Monitor
pub struct SecurityMonitor {
    rules: Vec<SecurityRule>,
    malware_signatures: Vec<MalwareSignature>,
    blocked_commands: HashSet<String>,
    quarantined_files: HashSet<PathBuf>,
    event_log: Arc<Mutex<VecDeque<SecurityEvent>>>,
    anomaly_detector: AnomalyDetector,
    command_tx: mpsc::UnboundedSender<SecurityCommand>,
    command_rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<SecurityCommand>>>>,
}

/// Anomaly detection for behavioral analysis
#[derive(Debug)]
struct AnomalyDetector {
    metrics_history: HashMap<String, VecDeque<f64>>,
    thresholds: HashMap<String, f64>,
    window_size: usize,
}

#[derive(Debug)]
enum SecurityCommand {
    CheckCommand {
        command: String,
        args: Vec<String>,
        source: String,
        response: oneshot::Sender<Result<bool>>,
    },
    CheckFileOperation {
        operation: FileOperation,
        source: String,
        response: oneshot::Sender<Result<bool>>,
    },
    ScanFile {
        path: PathBuf,
        response: oneshot::Sender<Result<Option<MalwareInfo>>>,
    },
    ReportAnomaly {
        metric: String,
        value: f64,
        description: String,
    },
    GetEvents {
        limit: usize,
        response: oneshot::Sender<Result<Vec<SecurityEvent>>>,
    },
}

/// File operation types
#[derive(Debug, Clone)]
pub enum FileOperation {
    Create(PathBuf),
    Read(PathBuf),
    Write(PathBuf),
    Delete(PathBuf),
    Execute(PathBuf),
}

/// Malware information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalwareInfo {
    pub malware_type: String,
    pub confidence: f64,
    pub threat_level: ThreatLevel,
    pub signatures_matched: Vec<String>,
}

impl SecurityMonitor {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut monitor = Self {
            rules: Vec::new(),
            malware_signatures: Vec::new(),
            blocked_commands: HashSet::new(),
            quarantined_files: HashSet::new(),
            event_log: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            anomaly_detector: AnomalyDetector::new(),
            command_tx: tx,
            command_rx: Arc::new(Mutex::new(Some(rx))),
        };

        monitor.initialize_default_rules();
        monitor.initialize_malware_signatures();
        monitor
    }

    /// Initialize default security rules
    fn initialize_default_rules(&mut self) {
        let rules = vec![
            SecurityRule {
                name: "block_shell_execution".to_string(),
                pattern: Regex::new(r"^(sh|bash|zsh|fish|powershell|cmd)\s").unwrap(),
                threat_level: ThreatLevel::Critical,
                action: SecurityAction::Block,
                enabled: true,
            },
            SecurityRule {
                name: "block_sudo".to_string(),
                pattern: Regex::new(r"\bsudo\b").unwrap(),
                threat_level: ThreatLevel::Critical,
                action: SecurityAction::Block,
                enabled: true,
            },
            SecurityRule {
                name: "block_dangerous_rm".to_string(),
                pattern: Regex::new(r"\brm\s+-rf\b").unwrap(),
                threat_level: ThreatLevel::High,
                action: SecurityAction::Block,
                enabled: true,
            },
            SecurityRule {
                name: "block_system_files".to_string(),
                pattern: Regex::new(r"/etc/|/usr/|/bin/|/sbin/|/boot/|/sys/|/proc/").unwrap(),
                threat_level: ThreatLevel::High,
                action: SecurityAction::Block,
                enabled: true,
            },
            SecurityRule {
                name: "block_network_tools".to_string(),
                pattern: Regex::new(r"\b(nc|netcat|ncat|telnet|ssh|scp|ftp|sftp)\b").unwrap(),
                threat_level: ThreatLevel::Medium,
                action: SecurityAction::Warn,
                enabled: true,
            },
        ];

        self.rules = rules;

        // Initialize blocked commands
        let blocked = [
            "rm", "rmdir", "del", "erase", "format", "fdisk", "mkfs",
            "dd", "shred", "wipe", "srm", "sudo", "su", "chmod +x",
            "curl", "wget", "ssh", "scp", "ftp", "telnet", "nc", "netcat",
        ];

        self.blocked_commands.extend(blocked.iter().map(|s| s.to_string()));
    }

    /// Initialize malware signatures
    fn initialize_malware_signatures(&mut self) {
        let signatures = vec![
            MalwareSignature {
                signature_type: "hash".to_string(),
                pattern: "44d88612fea8a8f36de82e1278abb02f".to_string(), // Example hash
                description: "Known ransomware signature".to_string(),
                threat_level: ThreatLevel::Critical,
            },
            MalwareSignature {
                signature_type: "pattern".to_string(),
                pattern: r"eval\(.*base64_decode.*\)".to_string(),
                description: "Obfuscated PHP malware pattern".to_string(),
                threat_level: ThreatLevel::High,
            },
            MalwareSignature {
                signature_type: "behavior".to_string(),
                pattern: "excessive_file_modification".to_string(),
                description: "Ransomware behavior pattern".to_string(),
                threat_level: ThreatLevel::Critical,
            },
        ];

        self.malware_signatures = signatures;
    }

    /// Check if command execution is allowed
    pub async fn check_command(&self, command: &str, args: &[String], source: &str) -> Result<bool> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(SecurityCommand::CheckCommand {
            command: command.to_string(),
            args: args.to_vec(),
            source: source.to_string(),
            response: tx,
        })?;

        rx.await
    }

    /// Check if file operation is allowed
    pub async fn check_file_operation(&self, operation: &FileOperation, source: &str) -> Result<bool> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(SecurityCommand::CheckFileOperation {
            operation: operation.clone(),
            source: source.to_string(),
            response: tx,
        })?;

        rx.await
    }

    /// Scan file for malware
    pub async fn scan_file(&self, path: &Path) -> Result<Option<MalwareInfo>> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(SecurityCommand::ScanFile {
            path: path.to_path_buf(),
            response: tx,
        })?;

        rx.await
    }

    /// Report security anomaly
    pub fn report_anomaly(&self, metric: &str, value: f64, description: &str) {
        let _ = self.command_tx.send(SecurityCommand::ReportAnomaly {
            metric: metric.to_string(),
            value,
            description: description.to_string(),
        });
    }

    /// Get recent security events
    pub async fn get_events(&self, limit: usize) -> Result<Vec<SecurityEvent>> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(SecurityCommand::GetEvents {
            limit,
            response: tx,
        })?;

        rx.await
    }

    /// Add custom security rule
    pub fn add_rule(&mut self, rule: SecurityRule) {
        self.rules.push(rule);
    }

    /// Enable/disable rule
    pub fn set_rule_enabled(&mut self, rule_name: &str, enabled: bool) {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.name == rule_name) {
            rule.enabled = enabled;
        }
    }

    /// Run the security monitor
    pub async fn run(self) -> Result<()> {
        let mut rx = self.command_rx.lock().unwrap().take().unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                SecurityCommand::CheckCommand { command, args, source, response } => {
                    let result = self.check_command_internal(&command, &args, &source);
                    let _ = response.send(Ok(result));
                }
                SecurityCommand::CheckFileOperation { operation, source, response } => {
                    let result = self.check_file_operation_internal(&operation, &source);
                    let _ = response.send(Ok(result));
                }
                SecurityCommand::ScanFile { path, response } => {
                    let result = self.scan_file_internal(&path).await;
                    let _ = response.send(result);
                }
                SecurityCommand::ReportAnomaly { metric, value, description } => {
                    self.anomaly_detector.detect_anomaly(&metric, value, &description);
                }
                SecurityCommand::GetEvents { limit, response } => {
                    let events = self.get_recent_events(limit);
                    let _ = response.send(Ok(events));
                }
            }
        }

        Ok(())
    }

    fn check_command_internal(&self, command: &str, args: &[String], source: &str) -> bool {
        // Check blocked commands
        if self.blocked_commands.contains(command) {
            self.log_event(SecurityEvent::BlockedShellCommand {
                command: command.to_string(),
                args: args.to_vec(),
                source: source.to_string(),
                reason: "Command is in blocked list".to_string(),
            });
            return false;
        }

        // Check security rules
        let full_command = format!("{} {}", command, args.join(" "));
        for rule in &self.rules {
            if rule.enabled && rule.pattern.is_match(&full_command) {
                match rule.action {
                    SecurityAction::Block => {
                        self.log_event(SecurityEvent::BlockedShellCommand {
                            command: command.to_string(),
                            args: args.to_vec(),
                            source: source.to_string(),
                            reason: format!("Blocked by rule: {}", rule.name),
                        });
                        return false;
                    }
                    SecurityAction::Warn => {
                        self.log_event(SecurityEvent::BlockedShellCommand {
                            command: command.to_string(),
                            args: args.to_vec(),
                            source: source.to_string(),
                            reason: format!("Warning from rule: {}", rule.name),
                        });
                        // Allow but log
                    }
                    _ => {} // Other actions handled differently
                }
            }
        }

        true
    }

    fn check_file_operation_internal(&self, operation: &FileOperation, source: &str) -> bool {
        match operation {
            FileOperation::Delete(path) => {
                // Always block file deletion for security
                self.log_event(SecurityEvent::BlockedFileDeletion {
                    path: path.clone(),
                    source: source.to_string(),
                    reason: "File deletion is blocked for security".to_string(),
                });
                false
            }
            FileOperation::Write(path) | FileOperation::Create(path) => {
                // Check if path is in quarantined files
                if self.quarantined_files.contains(path) {
                    self.log_event(SecurityEvent::BlockedFileDeletion {
                        path: path.clone(),
                        source: source.to_string(),
                        reason: "File is quarantined".to_string(),
                    });
                    return false;
                }

                // Check system paths
                if self.is_system_path(path) {
                    return false;
                }

                true
            }
            _ => true,
        }
    }

    async fn scan_file_internal(&self, path: &Path) -> Result<Option<MalwareInfo>> {
        if !path.exists() {
            return Ok(None);
        }

        // Read file content
        let content = tokio::fs::read(path).await?;
        let content_str = String::from_utf8_lossy(&content);

        // Calculate file hash
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = format!("{:x}", hasher.finalize());

        // Check hash signatures
        for signature in &self.malware_signatures {
            if signature.signature_type == "hash" && hash.contains(&signature.pattern) {
                let malware_info = MalwareInfo {
                    malware_type: "Hash-based detection".to_string(),
                    confidence: 0.95,
                    threat_level: signature.threat_level,
                    signatures_matched: vec![signature.description.clone()],
                };

                // Quarantine file
                self.quarantined_files.insert(path.to_path_buf());

                self.log_event(SecurityEvent::MalwareDetected {
                    file_path: path.to_path_buf(),
                    malware_type: malware_info.malware_type.clone(),
                    confidence: malware_info.confidence,
                    actions_taken: vec!["quarantined".to_string()],
                });

                return Ok(Some(malware_info));
            }
        }

        // Check pattern signatures
        for signature in &self.malware_signatures {
            if signature.signature_type == "pattern" {
                if let Ok(pattern) = Regex::new(&signature.pattern) {
                    if pattern.is_match(&content_str) {
                        let malware_info = MalwareInfo {
                            malware_type: "Pattern-based detection".to_string(),
                            confidence: 0.85,
                            threat_level: signature.threat_level,
                            signatures_matched: vec![signature.description.clone()],
                        };

                        self.quarantined_files.insert(path.to_path_buf());

                        self.log_event(SecurityEvent::MalwareDetected {
                            file_path: path.to_path_buf(),
                            malware_type: malware_info.malware_type.clone(),
                            confidence: malware_info.confidence,
                            actions_taken: vec!["quarantined".to_string()],
                        });

                        return Ok(Some(malware_info));
                    }
                }
            }
        }

        Ok(None)
    }

    fn is_system_path(&self, path: &Path) -> bool {
        let system_paths = [
            "/etc", "/usr", "/bin", "/sbin", "/boot", "/sys", "/proc", "/dev",
            "C:\\Windows", "C:\\System32", "C:\\Program Files",
        ];

        let path_str = path.to_string_lossy();
        system_paths.iter().any(|sys| path_str.starts_with(sys))
    }

    fn log_event(&self, event: SecurityEvent) {
        let mut log = self.event_log.lock().unwrap();

        // Keep only last 1000 events
        if log.len() >= 1000 {
            log.pop_front();
        }

        log.push_back(event);
    }

    fn get_recent_events(&self, limit: usize) -> Vec<SecurityEvent> {
        let log = self.event_log.lock().unwrap();
        log.iter().rev().take(limit).cloned().collect()
    }
}

impl AnomalyDetector {
    fn new() -> Self {
        Self {
            metrics_history: HashMap::new(),
            thresholds: HashMap::new(),
            window_size: 100,
        }
    }

    fn detect_anomaly(&mut self, metric: &str, value: f64, description: &str) {
        // Add to history
        let history = self.metrics_history.entry(metric.to_string())
            .or_insert_with(|| VecDeque::with_capacity(self.window_size));

        history.push_back(value);
        if history.len() > self.window_size {
            history.pop_front();
        }

        // Calculate statistics
        if history.len() >= 10 {
            let mean = history.iter().sum::<f64>() / history.len() as f64;
            let variance = history.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / history.len() as f64;
            let std_dev = variance.sqrt();

            // Check for anomaly (3-sigma rule)
            let threshold = self.thresholds.get(metric).copied().unwrap_or(3.0);
            if (value - mean).abs() > threshold * std_dev {
                // Log anomaly
                println!("Anomaly detected: {} - Value: {}, Mean: {}, StdDev: {}",
                    metric, value, mean, std_dev);
            }
        }
    }
}

impl Default for SecurityMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_command_blocking() {
        let monitor = SecurityMonitor::new();

        // Test blocked command
        let result = monitor.check_command("rm", &["-rf".to_string(), "/".to_string()], "test").await;
        assert_eq!(result.unwrap(), false);

        // Test allowed command
        let result = monitor.check_command("ls", &["-la".to_string()], "test").await;
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_file_operation_blocking() {
        let monitor = SecurityMonitor::new();

        // Test blocked deletion
        let operation = FileOperation::Delete(PathBuf::from("/etc/passwd"));
        let result = monitor.check_file_operation(&operation, "test").await;
        assert_eq!(result.unwrap(), false);

        // Test allowed read
        let operation = FileOperation::Read(PathBuf::from("test.txt"));
        let result = monitor.check_file_operation(&operation, "test").await;
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_malware_scan() {
        let monitor = SecurityMonitor::new();

        // Create test file
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        // In real test, would write malware content and test scanning
        // For now, just test the API
        assert!(monitor.scan_file(&test_file).is_ok());
    }
}
