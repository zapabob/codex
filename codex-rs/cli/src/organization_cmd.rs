//! Organization management commands

use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use codex_core::organizations::OrganizationManager;
use codex_core::organizations::OrganizationSkillsRepository;
use codex_core::skills::SkillsSharingManager;
use std::path::PathBuf;
use std::sync::Arc;

/// Organization management commands
#[derive(Debug, Parser)]
pub struct OrganizationCli {
    #[clap(subcommand)]
    pub command: OrganizationCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum OrganizationCommand {
    /// Create a new organization
    Create {
        /// Organization name
        #[clap(value_name = "NAME")]
        name: String,
        /// Organization description
        #[clap(long)]
        description: Option<String>,
        /// User ID of the creator
        #[clap(long)]
        creator: String,
    },
    /// Join an organization
    Join {
        /// Organization ID
        #[clap(value_name = "ORG_ID")]
        org_id: String,
        /// User ID
        #[clap(long)]
        user_id: String,
        /// Role (admin, member, viewer)
        #[clap(long, default_value = "member")]
        role: String,
    },
    /// Share a skill with an organization
    ShareSkill {
        /// Organization ID
        #[clap(long)]
        org_id: String,
        /// Skill name
        #[clap(long)]
        skill_name: String,
        /// Skill version
        #[clap(long)]
        version: String,
        /// Skill file path
        #[clap(long)]
        skill_file: PathBuf,
        /// User ID
        #[clap(long)]
        user_id: String,
        /// Access level (public, members_only, admin_only)
        #[clap(long, default_value = "members_only")]
        access_level: String,
    },
    /// List shared skills in an organization
    ListSkills {
        /// Organization ID
        #[clap(long)]
        org_id: String,
        /// User ID
        #[clap(long)]
        user_id: String,
    },
    /// Get skill usage statistics
    Statistics {
        /// Organization ID
        #[clap(long)]
        org_id: String,
        /// User ID
        #[clap(long)]
        user_id: String,
    },
}

/// Run organization command
pub async fn run_organization_command(cli: OrganizationCli, codex_home: PathBuf) -> Result<()> {
    let database_path = codex_home.join(".codex").join("organizations.db");
    let org_manager = Arc::new(
        OrganizationManager::new(database_path.clone())
            .await
            .context("Failed to create organization manager")?,
    );

    let pool = org_manager.pool.clone();
    let skills_repo = Arc::new(OrganizationSkillsRepository::new(pool));
    let sharing_manager = SkillsSharingManager::new(org_manager.clone(), skills_repo.clone());

    match cli.command {
        OrganizationCommand::Create {
            name,
            description,
            creator,
        } => {
            let org = org_manager
                .create_organization(name, description, creator)
                .await?;
            println!("Created organization: {}", org.id);
            println!("  Name: {}", org.name);
            if let Some(desc) = org.description {
                println!("  Description: {}", desc);
            }
        }
        OrganizationCommand::Join {
            org_id,
            user_id,
            role,
        } => {
            let member = org_manager.add_member(&org_id, &user_id, &role).await?;
            println!("Joined organization: {}", org_id);
            println!("  User ID: {}", member.user_id);
            println!("  Role: {}", member.role);
        }
        OrganizationCommand::ShareSkill {
            org_id,
            skill_name,
            version,
            skill_file,
            user_id,
            access_level,
        } => {
            let content = std::fs::read_to_string(&skill_file).context(format!(
                "Failed to read skill file: {}",
                skill_file.display()
            ))?;

            sharing_manager
                .share_skill(
                    &org_id,
                    skill_name.clone(),
                    version.clone(),
                    content,
                    user_id,
                    access_level.clone(),
                )
                .await?;

            println!(
                "Shared skill {} v{} with organization {}",
                skill_name, version, org_id
            );
            println!("  Access level: {}", access_level);
        }
        OrganizationCommand::ListSkills { org_id, user_id } => {
            let skills = sharing_manager.get_shared_skills(&org_id, &user_id).await?;

            println!("Shared skills in organization {}:", org_id);
            for skill in skills {
                println!(
                    "  - {} v{} ({} uses)",
                    skill.skill_name, skill.version, skill.usage_count
                );
                println!("    Shared by: {}", skill.shared_by);
                println!("    Access level: {}", skill.access_level);
            }
        }
        OrganizationCommand::Statistics { org_id, user_id } => {
            let stats = sharing_manager
                .get_usage_statistics(&org_id, &user_id)
                .await?;
            println!("{}", serde_json::to_string_pretty(&stats)?);
        }
    }

    Ok(())
}
