#!/usr/bin/env python3
"""
Background QA Service - Continuous Quality Assurance Daemon
Monitors file changes and runs automated QA analysis in background
"""

import os
import sys
import time
import json
import signal
import asyncio
import threading
from pathlib import Path
from typing import Dict, List, Any, Optional, Set
from dataclasses import dataclass
from datetime import datetime, timedelta
import logging
import subprocess

# Try to import watchdog for file monitoring
try:
    from watchdog.observers import Observer
    from watchdog.events import FileSystemEventHandler
    WATCHDOG_AVAILABLE = True
except ImportError:
    WATCHDOG_AVAILABLE = False
    print("Warning: watchdog not available. Install with: pip install watchdog")

from qa_engineer.scripts.run_qa_engineer import QAAnalyzer, QAReport

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s',
    handlers=[
        logging.FileHandler('background_qa.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

@dataclass
class QAJob:
    worktree_name: str
    worktree_path: Path
    last_qa_run: Optional[datetime] = None
    pending_changes: Set[Path] = None
    qa_report: Optional[QAReport] = None

    def __post_init__(self):
        if self.pending_changes is None:
            self.pending_changes = set()

@dataclass
class QAServiceConfig:
    watch_paths: List[Path]
    qa_interval: int = 300  # 5 minutes
    debounce_time: float = 2.0  # 2 seconds
    max_concurrent_qa: int = 2
    enable_auto_qa: bool = True
    exclude_patterns: List[str] = None
    include_patterns: List[str] = None

    def __post_init__(self):
        if self.exclude_patterns is None:
            self.exclude_patterns = ['*.log', '*.tmp', '__pycache__', '.git', 'node_modules', 'target']
        if self.include_patterns is None:
            self.include_patterns = ['*.py', '*.rs', '*.js', '*.ts', '*.java', '*.cpp', '*.c']

class FileChangeHandler(FileSystemEventHandler):
    """File system event handler for QA service"""

    def __init__(self, qa_service):
        self.qa_service = qa_service
        self.last_change_time = datetime.now()
        self.pending_changes: Dict[str, Set[Path]] = {}

    def should_process_file(self, filepath: Path) -> bool:
        """Check if file should be processed based on patterns"""

        # Check exclude patterns
        for pattern in self.qa_service.config.exclude_patterns:
            if filepath.match(pattern) or pattern in str(filepath):
                return False

        # Check include patterns
        for pattern in self.qa_service.config.include_patterns:
            if filepath.match(pattern):
                return True

        return False

    def on_modified(self, event):
        """Handle file modification events"""
        if event.is_directory:
            return

        filepath = Path(event.src_path)
        if not self.should_process_file(filepath):
            return

        # Find which worktree this file belongs to
        worktree_name = None
        for name, job in self.qa_service.qa_jobs.items():
            try:
                filepath.relative_to(job.worktree_path)
                worktree_name = name
                break
            except ValueError:
                continue

        if worktree_name:
            if worktree_name not in self.pending_changes:
                self.pending_changes[worktree_name] = set()
            self.pending_changes[worktree_name].add(filepath)
            self.last_change_time = datetime.now()

            logger.debug(f"Detected change in {worktree_name}: {filepath}")

    def on_created(self, event):
        """Handle file creation events"""
        self.on_modified(event)

    def on_deleted(self, event):
        """Handle file deletion events"""
        self.on_modified(event)

class BackgroundQAService:
    """Background QA service that continuously monitors and analyzes code"""

    def __init__(self, config: QAServiceConfig):
        self.config = config
        self.qa_jobs: Dict[str, QAJob] = {}
        self.running = False
        self.observer = None
        self.event_handler = FileChangeHandler(self)
        self.qa_thread = None
        self.stats = {
            'total_qa_runs': 0,
            'successful_qa_runs': 0,
            'failed_qa_runs': 0,
            'files_processed': 0,
            'last_activity': datetime.now()
        }

    def add_worktree(self, name: str, path: Path):
        """Add a worktree to be monitored"""

        if name in self.qa_jobs:
            logger.warning(f"Worktree '{name}' already being monitored")
            return

        self.qa_jobs[name] = QAJob(
            worktree_name=name,
            worktree_path=path
        )

        logger.info(f"Added worktree '{name}' to QA monitoring: {path}")

    def remove_worktree(self, name: str):
        """Remove a worktree from monitoring"""
        if name in self.qa_jobs:
            del self.qa_jobs[name]
            logger.info(f"Removed worktree '{name}' from QA monitoring")

    def start(self):
        """Start the background QA service"""
        if self.running:
            logger.warning("QA service is already running")
            return

        logger.info("Starting Background QA Service...")
        self.running = True

        # Start file system monitoring
        if WATCHDOG_AVAILABLE:
            self.observer = Observer()
            for watch_path in self.config.watch_paths:
                if watch_path.exists():
                    self.observer.schedule(self.event_handler, str(watch_path), recursive=True)
                    logger.info(f"Watching path: {watch_path}")

            self.observer.start()
        else:
            logger.warning("File system monitoring not available (watchdog not installed)")

        # Start QA processing thread
        self.qa_thread = threading.Thread(target=self._qa_worker, daemon=True)
        self.qa_thread.start()

        # Setup signal handlers
        signal.signal(signal.SIGINT, self._signal_handler)
        signal.signal(signal.SIGTERM, self._signal_handler)

        logger.info("Background QA Service started successfully")
        logger.info(f"Monitoring {len(self.qa_jobs)} worktrees")
        logger.info(f"QA interval: {self.config.qa_interval} seconds")

    def stop(self):
        """Stop the background QA service"""
        if not self.running:
            return

        logger.info("Stopping Background QA Service...")
        self.running = False

        if self.observer:
            self.observer.stop()
            self.observer.join(timeout=5)

        if self.qa_thread:
            self.qa_thread.join(timeout=5)

        self._save_stats()
        logger.info("Background QA Service stopped")

    def _signal_handler(self, signum, frame):
        """Handle shutdown signals"""
        logger.info(f"Received signal {signum}, shutting down...")
        self.stop()
        sys.exit(0)

    def _qa_worker(self):
        """Background worker that processes QA jobs"""
        logger.info("QA worker thread started")

        while self.running:
            try:
                # Check for pending changes and run QA
                current_time = datetime.now()

                for worktree_name, job in self.qa_jobs.items():
                    # Check if QA should be run
                    should_run_qa = False

                    # Time-based QA
                    if (job.last_qa_run is None or
                        (current_time - job.last_qa_run).seconds >= self.config.qa_interval):
                        should_run_qa = True

                    # Change-based QA (with debounce)
                    pending_changes = self.event_handler.pending_changes.get(worktree_name, set())
                    if (pending_changes and
                        (current_time - self.event_handler.last_change_time).seconds >= self.config.debounce_time):
                        should_run_qa = True

                    if should_run_qa and self.config.enable_auto_qa:
                        self._run_qa_for_worktree(worktree_name, job)

                        # Clear pending changes
                        if worktree_name in self.event_handler.pending_changes:
                            del self.event_handler.pending_changes[worktree_name]

                # Sleep before next check
                time.sleep(10)  # Check every 10 seconds

            except Exception as e:
                logger.error(f"QA worker error: {e}")
                time.sleep(30)  # Wait longer on error

        logger.info("QA worker thread stopped")

    def _run_qa_for_worktree(self, worktree_name: str, job: QAJob):
        """Run QA analysis for a specific worktree"""
        try:
            logger.info(f"Running QA analysis for worktree '{worktree_name}'")

            # Initialize QA analyzer
            analyzer = QAAnalyzer(job.worktree_path)

            # Generate QA report
            report = analyzer.generate_report()

            # Update job info
            job.last_qa_run = datetime.now()
            job.qa_report = report
            job.pending_changes.clear()

            # Update stats
            self.stats['total_qa_runs'] += 1
            self.stats['last_activity'] = datetime.now()

            if report.integration_status['can_merge']:
                self.stats['successful_qa_runs'] += 1
                logger.info(f"✅ QA passed for worktree '{worktree_name}'")
            else:
                self.stats['failed_qa_runs'] += 1
                logger.warning(f"❌ QA failed for worktree '{worktree_name}': {report.integration_status['blocking_issues']} issues")

            # Save report
            self._save_qa_report(worktree_name, report)

        except Exception as e:
            logger.error(f"Failed to run QA for worktree '{worktree_name}': {e}")
            self.stats['failed_qa_runs'] += 1

    def _save_qa_report(self, worktree_name: str, report: QAReport):
        """Save QA report to worktree artifacts"""
        try:
            job = self.qa_jobs[worktree_name]
            artifacts_dir = job.worktree_path / "artifacts"
            artifacts_dir.mkdir(exist_ok=True)

            # Save JSON report
            json_path = artifacts_dir / f"background_qa_report_{int(time.time())}.json"
            with open(json_path, 'w', encoding='utf-8') as f:
                json.dump({
                    "worktree": worktree_name,
                    "timestamp": report.timestamp,
                    "quality_scores": {
                        "algorithmic_complexity": report.metrics.algorithmic_complexity,
                        "quantum_optimization": report.metrics.quantum_optimization,
                        "software_engineering": report.metrics.software_engineering,
                        "code_quality": report.metrics.code_quality,
                        "performance": report.metrics.performance,
                        "security": report.metrics.security
                    },
                    "issues_count": len(report.issues),
                    "critical_issues": sum(1 for i in report.issues if i.severity.name == "CRITICAL"),
                    "integration_status": report.integration_status
                }, f, indent=2, ensure_ascii=False, default=str)

            logger.debug(f"Saved QA report for '{worktree_name}': {json_path}")

        except Exception as e:
            logger.error(f"Failed to save QA report: {e}")

    def _save_stats(self):
        """Save service statistics"""
        try:
            stats_file = Path("background_qa_stats.json")
            with open(stats_file, 'w', encoding='utf-8') as f:
                json.dump({
                    "timestamp": datetime.now().isoformat(),
                    "stats": self.stats,
                    "worktrees": list(self.qa_jobs.keys()),
                    "config": {
                        "qa_interval": self.config.qa_interval,
                        "debounce_time": self.config.debounce_time,
                        "max_concurrent_qa": self.config.max_concurrent_qa,
                        "enable_auto_qa": self.config.enable_auto_qa
                    }
                }, f, indent=2, ensure_ascii=False, default=str)

            logger.info(f"Service stats saved: {stats_file}")

        except Exception as e:
            logger.error(f"Failed to save stats: {e}")

    def get_status(self) -> Dict[str, Any]:
        """Get current service status"""
        return {
            "running": self.running,
            "worktrees": len(self.qa_jobs),
            "stats": self.stats,
            "watchdog_available": WATCHDOG_AVAILABLE,
            "config": {
                "qa_interval": self.config.qa_interval,
                "debounce_time": self.config.debounce_time,
                "max_concurrent_qa": self.config.max_concurrent_qa,
                "enable_auto_qa": self.config.enable_auto_qa
            }
        }

    def trigger_qa(self, worktree_name: str) -> bool:
        """Manually trigger QA for a specific worktree"""
        if worktree_name not in self.qa_jobs:
            logger.error(f"Worktree '{worktree_name}' not found")
            return False

        job = self.qa_jobs[worktree_name]
        logger.info(f"Manually triggering QA for worktree '{worktree_name}'")

        try:
            self._run_qa_for_worktree(worktree_name, job)
            return True
        except Exception as e:
            logger.error(f"Manual QA trigger failed: {e}")
            return False

def load_worktrees_from_manager() -> Dict[str, Path]:
    """Load worktree information from worktree manager"""
    worktrees = {}

    # Try to load from worktree manager
    manager_file = Path("tools/worktrees/.worktrees.json")
    if manager_file.exists():
        try:
            with open(manager_file, 'r', encoding='utf-8') as f:
                data = json.load(f)
                for name, info in data.items():
                    worktrees[name] = Path(info['path'])
        except Exception as e:
            logger.warning(f"Failed to load worktrees from manager: {e}")

    # Fallback: scan for git worktrees
    if not worktrees:
        try:
            result = subprocess.run(
                ["git", "worktree", "list", "--porcelain"],
                capture_output=True, text=True, cwd="."
            )

            if result.returncode == 0:
                current_worktree = None
                for line in result.stdout.split('\n'):
                    if line.startswith('worktree '):
                        path = Path(line.split(' ', 1)[1])
                        if path.exists():
                            worktree_name = path.name
                            worktrees[worktree_name] = path
        except Exception as e:
            logger.warning(f"Failed to scan git worktrees: {e}")

    return worktrees

def main():
    """Main entry point for background QA service"""

    import argparse

    parser = argparse.ArgumentParser(description="Background QA Service for Continuous Code Quality Assurance")
    parser.add_argument("--config", help="Configuration file path")
    parser.add_argument("--watch-path", action="append", help="Paths to watch (can specify multiple)")
    parser.add_argument("--qa-interval", type=int, default=300, help="QA analysis interval in seconds")
    parser.add_argument("--debounce-time", type=float, default=2.0, help="File change debounce time in seconds")
    parser.add_argument("--max-concurrent", type=int, default=2, help="Maximum concurrent QA analyses")
    parser.add_argument("--auto-discover-worktrees", action="store_true", help="Auto-discover git worktrees")
    parser.add_argument("--daemon", action="store_true", help="Run as daemon (background)")
    parser.add_argument("--status", action="store_true", help="Show service status")
    parser.add_argument("--trigger-qa", help="Manually trigger QA for specific worktree")

    args = parser.parse_args()

    # Load configuration
    watch_paths = []
    if args.watch_path:
        watch_paths = [Path(p) for p in args.watch_path]
    else:
        # Default watch paths
        watch_paths = [Path("./worktrees"), Path("./")]

    config = QAServiceConfig(
        watch_paths=watch_paths,
        qa_interval=args.qa_interval,
        debounce_time=args.debounce_time,
        max_concurrent_qa=args.max_concurrent
    )

    # Initialize service
    service = BackgroundQAService(config)

    # Auto-discover worktrees
    if args.auto_discover_worktrees:
        worktrees = load_worktrees_from_manager()
        for name, path in worktrees.items():
            service.add_worktree(name, path)
        logger.info(f"Auto-discovered {len(worktrees)} worktrees")

    # Handle commands
    if args.status:
        status = service.get_status()
        print(json.dumps(status, indent=2, default=str))
        return

    if args.trigger_qa:
        success = service.trigger_qa(args.trigger_qa)
        print(f"QA trigger {'successful' if success else 'failed'}")
        return

    # Start service
    try:
        service.start()

        if args.daemon:
            # Run as daemon
            print("Background QA Service started in daemon mode")
            print("PID:", os.getpid())
            print("Log file: background_qa.log")

            # Keep running
            while True:
                time.sleep(1)
        else:
            # Interactive mode
            print("Background QA Service started (press Ctrl+C to stop)")
            try:
                while True:
                    time.sleep(1)
            except KeyboardInterrupt:
                pass
            finally:
                service.stop()

    except KeyboardInterrupt:
        pass
    except Exception as e:
        logger.error(f"Service error: {e}")
        service.stop()
        sys.exit(1)

if __name__ == "__main__":
    main()