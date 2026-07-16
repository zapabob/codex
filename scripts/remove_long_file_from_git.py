#!/usr/bin/env python3
"""
Remove long-named files from git tree using git plumbing commands.
Strategy: Create a new tree without the long-named file, then amend HEAD.
"""
import subprocess
import sys
import os

REPO = r"C:\Users\downl\Desktop\codex-main"


def run_git(args, input_bytes=None):
    """Run a git command and return (returncode, stdout, stderr)."""
    result = subprocess.run(
        ["git"] + args,
        capture_output=True,
        cwd=REPO,
        input=input_bytes,
    )
    return result.returncode, result.stdout, result.stderr


def get_long_files_in_tree(tree_sha=b"HEAD"):
    """Get files with name > 255 bytes from a given tree."""
    rc, stdout, _ = run_git(["ls-tree", "-r", "--name-only", "-z", tree_sha.decode()])
    files = stdout.split(b"\x00")
    return [f for f in files if f and len(f.split(b"/")[-1]) > 255]


def get_subtree_sha(path):
    """Get the SHA of a subtree at the given path."""
    rc, stdout, _ = run_git(["rev-parse", f"HEAD:{path}"])
    if rc == 0:
        return stdout.strip()
    return None


def ls_tree_directory(tree_ref, path=""):
    """List all entries in a tree/subtree."""
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
        # format: mode SP type SP hash TAB name
        parts = line.split(b"\t", 1)
        if len(parts) != 2:
            continue
        meta, name = parts
        mode, obj_type, obj_hash = meta.split(b" ")
        entries.append((mode, obj_type, obj_hash, name))
    return entries


def create_tree_without_long_files(entries):
    """Create a new git tree object from entries, using mktree."""
    mktree_input = b""
    for mode, obj_type, obj_hash, name in entries:
        mktree_input += mode + b" " + obj_type + b" " + obj_hash + b"\t" + name + b"\n"
    
    rc, stdout, stderr = run_git(["mktree"], input_bytes=mktree_input)
    if rc != 0:
        print(f"mktree failed: {stderr.decode('utf-8', 'replace')}")
        return None
    return stdout.strip()


def main():
    print("=== Remove Long Files from Git Tree ===\n")
    
    # Check for long files in HEAD
    long_files = get_long_files_in_tree(b"HEAD")
    print(f"Long files in HEAD tree: {len(long_files)}")
    for f in long_files:
        print(f"  ({len(f.split(b'/')[-1])}b): {f[:60]}...")
    
    if not long_files:
        print("No long files found. Exiting.")
        return 0
    
    # Get the long files' directory paths
    dirs_to_fix = set()
    for f in long_files:
        parts = f.split(b"/")
        if len(parts) > 1:
            dirs_to_fix.add(b"/".join(parts[:-1]))
    
    print(f"\nDirectories to fix: {[d.decode() for d in dirs_to_fix]}")
    
    # For each directory, create a new tree without long files
    new_subtree_shas = {}
    for directory in dirs_to_fix:
        print(f"\nProcessing directory: {directory.decode()}")
        entries = ls_tree_directory("HEAD", directory.decode())
        print(f"  Total entries: {len(entries)}")
        
        # Filter out long-named files
        filtered = []
        removed = []
        for entry in entries:
            mode, obj_type, obj_hash, name = entry
            if len(name) > 255:
                removed.append(name)
                print(f"  REMOVING: ({len(name)}b) {name[:50]}...")
            else:
                filtered.append(entry)
        
        print(f"  Kept: {len(filtered)}, Removed: {len(removed)}")
        
        # Create new tree
        new_tree_sha = create_tree_without_long_files(filtered)
        if new_tree_sha:
            new_subtree_shas[directory] = new_tree_sha
            print(f"  New subtree SHA: {new_tree_sha.decode()}")
        else:
            print(f"  FAILED to create new tree!")
            return 1
    
    # Now update the ROOT tree to use the new subtrees
    print("\nUpdating root tree...")
    root_entries = ls_tree_directory("HEAD")
    print(f"Root entries: {len(root_entries)}")
    
    new_root_entries = []
    for mode, obj_type, obj_hash, name in root_entries:
        # Check if this is a subtree we need to update
        updated = False
        for dir_path, new_sha in new_subtree_shas.items():
            parts = dir_path.split(b"/")
            if len(parts) == 1 and name == parts[0] and obj_type == b"tree":
                # This is a direct subtree
                new_root_entries.append((mode, obj_type, new_sha, name))
                print(f"  Updated subtree '{name.decode()}' -> {new_sha.decode()[:8]}")
                updated = True
                break
        if not updated:
            new_root_entries.append((mode, obj_type, obj_hash, name))
    
    # Handle nested paths (e.g., .specstory/history)
    # We need to update .specstory tree first, then the root
    if any(b"/" in d for d in new_subtree_shas.keys()):
        print("\nHandling nested directories...")
        
        # Group by top-level directory
        top_level_updates = {}
        for dir_path, new_sha in new_subtree_shas.items():
            parts = dir_path.split(b"/")
            top_dir = parts[0]
            remaining = b"/".join(parts[1:]) if len(parts) > 1 else b""
            
            if top_dir not in top_level_updates:
                top_level_updates[top_dir] = {}
            if remaining:
                top_level_updates[top_dir][remaining] = new_sha
        
        # For each top-level dir, rebuild its tree
        for top_dir, sub_updates in top_level_updates.items():
            print(f"\n  Rebuilding '{top_dir.decode()}' tree...")
            top_entries = ls_tree_directory("HEAD", top_dir.decode())
            
            new_top_entries = []
            for mode, obj_type, obj_hash, name in top_entries:
                if name in sub_updates and obj_type == b"tree":
                    new_top_entries.append((mode, obj_type, sub_updates[name], name))
                    print(f"    Updated sub-tree '{name.decode()}'")
                else:
                    new_top_entries.append((mode, obj_type, obj_hash, name))
            
            new_top_sha = create_tree_without_long_files(new_top_entries)
            if not new_top_sha:
                print(f"  FAILED to rebuild '{top_dir.decode()}' tree!")
                return 1
            print(f"  New SHA for '{top_dir.decode()}': {new_top_sha.decode()[:8]}")
            
            # Update root entries to use new top-level tree
            new_root_entries2 = []
            for mode, obj_type, obj_hash, name in new_root_entries:
                if name == top_dir and obj_type == b"tree":
                    new_root_entries2.append((mode, obj_type, new_top_sha, name))
                    print(f"  Updated root entry '{top_dir.decode()}'")
                else:
                    new_root_entries2.append((mode, obj_type, obj_hash, name))
            new_root_entries = new_root_entries2
    
    # Create new root tree
    print("\nCreating new root tree...")
    new_root_sha = create_tree_without_long_files(new_root_entries)
    if not new_root_sha:
        print("FAILED to create new root tree!")
        return 1
    print(f"New root tree SHA: {new_root_sha.decode()}")
    
    # Get current HEAD commit info
    rc, head_sha, _ = run_git(["rev-parse", "HEAD"])
    head_sha = head_sha.strip()
    
    rc, parent_sha, _ = run_git(["rev-parse", "HEAD^"])
    parent_sha = parent_sha.strip()
    
    rc, commit_msg, _ = run_git(["log", "-1", "--format=%B", "HEAD"])
    commit_msg = commit_msg.strip()
    
    # Create new commit with the fixed tree
    print("\nCreating new commit...")
    env = os.environ.copy()
    env["GIT_AUTHOR_NAME"] = "zapabob"
    env["GIT_AUTHOR_EMAIL"] = "zapabob@users.noreply.github.com"
    
    # Use commit-tree to create a new commit
    new_commit_msg = commit_msg + b"\n\n[fix: remove filename exceeding Linux 255-byte limit]"
    rc, stdout, stderr = run_git(
        ["commit-tree", new_root_sha.decode(), "-p", parent_sha.decode(), "-m",
         new_commit_msg.decode("utf-8", "replace")],
    )
    
    if rc != 0:
        print(f"commit-tree failed: {stderr.decode('utf-8', 'replace')}")
        return 1
    
    new_commit_sha = stdout.strip()
    print(f"New commit SHA: {new_commit_sha.decode()}")
    
    # Update HEAD to point to new commit
    print("\nUpdating HEAD...")
    rc, _, stderr = run_git(["reset", "--hard", new_commit_sha.decode()])
    if rc != 0:
        print(f"reset failed: {stderr.decode('utf-8', 'replace')}")
        return 1
    
    # Verify
    print("\nVerifying...")
    long_after = get_long_files_in_tree(b"HEAD")
    print(f"Long files in new HEAD: {len(long_after)}")
    
    if long_after:
        print("WARNING: Still have long files!")
        for f in long_after:
            print(f"  {f[:60]}")
        return 1
    else:
        print("SUCCESS! All long files removed from git tree!")
        return 0


if __name__ == "__main__":
    sys.exit(main())
