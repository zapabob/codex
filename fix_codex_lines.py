#!/usr/bin/env python3
"""
Codex.rsの特定の行を修正
"""

# Read the file
with open('codex-rs/core/src/codex.rs', 'r', encoding='utf-8') as f:
    lines = f.readlines()

# Fix line 3502-3507: Remove conflict markers and keep upstream version
lines[3501] = "            mcp_startup_cancellation_token: CancellationToken::new(),\n"
lines[3502] = "            unified_exec_manager: UnifiedExecProcessManager::default(),\n"
lines[3503] = "            notifier: UserNotifier::new(None),\n"
# Remove lines 3504-3507 (conflict markers)
del lines[3504:3508]  # This will remove the old lines

# Fix line 3601-3606: Remove conflict markers and keep upstream version
# Adjust line numbers after previous deletion
lines[3599] = "            mcp_startup_cancellation_token: CancellationToken::new(),\n"
lines[3600] = "            unified_exec_manager: UnifiedExecProcessManager::default(),\n"
lines[3601] = "            notifier: UserNotifier::new(None),\n"
# Remove lines 3602-3605 (conflict markers)
del lines[3602:3606]

# Write back
with open('codex-rs/core/src/codex.rs', 'w', encoding='utf-8') as f:
    f.writelines(lines)

print("Fixed specific lines in codex.rs")