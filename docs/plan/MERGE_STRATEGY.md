# Zapabob Codex Merge Strategy

## Overview

This document describes the strategy for merging upstream OpenAI Codex changes while preserving zapabob's custom features.

## Custom Features Summary

### Slash Commands

- `/qc` - Quality Control analysis
- `/dev-mode` - Dev-mode orchestration
- `/git4d` - Git 4D visualization
- `/vr` - VR mode for Git 4D
- `/ar` - AR mode for Git 4D

### Custom Scripts

- `zapabob/scripts/load-env.sh` - Environment loading script
- `zapabob/scripts/setup-env-vars.ps1` - PowerShell environment setup
- `merge_with_custom_features.py` - Custom merge tool
- `advanced_merge_resolver.py` - Advanced merge conflict resolver

## Merge Strategy

### Phase 1: Preparation

1. **Identify Custom Features**

   ```bash
   python3 advanced_merge_resolver.py --identify
   ```

2. **Backup Current State**
   ```bash
   git backup-branch custom-features-$(date +%Y%m%d)
   ```

### Phase 2: Fetch Upstream

```bash
git fetch upstream
```

### Phase 3: Analyze Differences

```bash
# View upstream changes
git diff upstream/main...HEAD --stat

# Identify conflicts
git merge --no-commit upstream/main
git diff --name-only --diff-filter=U
```

### Phase 4: Merge with Custom Preservation

#### Automatic Merge

```bash
python3 advanced_merge_resolver.py
```

#### Manual Merge (if needed)

1. **For slash_command.rs**:
   - Preserve custom enum variants
   - Maintain description mappings
   - Keep feature gating rules

2. **For other files**:
   - Use three-way merge strategy
   - Prefer upstream for standard features
   - Preserve local for custom features

### Phase 5: Verification

```bash
# Check custom features preserved
python3 advanced_merge_resolver.py --verify

# Run tests
cd codex-rs && cargo test -p codex-tui

# Verify build
cargo build
```

## Conflict Resolution Rules

### Priority Order

1. **Custom Features**: Always preserve (Qc, DevMode, Git4d, Vr, Ar)
2. **Bug Fixes**: Prefer upstream
3. **New Features**: Prefer upstream if equivalent
4. **Documentation**: Prefer upstream
5. **Configuration**: Merge carefully, test thoroughly

### Three-Way Merge Logic

```
Local (HEAD)    Upstream      Result
-----------     --------      ------
Custom cmd      Standard      → Custom cmd (preserve)
Standard        Standard      → Upstream (use new)
Custom cmd      Modified     → Merge, preserve custom
```

## Troubleshooting

### Common Issues

#### 1. Conflict in slash_command.rs

**Solution**: Run the merge resolver

```bash
python3 advanced_merge_resolver.py --file codex-rs/tui/src/slash_command.rs
```

#### 2. Custom Features Not Detected

**Solution**: Check feature detection

```bash
python3 -c "
from advanced_merge_resolver import MergeConflictResolver
resolver = MergeConflictResolver()
features = resolver.identify_custom_features()
print(features)
"
```

#### 3. Build Failures After Merge

**Solution**: Check for missing dependencies

```bash
cd codex-rs && cargo check
```

## Automation Script

```bash
#!/bin/bash
# Full merge automation

set -e

echo "=== Zapabob Codex Merge Tool ==="

# Step 1: Backup
echo "[1/4] Backing up current state..."
git stash push -m "pre-merge-backup-$(date +%Y%m%d)"

# Step 2: Fetch
echo "[2/4] Fetching upstream..."
git fetch upstream

# Step 3: Merge
echo "[3/4] Merging upstream changes..."
if git merge upstream/main --no-edit; then
    echo "Merge successful!"
else
    echo "Conflicts detected, running resolver..."
    python3 advanced_merge_resolver.py
fi

# Step 4: Verify
echo "[4/4] Verifying custom features..."
python3 advanced_merge_resolver.py --verify

echo "=== Merge Complete ==="
```

## Rollback Procedure

If something goes wrong:

```bash
# Abort merge
git merge --abort

# Restore from backup
git stash pop

# Or reset to previous state
git reset --hard HEAD@{1}
```

## Maintenance

### Regular Upstream Sync

Schedule regular merges to avoid large conflicts:

- Weekly for active development
- Monthly for stable releases

### Testing Matrix

| Upstream Version | Local Version | Status |
| ---------------- | ------------- | ------ |
| v2.14.0          | Current       | ✓ OK   |
| v2.14.1          | Current       | ✓ OK   |

## References

- [OpenAI Codex Repository](https://github.com/openai/codex)
- [Zapabob Codex Repository](https://github.com/zapabob/Codex)
- [Plan Mode Documentation](IMPLEMENTATION.md)
