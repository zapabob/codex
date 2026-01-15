# QA Integration Guide - Parallel Development with Quality Assurance

## Overview

Complete QA-powered parallel development system that automatically reviews code quality before integration, featuring mathematical optimization analysis, quantum computing principles, and advanced software engineering best practices.

## System Components

### 1. QA Engineer Skill (`qa-engineer/`)
Advanced QA analysis with:
- **Mathematical Optimization**: Algorithmic complexity, Big O analysis
- **Quantum Optimization**: Quantum circuit efficiency, gate optimization
- **Software Engineering**: SOLID principles, clean architecture
- **Code Quality**: Readability, maintainability, extensibility metrics

### 2. Git Worktree Manager (`worktree_manager.py`)
Parallel development environment management:
- Automated worktree creation/deletion
- Terminal launching for each worktree
- QA integration and status tracking

### 3. Background QA Service (`background_qa_service.py`)
Continuous quality monitoring:
- File system change detection
- Automatic QA execution
- Background processing with watchdog

### 4. Pre-merge QA Hook (`premerge_qa_hook.py`)
Git integration for quality gates:
- Pre-merge QA validation
- Merge blocking based on quality criteria
- Detailed merge reports

## Quick Start

### 1. Install QA Components
```bash
# Install QA skill
codex $skill-install https://github.com/zapabob/codex-qa-engineer-skill

# Setup worktree manager
pip install -e tools/codex-supervisor/
```

### 2. Create Parallel Development Environment
```bash
# Create feature worktree with terminal
python tools/worktree_manager.py create feature/user-auth auth-feature "User authentication system"
python tools/worktree_manager.py launch auth-feature

# Create another worktree for parallel development
python tools/worktree_manager.py create feature/payment payment-feature "Payment processing"
python tools/worktree_manager.py launch payment-feature
```

### 3. Start Background QA Monitoring
```bash
# Start background QA service
python tools/background_qa_service.py --auto-discover-worktrees --daemon

# Check service status
python tools/background_qa_service.py --status
```

### 4. Install Git Hooks for Pre-merge QA
```bash
# Install pre-merge QA hooks
python tools/premerge_qa_hook.py --install-hooks
```

## Advanced Usage

### Parallel Development Workflow

#### Terminal 1: Feature Development
```bash
# Create and enter worktree
python tools/worktree_manager.py create feature/new-feature feat-001 "New feature implementation"
python tools/worktree_manager.py launch feat-001

# In the new terminal:
cd feat-001
# Develop code with real-time QA monitoring
```

#### Terminal 2: QA Monitoring
```bash
# Monitor all worktrees
python tools/worktree_manager.py list

# Run QA on specific worktree
python tools/worktree_manager.py qa feat-001

# View QA results
cat feat-001/artifacts/qa_report.md
```

#### Terminal 3: Background QA Service
```bash
# Start continuous monitoring
python tools/background_qa_service.py --watch . --qa-interval 180 --daemon

# Check service stats
python tools/background_qa_service.py --status
```

### Pre-merge Quality Assurance

#### Automatic Pre-merge QA
```bash
# Git hooks automatically run QA before merge
git checkout main
git merge feature/user-auth  # QA runs automatically

# If QA fails, merge is blocked with detailed report
cat merge-qa-reports/merge_qa_report_*.md
```

#### Manual Pre-merge QA Check
```bash
# Run pre-merge QA manually
python tools/premerge_qa_hook.py feature/user-auth main

# Check results
cat merge-qa-results.json
```

## QA Criteria and Scoring

### Quality Dimensions

#### 1. Mathematical Optimization (Algorithmic Complexity)
- **Time Complexity**: Big O notation analysis
- **Space Complexity**: Memory usage optimization
- **Algorithmic Efficiency**: Optimal algorithm selection

**Scoring**: A+ (Excellent) → D (Poor)

#### 2. Quantum Optimization (Circuit Efficiency)
- **Gate Count**: Minimize quantum operations
- **Circuit Depth**: Optimize layer efficiency
- **Qubit Utilization**: Minimize resource requirements

**Scoring**: A (Very Good) → N/A (No quantum code)

#### 3. Software Engineering (Architecture Quality)
- **SOLID Principles**: Single responsibility, open-closed, etc.
- **Design Patterns**: Appropriate pattern usage
- **Clean Architecture**: Proper layer separation

**Scoring**: A+ (Excellent) → C (Poor)

#### 4. Code Quality (Readability & Maintainability)
- **Readability**: Naming, structure, documentation (0-10)
- **Maintainability**: Change ease, complexity control (0-100)
- **Extensibility**: New feature addition facility (0-10)

**Scoring**: A- (Good) → C (Poor)

### Integration Status

#### Merge Blocking Criteria
- **Critical Issues**: Always block merge
- **High Issues**: Block if `block_on_high=true`
- **Quality Score**: Block if below `require_minimum_score`
- **Manual Override**: Allow with `--force` flag

#### Quality Gates
```json
{
  "qa_gates": {
    "block_on_critical": true,
    "block_on_high": false,
    "require_minimum_score": 7.0,
    "max_qa_time": 300
  }
}
```

## Configuration

### QA Service Configuration
```json
{
  "background_qa": {
    "watch_paths": ["./worktrees", "./"],
    "qa_interval": 300,
    "debounce_time": 2.0,
    "max_concurrent_qa": 2,
    "exclude_patterns": ["*.log", "*.tmp", "__pycache__"]
  }
}
```

### Worktree Manager Configuration
```json
{
  "worktree_manager": {
    "base_path": "./worktrees",
    "max_concurrent": 5,
    "auto_qa": true,
    "cleanup_days": 7
  }
}
```

### Pre-merge QA Configuration
```json
{
  "premerge_qa": {
    "block_on_critical": true,
    "block_on_high": false,
    "require_minimum_score": 7.0,
    "generate_diff_report": true
  }
}
```

## Troubleshooting

### Common Issues

#### Worktree Creation Fails
```bash
# Check git worktree support
git worktree --help

# Clean up orphaned worktrees
git worktree prune
```

#### QA Analysis Times Out
```bash
# Increase timeout
export QA_TIMEOUT=600

# Run QA manually for debugging
python .codex/skills/qa-engineer/scripts/run_qa-engineer.py
```

#### Background Service Not Detecting Changes
```bash
# Install watchdog
pip install watchdog

# Check file permissions
ls -la worktrees/
```

#### Pre-merge Hook Not Running
```bash
# Check hook permissions
ls -la .git/hooks/pre-merge-commit

# Test hook manually
.git/hooks/pre-merge-commit main feature/branch
```

## Performance Optimization

### Background QA Tuning
```bash
# Adjust QA frequency
python tools/background_qa_service.py --qa-interval 600  # 10 minutes

# Limit concurrent QA runs
python tools/background_qa_service.py --max-concurrent 1
```

### Worktree Management
```bash
# Auto-cleanup old worktrees
python tools/worktree_manager.py cleanup

# Monitor worktree status
python tools/worktree_manager.py list
```

### QA Analysis Optimization
```bash
# Run targeted QA (skip comprehensive analysis)
QA_LEVEL=standard python tools/premerge_qa_hook.py source target

# Cache QA results
export QA_CACHE_DIR=./.qa_cache
```

## Integration Examples

### CI/CD Pipeline Integration
```yaml
# .github/workflows/pr-qa.yml
name: PR QA Analysis
on: [pull_request]

jobs:
  qa-analysis:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
      with:
        fetch-depth: 0

    - name: Setup QA Environment
      run: |
        pip install -e tools/codex-supervisor/
        codex $skill-install ./qa-engineer/

    - name: Run QA Analysis
      run: |
        python tools/premerge_qa_hook.py ${{ github.base_ref }} ${{ github.head_ref }}

    - name: Upload QA Report
      uses: actions/upload-artifact@v3
      with:
        name: qa-report
        path: merge-qa-reports/
```

### IDE Integration (VS Code)
```json
// .vscode/settings.json
{
  "codex.qa": {
    "enableBackgroundService": true,
    "qaInterval": 300,
    "blockOnCritical": true,
    "worktreeBasePath": "./worktrees"
  }
}

// .vscode/tasks.json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Create Feature Worktree",
      "type": "shell",
      "command": "python",
      "args": ["tools/worktree_manager.py", "create", "feature/${input:featureName}", "${input:branchName}", "${input:description}"],
      "group": "build"
    }
  ]
}
```

## Monitoring and Analytics

### QA Metrics Dashboard
```bash
# Generate QA metrics report
python tools/qa_metrics_analyzer.py --period 30d

# View trends
cat qa-metrics-report.md
```

### Worktree Status Monitoring
```bash
# Continuous monitoring
watch -n 60 'python tools/worktree_manager.py list'

# Alert on QA failures
python tools/qa_alert_monitor.py --email user@company.com
```

## Best Practices

### Development Workflow
1. **Create worktree** for each feature/bugfix
2. **Launch terminal** for isolated development
3. **Background QA** monitors changes automatically
4. **Pre-merge QA** validates before integration
5. **Clean up** worktree after merge

### QA Quality Standards
- **Critical Issues**: Must be resolved before merge
- **High Issues**: Should be addressed, can be exceptions
- **Quality Score**: Maintain >7.0 average
- **Test Coverage**: >80% recommended

### Performance Guidelines
- **QA Interval**: 5-10 minutes for active development
- **Concurrent Limit**: 2-3 worktrees per developer
- **Cleanup**: Remove inactive worktrees weekly
- **Caching**: Enable QA result caching for faster re-runs

## Related Documentation

- [QA Engineer Skill Documentation](./qa-engineer/SKILL.md)
- [Codex Supervisor Guide](../codex-supervisor/README.md)
- [Worktree Manager API](./worktree_manager.py)
- [Background QA Service](./background_qa_service.py)

---

**System Status**: ✅ Fully Operational
**Integration Level**: 🔗 Complete Git Workflow Integration
**QA Coverage**: 🎯 Mathematical + Quantum + Software Engineering