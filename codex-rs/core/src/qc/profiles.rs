//! Test profile definitions and configuration

use serde::{Deserialize, Serialize};

/// Available test profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TestProfile {
    /// Minimal testing - fastest option
    Minimal,
    /// Standard testing - default option
    #[default]
    Standard,
    /// Full testing - comprehensive validation
    Full,
}

impl TestProfile {
    /// Get the commands to run for Rust tests
    pub fn rust_commands(&self) -> Vec<String> {
        match self {
            TestProfile::Minimal => {
                vec!["cargo test -p codex-cli".to_string()]
            }
            TestProfile::Standard => {
                vec![
                    "cargo test --all".to_string(),
                    "cargo clippy --all --all-targets -- -D warnings".to_string(),
                ]
            }
            TestProfile::Full => {
                let cmds = Self::Standard.rust_commands();
                // Tarpaulin is optional - will be marked as optional test
                cmds
            }
        }
    }

    /// Get the commands to run for Web/GUI tests
    /// These commands will only run if the working directory contains package.json
    pub fn web_commands(&self) -> Vec<String> {
        match self {
            TestProfile::Minimal => {
                vec![]
            }
            TestProfile::Standard => {
                // Try pnpm first, fall back to npm if pnpm doesn't exist
                vec!["(command -v pnpm > /dev/null 2>&1 && pnpm test) || npm test".to_string()]
            }
            TestProfile::Full => {
                vec![
                    "(command -v pnpm > /dev/null 2>&1 && pnpm test) || npm test".to_string(),
                    "(command -v pnpm > /dev/null 2>&1 && pnpm lint) || npm run lint".to_string(),
                ]
            }
        }
    }
    
    /// Get optional Rust commands (like coverage) that don't fail the QC if they fail
    pub fn optional_rust_commands(&self) -> Vec<String> {
        match self {
            TestProfile::Full => {
                vec!["cargo tarpaulin --workspace".to_string()]
            }
            _ => vec![],
        }
    }
}

impl std::fmt::Display for TestProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestProfile::Minimal => write!(f, "minimal"),
            TestProfile::Standard => write!(f, "standard"),
            TestProfile::Full => write!(f, "full"),
        }
    }
}

impl std::str::FromStr for TestProfile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "minimal" => Ok(TestProfile::Minimal),
            "standard" => Ok(TestProfile::Standard),
            "full" => Ok(TestProfile::Full),
            _ => Err(format!("Unknown test profile: {}", s)),
        }
    }
}

/// Configuration for test profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestProfileConfig {
    /// Default test profile to use
    #[serde(default)]
    pub default_profile: TestProfile,
}

impl Default for TestProfileConfig {
    fn default() -> Self {
        Self {
            default_profile: TestProfile::Standard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_from_str() {
        assert_eq!(
            "minimal".parse::<TestProfile>().unwrap(),
            TestProfile::Minimal
        );
        assert_eq!(
            "standard".parse::<TestProfile>().unwrap(),
            TestProfile::Standard
        );
        assert_eq!("full".parse::<TestProfile>().unwrap(), TestProfile::Full);
        assert!("invalid".parse::<TestProfile>().is_err());
    }

    #[test]
    fn test_minimal_profile_commands() {
        let profile = TestProfile::Minimal;
        let rust_cmds = profile.rust_commands();
        assert_eq!(rust_cmds.len(), 1);
        assert!(rust_cmds[0].contains("cargo test -p codex-cli"));

        let web_cmds = profile.web_commands();
        assert_eq!(web_cmds.len(), 0);
    }

    #[test]
    fn test_standard_profile_commands() {
        let profile = TestProfile::Standard;
        let rust_cmds = profile.rust_commands();
        assert_eq!(rust_cmds.len(), 2);
        assert!(rust_cmds[0].contains("cargo test --all"));
        assert!(rust_cmds[1].contains("cargo clippy"));

        let web_cmds = profile.web_commands();
        assert_eq!(web_cmds.len(), 1);
    }

    #[test]
    fn test_full_profile_commands() {
        let profile = TestProfile::Full;
        let rust_cmds = profile.rust_commands();
        // Full profile has same rust commands as standard, plus optional commands
        assert_eq!(rust_cmds.len(), 2);
        
        let optional_cmds = profile.optional_rust_commands();
        assert_eq!(optional_cmds.len(), 1);
        assert!(optional_cmds[0].contains("tarpaulin"));

        let web_cmds = profile.web_commands();
        assert_eq!(web_cmds.len(), 2);
    }
}
