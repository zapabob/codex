#!/usr/bin/env python3
"""
codex-rs/tui/src/status/card.rsの競合を修正
"""

# Read the file
with open('codex-rs/tui/src/status/card.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Remove all conflict markers - keep upstream version for most conflicts
# This file has many small conflicts, we'll keep the upstream versions

def resolve_conflicts(text):
    import re

    # Pattern to match conflict blocks
    pattern = r'<<<<<<< HEAD.*?(?=<<<<<<< HEAD|$)(.*?)>>>>>>> upstream/main'
    conflicts = re.findall(pattern, text, re.DOTALL)

    # For this file, we'll keep upstream versions (simpler approach)
    for conflict in conflicts:
        # Extract upstream part (after =======)
        lines = conflict.split('\n')
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

            upstream_content = '\n'.join(upstream_lines)
            # Replace the entire conflict block with upstream content
            text = text.replace('<<<<<<< HEAD' + conflict + '>>>>>>> upstream/main', upstream_content)

    return text

content = resolve_conflicts(content)

# Write back
with open('codex-rs/tui/src/status/card.rs', 'w', encoding='utf-8') as f:
    f.write(content)

print("Fixed conflicts in tui status card.rs")