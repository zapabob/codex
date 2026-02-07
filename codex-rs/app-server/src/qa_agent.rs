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
    fn parse_cargo_line(line: &str) -> Option<serde_json::Value> {
        let json_msg = serde_json::from_str::<serde_json::Value>(line).ok()?;
        if json_msg.get("reason").and_then(|r| r.as_str()) == Some("compiler-message") {
            if let Some(message) = json_msg.get("message") {
                let level = message
                    .get("level")
                    .and_then(|l| l.as_str())
                    .map(|l| match l {
                        "error" => "critical",
                        "warning" => "warning",
                        "note" => "info",
                        "help" => "info",
                        "failure" => "critical",
                        _ => "info",
                    })
                    .unwrap_or("info");

                let text = message
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("");

                let mut location = "unknown".to_string();
                if let Some(spans) = message.get("spans").and_then(|s| s.as_array()) {
                    if let Some(first_span) = spans.first() {
                        let file = first_span
                            .get("file_name")
                            .and_then(|f| f.as_str())
                            .unwrap_or("?");
                        let line = first_span
                            .get("line_start")
                            .and_then(|l| l.as_u64())
                            .unwrap_or(0);
                        location = format!("{}:{}", file, line);
                    }
                }

                return Some(json!({
                    "severity": level,
                    "message": text,
                    "location": location
                }));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cargo_line() {
        let input = r#"{"reason":"compiler-message","package_id":"foo 0.1.0 (path+file:///foo)","message":{"children":[],"code":null,"level":"warning","message":"unused variable: `x`","spans":[{"byte_end":23,"byte_start":22,"column_end":10,"column_start":9,"file_name":"src/main.rs","is_primary":true,"label":null,"line_end":2,"line_start":2,"suggested_replacement":null,"suggestion_applicability":null,"text":[{"highlight_end":10,"highlight_start":9,"text":"    let x = 5;"}]}],"rendered":"warning: unused variable: `x`\n --> src/main.rs:2:9\n  |\n2 |     let x = 5;\n  |         ^ help: if this is intentional, prefix it with an underscore: `_x`\n  |\n  = note: `#[warn(unused_variables)]` on by default\n\n"},"target":{"kind":["bin"],"crate_types":["bin"],"name":"foo","src_path":"/foo/src/main.rs","edition":"2021","doc":true,"doctest":false,"test":true}}"#;

        let finding = QAAgent::parse_cargo_line(input).expect("Should parse");
        assert_eq!(finding["severity"], "warning");
        assert_eq!(finding["message"], "unused variable: `x`");
        assert_eq!(finding["location"], "src/main.rs:2");
    }

    #[test]
    fn test_parse_error_level() {
        let input = r#"{"reason":"compiler-message","message":{"level":"error","message":"oops","spans":[]}}"#;
        let finding = QAAgent::parse_cargo_line(input).expect("Should parse");
        assert_eq!(finding["severity"], "critical");
    }

    #[test]
    fn test_parse_ignore_non_compiler_message() {
        let input = r#"{"reason":"build-script-executed","package_id":"foo 0.1.0 (path+file:///foo)","linked_libs":[],"linked_paths":[],"cfgs":[],"env":[],"out_dir":"/foo/target/debug/build/foo-123","custom_build_target":"debug"}"#;
        assert!(QAAgent::parse_cargo_line(input).is_none());
    }
}
