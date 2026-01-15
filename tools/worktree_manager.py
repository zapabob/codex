#!/usr/bin/env python3
"""
Git Worktree Manager - Parallel Development Environment Manager
Manages multiple git worktrees for parallel development with QA integration
"""

import os
import sys
import json
import subprocess
import shutil
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple
from dataclasses import dataclass, asdict
from enum import Enum
import logging
import time
from datetime import datetime

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class WorktreeStatus(Enum):
    CREATING = "creating"
    ACTIVE = "active"
    INACTIVE = "inactive"
    QA_PENDING = "qa_pending"
    QA_RUNNING = "qa_running"
    QA_PASSED = "qa_passed"
    QA_FAILED = "qa_failed"
    READY_MERGE = "ready_merge"
    MERGED = "merged"
    ABANDONED = "abandoned"

class WorktreeType(Enum):
    FEATURE = "feature"
    BUGFIX = "bugfix"
    REFACTOR = "refactor"
    EXPERIMENT = "experiment"
    QA_REVIEW = "qa_review"

@dataclass
class WorktreeInfo:
    name: str
    branch: str
    path: Path
    type: WorktreeType
    status: WorktreeStatus
    created_at: datetime
    updated_at: datetime
    description: str
    parent_commit: str
    qa_report_path: Optional[Path] = None
    terminal_pid: Optional[int] = None
    metadata: Dict[str, Any] = None

    def __post_init__(self):
        if self.metadata is None:
            self.metadata = {}

@dataclass
class WorktreeConfig:
    base_path: Path
    max_concurrent: int = 5
    auto_qa: bool = True
    qa_timeout: int = 300  # 5 minutes
    cleanup_inactive: bool = True
    cleanup_days: int = 7

class GitWorktreeManager:
    """Git worktree manager for parallel development"""

    def __init__(self, config: WorktreeConfig):
        self.config = config
        self.worktrees: Dict[str, WorktreeInfo] = {}
        self.worktrees_file = config.base_path / ".worktrees.json"
        self.config.base_path.mkdir(parents=True, exist_ok=True)
        self.load_worktrees()

    def load_worktrees(self):
        """Load worktree information from storage"""
        if self.worktrees_file.exists():
            try:
                with open(self.worktrees_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                    for name, info in data.items():
                        # Convert datetime strings back to datetime objects
                        info['created_at'] = datetime.fromisoformat(info['created_at'])
                        info['updated_at'] = datetime.fromisoformat(info['updated_at'])
                        # Convert path strings back to Path objects
                        info['path'] = Path(info['path'])
                        if info.get('qa_report_path'):
                            info['qa_report_path'] = Path(info['qa_report_path'])
                        self.worktrees[name] = WorktreeInfo(**info)
            except Exception as e:
                logger.error(f"Failed to load worktrees: {e}")

    def save_worktrees(self):
        """Save worktree information to storage"""
        try:
            data = {}
            for name, info in self.worktrees.items():
                info_dict = asdict(info)
                # Convert Path objects to strings
                info_dict['path'] = str(info_dict['path'])
                if info_dict.get('qa_report_path'):
                    info_dict['qa_report_path'] = str(info_dict['qa_report_path'])
                data[name] = info_dict

            with open(self.worktrees_file, 'w', encoding='utf-8') as f:
                json.dump(data, f, indent=2, ensure_ascii=False, default=str)
        except Exception as e:
            logger.error(f"Failed to save worktrees: {e}")

    def create_worktree(self, name: str, branch: str, worktree_type: WorktreeType,
                       description: str = "", base_branch: str = "main") -> Optional[WorktreeInfo]:
        """Create a new git worktree"""

        # Check limits
        active_count = sum(1 for wt in self.worktrees.values()
                          if wt.status in [WorktreeStatus.ACTIVE, WorktreeStatus.CREATING])
        if active_count >= self.config.max_concurrent:
            logger.error(f"Maximum concurrent worktrees ({self.config.max_concurrent}) reached")
            return None

        # Check if worktree already exists
        if name in self.worktrees:
            logger.error(f"Worktree '{name}' already exists")
            return None

        try:
            # Get current commit hash from base branch
            result = subprocess.run(
                ["git", "rev-parse", f"origin/{base_branch}"],
                capture_output=True, text=True, cwd=self.config.base_path
            )
            if result.returncode != 0:
                logger.error(f"Failed to get commit hash: {result.stderr}")
                return None

            parent_commit = result.stdout.strip()

            # Create worktree path
            worktree_path = self.config.base_path / name

            # Create git worktree
            logger.info(f"Creating worktree '{name}' at {worktree_path}")
            result = subprocess.run(
                ["git", "worktree", "add", "-b", branch, str(worktree_path)],
                capture_output=True, text=True, cwd=self.config.base_path
            )

            if result.returncode != 0:
                logger.error(f"Failed to create worktree: {result.stderr}")
                return None

            # Create worktree info
            now = datetime.now()
            worktree_info = WorktreeInfo(
                name=name,
                branch=branch,
                path=worktree_path,
                type=worktree_type,
                status=WorktreeStatus.ACTIVE,
                created_at=now,
                updated_at=now,
                description=description,
                parent_commit=parent_commit
            )

            self.worktrees[name] = worktree_info
            self.save_worktrees()

            logger.info(f"Worktree '{name}' created successfully")
            return worktree_info

        except Exception as e:
            logger.error(f"Failed to create worktree '{name}': {e}")
            return None

    def launch_terminal(self, worktree_name: str) -> bool:
        """Launch a new terminal in the worktree directory"""

        if worktree_name not in self.worktrees:
            logger.error(f"Worktree '{worktree_name}' not found")
            return False

        worktree_info = self.worktrees[worktree_name]

        try:
            if os.name == 'nt':  # Windows
                # Launch Windows Terminal or cmd
                cmd = f'start cmd /k "cd /d {worktree_info.path} && title Worktree: {worktree_name}"'
                process = subprocess.Popen(cmd, shell=True)
            else:  # Unix-like systems
                # Try to launch terminal
                terminals = ['gnome-terminal', 'konsole', 'xterm', 'terminal']
                launched = False

                for terminal in terminals:
                    try:
                        process = subprocess.Popen([
                            terminal, '--working-directory', str(worktree_info.path),
                            '-e', f'bash -c "echo Worktree: {worktree_name}; bash"'
                        ])
                        launched = True
                        break
                    except FileNotFoundError:
                        continue

                if not launched:
                    # Fallback to background process
                    process = subprocess.Popen(['bash'], cwd=worktree_info.path)

            worktree_info.terminal_pid = process.pid
            worktree_info.updated_at = datetime.now()
            self.save_worktrees()

            logger.info(f"Terminal launched for worktree '{worktree_name}' (PID: {process.pid})")
            return True

        except Exception as e:
            logger.error(f"Failed to launch terminal for '{worktree_name}': {e}")
            return False

    def run_qa_analysis(self, worktree_name: str) -> bool:
        """Run QA analysis on a worktree"""

        if worktree_name not in self.worktrees:
            logger.error(f"Worktree '{worktree_name}' not found")
            return False

        worktree_info = self.worktrees[worktree_name]

        try:
            # Update status
            worktree_info.status = WorktreeStatus.QA_RUNNING
            worktree_info.updated_at = datetime.now()
            self.save_worktrees()

            logger.info(f"Running QA analysis on worktree '{worktree_name}'")

            # Run QA analysis script
            qa_script = Path(__file__).parent / "codex-supervisor" / "supervisor.py"

            if not qa_script.exists():
                # Fallback to direct QA skill execution
                qa_script = worktree_info.path / ".codex" / "skills" / "qa-engineer" / "scripts" / "run_qa-engineer.py"

            if not qa_script.exists():
                logger.error("QA script not found")
                worktree_info.status = WorktreeStatus.QA_FAILED
                self.save_worktrees()
                return False

            # Run QA analysis
            result = subprocess.run(
                [sys.executable, str(qa_script)],
                cwd=worktree_info.path,
                capture_output=True,
                text=True,
                timeout=self.config.qa_timeout
            )

            # Check artifacts for QA report
            artifacts_dir = worktree_info.path / "artifacts"
            qa_report = artifacts_dir / "qa_report.json"

            if result.returncode == 0 and qa_report.exists():
                worktree_info.status = WorktreeStatus.QA_PASSED
                worktree_info.qa_report_path = qa_report
                logger.info(f"QA analysis passed for worktree '{worktree_name}'")

                # Check if merge is ready
                if self._can_merge(worktree_info):
                    worktree_info.status = WorktreeStatus.READY_MERGE

            else:
                worktree_info.status = WorktreeStatus.QA_FAILED
                logger.warning(f"QA analysis failed for worktree '{worktree_name}'")

            worktree_info.updated_at = datetime.now()
            self.save_worktrees()

            return worktree_info.status == WorktreeStatus.QA_PASSED

        except subprocess.TimeoutExpired:
            logger.error(f"QA analysis timed out for worktree '{worktree_name}'")
            worktree_info.status = WorktreeStatus.QA_FAILED
            worktree_info.updated_at = datetime.now()
            self.save_worktrees()
            return False
        except Exception as e:
            logger.error(f"QA analysis error for worktree '{worktree_name}': {e}")
            worktree_info.status = WorktreeStatus.QA_FAILED
            worktree_info.updated_at = datetime.now()
            self.save_worktrees()
            return False

    def _can_merge(self, worktree_info: WorktreeInfo) -> bool:
        """Check if worktree can be merged based on QA results"""

        if not worktree_info.qa_report_path or not worktree_info.qa_report_path.exists():
            return False

        try:
            with open(worktree_info.qa_report_path, 'r', encoding='utf-8') as f:
                qa_report = json.load(f)

            integration_status = qa_report.get('integration_status', {})
            return integration_status.get('can_merge', False)

        except Exception as e:
            logger.error(f"Failed to check merge status: {e}")
            return False

    def merge_worktree(self, worktree_name: str, target_branch: str = "main") -> bool:
        """Merge worktree back to main branch"""

        if worktree_name not in self.worktrees:
            logger.error(f"Worktree '{worktree_name}' not found")
            return False

        worktree_info = self.worktrees[worktree_name]

        # Check if ready for merge
        if worktree_info.status != WorktreeStatus.READY_MERGE:
            logger.error(f"Worktree '{worktree_name}' is not ready for merge (status: {worktree_info.status.value})")
            return False

        try:
            logger.info(f"Merging worktree '{worktree_name}' to {target_branch}")

            # Switch to target branch and merge
            result = subprocess.run(
                ["git", "checkout", target_branch],
                capture_output=True, text=True, cwd=self.config.base_path
            )

            if result.returncode != 0:
                logger.error(f"Failed to checkout {target_branch}: {result.stderr}")
                return False

            # Merge the worktree branch
            result = subprocess.run(
                ["git", "merge", worktree_info.branch, "--no-ff", "-m", f"Merge {worktree_info.type.value}: {worktree_info.description}"],
                capture_output=True, text=True, cwd=self.config.base_path
            )

            if result.returncode != 0:
                logger.error(f"Failed to merge {worktree_info.branch}: {result.stderr}")
                return False

            # Update worktree status
            worktree_info.status = WorktreeStatus.MERGED
            worktree_info.updated_at = datetime.now()
            self.save_worktrees()

            logger.info(f"Successfully merged worktree '{worktree_name}'")
            return True

        except Exception as e:
            logger.error(f"Failed to merge worktree '{worktree_name}': {e}")
            return False

    def cleanup_worktree(self, worktree_name: str) -> bool:
        """Clean up and remove a worktree"""

        if worktree_name not in self.worktrees:
            logger.error(f"Worktree '{worktree_name}' not found")
            return False

        worktree_info = self.worktrees[worktree_name]

        try:
            logger.info(f"Cleaning up worktree '{worktree_name}'")

            # Kill associated terminal if running
            if worktree_info.terminal_pid:
                try:
                    os.kill(worktree_info.terminal_pid, 9)  # SIGKILL
                    logger.info(f"Killed terminal process {worktree_info.terminal_pid}")
                except ProcessLookupError:
                    pass  # Process already dead

            # Remove git worktree
            result = subprocess.run(
                ["git", "worktree", "remove", worktree_name],
                capture_output=True, text=True, cwd=self.config.base_path
            )

            if result.returncode != 0:
                logger.warning(f"Failed to remove git worktree: {result.stderr}")

            # Remove directory if it still exists
            if worktree_info.path.exists():
                shutil.rmtree(worktree_info.path)
                logger.info(f"Removed worktree directory: {worktree_info.path}")

            # Remove from tracking
            del self.worktrees[worktree_name]
            self.save_worktrees()

            logger.info(f"Worktree '{worktree_name}' cleaned up successfully")
            return True

        except Exception as e:
            logger.error(f"Failed to cleanup worktree '{worktree_name}': {e}")
            return False

    def list_worktrees(self) -> List[WorktreeInfo]:
        """List all worktrees"""
        return list(self.worktrees.values())

    def get_worktree_status(self, worktree_name: str) -> Optional[WorktreeStatus]:
        """Get status of a specific worktree"""
        worktree = self.worktrees.get(worktree_name)
        return worktree.status if worktree else None

    def cleanup_inactive_worktrees(self):
        """Clean up old inactive worktrees"""
        if not self.config.cleanup_inactive:
            return

        cutoff_date = datetime.now().timestamp() - (self.config.cleanup_days * 24 * 60 * 60)
        to_cleanup = []

        for name, worktree in self.worktrees.items():
            if (worktree.status in [WorktreeStatus.INACTIVE, WorktreeStatus.ABANDONED] and
                worktree.updated_at.timestamp() < cutoff_date):
                to_cleanup.append(name)

        for name in to_cleanup:
            logger.info(f"Auto-cleaning up inactive worktree '{name}'")
            self.cleanup_worktree(name)

def main():
    """Main entry point for worktree manager"""

    import argparse

    parser = argparse.ArgumentParser(description="Git Worktree Manager for Parallel Development")
    parser.add_argument("action", choices=[
        "create", "launch", "qa", "merge", "cleanup", "list", "status"
    ], help="Action to perform")
    parser.add_argument("name", nargs="?", help="Worktree name")
    parser.add_argument("--branch", help="Branch name for new worktree")
    parser.add_argument("--type", choices=[t.value for t in WorktreeType],
                       default="feature", help="Worktree type")
    parser.add_argument("--description", default="", help="Worktree description")
    parser.add_argument("--base-path", default="./worktrees",
                       help="Base path for worktrees")
    parser.add_argument("--max-concurrent", type=int, default=5,
                       help="Maximum concurrent worktrees")

    args = parser.parse_args()

    # Initialize manager
    config = WorktreeConfig(
        base_path=Path(args.base_path),
        max_concurrent=args.max_concurrent
    )
    manager = GitWorktreeManager(config)

    # Execute action
    if args.action == "create":
        if not args.name or not args.branch:
            print("Error: name and branch required for create action")
            sys.exit(1)

        worktree_type = WorktreeType(args.type)
        worktree = manager.create_worktree(
            args.name, args.branch, worktree_type, args.description
        )

        if worktree:
            print(f"✅ Created worktree '{args.name}' at {worktree.path}")
            print(f"   Branch: {worktree.branch}")
            print(f"   Type: {worktree.type.value}")
        else:
            print("❌ Failed to create worktree")
            sys.exit(1)

    elif args.action == "launch":
        if not args.name:
            print("Error: name required for launch action")
            sys.exit(1)

        if manager.launch_terminal(args.name):
            print(f"✅ Launched terminal for worktree '{args.name}'")
        else:
            print("❌ Failed to launch terminal")
            sys.exit(1)

    elif args.action == "qa":
        if not args.name:
            print("Error: name required for qa action")
            sys.exit(1)

        if manager.run_qa_analysis(args.name):
            print(f"✅ QA analysis passed for worktree '{args.name}'")
        else:
            print(f"❌ QA analysis failed for worktree '{args.name}'")
            sys.exit(1)

    elif args.action == "merge":
        if not args.name:
            print("Error: name required for merge action")
            sys.exit(1)

        if manager.merge_worktree(args.name):
            print(f"✅ Merged worktree '{args.name}'")
        else:
            print("❌ Failed to merge worktree")
            sys.exit(1)

    elif args.action == "cleanup":
        if args.name:
            if manager.cleanup_worktree(args.name):
                print(f"✅ Cleaned up worktree '{args.name}'")
            else:
                print("❌ Failed to cleanup worktree")
                sys.exit(1)
        else:
            manager.cleanup_inactive_worktrees()
            print("✅ Cleaned up inactive worktrees")

    elif args.action == "list":
        worktrees = manager.list_worktrees()
        if worktrees:
            print("📁 Active Worktrees:")
            print("-" * 80)
            for wt in worktrees:
                print(f"{wt.name:20} {wt.branch:15} {wt.type.value:12} {wt.status.value:12} {wt.updated_at.strftime('%Y-%m-%d %H:%M')}")
        else:
            print("No worktrees found")

    elif args.action == "status":
        if not args.name:
            print("Error: name required for status action")
            sys.exit(1)

        status = manager.get_worktree_status(args.name)
        if status:
            print(f"Status of '{args.name}': {status.value}")
        else:
            print(f"Worktree '{args.name}' not found")
            sys.exit(1)

if __name__ == "__main__":
    main()