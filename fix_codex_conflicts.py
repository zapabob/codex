#!/usr/bin/env python3
"""
Codex.rs競合解消スクリプト
"""

import re

# Read the file
with open('codex-rs/core/src/codex.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Remove conflict markers and keep upstream version
# Pattern: <<<<<<< HEAD ... ======= ... >>>>>>> upstream/main
# Keep the upstream version (after =======)

def resolve_conflict(match):
    lines = match.group(0).split('\n')
    upstream_start = None
    for i, line in enumerate(lines):
        if line.strip() == '=======':
            upstream_start = i + 1
            break

    if upstream_start is not None:
        upstream_lines = []
        for line in lines[upstream_start:]:
            if line.strip() == '>>>>>>> upstream/main':
                break
            upstream_lines.append(line)
        return '\n'.join(upstream_lines)
    return match.group(0)

# Apply conflict resolution
pattern = r'<<<<<<< HEAD.*?(?=<<<<<<< HEAD|$)(.*?)>>>>>>> upstream/main'
content = re.sub(pattern, resolve_conflict, content, flags=re.DOTALL)

# Write back
with open('codex-rs/core/src/codex.rs', 'w', encoding='utf-8') as f:
    f.write(content)

print("Resolved conflicts in codex.rs")