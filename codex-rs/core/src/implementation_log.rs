//! Implementation log management module
//!
//! Manages automatic creation of implementation logs in _docs/ directory
//! and provides startup log reading functionality with nanj-style responses

use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use tracing::debug;
use tracing::error;

/// Create an implementation log using Python script
pub async fn create_implementation_log(
    feature_name: &str,
    task_description: &str,
    implementation_details: &str,
    worktree_name: Option<&str>,
) -> Result<PathBuf> {
    let script_path = find_implementation_logger_script()?;

    let mut cmd = Command::new("py");
    cmd.arg("-3");
    cmd.arg(&script_path);
    cmd.arg("create");
    cmd.arg(feature_name);
    cmd.arg(task_description);
    cmd.arg(implementation_details);

    if let Some(worktree) = worktree_name {
        // Note: Python script will auto-detect worktree if not provided
        // This is a placeholder for future enhancement
        debug!("Worktree name: {}", worktree);
    }

    let output = cmd
        .output()
        .context("Failed to execute implementation logger script")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Implementation logger script failed: {}", stderr);
        return Err(anyhow::anyhow!(
            "Failed to create implementation log: {}",
            stderr
        ));
    }

    // Parse JSON response to get log path
    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
        if let Some(path_str) = json.get("path").and_then(|p| p.as_str()) {
            return Ok(PathBuf::from(path_str));
        }
    }

    // Fallback: construct path from feature name
    let docs_dir = find_docs_directory()?;
    let now = time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc());
    let date = format!(
        "{:04}-{:02}-{:02}",
        now.year(),
        now.month() as u8,
        now.day()
    );
    let safe_feature_name =
        feature_name.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");
    let worktree = worktree_name.unwrap_or("main");
    let filename = format!("{}_{}{{{}}}.md", date, safe_feature_name, worktree);
    Ok(docs_dir.join(filename))
}

/// Load recent implementation logs
pub async fn load_recent_implementation_logs(limit: usize) -> Result<Vec<ImplementationLog>> {
    let script_path = find_implementation_logger_script()?;

    let mut cmd = Command::new("py");
    cmd.arg("-3");
    cmd.arg(&script_path);
    cmd.arg("load");
    cmd.arg(limit.to_string());

    let output = cmd
        .output()
        .context("Failed to execute implementation logger script")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Failed to load implementation logs: {}", stderr);
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let logs: Vec<ImplementationLog> =
        serde_json::from_str(&stdout).context("Failed to parse implementation logs JSON")?;

    Ok(logs)
}

/// Generate nanj-style response from implementation logs
pub async fn generate_nanj_response() -> Result<String> {
    let script_path = find_implementation_logger_script()?;

    let mut cmd = Command::new("py");
    cmd.arg("-3");
    cmd.arg(&script_path);
    cmd.arg("nanj");

    let output = cmd
        .output()
        .context("Failed to execute implementation logger script")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Failed to generate nanj response: {}", stderr);
        return Ok("実装ログの読み込みに失敗したで。".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}

/// Find the implementation logger Python script
fn find_implementation_logger_script() -> Result<PathBuf> {
    // Try to find script in scripts/ directory relative to workspace root
    let current_dir = std::env::current_dir()?;

    // Try multiple possible locations
    let possible_paths = vec![
        current_dir.join("scripts").join("implementation_logger.py"),
        current_dir
            .join("..")
            .join("scripts")
            .join("implementation_logger.py"),
        PathBuf::from("scripts").join("implementation_logger.py"),
    ];

    for path in possible_paths {
        if path.exists() {
            return Ok(path);
        }
    }

    Err(anyhow::anyhow!(
        "Could not find implementation_logger.py script"
    ))
}

/// Find the _docs directory
fn find_docs_directory() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let docs_dir = current_dir.join("_docs");

    if !docs_dir.exists() {
        std::fs::create_dir_all(&docs_dir).context("Failed to create _docs directory")?;
    }

    Ok(docs_dir)
}

/// Implementation log structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImplementationLog {
    pub file: String,
    pub date: String,
    pub feature: String,
    pub worktree: String,
    pub content: String,
}

/// Play sound file on plan/agent completion
pub async fn play_completion_sound() -> Result<()> {
    let sound_path = PathBuf::from(r"C:\Users\downl\Desktop\SO8T\.cursor\marisa_owattaze.wav");

    if !sound_path.exists() {
        debug!("Sound file not found: {}", sound_path.display());
        return Ok(());
    }

    // Use Windows Media Player or PowerShell to play sound
    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command");
        cmd.arg(format!(
            "(New-Object Media.SoundPlayer '{}').PlaySync()",
            sound_path.display()
        ));

        // Run in background (don't wait for completion)
        let _ = cmd.spawn();
    }

    #[cfg(not(target_os = "windows"))]
    {
        // For non-Windows systems, try common audio players
        let players = vec!["aplay", "paplay", "afplay"];
        for player in players {
            if Command::new("which").arg(player).output().is_ok() {
                let _ = Command::new(player).arg(&sound_path).spawn();
                break;
            }
        }
    }

    Ok(())
}
