#!/usr/bin/env python3
"""Delete files with names > 255 bytes using rename trick on Windows."""
import os
from pathlib import Path

SPECSTORY = Path(r"C:\Users\downl\Desktop\codex-main\.specstory\history")


def main():
    long_files = [f for f in SPECSTORY.iterdir() if len(f.name.encode("utf-8")) > 255]
    print(f"Found {len(long_files)} long-named files")

    for f in long_files:
        print(f"Attempting to delete: {len(f.name.encode())}b file")

        # Strategy 1: Rename to simple name first
        simple_path = SPECSTORY / "_to_delete_long_name.md"
        try:
            os.rename(str(f), str(simple_path))
            print(f"  Renamed to: {simple_path.name}")
            os.unlink(str(simple_path))
            print("  Deleted successfully!")
            continue
        except OSError as e:
            print(f"  Rename failed: {e}")

        # Strategy 2: Use Windows \\?\ long path prefix
        try:
            long_path = "\\\\?\\" + str(f)
            os.unlink(long_path)
            print(f"  Deleted via long path prefix!")
            continue
        except OSError as e:
            print(f"  Long path deletion failed: {e}")

        # Strategy 3: Use os.scandir to get the actual os.DirEntry
        try:
            with os.scandir(str(SPECSTORY)) as it:
                for entry in it:
                    if len(entry.name.encode("utf-8")) > 255:
                        # Use the raw entry path
                        raw_path = "\\\\?\\" + entry.path
                        os.unlink(raw_path)
                        print(f"  Deleted via scandir + long path!")
                        break
        except OSError as e:
            print(f"  Scandir deletion failed: {e}")

    remaining = [f for f in SPECSTORY.iterdir() if len(f.name.encode("utf-8")) > 255]
    print(f"\nRemaining long files: {len(remaining)}")
    return len(remaining) == 0


if __name__ == "__main__":
    success = main()
    exit(0 if success else 1)
