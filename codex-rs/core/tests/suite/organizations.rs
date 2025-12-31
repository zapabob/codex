#![cfg(not(target_os = "windows"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use anyhow::Result;
use codex_core::organizations::{OrganizationManager, OrganizationSkillsRepository};
use codex_core::skills::SkillsSharingManager;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

fn create_temp_db() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_organizations.db");
    (temp_dir, db_path)
}

#[tokio::test]
async fn test_create_organization() -> Result<()> {
    let (_temp_dir, db_path) = create_temp_db();
    let manager = OrganizationManager::new(db_path).await?;

    let org = manager
        .create_organization(
            "Test Org".to_string(),
            Some("Test Description".to_string()),
            "user-1".to_string(),
        )
        .await?;

    assert_eq!(org.name, "Test Org");
    assert_eq!(org.description, Some("Test Description".to_string()));
    assert!(!org.id.is_empty());

    // Verify creator is added as admin
    let members = manager.get_members(&org.id).await?;
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].user_id, "user-1");
    assert_eq!(members[0].role, "admin");

    Ok(())
}

#[tokio::test]
async fn test_add_member() -> Result<()> {
    let (_temp_dir, db_path) = create_temp_db();
    let manager = OrganizationManager::new(db_path).await?;

    let org = manager
        .create_organization("Test Org".to_string(), None, "user-1".to_string())
        .await?;

    let member = manager.add_member(&org.id, "user-2", "member").await?;
    assert_eq!(member.user_id, "user-2");
    assert_eq!(member.role, "member");

    let members = manager.get_members(&org.id).await?;
    assert_eq!(members.len(), 2);

    Ok(())
}

#[tokio::test]
async fn test_share_skill() -> Result<()> {
    let (_temp_dir, db_path) = create_temp_db();
    let manager = Arc::new(OrganizationManager::new(db_path.clone()).await?);
    let pool = manager.pool.clone();
    let skills_repo = Arc::new(OrganizationSkillsRepository::new(pool));
    let sharing_manager = SkillsSharingManager::new(manager.clone(), skills_repo.clone());

    let org = manager
        .create_organization("Test Org".to_string(), None, "user-1".to_string())
        .await?;

    sharing_manager
        .share_skill(
            &org.id,
            "test-skill".to_string(),
            "1.0.0".to_string(),
            "skill content".to_string(),
            "user-1".to_string(),
            "members_only".to_string(),
        )
        .await?;

    let skills = sharing_manager.get_shared_skills(&org.id, "user-1").await?;
    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0].skill_name, "test-skill");
    assert_eq!(skills[0].version, "1.0.0");
    assert_eq!(skills[0].shared_by, "user-1");

    Ok(())
}

#[tokio::test]
async fn test_get_usage_statistics() -> Result<()> {
    let (_temp_dir, db_path) = create_temp_db();
    let manager = Arc::new(OrganizationManager::new(db_path.clone()).await?);
    let pool = manager.pool.clone();
    let skills_repo = Arc::new(OrganizationSkillsRepository::new(pool));
    let sharing_manager = SkillsSharingManager::new(manager.clone(), skills_repo.clone());

    let org = manager
        .create_organization("Test Org".to_string(), None, "user-1".to_string())
        .await?;

    sharing_manager
        .share_skill(
            &org.id,
            "test-skill".to_string(),
            "1.0.0".to_string(),
            "skill content".to_string(),
            "user-1".to_string(),
            "members_only".to_string(),
        )
        .await?;

    let stats = sharing_manager.get_usage_statistics(&org.id, "user-1").await?;
    assert!(stats.total_skills >= 1);
    assert!(stats.total_usage >= 0);

    Ok(())
}
