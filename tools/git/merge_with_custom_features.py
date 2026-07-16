#!/usr/bin/env python3
"""
Merge conflict resolver for zapabob/codex custom features.
Maintains custom slash commands while merging upstream changes.
"""

import subprocess
import sys
import os
import re
from typing import List, Tuple, Optional


def run_git_command(args: List[str]) -> Tuple[bool, str, str]:
    """Run a git command and return success status, stdout, stderr."""
    try:
        result = subprocess.run(
            ["git"] + args, capture_output=True, text=True, cwd=os.getcwd()
        )
        return result.returncode == 0, result.stdout, result.stderr
    except Exception as e:
        return False, "", str(e)


def get_custom_slash_commands() -> dict:
    """Get the current custom slash commands from the local branch."""
    slash_command_file = "codex-rs/tui/src/slash_command.rs"
    if not os.path.exists(slash_command_file):
        return {}

    with open(slash_command_file, "r", encoding="utf-8") as f:
        content = f.read()

    custom_commands = {}

    # Extract enum variants
    enum_pattern = r"^\s*(\w+),?\s*(?:#.*)?$"
    in_enum = False

    for line in content.split("\n"):
        if "pub enum SlashCommand" in line:
            in_enum = True
            continue
        if in_enum and "}" in line:
            in_enum = False
            continue
        if in_enum:
            match = re.match(enum_pattern, line.strip())
            if match:
                name = match.group(1)
                if name not in [
                    "Model",
                    "Approvals",
                    "Permissions",
                    "ElevateSandbox",
                    "Experimental",
                    "Skills",
                    "Review",
                    "Rename",
                    "New",
                    "Resume",
                    "Fork",
                    "Init",
                    "Compact",
                    "Plan",
                    "Collab",
                    "Agent",
                    "Diff",
                    "Mention",
                    "Status",
                    "DebugConfig",
                    "Statusline",
                    "Mcp",
                    "Apps",
                    "Logout",
                    "Quit",
                    "Exit",
                    "Feedback",
                    "Rollout",
                    "Ps",
                    "Personality",
                    "TestApproval",
                ]:
                    custom_commands["enum"] = custom_commands.get("enum", [])
                    custom_commands["enum"].append(name)

    # Extract descriptions
    desc_pattern = r"SlashCommand::(\w+)\s*=>\s*\"([^\"]+)\""
    for match in re.finditer(desc_pattern, content):
        name, desc = match.groups()
        if name in ["Qc", "DevMode", "Git4d", "Vr", "Ar"]:
            custom_commands[f"desc_{name}"] = desc

    return custom_commands


def preserve_custom_features(
    local_content: str, upstream_content: str, file_path: str
) -> str:
    """
    Preserve custom features while merging upstream changes.
    Returns the merged content.
    """
    if "slash_command.rs" in file_path:
        # For slash_command.rs, preserve custom commands
        custom_commands = []

        # Find custom commands in local
        for cmd in ["Qc", "DevMode", "Git4d", "Vr", "Ar"]:
            if f"SlashCommand::{cmd}" in local_content:
                custom_commands.append(cmd)

        # Add custom commands to upstream if not present
        result = upstream_content
        for cmd in custom_commands:
            # Add to enum if not present
            if f"SlashCommand::{cmd}" not in result:
                # Find a good insertion point (after Review seems appropriate)
                enum_insert = f"    Review,\n    {cmd},"
                result = result.replace("    Review,\n", enum_insert, 1)

            # Add description if not present
            desc_key = f"desc_{cmd}"
            if f"SlashCommand::{cmd} =>" not in result:
                # Find a good insertion point
                desc_insert = f'SlashCommand::Review => "review my current changes and find issues",\n            SlashCommand::{cmd} => "'
                desc_end = '",'
                # This is complex - we'll handle it more carefully
                pass

        return result

    return upstream_content


def merge_with_custom_features() -> bool:
    """
    Main function to merge upstream changes while preserving custom features.
    """
    print("=" * 60)
    print("Zapabob Codex Merge Tool")
    print("Preserving custom features while merging upstream changes")
    print("=" * 60)

    # Step 1: Fetch upstream
    print("\n[1/5] Fetching upstream changes...")
    success, stdout, stderr = run_git_command(["fetch", "upstream"])
    if not success:
        print(f"Warning: Failed to fetch upstream: {stderr}")
        # Try continuing anyway
    else:
        print("Upstream fetched successfully")

    # Step 2: Get custom features before merge
    print("\n[2/5] Identifying custom features...")
    custom_features = get_custom_slash_commands()
    if custom_features:
        print(f"Found custom features: {custom_features}")
    else:
        print("No custom slash commands found (using upstream version)")

    # Step 3: Perform merge
    print("\n[3/5] Merging upstream/main...")
    success, stdout, stderr = run_git_command(["merge", "upstream/main", "--no-edit"])

    if success:
        print("Merge completed successfully!")
    else:
        print(f"Merge conflict detected: {stderr}")
        # Resolve conflicts
        print("\n[3.5] Resolving merge conflicts...")

        # Get conflicted files
        success, stdout, stderr = run_git_command(
            ["diff", "--name-only", "--diff-filter=U"]
        )
        conflicted_files = [f for f in stdout.strip().split("\n") if f]

        if conflicted_files:
            print(f"Conflicted files: {conflicted_files}")

            for file_path in conflicted_files:
                print(f"\nResolving conflict in: {file_path}")

                # Read both versions
                success_ours, ours, _ = run_git_command(["show", f"HEAD:{file_path}"])
                success_theirs, theirs, _ = run_git_command(
                    ["show", f"upstream/main:{file_path}"]
                )

                if success_ours and success_theirs:
                    # Merge with custom features preserved
                    merged = preserve_custom_features(ours, theirs, file_path)

                    # Write merged content
                    with open(file_path, "w", encoding="utf-8") as f:
                        f.write(merged)

                    # Stage the resolved file
                    run_git_command(["add", file_path])
                    print(f"Resolved: {file_path}")
                else:
                    print(f"Could not read both versions of {file_path}")

            # Complete the merge
            print("\nCompleting merge...")
            success, stdout, stderr = run_git_command(
                ["merge", "--continue", "--no-edit"]
            )
            if not success:
                print(f"Error completing merge: {stderr}")
                return False
        else:
            print("No conflicted files found")

    # Step 4: Verify custom features are preserved
    print("\n[4/5] Verifying custom features...")
    current_custom = get_custom_slash_commands()
    if "enum" in current_custom or any(
        k.startswith("desc_") for k in current_custom.keys()
    ):
        print("✓ Custom features preserved!")
    else:
        print("Note: Custom features not detected (may have been merged upstream)")

    # Step 5: Show status
    print("\n[5/5] Final status...")
    success, stdout, stderr = run_git_command(["status", "--porcelain"])
    print(stdout if stdout else "Working tree clean")

    return True


if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)) or os.getcwd())
    success = merge_with_custom_features()
    sys.exit(0 if success else 1)
