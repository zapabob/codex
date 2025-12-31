//! Skills sharing functionality for organizations
//!
//! Provides organization-level Skills sharing, versioning, and access control

use anyhow::Result;
use std::sync::Arc;
use tracing::info;

use crate::organizations::OrganizationManager;
use crate::organizations::OrganizationSkillsRepository;

/// Skills sharing manager
pub struct SkillsSharingManager {
    /// Organization manager
    org_manager: Arc<OrganizationManager>,
    /// Skills repository
    skills_repo: Arc<OrganizationSkillsRepository>,
}

impl SkillsSharingManager {
    /// Create a new Skills sharing manager
    pub fn new(
        org_manager: Arc<OrganizationManager>,
        skills_repo: Arc<OrganizationSkillsRepository>,
    ) -> Self {
        Self {
            org_manager,
            skills_repo,
        }
    }

    /// Share a skill with an organization
    pub async fn share_skill(
        &self,
        org_id: &str,
        skill_name: String,
        version: String,
        skill_content: String,
        user_id: String,
        access_level: String,
    ) -> Result<()> {
        // Verify user is a member
        if !self.org_manager.is_member(org_id, &user_id).await? {
            return Err(anyhow::anyhow!("User is not a member of the organization"));
        }

        // Check access level permissions
        let user_role = self.org_manager.get_user_role(org_id, &user_id).await?;
        if let Some(role) = user_role
            && access_level == "admin_only" && role != "admin" {
                return Err(anyhow::anyhow!(
                    "Only admins can share skills with admin_only access"
                ));
            }

        self.skills_repo
            .share_skill(
                org_id,
                skill_name,
                version,
                skill_content,
                user_id,
                access_level,
            )
            .await?;

        info!("Skill shared successfully");
        Ok(())
    }

    /// Get shared skills for an organization
    pub async fn get_shared_skills(
        &self,
        org_id: &str,
        user_id: &str,
    ) -> Result<Vec<crate::organizations::schema::SkillShare>> {
        // Verify user is a member
        if !self.org_manager.is_member(org_id, user_id).await? {
            return Err(anyhow::anyhow!("User is not a member of the organization"));
        }

        let skills = self.skills_repo.get_shared_skills(org_id).await?;

        // Filter by access level
        let user_role = self.org_manager.get_user_role(org_id, user_id).await?;
        let filtered: Vec<_> = skills
            .into_iter()
            .filter(|skill| match skill.access_level.as_str() {
                "public" => true,
                "members_only" => true,
                "admin_only" => user_role.as_deref() == Some("admin"),
                _ => false,
            })
            .collect();

        Ok(filtered)
    }

    /// Get a specific shared skill
    pub async fn get_skill(
        &self,
        org_id: &str,
        skill_name: &str,
        version: Option<&str>,
        user_id: &str,
    ) -> Result<Option<crate::organizations::schema::SkillShare>> {
        // Verify user is a member
        if !self.org_manager.is_member(org_id, user_id).await? {
            return Err(anyhow::anyhow!("User is not a member of the organization"));
        }

        let skill = self
            .skills_repo
            .get_skill(org_id, skill_name, version)
            .await?;

        if let Some(skill) = skill {
            // Check access level
            let user_role = self.org_manager.get_user_role(org_id, user_id).await?;
            let has_access = match skill.access_level.as_str() {
                "public" => true,
                "members_only" => true,
                "admin_only" => user_role.as_deref() == Some("admin"),
                _ => false,
            };

            if has_access {
                // Increment usage count
                self.skills_repo.increment_usage(&skill.id).await?;
                Ok(Some(skill))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Get usage statistics for an organization
    pub async fn get_usage_statistics(
        &self,
        org_id: &str,
        user_id: &str,
    ) -> Result<serde_json::Value> {
        // Verify user is a member
        if !self.org_manager.is_member(org_id, user_id).await? {
            return Err(anyhow::anyhow!("User is not a member of the organization"));
        }

        self.skills_repo.get_usage_statistics(org_id).await
    }
}
