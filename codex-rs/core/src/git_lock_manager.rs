//! Git Lock Manager for Parallel Development
//!
//! This module provides Git repository locking mechanisms to prevent deadlocks
//! and conflicts in parallel development scenarios.

use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use git2::{Repository, RepositoryState};

/// Entry for a lock in the lock manager
#[derive(Debug, Clone)]
pub struct LockEntry {
    pub lock_type: LockType,
    pub owner: String,
    pub timestamp: DateTime<Utc>,
    pub files: HashSet<PathBuf>,
    pub branch: Option<String>,
}

/// Type of lock
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockType {
    /// Lock specific files
    File,
    /// Lock entire branch
    Branch,
    /// Lock repository-wide operations
    Repository,
}

/// File-level lock for individual files
#[derive(Debug, Clone)]
pub struct FileLock {
    pub path: PathBuf,
    pub owner: String,
    pub timestamp: DateTime<Utc>,
    pub operation: FileOperation,
}

/// Type of file operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileOperation {
    Read,
    Write,
    Delete,
}

/// Branch-level lock for entire branches
#[derive(Debug, Clone)]
pub struct BranchLock {
    pub branch: String,
    pub owner: String,
    pub timestamp: DateTime<Utc>,
    pub operations: Vec<BranchOperation>,
}

/// Type of branch operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchOperation {
    Checkout,
    Merge,
    Rebase,
    Push,
    Pull,
}

/// Git Lock Manager for repository coordination
#[derive(Debug)]
pub struct GitLockManager {
    repo_path: PathBuf,
    locks: Arc<Mutex<BTreeMap<String, LockEntry>>>,
    max_concurrent_operations: usize,
    active_operations: Arc<Mutex<usize>>,
}

impl GitLockManager {
    /// Create new GitLockManager for repository at given path
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Result<Self> {
        let repo_path = repo_path.as_ref().to_path_buf();

        // Verify it's a valid git repository
        Repository::open(&repo_path)?;

        Ok(Self {
            repo_path,
            locks: Arc::new(Mutex::new(BTreeMap::new())),
            max_concurrent_operations: 10, // Default limit
            active_operations: Arc::new(Mutex::new(0)),
        })
    }

    /// Create new GitLockManager with custom concurrent operation limit
    pub fn with_concurrency_limit<P: AsRef<Path>>(repo_path: P, max_concurrent: usize) -> Result<Self> {
        let mut manager = Self::new(repo_path)?;
        manager.max_concurrent_operations = max_concurrent;
        Ok(manager)
    }

    /// Acquire file lock for reading
    pub async fn acquire_file_read_lock<P: AsRef<Path>>(&self, file_path: P, owner: &str) -> Result<FileLock> {
        self.acquire_file_lock(file_path, owner, FileOperation::Read).await
    }

    /// Acquire file lock for writing
    pub async fn acquire_file_write_lock<P: AsRef<Path>>(&self, file_path: P, owner: &str) -> Result<FileLock> {
        self.acquire_file_lock(file_path, owner, FileOperation::Write).await
    }

    /// Acquire file lock for deletion
    pub async fn acquire_file_delete_lock<P: AsRef<Path>>(&self, file_path: P, owner: &str) -> Result<FileLock> {
        self.acquire_file_lock(file_path, owner, FileOperation::Delete).await
    }

    /// Internal method to acquire file lock
    async fn acquire_file_lock<P: AsRef<Path>>(&self, file_path: P, owner: &str, operation: FileOperation) -> Result<FileLock> {
        let file_path = file_path.as_ref().to_path_buf();
        let lock_key = format!("file:{}", file_path.display());

        // Check for concurrent operation limit
        self.check_concurrent_limit().await?;

        let mut locks = self.locks.lock().await;

        // Check if file is already locked by another owner
        if let Some(existing) = locks.get(&lock_key) {
            if existing.owner != owner {
                // Check if the lock conflicts with current operation
                if self.operations_conflict(&operation, &existing.lock_type) {
                    return Err(anyhow!("File {} is locked by {} for conflicting operation",
                        file_path.display(), existing.owner));
                }
            }
        }

        // Create lock entry
        let lock_entry = LockEntry {
            lock_type: LockType::File,
            owner: owner.to_string(),
            timestamp: Utc::now(),
            files: HashSet::from([file_path.clone()]),
            branch: None,
        };

        locks.insert(lock_key, lock_entry);

        // Increment active operations
        *self.active_operations.lock().await += 1;

        Ok(FileLock {
            path: file_path,
            owner: owner.to_string(),
            timestamp: Utc::now(),
            operation,
        })
    }

    /// Acquire branch lock
    pub async fn acquire_branch_lock(&self, branch: &str, owner: &str, operations: Vec<BranchOperation>) -> Result<BranchLock> {
        let lock_key = format!("branch:{}", branch);

        // Check for concurrent operation limit
        self.check_concurrent_limit().await?;

        let mut locks = self.locks.lock().await;

        // Check if branch is already locked
        if let Some(existing) = locks.get(&lock_key) {
            if existing.owner != owner {
                return Err(anyhow!("Branch {} is locked by {}", branch, existing.owner));
            }
        }

        // Create lock entry
        let lock_entry = LockEntry {
            lock_type: LockType::Branch,
            owner: owner.to_string(),
            timestamp: Utc::now(),
            files: HashSet::new(),
            branch: Some(branch.to_string()),
        };

        locks.insert(lock_key, lock_entry);

        // Increment active operations
        *self.active_operations.lock().await += 1;

        Ok(BranchLock {
            branch: branch.to_string(),
            owner: owner.to_string(),
            timestamp: Utc::now(),
            operations,
        })
    }

    /// Release file lock
    pub async fn release_file_lock(&self, lock: &FileLock) -> Result<()> {
        let lock_key = format!("file:{}", lock.path.display());

        let mut locks = self.locks.lock().await;
        locks.remove(&lock_key);

        // Decrement active operations
        let mut active = self.active_operations.lock().await;
        if *active > 0 {
            *active -= 1;
        }

        Ok(())
    }

    /// Release branch lock
    pub async fn release_branch_lock(&self, lock: &BranchLock) -> Result<()> {
        let lock_key = format!("branch:{}", lock.branch);

        let mut locks = self.locks.lock().await;
        locks.remove(&lock_key);

        // Decrement active operations
        let mut active = self.active_operations.lock().await;
        if *active > 0 {
            *active -= 1;
        }

        Ok(())
    }

    /// Check if operations conflict
    fn operations_conflict(&self, new_op: &FileOperation, existing_type: &LockType) -> bool {
        match (new_op, existing_type) {
            (FileOperation::Write, LockType::File) => true,
            (FileOperation::Delete, LockType::File) => true,
            (FileOperation::Write, LockType::Branch) => true,
            (FileOperation::Delete, LockType::Branch) => true,
            _ => false,
        }
    }

    /// Check concurrent operation limit
    async fn check_concurrent_limit(&self) -> Result<()> {
        let active = *self.active_operations.lock().await;
        if active >= self.max_concurrent_operations {
            return Err(anyhow!("Maximum concurrent operations ({}) exceeded", self.max_concurrent_operations));
        }
        Ok(())
    }

    /// Get current active locks
    pub async fn get_active_locks(&self) -> Vec<LockEntry> {
        let locks = self.locks.lock().await;
        locks.values().cloned().collect()
    }

    /// Check if file is locked
    pub async fn is_file_locked<P: AsRef<Path>>(&self, file_path: P) -> bool {
        let lock_key = format!("file:{}", file_path.as_ref().display());
        let locks = self.locks.lock().await;
        locks.contains_key(&lock_key)
    }

    /// Check if branch is locked
    pub async fn is_branch_locked(&self, branch: &str) -> bool {
        let lock_key = format!("branch:{}", branch);
        let locks = self.locks.lock().await;
        locks.contains_key(&lock_key)
    }

    /// Get lock owner for file
    pub async fn get_file_lock_owner<P: AsRef<Path>>(&self, file_path: P) -> Option<String> {
        let lock_key = format!("file:{}", file_path.as_ref().display());
        let locks = self.locks.lock().await;
        locks.get(&lock_key).map(|entry| entry.owner.clone())
    }

    /// Get lock owner for branch
    pub async fn get_branch_lock_owner(&self, branch: &str) -> Option<String> {
        let lock_key = format!("branch:{}", branch);
        let locks = self.locks.lock().await;
        locks.get(&lock_key).map(|entry| entry.owner.clone())
    }

    /// Force release all locks (emergency use only)
    pub async fn force_release_all_locks(&self) -> usize {
        let mut locks = self.locks.lock().await;
        let count = locks.len();
        locks.clear();

        let mut active = self.active_operations.lock().await;
        *active = 0;

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_repo() -> Result<(TempDir, GitLockManager)> {
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path();

        // Initialize git repository
        Repository::init(repo_path)?;

        // Create a test file
        fs::write(repo_path.join("test.txt"), "test content")?;

        let manager = GitLockManager::new(repo_path)?;
        Ok((temp_dir, manager))
    }

    #[tokio::test]
    async fn test_file_lock_acquisition() {
        let (_temp, manager) = setup_test_repo().unwrap();

        // Acquire read lock
        let lock = manager.acquire_file_read_lock("test.txt", "user1").await.unwrap();
        assert_eq!(lock.owner, "user1");
        assert_eq!(lock.operation, FileOperation::Read);

        // Check if file is locked
        assert!(manager.is_file_locked("test.txt").await);
        assert_eq!(manager.get_file_lock_owner("test.txt").await, Some("user1".to_string()));

        // Release lock
        manager.release_file_lock(&lock).await.unwrap();
        assert!(!manager.is_file_locked("test.txt").await);
    }

    #[tokio::test]
    async fn test_concurrent_lock_conflict() {
        let (_temp, manager) = setup_test_repo().unwrap();

        // First user acquires write lock
        let _lock1 = manager.acquire_file_write_lock("test.txt", "user1").await.unwrap();

        // Second user tries to acquire conflicting lock - should fail
        let result = manager.acquire_file_write_lock("test.txt", "user2").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_branch_lock() {
        let (_temp, manager) = setup_test_repo().unwrap();

        // Acquire branch lock
        let lock = manager.acquire_branch_lock("main", "user1", vec![BranchOperation::Merge]).await.unwrap();
        assert_eq!(lock.owner, "user1");
        assert_eq!(lock.branch, "main");

        // Check if branch is locked
        assert!(manager.is_branch_locked("main").await);
        assert_eq!(manager.get_branch_lock_owner("main").await, Some("user1".to_string()));

        // Release lock
        manager.release_branch_lock(&lock).await.unwrap();
        assert!(!manager.is_branch_locked("main").await);
    }

    #[tokio::test]
    async fn test_concurrent_limit() {
        let (_temp, mut manager) = setup_test_repo().unwrap();

        // Set very low concurrent limit
        manager.max_concurrent_operations = 1;

        // Acquire first lock
        let _lock1 = manager.acquire_file_read_lock("test.txt", "user1").await.unwrap();

        // Second lock should fail due to limit
        let result = manager.acquire_file_read_lock("other.txt", "user2").await;
        assert!(result.is_err());
    }
}
