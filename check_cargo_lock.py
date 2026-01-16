#!/usr/bin/env python3
"""
Check Cargo.lock file validity
"""

import sys
import os

def check_cargo_lock():
    cargo_lock_path = "codex-rs/Cargo.lock"

    if not os.path.exists(cargo_lock_path):
        print("ERROR: Cargo.lock not found")
        return False

    try:
        # Read the file
        with open(cargo_lock_path, 'r', encoding='utf-8') as f:
            content = f.read()

        print("SUCCESS: Cargo.lock file read successfully")
        print(f"File size: {len(content)} characters")

        # Check for common issues
        lines = content.split('\n')

        # Look for assert_cmd issue
        assert_cmd_lines = [i for i, line in enumerate(lines) if 'assert_cmd' in line]
        if assert_cmd_lines:
            print(f"Found assert_cmd references at lines: {assert_cmd_lines}")
            for line_num in assert_cmd_lines[:3]:
                print(f"  Line {line_num + 1}: {lines[line_num]}")

        # Check if it looks like valid TOML
        if content.startswith('version = 3'):
            print("SUCCESS: File appears to be valid Cargo.lock format")
            return True
        else:
            print("WARNING: File does not start with expected Cargo.lock header")
            return False

    except Exception as e:
        print(f"ERROR: Failed to read Cargo.lock: {e}")
        return False

if __name__ == "__main__":
    success = check_cargo_lock()
    sys.exit(0 if success else 1)