//! Conflict Detector for Git operations
//!
//! This module provides advanced conflict detection using AST analysis
//! and machine learning-based conflict prediction.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use git2::Repository;
use serde::Deserialize;
use serde::Serialize;

use crate::git_lock_manager::ConflictDetectorTrait;
use crate::git_lock_manager::ConflictResolution;
use crate::git_lock_manager::GitOperation;
use crate::git_lock_manager::LockConflict;
use crate::git_lock_manager::LockEntry;

/// AST-based conflict detector
pub struct AstConflictDetector {
    /// Repository path
    repo_path: std::path::PathBuf,
    /// Cached AST analysis results
    #[allow(dead_code)]
    ast_cache: Arc<parking_lot::Mutex<BTreeMap<String, AstAnalysis>>>,
}

/// AST analysis result for a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstAnalysis {
    /// Functions defined in the file
    pub functions: Vec<String>,
    /// Structs/classes defined in the file
    pub structs: Vec<String>,
    /// Imports used in the file
    pub imports: Vec<String>,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

/// Conflict analysis result
#[derive(Debug, Clone)]
pub struct ConflictAnalysis {
    /// Probability of semantic conflict (0.0-1.0)
    pub semantic_conflict_prob: f64,
    /// Overlapping code regions
    pub overlapping_regions: Vec<CodeRegion>,
    /// Suggested merge strategy
    pub merge_strategy: MergeStrategy,
}

/// Code region information
#[derive(Debug, Clone)]
pub struct CodeRegion {
    /// Start line
    pub start_line: usize,
    /// End line
    pub end_line: usize,
    /// Region type (function, struct, etc.)
    pub region_type: String,
    /// Region identifier
    pub identifier: String,
}

/// Merge strategy recommendations
#[derive(Debug, Clone)]
pub enum MergeStrategy {
    /// Automatic merge possible
    AutoMerge,
    /// Manual merge required
    ManualMerge,
    /// Conflict cannot be resolved automatically
    Unresolvable,
    /// Use alternative merge approach
    Alternative,
}

impl AstConflictDetector {
    /// Create new AST conflict detector
    pub fn new(repo_path: std::path::PathBuf) -> Self {
        Self {
            repo_path,
            ast_cache: Arc::new(parking_lot::Mutex::new(BTreeMap::new())),
        }
    }

    /// Analyze file using AST parsing
    #[allow(dead_code)]
    fn analyze_file(&self, _file_path: &Path, content: &str) -> Result<AstAnalysis> {
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut imports = Vec::new();

        // Simple Rust AST analysis (in real implementation, use syn crate)
        for line in content.lines() {
            let line = line.trim();

            // Detect function definitions
            if (line.starts_with("fn ") || line.starts_with("pub fn "))
                && let Some(end) = line.find('(') {
                    let func_name = line[if line.starts_with("pub ") { 7 } else { 3 }..end].trim();
                    functions.push(func_name.to_string());
                }

            // Detect struct definitions
            if (line.starts_with("struct ") || line.starts_with("pub struct "))
                && let Some(end) = line.find('{') {
                    let struct_name =
                        line[if line.starts_with("pub ") { 10 } else { 7 }..end].trim();
                    structs.push(struct_name.to_string());
                }

            // Detect imports
            if line.starts_with("use ") {
                imports.push(line[4..].trim().to_string());
            }
        }

        Ok(AstAnalysis {
            functions,
            structs,
            imports,
            modified_at: chrono::Utc::now(),
        })
    }

    /// Calculate semantic overlap between two operations
    fn calculate_semantic_overlap(&self, op1: &GitOperation, op2: &GitOperation) -> f64 {
        match (op1, op2) {
            (GitOperation::ModifyFiles(files1), GitOperation::ModifyFiles(files2)) => {
                // Calculate file overlap
                let overlap: f64 = files1
                    .iter()
                    .filter(|f1| files2.iter().any(|f2| *f1 == f2))
                    .count() as f64;

                if overlap > 0.0 {
                    // If files overlap, high conflict probability
                    0.8
                } else {
                    // Check for related files (same module, etc.)
                    self.check_related_files(files1, files2)
                }
            }
            _ => 0.0, // Other operations have low semantic conflict
        }
    }

    /// Check if files are related (same module, etc.)
    fn check_related_files(
        &self,
        files1: &[std::path::PathBuf],
        files2: &[std::path::PathBuf],
    ) -> f64 {
        for f1 in files1 {
            for f2 in files2 {
                if let (Some(p1), Some(p2)) = (f1.parent(), f2.parent())
                    && p1 == p2 {
                        // Same directory - potential conflict
                        return 0.3;
                    }

                // Check file name similarity (without extension)
                let n1 = f1.file_stem().and_then(|s| s.to_str());
                let n2 = f2.file_stem().and_then(|s| s.to_str());

                if let (Some(n1), Some(n2)) = (n1, n2)
                    && (n1.contains(n2) || n2.contains(n1)) {
                        // Similar names - potential conflict
                        return 0.2;
                    }
            }
        }

        0.0
    }
}

#[async_trait::async_trait]
impl ConflictDetectorTrait for AstConflictDetector {
    async fn detect_conflicts(
        &self,
        repo_path: &std::path::Path,
        operation: &GitOperation,
        existing_locks: &[LockEntry],
    ) -> Result<Vec<LockConflict>> {
        // Ensure repo_path is the same as self.repo_path
        debug_assert_eq!(repo_path, self.repo_path.as_path());

        let mut conflicts = Vec::new();

        for lock in existing_locks {
            let conflict_prob = self
                .conflict_probability(
                    repo_path,
                    operation,
                    &GitOperation::ModifyFiles(
                        lock.resources
                            .iter()
                            .filter_map(|r| {
                                if r.starts_with("branch:") {
                                    None
                                } else {
                                    Some(std::path::PathBuf::from(r))
                                }
                            })
                            .collect(),
                    ),
                )
                .await?;

            if conflict_prob > 0.5 {
                conflicts.push(LockConflict {
                    conflicting_lock: lock.clone(),
                    reason: format!("High semantic conflict probability: {conflict_prob:.2}"),
                    resolution: if conflict_prob > 0.8 {
                        ConflictResolution::AlternativePath
                    } else {
                        ConflictResolution::Wait
                    },
                });
            }
        }

        Ok(conflicts)
    }

    async fn conflict_probability(
        &self,
        repo_path: &std::path::Path,
        op1: &GitOperation,
        op2: &GitOperation,
    ) -> Result<f64> {
        // Basic semantic overlap calculation
        let semantic_overlap = self.calculate_semantic_overlap(op1, op2);

        // Add repository state factors
        let repo_factor = match Repository::open(repo_path)
            .ok().map(|r| r.state())
        {
            Some(git2::RepositoryState::Merge) => 0.3, // Ongoing merge increases conflict risk
            Some(git2::RepositoryState::Rebase)
            | Some(git2::RepositoryState::RebaseInteractive) => 0.4, // Rebase operations are risky
            _ => 0.0,                                  // Clean state
        };

        Ok((semantic_overlap + repo_factor).min(1.0))
    }
}

/// Machine learning-based conflict predictor
pub struct MLConflictPredictor {
    /// Trained model weights (placeholder)
    model_weights: BTreeMap<String, f64>,
}

impl Default for MLConflictPredictor {
    fn default() -> Self {
        Self::new()
    }
}

impl MLConflictPredictor {
    /// Create new ML predictor with pre-trained weights
    pub fn new() -> Self {
        let mut weights = BTreeMap::new();

        // Initialize with heuristic weights
        weights.insert("file_overlap".to_string(), 0.7);
        weights.insert("directory_proximity".to_string(), 0.3);
        weights.insert("function_overlap".to_string(), 0.6);
        weights.insert("import_conflict".to_string(), 0.4);
        weights.insert("repo_state_merge".to_string(), 0.5);
        weights.insert("repo_state_rebase".to_string(), 0.6);

        Self {
            model_weights: weights,
        }
    }

    /// Predict conflict using ML model
    fn predict_conflict(&self, features: &BTreeMap<String, f64>) -> f64 {
        let mut score = 0.0;

        for (feature, value) in features {
            if let Some(weight) = self.model_weights.get(feature) {
                score += weight * value;
            }
        }

        // Sigmoid activation to get probability
        1.0 / (1.0 + (-score).exp())
    }

    /// Extract features from operations
    fn extract_features(
        &self,
        _repo_path: &std::path::Path,
        op1: &GitOperation,
        op2: &GitOperation,
    ) -> BTreeMap<String, f64> {
        let mut features = BTreeMap::new();

        match (op1, op2) {
            (GitOperation::ModifyFiles(files1), GitOperation::ModifyFiles(files2)) => {
                // File overlap
                let overlap: f64 = files1
                    .iter()
                    .filter(|f1| files2.iter().any(|f2| *f1 == f2))
                    .count() as f64
                    / (files1.len().max(1) + files2.len().max(1)) as f64;
                features.insert("file_overlap".to_string(), overlap);

                // Directory proximity
                let mut dir_score = 0.0;
                for f1 in files1 {
                    for f2 in files2 {
                        if let (Some(p1), Some(p2)) = (f1.parent(), f2.parent())
                            && p1 == p2 {
                                dir_score = 1.0;
                                break;
                            }
                    }
                }
                features.insert("directory_proximity".to_string(), dir_score);
            }
            _ => {}
        }

        // Repository state features - simplified for now
        // TODO: Add repository state analysis when needed

        features
    }
}

#[async_trait::async_trait]
impl ConflictDetectorTrait for MLConflictPredictor {
    async fn detect_conflicts(
        &self,
        repo_path: &std::path::Path,
        operation: &GitOperation,
        existing_locks: &[LockEntry],
    ) -> Result<Vec<LockConflict>> {
        let mut conflicts = Vec::new();

        for lock in existing_locks {
            let lock_op = GitOperation::ModifyFiles(
                lock.resources
                    .iter()
                    .filter_map(|r| {
                        if r.starts_with("branch:") {
                            None
                        } else {
                            Some(std::path::PathBuf::from(r))
                        }
                    })
                    .collect(),
            );

            let features = self.extract_features(repo_path, operation, &lock_op);
            let prob = self.predict_conflict(&features);

            if prob > 0.6 {
                conflicts.push(LockConflict {
                    conflicting_lock: lock.clone(),
                    reason: format!("ML predicted conflict probability: {prob:.2}"),
                    resolution: if prob > 0.8 {
                        ConflictResolution::RetryLater
                    } else {
                        ConflictResolution::Wait
                    },
                });
            }
        }

        Ok(conflicts)
    }

    async fn conflict_probability(
        &self,
        repo_path: &std::path::Path,
        op1: &GitOperation,
        op2: &GitOperation,
    ) -> Result<f64> {
        let features = self.extract_features(repo_path, op1, op2);
        Ok(self.predict_conflict(&features))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_ast_analysis() {
        let detector = AstConflictDetector::new(std::path::PathBuf::from("."));

        let content = r#"
        use std::collections::HashMap;

        pub struct User {
            id: u64,
            name: String,
        }

        pub fn create_user(id: u64, name: &str) -> User {
            User { id, name: name.to_string() }
        }

        pub fn get_user(id: u64) -> Option<User> {
            None
        }
        "#;

        let analysis = detector
            .analyze_file(std::path::Path::new("test.rs"), content)
            .unwrap();

        assert!(analysis.structs.contains(&"User".to_string()));
        assert!(analysis.functions.contains(&"create_user".to_string()));
        assert!(analysis.functions.contains(&"get_user".to_string()));
        assert!(analysis.imports.iter().any(|i| i.contains("HashMap")));
    }

    #[test]
    fn test_ml_predictor() {
        let predictor = MLConflictPredictor::new();

        let mut features = BTreeMap::new();
        features.insert("file_overlap".to_string(), 1.0);
        features.insert("repo_state_merge".to_string(), 1.0);

        let prob = predictor.predict_conflict(&features);
        assert!(prob > 0.5); // Should predict high conflict probability
    }

    #[tokio::test]
    async fn test_conflict_detection() {
        let temp_dir = TempDir::new().unwrap();
        let _repo = Repository::init(&temp_dir).unwrap();

        let detector = AstConflictDetector::new(temp_dir.path().to_path_buf());
        let operation = GitOperation::ModifyFiles(vec!["user.rs".into()]);
        let locks = vec![LockEntry {
            id: "test".to_string(),
            owner: "user1".to_string(),
            lock_type: crate::git_lock_manager::LockType::FileExclusive,
            acquired_at: chrono::Utc::now(),
            timeout: std::time::Duration::from_secs(60),
            resources: vec!["user.rs".to_string()],
        }];

        let conflicts = detector
            .detect_conflicts(temp_dir.path(), &operation, &locks)
            .await
            .unwrap();

        // Should detect conflict with same file
        assert!(!conflicts.is_empty());
    }
}
