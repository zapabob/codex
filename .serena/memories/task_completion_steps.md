# Task Completion Steps

## After Making Code Changes

1. **Format Code**
   ```powershell
   just fmt
   # or
   cd codex-rs
   cargo fmt -- --config imports_granularity=Item
   ```

2. **Run Linting**
   ```powershell
   just clippy
   # or
   cd codex-rs
   cargo clippy --all-features --tests
   ```

3. **Auto-fix Issues (if any)**
   ```powershell
   just fix
   # or
   cd codex-rs
   cargo clippy --fix --all-features --tests --allow-dirty
   ```

4. **Build Check**
   ```powershell
   cd codex-rs
   cargo check --all-features
   ```

5. **Run Tests** (if applicable)
   ```powershell
   just test
   # or
   cd codex-rs
   cargo nextest run --no-fail-fast
   ```

## Before Committing

1. **Ensure Zero Errors/Warnings**
   - All `cargo check` errors must be resolved
   - All `cargo clippy` warnings must be fixed or suppressed with good reason

2. **Test Installation** (if binary changes)
   ```powershell
   cd codex-rs
   cargo install --path cli --force
   cargo install --path tui --force
   codex --version
   codex --help
   ```

3. **Update Documentation** (if APIs changed)
   - Update doc comments
   - Update README if needed
   - Update CHANGELOG.md

## Git Commit Best Practices

- **Staging**: Only stage relevant files (exclude `target/`, temporary files)
- **Commit Messages**: Use conventional format
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation
  - `refactor:` for code restructuring
  - `test:` for test additions
- **Atomic Commits**: Each commit should do one logical change

## Performance Verification

- **Build Performance**: Monitor build times, use incremental builds
- **Runtime Performance**: Test with realistic workloads
- **Memory Usage**: Check for memory leaks in long-running processes

## Cross-Platform Testing

- **Windows**: Test sandboxing, path handling, permissions
- **Linux/macOS**: Test with equivalent environments
- **VR/AR**: Test with supported hardware if available

## Final Verification

1. **Clean Build**
   ```powershell
   cd codex-rs
   cargo clean
   cargo build --release --all-features
   ```

2. **Integration Tests**
   ```powershell
   cargo test --test integration
   ```

3. **Documentation Build** (if applicable)
   ```powershell
   cargo doc --no-deps --all-features
   ```