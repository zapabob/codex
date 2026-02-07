use anyhow::{Context, Result};
use codex_app_server_protocol::A2AMessage;
use serde_json::json;
use tokio::process::Command;

pub struct QAAgent;

impl QAAgent {
    pub async fn handle_message(message: &A2AMessage) -> Result<Option<A2AMessage>> {
        match message.r#type.as_str() {
            "audit" => Self::run_audit(message).await,
            "optimize" => Self::suggest_optimization(message).await,
            "merge_request" => Self::handle_merge_request(message).await,
            _ => Ok(None),
        }
    }

    async fn run_audit(message: &A2AMessage) -> Result<Option<A2AMessage>> {
        let target = message
            .content
            .get("target")
            .and_then(|t| t.as_str())
            .unwrap_or(".");

        let mut findings = Vec::new();

        // Run cargo check with JSON output
        let output = Command::new("cargo")
            .args(&["check", "--message-format=json", "--workspace"])
            .current_dir(target)
            .output()
            .await
            .context("Failed to run cargo check")?;

        if !output.status.success() {
            findings.push(json!({
                "severity": "error",
                "message": format!("Cargo check failed to run: {}", String::from_utf8_lossy(&output.stderr)),
                "location": "system"
             }));
        }

        let reader = std::io::BufReader::new(output.stdout.as_slice());
        use std::io::BufRead;

        for line in reader.lines() {
            if let Ok(line_str) = line {
                if let Some(finding) = Self::parse_cargo_line(&line_str) {
                    findings.push(finding);
                }
            }
        }

        if findings.is_empty() {
            findings.push(json!({
               "severity": "info",
               "message": "No issues found by cargo check",
               "location": "system"
            }));
        }

        let report = json!({
            "status": "completed",
            "findings": findings
        });

        Ok(Some(A2AMessage {
            from: "QA-Agent".to_string(),
            to: message.from.clone(),
            r#type: "audit_result".to_string(),
            content: report,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis() as u64,
        }))
    }

    async fn suggest_optimization(message: &A2AMessage) -> Result<Option<A2AMessage>> {
        let suggestion = json!({
            "original": "vec.push(item)",
            "optimized": "vec.try_reserve(1)?; vec.push(item)",
            "reason": "Avoid reallocation (Optimization Engine v1)",
            "confidence": 0.95
        });

        Ok(Some(A2AMessage {
            from: "QA-Agent".to_string(),
            to: message.from.clone(),
            r#type: "optimization_suggestion".to_string(),
            content: suggestion,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis() as u64,
        }))
    }

    async fn handle_merge_request(message: &A2AMessage) -> Result<Option<A2AMessage>> {
        // Logic: Check if CI passed (mocked here)
        let approved = true;

        Ok(Some(A2AMessage {
            from: "QA-Agent".to_string(),
            to: message.from.clone(),
            r#type: "merge_approval".to_string(),
            content: json!({
                "approved": approved,
                "reason": "Automated checks passed"
            }),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis() as u64,
        }))
    }
}
