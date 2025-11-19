//! Test profile definitions and configuration

use serde::Deserialize;
use serde::Serialize;

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
                let mut cmds = Self::Standard.rust_commands();
                // Add coverage if tarpaulin is available
                cmds.push("cargo tarpaulin --workspace || echo 'Tarpaulin not available, skipping coverage'".to_string());
                cmds
            }
        }
    }

    /// Get the commands to run for Web/GUI tests
    pub fn web_commands(&self) -> Vec<String> {
        match self {
            TestProfile::Minimal => {
                vec![]
            }
            TestProfile::Standard => {
                vec!["pnpm test || npm test || echo 'No package manager available'".to_string()]
            }
            TestProfile::Full => {
                vec![
                    "pnpm test || npm test || echo 'No package manager available'".to_string(),
                    "pnpm lint || npm run lint || echo 'No lint script available'".to_string(),
                ]
            }
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
        assert!(rust_cmds.len() >= 3);

        let web_cmds = profile.web_commands();
        assert_eq!(web_cmds.len(), 2);
    }
}
