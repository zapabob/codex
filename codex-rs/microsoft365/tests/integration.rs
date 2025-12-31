#![cfg(not(target_os = "windows"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use anyhow::Result;
use codex_microsoft365::Microsoft365AuthManager;
use codex_microsoft365::Microsoft365Client;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_temp_home() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let home_path = temp_dir.path().to_path_buf();
    (temp_dir, home_path)
}

#[tokio::test]
async fn test_auth_manager_creation() -> Result<()> {
    let (_temp_dir, home_path) = create_temp_home();

    let auth_manager = Microsoft365AuthManager::new(
        "test-client-id".to_string(),
        "test-tenant-id".to_string(),
        "http://localhost:8080/redirect".to_string(),
        home_path,
    )?;

    // Verify auth manager is created by testing authorization URL generation
    let (auth_url, _csrf_token) =
        auth_manager.get_authorization_url(vec!["Files.ReadWrite".to_string()])?;

    // Verify URL contains expected components
    assert!(auth_url.contains("login.microsoftonline.com"));
    assert!(auth_url.contains("test-tenant-id") || auth_url.contains("oauth2"));

    Ok(())
}

#[tokio::test]
async fn test_client_creation() -> Result<()> {
    let (_temp_dir, home_path) = create_temp_home();

    let auth_manager = Microsoft365AuthManager::new(
        "test-client-id".to_string(),
        "test-tenant-id".to_string(),
        "http://localhost:8080/redirect".to_string(),
        home_path,
    )?;

    let client = Microsoft365Client::new(std::sync::Arc::new(auth_manager));

    // Verify client is created
    // Note: Actual API calls would require valid tokens
    // The client structure is verified by successful creation

    Ok(())
}

#[tokio::test]
async fn test_auth_url_generation() -> Result<()> {
    let (_temp_dir, home_path) = create_temp_home();

    let auth_manager = Microsoft365AuthManager::new(
        "test-client-id".to_string(),
        "test-tenant-id".to_string(),
        "http://localhost:8080/redirect".to_string(),
        home_path,
    )?;

    // Generate auth URL with scopes
    let (auth_url, csrf_token) = auth_manager.get_authorization_url(vec![
        "Files.ReadWrite".to_string(),
        "Mail.ReadWrite".to_string(),
    ])?;

    // Verify URL contains expected components
    assert!(auth_url.contains("login.microsoftonline.com"));
    assert!(!csrf_token.secret().is_empty());

    Ok(())
}
