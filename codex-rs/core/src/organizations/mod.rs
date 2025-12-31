//! Organizations module for Skills sharing and management
//!
//! Provides organization-level Skills sharing, versioning, and access control

pub mod manager;
pub mod schema;
pub mod skills_repo;

pub use manager::OrganizationManager;
pub use schema::Member;
pub use schema::Organization;
pub use schema::SkillShare;
pub use skills_repo::OrganizationSkillsRepository;
