#!/usr/bin/env python3
"""
Build Manager Agent - Advanced Build Orchestration
Provides incremental compilation, hot reload, and intelligent build management
"""

import os
import sys
import json
import hashlib
import shutil
import subprocess
import time
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple
from dataclasses import dataclass
from datetime import datetime
import platform
import psutil

# Try to import tqdm for progress bars
try:
    from tqdm import tqdm
    TqdmAvailable = True
except ImportError:
    TqdmAvailable = False
    print("Warning: tqdm not available. Install with: pip install tqdm")

@dataclass
class BuildConfig:
    project_root: Path
    target: str = "debug"  # debug, release, test
    incremental: bool = True
    parallel_jobs: Optional[int] = None
    hot_reload: bool = False
    progress_enabled: bool = True
    verbose: bool = False

@dataclass
class BuildResult:
    success: bool
    build_time: float
    artifacts: List[Path]
    warnings: List[str]
    errors: List[str]
    metrics: Dict[str, Any]

class BuildCache:
    """MD5-based build cache for change detection"""

    def __init__(self, cache_file: Path):
        self.cache_file = cache_file
        self.cache: Dict[str, str] = {}
        self.load_cache()

    def load_cache(self):
        """Load existing cache"""
        if self.cache_file.exists():
            try:
                with open(self.cache_file, 'r', encoding='utf-8') as f:
                    self.cache = json.load(f)
            except Exception as e:
                print(f"Warning: Could not load build cache: {e}")
                self.cache = {}

    def save_cache(self):
        """Save cache to disk"""
        try:
            self.cache_file.parent.mkdir(parents=True, exist_ok=True)
            with open(self.cache_file, 'w', encoding='utf-8') as f:
                json.dump(self.cache, f, indent=2)
        except Exception as e:
            print(f"Warning: Could not save build cache: {e}")

    def get_file_hash(self, file_path: Path) -> str:
        """Calculate MD5 hash of file"""
        try:
            with open(file_path, 'rb') as f:
                return hashlib.md5(f.read()).hexdigest()
        except Exception:
            return ""

    def has_file_changed(self, file_path: Path) -> bool:
        """Check if file has changed since last build"""
        current_hash = self.get_file_hash(file_path)
        cached_hash = self.cache.get(str(file_path))

        if current_hash != cached_hash:
            self.cache[str(file_path)] = current_hash
            return True
        return False

    def get_changed_files(self, source_files: List[Path]) -> List[Path]:
        """Get list of files that have changed"""
        changed = []
        for file_path in source_files:
            if file_path.exists() and self.has_file_changed(file_path):
                changed.append(file_path)
        return changed

class BuildManager:
    """Advanced build orchestration manager"""

    def __init__(self, config: BuildConfig):
        self.config = config
        self.cache = BuildCache(config.project_root / ".build_cache.json")
        self.start_time = None

    def detect_project_type(self) -> str:
        """Detect project type (Rust, Node.js, Python, etc.)"""
        if (self.config.project_root / "Cargo.toml").exists():
            return "rust"
        elif (self.config.project_root / "package.json").exists():
            return "nodejs"
        elif (self.config.project_root / "setup.py").exists() or (self.config.project_root / "pyproject.toml").exists():
            return "python"
        elif (self.config.project_root / "Makefile").exists() or (self.config.project_root / "CMakeLists.txt").exists():
            return "native"
        else:
            return "unknown"

    def get_source_files(self) -> List[Path]:
        """Get all source files based on project type"""
        project_type = self.detect_project_type()
        source_files = []

        if project_type == "rust":
            # Rust source files
            for ext in [".rs", ".toml"]:
                source_files.extend(self.config.project_root.rglob(f"**/*{ext}"))
        elif project_type == "nodejs":
            # JavaScript/TypeScript source files
            for ext in [".js", ".ts", ".jsx", ".tsx", ".json"]:
                source_files.extend(self.config.project_root.rglob(f"**/*{ext}"))
        elif project_type == "python":
            # Python source files
            for ext in [".py", ".toml", ".txt"]:
                source_files.extend(self.config.project_root.rglob(f"**/*{ext}"))

        # Exclude common directories
        exclude_patterns = ["target", "node_modules", "dist", "build", ".git", "__pycache__"]
        filtered_files = []
        for file_path in source_files:
            if not any(pattern in str(file_path) for pattern in exclude_patterns):
                filtered_files.append(file_path)

        return filtered_files

    def should_incremental_build(self) -> Tuple[bool, List[Path]]:
        """Determine if incremental build is possible and get changed files"""
        if not self.config.incremental:
            return False, []

        source_files = self.get_source_files()
        changed_files = self.cache.get_changed_files(source_files)

        # If many files changed or it's a major change, do full rebuild
        change_ratio = len(changed_files) / len(source_files) if source_files else 0
        if change_ratio > 0.5:  # More than 50% files changed
            return False, []

        return len(changed_files) > 0, changed_files

    def setup_build_environment(self) -> Dict[str, str]:
        """Setup environment variables for optimized build"""
        env = os.environ.copy()

        # CPU core detection
        cpu_count = psutil.cpu_count(logical=True)
        if self.config.parallel_jobs is None:
            # Use 75% of available cores for build, leave some for system
            self.config.parallel_jobs = max(1, int(cpu_count * 0.75))

        project_type = self.detect_project_type()

        if project_type == "rust":
            # Rust-specific optimizations
            env['CARGO_INCREMENTAL'] = '1' if self.config.incremental else '0'
            env['CARGO_BUILD_JOBS'] = str(self.config.parallel_jobs)

            # Disable sccache as it conflicts with incremental compilation
            env['SCCACHE_DISABLE'] = '1'
            env['RUSTC_WRAPPER'] = ''

            # Target-specific settings
            if self.config.target == "release":
                env['CARGO_PROFILE_RELEASE_INCREMENTAL'] = 'false'
                env['CARGO_PROFILE_RELEASE_LTO'] = 'true'
                env['CARGO_PROFILE_RELEASE_CODEGEN_UNITS'] = '1'
            else:
                env['CARGO_PROFILE_DEV_INCREMENTAL'] = 'true'
                env['CARGO_PROFILE_DEV_CODEGEN_UNITS'] = str(min(16, self.config.parallel_jobs))

        elif project_type == "nodejs":
            # Node.js specific settings
            env['NODE_ENV'] = 'production' if self.config.target == "release" else 'development'
            env['JOBS'] = str(self.config.parallel_jobs)

        return env

    def run_build(self) -> BuildResult:
        """Execute the build process"""
        self.start_time = time.time()

        project_type = self.detect_project_type()
        print(f"🔧 Build Manager - {project_type.upper()} Project")
        print("=" * 60)

        # Setup environment
        env = self.setup_build_environment()

        # Determine build strategy
        incremental_possible, changed_files = self.should_incremental_build()

        if incremental_possible:
            print(f"⚡ Incremental build: {len(changed_files)} files changed")
            build_type = "incremental"
        else:
            print("🔄 Full rebuild required")
            build_type = "full"

        # Execute build based on project type
        if project_type == "rust":
            result = self.build_rust_project(env, build_type)
        elif project_type == "nodejs":
            result = self.build_nodejs_project(env, build_type)
        elif project_type == "python":
            result = self.build_python_project(env, build_type)
        else:
            result = self.build_generic_project(env, build_type)

        # Calculate metrics
        build_time = time.time() - self.start_time
        result.build_time = build_time
        result.metrics = self.calculate_build_metrics(build_type, changed_files, build_time)

        # Save cache
        self.cache.save_cache()

        # Hot reload if requested
        if self.config.hot_reload and result.success:
            self.perform_hot_reload(result.artifacts)

        return result

    def build_rust_project(self, env: Dict[str, str], build_type: str) -> BuildResult:
        """Build Rust project with Cargo"""
        print("🦀 Building Rust project...")

        cmd = ["cargo", "build"]
        if self.config.target == "release":
            cmd.append("--release")

        if self.config.verbose:
            cmd.append("--verbose")

        # Execute build
        success = self.run_command_with_progress(cmd, "Compiling Rust", env=env)

        if success:
            # Find built artifacts
            target_dir = self.config.project_root / "target" / self.config.target
            artifacts = []
            if target_dir.exists():
                # Find the main binary (usually matches directory name or from Cargo.toml)
                cargo_toml = self.config.project_root / "Cargo.toml"
                if cargo_toml.exists():
                    try:
                        import toml
                        cargo_config = toml.load(cargo_toml)
                        bin_name = cargo_config.get("package", {}).get("name", "unknown")
                        bin_path = target_dir / bin_name
                        if bin_path.exists():
                            artifacts.append(bin_path)
                    except:
                        pass

                # Add any executable files found
                for item in target_dir.rglob("*"):
                    if item.is_file() and os.access(item, os.X_OK):
                        artifacts.append(item)

            warnings = []
            errors = []
        else:
            artifacts = []
            warnings = ["Build failed"]
            errors = ["Check build output above"]

        return BuildResult(success, 0, artifacts, warnings, errors, {})

    def build_nodejs_project(self, env: Dict[str, str], build_type: str) -> BuildResult:
        """Build Node.js project"""
        print("📦 Building Node.js project...")

        # Install dependencies
        if (self.config.project_root / "package-lock.json").exists() or (self.config.project_root / "yarn.lock").exists():
            print("📥 Installing dependencies...")
            self.run_command_with_progress(["npm", "ci"], "Installing dependencies")

        # Run build
        success = self.run_command_with_progress(["npm", "run", "build"], "Building project")

        if success:
            artifacts = list(self.config.project_root.glob("dist/**/*")) + \
                       list(self.config.project_root.glob("build/**/*"))
            warnings = []
            errors = []
        else:
            artifacts = []
            warnings = ["Build failed"]
            errors = ["Check npm build output"]

        return BuildResult(success, 0, artifacts, warnings, errors, {})

    def build_python_project(self, env: Dict[str, str], build_type: str) -> BuildResult:
        """Build Python project"""
        print("🐍 Building Python project...")

        # Build wheel or install in development mode
        if (self.config.project_root / "setup.py").exists():
            success = self.run_command_with_progress(["python", "setup.py", "build"], "Building Python package")
        else:
            success = True  # No build step needed for simple Python projects

        artifacts = []
        warnings = []
        errors = []

        if not success:
            warnings = ["Build failed"]
            errors = ["Check Python build output"]

        return BuildResult(success, 0, artifacts, warnings, errors, {})

    def build_generic_project(self, env: Dict[str, str], build_type: str) -> BuildResult:
        """Generic build for unsupported project types"""
        print("🔧 Generic build process...")

        # Try common build commands
        build_commands = [
            ["make"],
            ["make", "build"],
            ["./build.sh"],
            ["cmake", "--build", "build"]
        ]

        success = False
        for cmd in build_commands:
            if self.run_command_with_progress(cmd, f"Trying: {' '.join(cmd)}"):
                success = True
                break

        artifacts = []
        warnings = [] if success else ["No suitable build command found"]
        errors = [] if success else ["Check project documentation for build instructions"]

        return BuildResult(success, 0, artifacts, warnings, errors, {})

    def run_command_with_progress(self, cmd: List[str], description: str,
                                env: Optional[Dict[str, str]] = None) -> bool:
        """Run command with progress visualization"""
        try:
            if self.config.progress_enabled and TqdmAvailable:
                # Run with progress bar
                process = subprocess.Popen(
                    cmd,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.STDOUT,
                    text=True,
                    env=env or os.environ,
                    cwd=self.config.project_root
                )

                lines_processed = 0
                with tqdm(desc=description, unit="lines") as pbar:
                    for line in process.stdout:
                        lines_processed += 1
                        pbar.update(1)
                        if lines_processed % 10 == 0:  # Update description occasionally
                            pbar.set_description(f"{description}: {lines_processed} lines")

                        if self.config.verbose:
                            print(line.rstrip())

                return process.wait() == 0
            else:
                # Run without progress bar
                result = subprocess.run(
                    cmd,
                    capture_output=not self.config.verbose,
                    text=True,
                    env=env or os.environ,
                    cwd=self.config.project_root
                )

                if self.config.verbose:
                    if result.stdout:
                        print(result.stdout)
                    if result.stderr:
                        print(result.stderr)

                return result.returncode == 0

        except FileNotFoundError:
            print(f"Command not found: {' '.join(cmd)}")
            return False
        except Exception as e:
            print(f"Command execution error: {e}")
            return False

    def calculate_build_metrics(self, build_type: str, changed_files: List[Path],
                              build_time: float) -> Dict[str, Any]:
        """Calculate build performance metrics"""
        source_files = self.get_source_files()

        return {
            "build_type": build_type,
            "total_source_files": len(source_files),
            "changed_files": len(changed_files),
            "change_ratio": len(changed_files) / len(source_files) if source_files else 0,
            "build_time_seconds": build_time,
            "build_time_formatted": ".1f",
            "parallel_jobs": self.config.parallel_jobs,
            "project_type": self.detect_project_type(),
            "platform": platform.platform(),
            "python_version": f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}"
        }

    def perform_hot_reload(self, artifacts: List[Path]) -> bool:
        """Perform hot reload by replacing running binaries"""
        print("🔄 Performing hot reload...")

        # This is a simplified implementation
        # In practice, you'd need more sophisticated process management

        success = True
        for artifact in artifacts:
            if artifact.is_file() and os.access(artifact, os.X_OK):
                print(f"  📦 Reloading: {artifact.name}")

                # Find running processes with this binary
                try:
                    # This is platform-specific and would need more robust implementation
                    print(f"  ⚠️ Hot reload simulation for {artifact.name} (actual implementation needed)")
                except Exception as e:
                    print(f"  ❌ Hot reload failed for {artifact.name}: {e}")
                    success = False

        return success

    def generate_report(self, result: BuildResult) -> str:
        """Generate comprehensive build report"""
        metrics = result.metrics

        report = f"""
🚀 Build Manager Report - {metrics['build_type'].title()} Build
{"=" * 80}
Build Target: {self.config.target}
Platform: {metrics['platform']}
Project Type: {metrics['project_type']}

📊 Build Statistics
{"-" * 60}
Files Processed: {metrics['total_source_files']:,}
Changed Files: {metrics['changed_files']:,} ({metrics['change_ratio']:.1%})
Build Time: {metrics['build_time_formatted']}
Parallel Jobs: {metrics['parallel_jobs']}
Python Version: {metrics['python_version']}

⚡ Build Configuration
{"-" * 60}
Incremental: {'✅' if self.config.incremental else '❌'}
Hot Reload: {'✅' if self.config.hot_reload else '❌'}
Progress Bars: {'✅' if self.config.progress_enabled and TqdmAvailable else '❌'}
Verbose Output: {'✅' if self.config.verbose else '❌'}

📁 Build Artifacts
{"-" * 60}
"""

        if result.artifacts:
            for artifact in result.artifacts[:10]:  # Show first 10
                report += f"✅ {artifact.name} ({artifact.stat().st_size / 1024 / 1024:.1f}MB)\n"
            if len(result.artifacts) > 10:
                report += f"... and {len(result.artifacts) - 10} more artifacts\n"
        else:
            report += "❌ No artifacts generated\n"

        if result.warnings:
            report += f"""
⚠️ Build Warnings
{"-" * 60}
"""
            for warning in result.warnings:
                report += f"• {warning}\n"

        if result.errors:
            report += f"""
❌ Build Errors
{"-" * 60}
"""
            for error in result.errors:
                report += f"• {error}\n"

        report += f"""
✅ Build Verification
{"-" * 60}
Compilation: {'✅ PASSED' if result.success else '❌ FAILED'}

🎯 Next Steps
{"-" * 60}
{'1. Test the built artifacts' if result.success else '1. Fix build errors and retry'}
{'2. Run automated tests' if result.success else '2. Check error logs for details'}
{'3. Deploy or distribute' if result.success else '3. Review build configuration'}
"""

        return report

def main():
    """Main entry point for build manager"""

    import argparse

    parser = argparse.ArgumentParser(description="Advanced Build Manager for Intelligent Compilation")
    parser.add_argument("command", choices=[
        "build", "incremental", "release", "hot-reload", "status"
    ], help="Build command to execute")
    parser.add_argument("--target", choices=["debug", "release", "test"],
                       default="debug", help="Build target")
    parser.add_argument("--no-incremental", action="store_true",
                       help="Disable incremental compilation")
    parser.add_argument("--jobs", type=int, help="Number of parallel jobs")
    parser.add_argument("--hot-reload", action="store_true",
                       help="Enable hot reload after build")
    parser.add_argument("--no-progress", action="store_true",
                       help="Disable progress bars")
    parser.add_argument("--verbose", action="store_true",
                       help="Enable verbose output")
    parser.add_argument("--project-root", type=Path, default=".",
                       help="Project root directory")

    args = parser.parse_args()

    # Create build configuration
    config = BuildConfig(
        project_root=args.project_root.resolve(),
        target=args.target,
        incremental=not args.no_incremental,
        parallel_jobs=args.jobs,
        hot_reload=args.hot_reload,
        progress_enabled=not args.no_progress,
        verbose=args.verbose
    )

    # Override command-specific settings
    if args.command == "incremental":
        config.incremental = True
        config.target = "debug"
    elif args.command == "release":
        config.incremental = False
        config.target = "release"
    elif args.command == "hot-reload":
        config.hot_reload = True
        config.target = "release"

    # Initialize build manager
    manager = BuildManager(config)

    # Execute command
    if args.command in ["build", "incremental", "release", "hot-reload"]:
        # Perform build
        result = manager.run_build()

        # Generate and display report
        report = manager.generate_report(result)
        print(report)

        # Exit with appropriate code
        sys.exit(0 if result.success else 1)

    elif args.command == "status":
        # Show build status
        project_type = manager.detect_project_type()
        source_files = manager.get_source_files()
        incremental_possible, changed_files = manager.should_incremental_build()

        print("🏗️ Build Manager Status")
        print("=" * 40)
        print(f"Project Type: {project_type}")
        print(f"Source Files: {len(source_files):,}")
        print(f"Incremental Build: {'Available' if incremental_possible else 'Not Available'}")
        if incremental_possible:
            print(f"Changed Files: {len(changed_files):,}")
        print(f"Parallel Jobs: {config.parallel_jobs}")
        print(f"Progress Bars: {'Enabled' if config.progress_enabled and TqdmAvailable else 'Disabled'}")

if __name__ == "__main__":
    main()