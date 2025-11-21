# Test Profiles

QC orchestrator test profiles define which tests and checks are run during pre-merge quality validation.

## Available Profiles

### `minimal`
Fastest profile for quick validation. Runs only essential tests.

**Rust:**
- `cargo test -p codex-cli` - Test CLI package only

**Web/GUI:**
- No tests

**Use case:** Quick feedback during development, small changes

---

### `standard` (default)
Balanced profile for most development workflows. Recommended for regular commits.

**Rust:**
- `cargo test --all` - Run all Rust tests across all packages
- `cargo clippy --all --all-targets -- -D warnings` - Lint all Rust code with warnings as errors

**Web/GUI:**
- `pnpm test` (or `npm test` if pnpm unavailable) - Run tests in appropriate packages

**Use case:** Standard development workflow, regular commits

---

### `full`
Comprehensive validation for critical changes or pre-merge verification.

**Rust:**
- All tests from `standard` profile
- `cargo tarpaulin --workspace` - Code coverage (if available)

**Web/GUI:**
- All tests from `standard` profile
- `pnpm lint` (or `npm run lint`) - Web lint validation

**Use case:** Critical changes, release preparation, pre-merge final check

---

## Configuration

Set the default test profile in `config.toml`:

```toml
[qc]
default_profile = "standard"
```

Or specify a profile when running QC:

```bash
codex qc --profile full
```

## Adding Custom Profiles

Future versions may support custom test profiles defined in configuration files.
