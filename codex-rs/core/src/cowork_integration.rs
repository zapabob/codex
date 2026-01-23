//! ClaudeCowork Integration Bridge
//!
//! Rust-Python統合ブリッジ: CodexコアからPythonスクリプトを呼び出す

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::{Context, Result};

/// ClaudeCowork機能タイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoworkFeature {
    /// ブラウザ自動化
    BrowserAutomation,
    /// ドキュメント生成
    DocumentGeneration,
    /// 外部サービス統合
    ExternalConnector,
    /// セッション管理
    SessionManagement,
    /// ファイル管理
    FileManagement,
    /// データ分析
    DataAnalysis,
}

/// ClaudeCowork統合設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoworkIntegrationConfig {
    pub python_path: PathBuf,
    pub scripts_dir: PathBuf,
    pub timeout_seconds: u64,
    pub enable_sandbox: bool,
}

impl Default for CoworkIntegrationConfig {
    fn default() -> Self {
        Self {
            python_path: PathBuf::from("python3"),
            scripts_dir: PathBuf::from("scripts"),
            timeout_seconds: 300,
            enable_sandbox: true,
        }
    }
}

/// ClaudeCowork統合マネージャー
pub struct CoworkIntegrationManager {
    config: CoworkIntegrationConfig,
}

impl CoworkIntegrationManager {
    /// 新しいマネージャーを作成
    pub fn new(config: CoworkIntegrationConfig) -> Self {
        Self { config }
    }

    /// Pythonスクリプトを実行
    pub async fn execute_python_script(
        &self,
        script_name: &str,
        args: Vec<String>,
        input_data: Option<serde_json::Value>,
    ) -> Result<CoworkExecutionResult> {
        let script_path = self.config.scripts_dir.join(script_name);
        
        if !script_path.exists() {
            anyhow::bail!("Script not found: {}", script_path.display());
        }

        let mut cmd = Command::new(&self.config.python_path);
        cmd.arg(&script_path);
        cmd.args(&args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::piped());

        // 環境変数設定
        cmd.env("CODEX_COWORK_MODE", "1");
        cmd.env("CODEX_SCRIPTS_DIR", self.config.scripts_dir.to_string_lossy().to_string());

        let mut child = cmd.spawn()
            .context("Failed to spawn Python process")?;

        // 入力データを送信（JSON形式）
        if let Some(input) = input_data {
            if let Some(stdin) = child.stdin.as_mut() {
                let json_input = serde_json::to_string(&input)?;
                stdin.write_all(json_input.as_bytes()).await?;
                stdin.flush().await?;
            }
        }

        // 実行とタイムアウト処理
        let output = tokio::time::timeout(
            std::time::Duration::from_secs(self.config.timeout_seconds),
            child.wait_with_output(),
        )
        .await
        .context("Script execution timeout")?
        .context("Failed to wait for process")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // JSON出力をパース
        let result_data: serde_json::Value = if !stdout.is_empty() {
            serde_json::from_str(&stdout).unwrap_or_else(|_| {
                serde_json::json!({
                    "success": output.status.success(),
                    "output": stdout,
                    "error": if !stderr.is_empty() { Some(stderr) } else { None }
                })
            })
        } else {
            serde_json::json!({
                "success": output.status.success(),
                "error": if !stderr.is_empty() { Some(stderr) } else { None }
            })
        };

        Ok(CoworkExecutionResult {
            success: output.status.success(),
            exit_code: output.status.code(),
            data: result_data,
            stdout,
            stderr,
        })
    }

    /// ブラウザ自動化を実行
    pub async fn execute_browser_automation(
        &self,
        task: BrowserAutomationTask,
    ) -> Result<CoworkExecutionResult> {
        let input_data = serde_json::to_value(&task)?;
        let args = vec!["--task".to_string(), serde_json::to_string(&task)?];
        
        self.execute_python_script("cowork_browser_automation.py", args, Some(input_data)).await
    }

    /// ドキュメント生成を実行
    pub async fn generate_document(
        &self,
        doc_type: DocumentType,
        output_path: PathBuf,
        content: serde_json::Value,
    ) -> Result<CoworkExecutionResult> {
        let input_data = serde_json::json!({
            "type": doc_type,
            "output_path": output_path.to_string_lossy(),
            "content": content
        });
        
        let args = vec![
            "--type".to_string(),
            format!("{:?}", doc_type),
            "--output".to_string(),
            output_path.to_string_lossy().to_string(),
        ];
        
        self.execute_python_script("cowork_document_generator.py", args, Some(input_data)).await
    }

    /// セッション管理操作
    pub async fn manage_session(
        &self,
        operation: SessionOperation,
    ) -> Result<CoworkExecutionResult> {
        let input_data = serde_json::to_value(&operation)?;
        let args = vec!["--operation".to_string(), serde_json::to_string(&operation)?];
        
        self.execute_python_script("cowork_session_manager.py", args, Some(input_data)).await
    }
}

/// 実行結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoworkExecutionResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub data: serde_json::Value,
    pub stdout: String,
    pub stderr: String,
}

/// ブラウザ自動化タスク
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserAutomationTask {
    pub action: String,
    pub url: Option<String>,
    pub selector: Option<String>,
    pub form_data: Option<HashMap<String, String>>,
    pub workflow: Option<Vec<serde_json::Value>>,
}

/// ドキュメントタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    Excel,
    Word,
    PowerPoint,
}

/// セッション操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionOperation {
    pub operation: String, // create, get, list, rename, delete, add_task, etc.
    pub session_id: Option<String>,
    pub name: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cowork_integration_config() {
        let config = CoworkIntegrationConfig::default();
        assert_eq!(config.timeout_seconds, 300);
    }
}
