use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

/// Lock information stored in lock.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockInfo {
    pub version: String,
    pub pid: u32,
    #[cfg(unix)]
    pub ppid: Option<u32>,
    #[cfg(unix)]
    pub uid: Option<u32>,
    pub hostname: String,
    pub repo_path: String,
    pub started_at: u64,
    pub expires_at: Option<u64>,
}

impl LockInfo {
    /// Create a new lock info for the current process
    pub fn new(repo_path: &Path, ttl: Option<Duration>) -> Result<Self> {
        let pid = std::process::id();
        let hostname = hostname::get()
            .context("Failed to get hostname")?
            .to_string_lossy()
            .to_string();
        
        let started_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        let expires_at = ttl.map(|ttl| started_at + ttl.as_secs());

        #[cfg(unix)]
        let (ppid, uid) = {
            use std::os::unix::process::parent_id;
            let ppid = Some(parent_id());
            let uid = nix::unistd::getuid().as_raw();
            (ppid, Some(uid))
        };

        Ok(Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            pid,
            #[cfg(unix)]
            ppid,
            #[cfg(unix)]
            uid,
            hostname,
            repo_path: repo_path.to_string_lossy().to_string(),
            started_at,
            expires_at,
        })
    }

    /// Check if this lock is stale
    pub fn is_stale(&self) -> bool {
        // Check if expired by TTL
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if now > expires_at {
                return true;
            }
        }

        // Check if process is still alive
        self.is_process_dead()
    }

    /// Check if the process is dead
    fn is_process_dead(&self) -> bool {
        #[cfg(unix)]
        {
            // On Unix, send signal 0 to check if process exists
            use nix::sys::signal::{kill, Signal};
            use nix::unistd::Pid;
            
            let pid = Pid::from_raw(self.pid as i32);
            match kill(pid, Signal::SIGCONT) {
                Ok(_) => false, // Process exists
                Err(_) => true, // Process doesn't exist
            }
        }

        #[cfg(windows)]
        {
            // On Windows, try to open the process
            use winapi::um::processthreadsapi::OpenProcess;
            use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
            use winapi::um::handleapi::CloseHandle;
            
            unsafe {
                let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, self.pid);
                if handle.is_null() {
                    true // Process doesn't exist
                } else {
                    CloseHandle(handle);
                    false // Process exists
                }
            }
        }

        #[cfg(not(any(unix, windows)))]
        {
            // For other platforms, assume not stale
            false
        }
    }
}

/// Repository lock manager
pub struct RepoLock {
    lock_path: PathBuf,
    #[allow(dead_code)]
    lock_info: LockInfo,
    acquired: bool,
}

impl RepoLock {
    /// Attempt to acquire a lock on the repository
    pub fn try_acquire(repo_path: &Path, ttl: Option<Duration>) -> Result<Self> {
        let lock_dir = repo_path.join(".codex");
        fs::create_dir_all(&lock_dir)?;
        
        let lock_path = lock_dir.join("lock.json");
        let lock_info = LockInfo::new(repo_path, ttl)?;

        // Check for existing lock
        if lock_path.exists() {
            let existing = Self::read_lock(&lock_path)?;
            if !existing.is_stale() {
                return Err(anyhow!(
                    "Repository is locked by process {} on {} (started at {}). \
                    Use 'codex unlock --force' to remove stale locks.",
                    existing.pid,
                    existing.hostname,
                    existing.started_at
                ));
            }
            // Remove stale lock
            fs::remove_file(&lock_path)?;
        }

        // Try to create lock file atomically
        #[cfg(unix)]
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(&lock_path)?;

        #[cfg(not(unix))]
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)?;

        // Write lock info
        let mut file = file;
        let lock_json = serde_json::to_string_pretty(&lock_info)?;
        file.write_all(lock_json.as_bytes())?;
        file.sync_all()?;

        Ok(Self {
            lock_path,
            lock_info,
            acquired: true,
        })
    }

    /// Wait and acquire lock with retry
    pub async fn acquire_with_wait(
        repo_path: &Path,
        ttl: Option<Duration>,
        max_wait: Duration,
    ) -> Result<Self> {
        let start = std::time::Instant::now();
        let mut retry_delay = Duration::from_millis(100);
        
        loop {
            match Self::try_acquire(repo_path, ttl) {
                Ok(lock) => return Ok(lock),
                Err(e) => {
                    if start.elapsed() > max_wait {
                        return Err(anyhow!("Timeout waiting for lock: {}", e));
                    }
                    tokio::time::sleep(retry_delay).await;
                    retry_delay = std::cmp::min(retry_delay * 2, Duration::from_secs(5));
                }
            }
        }
    }

    /// Read existing lock from file
    fn read_lock(path: &Path) -> Result<LockInfo> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lock_info: LockInfo = serde_json::from_str(&contents)?;
        Ok(lock_info)
    }

    /// Release the lock
    pub fn release(&mut self) -> Result<()> {
        if self.acquired && self.lock_path.exists() {
            fs::remove_file(&self.lock_path)?;
            self.acquired = false;
        }
        Ok(())
    }

    /// Force remove a lock file (for stale locks)
    pub fn force_unlock(repo_path: &Path) -> Result<()> {
        let lock_path = repo_path.join(".codex").join("lock.json");
        if lock_path.exists() {
            fs::remove_file(&lock_path)?;
            Ok(())
        } else {
            Err(anyhow!("No lock file found"))
        }
    }

    /// Get current lock info if it exists
    pub fn get_current(repo_path: &Path) -> Result<Option<LockInfo>> {
        let lock_path = repo_path.join(".codex").join("lock.json");
        if lock_path.exists() {
            Ok(Some(Self::read_lock(&lock_path)?))
        } else {
            Ok(None)
        }
    }
}

impl Drop for RepoLock {
    fn drop(&mut self) {
        let _ = self.release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_lock_info_creation() {
        let temp_dir = TempDir::new().unwrap();
        let lock_info = LockInfo::new(temp_dir.path(), None).unwrap();
        
        assert!(lock_info.pid > 0);
        assert!(!lock_info.hostname.is_empty());
        assert_eq!(lock_info.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_lock_acquire_release() {
        let temp_dir = TempDir::new().unwrap();
        let mut lock = RepoLock::try_acquire(temp_dir.path(), None).unwrap();
        
        let lock_path = temp_dir.path().join(".codex").join("lock.json");
        assert!(lock_path.exists());
        
        lock.release().unwrap();
        assert!(!lock_path.exists());
    }

    #[test]
    fn test_lock_conflict() {
        let temp_dir = TempDir::new().unwrap();
        let _lock1 = RepoLock::try_acquire(temp_dir.path(), None).unwrap();
        
        let result = RepoLock::try_acquire(temp_dir.path(), None);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_lock_wait() {
        let temp_dir = TempDir::new().unwrap();
        let lock1 = RepoLock::try_acquire(temp_dir.path(), None).unwrap();
        
        // Spawn task to release lock after 200ms
        let path = temp_dir.path().to_path_buf();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(200)).await;
            drop(lock1);
        });
        
        // Try to acquire with wait
        let lock2 = RepoLock::acquire_with_wait(
            temp_dir.path(),
            None,
            Duration::from_secs(5),
        ).await;
        
        assert!(lock2.is_ok());
    }
}
