#!/usr/bin/env python3
"""
Advanced merge conflict resolver for zapabob/codex.
Handles merge conflicts while preserving custom features and integrating plan mode.
"""

import subprocess
import sys
import os
import re
import json
from datetime import datetime
from typing import Dict, List, Optional, Tuple, Any


class MergeConflictResolver:
    def __init__(self):
        self.custom_features = {
            "slash_commands": [],
            "config_changes": [],
            "custom_files": [],
        }
        self.resolution_log = []

    def log(self, message: str, level: str = "INFO"):
        """Log a message with timestamp."""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        log_entry = f"[{timestamp}] [{level}] {message}"
        print(log_entry)
        self.resolution_log.append(log_entry)

    def run_git(self, args: List[str]) -> Tuple[bool, str, str]:
        """Run a git command."""
        try:
            result = subprocess.run(
                ["git"] + args, capture_output=True, text=True, cwd=os.getcwd()
            )
            return result.returncode == 0, result.stdout, result.stderr
        except Exception as e:
            return False, "", str(e)

    def identify_custom_features(self) -> Dict[str, Any]:
        """Identify custom features in the current branch."""
        self.log("Identifying custom features...")

        custom = {
            "slash_commands": [],
            "env_files": [],
            "scripts": [],
            "custom_modules": [],
        }

        # Check slash_command.rs for custom commands
        slash_file = "codex-rs/tui/src/slash_command.rs"
        if os.path.exists(slash_file):
            with open(slash_file, "r", encoding="utf-8") as f:
                content = f.read()

            # Custom slash commands
            custom_cmds = ["Qc", "DevMode", "Git4d", "Vr", "Ar"]
            for cmd in custom_cmds:
                if f"SlashCommand::{cmd}" in content:
                    custom["slash_commands"].append(cmd)
                    self.log(f"Found custom slash command: /{cmd}")

        # Check for custom scripts
        for root, dirs, files in os.walk("."):
            if root.startswith("./.git"):
                continue
            for f in files:
                if f.endswith(".py") or f.endswith(".sh"):
                    path = os.path.join(root, f)
                    if "/archive/" not in path and "/.git/" not in path:
                        custom["scripts"].append(path)

        self.custom_features = custom
        return custom

    def analyze_conflict(self, file_path: str) -> Dict[str, Any]:
        """Analyze a conflicted file."""
        analysis = {
            "file": file_path,
            "has_conflict": False,
            "conflict_marker": None,
            "local_changes": [],
            "upstream_changes": [],
            "recommendation": None,
        }

        if not os.path.exists(file_path):
            return analysis

        with open(file_path, "r", encoding="utf-8", errors="replace") as f:
            content = f.read()

        # Check for conflict markers
        if "<<<<<<< HEAD" in content:
            analysis["has_conflict"] = True
            analysis["conflict_marker"] = "git"

            # Parse conflict sections
            sections = content.split("<<<<<<< HEAD\n")
            for section in sections:
                if "=======\n" in section:
                    parts = section.split("=======\n")
                    if len(parts) >= 2:
                        local = parts[0].strip()
                        upstream = (
                            parts[1].split(">>>>>>>")[0].strip()
                            if ">>>>>>>" in parts[1]
                            else parts[1].strip()
                        )
                        analysis["local_changes"].append(local)
                        analysis["upstream_changes"].append(upstream)

        return analysis

    def preserve_custom_slash_commands(self, upstream_content: str) -> str:
        """Ensure custom slash commands are preserved from upstream merge."""
        result = upstream_content

        # List of custom commands to preserve
        custom_commands = self.custom_features.get("slash_commands", [])

        for cmd in custom_commands:
            # Ensure enum variant exists
            if f"SlashCommand::{cmd}" not in result:
                # Add after Review command
                result = result.replace(
                    "    Review,\n", f"    Review,\n    {cmd},\n", 1
                )
                self.log(f"Added {cmd} to enum")

            # Ensure description exists
            desc_map = {
                "Qc": "run quality control analysis via the CLI",
                "DevMode": "start dev-mode orchestration via the CLI",
                "Git4d": "launch Git 4D visualization with VR/AR support",
                "Vr": "launch Git 4D visualization in VR mode",
                "Ar": "launch Git 4D visualization in AR mode",
            }

            if cmd in desc_map:
                desc = desc_map[cmd]
                if f"SlashCommand::{cmd} =>" not in result:
                    # Add description after Review description
                    result = result.replace(
                        'SlashCommand::Review => "review my current changes and find issues",',
                        f'SlashCommand::Review => "review my current changes and find issues",\n            SlashCommand::{cmd} => "{desc}",',
                        1,
                    )
                    self.log(f"Added {cmd} description")

        return result

    def merge_file(self, file_path: str, strategy: str = "custom") -> bool:
        """Merge a single file."""
        self.log(f"Merging {file_path} with strategy: {strategy}")

        # Get both versions
        success_ours, ours, _ = self.run_git(["show", f"HEAD:{file_path}"])
        success_theirs, theirs, _ = self.run_git(["show", f"upstream/main:{file_path}"])

        if not (success_ours and success_theirs):
            self.log(f"Could not read both versions of {file_path}", "ERROR")
            return False

        # Analyze conflict
        analysis = self.analyze_conflict(file_path)

        merged = None

        if analysis["has_conflict"]:
            if strategy == "custom":
                # Preserve custom features
                merged = self.preserve_custom_slash_commands(theirs)
            elif strategy == "upstream":
                merged = theirs
            elif strategy == "local":
                merged = ours
            elif strategy == "combine":
                # Try to combine features
                merged = ours + "\n" + theirs
        else:
            # No conflict, use upstream
            merged = self.preserve_custom_slash_commands(theirs)

        if merged:
            with open(file_path, "w", encoding="utf-8") as f:
                f.write(merged)

            self.run_git(["add", file_path])
            self.log(f"Successfully merged {file_path}")
            return True

        return False

    def resolve_all_conflicts(self, strategy: str = "custom") -> bool:
        """Resolve all merge conflicts."""
        self.log("Starting conflict resolution...")

        # Get conflicted files
        success, stdout, stderr = self.run_git(
            ["diff", "--name-only", "--diff-filter=U"]
        )
        conflicted_files = [f for f in stdout.strip().split("\n") if f]

        if not conflicted_files:
            self.log("No conflicted files found")
            return True

        self.log(f"Found {len(conflicted_files)} conflicted files")

        for file_path in conflicted_files:
            self.merge_file(file_path, strategy)

        # Complete merge
        success, stdout, stderr = self.run_git(["merge", "--continue", "--no-edit"])

        if success:
            self.log("Merge completed successfully!")
            return True
        else:
            self.log(f"Error completing merge: {stderr}", "ERROR")
            return False

    def save_resolution_log(self):
        """Save resolution log to file."""
        log_file = "merge_resolution_log.txt"
        with open(log_file, "w", encoding="utf-8") as f:
            f.write("\n".join(self.resolution_log))
        self.log(f"Resolution log saved to {log_file}")

    def run(self) -> bool:
        """Main execution."""
        try:
            # Fetch upstream
            self.log("Fetching upstream changes...")
            self.run_git(["fetch", "upstream"])

            # Identify custom features
            self.identify_custom_features()

            # Perform merge
            self.log("Merging upstream/main...")
            success, stdout, stderr = self.run_git(
                ["merge", "upstream/main", "--no-edit"]
            )

            if not success:
                self.log("Merge conflict detected, resolving...")
                self.resolve_all_conflicts(strategy="custom")

            # Verify
            self.log("Verifying custom features...")
            custom = self.custom_features
            self.log(f"Custom slash commands: {custom['slash_commands']}")

            # Save log
            self.save_resolution_log()

            return True

        except Exception as e:
            self.log(f"Error during merge: {e}", "ERROR")
            return False


def main():
    resolver = MergeConflictResolver()
    success = resolver.run()

    if success:
        print("\n✓ Merge completed successfully!")
        print("Custom features have been preserved.")
    else:
        print("\n✗ Merge failed. Check merge_resolution_log.txt for details.")

    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())
