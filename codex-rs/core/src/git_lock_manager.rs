//! Git Lock Manager for parallel development deadlock prevention
//!
//! This module provides fine-grained locking mechanisms for Git repositories
//! to prevent conflicts and deadlocks in parallel development scenarios.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use git2::Repository;
use git2::RepositoryState;
use parking_lot::Mutex;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// Lock entry representing a held lock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockEntry {
    /// Unique lock ID
    pub id: String,
    /// Owner of the lock (user/process ID)
    pub owner: String,
    /// Lock type
    pub lock_type: LockType,
    /// Timestamp when lock was acquired
    pub acquired_at: DateTime<Utc>,
    /// Lock timeout duration
    pub timeout: Duration,
    /// Files/branches covered by this lock
    pub resources: Vec<String>,
}

/// Types of locks that can be acquired
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockType {
    /// Exclusive file lock (blocks all access to specific files)
    FileExclusive,
    /// Shared file lock (allows read access but blocks write)
    FileShared,
    /// Branch-level lock (blocks branch operations)
    BranchExclusive,
    /// Repository-level lock (blocks all operations)
    RepositoryExclusive,
}

/// Lock conflict detection result
#[derive(Debug, Clone)]
pub struct LockConflict {
    /// Conflicting lock
    pub conflicting_lock: LockEntry,
    /// Conflict reason
    pub reason: String,
    /// Suggested resolution
    pub resolution: ConflictResolution,
}

/// Suggested conflict resolution strategies
#[derive(Debug, Clone)]
pub enum ConflictResolution {
    /// Wait for lock to be released
    Wait,
    /// Force release conflicting lock (dangerous)
    ForceRelease,
    /// Retry operation later
    RetryLater,
    /// Use alternative approach
    AlternativePath,
}

/// Git Lock Manager - main coordination point for repository locking
pub struct GitLockManager {
    /// Path to the Git repository
    repo_path: PathBuf,
    /// Active locks storage
    locks: Arc<Mutex<BTreeMap<String, LockEntry>>>,
    /// Maximum concurrent operations semaphore
    concurrency_limit: Arc<Semaphore>,
    /// Lock timeout settings
    default_timeout: Duration,
    /// Conflict detector
    conflict_detector: Option<Arc<dyn ConflictDetectorTrait>>,
}

/// Trait for conflict detection algorithms
#[async_trait::async_trait]
pub trait ConflictDetectorTrait: Send + Sync {
    /// Detect potential conflicts between operations
    async fn detect_conflicts(
        &self,
        repo_path: &std::path::Path,
        operation: &GitOperation,
        existing_locks: &[LockEntry],
    ) -> Result<Vec<LockConflict>>;

    /// Calculate conflict probability score (0.0-1.0)
    async fn conflict_probability(
        &self,
        repo_path: &std::path::Path,
        op1: &GitOperation,
        op2: &GitOperation,
    ) -> Result<f64>;
}

/// Git operations that require locking
#[derive(Debug, Clone)]
pub enum GitOperation {
    /// File modification operations
    ModifyFiles(Vec<PathBuf>),
    /// Branch operations
    CreateBranch(String),
    /// Merge operations
    MergeBranches { source: String, target: String },
    /// Rebase operations
    RebaseBranch { branch: String, base: String },
    /// Repository-wide operations
    RepositoryMaintenance,
}

impl GitLockManager {
    /// Create new GitLockManager for repository
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Result<Self> {
        let repo_path = repo_path.as_ref().to_path_buf();

        // Validate repository exists and is valid
        let _repo = Repository::open(&repo_path)?;

        Ok(Self {
            repo_path,
            locks: Arc::new(Mutex::new(BTreeMap::new())),
            concurrency_limit: Arc::new(Semaphore::new(10)), // Allow 10 concurrent operations
            default_timeout: Duration::from_secs(300),       // 5 minutes
            conflict_detector: None,
        })
    }

    /// Set conflict detector
    pub fn with_conflict_detector(mut self, detector: Arc<dyn ConflictDetectorTrait>) -> Self {
        self.conflict_detector = Some(detector);
        self
    }

    /// Set concurrency limit
    pub fn with_concurrency_limit(mut self, limit: usize) -> Self {
        self.concurrency_limit = Arc::new(Semaphore::new(limit));
        self
    }

    /// Set default lock timeout
    pub fn with_default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Acquire lock for operation
    pub async fn acquire_lock(
        &self,
        operation: GitOperation,
        owner: String,
        timeout: Option<Duration>,
    ) -> Result<LockGuard> {
        let timeout = timeout.unwrap_or(self.default_timeout);

        // Check for conflicts before acquiring permit
        if let Some(detector) = &self.conflict_detector {
            let conflicts = detector
                .detect_conflicts(&self.repo_path, &operation, &[])
                .await?;
            if !conflicts.is_empty() {
                return Err(anyhow::anyhow!("Lock conflicts detected: {:?}", conflicts));
            }
        }

        // Acquire concurrency permit
        let permit =
            tokio::time::timeout(Duration::from_secs(30), self.concurrency_limit.acquire())
                .await??;

        // Generate lock ID
        let lock_id = format!("{}_{}", owner, chrono::Utc::now().timestamp_millis());

        // Create lock entry
        let lock_entry = LockEntry {
            id: lock_id.clone(),
            owner,
            lock_type: operation.lock_type(),
            acquired_at: Utc::now(),
            timeout,
            resources: operation.resources(),
        };

        // Store lock
        {
            let mut locks = self.locks.lock();
            locks.insert(lock_id.clone(), lock_entry);
        }

        Ok(LockGuard {
            manager: self,
            lock_id,
            _permit: permit,
        })
    }

    /// Check for conflicts with existing locks
    async fn check_conflicts(&self, operation: &GitOperation) -> Result<Vec<LockConflict>> {
        let locks = {
            let locks = self.locks.lock();
            locks.values().cloned().collect::<Vec<_>>()
        };

        if let Some(detector) = &self.conflict_detector {
            detector
                .detect_conflicts(&self.repo_path, operation, &locks)
                .await
        } else {
            // Simple conflict detection without advanced analysis
            let mut conflicts = Vec::new();
            for lock in locks {
                if operation.conflicts_with(&lock) {
                    conflicts.push(LockConflict {
                        conflicting_lock: lock,
                        reason: "Resource conflict".to_string(),
                        resolution: ConflictResolution::Wait,
                    });
                }
            }
            Ok(conflicts)
        }
    }

    /// Release lock (called by LockGuard drop)
    fn release_lock(&self, lock_id: &str) {
        let mut locks = self.locks.lock();
        locks.remove(lock_id);
    }

    /// Get current active locks
    pub fn active_locks(&self) -> Vec<LockEntry> {
        let locks = self.locks.lock();
        locks.values().cloned().collect()
    }

    /// Force release expired locks
    pub fn cleanup_expired_locks(&self) {
        let mut locks = self.locks.lock();
        let now = Utc::now();

        locks.retain(|_, lock| {
            let elapsed = now.signed_duration_since(lock.acquired_at);
            elapsed.num_seconds() < lock.timeout.as_secs() as i64
        });
    }
}

/// RAII guard for locks - automatically releases lock when dropped
pub struct LockGuard<'a> {
    manager: &'a GitLockManager,
    lock_id: String,
    _permit: tokio::sync::SemaphorePermit<'a>,
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        self.manager.release_lock(&self.lock_id);
    }
}

impl GitOperation {
    /// Get lock type required for this operation
    fn lock_type(&self) -> LockType {
        match self {
            GitOperation::ModifyFiles(_) => LockType::FileExclusive,
            GitOperation::CreateBranch(_) => LockType::BranchExclusive,
            GitOperation::MergeBranches { .. } => LockType::BranchExclusive,
            GitOperation::RebaseBranch { .. } => LockType::BranchExclusive,
            GitOperation::RepositoryMaintenance => LockType::RepositoryExclusive,
        }
    }

    /// Get resources affected by this operation
    fn resources(&self) -> Vec<String> {
        match self {
            GitOperation::ModifyFiles(files) => files
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
            GitOperation::CreateBranch(branch) => vec![format!("branch:{}", branch)],
            GitOperation::MergeBranches { source, target } => {
                vec![format!("branch:{}", source), format!("branch:{}", target)]
            }
            GitOperation::RebaseBranch { branch, base } => {
                vec![format!("branch:{}", branch), format!("branch:{}", base)]
            }
            GitOperation::RepositoryMaintenance => vec!["repository".to_string()],
        }
    }

    /// Check if this operation conflicts with an existing lock
    fn conflicts_with(&self, lock: &LockEntry) -> bool {
        let self_resources = self.resources();

        // Check resource overlap
        for resource in &self_resources {
            if lock.resources.contains(resource) {
                match (&self.lock_type(), &lock.lock_type) {
                    // Two exclusive locks always conflict
                    (LockType::FileExclusive, LockType::FileExclusive) => return true,
                    (LockType::BranchExclusive, LockType::BranchExclusive) => return true,
                    (LockType::RepositoryExclusive, _) => return true,
                    (_, LockType::RepositoryExclusive) => return true,

                    // Exclusive vs Shared conflicts
                    (LockType::FileExclusive, LockType::FileShared) => return true,
                    (LockType::FileShared, LockType::FileExclusive) => return true,

                    // Shared vs Shared is OK for read operations
                    (LockType::FileShared, LockType::FileShared) => continue,
                    _ => continue,
                }
            }
        }

        false
    }
}

/// Default conflict detector implementation
pub struct BasicConflictDetector;

#[async_trait::async_trait]
impl ConflictDetectorTrait for BasicConflictDetector {
    async fn detect_conflicts(
        &self,
        _repo_path: &std::path::Path,
        operation: &GitOperation,
        existing_locks: &[LockEntry],
    ) -> Result<Vec<LockConflict>> {
        let mut conflicts = Vec::new();

        for lock in existing_locks {
            if operation.conflicts_with(lock) {
                conflicts.push(LockConflict {
                    conflicting_lock: lock.clone(),
                    reason: format!("Resource conflict on {:?}", lock.resources),
                    resolution: ConflictResolution::Wait,
                });
            }
        }

        Ok(conflicts)
    }

    async fn conflict_probability(
        &self,
        _repo_path: &std::path::Path,
        op1: &GitOperation,
        op2: &GitOperation,
    ) -> Result<f64> {
        if op1.conflicts_with(&LockEntry {
            id: "".to_string(),
            owner: "".to_string(),
            lock_type: op2.lock_type(),
            acquired_at: Utc::now(),
            timeout: Duration::from_secs(0),
            resources: op2.resources(),
        }) {
            Ok(1.0) // Certain conflict
        } else {
            Ok(0.0) // No conflict
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_lock_acquisition() {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(&temp_dir).unwrap();

        // Create bare repo for testing
        drop(repo);

        let manager = GitLockManager::new(&temp_dir).unwrap();

        let operation = GitOperation::ModifyFiles(vec!["test.txt".into()]);
        let guard = manager
            .acquire_lock(operation, "test_user".to_string(), None)
            .await
            .unwrap();

        // Lock should be active
        assert_eq!(manager.active_locks().len(), 1);

        // Lock should be released when guard is dropped
        drop(guard);
        assert_eq!(manager.active_locks().len(), 0);
    }

    #[test]
    fn test_operation_conflicts() {
        let op1 = GitOperation::ModifyFiles(vec!["file1.txt".into()]);
        let op2 = GitOperation::ModifyFiles(vec!["file1.txt".into()]);

        // Same file operations should conflict
        assert!(op1.conflicts_with(&LockEntry {
            id: "test".to_string(),
            owner: "user".to_string(),
            lock_type: LockType::FileExclusive,
            acquired_at: Utc::now(),
            timeout: Duration::from_secs(60),
            resources: vec!["file1.txt".to_string()],
        }));
    }
}
