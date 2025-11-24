//! Quarantine System for Malware Detection
//!
//! Provides safe isolation, deletion, and restoration of detected malware files.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Quarantine entry status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuarantineStatus {
    /// File is quarantined and isolated
    Quarantined,
    /// File has been permanently deleted
    Deleted,
    /// File has been restored to original location
    Restored,
}

/// Quarantine entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarantineEntry {
    /// Unique quarantine ID
    pub id: String,
    /// Original file path
    pub original_path: PathBuf,
    /// Quarantine location
    pub quarantine_path: PathBuf,
    /// Threat name/type
    pub threat_name: String,
    /// Detection confidence (0.0-1.0)
    pub confidence: f32,
    /// Quarantine timestamp
    pub quarantined_at: chrono::DateTime<chrono::Utc>,
    /// Current status
    pub status: QuarantineStatus,
    /// File hash (SHA256) for verification
    pub file_hash: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Quarantine Manager
pub struct Quarantine {
    /// Quarantine directory
    quarantine_dir: PathBuf,
    /// Active quarantine entries
    entries: Arc<RwLock<HashMap<String, QuarantineEntry>>>,
    /// Quarantine statistics
    stats: Arc<RwLock<QuarantineStats>>,
}

/// Quarantine statistics
#[derive(Debug, Default)]
struct QuarantineStats {
    total_quarantined: usize,
    total_deleted: usize,
    total_restored: usize,
    total_size_bytes: u64,
}

impl Quarantine {
    /// Create a new quarantine manager
    pub fn new(quarantine_dir: PathBuf) -> Self {
        Self {
            quarantine_dir,
            entries: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(QuarantineStats::default())),
        }
    }

    /// Initialize quarantine directory
    pub async fn initialize(&self) -> Result<()> {
        // Create quarantine directory if it doesn't exist
        if !self.quarantine_dir.exists() {
            fs::create_dir_all(&self.quarantine_dir)
                .await
                .context("Failed to create quarantine directory")?;
            info!("Created quarantine directory: {:?}", self.quarantine_dir);
        }

        // Load existing quarantine entries
        self.load_entries().await?;

        Ok(())
    }

    /// Quarantine a file (move to quarantine directory)
    pub async fn quarantine_file(
        &self,
        file_path: &Path,
        threat_name: &str,
        confidence: f32,
    ) -> Result<QuarantineEntry> {
        debug!("Quarantining file: {:?}", file_path);

        // Generate unique quarantine ID
        let id = Uuid::new_v4().to_string();

        // Create quarantine path
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        let quarantine_path = self
            .quarantine_dir
            .join(format!("{}_{}", id, file_name));

        // Calculate file hash before moving
        let file_hash = self.calculate_file_hash(file_path).await.ok();

        // Get file size
        let metadata = fs::metadata(file_path).await?;
        let file_size = metadata.len();

        // Move file to quarantine
        fs::rename(file_path, &quarantine_path)
            .await
            .context("Failed to move file to quarantine")?;

        // Create quarantine entry
        let entry = QuarantineEntry {
            id: id.clone(),
            original_path: file_path.to_path_buf(),
            quarantine_path: quarantine_path.clone(),
            threat_name: threat_name.to_string(),
            confidence,
            quarantined_at: chrono::Utc::now(),
            status: QuarantineStatus::Quarantined,
            file_hash,
            metadata: HashMap::new(),
        };

        // Store entry
        {
            let mut entries = self.entries.write().await;
            entries.insert(id.clone(), entry.clone());
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_quarantined += 1;
            stats.total_size_bytes += file_size;
        }

        // Persist entries
        self.save_entries().await?;

        info!(
            "File quarantined: {:?} -> {:?} (threat: {})",
            file_path, quarantine_path, threat_name
        );

        Ok(entry)
    }

    /// Permanently delete a quarantined file
    pub async fn delete_file(&self, entry_id: &str) -> Result<()> {
        debug!("Deleting quarantined file: {}", entry_id);

        let mut entries = self.entries.write().await;
        let entry = entries
            .get_mut(entry_id)
            .context("Quarantine entry not found")?;

        if entry.status != QuarantineStatus::Quarantined {
            return Err(anyhow::anyhow!("File is not in quarantined status"));
        }

        // Delete file from quarantine
        if entry.quarantine_path.exists() {
            fs::remove_file(&entry.quarantine_path)
                .await
                .context("Failed to delete quarantined file")?;
        }

        // Update entry status
        entry.status = QuarantineStatus::Deleted;

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_deleted += 1;
        }

        // Persist entries
        drop(entries);
        self.save_entries().await?;

        info!("Quarantined file deleted: {}", entry_id);

        Ok(())
    }

    /// Restore a quarantined file to its original location
    pub async fn restore_file(&self, entry_id: &str) -> Result<()> {
        debug!("Restoring quarantined file: {}", entry_id);

        let mut entries = self.entries.write().await;
        let entry = entries
            .get_mut(entry_id)
            .context("Quarantine entry not found")?;

        if entry.status != QuarantineStatus::Quarantined {
            return Err(anyhow::anyhow!("File is not in quarantined status"));
        }

        // Check if original location exists (warn if it does)
        if entry.original_path.exists() {
            warn!(
                "Original file already exists: {:?}. Restore will overwrite.",
                entry.original_path
            );
        }

        // Restore file to original location
        fs::rename(&entry.quarantine_path, &entry.original_path)
            .await
            .context("Failed to restore file")?;

        // Update entry status
        entry.status = QuarantineStatus::Restored;

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_restored += 1;
        }

        // Remove entry from active quarantine
        entries.remove(entry_id);

        // Persist entries
        drop(entries);
        self.save_entries().await?;

        info!("Quarantined file restored: {:?}", entry.original_path);

        Ok(())
    }

    /// Get all quarantine entries
    pub async fn list_entries(&self) -> Vec<QuarantineEntry> {
        let entries = self.entries.read().await;
        entries.values().cloned().collect()
    }

    /// Get a specific quarantine entry
    pub async fn get_entry(&self, entry_id: &str) -> Option<QuarantineEntry> {
        let entries = self.entries.read().await;
        entries.get(entry_id).cloned()
    }

    /// Get quarantine statistics
    pub async fn get_stats(&self) -> QuarantineStats {
        self.stats.read().await.clone()
    }

    /// Calculate SHA256 hash of a file
    async fn calculate_file_hash(&self, file_path: &Path) -> Result<String> {
        use sha2::{Digest, Sha256};

        let content = fs::read(file_path).await?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
    }

    /// Load quarantine entries from disk
    async fn load_entries(&self) -> Result<()> {
        let entries_file = self.quarantine_dir.join("entries.json");

        if !entries_file.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&entries_file).await?;
        let entries: HashMap<String, QuarantineEntry> = serde_json::from_str(&content)?;

        let mut active_entries = self.entries.write().await;
        *active_entries = entries;

        // Update statistics from loaded entries
        let mut stats = self.stats.write().await;
        stats.total_quarantined = active_entries
            .values()
            .filter(|e| e.status == QuarantineStatus::Quarantined)
            .count();
        stats.total_deleted = active_entries
            .values()
            .filter(|e| e.status == QuarantineStatus::Deleted)
            .count();
        stats.total_restored = active_entries
            .values()
            .filter(|e| e.status == QuarantineStatus::Restored)
            .count();

        Ok(())
    }

    /// Save quarantine entries to disk
    async fn save_entries(&self) -> Result<()> {
        let entries_file = self.quarantine_dir.join("entries.json");

        let entries = self.entries.read().await;
        let content = serde_json::to_string_pretty(&*entries)?;

        fs::write(&entries_file, content)
            .await
            .context("Failed to save quarantine entries")?;

        Ok(())
    }
}

impl Clone for QuarantineStats {
    fn clone(&self) -> Self {
        Self {
            total_quarantined: self.total_quarantined,
            total_deleted: self.total_deleted,
            total_restored: self.total_restored,
            total_size_bytes: self.total_size_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_quarantine_file() {
        let temp_dir = TempDir::new().unwrap();
        let quarantine_dir = temp_dir.path().join("quarantine");
        let test_file = temp_dir.path().join("malware.exe");

        // Create test file
        fs::write(&test_file, b"malicious content").unwrap();

        let quarantine = Quarantine::new(quarantine_dir.clone());
        quarantine.initialize().await.unwrap();

        // Quarantine file
        let entry = quarantine
            .quarantine_file(&test_file, "TestMalware", 0.9)
            .await
            .unwrap();

        assert_eq!(entry.status, QuarantineStatus::Quarantined);
        assert_eq!(entry.threat_name, "TestMalware");
        assert!(!test_file.exists()); // Original file should be moved
        assert!(entry.quarantine_path.exists()); // Quarantined file should exist

        // Verify entry can be retrieved
        let retrieved = quarantine.get_entry(&entry.id).await.unwrap();
        assert_eq!(retrieved.id, entry.id);
    }

    #[tokio::test]
    async fn test_delete_file() {
        let temp_dir = TempDir::new().unwrap();
        let quarantine_dir = temp_dir.path().join("quarantine");
        let test_file = temp_dir.path().join("malware.exe");

        fs::write(&test_file, b"malicious content").unwrap();

        let quarantine = Quarantine::new(quarantine_dir.clone());
        quarantine.initialize().await.unwrap();

        let entry = quarantine
            .quarantine_file(&test_file, "TestMalware", 0.9)
            .await
            .unwrap();

        // Delete quarantined file
        quarantine.delete_file(&entry.id).await.unwrap();

        let deleted_entry = quarantine.get_entry(&entry.id).await.unwrap();
        assert_eq!(deleted_entry.status, QuarantineStatus::Deleted);
        assert!(!deleted_entry.quarantine_path.exists()); // File should be deleted
    }

    #[tokio::test]
    async fn test_restore_file() {
        let temp_dir = TempDir::new().unwrap();
        let quarantine_dir = temp_dir.path().join("quarantine");
        let test_file = temp_dir.path().join("malware.exe");

        fs::write(&test_file, b"malicious content").unwrap();

        let quarantine = Quarantine::new(quarantine_dir.clone());
        quarantine.initialize().await.unwrap();

        let entry = quarantine
            .quarantine_file(&test_file, "TestMalware", 0.9)
            .await
            .unwrap();

        // Restore file
        quarantine.restore_file(&entry.id).await.unwrap();

        assert!(test_file.exists()); // Original file should be restored
        assert!(!entry.quarantine_path.exists()); // Quarantined file should be gone
    }
}

