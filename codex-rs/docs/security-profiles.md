# Security Profiles

This document describes the security profiles available in Codex and their recommended usage.

## Overview

Codex uses **Security Profiles** to control what operations the AI agent can perform. Each profile defines a specific set of permissions for:
- **File System Access** (read/write)
- **Network Access**
- **Process Spawning**
- **Command Execution**

## Available Profiles

### 1. Offline (Default)

**Maximum security: Read-only filesystem, no network.**

```
- Network: ❌ DENIED
- Disk Read: ✅ ALLOWED (full filesystem)
- Disk Write: ❌ DENIED
- Process Spawn: ⚠️ RESTRICTED (via execpolicy)
```

**Use Cases:**
- Initial exploration of untrusted workspaces
- Security-sensitive environments
- Code review without modifications
- Reading documentation or logs

**Example:**
```bash
codex --profile offline
```

---

### 2. Read+Net

**Read-only filesystem with network access for research.**

```
- Network: ✅ ALLOWED (read-only APIs)
- Disk Read: ✅ ALLOWED (full filesystem)
- Disk Write: ❌ DENIED
- Process Spawn: ⚠️ RESTRICTED (via execpolicy)
```

**Use Cases:**
- Research and documentation tasks
- Fetching information from APIs
- Reading code + searching online references
- Learning about new libraries/frameworks

**Example:**
```bash
codex --profile net-read-only
```

---

### 3. Workspace (Standard Development)

**Workspace write access, no network.**

```
- Network: ❌ DENIED
- Disk Read: ✅ ALLOWED (full filesystem)
- Disk Write: ✅ ALLOWED (workspace + tmp only)
- Process Spawn: ⚠️ RESTRICTED (via execpolicy)
```

**Use Cases:**
- Normal development workflow
- Code editing and refactoring
- Running tests (without network)
- Local build tasks

**Example:**
```bash
codex --profile workspace
```

---

### 4. Workspace+Net (Most Common)

**Workspace write with network access.**

```
- Network: ✅ ALLOWED
- Disk Read: ✅ ALLOWED (full filesystem)
- Disk Write: ✅ ALLOWED (workspace + tmp only)
- Process Spawn: ⚠️ RESTRICTED (via execpolicy)
```

**Use Cases:**
- Development with package managers (npm, cargo, pip)
- API integration testing
- Deployment script generation
- Fetching dependencies

**Example:**
```bash
codex --profile workspace-net
```

---

### 5. Trusted (⚠️ Use with Caution)

**Full system access - requires explicit opt-in.**

```
- Network: ✅ ALLOWED
- Disk Read: ✅ ALLOWED (full filesystem)
- Disk Write: ✅ ALLOWED (full filesystem)
- Process Spawn: ✅ UNRESTRICTED
```

**⚠️ WARNING:** This profile bypasses most security restrictions.

**Use Cases:**
- System administration tasks
- Deployment to production servers
- System-wide configuration changes
- Only use when you fully trust the workspace

**Example:**
```bash
codex --profile trusted
```

---

## Security Mechanisms

### 1. Execution Policy (execpolicy)

All command executions are validated against the **execution policy** defined in `execpolicy/src/default.policy`.

**Policy Outcomes:**
- ✅ **Safe**: Command is verified as safe (e.g., `ls`, `cat`)
- ⚠️ **Match**: Command matched a rule, caller must verify paths
- ❌ **Forbidden**: Command is explicitly blocked (e.g., `rm -rf /`)
- ❓ **Unverified**: Safety unknown, requires user approval

### 2. Platform-Specific Sandboxing

| Platform | Mechanism | Status |
|----------|-----------|--------|
| **Linux** | Landlock + seccomp | ✅ Implemented |
| **macOS** | Seatbelt (sandbox-exec) | ✅ Implemented |
| **Windows** | AppContainer/Job Object | 🚧 In Progress |

### 3. Audit Logging

All security-critical operations are logged to an **audit log** with:
- Timestamp (UTC)
- Operation type (FileRead, FileWrite, CommandExec, etc.)
- Decision (Allowed/Denied/RequiresApproval)
- Sanitized target (privacy-aware: replaces usernames with `[USER]`)

**Example Audit Entry:**
```json
{
  "timestamp": "2025-10-08T03:59:33Z",
  "operation": "command_exec",
  "target": "curl https://example.com",
  "decision": "denied",
  "reason": "Network access forbidden in Offline profile",
  "session_id": "abc123"
}
```

---

## Choosing the Right Profile

| Task | Recommended Profile |
|------|-------------------|
| Explore new codebase | **Offline** |
| Research + documentation | **Read+Net** |
| Normal coding | **Workspace** |
| Full-stack development | **Workspace+Net** |
| System administration | **Trusted** (⚠️ caution) |

---

## Profile Configuration

### Via Command Line

```bash
# Set profile explicitly
codex --profile workspace-net

# Use environment variable
export CODEX_SECURITY_PROFILE=workspace
codex
```

### Via Configuration File

Edit `~/.codex/config.toml`:

```toml
[sandbox]
mode = "workspace-write"
network_access = true

# This maps to "Workspace+Net" profile
```

### Programmatic (Rust API)

```rust
use codex_core::SecurityProfile;

let profile = SecurityProfile::WorkspaceNet;

assert!(profile.allows_network());
assert!(profile.allows_write());
assert!(!profile.allows_full_write());

let sandbox_policy = profile.to_sandbox_policy();
```

---

## Default Behavior

If no profile is specified:
- **Default:** `Offline` (safest option)
- **Rationale:** Fail-safe design - start with maximum restrictions

---

## Security Best Practices

### ✅ DO:
- Start with `Offline` or `Workspace` for new projects
- Escalate privileges only when needed
- Review audit logs periodically
- Use `Trusted` profile only for known, trusted workspaces

### ❌ DON'T:
- Use `Trusted` profile by default
- Ignore sandbox violations in audit logs
- Disable security features without understanding implications
- Run untrusted code with `Trusted` profile

---

## Testing Security Profiles

### E2E Security Tests

The codebase includes **sandbox escape tests** that verify security boundaries:

```bash
# Run security tests
cargo test -p codex-execpolicy --test sandbox_escape_tests

# Expected output:
# ✅ test_sandbox_blocks_network_curl ... ok
# ✅ test_sandbox_blocks_unauthorized_write_etc ... ok
# ✅ test_sandbox_blocks_shell_escape_bash ... ok
```

### Manual Testing

```bash
# Test network blocking (Offline profile)
codex --profile offline
> codex: curl https://example.com
# Should be denied or require approval

# Test workspace write (Workspace profile)
codex --profile workspace
> codex: create a test file
# Should succeed within workspace

# Test system write blocking
> codex: write to /etc/passwd
# Should be denied
```

---

## Troubleshooting

### "Operation Denied" Errors

If you see unexpected denials:

1. **Check current profile:**
   ```bash
   codex --show-profile
   ```

2. **Review audit log:**
   ```bash
   cat ~/.codex/audit.log | jq '.[] | select(.decision == "denied")'
   ```

3. **Escalate if needed:**
   ```bash
   codex --profile workspace-net  # Add network access
   ```

### Profile Not Applied

- Ensure `config.toml` syntax is correct
- Environment variables override config file
- Command-line flags override both

---

## Implementation Details

### Profile → SandboxPolicy Mapping

```rust
SecurityProfile::Offline      → SandboxPolicy::ReadOnly
SecurityProfile::NetReadOnly  → SandboxPolicy::ReadOnly
SecurityProfile::Workspace    → SandboxPolicy::WorkspaceWrite { network: false }
SecurityProfile::WorkspaceNet → SandboxPolicy::WorkspaceWrite { network: true }
SecurityProfile::Trusted      → SandboxPolicy::DangerFullAccess
```

### File Structure

```
codex-rs/
├── core/src/security_profile.rs    # Profile definitions
├── execpolicy/                     # Command validation
│   ├── src/default.policy          # Starlark policy rules
│   └── tests/sandbox_escape_tests.rs
├── audit/                          # Audit logging
│   └── src/lib.rs
└── linux-sandbox/                  # Platform sandboxing
    └── src/landlock.rs
```

---

## Future Enhancements

### Planned Features:
- 🔜 **Per-directory profiles** (`.codex-profile` file)
- 🔜 **Temporary privilege elevation** (time-limited `Trusted` mode)
- 🔜 **Profile templates** for common workflows
- 🔜 **Network domain allowlists** (allow specific APIs only)

### Roadmap:
- **Week 1-2:** Complete Windows sandbox implementation
- **Week 3-4:** WASM plugin system for low-privilege extensions
- **Week 5-6:** Enhanced audit log analysis tools
- **Week 7-8:** CI/CD integration for security tests

---

## Related Documentation

- [Platform Sandboxing](../docs/platform-sandboxing.md)
- [Execution Policy](../codex-rs/execpolicy/README.md)
- [Configuration Guide](../docs/config.md)
- [Audit Logging](../codex-rs/audit/src/lib.rs)

---

## Contributing

If you find security vulnerabilities or have suggestions:

1. **Security issues:** Email security@example.com (do not open public issues)
2. **Feature requests:** Open a GitHub issue with `[security]` tag
3. **Pull requests:** Include E2E security tests

---

**Last Updated:** 2025-10-08  
**Version:** 0.0.0  
**Status:** ✅ Implemented and Tested

