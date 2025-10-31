#[cfg(test)]
mod lock_integration_tests {
    use codex_core::lock::RepoLock;
    use std::process::{Command, Stdio};
    use std::time::Duration;
    use tempfile::TempDir;

    #[test]
    fn test_lock_prevents_concurrent_access() {
        let temp_dir = TempDir::new().unwrap();
        
        // First process acquires lock
        let lock1 = RepoLock::try_acquire(temp_dir.path(), None).unwrap();
        
        // Second process should fail
        let result = RepoLock::try_acquire(temp_dir.path(), None);
        assert!(result.is_err(), "Second lock should fail");
        
        // Clean up
        drop(lock1);
    }

    #[test]
    fn test_stale_lock_removal() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create a lock with very short TTL
        let ttl = Duration::from_millis(100);
        let lock = RepoLock::try_acquire(temp_dir.path(), Some(ttl)).unwrap();
        drop(lock);
        
        // Wait for TTL to expire
        std::thread::sleep(Duration::from_millis(200));
        
        // Should be able to acquire lock again (stale lock removed)
        let lock2 = RepoLock::try_acquire(temp_dir.path(), None);
        assert!(lock2.is_ok(), "Should acquire lock after TTL expiry");
    }

    #[test]
    fn test_lock_force_unlock() {
        let temp_dir = TempDir::new().unwrap();
        
        // Acquire lock
        let _lock = RepoLock::try_acquire(temp_dir.path(), None).unwrap();
        
        // Force unlock
        RepoLock::force_unlock(temp_dir.path()).unwrap();
        
        // Should be able to acquire new lock
        let lock2 = RepoLock::try_acquire(temp_dir.path(), None);
        assert!(lock2.is_ok(), "Should acquire lock after force unlock");
    }

    #[tokio::test]
    async fn test_lock_wait_and_acquire() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();
        
        // Acquire lock in first "process"
        let lock1 = RepoLock::try_acquire(&path, None).unwrap();
        
        // Spawn task to release lock after delay
        let path_clone = path.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await;
            drop(lock1);
        });
        
        // Wait for lock to be released
        let lock2 = RepoLock::acquire_with_wait(
            &path,
            None,
            Duration::from_secs(5),
        ).await;
        
        assert!(lock2.is_ok(), "Should acquire lock after wait");
    }

    #[test]
    fn test_lock_status_check() {
        let temp_dir = TempDir::new().unwrap();
        
        // No lock initially
        let status = RepoLock::get_current(temp_dir.path()).unwrap();
        assert!(status.is_none(), "No lock should exist initially");
        
        // Acquire lock
        let _lock = RepoLock::try_acquire(temp_dir.path(), None).unwrap();
        
        // Check status
        let status = RepoLock::get_current(temp_dir.path()).unwrap();
        assert!(status.is_some(), "Lock should exist");
        
        let lock_info = status.unwrap();
        assert_eq!(lock_info.pid, std::process::id());
    }

    #[test]
    #[cfg(unix)]
    fn test_lock_with_process_check() {
        use std::fs;
        
        let temp_dir = TempDir::new().unwrap();
        
        // Create a lock file with a fake PID that doesn't exist
        let lock_dir = temp_dir.path().join(".codex");
        fs::create_dir_all(&lock_dir).unwrap();
        
        let fake_lock = serde_json::json!({
            "version": "0.52.0",
            "pid": 99999999,
            "hostname": "test-host",
            "repo_path": temp_dir.path().to_string_lossy(),
            "started_at": 1698765432u64,
            "expires_at": serde_json::Value::Null
        });
        
        fs::write(
            lock_dir.join("lock.json"),
            serde_json::to_string_pretty(&fake_lock).unwrap(),
        ).unwrap();
        
        // Should detect as stale and remove it
        let lock = RepoLock::try_acquire(temp_dir.path(), None);
        assert!(lock.is_ok(), "Should acquire lock after detecting stale lock");
    }
}
