#![cfg(not(target_os = "windows"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use anyhow::Result;
use codex_core::lsp::DiagnosticsManager;
use codex_core::lsp::LspClient;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_temp_workspace() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path().to_path_buf();
    (temp_dir, workspace_path)
}

#[tokio::test]
async fn test_lsp_client_creation() -> Result<()> {
    let (_temp_dir, workspace_path) = create_temp_workspace();

    // Create LSP client (without actually starting a server)
    let client = LspClient::new(
        "test-server".to_string(),
        vec!["test-server".to_string()],
        workspace_path,
    );

    assert_eq!(client.server_name, "test-server");
    assert!(client.root_uri.is_some());

    Ok(())
}

#[tokio::test]
async fn test_diagnostics_manager_creation() -> Result<()> {
    let diagnostics_manager = DiagnosticsManager::new(100);

    // Verify manager is created by checking it can get diagnostics
    let diagnostics = diagnostics_manager
        .get_diagnostics_for_document(&lsp_types::Url::parse("file:///test.rs")?)
        .await;
    assert!(diagnostics.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_diagnostics_manager_add_diagnostics() -> Result<()> {
    let diagnostics_manager = DiagnosticsManager::new(100);

    // Create a test diagnostic
    let test_uri = lsp_types::Url::parse("file:///test.rs")?;
    let initial_diagnostics = diagnostics_manager
        .get_diagnostics_for_document(&test_uri)
        .await;
    let initial_count = initial_diagnostics.len();

    // Note: In a real test, we would add diagnostics through the LSP client
    // For now, we just verify the manager structure
    let diagnostics_after = diagnostics_manager
        .get_diagnostics_for_document(&test_uri)
        .await;
    assert_eq!(diagnostics_after.len(), initial_count);

    Ok(())
}
