#!/usr/bin/env python3
"""
Codex Repository Merge Tool
Merges OpenAI/codex while preserving custom features (especially codex-gui-x)
"""

import subprocess
import os
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent
CUSTOM_FILES = [
    # Our custom GUI files that must be preserved
    "codex-gui-x/src/components/chat/",
    "codex-gui-x/src/hooks/",
    "codex-gui-x/src/services/",
    "codex-gui-x/src/store/",
    "codex-gui-x/src/types/mcp.ts",
    # Documentation
    "AGENT.md",
    "REQUIREMENTS.md",
]


def run_cmd(cmd, cwd=None, check=True):
    """Run a command and return the result"""
    print(f"Running: {cmd}")
    result = subprocess.run(
        cmd, shell=True, cwd=cwd or REPO_ROOT, capture_output=True, text=True
    )
    if check and result.returncode != 0:
        print(f"Error: {result.stderr}")
        return False, result.stderr
    return True, result.stdout


def backup_custom_files():
    """Backup custom files before merge"""
    print("\n=== Backing up custom files ===")
    backup_dir = REPO_ROOT / ".backup_custom"
    backup_dir.mkdir(exist_ok=True)

    for pattern in CUSTOM_FILES:
        path = REPO_ROOT / pattern
        if path.exists():
            dest = backup_dir / pattern.rstrip("/")
            if path.is_dir():
                dest.mkdir(parents=True, exist_ok=True)
                run_cmd(f'cp -r "{path}" "{dest}"')
            else:
                dest.parent.mkdir(parents=True, exist_ok=True)
                run_cmd(f'cp "{path}" "{dest}"')
            print(f"  Backed up: {pattern}")

    return backup_dir


def restore_custom_files(backup_dir):
    """Restore custom files after merge"""
    print("\n=== Restoring custom files ===")

    for pattern in CUSTOM_FILES:
        src = backup_dir / pattern.rstrip("/")
        dest = REPO_ROOT / pattern

        if src.exists():
            if dest.exists():
                run_cmd(f'rm -rf "{dest}"')

            parent = dest.parent
            parent.mkdir(parents=True, exist_ok=True)

            if src.is_dir():
                run_cmd(f'cp -r "{src}" "{dest}"')
            else:
                run_cmd(f'cp "{src}" "{dest}"')
            print(f"  Restored: {pattern}")
        else:
            print(f"  Warning: {pattern} not found in backup")


def merge_with_upstream():
    """Perform the merge with upstream"""
    print("\n=== Merging with upstream/main ===")

    # First, let's try a strategy merge
    # Create a merge strategy that prefers our changes for certain files

    # Stage 1: Merge but keep our custom files
    success, _ = run_cmd("git merge --no-commit -X ours upstream/main", check=False)

    if not success:
        print("Merge with -X ours failed, checking conflicts...")
        # List conflicts
        run_cmd("git diff --name-only --diff-filter=U")

    return True


def resolve_conflicts():
    """Resolve remaining conflicts preferring our custom files"""
    print("\n=== Resolving conflicts ===")

    # For codex-gui-x files, prefer ours
    for root, dirs, files in os.walk(REPO_ROOT / "codex-gui-x"):
        for f in files:
            fpath = Path(root) / f
            rel = fpath.relative_to(REPO_ROOT)

            # Check if file has conflict markers
            try:
                content = fpath.read_text(encoding="utf-8")
                if "<<<<<<<" in content:
                    print(f"  Resolving: {rel}")
                    # Keep our version (after =======)
                    parts = content.split("=======\n")
                    if len(parts) >= 2:
                        # Take the part after ======= (our changes)
                        ours = parts[0].replace("<<<<<<< ours\n", "")
                        theirs = (
                            parts[1].replace(">>>>>>> theirs\n", "")
                            if len(parts) > 2
                            else ""
                        )

                        # For codex-gui-x, prefer ours
                        fpath.write_text(ours, encoding="utf-8")
                        run_cmd(f'git add "{fpath}"')
            except Exception as e:
                print(f"  Warning: Could not resolve {rel}: {e}")


def main():
    print("=== Codex Merge Tool ===")
    print("Merging OpenAI/codex while preserving custom features\n")

    # Step 1: Backup custom files
    backup_dir = backup_custom_files()

    # Step 2: Try merge
    merge_with_upstream()

    # Step 3: If merge had issues, restore custom files
    # and merge again more carefully

    # For now, let's restore our custom files
    restore_custom_files(backup_dir)

    print("\n=== Merge Complete ===")
    print("Custom files have been preserved.")
    print("Please review any remaining conflicts manually.")


if __name__ == "__main__":
    main()
