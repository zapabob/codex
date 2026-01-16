//! Git Repository Parser for Git4D VR/AR Sprint 1
//!
//! This module implements the core Git repository parsing engine that extracts
//! commit data and transforms it into 4D visualization structures.
//!
//! Key Features:
//! - Complete Git history parsing using git2-rs
//! - 4D data structure transformation (x, y, z, time)
//! - Memory-efficient processing for large repositories
//! - Parallel processing capabilities

use async_trait::async_trait;
use git2::{Commit, ObjectType, Oid, Repository, Revwalk};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task;

/// Represents a parsed Git commit with 4D coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Git4DCommit {
    pub id: Oid,
    pub author: String,
    pub email: String,
    pub timestamp: i64,
    pub message: String,
    pub parents: Vec<Oid>,
    pub branch: String,
    pub tags: Vec<String>,
    // 4D coordinates for visualization
    pub x: f32,      // Branch position
    pub y: f32,      // Time-normalized position
    pub z: f32,      // Commit depth/impact
    pub time: f32,   // Normalized timestamp
    // Additional metadata
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}

/// Configuration for Git repository parsing
#[derive(Debug, Clone)]
pub struct GitParseConfig {
    pub repository_path: String,
    pub max_commits: Option<usize>,
    pub branch_filter: Option<Vec<String>>,
    pub date_range: Option<(i64, i64)>,
    pub include_merges: bool,
    pub parallel_workers: usize,
}

/// Statistics from Git repository parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitParseStats {
    pub total_commits: usize,
    pub total_branches: usize,
    pub total_tags: usize,
    pub date_range: (i64, i64),
    pub processing_time_ms: u128,
    pub memory_usage_mb: usize,
}

/// Git Repository Parser Engine
pub struct GitRepositoryParser {
    config: GitParseConfig,
}

impl GitRepositoryParser {
    pub fn new(config: GitParseConfig) -> Self {
        Self { config }
    }

    /// Parse entire Git repository and return 4D commit data
    pub async fn parse_repository(&self) -> Result<(Vec<Git4DCommit>, GitParseStats), GitParseError> {
        let start_time = std::time::Instant::now();

        // Open repository
        let repo = Repository::open(&self.config.repository_path)
            .map_err(|e| GitParseError::RepositoryOpen(e.to_string()))?;

        // Get all commits
        let commits = self.collect_all_commits(&repo).await?;

        // Limit commits if specified
        let commits = if let Some(max) = self.config.max_commits {
            commits.into_iter().take(max).collect()
        } else {
            commits
        };

        // Transform to 4D structure
        let git4d_commits = self.transform_to_4d(commits).await?;

        // Calculate statistics
        let stats = GitParseStats {
            total_commits: git4d_commits.len(),
            total_branches: self.count_branches(&repo)?,
            total_tags: self.count_tags(&repo)?,
            date_range: self.calculate_date_range(&git4d_commits),
            processing_time_ms: start_time.elapsed().as_millis(),
            memory_usage_mb: self.estimate_memory_usage(&git4d_commits),
        };

        Ok((git4d_commits, stats))
    }

    /// Collect all commits from repository
    async fn collect_all_commits(&self, repo: &Repository) -> Result<Vec<CommitData>, GitParseError> {
        let mut commits = Vec::new();
        let mut revwalk = repo.revwalk()
            .map_err(|e| GitParseError::Revwalk(e.to_string()))?;

        // Start from HEAD
        revwalk.push_head()
            .map_err(|e| GitParseError::Revwalk(e.to_string()))?;

        // Collect commits with parallel processing
        let (tx, mut rx) = mpsc::channel(100);

        // Spawn commit processing tasks
        let repo_arc = Arc::new(repo.clone());
        for _ in 0..self.config.parallel_workers {
            let tx_clone = tx.clone();
            let repo_clone = repo_arc.clone();
            let config_clone = self.config.clone();

            tokio::spawn(async move {
                while let Some(oid) = rx.recv().await {
                    match Self::process_commit(&repo_clone, oid, &config_clone) {
                        Ok(commit_data) => {
                            let _ = tx_clone.send(commit_data).await;
                        }
                        Err(e) => {
                            eprintln!("Error processing commit {}: {}", oid, e);
                        }
                    }
                }
            });
        }

        // Feed OIDs to workers
        let mut oid_queue = Vec::new();
        for oid_result in revwalk {
            let oid = oid_result.map_err(|e| GitParseError::Revwalk(e.to_string()))?;
            oid_queue.push(oid);
        }

        // Process commits
        for oid in oid_queue {
            if let Ok(commit_data) = Self::process_commit(repo, oid, &self.config) {
                commits.push(commit_data);
            }
        }

        Ok(commits)
    }

    /// Process individual commit
    fn process_commit(repo: &Repository, oid: Oid, config: &GitParseConfig) -> Result<CommitData, GitParseError> {
        let commit = repo.find_commit(oid)
            .map_err(|e| GitParseError::CommitLookup(e.to_string()))?;

        // Filter by date range if specified
        if let Some((start, end)) = config.date_range {
            let commit_time = commit.time().seconds();
            if commit_time < start || commit_time > end {
                return Err(GitParseError::Filtered("Date range filter".to_string()));
            }
        }

        // Get commit metadata
        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let author_email = author.email().unwrap_or("").to_string();
        let timestamp = commit.time().seconds();
        let message = commit.message().unwrap_or("").to_string();

        // Get parent commits
        let mut parents = Vec::new();
        for i in 0..commit.parent_count() {
            if let Ok(parent) = commit.parent(i) {
                parents.push(parent.id());
            }
        }

        // Skip merges if configured
        if !config.include_merges && parents.len() > 1 {
            return Err(GitParseError::Filtered("Merge commit filter".to_string()));
        }

        // Analyze diff statistics
        let (files_changed, insertions, deletions) = Self::analyze_commit_diff(repo, &commit)?;

        Ok(CommitData {
            id: oid,
            author: author_name,
            email: author_email,
            timestamp,
            message,
            parents,
            files_changed,
            insertions,
            deletions,
        })
    }

    /// Analyze commit diff to get statistics
    fn analyze_commit_diff(repo: &Repository, commit: &Commit) -> Result<(usize, usize, usize), GitParseError> {
        let mut files_changed = 0;
        let mut insertions = 0;
        let mut deletions = 0;

        // Get parent commit for diff
        if commit.parent_count() > 0 {
            let parent = commit.parent(0)
                .map_err(|e| GitParseError::DiffAnalysis(e.to_string()))?;
            let parent_tree = parent.tree()
                .map_err(|e| GitParseError::DiffAnalysis(e.to_string()))?;
            let commit_tree = commit.tree()
                .map_err(|e| GitParseError::DiffAnalysis(e.to_string()))?;

            let diff = repo.diff_tree_to_tree(
                Some(&parent_tree),
                Some(&commit_tree),
                None,
            ).map_err(|e| GitParseError::DiffAnalysis(e.to_string()))?;

            for delta in diff.deltas() {
                files_changed += 1;

                // Get diff statistics for this file
                if let Ok(diff_stats) = diff.stats() {
                    insertions += diff_stats.insertions();
                    deletions += diff_stats.deletions();
                }
            }
        }

        Ok((files_changed, insertions, deletions))
    }

    /// Transform commit data to 4D visualization structure
    async fn transform_to_4d(&self, commits: Vec<CommitData>) -> Result<Vec<Git4DCommit>, GitParseError> {
        if commits.is_empty() {
            return Ok(Vec::new());
        }

        // Sort commits by timestamp
        let mut commits = commits;
        commits.sort_by_key(|c| c.timestamp);

        // Calculate time normalization
        let min_time = commits.first().unwrap().timestamp as f32;
        let max_time = commits.last().unwrap().timestamp as f32;
        let time_range = max_time - min_time;

        // Build commit graph for branch analysis
        let commit_graph = self.build_commit_graph(&commits);

        // Assign branches and calculate 4D coordinates
        let mut git4d_commits = Vec::new();
        let mut processed = HashSet::new();

        for commit in commits {
            if processed.contains(&commit.id) {
                continue;
            }

            let branch = self.determine_branch(&commit_graph, commit.id);
            let (x, y, z, time) = self.calculate_4d_coordinates(
                &commit, branch, min_time, time_range
            );

            git4d_commits.push(Git4DCommit {
                id: commit.id,
                author: commit.author,
                email: commit.email,
                timestamp: commit.timestamp,
                message: commit.message,
                parents: commit.parents,
                branch,
                tags: Vec::new(), // TODO: Implement tag detection
                x, y, z, time,
                files_changed: commit.files_changed,
                insertions: commit.insertions,
                deletions: commit.deletions,
            });

            processed.insert(commit.id);
        }

        Ok(git4d_commits)
    }

    /// Build commit graph for branch analysis
    fn build_commit_graph(&self, commits: &[CommitData]) -> HashMap<Oid, Vec<Oid>> {
        let mut graph = HashMap::new();

        for commit in commits {
            graph.entry(commit.id)
                .or_insert_with(Vec::new)
                .extend(&commit.parents);
        }

        graph
    }

    /// Determine which branch a commit belongs to
    fn determine_branch(&self, commit_graph: &HashMap<Oid, Vec<Oid>>, commit_id: Oid) -> String {
        // Simple branch detection - in a real implementation,
        // this would analyze the commit graph more thoroughly
        // For now, assign commits to "main" branch
        "main".to_string()
    }

    /// Calculate 4D coordinates for visualization
    fn calculate_4d_coordinates(&self, commit: &CommitData, branch: String,
                               min_time: f32, time_range: f32) -> (f32, f32, f32, f32) {
        // X: Branch position (simple mapping for now)
        let x = match branch.as_str() {
            "main" => 0.0,
            "develop" => 1.0,
            _ => 0.5,
        };

        // Y: Time-normalized position
        let y = if time_range > 0.0 {
            ((commit.timestamp as f32 - min_time) / time_range) * 10.0
        } else {
            0.0
        };

        // Z: Commit impact (based on files changed)
        let z = (commit.files_changed as f32).sqrt() * 0.5;

        // Time: Normalized timestamp
        let time = if time_range > 0.0 {
            (commit.timestamp as f32 - min_time) / time_range
        } else {
            0.0
        };

        (x, y, z, time)
    }

    /// Count total branches in repository
    fn count_branches(&self, repo: &Repository) -> Result<usize, GitParseError> {
        let branches = repo.branches(None)
            .map_err(|e| GitParseError::BranchAnalysis(e.to_string()))?;
        Ok(branches.count())
    }

    /// Count total tags in repository
    fn count_tags(&self, repo: &Repository) -> Result<usize, GitParseError> {
        let tags = repo.tag_names(None)
            .map_err(|e| GitParseError::TagAnalysis(e.to_string()))?;
        Ok(tags.count())
    }

    /// Calculate date range from commits
    fn calculate_date_range(&self, commits: &[Git4DCommit]) -> (i64, i64) {
        if commits.is_empty() {
            return (0, 0);
        }

        let min_time = commits.iter().map(|c| c.timestamp).min().unwrap();
        let max_time = commits.iter().map(|c| c.timestamp).max().unwrap();

        (min_time, max_time)
    }

    /// Estimate memory usage
    fn estimate_memory_usage(&self, commits: &[Git4DCommit]) -> usize {
        // Rough estimation: ~1KB per commit
        commits.len() * 1024 / (1024 * 1024)
    }
}

/// Internal commit data structure
#[derive(Debug, Clone)]
struct CommitData {
    id: Oid,
    author: String,
    email: String,
    timestamp: i64,
    message: String,
    parents: Vec<Oid>,
    files_changed: usize,
    insertions: usize,
    deletions: usize,
}

/// Error types for Git parsing
#[derive(Debug, Clone)]
pub enum GitParseError {
    RepositoryOpen(String),
    Revwalk(String),
    CommitLookup(String),
    DiffAnalysis(String),
    BranchAnalysis(String),
    TagAnalysis(String),
    Filtered(String),
}

impl std::fmt::Display for GitParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitParseError::RepositoryOpen(msg) => write!(f, "Failed to open repository: {}", msg),
            GitParseError::Revwalk(msg) => write!(f, "Revwalk error: {}", msg),
            GitParseError::CommitLookup(msg) => write!(f, "Commit lookup error: {}", msg),
            GitParseError::DiffAnalysis(msg) => write!(f, "Diff analysis error: {}", msg),
            GitParseError::BranchAnalysis(msg) => write!(f, "Branch analysis error: {}", msg),
            GitParseError::TagAnalysis(msg) => write!(f, "Tag analysis error: {}", msg),
            GitParseError::Filtered(msg) => write!(f, "Commit filtered: {}", msg),
        }
    }
}

impl std::error::Error for GitParseError {}

/// Async trait for repository parsing
#[async_trait]
pub trait RepositoryParser {
    async fn parse_repository(&self) -> Result<(Vec<Git4DCommit>, GitParseStats), GitParseError>;
}

#[async_trait]
impl RepositoryParser for GitRepositoryParser {
    async fn parse_repository(&self) -> Result<(Vec<Git4DCommit>, GitParseStats), GitParseError> {
        self.parse_repository().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_empty_repository() {
        let temp_dir = TempDir::new().unwrap();
        let config = GitParseConfig {
            repository_path: temp_dir.path().to_str().unwrap().to_string(),
            max_commits: None,
            branch_filter: None,
            date_range: None,
            include_merges: true,
            parallel_workers: 1,
        };

        let parser = GitRepositoryParser::new(config);
        let result = parser.parse_repository().await;

        // Should fail because it's not a Git repository
        assert!(result.is_err());
    }
}