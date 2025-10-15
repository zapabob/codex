//! File edit conflict resolution for multi-agent orchestration.
//!
//! Provides mechanisms to track and resolve conflicts when multiple agents
//! attempt to edit the same files concurrently.

use anyhow::{Context, Result};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Strategy for resolving edit conflicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeStrategy {
    /// Execute edits sequentially (safe but slower)
    Sequential,
    /// Attempt three-way merge (faster but may fail)
    ThreeWayMerge,
    /// Last write wins (fast but risky)
    LastWriteWins,
}

impl Default for MergeStrategy {
    fn default() -> Self {
        Self::Sequential
    }
}

/// Token representing permission to edit a file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EditToken {
    /// File path being edited
    pub file_path: PathBuf,
    /// Agent that requested the edit
    pub agent_name: String,
    /// Unique edit ID
    pub edit_id: uuid::Uuid,
}

/// A single edit operation on a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditOperation {
    /// Agent performing the edit
    pub agent_name: String,
    /// Original content (before edit)
    pub original_content: Option<String>,
    /// New content (after edit)
    pub new_content: String,
    /// Edit timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Edit ID
    pub edit_id: uuid::Uuid,
}

/// Merged content from multiple edits.
#[derive(Debug, Clone)]
pub struct MergedContent {
    /// Final merged content
    pub content: String,
    /// Whether conflicts were detected
    pub had_conflicts: bool,
    /// Agents that contributed to the merge
    pub contributors: Vec<String>,
}

/// Tracks file edits and manages conflicts.
pub struct FileEditTracker {
    /// Map of file paths to their edit queues
    file_edits: DashMap<PathBuf, Arc<RwLock<Vec<EditOperation>>>>,
    /// Default merge strategy
    strategy: MergeStrategy,
}

impl FileEditTracker {
    /// Create a new file edit tracker.
    pub fn new(strategy: MergeStrategy) -> Self {
        Self {
            file_edits: DashMap::new(),
            strategy,
        }
    }

    /// Request permission to edit a file.
    ///
    /// Returns an `EditToken` that must be used to commit the edit.
    pub async fn request_edit(&self, file: PathBuf, agent: String) -> EditToken {
        let edit_id = uuid::Uuid::new_v4();

        // Ensure the file has an edit queue
        self.file_edits
            .entry(file.clone())
            .or_insert_with(|| Arc::new(RwLock::new(Vec::new())));

        debug!(
            "Agent '{}' requested edit permission for {:?} (ID: {})",
            agent, file, edit_id
        );

        EditToken {
            file_path: file,
            agent_name: agent,
            edit_id,
        }
    }

    /// Commit an edit using the provided token.
    ///
    /// This adds the edit to the queue and waits if sequential execution is required.
    pub async fn commit_edit(
        &self,
        token: EditToken,
        original_content: Option<String>,
        new_content: String,
    ) -> Result<()> {
        let edit_op = EditOperation {
            agent_name: token.agent_name.clone(),
            original_content,
            new_content,
            timestamp: chrono::Utc::now(),
            edit_id: token.edit_id,
        };

        if let Some(edit_queue) = self.file_edits.get(&token.file_path) {
            let mut queue = edit_queue.write().await;
            queue.push(edit_op);

            info!(
                "Agent '{}' committed edit to {:?} (ID: {}, queue length: {})",
                token.agent_name,
                token.file_path,
                token.edit_id,
                queue.len()
            );
        } else {
            anyhow::bail!("Edit token for non-existent file: {:?}", token.file_path);
        }

        Ok(())
    }

    /// Resolve conflicts for a file and return merged content.
    pub async fn resolve_conflicts(&self, file: &Path) -> Result<MergedContent> {
        let edit_queue = self
            .file_edits
            .get(file)
            .context("No edits found for file")?;

        let queue = edit_queue.read().await;

        if queue.is_empty() {
            anyhow::bail!("No edits to resolve for {:?}", file);
        }

        // If only one edit, no conflict
        if queue.len() == 1 {
            return Ok(MergedContent {
                content: queue[0].new_content.clone(),
                had_conflicts: false,
                contributors: vec![queue[0].agent_name.clone()],
            });
        }

        // Multiple edits - resolve based on strategy
        match self.strategy {
            MergeStrategy::Sequential => self.resolve_sequential(&queue).await,
            MergeStrategy::ThreeWayMerge => self.resolve_three_way(&queue).await,
            MergeStrategy::LastWriteWins => self.resolve_last_write_wins(&queue).await,
        }
    }

    /// Resolve conflicts using sequential strategy (last edit wins).
    async fn resolve_sequential(&self, queue: &[EditOperation]) -> Result<MergedContent> {
        let last_edit = queue.last().context("Empty edit queue")?;

        info!(
            "Resolving {} edits sequentially, using last edit from '{}'",
            queue.len(),
            last_edit.agent_name
        );

        Ok(MergedContent {
            content: last_edit.new_content.clone(),
            had_conflicts: queue.len() > 1,
            contributors: queue.iter().map(|e| e.agent_name.clone()).collect(),
        })
    }

    /// Resolve conflicts using three-way merge strategy.
    async fn resolve_three_way(&self, queue: &[EditOperation]) -> Result<MergedContent> {
        warn!(
            "Three-way merge not yet implemented, falling back to sequential for {} edits",
            queue.len()
        );

        // TODO: Implement actual three-way merge using `similar` crate
        // For now, fallback to sequential
        self.resolve_sequential(queue).await
    }

    /// Resolve conflicts using last-write-wins strategy.
    async fn resolve_last_write_wins(&self, queue: &[EditOperation]) -> Result<MergedContent> {
        // Sort by timestamp and take the latest
        let mut sorted = queue.to_vec();
        sorted.sort_by_key(|e| e.timestamp);

        let latest = sorted.last().context("Empty edit queue")?;

        info!(
            "Resolving {} edits using last-write-wins, winner: '{}'",
            queue.len(),
            latest.agent_name
        );

        Ok(MergedContent {
            content: latest.new_content.clone(),
            had_conflicts: queue.len() > 1,
            contributors: queue.iter().map(|e| e.agent_name.clone()).collect(),
        })
    }

    /// Get the number of pending edits for a file.
    pub async fn pending_edits_count(&self, file: &Path) -> usize {
        if let Some(edit_queue) = self.file_edits.get(file) {
            edit_queue.read().await.len()
        } else {
            0
        }
    }

    /// Clear all edits for a file.
    pub async fn clear_edits(&self, file: &Path) {
        if let Some(edit_queue) = self.file_edits.get(file) {
            edit_queue.write().await.clear();
            debug!("Cleared all edits for {:?}", file);
        }
    }
}

/// Conflict resolver that integrates with the orchestrator.
pub struct ConflictResolver {
    /// File edit tracker
    tracker: Arc<FileEditTracker>,
}

impl ConflictResolver {
    /// Create a new conflict resolver with the specified strategy.
    pub fn new(strategy: MergeStrategy) -> Self {
        Self {
            tracker: Arc::new(FileEditTracker::new(strategy)),
        }
    }

    /// Get a reference to the file edit tracker.
    pub fn tracker(&self) -> Arc<FileEditTracker> {
        self.tracker.clone()
    }

    /// Resolve all pending conflicts and return a map of file paths to merged content.
    pub async fn resolve_all(&self) -> Result<Vec<(PathBuf, MergedContent)>> {
        let mut results = Vec::new();

        for entry in self.tracker.file_edits.iter() {
            let file_path = entry.key().clone();
            match self.tracker.resolve_conflicts(&file_path).await {
                Ok(merged) => {
                    results.push((file_path.clone(), merged));
                }
                Err(e) => {
                    warn!("Failed to resolve conflicts for {:?}: {}", file_path, e);
                }
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_single_edit_no_conflict() {
        let tracker = FileEditTracker::new(MergeStrategy::Sequential);
        let file = PathBuf::from("test.txt");

        let token = tracker
            .request_edit(file.clone(), "agent1".to_string())
            .await;
        tracker
            .commit_edit(token, None, "Hello, world!".to_string())
            .await
            .unwrap();

        let merged = tracker.resolve_conflicts(&file).await.unwrap();
        assert_eq!(merged.content, "Hello, world!");
        assert!(!merged.had_conflicts);
        assert_eq!(merged.contributors, vec!["agent1"]);
    }

    #[tokio::test]
    async fn test_multiple_edits_sequential() {
        let tracker = FileEditTracker::new(MergeStrategy::Sequential);
        let file = PathBuf::from("test.txt");

        let token1 = tracker
            .request_edit(file.clone(), "agent1".to_string())
            .await;
        let token2 = tracker
            .request_edit(file.clone(), "agent2".to_string())
            .await;

        tracker
            .commit_edit(token1, None, "First edit".to_string())
            .await
            .unwrap();
        tracker
            .commit_edit(token2, None, "Second edit".to_string())
            .await
            .unwrap();

        let merged = tracker.resolve_conflicts(&file).await.unwrap();
        assert_eq!(merged.content, "Second edit");
        assert!(merged.had_conflicts);
        assert_eq!(merged.contributors.len(), 2);
    }

    #[tokio::test]
    async fn test_last_write_wins() {
        let tracker = FileEditTracker::new(MergeStrategy::LastWriteWins);
        let file = PathBuf::from("test.txt");

        let token1 = tracker
            .request_edit(file.clone(), "agent1".to_string())
            .await;
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        let token2 = tracker
            .request_edit(file.clone(), "agent2".to_string())
            .await;

        tracker
            .commit_edit(token1, None, "Older edit".to_string())
            .await
            .unwrap();
        tracker
            .commit_edit(token2, None, "Newer edit".to_string())
            .await
            .unwrap();

        let merged = tracker.resolve_conflicts(&file).await.unwrap();
        assert_eq!(merged.content, "Newer edit");
        assert!(merged.had_conflicts);
    }
}
