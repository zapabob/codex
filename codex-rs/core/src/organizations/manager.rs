//! Organization management system

use anyhow::Context;
use anyhow::Result;
use sqlx::Row;
use sqlx::sqlite::SqlitePool;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use tracing::info;
use uuid::Uuid;

use crate::organizations::schema::Member;
use crate::organizations::schema::Organization;
use crate::organizations::schema::SkillShare;

/// Organization manager
pub struct OrganizationManager {
    /// Database connection pool
    pub pool: Arc<SqlitePool>,
}

impl OrganizationManager {
    /// Create a new organization manager
    pub async fn new(database_path: PathBuf) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = database_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let database_url = format!("sqlite:{}", database_path.display());
        let pool = SqlitePool::connect(&database_url)
            .await
            .context("Failed to connect to database")?;

        // Initialize schema
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS organizations (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS members (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                role TEXT NOT NULL,
                joined_at TEXT NOT NULL,
                FOREIGN KEY (organization_id) REFERENCES organizations(id)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS skill_shares (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                skill_name TEXT NOT NULL,
                version TEXT NOT NULL,
                content TEXT NOT NULL,
                shared_by TEXT NOT NULL,
                shared_at TEXT NOT NULL,
                access_level TEXT NOT NULL,
                usage_count INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (organization_id) REFERENCES organizations(id),
                UNIQUE(organization_id, skill_name, version)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        info!("Organization manager initialized");
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// Create a new organization
    pub async fn create_organization(
        &self,
        name: String,
        description: Option<String>,
        creator_user_id: String,
    ) -> Result<Organization> {
        let id = Uuid::new_v4().to_string();
        let now = SystemTime::now();

        sqlx::query(
            r#"
            INSERT INTO organizations (id, name, description, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&name)
        .bind(&description)
        .bind(now.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64)
        .bind(now.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64)
        .execute(&*self.pool)
        .await?;

        // Add creator as admin
        self.add_member(&id, &creator_user_id, "admin").await?;

        let org = Organization {
            id,
            name,
            description,
            created_at: now,
            updated_at: now,
        };

        info!("Created organization: {}", org.name);
        Ok(org)
    }

    /// Get an organization by ID
    pub async fn get_organization(&self, org_id: &str) -> Result<Option<Organization>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM organizations
            WHERE id = ?
            "#,
        )
        .bind(org_id)
        .fetch_optional(&*self.pool)
        .await?;

        if let Some(row) = row {
            let created_secs = row.get::<i64, _>(3) as u64;
            let updated_secs = row.get::<i64, _>(4) as u64;
            Ok(Some(Organization {
                id: row.get(0),
                name: row.get(1),
                description: row.get(2),
                created_at: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(created_secs),
                updated_at: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(updated_secs),
            }))
        } else {
            Ok(None)
        }
    }

    /// Add a member to an organization
    pub async fn add_member(&self, org_id: &str, user_id: &str, role: &str) -> Result<Member> {
        let id = Uuid::new_v4().to_string();
        let now = SystemTime::now();

        sqlx::query(
            r#"
            INSERT INTO members (id, organization_id, user_id, role, joined_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(org_id)
        .bind(user_id)
        .bind(role)
        .bind(now.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64)
        .execute(&*self.pool)
        .await?;

        let member = Member {
            id,
            organization_id: org_id.to_string(),
            user_id: user_id.to_string(),
            role: role.to_string(),
            joined_at: now,
        };

        info!("Added member {} to organization {}", user_id, org_id);
        Ok(member)
    }

    /// Get members of an organization
    pub async fn get_members(&self, org_id: &str) -> Result<Vec<Member>> {
        let rows = sqlx::query(
            r#"
            SELECT id, organization_id, user_id, role, joined_at
            FROM members
            WHERE organization_id = ?
            "#,
        )
        .bind(org_id)
        .fetch_all(&*self.pool)
        .await?;

        let members: Vec<Member> = rows
            .into_iter()
            .map(|row| {
                let joined_secs = row.get::<i64, _>(4) as u64;
                Member {
                    id: row.get(0),
                    organization_id: row.get(1),
                    user_id: row.get(2),
                    role: row.get(3),
                    joined_at: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(joined_secs),
                }
            })
            .collect();

        Ok(members)
    }

    /// Check if user is a member of an organization
    pub async fn is_member(&self, org_id: &str, user_id: &str) -> Result<bool> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) FROM members
            WHERE organization_id = ? AND user_id = ?
            "#,
        )
        .bind(org_id)
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await?;

        Ok(row.get::<i64, _>(0) > 0)
    }

    /// Get user's role in an organization
    pub async fn get_user_role(&self, org_id: &str, user_id: &str) -> Result<Option<String>> {
        let row = sqlx::query(
            r#"
            SELECT role FROM members
            WHERE organization_id = ? AND user_id = ?
            "#,
        )
        .bind(org_id)
        .bind(user_id)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(row.map(|r| r.get(0)))
    }
}
