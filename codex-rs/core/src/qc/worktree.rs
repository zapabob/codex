//! Git worktree detection and information

use std::path::PathBuf;
use std::process::Command;

/// Information about a Git worktree
#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    /// Path to the worktree
    pub path: PathBuf,
    /// Name of the worktree (derived from branch or path)
    pub name: String,
    /// Current branch name
    pub branch: String,
}

impl WorktreeInfo {
    /// Detect the current worktree from the current working directory
    pub fn detect() -> Result<Self, String> {
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;

        // Get the git directory
        let git_dir_output = Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .current_dir(&current_dir)
            .output()
            .map_err(|e| format!("Failed to execute git command: {}", e))?;

        if !git_dir_output.status.success() {
            return Err("Not a git repository".to_string());
        }

        // Get the current branch
        let branch_output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(&current_dir)
            .output()
            .map_err(|e| format!("Failed to get branch name: {}", e))?;

        let branch = String::from_utf8_lossy(&branch_output.stdout)
            .trim()
            .to_string();

        // Try to get worktree info
        let worktree_output = Command::new("git")
            .args(["worktree", "list", "--porcelain"])
            .current_dir(&current_dir)
            .output()
            .map_err(|e| format!("Failed to list worktrees: {}", e))?;

        let worktree_list = String::from_utf8_lossy(&worktree_output.stdout);

        // Parse worktree list to find the current one
        let name = Self::parse_worktree_name(&current_dir, &worktree_list)
            .unwrap_or_else(|| branch.clone());

        Ok(WorktreeInfo {
            path: current_dir,
            name,
            branch,
        })
    }

    /// Parse worktree name from git worktree list output
    fn parse_worktree_name(current_path: &PathBuf, worktree_list: &str) -> Option<String> {
        let mut current_worktree_path: Option<PathBuf> = None;
        let mut current_branch: Option<String> = None;

        for line in worktree_list.lines() {
            if line.starts_with("worktree ") {
                if let Some(path) = current_worktree_path.take()
                    && path == *current_path
                    && let Some(branch_name) = current_branch
                {
                    // Extract simple name from refs/heads/branch-name
                    if let Some(name) = branch_name.strip_prefix("refs/heads/") {
                        return Some(name.to_string());
                    }
                    return Some(branch_name);
                }

                let path_str = line.strip_prefix("worktree ")?;
                current_worktree_path = Some(PathBuf::from(path_str));
            } else if line.starts_with("branch ") {
                let branch_str = line.strip_prefix("branch ")?;
                current_branch = Some(branch_str.to_string());
            }
        }

        // Check the last worktree
        if let Some(path) = current_worktree_path
            && path == *current_path
            && let Some(branch_name) = current_branch
        {
            if let Some(name) = branch_name.strip_prefix("refs/heads/") {
                return Some(name.to_string());
            }
            return Some(branch_name);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_worktree_name() {
        let worktree_list = "worktree /home/user/project\nHEAD 1234567890abcdef\nbranch refs/heads/main\n\nworktree /home/user/project-feature\nHEAD abcdef1234567890\nbranch refs/heads/feature-branch\n";

        let path = PathBuf::from("/home/user/project-feature");
        let name = WorktreeInfo::parse_worktree_name(&path, worktree_list);

        assert_eq!(name, Some("feature-branch".to_string()));
    }
}
