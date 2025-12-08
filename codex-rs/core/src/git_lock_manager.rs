//! Git Lock Manager for Parallel Development Deadlock Prevention
//!
//! This module provides Git repository-level locking mechanisms to prevent
//! deadlocks and conflicts in parallel development scenarios.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

/// Entry in the lock table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockEntry {
    /// Lock type (file or branch)
    pub lock_type: LockType,
    /// Owner of the lock (user/process ID)
    pub owner: String,
    /// Timestamp when lock was acquired
    pub timestamp: DateTime<Utc>,
    /// Lock metadata
    pub metadata: BTreeMap<String, String>,
}

/// Type of lock
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockType {
    /// File-level lock
    File,
    /// Branch-level lock
    Branch,
    /// Repository-level lock
    Repository,
}

/// File lock information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLock {
    /// File path relative to repository root
    pub path: PathBuf,
    /// Lock owner
    pub owner: String,
    /// Lock timestamp
    pub timestamp: DateTime<Utc>,
    /// Lock reason/description
    pub reason: String,
}

/// Branch lock information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchLock {
    /// Branch name
    pub branch: String,
    /// Lock owner
    pub owner: String,
    /// Lock timestamp
    pub timestamp: DateTime<Utc>,
    /// Locked files (if partial branch lock)
    pub files: Vec<PathBuf>,
}

/// Git Lock Manager
pub struct GitLockManager {
    /// Repository root path
    repo_path: PathBuf,
    /// Lock table (key: lock identifier, value: lock entry)
    locks: Arc<Mutex<BTreeMap<String, LockEntry>>>,
    /// Lock file path
    lock_file: PathBuf,
}

impl GitLockManager {
    /// Create new GitLockManager for the given repository
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Result<Self> {
        let repo_path = repo_path.as_ref().to_path_buf();
        let lock_file = repo_path.join(".git").join("codex_locks.json");

        Ok(Self {
            repo_path,
            locks: Arc::new(Mutex::new(BTreeMap::new())),
            lock_file,
        })
    }

    /// Acquire a file lock
    pub async fn acquire_file_lock(
        &self,
        file_path: &Path,
        owner: &str,
        reason: &str,
    ) -> Result<FileLock> {
        let mut locks = self.locks.lock().await;

        let lock_key = format!("file:{}", file_path.display());
        if locks.contains_key(&lock_key) {
            return Err(anyhow::anyhow!("File {} is already locked", file_path.display()));
        }

        let file_lock = FileLock {
            path: file_path.to_path_buf(),
            owner: owner.to_string(),
            timestamp: Utc::now(),
            reason: reason.to_string(),
        };

        let lock_entry = LockEntry {
            lock_type: LockType::File,
            owner: owner.to_string(),
            timestamp: file_lock.timestamp,
            metadata: BTreeMap::from([
                ("path".to_string(), file_path.display().to_string()),
                ("reason".to_string(), reason.to_string()),
            ]),
        };

        locks.insert(lock_key, lock_entry);
        self.save_locks(&locks).await?;

        Ok(file_lock)
    }

    /// Release a file lock
    pub async fn release_file_lock(&self, file_path: &Path, owner: &str) -> Result<()> {
        let mut locks = self.locks.lock().await;

        let lock_key = format!("file:{}", file_path.display());
        if let Some(entry) = locks.get(&lock_key) {
            if entry.owner != owner {
                return Err(anyhow::anyhow!("Lock owned by different user: {}", entry.owner));
            }
            locks.remove(&lock_key);
            self.save_locks(&locks).await?;
        }

        Ok(())
    }

    /// Acquire a branch lock
    pub async fn acquire_branch_lock(
        &self,
        branch: &str,
        owner: &str,
        files: Option<Vec<PathBuf>>,
    ) -> Result<BranchLock> {
        let mut locks = self.locks.lock().await;

        let lock_key = format!("branch:{}", branch);
        if locks.contains_key(&lock_key) {
            return Err(anyhow::anyhow!("Branch {} is already locked", branch));
        }

        let branch_lock = BranchLock {
            branch: branch.to_string(),
            owner: owner.to_string(),
            timestamp: Utc::now(),
            files: files.unwrap_or_default(),
        };

        let lock_entry = LockEntry {
            lock_type: LockType::Branch,
            owner: owner.to_string(),
            timestamp: branch_lock.timestamp,
            metadata: BTreeMap::from([
                ("branch".to_string(), branch.to_string()),
                ("files_count".to_string(), branch_lock.files.len().to_string()),
            ]),
        };

        locks.insert(lock_key, lock_entry);
        self.save_locks(&locks).await?;

        Ok(branch_lock)
    }

    /// Release a branch lock
    pub async fn release_branch_lock(&self, branch: &str, owner: &str) -> Result<()> {
        let mut locks = self.locks.lock().await;

        let lock_key = format!("branch:{}", branch);
        if let Some(entry) = locks.get(&lock_key) {
            if entry.owner != owner {
                return Err(anyhow::anyhow!("Lock owned by different user: {}", entry.owner));
            }
            locks.remove(&lock_key);
            self.save_locks(&locks).await?;
        }

        Ok(())
    }

    /// Check if a file is locked
    pub async fn is_file_locked(&self, file_path: &Path) -> Result<Option<String>> {
        let locks = self.locks.lock().await;
        let lock_key = format!("file:{}", file_path.display());

        if let Some(entry) = locks.get(&lock_key) {
            Ok(Some(entry.owner.clone()))
        } else {
            Ok(None)
        }
    }

    /// Check if a branch is locked
    pub async fn is_branch_locked(&self, branch: &str) -> Result<Option<String>> {
        let locks = self.locks.lock().await;
        let lock_key = format!("branch:{}", branch);

        if let Some(entry) = locks.get(&lock_key) {
            Ok(Some(entry.owner.clone()))
        } else {
            Ok(None)
        }
    }

    /// List all active locks
    pub async fn list_locks(&self) -> Result<Vec<LockEntry>> {
        let locks = self.locks.lock().await;
        Ok(locks.values().cloned().collect())
    }

    /// Load locks from disk
    pub async fn load_locks(&self) -> Result<()> {
        if !self.lock_file.exists() {
            return Ok(());
        }

        let data = tokio::fs::read_to_string(&self.lock_file)
            .await
            .with_context(|| format!("Failed to read lock file: {}", self.lock_file.display()))?;

        let locks: BTreeMap<String, LockEntry> = serde_json::from_str(&data)
            .with_context(|| "Failed to parse lock file")?;

        let mut current_locks = self.locks.lock().await;
        *current_locks = locks;

        Ok(())
    }

    /// Save locks to disk
    async fn save_locks(&self, locks: &BTreeMap<String, LockEntry>) -> Result<()> {
        // Ensure .git directory exists
        if let Some(parent) = self.lock_file.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .with_context(|| format!("Failed to create .git directory: {}", parent.display()))?;
        }

        let data = serde_json::to_string_pretty(locks)
            .with_context(|| "Failed to serialize locks")?;

        tokio::fs::write(&self.lock_file, data)
            .await
            .with_context(|| format!("Failed to write lock file: {}", self.lock_file.display()))?;

        Ok(())
    }

    /// Clean up stale locks (older than specified duration)
    pub async fn cleanup_stale_locks(&self, max_age_seconds: i64) -> Result<usize> {
        let mut locks = self.locks.lock().await;
        let now = Utc::now();
        let mut removed_count = 0;

        locks.retain(|_, entry| {
            let age = now.signed_duration_since(entry.timestamp).num_seconds();
            if age > max_age_seconds {
                removed_count += 1;
                false
            } else {
                true
            }
        });

        if removed_count > 0 {
            self.save_locks(&locks).await?;
        }

        Ok(removed_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_lock_basic() {
        let temp_dir = TempDir::new().unwrap();
        let manager = GitLockManager::new(&temp_dir).unwrap();

        let file_path = Path::new("test.txt");

        // Acquire lock
        let lock = manager.acquire_file_lock(file_path, "user1", "testing").await.unwrap();
        assert_eq!(lock.owner, "user1");
        assert_eq!(lock.path, file_path);

        // Check if locked
        let owner = manager.is_file_locked(file_path).await.unwrap();
        assert_eq!(owner, Some("user1".to_string()));

        // Try to acquire same lock (should fail)
        let result = manager.acquire_file_lock(file_path, "user2", "testing").await;
        assert!(result.is_err());

        // Release lock
        manager.release_file_lock(file_path, "user1").await.unwrap();

        // Check if unlocked
        let owner = manager.is_file_locked(file_path).await.unwrap();
        assert_eq!(owner, None);
    }

    #[tokio::test]
    async fn test_branch_lock_basic() {
        let temp_dir = TempDir::new().unwrap();
        let manager = GitLockManager::new(&temp_dir).unwrap();

        let branch = "feature/test";

        // Acquire lock
        let lock = manager.acquire_branch_lock(branch, "user1", None).await.unwrap();
        assert_eq!(lock.owner, "user1");
        assert_eq!(lock.branch, branch);

        // Check if locked
        let owner = manager.is_branch_locked(branch).await.unwrap();
        assert_eq!(owner, Some("user1".to_string()));

        // Release lock
        manager.release_branch_lock(branch, "user1").await.unwrap();

        // Check if unlocked
        let owner = manager.is_branch_locked(branch).await.unwrap();
        assert_eq!(owner, None);
    }
}