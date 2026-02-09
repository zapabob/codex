# Plan Mode Implementation

## Overview

This document describes the `/plan` mode implementation in zapabob/codex, which provides collaborative planning capabilities while maintaining custom features from the upstream repository.

## Custom Slash Commands

The following custom slash commands have been added to preserve:

### Quality Control Commands

- `/qc` - Run quality control analysis via the CLI
- `/dev-mode` - Start dev-mode orchestration via the CLI

### Git 4D Visualization Commands

- `/git4d` - Launch Git 4D visualization with VR/AR support
- `/vr` - Launch Git 4D visualization in VR mode
- `/ar` - Launch Git 4D visualization in AR mode

## Implementation Details

### Slash Command Architecture

The slash command system is implemented in `codex-rs/tui/src/slash_command.rs`:

```rust
pub enum SlashCommand {
    // ... standard commands ...
    Plan,
    Collab,
    // ... custom commands ...
    Qc,
    DevMode,
    Git4d,
    Vr,
    Ar,
}
```

### Plan Mode Integration

Plan mode is integrated with the collaboration modes system:

1. **Mode Detection**: The `/plan` command checks if collaboration modes are enabled
2. **Mode Switching**: Sets the appropriate collaboration mask for plan mode
3. **Feature Gating**: The plan command is hidden when collaboration modes are disabled

### Conflict Resolution Strategy

When merging upstream changes, custom features are preserved using:

1. **Slash Command Preservation**: Custom commands are detected and added to the enum
2. **Description Mapping**: Custom command descriptions are mapped and inserted
3. **Feature Gating**: Custom commands maintain their availability rules

## Merge Process

### Upstream Merge

```bash
# Fetch upstream changes
git fetch upstream

# Merge with custom feature preservation
git merge upstream/main --no-edit

# Resolve conflicts if any
python3 advanced_merge_resolver.py
```

### Custom Feature Detection

The merge resolver identifies custom features by:

1. Scanning `slash_command.rs` for non-standard commands
2. Checking environment scripts in `zapabob/scripts/`
3. Verifying custom module implementations

## Testing

To verify the implementation:

```bash
# Build the project
cd codex-rs && cargo build

# Run tests
cargo test -p codex-tui

# Verify slash commands
/codex --help | grep -E "^/"
```

## Known Issues

1. VR/AR modes require WebXR-compatible browsers
2. Git 4D visualization may have performance issues on large repositories
3. QC analysis is resource-intensive for large codebases

## Future Improvements

1. Add more visualization modes
2. Implement real-time collaboration
3. Add plan templates and examples
4. Integrate with external planning tools
