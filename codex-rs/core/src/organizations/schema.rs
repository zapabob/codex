//! Database schema for organizations

use serde::Deserialize;
use serde::Serialize;
use std::time::SystemTime;

/// Organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    /// Organization ID
    pub id: String,
    /// Organization name
    pub name: String,
    /// Organization description
    pub description: Option<String>,
    /// Created timestamp
    pub created_at: SystemTime,
    /// Updated timestamp
    pub updated_at: SystemTime,
}

/// Organization member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    /// Member ID
    pub id: String,
    /// Organization ID
    pub organization_id: String,
    /// User ID
    pub user_id: String,
    /// Role (admin, member, viewer)
    pub role: String,
    /// Joined timestamp
    pub joined_at: SystemTime,
}

/// Shared skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillShare {
    /// Share ID
    pub id: String,
    /// Organization ID
    pub organization_id: String,
    /// Skill name
    pub skill_name: String,
    /// Skill version
    pub version: String,
    /// Skill content (YAML)
    pub content: String,
    /// Shared by user ID
    pub shared_by: String,
    /// Shared timestamp
    pub shared_at: SystemTime,
    /// Access level (public, members_only, admin_only)
    pub access_level: String,
    /// Usage count
    pub usage_count: u64,
}
