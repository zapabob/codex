#!/usr/bin/env python3
"""
Specific merge conflict resolver for the current merge conflicts.
"""

import subprocess
import sys
import os
import re


def run_git(args):
    result = subprocess.run(["git"] + args, capture_output=True, text=True)
    return result.returncode == 0, result.stdout, result.stderr


def resolve_chatwidget_conflict():
    """Resolve chatwidget.rs conflict while preserving custom features."""
    file_path = "codex-rs/tui/src/chatwidget.rs"

    if not os.path.exists(file_path):
        print(f"File not found: {file_path}")
        return False

    with open(file_path, "r", encoding="utf-8", errors="replace") as f:
        content = f.read()

    # Custom features to preserve in chatwidget
    custom_features = [
        "SlashCommand::Qc",
        "SlashCommand::DevMode",
        "SlashCommand::Git4d",
        "SlashCommand::Vr",
        "SlashCommand::Ar",
    ]

    # Check if conflict markers exist
    if "<<<<<<< HEAD" not in content:
        print("No conflict markers in chatwidget.rs")
        # Still ensure custom features are present
        for cmd in custom_features:
            if cmd not in content:
                print(f"Warning: {cmd} not found in chatwidget.rs")
        return True

    # Get both versions
    success_ours, ours, _ = run_git(["show", f"HEAD:{file_path}"])
    success_theirs, theirs, _ = run_git(["show", f"upstream/main:{file_path}"])

    if not (success_ours and success_theirs):
        print("Could not read both versions")
        return False

    # Check which version has custom features
    our_custom_count = sum(1 for cmd in custom_features if cmd in ours)
    their_custom_count = sum(1 for cmd in custom_features if cmd in theirs)

    print(f"Local custom features: {our_custom_count}")
    print(f"Upstream custom features: {their_custom_count}")

    # Prefer the version with more custom features
    if our_custom_count >= their_custom_count:
        merged = ours
        print("Using local version (more custom features)")
    else:
        merged = theirs
        print("Using upstream version")

    # Remove conflict markers if present
    merged = re.sub(
        r"<<<<<<< HEAD\n.*?\n=======\n.*?\n>>>>>>> .*?\n", "", merged, flags=re.DOTALL
    )

    # Write merged content
    with open(file_path, "w", encoding="utf-8") as f:
        f.write(merged)

    print(f"Resolved: {file_path}")
    return True


def resolve_cargo_conflict():
    """Resolve Cargo.toml conflict."""
    file_path = "codex-rs/Cargo.toml"

    if not os.path.exists(file_path):
        print(f"File not found: {file_path}")
        return False

    with open(file_path, "r", encoding="utf-8", errors="replace") as f:
        content = f.read()

    if "<<<<<<< HEAD" not in content:
        print("No conflict markers in Cargo.toml")
        return True

    # For Cargo.toml, prefer upstream but keep our workspace members
    success_ours, ours, _ = run_git(["show", f"HEAD:{file_path}"])
    success_theirs, theirs, _ = run_git(["show", f"upstream/main:{file_path}"])

    # Remove conflict markers
    merged = re.sub(
        r"<<<<<<< HEAD\n.*?\n=======\n.*?\n>>>>>>> .*?\n", "", theirs, flags=re.DOTALL
    )

    # Ensure our workspace members are preserved
    workspace_pattern = r"members\s*=\s*\[([^\]]*)\]"
    ours_match = re.search(workspace_pattern, ours)
    theirs_match = re.search(workspace_pattern, theirs)

    if ours_match and theirs_match:
        # Merge workspace members
        ours_members = set(re.findall(r'"[^"]+"', ours_match.group(1)))
        theirs_members = set(re.findall(r'"[^"]+"', theirs_match.group(1)))
        all_members = ours_members.union(theirs_members)

        merged = re.sub(
            workspace_pattern,
            f"members = [{', '.join(sorted(all_members))}]",
            merged,
            flags=re.DOTALL,
        )

    with open(file_path, "w", encoding="utf-8") as f:
        f.write(merged)

    print(f"Resolved: {file_path}")
    return True


def resolve_event_mapping_conflict():
    """Resolve event_mapping.rs delete conflict."""
    file_path = "codex-rs/core/src/event_mapping.rs"

    # Check what we want - upstream modified or local deleted
    success_ours, ours, _ = run_git(["ls-tree", "-r", "HEAD", "--name-only", file_path])
    success_theirs, theirs, _ = run_git(
        ["ls-tree", "-r", "upstream/main", "--name-only", file_path]
    )

    if success_theirs and theirs.strip():
        # Upstream has it, restore it
        success, stdout, stderr = run_git(["checkout", "--ours", file_path])
        if not success:
            # Try using theirs instead
            success, stdout, stderr = run_git(["checkout", "--theirs", file_path])
        print(f"Restored: {file_path}")
    else:
        # Both deleted, but check if ours was intentional
        print(f"File {file_path} deleted in both branches")

    return True


def resolve_lock_conflict():
    """Resolve Cargo.lock conflict."""
    file_path = "codex-rs/Cargo.lock"

    if not os.path.exists(file_path):
        return True

    with open(file_path, "r", encoding="utf-8", errors="replace") as f:
        content = f.read()

    if "<<<<<<< HEAD" not in content:
        print("No conflict markers in Cargo.lock")
        return True

    # For lock files, use upstream version (they update dependencies)
    success, theirs, _ = run_git(["show", f"upstream/main:{file_path}"])

    if success:
        with open(file_path, "w", encoding="utf-8") as f:
            f.write(theirs)
        print(f"Resolved: {file_path}")

    return True


def main():
    print("=" * 60)
    print("Resolving Merge Conflicts")
    print("=" * 60)

    # Resolve each conflict
    conflicts = [
        ("chatwidget.rs", resolve_chatwidget_conflict),
        ("Cargo.toml", resolve_cargo_conflict),
        ("event_mapping.rs", resolve_event_mapping_conflict),
        ("Cargo.lock", resolve_lock_conflict),
    ]

    for filename, resolver in conflicts:
        print(f"\n--- Resolving {filename} ---")
        try:
            resolver()
        except Exception as e:
            print(f"Error resolving {filename}: {e}")

    # Stage resolved files
    print("\n--- Staging resolved files ---")
    run_git(["add", "codex-rs/tui/src/chatwidget.rs"])
    run_git(["add", "codex-rs/Cargo.toml"])
    run_git(["add", "codex-rs/Cargo.lock"])

    # Complete merge
    print("\n--- Completing merge ---")
    success, stdout, stderr = run_git(["merge", "--continue", "--no-edit"])

    if success:
        print("✓ Merge completed successfully!")
        return 0
    else:
        print(f"✗ Merge failed: {stderr}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
