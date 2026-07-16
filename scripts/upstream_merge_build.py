#!/usr/bin/env python3
from __future__ import annotations

import argparse
import fnmatch
import hashlib
import json
import os
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable, Sequence


DEFAULT_SYNC_BRANCH = os.environ.get(
    "CODEX_UPSTREAM_SYNC_BRANCH", "codex/upstream-sync-2026-03-22"
)
DEFAULT_REMOTE = os.environ.get("CODEX_UPSTREAM_REMOTE", "upstream")
DEFAULT_BRANCH = os.environ.get("CODEX_UPSTREAM_BRANCH", "main")
DEFAULT_JOBS = int(
    os.environ.get("CODEX_FAST_BUILD_JOBS", os.environ.get("FAST_BUILD_JOBS", "12"))
)
DEFAULT_METHOD = os.environ.get("CODEX_FAST_BUILD_METHOD", "md5")
DEFAULT_PROFILE = os.environ.get("CODEX_FAST_BUILD_PROFILE", "release")
DEFAULT_MIN_FREE_GB = float(os.environ.get("CODEX_MIN_FREE_GB", "8"))
DEFAULT_WATCH_INTERVAL_SECONDS = int(os.environ.get("CODEX_WATCH_INTERVAL_SECONDS", "30"))
DEFAULT_MAX_ATTEMPTS = int(os.environ.get("CODEX_WATCH_MAX_ATTEMPTS", "0"))

DEFAULT_WORK_DIR = Path(r"F:\codex-sync")
DEFAULT_CARGO_HOME = Path.home() / ".cargo"
DEFAULT_TARGET_DIR = Path(r"F:\codex-targets")
DEFAULT_FALLBACK_WORK_DIR = Path(r"H:\codex-sync\worktrees")
DEFAULT_FALLBACK_TARGET_DIR = Path(r"H:\codex-build\target")
DEFAULT_CLI_TARGETS = ["codex-cli", "codex-tui"]


@dataclass(frozen=True)
class FeatureRule:
    name: str
    custom_patterns: tuple[str, ...]
    upstream_patterns: tuple[str, ...]
    recommendation: str


@dataclass(frozen=True)
class ConflictRule:
    pattern: str
    strategy: str


@dataclass(frozen=True)
class StoragePlan:
    cargo_home: Path
    work_dir: Path
    target_dir: Path
    report_dir: Path
    log_dir: Path
    worktree_root: Path
    target_fallback_used: bool
    worktree_fallback_used: bool
    free_gb: float
    min_free_gb: float


@dataclass(frozen=True)
class Target:
    name: str
    kind: str
    cwd: Path
    build_cmd: list[str]
    watch_roots: tuple[Path, ...]
    install_relpaths: dict[str, str] = field(default_factory=dict)
    process_names: tuple[str, ...] = ()
    description: str = ""


FEATURE_RULES = (
    FeatureRule(
        name="Agents",
        custom_patterns=("codex-rs/core/src/agents/**", ".codex/agents/**"),
        upstream_patterns=("codex-rs/core/src/agent/**", "codex-rs/cli/src/*agent*"),
        recommendation="prefer upstream agent flow, reapply custom orchestration hooks only where they add unique value",
    ),
    FeatureRule(
        name="Plan",
        custom_patterns=("codex-rs/core/src/plan/**", "codex-rs/cli/src/plan_*"),
        upstream_patterns=("codex-rs/core/src/tasks/**", "codex-rs/core/src/codex/**"),
        recommendation="keep upstream collaboration and task model, port custom budgeting/logging as adapters",
    ),
    FeatureRule(
        name="Orchestration",
        custom_patterns=("codex-rs/core/src/orchestration/**",),
        upstream_patterns=("codex-rs/core/src/agent/**", "codex-rs/core/src/tasks/**"),
        recommendation="preserve unique custom orchestration behind wrappers or feature gates",
    ),
    FeatureRule(
        name="MCP/Plugins",
        custom_patterns=("codex-rs/core/src/mcp_*", ".codex/mcp-servers.yaml"),
        upstream_patterns=("codex-rs/core/src/mcp/**", "codex-rs/core/src/plugins/**"),
        recommendation="use upstream schemas/loaders first, keep custom loaders only as thin wrappers",
    ),
    FeatureRule(
        name="GUI",
        custom_patterns=("codex-gui-x/**",),
        upstream_patterns=("gui/**", "codex-rs/app-server-protocol/**"),
        recommendation="keep codex-gui-x isolated and only track shared protocol changes",
    ),
)


CONFLICT_RULES = (
    ConflictRule("codex-rs/**", "manual"),
    ConflictRule("gui/**", "manual"),
    ConflictRule("codex-gui-x/**", "ours"),
    ConflictRule(".codex/**", "ours"),
    ConflictRule("_docs/**", "ours"),
    ConflictRule("docs/**", "theirs"),
    ConflictRule(".github/workflows/**", "theirs"),
    ConflictRule("justfile", "manual"),
    ConflictRule("scripts/**", "ours"),
    ConflictRule("tools/**", "ours"),
)


def utc_now() -> datetime:
    return datetime.now(timezone.utc)


def now() -> str:
    return utc_now().astimezone().strftime("%H:%M:%S")


def binary_name(base: str) -> str:
    return f"{base}.exe" if os.name == "nt" else base


def npm_command(script: str) -> list[str]:
    return ["npm.cmd", "run", script] if os.name == "nt" else ["npm", "run", script]


def sanitize_branch_name(branch: str) -> str:
    return "".join(char if char.isalnum() or char in "-._" else "-" for char in branch)


class Logger:
    def __init__(self, log_file: Path | None) -> None:
        self.log_file = log_file
        if self.log_file:
            self.log_file.parent.mkdir(parents=True, exist_ok=True)

    def emit(self, level: str, message: str) -> None:
        line = f"[{now()}] [{level}] {message}"
        print(line)
        if self.log_file:
            with self.log_file.open("a", encoding="utf-8") as handle:
                handle.write(line + "\n")

    def info(self, message: str) -> None:
        self.emit("INFO", message)

    def warn(self, message: str) -> None:
        self.emit("WARN", message)

    def error(self, message: str) -> None:
        self.emit("ERROR", message)


def run(
    command: Sequence[str],
    cwd: Path,
    logger: Logger,
    extra_env: dict[str, str] | None = None,
    capture_output: bool = False,
    check: bool = True,
) -> subprocess.CompletedProcess[str]:
    env = os.environ.copy()
    if extra_env:
        env.update(extra_env)
    logger.info(f"Running in {cwd}: {' '.join(command)}")
    return subprocess.run(
        list(command),
        cwd=cwd,
        env=env,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=capture_output,
        check=check,
    )


def git_output(
    repo_root: Path,
    args: Sequence[str],
    logger: Logger,
    cwd: Path | None = None,
) -> str:
    completed = run(
        ["git", *args],
        cwd or repo_root,
        logger,
        capture_output=True,
    )
    return completed.stdout.strip()


def git_lines(
    repo_root: Path,
    args: Sequence[str],
    logger: Logger,
    cwd: Path | None = None,
) -> list[str]:
    output = git_output(repo_root, args, logger, cwd)
    return [line.strip() for line in output.splitlines() if line.strip()]


def get_repo_root(args: argparse.Namespace) -> Path:
    if args.repo_root:
        return args.repo_root.resolve()
    return Path(__file__).resolve().parents[1]


def get_workspace_root(repo_root: Path) -> Path:
    return repo_root / "codex-rs"


def get_cache_path(repo_root: Path) -> Path:
    return repo_root / ".codex-fast-build-cache.json"


def get_shared_watch(repo_root: Path) -> list[Path]:
    workspace_root = get_workspace_root(repo_root)
    return [
        repo_root / "justfile",
        workspace_root / "Cargo.toml",
        workspace_root / "Cargo.lock",
        repo_root / "package.json",
    ]


def drive_free_gb(path: Path) -> float:
    usage = shutil.disk_usage(path.anchor or str(path))
    return usage.free / (1024**3)


def choose_storage(args: argparse.Namespace, logger: Logger) -> StoragePlan:
    work_dir = args.work_dir.resolve()
    cargo_home = args.cargo_home.resolve()
    target_dir = args.target_dir.resolve()
    fallback_work_dir = args.fallback_work_dir.resolve()
    fallback_target_dir = args.fallback_target_dir.resolve()
    free_gb = drive_free_gb(work_dir)
    target_fallback = free_gb < args.min_free_gb
    worktree_fallback = free_gb < args.min_free_gb
    chosen_target_dir = fallback_target_dir if target_fallback else target_dir
    chosen_worktree_root = fallback_work_dir if worktree_fallback else (work_dir / "worktrees")
    report_dir = work_dir / "reports"
    log_dir = work_dir / "logs"
    logger.info(
        f"Primary work drive free space: {free_gb:.2f} GiB (threshold {args.min_free_gb:.2f} GiB)"
    )
    if target_fallback:
        logger.warn(f"Target dir fallback enabled: {chosen_target_dir}")
    if worktree_fallback:
        logger.warn(f"Worktree dir fallback enabled: {chosen_worktree_root}")
    return StoragePlan(
        cargo_home=cargo_home,
        work_dir=work_dir,
        target_dir=chosen_target_dir,
        report_dir=report_dir,
        log_dir=log_dir,
        worktree_root=chosen_worktree_root,
        target_fallback_used=target_fallback,
        worktree_fallback_used=worktree_fallback,
        free_gb=free_gb,
        min_free_gb=args.min_free_gb,
    )


def ensure_directories(storage: StoragePlan, dry_run: bool, logger: Logger) -> None:
    directories = [
        storage.cargo_home,
        storage.cargo_home / "bin",
        storage.work_dir,
        storage.report_dir,
        storage.log_dir,
        storage.target_dir,
        storage.worktree_root,
    ]
    for directory in directories:
        if dry_run:
            logger.info(f"[dry-run] ensure directory: {directory}")
            continue
        directory.mkdir(parents=True, exist_ok=True)


def get_targets(repo_root: Path, profile: str) -> dict[str, Target]:
    workspace_root = get_workspace_root(repo_root)
    profile_dir = "release" if profile == "release" else profile
    return {
        "codex-cli": Target(
            name="codex-cli",
            kind="rust",
            cwd=workspace_root,
            build_cmd=[
                "cargo",
                "build",
                "--release",
                "-p",
                "codex-cli",
                "--features",
                "custom-features",
            ],
            watch_roots=(
                workspace_root / "cli",
                workspace_root / "core",
                workspace_root / "exec",
                workspace_root / "protocol",
                workspace_root / "config",
                workspace_root / "state",
                workspace_root / "mcp-server",
                workspace_root / "deep-research",
                workspace_root / "utils",
            ),
            install_relpaths={
                f"{profile_dir}/{binary_name('codex')}": binary_name("codex"),
            },
            process_names=("codex",),
            description="Rust CLI binary with custom features",
        ),
        "codex-tui": Target(
            name="codex-tui",
            kind="rust",
            cwd=workspace_root,
            build_cmd=["cargo", "build", "--release", "-p", "codex-tui"],
            watch_roots=(
                workspace_root / "tui",
                workspace_root / "core",
                workspace_root / "protocol",
                workspace_root / "state",
                workspace_root / "utils",
            ),
            install_relpaths={
                f"{profile_dir}/{binary_name('codex-tui')}": binary_name("codex-tui"),
            },
            process_names=("codex-tui",),
            description="Rust TUI binary",
        ),
        "codex-gui": Target(
            name="codex-gui",
            kind="rust",
            cwd=workspace_root / "gui",
            build_cmd=[
                "cargo",
                "build",
                "--release",
                "--manifest-path",
                str((workspace_root / "gui" / "Cargo.toml").resolve()),
            ],
            watch_roots=(
                workspace_root / "gui",
                workspace_root / "core",
                workspace_root / "protocol",
                workspace_root / "state",
            ),
            install_relpaths={
                f"{profile_dir}/{binary_name('codex-gui')}": binary_name("codex-gui"),
            },
            process_names=("codex-gui",),
            description="Custom Rust GUI binary",
        ),
        "codex-gui-x": Target(
            name="codex-gui-x",
            kind="node",
            cwd=repo_root / "codex-gui-x",
            build_cmd=npm_command("build"),
            watch_roots=(
                repo_root / "codex-gui-x" / "src",
                repo_root / "codex-gui-x" / "public",
                repo_root / "codex-gui-x" / "package.json",
                repo_root / "codex-gui-x" / "tsconfig.json",
                repo_root / "codex-gui-x" / "vite.config.ts",
            ),
            description="Custom Vite GUI bundle",
        ),
        "extensions": Target(
            name="extensions",
            kind="node",
            cwd=repo_root / "extensions",
            build_cmd=npm_command("compile"),
            watch_roots=(
                repo_root / "extensions" / "src",
                repo_root / "extensions" / "package.json",
                repo_root / "extensions" / "tsconfig.json",
            ),
            description="Custom extension bundle",
        ),
    }


def iter_files(paths: Iterable[Path]) -> list[Path]:
    files: set[Path] = set()
    ignored_dirs = {
        "target",
        "node_modules",
        ".git",
        "dist",
        "build",
        ".next",
        ".turbo",
        ".cache",
    }
    ignored_suffixes = {".pyc", ".pyo", ".pkl", ".log", ".tmp", ".swp"}
    for path in paths:
        if not path.exists():
            continue
        if path.is_file():
            files.add(path)
            continue
        for child in path.rglob("*"):
            if any(part in ignored_dirs for part in child.parts):
                continue
            if child.is_file() and child.suffix not in ignored_suffixes:
                files.add(child)
    return sorted(files)


def cargo_metadata_digest(workspace_root: Path) -> str:
    completed = subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        cwd=workspace_root,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=True,
    )
    return hashlib.md5(completed.stdout.encode("utf-8")).hexdigest()


def fingerprint(
    repo_root: Path,
    workspace_root: Path,
    target: Target,
    files: list[Path],
    method: str,
) -> str:
    digest = hashlib.md5()
    metadata_digest = (
        cargo_metadata_digest(workspace_root)
        if method == "cargo-metadata" and target.kind == "rust"
        else None
    )
    for path in files:
        relative = path.relative_to(repo_root).as_posix()
        digest.update(relative.encode("utf-8"))
        stat = path.stat()
        if method == "mtime":
            digest.update(f"{stat.st_mtime_ns}:{stat.st_size}".encode("utf-8"))
            continue
        if method == "cargo-metadata":
            digest.update(
                f"{stat.st_mtime_ns}:{stat.st_size}:{path.parent.name}".encode("utf-8")
            )
            continue
        digest.update(path.read_bytes())
    if metadata_digest:
        digest.update(metadata_digest.encode("utf-8"))
    return digest.hexdigest()


def load_cache(cache_path: Path) -> dict:
    if not cache_path.exists():
        return {"targets": {}}
    try:
        return json.loads(cache_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        return {"targets": {}}


def save_cache(cache_path: Path, cache: dict) -> None:
    cache_path.write_text(
        json.dumps(cache, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def resolve_targets(requested: list[str] | None, targets: dict[str, Target]) -> list[str]:
    if not requested:
        return ["codex-cli", "codex-tui", "codex-gui", "codex-gui-x", "extensions"]
    resolved: list[str] = []
    for item in requested:
        for part in [part.strip() for part in item.split(",") if part.strip()]:
            if part == "all":
                resolved.extend(targets)
                continue
            if part not in targets:
                raise SystemExit(
                    f"Unknown target '{part}'. Choose from: {', '.join(sorted(targets))}"
                )
            resolved.append(part)
    return list(dict.fromkeys(resolved))


def resolve_install_targets(
    requested: list[str] | None,
    targets: dict[str, Target],
) -> list[str]:
    if not requested:
        return [name for name in DEFAULT_CLI_TARGETS if name in targets]
    return resolve_targets(requested, targets)


def detect_changed(
    repo_root: Path,
    workspace_root: Path,
    cache_path: Path,
    shared_watch: list[Path],
    targets: dict[str, Target],
    target_names: list[str],
    method: str,
    logger: Logger,
) -> tuple[list[str], dict]:
    cache = load_cache(cache_path)
    cache.setdefault("targets", {})
    changed: list[str] = []
    for name in target_names:
        target = targets[name]
        files = iter_files([*target.watch_roots, *shared_watch])
        current = fingerprint(repo_root, workspace_root, target, files, method)
        record = cache["targets"].get(name, {})
        previous = record.get("fingerprint") if record.get("method") == method else None
        logger.info(f"{name}: scanned {len(files)} input files via {method}")
        if current != previous:
            changed.append(name)
        cache["targets"][name] = {
            "description": target.description,
            "fingerprint": current,
            "files": [path.relative_to(repo_root).as_posix() for path in files],
            "method": method,
            "updated_at": utc_now().isoformat(),
        }
    return changed, cache


def add_common_flags(command: list[str], profile: str, jobs: int) -> list[str]:
    if command[:2] != ["cargo", "build"]:
        return command
    patched = command.copy()
    if "--release" in patched and profile != "release":
        patched.remove("--release")
        patched[2:2] = ["--profile", profile]
    patched.extend(["-j", str(jobs)])
    return patched


def build_env(storage: StoragePlan, jobs: int, deny_warnings: bool, args: argparse.Namespace) -> dict[str, str]:
    env = {
        "CARGO_HOME": str(storage.cargo_home),
        "CARGO_TARGET_DIR": str(storage.target_dir),
        "CARGO_BUILD_JOBS": str(jobs),
        "CODEX_FAST_BUILD_JOBS": str(jobs),
        "CARGO_INCREMENTAL": "1",
    }
    if args.rustc_wrapper is not None:
        env["RUSTC_WRAPPER"] = args.rustc_wrapper
    if args.rusty_v8_archive:
        env["RUSTY_V8_ARCHIVE"] = args.rusty_v8_archive
    if args.rusty_v8_src_binding_path:
        env["RUSTY_V8_SRC_BINDING_PATH"] = str(args.rusty_v8_src_binding_path.resolve())
    if deny_warnings:
        env["RUSTFLAGS"] = "-D warnings"
    return env


def find_install_artifacts(
    storage: StoragePlan,
    target: Target,
    profile: str,
) -> dict[Path, str]:
    artifacts: dict[Path, str] = {}
    for relpath, dest in target.install_relpaths.items():
        candidate = storage.target_dir / relpath.replace(
            "release", "release" if profile == "release" else profile, 1
        )
        artifacts[candidate] = dest
    return artifacts


def kill_by_windows_path(executable_path: Path, logger: Logger) -> None:
    normalized = str(executable_path.resolve()).replace("'", "''")
    logger.info(f"Stopping process if running from path: {normalized}")
    script = (
        "$p = Get-CimInstance Win32_Process | "
        f"Where-Object {{ $_.ExecutablePath -eq '{normalized}' }}; "
        "foreach ($proc in $p) { "
        "try { Stop-Process -Id $proc.ProcessId -ErrorAction Stop } catch {}; "
        "try { Stop-Process -Id $proc.ProcessId -Force -ErrorAction SilentlyContinue } catch {} "
        "}"
    )
    subprocess.run(
        ["powershell", "-NoProfile", "-Command", script],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )


def kill_processes(
    targets_to_kill: Iterable[Target],
    storage: StoragePlan,
    profile: str,
    logger: Logger,
) -> None:
    install_paths = sorted(
        {
            storage.cargo_home / "bin" / dest_name
            for target in targets_to_kill
            for dest_name in target.install_relpaths.values()
        }
    )
    if install_paths:
        for path in install_paths:
            if os.name == "nt":
                kill_by_windows_path(path, logger)
            else:
                subprocess.run(
                    ["pkill", "-f", str(path)],
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                    check=False,
                )
        return
    names = sorted({name for target in targets_to_kill for name in target.process_names})
    if not names:
        logger.info("No processes configured for selected targets.")
        return
    for name in names:
        subprocess.run(
            ["pkill", "-f", name] if os.name != "nt" else ["taskkill", "/F", "/IM", binary_name(name), "/T"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )


def verify_binary(binary_path: Path, logger: Logger) -> dict[str, str | None]:
    info: dict[str, str | None] = {
        "path": str(binary_path),
        "resolved_path": str(binary_path.resolve()) if binary_path.exists() else None,
        "version": None,
    }
    if not binary_path.exists():
        logger.warn(f"Binary not found for verification: {binary_path}")
        return info
    completed = subprocess.run(
        [str(binary_path), "--version"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    if completed.returncode == 0:
        info["version"] = completed.stdout.strip()
        logger.info(f"Verified {binary_path.name}: {info['version']}")
    else:
        logger.warn(f"Version check failed for {binary_path}: exit {completed.returncode}")
    return info


def classify_strategy(path: str) -> str:
    for rule in CONFLICT_RULES:
        if fnmatch.fnmatch(path, rule.pattern):
            return rule.strategy
    return "manual"


def feature_equivalent(
    repo_root: Path,
    upstream_ref: str,
    logger: Logger,
    rule: FeatureRule,
) -> bool:
    for pattern in rule.upstream_patterns:
        matches = git_lines(
            repo_root, ["ls-tree", "-r", "--name-only", upstream_ref, "--", pattern], logger
        )
        if matches:
            return True
    return False


def collect_analysis(
    repo_root: Path,
    upstream_ref: str,
    base_branch: str,
    logger: Logger,
) -> dict:
    current_branch = git_output(repo_root, ["branch", "--show-current"], logger) or "DETACHED"
    custom_paths = sorted(
        set(git_lines(repo_root, ["diff", "--name-only", f"{base_branch}...main"], logger))
    )
    upstream_paths = sorted(
        set(
            git_lines(
                repo_root, ["diff", "--name-only", f"{base_branch}...{upstream_ref}"], logger
            )
        )
    )
    custom_set = set(custom_paths)
    upstream_set = set(upstream_paths)
    official_only = sorted(upstream_set - custom_set)
    custom_only = sorted(custom_set - upstream_set)
    overlap = sorted(custom_set & upstream_set)
    custom_commits = git_lines(
        repo_root,
        [
            "log",
            "--oneline",
            "--no-merges",
            f"{upstream_ref}..main",
            "--",
            "codex-rs",
            "scripts",
            "tools",
            "codex-gui-x",
            "justfile",
        ],
        logger,
    )
    try:
        range_diff = subprocess.run(
            ["git", "range-diff", f"{upstream_ref}...main", f"{upstream_ref}...{base_branch}"],
            cwd=repo_root,
            text=True,
            encoding="utf-8",
            errors="replace",
            capture_output=True,
            timeout=20,
            check=False,
        )
        range_diff_text = (range_diff.stdout or range_diff.stderr).strip()
    except subprocess.TimeoutExpired:
        range_diff_text = "range-diff timed out after 20 seconds"
    feature_summary = []
    for rule in FEATURE_RULES:
        equivalent = feature_equivalent(repo_root, upstream_ref, logger, rule)
        matching_custom = [
            path
            for path in custom_paths
            if any(fnmatch.fnmatch(path, pattern) for pattern in rule.custom_patterns)
        ]
        matching_overlap = [
            path
            for path in overlap
            if any(
                fnmatch.fnmatch(path, pattern)
                for pattern in (rule.custom_patterns + rule.upstream_patterns)
            )
        ]
        feature_summary.append(
            {
                "name": rule.name,
                "upstream_equivalent": equivalent,
                "recommendation": (
                    rule.recommendation
                    if equivalent
                    else "keep custom implementation; no equivalent upstream surface detected"
                ),
                "custom_matches": matching_custom[:50],
                "overlap_matches": matching_overlap[:50],
            }
        )
    return {
        "generated_at": utc_now().isoformat(),
        "current_branch": current_branch,
        "base_branch": base_branch,
        "upstream_ref": upstream_ref,
        "official_only": official_only,
        "custom_only": custom_only,
        "overlap": overlap,
        "custom_commits": custom_commits,
        "range_diff": range_diff_text,
        "feature_summary": feature_summary,
        "strategy_counts": {
            "ours": sum(1 for path in overlap if classify_strategy(path) == "ours"),
            "theirs": sum(1 for path in overlap if classify_strategy(path) == "theirs"),
            "manual": sum(1 for path in overlap if classify_strategy(path) == "manual"),
        },
    }


def render_markdown_report(analysis: dict, storage: StoragePlan) -> str:
    overlap = analysis["overlap"]
    ours = [path for path in overlap if classify_strategy(path) == "ours"]
    theirs = [path for path in overlap if classify_strategy(path) == "theirs"]
    manual = [path for path in overlap if classify_strategy(path) == "manual"]
    lines = [
        "# Upstream Sync Analysis",
        "",
        f"- Generated: `{analysis['generated_at']}`",
        f"- Current branch: `{analysis['current_branch']}`",
        f"- Base branch: `{analysis['base_branch']}`",
        f"- Upstream ref: `{analysis['upstream_ref']}`",
        f"- Cargo home: `{storage.cargo_home}`",
        f"- Target dir: `{storage.target_dir}`",
        f"- Worktree root: `{storage.worktree_root}`",
        "",
        "## Summary",
        "",
        f"- Official-only paths: **{len(analysis['official_only'])}**",
        f"- Custom-only paths: **{len(analysis['custom_only'])}**",
        f"- Overlap paths: **{len(overlap)}**",
        f"- Overlap default strategy: `ours={len(ours)}` `theirs={len(theirs)}` `manual={len(manual)}`",
        "",
        "## Feature Strategy",
        "",
    ]
    for feature in analysis["feature_summary"]:
        lines.extend(
            [
                f"### {feature['name']}",
                "",
                f"- Upstream equivalent: {'yes' if feature['upstream_equivalent'] else 'no'}",
                f"- Recommendation: {feature['recommendation']}",
                "",
            ]
        )
    lines.extend(
        [
            "## Official-only",
            "",
            *[f"- `{path}`" for path in analysis["official_only"][:120]],
            "",
            "## Custom-only",
            "",
            *[f"- `{path}`" for path in analysis["custom_only"][:120]],
            "",
            "## Overlap",
            "",
            *[f"- `{path}` ({classify_strategy(path)})" for path in overlap[:160]],
            "",
            "## Custom Commits",
            "",
            *[f"- `{entry}`" for entry in analysis["custom_commits"][:80]],
            "",
            "## Range Diff",
            "",
            "```text",
            analysis["range_diff"] or "(no range-diff output)",
            "```",
            "",
        ]
    )
    return "\n".join(lines).rstrip() + "\n"


def write_analysis_outputs(
    analysis: dict,
    storage: StoragePlan,
    stem: str,
    logger: Logger,
    dry_run: bool,
) -> tuple[Path, Path]:
    json_path = storage.report_dir / f"{stem}.json"
    md_path = storage.report_dir / f"{stem}.md"
    markdown = render_markdown_report(analysis, storage)
    if dry_run:
        logger.info(f"[dry-run] would write report: {json_path}")
        logger.info(f"[dry-run] would write report: {md_path}")
        return json_path, md_path
    json_path.parent.mkdir(parents=True, exist_ok=True)
    json_path.write_text(
        json.dumps(analysis, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    md_path.write_text(markdown, encoding="utf-8")
    logger.info(f"Wrote report: {json_path}")
    logger.info(f"Wrote report: {md_path}")
    return json_path, md_path


def ensure_branch_ref(
    repo_root: Path,
    branch: str,
    start_point: str,
    logger: Logger,
    dry_run: bool,
) -> None:
    existing = git_lines(repo_root, ["branch", "--list", branch], logger)
    if existing:
        logger.info(f"Branch ref already exists: {branch}")
        return
    if dry_run:
        logger.info(f"[dry-run] would create branch ref {branch} -> {start_point}")
        return
    run(["git", "branch", branch, start_point], repo_root, logger)
    logger.info(f"Created branch ref {branch} -> {start_point}")


def worktree_path_for(storage: StoragePlan, branch: str) -> Path:
    return storage.worktree_root / sanitize_branch_name(branch)


def prepare_worktree(
    repo_root: Path,
    storage: StoragePlan,
    branch: str,
    logger: Logger,
    dry_run: bool,
) -> Path:
    path = worktree_path_for(storage, branch)
    if path.exists() and (path / ".git").exists():
        logger.info(f"Worktree already exists: {path}")
        return path
    if dry_run:
        logger.info(f"[dry-run] would create worktree: {path}")
        return path
    path.parent.mkdir(parents=True, exist_ok=True)
    run(["git", "worktree", "add", str(path), branch], repo_root, logger)
    logger.info(f"Prepared worktree: {path}")
    return path


def attempt_merge(
    repo_root: Path,
    worktree_root: Path,
    upstream_ref: str,
    logger: Logger,
    rules: list[str],
) -> dict[str, object]:
    merge = run(
        ["git", "merge", "--no-commit", "--no-ff", upstream_ref],
        worktree_root,
        logger,
        check=False,
    )
    if merge.returncode not in {0, 1}:
        raise subprocess.CalledProcessError(merge.returncode, merge.args)
    initial_conflicts = git_lines(
        repo_root, ["diff", "--name-only", "--diff-filter=U"], logger, cwd=worktree_root
    )
    resolver_failed = False
    resolver_exit_code: int | None = None
    unresolved_conflicts = initial_conflicts.copy()
    if initial_conflicts:
        resolver = [
            sys.executable,
            str(repo_root / "scripts" / "resolve_merge_conflicts.py"),
            "--repo-root",
            str(worktree_root),
            *initial_conflicts,
        ]
        for rule in rules:
            resolver.extend(["--rule", rule])
        resolver_run = run(
            resolver,
            worktree_root,
            logger,
            check=False,
        )
        resolver_exit_code = resolver_run.returncode
        resolver_failed = resolver_run.returncode != 0
        if initial_conflicts:
            run(["git", "add", "--", *initial_conflicts], worktree_root, logger, check=False)
        unresolved_conflicts = git_lines(
            repo_root,
            ["diff", "--name-only", "--diff-filter=U"],
            logger,
            cwd=worktree_root,
        )
        if unresolved_conflicts:
            logger.warn(
                f"Merge still has {len(unresolved_conflicts)} unresolved conflicts after rewrite attempt."
            )
        else:
            logger.warn("Merge conflicts were rewritten and staged; review the worktree carefully.")
    return {
        "merge_exit_code": merge.returncode,
        "resolver_failed": resolver_failed,
        "resolver_exit_code": resolver_exit_code,
        "initial_conflicts": initial_conflicts,
        "remaining_conflicts": unresolved_conflicts,
    }


def cmd_list_targets(args: argparse.Namespace) -> int:
    repo_root = get_repo_root(args)
    logger = Logger(None)
    choose_storage(args, logger)
    targets = get_targets(repo_root, args.profile)
    for target in targets.values():
        print(f"{target.name:12} {target.kind:5} {target.description}")
    return 0


def cmd_analyze(args: argparse.Namespace) -> int:
    repo_root = get_repo_root(args)
    logger = Logger(args.log_file)
    storage = choose_storage(args, logger)
    ensure_directories(storage, args.dry_run, logger)
    upstream_ref = f"{args.remote}/{args.branch}"
    analysis = collect_analysis(repo_root, upstream_ref, args.base_branch, logger)
    stem = args.report_stem or f"upstream-sync-{utc_now().strftime('%Y%m%d-%H%M%S')}"
    write_analysis_outputs(analysis, storage, stem, logger, args.dry_run)
    return 0


def cmd_sync(args: argparse.Namespace) -> int:
    repo_root = get_repo_root(args)
    logger = Logger(args.log_file)
    storage = choose_storage(args, logger)
    ensure_directories(storage, args.dry_run, logger)
    remotes = ["origin", args.remote] if args.include_origin else [args.remote]
    if not args.no_fetch:
        for remote in remotes:
            if args.dry_run:
                logger.info(f"[dry-run] would fetch remote: {remote}")
            else:
                run(["git", "fetch", "--prune", remote], repo_root, logger)
    upstream_ref = f"{args.remote}/{args.branch}"
    if not args.dry_run:
        run(["git", "rev-parse", "--verify", upstream_ref], repo_root, logger)
        run(["git", "rev-parse", "--verify", args.base_branch], repo_root, logger)
    ensure_branch_ref(repo_root, args.create_branch, args.base_branch, logger, args.dry_run)
    worktree_root = prepare_worktree(repo_root, storage, args.create_branch, logger, args.dry_run)
    merge_result: dict[str, object] = {
        "merge_exit_code": None,
        "resolver_failed": False,
        "resolver_exit_code": None,
        "initial_conflicts": [],
        "remaining_conflicts": [],
    }
    if args.merge:
        if args.dry_run:
            logger.info(f"[dry-run] would merge {upstream_ref} into worktree {worktree_root}")
        else:
            merge_result = attempt_merge(repo_root, worktree_root, upstream_ref, logger, args.rule)
    analysis = collect_analysis(repo_root, upstream_ref, args.base_branch, logger)
    analysis["prepared_branch"] = args.create_branch
    analysis["prepared_worktree"] = str(worktree_root)
    analysis["merge_conflicts"] = merge_result["initial_conflicts"]
    analysis["remaining_merge_conflicts"] = merge_result["remaining_conflicts"]
    analysis["merge_exit_code"] = merge_result["merge_exit_code"]
    analysis["resolver_failed"] = merge_result["resolver_failed"]
    analysis["resolver_exit_code"] = merge_result["resolver_exit_code"]
    stem = args.report_stem or f"sync-{sanitize_branch_name(args.create_branch)}"
    write_analysis_outputs(analysis, storage, stem, logger, args.dry_run)
    remaining_conflicts = merge_result["remaining_conflicts"]
    if isinstance(remaining_conflicts, list) and remaining_conflicts:
        logger.warn(
            f"Prepared worktree still needs manual conflict resolution for {len(remaining_conflicts)} files."
        )
    return 0


def cmd_build(args: argparse.Namespace) -> int:
    repo_root = get_repo_root(args)
    logger = Logger(args.log_file)
    storage = choose_storage(args, logger)
    ensure_directories(storage, args.dry_run, logger)
    workspace_root = get_workspace_root(repo_root)
    cache_path = get_cache_path(repo_root)
    targets = get_targets(repo_root, args.profile)
    shared_watch = get_shared_watch(repo_root)
    target_names = resolve_targets(args.targets, targets)
    changed, cache = detect_changed(
        repo_root,
        workspace_root,
        cache_path,
        shared_watch,
        targets,
        target_names,
        args.method,
        logger,
    )
    selected = target_names if args.force or not args.changed_only else changed
    if args.changed_only and not selected and not args.force:
        logger.info("No target inputs changed; skipping build.")
        if not args.dry_run:
            save_cache(cache_path, cache)
        return 0
    env = build_env(storage, args.jobs, args.deny_warnings, args)
    for name in selected:
        command = add_common_flags(targets[name].build_cmd, args.profile, args.jobs)
        if args.dry_run:
            logger.info(f"[dry-run] would build target {name}: {' '.join(command)}")
            continue
        run(command, targets[name].cwd, logger, env)
    if not args.dry_run:
        save_cache(cache_path, cache)
    return 0


def cmd_kill(args: argparse.Namespace) -> int:
    repo_root = get_repo_root(args)
    logger = Logger(args.log_file)
    storage = choose_storage(args, logger)
    targets = get_targets(repo_root, args.profile)
    kill_processes(
        [targets[name] for name in resolve_install_targets(args.targets, targets)],
        storage,
        args.profile,
        logger,
    )
    return 0


def cmd_install(args: argparse.Namespace) -> int:
    repo_root = get_repo_root(args)
    logger = Logger(args.log_file)
    storage = choose_storage(args, logger)
    ensure_directories(storage, args.dry_run, logger)
    targets = get_targets(repo_root, args.profile)
    target_names = [
        name
        for name in resolve_install_targets(args.targets, targets)
        if targets[name].install_relpaths
    ]
    if not target_names:
        logger.info("Selected targets do not produce installable binaries; skipping install.")
        return 0
    kill_processes([targets[name] for name in target_names], storage, args.profile, logger)
    installs: dict[str, dict[str, str | None]] = {}
    for name in target_names:
        artifact_map = find_install_artifacts(storage, targets[name], args.profile)
        for source, dest_name in artifact_map.items():
            destination = storage.cargo_home / "bin" / dest_name
            if args.dry_run:
                logger.info(f"[dry-run] would install {source} -> {destination}")
                continue
            if not source.exists():
                raise FileNotFoundError(f"Missing build artifact for {name}: {source}")
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, destination)
            logger.info(f"Installed {name}: {source} -> {destination}")
            installs[dest_name] = verify_binary(destination, logger)
    if installs and not args.dry_run:
        summary_path = storage.report_dir / "install-summary.json"
        summary_path.write_text(
            json.dumps(installs, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        logger.info(f"Wrote install summary: {summary_path}")
    return 0


def cmd_full(args: argparse.Namespace) -> int:
    logger = Logger(args.log_file)
    if not args.skip_sync:
        cmd_sync(argparse.Namespace(**vars(args)))
    if not args.skip_analyze:
        cmd_analyze(argparse.Namespace(**vars(args)))
    cmd_build(argparse.Namespace(**vars(args)))
    if args.install:
        cmd_install(argparse.Namespace(**vars(args)))
    else:
        logger.info("Skipping install phase; pass --install to copy binaries.")
    return 0


def cmd_watch(args: argparse.Namespace) -> int:
    logger = Logger(args.log_file)
    attempt = 0
    while True:
        attempt += 1
        logger.info(f"Watch cycle {attempt} started.")
        try:
            if args.mode == "full":
                cmd_full(argparse.Namespace(**vars(args)))
            elif args.mode == "build":
                cmd_build(argparse.Namespace(**vars(args)))
                if args.install:
                    cmd_install(argparse.Namespace(**vars(args)))
            elif args.mode == "install":
                cmd_install(argparse.Namespace(**vars(args)))
            else:
                raise SystemExit(f"Unsupported watch mode: {args.mode}")
            logger.info(f"Watch cycle {attempt} completed successfully.")
            if not args.keep_running:
                return 0
        except (subprocess.CalledProcessError, FileNotFoundError) as exc:
            logger.error(f"Watch cycle {attempt} failed: {exc}")
            if args.max_attempts and attempt >= args.max_attempts:
                logger.error("Maximum watch attempts reached; stopping.")
                return 1
        if args.max_attempts and attempt >= args.max_attempts:
            logger.info("Maximum watch attempts reached after success; stopping.")
            return 0
        logger.info(f"Sleeping {args.watch_interval_seconds} seconds before the next cycle.")
        time.sleep(args.watch_interval_seconds)


def common_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--repo-root", type=Path, help="Repository root (defaults to script parent repo)")
    parser.add_argument("--work-dir", type=Path, default=DEFAULT_WORK_DIR)
    parser.add_argument("--cargo-home", type=Path, default=DEFAULT_CARGO_HOME)
    parser.add_argument("--target-dir", type=Path, default=DEFAULT_TARGET_DIR)
    parser.add_argument("--fallback-target-dir", type=Path, default=DEFAULT_FALLBACK_TARGET_DIR)
    parser.add_argument("--fallback-work-dir", type=Path, default=DEFAULT_FALLBACK_WORK_DIR)
    parser.add_argument("--min-free-gb", type=float, default=DEFAULT_MIN_FREE_GB)
    parser.add_argument("--profile", default=DEFAULT_PROFILE)
    parser.add_argument("--log-file", type=Path)
    parser.add_argument(
        "--rustc-wrapper",
        default=os.environ.get("RUSTC_WRAPPER"),
        help="Override RUSTC_WRAPPER for builds; pass empty string to disable wrappers",
    )
    parser.add_argument(
        "--rusty-v8-archive",
        default=os.environ.get("RUSTY_V8_ARCHIVE"),
        help="Prebuilt rusty_v8 archive URL or path",
    )
    parser.add_argument(
        "--rusty-v8-src-binding-path",
        type=Path,
        default=Path(os.environ["RUSTY_V8_SRC_BINDING_PATH"])
        if os.environ.get("RUSTY_V8_SRC_BINDING_PATH")
        else None,
        help="Path to pre-generated rusty_v8 src_binding file",
    )
    parser.add_argument("--dry-run", action="store_true")
    return parser


def add_build_flags(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("targets", nargs="*", help="Target names or comma-separated groups")
    parser.add_argument("--jobs", type=int, default=DEFAULT_JOBS)
    parser.add_argument("--method", choices=["md5", "mtime", "cargo-metadata"], default=DEFAULT_METHOD)
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--changed-only", action="store_true")
    parser.add_argument("--no-deny-warnings", dest="deny_warnings", action="store_false")
    parser.set_defaults(deny_warnings=True)


def add_sync_flags(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--remote", default=DEFAULT_REMOTE)
    parser.add_argument("--branch", default=DEFAULT_BRANCH)
    parser.add_argument("--base-branch", default=DEFAULT_SYNC_BRANCH)
    parser.add_argument(
        "--create-branch",
        default=f"codex/upstream-sync-automation-{datetime.now().strftime('%Y-%m-%d')}",
    )
    parser.add_argument("--include-origin", action="store_true")
    parser.add_argument("--no-fetch", action="store_true")
    parser.add_argument("--merge", action="store_true")
    parser.add_argument("--rule", action="append", default=[])
    parser.add_argument("--report-stem", help="Report file stem without extension")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Codex upstream sync, merge analysis, F-drive aware builds, and overwrite install automation",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    common = common_parser()

    list_parser = subparsers.add_parser("list-targets", parents=[common], help="List supported build targets")
    list_parser.set_defaults(func=cmd_list_targets)

    analyze_parser = subparsers.add_parser("analyze", parents=[common], help="Generate upstream/custom analysis reports")
    add_sync_flags(analyze_parser)
    analyze_parser.set_defaults(func=cmd_analyze)

    sync_parser = subparsers.add_parser("sync", parents=[common], help="Fetch refs, prepare branch/worktree, and optionally merge")
    add_sync_flags(sync_parser)
    sync_parser.set_defaults(func=cmd_sync)

    build_parser_cmd = subparsers.add_parser("build", parents=[common], help="Run differential builds")
    add_build_flags(build_parser_cmd)
    build_parser_cmd.set_defaults(func=cmd_build)

    install_parser = subparsers.add_parser("install", parents=[common], help="Kill running processes and overwrite install built binaries")
    install_parser.add_argument("targets", nargs="*", help="Target names or comma-separated groups")
    install_parser.set_defaults(func=cmd_install)

    kill_parser = subparsers.add_parser("kill", parents=[common], help="Stop running target processes")
    kill_parser.add_argument("targets", nargs="*", help="Target names or comma-separated groups")
    kill_parser.set_defaults(func=cmd_kill)

    full_parser = subparsers.add_parser("full", parents=[common], help="Run sync, analyze, build, and install")
    add_sync_flags(full_parser)
    add_build_flags(full_parser)
    full_parser.add_argument("--install", action="store_true")
    full_parser.add_argument("--skip-sync", action="store_true")
    full_parser.add_argument("--skip-analyze", action="store_true")
    full_parser.set_defaults(func=cmd_full)

    watch_parser = subparsers.add_parser(
        "watch",
        parents=[common],
        help="Repeat build/full/install cycles until success or max attempts",
    )
    add_sync_flags(watch_parser)
    add_build_flags(watch_parser)
    watch_parser.add_argument(
        "--mode",
        choices=["build", "full", "install"],
        default="build",
        help="Pipeline to repeat in each watch cycle",
    )
    watch_parser.add_argument("--install", action="store_true")
    watch_parser.add_argument("--skip-sync", action="store_true")
    watch_parser.add_argument("--skip-analyze", action="store_true")
    watch_parser.add_argument(
        "--watch-interval-seconds",
        type=int,
        default=DEFAULT_WATCH_INTERVAL_SECONDS,
        help="Seconds to sleep between watch cycles",
    )
    watch_parser.add_argument(
        "--max-attempts",
        type=int,
        default=DEFAULT_MAX_ATTEMPTS,
        help="Stop after this many cycles; 0 means keep retrying indefinitely",
    )
    watch_parser.add_argument(
        "--keep-running",
        action="store_true",
        help="Continue polling/building even after one successful cycle",
    )
    watch_parser.set_defaults(func=cmd_watch)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
