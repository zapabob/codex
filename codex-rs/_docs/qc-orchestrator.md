# QC Orchestrator

## Overview

The QC (Quality Control) Orchestrator is a production-ready sub-agent feature that automates quality checks for code changes in the Codex repository. It provides automated testing, diff analysis, risk assessment, and PR recommendations based on configurable policies.

## Features

- **Automated Testing**: Runs Rust tests, linting (Clippy), and web tests based on the selected test profile
- **Git Integration**: Uses git2 to compute diff statistics between branches
- **Risk Scoring**: Calculates risk scores based on test results and diff size
- **200-line Rule**: Automatically recommends PR creation for changes exceeding 200 lines
- **Structured Logging**: Writes human-readable QC logs to `_docs/logs` in markdown format
- **Multiple Test Profiles**: Supports Minimal, Standard, and Full test profiles

## Usage

### CLI Command

```bash
codex qc [OPTIONS]
```

### Options

- `--feature <FEATURE>`: Description of the feature being tested (optional)
- `--profile <PROFILE>`: Test profile to use: `minimal`, `standard`, or `full` (optional, defaults to standard)
- `--agent-name <AGENT_NAME>`: Logical agent name (optional, defaults to `codex-cli-agent`)
- `--ai-name <AI_NAME>`: AI model identifier (optional, defaults to `claude-code`)

### Examples

#### Minimal Testing (Fast)
```bash
codex qc --feature "Add new API endpoint" --profile minimal
```

#### Standard Testing (Recommended)
```bash
codex qc --feature "Refactor authentication module" --profile standard
```

#### Full Testing (Comprehensive)
```bash
codex qc --feature "Major release preparation" --profile full
```

## Test Profiles

### Minimal Profile
- Rust CLI tests only (`cargo test -p codex-cli`)
- Fast execution (~30 seconds)
- Suitable for quick checks during development

### Standard Profile (Default)
- All Rust tests (`cargo test --all`)
- Rust linting (`cargo clippy --all --all-targets -- -D warnings`)
- Web tests (`pnpm test` or `npm test`)
- Moderate execution time (~5-10 minutes)
- Recommended for pre-commit checks

### Full Profile
- All tests from Standard profile
- Rust code coverage (`cargo tarpaulin --workspace`, if available)
- Web linting (`pnpm lint` or `npm run lint`)
- Longest execution time (~15-20 minutes)
- Recommended for pre-release validation

## Output

The QC orchestrator provides:

1. **Console Summary**: Real-time feedback with:
   - Timestamp and worktree information
   - Diff statistics (files and lines changed)
   - Risk score (0.0-1.0)
   - Recommendation (MergeOk, NeedsFix, CreatePrForReview)
   - Test results

2. **Log Files**: Detailed markdown logs in `_docs/logs/` with:
   - Complete test execution details
   - Warnings and issues found
   - Risk assessment rationale
   - Recommendation reasoning

### Log File Format

Logs are written to: `_docs/logs/YYYY-MM-DD-{worktree}-impl.md`

Each QC run appends a new section with:
- Timestamp (with timezone)
- Worktree/branch name
- Feature description
- Change statistics
- Test results (passed/failed/skipped)
- Risk score and recommendation
- Detailed reasons and issues

## Risk Scoring

The risk score (0.0-1.0) is calculated based on:

- **Test Failures**: +0.3 per failed test (max 0.6)
- **Diff Size**: 
  - +0.2 for 200-499 lines changed
  - +0.4 for 500+ lines changed

Higher risk scores lead to stricter recommendations.

## Recommendations

### MergeOk
- All tests pass
- Changes are under 200 lines
- Risk score < 0.7
- Safe to merge directly

### NeedsFix
- One or more tests failed
- Must address issues before merging
- Re-run QC after fixes

### CreatePrForReview
- Changes exceed 200 lines (200行ルール)
- Automatic PR recommendation
- Human review recommended even if tests pass

## Configuration

### Default Configuration

```rust
QcConfig {
    default_profile: TestProfile::Standard,
    max_lines_without_pr: 200,
    base_ref: "main",
}
```

The orchestrator automatically falls back to alternative base references if the configured one is not found:
1. Configured `base_ref`
2. `origin/main`
3. `origin/master`
4. `HEAD~1`

## Architecture

### Core Components

1. **qc_orchestrator.rs**: Main orchestration logic
   - `run_qc()`: Entry point for QC execution
   - `compute_diff_stats()`: Git diff analysis
   - `run_tests()`: Test execution
   - `compute_risk_score()`: Risk calculation
   - `build_recommendation()`: Decision logic
   - `write_log()`: Log file generation

2. **CLI Integration**: `codex-rs/cli/src/main.rs`
   - Clap-based argument parsing
   - QC subcommand handler
   - Result formatting and display

### Type System

```rust
pub enum TestProfile { Minimal, Standard, Full }
pub struct QcConfig { ... }
pub struct QcInput { ... }
pub struct DiffStats { changed_lines, changed_files }
pub enum CommandStatus { NotRun, Passed, Failed }
pub struct TestResult { label, command, status, warnings }
pub enum Recommendation { MergeOk, NeedsFix, CreatePrForReview }
pub struct QcResult { ... }
```

## Dependencies

- **git2**: Git repository operations
- **chrono**: Timestamp handling with timezone support
- **clap**: CLI argument parsing
- **anyhow**: Error handling

## Best Practices

1. **Run QC Before Committing**: Catch issues early
   ```bash
   codex qc --profile standard
   ```

2. **Review Log Files**: Check `_docs/logs/` for detailed analysis
   ```bash
   cat _docs/logs/2025-11-19-feature-branch-impl.md
   ```

3. **Follow Recommendations**: Respect the 200-line rule and PR recommendations

4. **Fix Issues Promptly**: Re-run QC after addressing failures
   ```bash
   # Fix issues...
   codex qc --feature "Fix test failures"
   ```

5. **Use Full Profile for Releases**: Ensure comprehensive testing
   ```bash
   codex qc --profile full --feature "v2.4.0 release"
   ```

## Integration with Tauri GUI

The QC orchestrator is designed to be callable from both the CLI and future Tauri GUI implementations. The structured output and log format support both command-line and graphical interfaces.

## Troubleshooting

### "Failed to resolve base reference"
- Ensure you're in a Git repository
- Check that the base branch exists
- The orchestrator will automatically try fallback references

### "cargo test failed"
- Ensure the `codex-rs` directory exists
- Verify Cargo.toml is properly configured
- Check that dependencies are installed

### "pnpm/npm not found"
- Web tests will be skipped if neither is available
- Install pnpm: `npm install -g pnpm`
- This is expected in Rust-only repositories

## Future Enhancements

- [ ] Configurable base reference via CLI argument
- [ ] Custom test profiles via config file
- [ ] Parallel test execution
- [ ] Integration with CI/CD pipelines
- [ ] GUI integration in Tauri app
- [ ] Historical QC analytics
- [ ] Custom risk score weights
- [ ] Email/Slack notifications

## License

See repository LICENSE file.
