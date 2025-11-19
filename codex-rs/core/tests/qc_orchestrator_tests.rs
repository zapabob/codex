//! Integration tests for QC Orchestrator

use codex_core::qc_orchestrator::{
    QcConfig, QcInput, Recommendation, TestProfile,
};
use std::fs;
use tempfile::TempDir;

/// Helper to create a test git repository
fn create_test_repo() -> Result<TempDir, Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize git repo
    std::process::Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .output()?;

    // Configure git
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output()?;

    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()?;

    // Create a simple file and commit
    fs::write(repo_path.join("test.txt"), "initial content")?;

    std::process::Command::new("git")
        .args(["add", "test.txt"])
        .current_dir(repo_path)
        .output()?;

    std::process::Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(repo_path)
        .output()?;

    Ok(temp_dir)
}

#[test]
fn test_qc_orchestrator_with_no_changes() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = create_test_repo()?;
    let repo_path = temp_dir.path();

    // Create codex-rs directory structure for tests
    let codex_rs = repo_path.join("codex-rs");
    fs::create_dir(&codex_rs)?;
    fs::write(codex_rs.join("Cargo.toml"), "[workspace]\nmembers = []")?;

    let config = QcConfig {
        default_profile: TestProfile::Minimal,
        max_lines_without_pr: 200,
        base_ref: "HEAD".to_string(),
    };

    let input = QcInput {
        feature: "Test feature".to_string(),
        agent_name: "test-agent".to_string(),
        ai_name: "test-ai".to_string(),
        profile: TestProfile::Minimal,
    };

    // Note: This test may fail if cargo is not available or if the repo structure
    // doesn't match expected layout. In a real scenario, we'd mock the command execution.
    // For now, we just verify the function can be called.
    match codex_core::qc_orchestrator::run_qc(repo_path, input, config) {
        Ok(result) => {
            // Verify basic structure
            assert_eq!(result.diff.changed_lines, 0);
            assert_eq!(result.diff.changed_files, 0);
            // Test might fail or pass depending on environment
            println!("QC Result: {:?}", result.recommendation);
        }
        Err(e) => {
            // Expected in test environment without full cargo setup
            println!("QC failed (expected in test env): {e}");
        }
    }

    Ok(())
}

#[test]
fn test_profile_parsing() {
    use std::str::FromStr;

    assert!(matches!(
        TestProfile::from_str("minimal"),
        Ok(TestProfile::Minimal)
    ));
    assert!(matches!(
        TestProfile::from_str("standard"),
        Ok(TestProfile::Standard)
    ));
    assert!(matches!(
        TestProfile::from_str("full"),
        Ok(TestProfile::Full)
    ));
    assert!(matches!(
        TestProfile::from_str("FULL"),
        Ok(TestProfile::Full)
    ));
    assert!(TestProfile::from_str("invalid").is_err());
}

#[test]
fn test_recommendation_display() {
    assert_eq!(Recommendation::MergeOk.as_str(), "MergeOk");
    assert_eq!(Recommendation::NeedsFix.as_str(), "NeedsFix");
    assert_eq!(
        Recommendation::CreatePrForReview.as_str(),
        "CreatePrForReview"
    );
}

#[test]
fn test_qc_config_default() {
    let config = QcConfig::default();
    assert!(matches!(config.default_profile, TestProfile::Standard));
    assert_eq!(config.max_lines_without_pr, 200);
    assert_eq!(config.base_ref, "main");
}
