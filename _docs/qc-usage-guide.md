# QC Orchestrator Usage Guide

The QC Orchestrator is a production-ready quality check system for AI-assisted multi-worktree development. It performs automated pre-merge checks including testing, linting, diff analysis, and risk assessment.

## Basic Usage

### Running QC with Default Profile

```bash
codex qc
```

This runs the `standard` profile by default, which includes:
- All Rust tests (`cargo test --all`)
- Rust linting with warnings as errors (`cargo clippy --all --all-targets -- -D warnings`)
- Web/GUI tests if available (`pnpm test` or `npm test`)

### Specifying a Test Profile

```bash
# Minimal profile - fastest, for quick checks
codex qc --profile minimal

# Standard profile - balanced (default)
codex qc --profile standard

# Full profile - comprehensive validation
codex qc --profile full
```

## Test Profiles

See [test-profiles.md](test-profiles.md) for detailed information about each profile.

## Output

The QC orchestrator provides:

1. **Console Output** - Real-time feedback with:
   - Test execution status
   - Summary statistics (lines changed, files changed, risk score)
   - Merge recommendation
   - Warnings (if any)

2. **Log File** - Structured markdown log in `_docs/logs/`:
   - File naming: `YYYY-MM-DD-{worktreename}-impl.md`
   - Includes timestamp with timezone
   - Test results with pass/fail indicators
   - Full metrics and recommendation

### Example Console Output

```
🔍 Running QC checks...

Profile: standard
Worktree: feature-new-component
Branch: feature/new-component

Running: cargo test --all
✅ Command succeeded

Running: cargo clippy --all --all-targets -- -D warnings
✅ Command succeeded

📊 QC Summary:
─────────────────────────────────────
Changed lines: +150 / -30 (Total: 180)
Files changed: 5
Risk score: 0.42
Recommendation: ✅ Approve (safe to merge)

📝 Log written to: _docs/logs/2025-11-19-feature-new-component-impl.md
```

### Example Log Entry

```markdown
## 2025-11-19 13:40:12 +0900

- Worktree: feature-new-component
- 機能: Add new React component for user profile
- Profile: standard
- Changed lines: +150 / -30 (Total: 180)
- Files changed: 5
- Risk score: 0.42
- Recommendation: ✅ Approve (safe to merge)

### Test Results

**Rust:**
- ✅ `cargo test --all`
- ✅ `cargo clippy --all --all-targets -- -D warnings`

**Web/GUI:**
- ✅ `pnpm test`

---
```

## The 200-Line Rule

The QC orchestrator enforces a **200-line rule** for merge recommendations:

- If **total changed lines (additions + deletions) exceed 200**, the orchestrator will **automatically recommend opening a PR** for human review
- This helps ensure large changes get proper review attention
- The recommendation appears in both console output and log files

### Example: Large Change Triggering PR Recommendation

```
📊 QC Summary:
─────────────────────────────────────
Changed lines: +180 / -75 (Total: 255)
Files changed: 12
Risk score: 0.68
Recommendation: 🔍 Request PR (review recommended)

⚠️  Warnings:
  - Total changed lines (255) exceeds 200-line threshold
```

## Risk Scoring

The QC orchestrator calculates a risk score (0.0 to 1.0) based on:
- **70% weight**: Number of changed lines (normalized to 500 lines max)
- **30% weight**: Number of files changed (normalized to 20 files max)

Risk score influences the merge recommendation:
- **< 0.7**: Generally safe to merge (if tests pass)
- **≥ 0.7**: Recommend PR review

## Merge Recommendations

The orchestrator provides three types of recommendations:

1. **✅ Approve (safe to merge)**
   - All tests passed
   - Changed lines ≤ 200
   - Risk score < 0.7

2. **🔍 Request PR (review recommended)**
   - Tests passed but:
     - Changed lines > 200 lines, OR
     - Risk score ≥ 0.7

3. **❌ Reject (tests failed)**
   - One or more tests failed
   - Should not merge until fixed

## Integration with Development Workflow

### Pre-commit Hook (Recommended)

Add QC checks to your pre-commit workflow:

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Running QC checks..."
QC_OUTPUT=$(codex qc --profile minimal)
QC_EXIT=$?
echo "$QC_OUTPUT"

# Parse recommendation from QC output
RECOMMENDATION=$(echo "$QC_OUTPUT" | grep -Eo 'Recommendation: (Approve|Request PR|Reject)' | awk '{print $2}')

if [ "$QC_EXIT" -ne 0 ] || [ "$RECOMMENDATION" = "Request PR" ] || [ "$RECOMMENDATION" = "Reject" ]; then
    echo "QC checks did not approve this commit. Recommendation: $RECOMMENDATION"
    echo "Please fix issues or open a PR for review before committing."
    exit 1
fi
```

### Pre-merge Workflow

1. Make your changes in a worktree
2. Run `codex qc` (or `codex qc --profile full` for critical changes)
3. Review the recommendation:
   - **Approve**: Merge directly
   - **Request PR**: Open a PR for review
   - **Reject**: Fix failing tests first

### CI/CD Integration

The QC orchestrator can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run QC checks
  run: |
    codex qc --profile full
    if [ $? -ne 0 ]; then
      echo "QC checks failed"
      exit 1
    fi
```

## Configuration

The default test profile can be configured in `config.toml`:

```toml
[qc]
default_profile = "standard"  # Options: minimal, standard, full
```

## Worktree Detection

The QC orchestrator automatically detects:
- Current Git worktree path
- Worktree name (from branch name or path)
- Current branch

This information is included in log files for tracking and organization.

## Troubleshooting

### Tests Running from Wrong Directory

The orchestrator automatically runs Rust tests from the `codex-rs` subdirectory if it exists. If you have a different project structure, you may need to adjust your test commands.

### Log Files Not Created

Ensure the `_docs/logs` directory exists or can be created. The orchestrator will create it automatically if the parent `_docs` directory exists.

### Worktree Detection Fails

The orchestrator requires running from within a Git repository. Ensure:
- You're in a Git repository
- Git is installed and accessible

## Advanced Usage

### Custom Test Commands

While the orchestrator provides predefined profiles, you can extend it by:
1. Modifying profile commands in `codex-rs/core/src/qc/profiles.rs`
2. Adding new profiles for specific workflows
3. Creating wrapper scripts that run `codex qc` with additional checks

### Analyzing Logs

Logs are stored in markdown format for easy reading and parsing. You can:
- View logs directly with any markdown viewer
- Parse logs programmatically for metrics
- Aggregate logs for trend analysis

Example: Count test runs by profile
```bash
grep "Profile:" _docs/logs/*.md | sort | uniq -c
```

## Future Enhancements

Potential future additions:
- Custom test profile definitions in config
- Integration with specific Tauri GUI components
- Historical trend analysis
- Coverage threshold enforcement
- Security vulnerability scanning integration
