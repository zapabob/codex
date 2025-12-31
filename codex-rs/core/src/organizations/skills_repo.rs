//! Organization Skills repository

use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Row};
use std::sync::Arc;
use std::time::SystemTime;
use tracing::info;
use uuid::Uuid;

use crate::organizations::schema::SkillShare;

/// Organization Skills repository
pub struct OrganizationSkillsRepository {
    /// Database connection pool
    pool: Arc<SqlitePool>,
}

impl OrganizationSkillsRepository {
    /// Create a new Skills repository
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    /// Share a skill with an organization
    pub async fn share_skill(
        &self,
        org_id: &str,
        skill_name: String,
        version: String,
        content: String,
        shared_by: String,
        access_level: String,
    ) -> Result<SkillShare> {
        let id = Uuid::new_v4().to_string();
        let now = SystemTime::now();

        sqlx::query(
            r#"
            INSERT INTO skill_shares 
            (id, organization_id, skill_name, version, content, shared_by, shared_at, access_level, usage_count)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0)
            "#,
        )
        .bind(&id)
        .bind(org_id)
        .bind(&skill_name)
        .bind(&version)
        .bind(&content)
        .bind(&shared_by)
        .bind(now.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64)
        .bind(&access_level)
        .execute(&*self.pool)
        .await?;

        let share = SkillShare {
            id,
            organization_id: org_id.to_string(),
            skill_name,
            version,
            content,
            shared_by,
            shared_at: now,
            access_level,
            usage_count: 0,
        };

        info!("Shared skill {} v{} with organization {}", share.skill_name, share.version, org_id);
        Ok(share)
    }

    /// Get shared skills for an organization
    pub async fn get_shared_skills(&self, org_id: &str) -> Result<Vec<SkillShare>> {
        let rows = sqlx::query(
            r#"
            SELECT id, organization_id, skill_name, version, content, shared_by, shared_at, access_level, usage_count
            FROM skill_shares
            WHERE organization_id = ?
            ORDER BY shared_at DESC
            "#,
        )
        .bind(org_id)
        .fetch_all(&*self.pool)
        .await?;

        let skills: Vec<SkillShare> = rows
            .into_iter()
            .map(|row| {
                let timestamp_secs = row.get::<i64, _>(6) as u64;
                SkillShare {
                    id: row.get(0),
                    organization_id: row.get(1),
                    skill_name: row.get(2),
                    version: row.get(3),
                    content: row.get(4),
                    shared_by: row.get(5),
                    shared_at: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp_secs),
                    access_level: row.get(7),
                    usage_count: row.get::<i64, _>(8) as u64,
                }
            })
            .collect();

        Ok(skills)
    }

    /// Get a specific shared skill
    pub async fn get_skill(
        &self,
        org_id: &str,
        skill_name: &str,
        version: Option<&str>,
    ) -> Result<Option<SkillShare>> {
        let query = if let Some(version) = version {
            sqlx::query(
                r#"
                SELECT id, organization_id, skill_name, version, content, shared_by, shared_at, access_level, usage_count
                FROM skill_shares
                WHERE organization_id = ? AND skill_name = ? AND version = ?
                "#,
            )
            .bind(org_id)
            .bind(skill_name)
            .bind(version)
        } else {
            sqlx::query(
                r#"
                SELECT id, organization_id, skill_name, version, content, shared_by, shared_at, access_level, usage_count
                FROM skill_shares
                WHERE organization_id = ? AND skill_name = ?
                ORDER BY shared_at DESC
                LIMIT 1
                "#,
            )
            .bind(org_id)
            .bind(skill_name)
        };

        let row = query.fetch_optional(&*self.pool).await?;

        if let Some(row) = row {
            let timestamp_secs = row.get::<i64, _>(6) as u64;
            Ok(Some(SkillShare {
                id: row.get(0),
                organization_id: row.get(1),
                skill_name: row.get(2),
                version: row.get(3),
                content: row.get(4),
                shared_by: row.get(5),
                shared_at: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp_secs),
                access_level: row.get(7),
                usage_count: row.get::<i64, _>(8) as u64,
            }))
        } else {
            Ok(None)
        }
    }

    /// Increment usage count for a skill
    pub async fn increment_usage(&self, share_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE skill_shares
            SET usage_count = usage_count + 1
            WHERE id = ?
            "#,
        )
        .bind(share_id)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    /// Get skill usage statistics
    pub async fn get_usage_statistics(&self, org_id: &str) -> Result<serde_json::Value> {
        let rows = sqlx::query(
            r#"
            SELECT skill_name, SUM(usage_count) as total_usage, COUNT(*) as share_count
            FROM skill_shares
            WHERE organization_id = ?
            GROUP BY skill_name
            ORDER BY total_usage DESC
            "#,
        )
        .bind(org_id)
        .fetch_all(&*self.pool)
        .await?;

        let stats: Vec<serde_json::Value> = rows
            .into_iter()
            .map(|row| {
                serde_json::json!({
                    "skill_name": row.get::<String, _>(0),
                    "total_usage": row.get::<i64, _>(1),
                    "share_count": row.get::<i64, _>(2),
                })
            })
            .collect();

        Ok(serde_json::json!({
            "organization_id": org_id,
            "statistics": stats
        }))
    }
}
