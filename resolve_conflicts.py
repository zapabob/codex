#!/usr/bin/env python3
"""
Auto-resolve git merge conflicts in Rust files.
Strategy: Prefer upstream version but integrate unique HEAD additions.
"""

import re
import os
from pathlib import Path

CONFLICT_PATTERN = re.compile(
    r'<<<<<<< HEAD\r?\n(.*?)=======\r?\n(.*?)>>>>>>> upstream/main\r?\n?',
    re.DOTALL
)

def resolve_conflict(head: str, upstream: str) -> str:
    """
    Resolve a conflict block. Strategy:
    - Prefer upstream as the base
    - If HEAD has unique imports or additions, try to merge them
    """
    head_lines = set(line.strip() for line in head.strip().split('\n') if line.strip())
    upstream_lines = set(line.strip() for line in upstream.strip().split('\n') if line.strip())
    
    # If they're roughly equivalent, use upstream
    if head_lines == upstream_lines:
        return upstream
    
    # For most cases, prefer upstream (newer/cleaner implementation)
    # Special case: if HEAD has imports, merge them
    head_has_only_imports = all(
        l.startswith('use ') or l.startswith('pub use ') or l.startswith('mod ') or l.startswith('pub mod ')
        for l in head_lines if l
    )
    upstream_has_only_imports = all(
        l.startswith('use ') or l.startswith('pub use ') or l.startswith('mod ') or l.startswith('pub mod ')
        for l in upstream_lines if l
    )
    
    if head_has_only_imports and upstream_has_only_imports:
        # Merge imports
        merged = upstream_lines | head_lines
        return '\n'.join(sorted(merged)) + '\n'
    
    # Default: use upstream
    return upstream

def process_file(filepath: Path) -> bool:
    """Process a single file, resolving all conflicts. Returns True if modified."""
    try:
        content = filepath.read_text(encoding='utf-8')
    except Exception as e:
        print(f"  Error reading {filepath}: {e}")
        return False
    
    if '<<<<<<< HEAD' not in content:
        return False
    
    def replacer(match):
        head = match.group(1)
        upstream = match.group(2)
        return resolve_conflict(head, upstream)
    
    new_content = CONFLICT_PATTERN.sub(replacer, content)
    
    if new_content != content:
        filepath.write_text(new_content, encoding='utf-8')
        return True
    return False

def main():
    base_dir = Path(r"c:\Users\downl\Desktop\codex-main\codex-rs")
    
    # Find all .rs files with conflicts
    conflict_files = []
    for rs_file in base_dir.rglob("*.rs"):
        try:
            content = rs_file.read_text(encoding='utf-8')
            if '<<<<<<< HEAD' in content:
                conflict_files.append(rs_file)
        except:
            pass
    
    print(f"Found {len(conflict_files)} files with conflicts")
    
    resolved = 0
    for filepath in conflict_files:
        rel_path = filepath.relative_to(base_dir)
        if process_file(filepath):
            print(f"  Resolved: {rel_path}")
            resolved += 1
        else:
            print(f"  Skipped: {rel_path}")
    
    print(f"\nResolved conflicts in {resolved} files")

if __name__ == "__main__":
    main()
