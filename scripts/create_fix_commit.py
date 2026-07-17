#!/usr/bin/env python3
"""
Create a new forward commit that removes long-named files from the git tree.
Does NOT rewrite history (no force push needed).
"""

import subprocess
import sys

REPO = r"C:\Users\downl\Desktop\codex-main"


def run_git(args, input_bytes=None):
    result = subprocess.run(
        ["git"] + args,
        capture_output=True,
        cwd=REPO,
        input=input_bytes,
    )
    return result.returncode, result.stdout, result.stderr


def ls_tree_directory(tree_ref, path=""):
    if path:
        ref = f"{tree_ref}:{path}"
    else:
        ref = tree_ref
    rc, stdout, _ = run_git(["ls-tree", ref])
    if rc != 0:
        return []
    entries = []
    for line in stdout.strip().split(b"\n"):
        if not line:
            continue
        parts = line.split(b"\t", 1)
        if len(parts) != 2:
            continue
        meta, name = parts
        mode, obj_type, obj_hash = meta.split(b" ")
        entries.append((mode, obj_type, obj_hash, name))
    return entries


def create_tree(entries):
    mktree_input = b""
    for mode, obj_type, obj_hash, name in entries:
        mktree_input += mode + b" " + obj_type + b" " + obj_hash + b"\t" + name + b"\n"
    rc, stdout, stderr = run_git(["mktree"], input_bytes=mktree_input)
    if rc != 0:
        print(f"mktree failed: {stderr.decode('utf-8', 'replace')}")
        return None
    return stdout.strip()


def get_long_files_in_head():
    rc, stdout, _ = run_git(["ls-tree", "-r", "--name-only", "-z", "HEAD"])
    files = stdout.split(b"\x00")
    return [f for f in files if f and len(f.split(b"/")[-1]) > 255]


def main():
    print("=== Create Fix Commit for Long Filenames ===\n")

    # Check for long files
    long_files = get_long_files_in_head()
    print(f"Long files in HEAD: {len(long_files)}")
    for f in long_files:
        print(f"  ({len(f.split(b'/')[-1])}b): {f[:60]}...")

    if not long_files:
        print("No long files found. Nothing to do.")
        return 0

    # Get HEAD commit SHA
    rc, head_sha, _ = run_git(["rev-parse", "HEAD"])
    head_sha = head_sha.strip()
    print(f"\nCurrent HEAD: {head_sha.decode()[:12]}")

    # Step 1: Fix the .specstory/history tree
    print("\nStep 1: Fix .specstory/history tree")
    history_entries = ls_tree_directory("HEAD", ".specstory/history")
    print(f"  Entries in .specstory/history: {len(history_entries)}")

    filtered_history = []
    removed = []
    for entry in history_entries:
        mode, obj_type, obj_hash, name = entry
        if len(name) > 255:
            removed.append(name)
            print(f"  REMOVING ({len(name)}b): {name[:50]}...")
        else:
            filtered_history.append(entry)

    new_history_sha = create_tree(filtered_history)
    if not new_history_sha:
        return 1
    print(f"  New history tree: {new_history_sha.decode()[:8]}")

    # Step 2: Fix .specstory tree
    print("\nStep 2: Fix .specstory tree")
    specstory_entries = ls_tree_directory("HEAD", ".specstory")
    new_specstory_entries = []
    for mode, obj_type, obj_hash, name in specstory_entries:
        if name == b"history" and obj_type == b"tree":
            new_specstory_entries.append((mode, obj_type, new_history_sha, name))
            print(f"  Updated 'history' -> {new_history_sha.decode()[:8]}")
        else:
            new_specstory_entries.append((mode, obj_type, obj_hash, name))

    new_specstory_sha = create_tree(new_specstory_entries)
    if not new_specstory_sha:
        return 1
    print(f"  New .specstory tree: {new_specstory_sha.decode()[:8]}")

    # Step 3: Fix root tree
    print("\nStep 3: Fix root tree")
    root_entries = ls_tree_directory("HEAD")
    new_root_entries = []
    for mode, obj_type, obj_hash, name in root_entries:
        if name == b".specstory" and obj_type == b"tree":
            new_root_entries.append((mode, obj_type, new_specstory_sha, name))
            print(f"  Updated '.specstory' -> {new_specstory_sha.decode()[:8]}")
        else:
            new_root_entries.append((mode, obj_type, obj_hash, name))

    new_root_sha = create_tree(new_root_entries)
    if not new_root_sha:
        return 1
    print(f"  New root tree: {new_root_sha.decode()[:8]}")

    # Step 4: Create new commit (CHILD of HEAD, not replacement)
    print("\nStep 4: Create new commit")
    commit_msg = (
        "fix(ci): remove filename exceeding Linux 255-byte limit\n\n"
        "The file .specstory/history/2026-01-28_17-09Z-@公式統合...\n"
        "has a 477-byte UTF-8 encoded filename which exceeds Linux's\n"
        "255-byte limit. This causes 'git checkout' to fail on all\n"
        "Linux-based GitHub Actions runners.\n\n"
        "Using git plumbing (mktree + commit-tree) to create a new\n"
        "tree object without the problematic filename."
    )

    rc, stdout, stderr = run_git(
        [
            "commit-tree",
            new_root_sha.decode(),
            "-p",
            head_sha.decode(),
            "-m",
            commit_msg,
        ],
    )

    if rc != 0:
        print(f"commit-tree failed: {stderr.decode('utf-8', 'replace')}")
        return 1

    new_commit_sha = stdout.strip()
    print(f"  New commit: {new_commit_sha.decode()[:12]}")

    # Step 5: Update HEAD to new commit (not rewriting - it's a child)
    print("\nStep 5: Update HEAD to new commit")
    rc, _, stderr = run_git(["reset", "--soft", new_commit_sha.decode()])
    if rc != 0:
        print(f"reset --soft failed: {stderr.decode('utf-8', 'replace')}")
        return 1
    print("  HEAD updated successfully")

    # Step 6: Verify
    print("\nStep 6: Verify")
    long_after = get_long_files_in_head()
    print(f"  Long files in new HEAD: {len(long_after)}")

    rc, head_after, _ = run_git(["rev-parse", "HEAD"])
    print(f"  New HEAD: {head_after.strip().decode()[:12]}")

    rc, log, _ = run_git(["log", "--oneline", "-3"])
    print(f"\nGit log:")
    for line in log.decode().strip().split("\n"):
        print(f"  {line}")

    if long_after:
        print("\nWARNING: Still have long files!")
        return 1
    else:
        print("\nSUCCESS! Ready to push normally (no force push needed)")
        return 0


if __name__ == "__main__":
    sys.exit(main())
