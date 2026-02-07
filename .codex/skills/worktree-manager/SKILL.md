---
name: worktree-manager
description: Advanced Git worktree management agent that orchestrates parallel development environments with integrated quality assurance and automated workflow management. Enables concurrent feature development, isolated testing environments, and seamless integration with CI/CD pipelines.
---

# Worktree Manager Agent Skill

## Overview

Advanced Git worktree management agent that orchestrates parallel development environments with integrated quality assurance and automated workflow management. Enables concurrent feature development, isolated testing environments, and seamless integration with CI/CD pipelines.

## Capabilities

- **Parallel Development**: Create and manage multiple isolated worktrees for concurrent development
- **Environment Isolation**: Complete separation of development branches and workspaces
- **Quality Integration**: Automatic QA execution per worktree with status tracking
- **Terminal Management**: Automated terminal launching and process management
- **Workflow Automation**: Git operations, merging, and conflict resolution
- **Resource Management**: Worktree lifecycle management and cleanup automation

## Tools Required

### MCP Tools
- `codex_read_file` - Reading worktree configurations and status files
- `codex_write_file` - Generating worktree metadata and reports
- `codex_codebase_search` - Analyzing project structure across worktrees
- `grep` - Searching for worktree-related files and patterns
- `read_file` - File access for worktree operations
- `write` - Creating worktree artifacts and logs

### File System Access
- **Read**: Full repository access for worktree management
- **Write**: Limited to `./worktrees`, `./.worktrees.json`, `./worktree-artifacts`

### Network Access
- **None required** - Local Git operations only

### Shell Commands
- `git worktree` - Git worktree operations (add, remove, list, prune)
- `git checkout` - Branch switching and creation
- `git merge` - Worktree merging operations
- `git status` - Worktree status checking
- `git log` - Worktree history analysis
- `ps` - Process management for terminal instances

## Usage Examples

### Create Parallel Development Environment
```bash
codex $worktree-manager "Create parallel development environment for user authentication feature"
```

### Manage Feature Branches
```bash
codex $worktree-manager "Set up isolated worktrees for auth, payment, and UI features"
```

### QA-Integrated Development
```bash
codex $worktree-manager "Create worktree with automatic QA validation for security audit"
```

### Worktree Lifecycle Management
```bash
codex $worktree-manager "Manage worktree lifecycle: create, develop, qa, merge, cleanup"
```

## Output Format

### Worktree Management Report
```
ðEEWorktree Manager Report - Parallel Development Environment
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Operation: Create Parallel Environment
Timestamp: 2026-01-04 15:30:00 UTC
Duration: 45.2 seconds

ð Environment Summary
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Total Worktrees: 4 (3 active, 1 inactive)
Active Branches: feature/auth, feature/payment, feature/ui
Base Branch: main
Repository: /path/to/project

ð¿ Worktree Status Overview
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
âââââââââââââââââââ¬âââââââââââââ¬âââââââââââââ¬âââââââââââââ¬ââââââââââââââââââE
âEWorktree        âEBranch     âEStatus     âEQA Status  âELast Activity   âE
âââââââââââââââââââ¼âââââââââââââ¼âââââââââââââ¼âââââââââââââ¼ââââââââââââââââââ¤
âEauth-feature    âEfeature/auth âEactive    âEqa_passed âE15:28:33        âE
âEpayment-feature âEfeature/pay âEactive    âEqa_runningâE15:29:45        âE
âEui-feature      âEfeature/ui  âEactive    âEqa_pendingâE15:30:00        âE
âEold-experiment  âEexperiment  âEinactive  âEqa_failed âE2026-01-03      âE
âââââââââââââââââââ´âââââââââââââ´âââââââââââââ´âââââââââââââ´ââââââââââââââââââE

ð¥EETerminal Management
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Active Terminals: 3
ââ auth-feature (PID: 12345) - /path/to/project/auth-feature
ââ payment-feature (PID: 12346) - /path/to/project/payment-feature
ââ ui-feature (PID: 12347) - /path/to/project/ui-feature

ð¬ QA Integration Status
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
QA Service: Active (monitoring 3 worktrees)
Recent QA Results:
âââ auth-feature: âEPASSED (security: A+, performance: A)
âââ payment-feature: ð RUNNING (45% complete)
âââ ui-feature: â³ PENDING (waiting for changes)

ð Worktree Artifacts
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
âââ worktrees/auth-feature/artifacts/qa_report.json
âââ worktrees/payment-feature/artifacts/qa_report.json
âââ worktrees/ui-feature/artifacts/ (empty - awaiting development)
âââ .worktrees.json (management metadata)

âï¸EConfiguration
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Max Concurrent Worktrees: 5
Auto QA: Enabled
Cleanup Policy: 7 days inactive
Terminal Auto-launch: Enabled

ð Recommended Actions
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
1. Start development in ui-feature worktree
2. Monitor QA results in payment-feature
3. Consider merging auth-feature (QA passed)
4. Review and cleanup old-experiment worktree

ð¡ Parallel Development Tips
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
- Each worktree has isolated node_modules and build artifacts
- QA runs automatically on file changes (debounced 2s)
- Use 'worktree-manager merge <name>' when ready to integrate
- Terminals auto-close on worktree removal

ð Performance Metrics
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Worktree Creation Time: 2.3s average
QA Execution Time: 45.2s average
Terminal Launch Time: 1.1s average
Resource Usage: CPU 15%, Memory 250MB
```

### Worktree Status JSON
```json
{
  "worktree_manager_report": {
    "timestamp": "2026-01-04T15:30:00Z",
    "operation": "parallel_environment_setup",
    "duration_seconds": 45.2,
    "environment_summary": {
      "total_worktrees": 4,
      "active_worktrees": 3,
      "inactive_worktrees": 1,
      "base_branch": "main",
      "repository_path": "/path/to/project"
    },
    "worktrees": [
      {
        "name": "auth-feature",
        "branch": "feature/auth",
        "path": "/path/to/project/auth-feature",
        "status": "active",
        "qa_status": "qa_passed",
        "terminal_pid": 12345,
        "created_at": "2026-01-04T15:25:00Z",
        "last_activity": "2026-01-04T15:28:33Z",
        "qa_report_path": "/path/to/project/auth-feature/artifacts/qa_report.json"
      }
    ],
    "qa_integration": {
      "service_active": true,
      "monitored_worktrees": 3,
      "recent_results": [
        {
          "worktree": "auth-feature",
          "status": "passed",
          "security_score": "A+",
          "performance_score": "A",
          "completed_at": "2026-01-04T15:28:33Z"
        }
      ]
    },
    "performance_metrics": {
      "avg_creation_time": 2.3,
      "avg_qa_time": 45.2,
      "cpu_usage_percent": 15,
      "memory_usage_mb": 250
    },
    "recommendations": [
      "Start development in ui-feature worktree",
      "Monitor QA results in payment-feature",
      "Consider merging auth-feature (QA passed)",
      "Review and cleanup old-experiment worktree"
    ]
  }
}
```

## Worktree Operations

### Creation Patterns
```bash
# Feature development
codex $worktree-manager "Create feature worktree for user authentication"

# Bug fix isolation
codex $worktree-manager "Create isolated worktree for critical bug fix"

# Experimental development
codex $worktree-manager "Create experimental worktree for new architecture"
```

### QA Integration
```bash
# Automatic QA on creation
codex $worktree-manager "Create worktree with QA validation"

# Manual QA trigger
codex $worktree-manager "Run QA analysis on auth-feature worktree"

# Continuous monitoring
codex $worktree-manager "Enable background QA monitoring for all worktrees"
```

### Lifecycle Management
```bash
# Status monitoring
codex $worktree-manager "Show status of all worktrees"

# Safe merging
codex $worktree-manager "Merge auth-feature to main (QA validated)"

# Cleanup operations
codex $worktree-manager "Cleanup inactive worktrees older than 7 days"
```

## Worktree Architecture

### Directory Structure
```
project/
âââ .git/                          # Main repository
âââ main-workspace/               # Primary development
âââ worktrees/                    # Worktree container
âE  âââ feature-auth/            # Feature worktree
âE  âE  âââ .git -> ../.git      # Git link
âE  âE  âââ src/                 # Isolated source
âE  âE  âââ node_modules/        # Isolated dependencies
âE  âE  âââ artifacts/           # QA reports, builds
âE  âE  âââ .worktree_metadata   # Worktree info
âE  âââ feature-payment/         # Another feature
âE  âââ experimental-ui/         # Experimental work
âââ .worktrees.json              # Management metadata
âââ worktree-artifacts/          # Shared artifacts
```

### Isolation Benefits
- **Dependency Isolation**: Each worktree has separate node_modules, target/
- **Build Isolation**: Independent build artifacts and caches
- **QA Isolation**: Separate QA reports and results per worktree
- **Branch Isolation**: Complete Git branch separation

### Resource Management
- **Automatic Cleanup**: Inactive worktrees removed after configurable period
- **Disk Usage Monitoring**: Track worktree sizes and cleanup recommendations
- **Process Management**: Terminal processes tracked and cleaned up
- **Git Maintenance**: Worktree pruning and repository optimization

## QA Integration Architecture

### Background QA Service
```python
# Automatic QA monitoring per worktree
qa_service = BackgroundQAService()
qa_service.add_worktree("feature-auth", Path("./worktrees/feature-auth"))
qa_service.start()  # Monitors file changes, runs QA automatically
```

### Pre-merge Validation
```bash
# Git hooks integration
codex $worktree-manager "Setup pre-merge QA hooks"

# Validates before merging to main
git checkout main
git merge feature/auth  # QA validation runs automatically
```

### CI/CD Integration
```yaml
# GitHub Actions workflow
- name: Worktree QA Validation
  run: |
    codex $worktree-manager "Validate all active worktrees"
    codex $worktree-manager "Generate merge readiness report"
```

## Configuration

### Worktree Manager Settings
```json
{
  "worktree_manager": {
    "base_path": "./worktrees",
    "max_concurrent_worktrees": 5,
    "auto_qa_enabled": true,
    "qa_interval_seconds": 300,
    "cleanup_inactive_days": 7,
    "terminal_auto_launch": true,
    "resource_monitoring": true
  }
}
```

### QA Integration Settings
```json
{
  "qa_integration": {
    "background_service_enabled": true,
    "file_change_debounce_seconds": 2.0,
    "qa_timeout_seconds": 300,
    "parallel_qa_limit": 2,
    "auto_merge_qa_passed": false,
    "notification_webhooks": [
      "https://slack.com/webhook/...",
      "https://discord.com/api/webhooks/..."
    ]
  }
}
```

### Git Integration
```json
{
  "git_integration": {
    "auto_setup_hooks": true,
    "pre_merge_qa_enabled": true,
    "branch_naming_convention": "feature/*,bugfix/*,hotfix/*",
    "merge_commit_messages": true,
    "conflict_resolution_assistance": true
  }
}
```

## Performance Optimization

### Worktree Creation
- **Template Reuse**: Copy from existing worktrees for faster setup
- **Dependency Caching**: Shared cache for common dependencies
- **Parallel Setup**: Concurrent worktree initialization

### QA Execution
- **Incremental Analysis**: Only analyze changed files
- **Caching**: Cache analysis results across similar worktrees
- **Parallel QA**: Multiple worktrees analyzed simultaneously

### Resource Management
- **Memory Limits**: Configurable memory limits per worktree
- **Disk Quotas**: Automatic cleanup when disk usage exceeds limits
- **Process Limits**: Terminal and background process management

## Troubleshooting

### Common Issues

#### Worktree Creation Failures
```bash
# Check Git worktree support
git worktree --help

# Clean orphaned worktrees
git worktree prune --verbose

# Check disk space
df -h
```

#### QA Integration Problems
```bash
# Restart QA service
codex $worktree-manager "Restart QA service"

# Check QA service status
codex $worktree-manager "Show QA service status"

# Manual QA trigger
codex $worktree-manager "Run QA on specific-worktree"
```

#### Terminal Launch Issues
```bash
# Check terminal availability
which gnome-terminal || which konsole || which xterm

# Manual terminal launch
codex $worktree-manager "Launch terminal for worktree-name"

# Check process limits
ulimit -u
```

#### Merge Conflicts
```bash
# Check worktree status before merge
codex $worktree-manager "Validate worktree-name for merge"

# Resolve conflicts manually
cd worktrees/worktree-name
git mergetool

# Retry merge
codex $worktree-manager "Merge worktree-name"
```

## Best Practices

### Development Workflow
1. **Create Descriptive Worktrees**: Use clear names like `feature/user-auth-api`
2. **Regular QA Checks**: Run QA frequently during development
3. **Keep Worktrees Focused**: One feature or fix per worktree
4. **Regular Merging**: Don't let worktrees get too far behind main
5. **Clean Up Regularly**: Remove completed or abandoned worktrees

### QA Integration
1. **Early QA**: Run QA as soon as worktree is created
2. **Continuous Monitoring**: Keep background QA service running
3. **Address Issues Early**: Fix QA issues before they compound
4. **Review QA Reports**: Use reports to improve development practices

### Resource Management
1. **Monitor Disk Usage**: Regular cleanup of old worktrees
2. **Limit Concurrent Worktrees**: Don't exceed system capacity
3. **Close Unused Terminals**: Free up system resources
4. **Regular Maintenance**: Run `git worktree prune` periodically

## Integration Examples

### Development Team Workflow
```bash
# Team lead creates worktree structure
codex $worktree-manager "Setup team worktrees for sprint"

# Developers work in parallel
codex $worktree-manager "Create my-feature worktree"
# ... development work ...
codex $worktree-manager "Mark my-feature ready for review"

# QA team validates
codex $worktree-manager "Run QA on all ready worktrees"

# Merge validated worktrees
codex $worktree-manager "Merge all qa-passed worktrees"
```

### CI/CD Pipeline
```yaml
# Comprehensive CI/CD workflow
- name: Setup Parallel Development
  run: codex $worktree-manager "Create CI worktrees"

- name: Parallel QA Validation
  run: codex $worktree-manager "Validate all worktrees"

- name: Merge Ready Worktrees
  run: codex $worktree-manager "Auto-merge qa-passed worktrees"

- name: Cleanup
  run: codex $worktree-manager "Cleanup completed worktrees"
```

### Automated Maintenance
```yaml
# Daily maintenance cron job
0 2 * * * codex $worktree-manager "Daily maintenance: cleanup old worktrees, update QA baselines"
```

## References

- [Git Worktree Documentation](https://git-scm.com/docs/git-worktree)
- [Parallel Development Patterns](https://martinfowler.com/bliki/FeatureBranch.html)
- [Branching Strategies](https://www.atlassian.com/git/tutorials/comparing-workflows)
- [Git Flow](https://nvie.com/posts/a-successful-git-branching-model/)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-worktree-manager-skill`
**Version**: 2.10.0
**Compatibility**: Codex v2.10.0+
