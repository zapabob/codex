//! ClaudeCowork Integration Bridge
//!
//! Rust-Python統合ブリッジ: CodexコアからPythonスクリプトを呼び出す

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

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
    /// Git4D可視化
    Git4DVisualization,
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
            anyhow::bail!(
                "Script not found: {} (scripts_dir: {})",
                script_path.display(),
                self.config.scripts_dir.display()
            );
        }

        tracing::debug!(
            "Executing Python script: {} with args: {:?}",
            script_path.display(),
            args
        );

        let mut cmd = Command::new(&self.config.python_path);
        cmd.arg(&script_path);
        cmd.args(&args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::piped());

        // 環境変数設定
        cmd.env("CODEX_COWORK_MODE", "1");
        cmd.env(
            "CODEX_SCRIPTS_DIR",
            self.config.scripts_dir.to_string_lossy().to_string(),
        );

        // サンドボックス設定（macOSライクサンドボックス統合）
        if self.config.enable_sandbox {
            // macOS Seatbeltスタイルのサンドボックス環境変数を設定
            cmd.env("CODEX_SANDBOX_ENABLED", "1");
            cmd.env("CODEX_SANDBOX_MODE", "macos-style");

            // ファイル共有とアクセス制御の設定
            // サンドボックス内でのcowork機能の利用を許可
            cmd.env("CODEX_COWORK_SANDBOXED", "1");

            // ネットワーク分離と許可リスト
            // 必要に応じてネットワークアクセスを制限
            cmd.env("CODEX_NETWORK_ISOLATION", "permissive");
        }

        let mut child = cmd.spawn().with_context(|| {
            format!(
                "Failed to spawn Python process: {} {}",
                self.config.python_path.display(),
                script_path.display()
            )
        })?;

        // 入力データを送信（JSON形式）
        if let Some(input) = input_data
            && let Some(stdin) = child.stdin.as_mut()
        {
            let json_input = serde_json::to_string(&input)?;
            stdin.write_all(json_input.as_bytes()).await?;
            stdin.flush().await?;
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
        let stderr_value = if stderr.is_empty() {
            None
        } else {
            Some(stderr.clone())
        };

        // JSON出力をパース
        let result_data: serde_json::Value = if !stdout.is_empty() {
            serde_json::from_str(&stdout).unwrap_or_else(|e| {
                tracing::warn!(
                    "Failed to parse JSON output from script {}: {}",
                    script_name,
                    e
                );
                serde_json::json!({
                    "success": output.status.success(),
                    "output": stdout,
                    "error": stderr_value.clone(),
                    "parse_error": format!("Failed to parse JSON: {}", e)
                })
            })
        } else {
            serde_json::json!({
                "success": output.status.success(),
                "error": stderr_value
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

        self.execute_python_script("cowork_browser_automation.py", args, Some(input_data))
            .await
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

        self.execute_python_script("cowork_document_generator.py", args, Some(input_data))
            .await
    }

    /// セッション管理操作
    pub async fn manage_session(
        &self,
        operation: SessionOperation,
    ) -> Result<CoworkExecutionResult> {
        let input_data = serde_json::to_value(&operation)?;
        let args = vec![
            "--operation".to_string(),
            serde_json::to_string(&operation)?,
        ];

        self.execute_python_script("cowork_session_manager.py", args, Some(input_data))
            .await
    }
}

/// Git4D可視化を起動
///
/// GUI側のAPIエンドポイントを呼び出してGit4D可視化を起動します
pub async fn launch_git4d_visualization(repository_path: PathBuf, mode: String) -> Result<()> {
    use reqwest::Client;

    // Validate repository path exists
    if !repository_path.exists() {
        anyhow::bail!(
            "Repository path does not exist: {}. Please check the path and try again.",
            repository_path.display()
        );
    }

    // Check if it's a git repository
    let git_dir = repository_path.join(".git");
    if !git_dir.exists() && !repository_path.is_file() {
        // Try to find git repository in parent directories
        let mut current = repository_path.clone();
        let mut found_git = false;
        for _ in 0..10 {
            if current.join(".git").exists() {
                found_git = true;
                break;
            }
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }

        if !found_git {
            anyhow::bail!(
                "No git repository found at: {}. Please navigate to a git repository and try again.",
                repository_path.display()
            );
        }
    }

    // Validate mode
    if !["desktop", "vr", "ar"].contains(&mode.as_str()) {
        anyhow::bail!(
            "Invalid visualization mode: {}. Must be one of: desktop, vr, ar",
            mode
        );
    }

    // Check if GUI is running
    let gui_port = std::env::var("CODEX_GUI_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8787);

    let url = format!("http://localhost:{}/api/visualization/git4d", gui_port);

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .context("Failed to create HTTP client")?;

    let payload = serde_json::json!({
        "mode": mode,
        "repository_path": repository_path.to_string_lossy().to_string(),
    });

    tracing::debug!(
        "Launching Git4D visualization: mode={}, path={:?}",
        mode,
        repository_path
    );

    // Attempt to connect to GUI API
    let response = match client.post(&url).json(&payload).send().await {
        Ok(res) => res,
        Err(e) => {
            if e.is_timeout() || e.is_connect() {
                anyhow::bail!(
                    "GUI server is not running or not accessible at {}. Please start the GUI server (codex-gui) and try again.",
                    url
                );
            }
            return Err(e).context("Failed to send request to GUI API");
        }
    };

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        // Provide user-friendly error messages
        let error_msg = match status.as_u16() {
            404 => {
                format!("Visualization endpoint not found. The GUI server may need to be updated.")
            }
            422 => format!("Invalid request parameters: {}", error_text),
            500 => format!("Server error while launching visualization: {}", error_text),
            _ => format!("GUI API returned error status {}: {}", status, error_text),
        };

        anyhow::bail!("{}", error_msg);
    }

    // Check response for VR/AR device availability warnings
    if mode == "vr" || mode == "ar" {
        let response_text = response.text().await.unwrap_or_default();
        if response_text.contains("device not available")
            || response_text.contains("VR not available")
        {
            tracing::warn!(
                "VR/AR device may not be available. Visualization will start in desktop mode."
            );
        }
    }

    tracing::info!("Git4D visualization launched successfully in {} mode", mode);
    Ok(())
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
