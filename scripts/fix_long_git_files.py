#!/usr/bin/env python3
"""Remove files with names > 255 bytes from git index and tree."""

import subprocess
import sys
import os

REPO = r"C:\Users\downl\Desktop\codex-main"


def get_long_files():
    """Find files with names > 255 bytes in HEAD tree."""
    result = subprocess.run(
        ["git", "ls-tree", "-r", "--name-only", "-z", "HEAD"],
        capture_output=True,
        cwd=REPO,
    )
    files = result.stdout.split(b"\x00")
    return [f for f in files if f and len(f.split(b"/")[-1]) > 255]


def remove_from_index(fpath_bytes):
    """Remove a file from git index using exact bytes."""
    # Try git update-index --remove (lower level, handles binary paths better)
    result = subprocess.run(
        ["git", "update-index", "--remove", "--", fpath_bytes],
        capture_output=True,
        cwd=REPO,
    )
    if result.returncode == 0:
        return True, "update-index --remove"

    # Try git rm --cached
    result2 = subprocess.run(
        ["git", "rm", "--cached", "--", fpath_bytes],
        capture_output=True,
        cwd=REPO,
    )
    if result2.returncode == 0:
        return True, "rm --cached"

    err = result2.stderr.decode("utf-8", "replace")
    return False, f"FAILED: {err[:100]}"


def main():
    print("=== Fix Long Git Filenames ===")
    long_files = get_long_files()
    print(f"Files with name > 255 bytes in HEAD: {len(long_files)}")

    for fpath in long_files:
        name_part = fpath.split(b"/")[-1]
        print(f"\nRemoving ({len(name_part)}b): {fpath[:60]}...")
        success, msg = remove_from_index(fpath)
        status = "OK" if success else "FAIL"
        print(f"  [{status}] {msg}")

    # Verify
    long_after = get_long_files()
    print(f"\nLong files remaining in index: {len(long_after)}")

    if long_after:
        print("WARNING: Some files still remain!")
        for f in long_after:
            print(f"  {f[:80]}")
        sys.exit(1)
    else:
        print("All long files removed from index!")


if __name__ == "__main__":
    main()
