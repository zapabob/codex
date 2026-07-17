#!/usr/bin/env python3
"""Repository organizer - moves temp files to proper directories.
Does NOT delete any files. Only moves to organized subdirectories.
"""

import os
import shutil
import sys
from pathlib import Path

ROOT = Path(r"C:\Users\downl\Desktop\codex-main")


def move_file(src: Path, dst_dir: Path, dry_run: bool = False) -> bool:
    """Move a file to a directory, creating it if needed."""
    if not src.exists():
        return False

    dst_dir.mkdir(parents=True, exist_ok=True)
    dst = dst_dir / src.name

    # Don't overwrite existing organized files
    if dst.exists():
        print(f"  SKIP (exists): {src.name} -> {dst_dir.relative_to(ROOT)}/")
        return False

    if dry_run:
        print(f"  WOULD MOVE: {src.name} -> {dst_dir.relative_to(ROOT)}/")
        return True

    shutil.move(str(src), str(dst))
    print(f"  MOVED: {src.name} -> {dst_dir.relative_to(ROOT)}/")
    return True


def organize(dry_run: bool = False):
    logs_build = ROOT / "logs" / "build"
    logs_checks = ROOT / "logs" / "checks"
    logs_tui = ROOT / "logs" / "tui"
    archive_temp = ROOT / "archive" / "temp"

    print(f"=== Repository Organizer (dry_run={dry_run}) ===\n")

    # Build logs
    build_log_patterns = [
        "build_*.txt",
        "build_*.log",
        "gui_build*.txt",
        "gui_build*.log",
        "check_*.log",
        "check_*.txt",
        "workspace_check.txt",
        "nextest_results.txt",
        "test_results.txt",
        "supervisor_errors.txt",
        "supervisor2.txt",
    ]
    print("[DIR] Moving build logs to logs/build/...")
    for pattern in build_log_patterns:
        for f in ROOT.glob(pattern):
            if f.parent == ROOT:
                move_file(f, logs_build, dry_run)

    # TUI check logs
    print("\n[DIR] Moving TUI logs to logs/tui/...")
    tui_patterns = ["tui*.txt", "tui*.log"]
    for pattern in tui_patterns:
        for f in ROOT.glob(pattern):
            if f.parent == ROOT:
                move_file(f, logs_tui, dry_run)

    # Check/error logs
    print("\n[DIR] Moving check logs to logs/checks/...")
    check_patterns = [
        "build_errors*.txt",
        "_tmp_*.log",
        "commit_msg*.txt",
        "commit_warn.txt",
        "conflicts.txt",
        "resolve_conflicts.log",
        "unmerged_files.txt",
        "jsonrpc_refs.txt",
        "skills_list*.txt",
        "serena_*.txt",
        "found_serena*.txt",
        "status.txt",
    ]
    for pattern in check_patterns:
        for f in ROOT.glob(pattern):
            if f.parent == ROOT:
                move_file(f, logs_checks, dry_run)

    # Temp scripts and misc files
    print("\n[DIR] Moving temp files to archive/temp/...")
    temp_patterns = [
        "temp_*.yml",
        "temp_*.rs",
        "temp_*.py",
        "force_local.py",
        "status.py",
        "status_clean.py",
        "replace_string.js",
        "replace_string.py",
        "resolve_conflicts.py",
    ]
    for pattern in temp_patterns:
        for f in ROOT.glob(pattern):
            if f.parent == ROOT:
                move_file(f, archive_temp, dry_run)

    # Misc txt and log files
    print("\n[DIR] Moving misc files to logs/checks/...")
    misc_files = [
        ROOT / "ci.b64",
        ROOT / "integration.b64",
        ROOT / "__pycache__",
    ]
    for f in misc_files:
        if f.is_file() and f.parent == ROOT:
            move_file(f, logs_checks, dry_run)

    print("\n[OK] Organization complete!")


if __name__ == "__main__":
    dry_run = "--dry-run" in sys.argv
    organize(dry_run=dry_run)
