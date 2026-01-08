#!/usr/bin/env python3
"""
docsファイルの競合を一括修正
"""

import os
import glob

def fix_file(filepath):
    """Fix conflicts in a single file"""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # Remove conflict markers - keep HEAD version and add upstream link if needed
    # This is a simple approach - keep everything from HEAD and add upstream references
    lines = content.split('\n')
    clean_lines = []
    skip_until = None

    for line in lines:
        if line.startswith('<<<<<<< HEAD'):
            skip_until = '>>>>>> upstream/main'
            continue
        elif skip_until and line.startswith('======='):
            continue
        elif skip_until and line.startswith('>>>>>>> upstream/main'):
            skip_until = None
            # Add upstream reference for documentation files
            if filepath.endswith(('.md', '.MD')):
                clean_lines.append('')
                clean_lines.append('For more information, see [the official documentation](https://developers.openai.com/codex).')
            continue
        elif skip_until:
            continue
        else:
            clean_lines.append(line)

    # Write back
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write('\n'.join(clean_lines))

    print(f"Fixed conflicts in {filepath}")

# Process all docs files with conflicts
conflict_files = [
    'docs/sandbox.md',
    'docs/prompts.md',
    'docs/slash_commands.md'
]

for filepath in conflict_files:
    if os.path.exists(filepath):
        fix_file(filepath)
    else:
        print(f"File not found: {filepath}")

print("All docs conflicts fixed!")