//! Security profiles for sandboxing and execution policy.
//!
//! This module defines security profiles that combine sandbox policies
//! with network access restrictions to provide granular security control.

use crate::protocol::SandboxPolicy;
use serde::Deserialize;
use serde::Serialize;

/// Comprehensive security profile that defines execution boundaries.
///
/// These profiles provide a higher-level abstraction over `SandboxPolicy`
/// with clearer intent and more granular control over capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SecurityProfile {
    /// Maximum security: No network, read-only filesystem.
    ///
    /// Use case: Initial exploration, untrusted workspaces.
    /// - Network: DENIED
    /// - Disk Read: ALLOWED (full filesystem)
    /// - Disk Write: DENIED
    /// - Process Spawn: RESTRICTED (via execpolicy)
    Offline,

    /// Read-only with network access for research/documentation.
    ///
    /// Use case: Research tasks, documentation generation.
    /// - Network: ALLOWED (read-only APIs)
    /// - Disk Read: ALLOWED (full filesystem)
    /// - Disk Write: DENIED
    /// - Process Spawn: RESTRICTED (via execpolicy)
    NetReadOnly,

    /// Standard development mode: workspace write access.
    ///
    /// Use case: Normal development workflow.
    /// - Network: DENIED
    /// - Disk Read: ALLOWED (full filesystem)
    /// - Disk Write: ALLOWED (workspace + tmp only)
    /// - Process Spawn: RESTRICTED (via execpolicy)
    WorkspaceWrite,

    /// Workspace write with network (most common).
    ///
    /// Use case: Development with package managers, API calls.
    /// - Network: ALLOWED
    /// - Disk Read: ALLOWED (full filesystem)
    /// - Disk Write: ALLOWED (workspace + tmp only)
    /// - Process Spawn: RESTRICTED (via execpolicy)
    WorkspaceNet,

    /// Full access mode - requires explicit opt-in.
    ///
    /// Use case: System administration, deployment scripts.
    /// - Network: ALLOWED
    /// - Disk Read: ALLOWED (full filesystem)
    /// - Disk Write: ALLOWED (full filesystem, use with caution)
    /// - Process Spawn: UNRESTRICTED
    ///
    /// ⚠️ **WARNING**: This profile bypasses most security restrictions.
    /// Only use when absolutely necessary and you trust the workspace.
    Trusted,
}

impl SecurityProfile {
    /// Returns the default security profile (safest option).
    pub fn default_profile() -> Self {
        Self::Offline
    }

    /// Converts the security profile to the corresponding `SandboxPolicy`.
    pub fn to_sandbox_policy(&self) -> SandboxPolicy {
        match self {
            Self::Offline => SandboxPolicy::ReadOnly,
            Self::NetReadOnly => SandboxPolicy::ReadOnly,
            Self::WorkspaceWrite => SandboxPolicy::WorkspaceWrite {
                writable_roots: vec![],
                network_access: false,
                exclude_tmpdir_env_var: false,
                exclude_slash_tmp: false,
            },
            Self::WorkspaceNet => SandboxPolicy::WorkspaceWrite {
                writable_roots: vec![],
                network_access: true,
                exclude_tmpdir_env_var: false,
                exclude_slash_tmp: false,
            },
            Self::Trusted => SandboxPolicy::DangerFullAccess,
        }
    }

    /// Returns whether network access is allowed.
    pub fn allows_network(&self) -> bool {
        matches!(self, Self::NetReadOnly | Self::WorkspaceNet | Self::Trusted)
    }

    /// Returns whether filesystem writes are allowed.
    pub fn allows_write(&self) -> bool {
        matches!(
            self,
            Self::WorkspaceWrite | Self::WorkspaceNet | Self::Trusted
        )
    }

    /// Returns whether full filesystem write is allowed.
    pub fn allows_full_write(&self) -> bool {
        matches!(self, Self::Trusted)
    }

    /// Returns a human-readable description of the profile.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Offline => "Maximum security: read-only, no network",
            Self::NetReadOnly => "Read-only with network for research",
            Self::WorkspaceWrite => "Standard: workspace write, no network",
            Self::WorkspaceNet => "Development: workspace write with network",
            Self::Trusted => "⚠️ Full access (use with caution)",
        }
    }

    /// Returns a short label for UI display.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Offline => "Offline",
            Self::NetReadOnly => "Read+Net",
            Self::WorkspaceWrite => "Workspace",
            Self::WorkspaceNet => "Workspace+Net",
            Self::Trusted => "Trusted",
        }
    }

    /// Returns the list of all available profiles.
    pub fn all_profiles() -> &'static [SecurityProfile] {
        &[
            Self::Offline,
            Self::NetReadOnly,
            Self::WorkspaceWrite,
            Self::WorkspaceNet,
            Self::Trusted,
        ]
    }

    /// Parses a profile from a string name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "offline" => Some(Self::Offline),
            "net-read-only" | "netreadonly" | "read-net" => Some(Self::NetReadOnly),
            "workspace-write" | "workspacewrite" | "workspace" => Some(Self::WorkspaceWrite),
            "workspace-net" | "workspacenet" => Some(Self::WorkspaceNet),
            "trusted" | "full-access" => Some(Self::Trusted),
            _ => None,
        }
    }
}

impl Default for SecurityProfile {
    fn default() -> Self {
        Self::default_profile()
    }
}

impl std::fmt::Display for SecurityProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_safest() {
        let profile = SecurityProfile::default();
        assert_eq!(profile, SecurityProfile::Offline);
        assert!(!profile.allows_network());
        assert!(!profile.allows_write());
    }

    #[test]
    fn test_offline_profile() {
        let profile = SecurityProfile::Offline;
        assert!(!profile.allows_network());
        assert!(!profile.allows_write());
        assert!(!profile.allows_full_write());
    }

    #[test]
    fn test_net_read_only_profile() {
        let profile = SecurityProfile::NetReadOnly;
        assert!(profile.allows_network());
        assert!(!profile.allows_write());
        assert!(!profile.allows_full_write());
    }

    #[test]
    fn test_workspace_write_profile() {
        let profile = SecurityProfile::WorkspaceWrite;
        assert!(!profile.allows_network());
        assert!(profile.allows_write());
        assert!(!profile.allows_full_write());
    }

    #[test]
    fn test_workspace_net_profile() {
        let profile = SecurityProfile::WorkspaceNet;
        assert!(profile.allows_network());
        assert!(profile.allows_write());
        assert!(!profile.allows_full_write());
    }

    #[test]
    fn test_trusted_profile() {
        let profile = SecurityProfile::Trusted;
        assert!(profile.allows_network());
        assert!(profile.allows_write());
        assert!(profile.allows_full_write());
    }

    #[test]
    fn test_to_sandbox_policy() {
        assert_eq!(
            SecurityProfile::Offline.to_sandbox_policy(),
            SandboxPolicy::ReadOnly
        );

        assert_eq!(
            SecurityProfile::Trusted.to_sandbox_policy(),
            SandboxPolicy::DangerFullAccess
        );
    }

    #[test]
    fn test_from_name() {
        assert_eq!(
            SecurityProfile::from_name("offline"),
            Some(SecurityProfile::Offline)
        );
        assert_eq!(
            SecurityProfile::from_name("workspace-net"),
            Some(SecurityProfile::WorkspaceNet)
        );
        assert_eq!(
            SecurityProfile::from_name("trusted"),
            Some(SecurityProfile::Trusted)
        );
        assert_eq!(SecurityProfile::from_name("invalid"), None);
    }

    #[test]
    fn test_all_profiles_count() {
        assert_eq!(SecurityProfile::all_profiles().len(), 5);
    }
}
