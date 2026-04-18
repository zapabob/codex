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
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable, Sequence

REPO_ROOT = Path(__file__).resolve().parents[1]
WORKSPACE_ROOT = REPO_ROOT / "codex-rs"
CACHE_PATH = REPO_ROOT / ".codex-fast-build-cache.json"
DEFAULT_JOBS = int(
    os.environ.get("CODEX_FAST_BUILD_JOBS", os.environ.get("FAST_BUILD_JOBS", "12"))
)
DEFAULT_METHOD = os.environ.get("CODEX_FAST_BUILD_METHOD", "md5")
DEFAULT_PROFILE = os.environ.get("CODEX_FAST_BUILD_PROFILE", "release")
DEFAULT_SYNC_BRANCH = os.environ.get(
    "CODEX_UPSTREAM_SYNC_BRANCH", "codex/upstream-sync-2026-03-22"
)
DEFAULT_REPORT_PATH = REPO_ROOT / "_docs" / "upstream-sync-report.md"
_CARGO_METADATA_DIGEST: str | None = None


def binary_name(base: str) -> str:
    return f"{base}.exe" if os.name == "nt" else base


def npm_command(script: str) -> list[str]:
    return ["npm.cmd", "run", script] if os.name == "nt" else ["npm", "run", script]


@dataclass(frozen=True)
class Target:
    name: str
    kind: str
    cwd: Path
    build_cmd: list[str]
    watch_roots: tuple[Path, ...]
    install_map: dict[Path, str] = field(default_factory=dict)
    process_names: tuple[str, ...] = ()
    package: str | None = None
    description: str = ""


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


TARGETS: dict[str, Target] = {
    "codex-cli": Target(
        name="codex-cli",
        kind="rust",
        cwd=WORKSPACE_ROOT,
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
            WORKSPACE_ROOT / "cli",
            WORKSPACE_ROOT / "core",
            WORKSPACE_ROOT / "exec",
            WORKSPACE_ROOT / "protocol",
            WORKSPACE_ROOT / "config",
            WORKSPACE_ROOT / "state",
            WORKSPACE_ROOT / "mcp-server",
            WORKSPACE_ROOT / "deep-research",
            WORKSPACE_ROOT / "utils",
        ),
        install_map={
            WORKSPACE_ROOT / "target" / "release" / binary_name("codex"): binary_name("codex")
        },
        process_names=("codex",),
        package="codex-cli",
        description="Rust CLI binary with custom features",
    ),
    "codex-tui": Target(
        name="codex-tui",
        kind="rust",
        cwd=WORKSPACE_ROOT,
        build_cmd=["cargo", "build", "--release", "-p", "codex-tui"],
        watch_roots=(
            WORKSPACE_ROOT / "tui",
            WORKSPACE_ROOT / "core",
            WORKSPACE_ROOT / "protocol",
            WORKSPACE_ROOT / "state",
            WORKSPACE_ROOT / "utils",
        ),
        install_map={
            WORKSPACE_ROOT / "target" / "release" / binary_name("codex-tui"): binary_name("codex-tui")
        },
        process_names=("codex-tui",),
        package="codex-tui",
        description="Rust TUI binary",
    ),
    "codex-gui": Target(
        name="codex-gui",
        kind="rust",
        cwd=WORKSPACE_ROOT / "gui",
        build_cmd=[
            "cargo",
            "build",
            "--release",
            "--manifest-path",
            str((WORKSPACE_ROOT / "gui" / "Cargo.toml").resolve()),
        ],
        watch_roots=(
            WORKSPACE_ROOT / "gui",
            WORKSPACE_ROOT / "core",
            WORKSPACE_ROOT / "protocol",
            WORKSPACE_ROOT / "state",
        ),
        install_map={
            WORKSPACE_ROOT / "gui" / "target" / "release" / binary_name("codex-gui"): binary_name("codex-gui")
        },
        process_names=("codex-gui",),
        description="Legacy custom Rust GUI binary pending plugin migration",
    ),
    "codex-gui-x": Target(
        name="codex-gui-x",
        kind="node",
        cwd=REPO_ROOT / "codex-gui-x",
        build_cmd=npm_command("build"),
        watch_roots=(
            REPO_ROOT / "codex-gui-x" / "src",
            REPO_ROOT / "codex-gui-x" / "public",
            REPO_ROOT / "codex-gui-x" / "package.json",
            REPO_ROOT / "codex-gui-x" / "tsconfig.json",
            REPO_ROOT / "codex-gui-x" / "vite.config.ts",
        ),
        description="Legacy custom Vite GUI bundle pending plugin migration",
    ),
    "extensions": Target(
        name="extensions",
        kind="node",
        cwd=REPO_ROOT / "extensions",
        build_cmd=npm_command("compile"),
        watch_roots=(
            REPO_ROOT / "extensions" / "src",
            REPO_ROOT / "extensions" / "package.json",
            REPO_ROOT / "extensions" / "tsconfig.json",
        ),
        description="Custom extension bundle",
    ),
}

FEATURE_RULES = (
    FeatureRule(
        name="Agents",
        custom_patterns=("codex-rs/core/src/agents/**", ".codex/agents/**"),
        upstream_patterns=("codex-rs/core/src/agent/**", "codex-rs/cli/src/*agent*"),
        recommendation="upstream-first; reinject only fork-only orchestration hooks that still have no official surface",
    ),
    FeatureRule(
        name="Plan",
        custom_patterns=("codex-rs/core/src/plan/**", "codex-rs/cli/src/plan_*"),
        upstream_patterns=("codex-rs/core/src/tasks/**", "codex-rs/core/src/codex/**"),
        recommendation="prefer upstream collaboration flow, keep only fork-specific budget and audit adapters",
    ),
    FeatureRule(
        name="Orchestration",
        custom_patterns=("codex-rs/core/src/orchestration/**",),
        upstream_patterns=("codex-rs/core/src/agent/**", "codex-rs/core/src/tasks/**"),
        recommendation="move common behavior behind upstream APIs and reinject only unique fork execution policies",
    ),
    FeatureRule(
        name="MCP",
        custom_patterns=("codex-rs/core/src/mcp_*", ".codex/mcp-servers.yaml"),
        upstream_patterns=("codex-rs/core/src/mcp/**", "codex-rs/core/src/plugins/**"),
        recommendation="upstream-first for schema, marketplace, and tool loading; keep custom loaders only as wrappers",
    ),
    FeatureRule(
        name="GUI To Plugin",
        custom_patterns=("gui/**", "codex-gui-x/**", "codex-rs/gui/**", "codex-rs/tauri-gui/**"),
        upstream_patterns=("codex-rs/app-server/**", "codex-rs/app-server-protocol/**", "codex-rs/core/src/plugins/**"),
        recommendation="migrate legacy GUI surfaces onto the official app-server and plugin seams; retire GUI-only launch paths after parity",
    ),
)

CONFLICT_RULES = (
    ConflictRule("_docs/**", "keep-fork"),
    ConflictRule(".codex/**", "keep-fork"),
    ConflictRule("scripts/**", "keep-fork"),
    ConflictRule("tools/**", "keep-fork"),
    ConflictRule("codex-rs/deep-research/**", "upstream-plus-reinject"),
    ConflictRule("codex-rs/core/src/agents/**", "upstream-plus-reinject"),
    ConflictRule("codex-rs/core/src/orchestration/**", "upstream-plus-reinject"),
    ConflictRule("codex-rs/core/src/plan/**", "upstream-plus-reinject"),
    ConflictRule("gui/src/app/virtual-os/**", "retire-after-parity"),
    ConflictRule("gui/src/components/virtual-os/**", "retire-after-parity"),
    ConflictRule("codex-rs/**/virtual-os/**", "retire-after-parity"),
    ConflictRule("gui/**", "plugin-migrate"),
    ConflictRule("codex-gui-x/**", "plugin-migrate"),
    ConflictRule("codex-rs/gui/**", "plugin-migrate"),
    ConflictRule("codex-rs/tauri-gui/**", "plugin-migrate"),
    ConflictRule(".agents/plugins/**", "upstream-first"),
    ConflictRule("plugins/**", "upstream-first"),
    ConflictRule("codex-cli/**", "upstream-first"),
    ConflictRule("codex-rs/cli/**", "upstream-first"),
    ConflictRule("codex-rs/app-server/**", "upstream-first"),
    ConflictRule("codex-rs/app-server-protocol/**", "upstream-first"),
    ConflictRule("codex-rs/protocol/**", "upstream-first"),
    ConflictRule("codex-rs/core/src/plugins/**", "upstream-first"),
    ConflictRule("codex-rs/**", "upstream-first"),
    ConflictRule("docs/**", "upstream-first"),
    ConflictRule(".github/workflows/**", "upstream-first"),
    ConflictRule("justfile", "manual"),
)

REPORT_CATEGORY_TITLES = {
    "upstream-first": "Adopted",
    "upstream-plus-reinject": "Reinjected",
    "plugin-migrate": "Migrated To Plugin",
    "retire-after-parity": "Retire After Parity",
    "keep-fork": "Kept Fork-Specific",
    "manual": "Manual Review",
}

SHARED_WATCH = [
    REPO_ROOT / "justfile",
    WORKSPACE_ROOT / "Cargo.toml",
    WORKSPACE_ROOT / "Cargo.lock",
    REPO_ROOT / "package.json",
]


def now() -> str:
    return datetime.now(timezone.utc).astimezone().strftime("%H:%M:%S")


def utc_now() -> datetime:
    return datetime.now(timezone.utc)


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
    display_cwd = cwd.relative_to(REPO_ROOT) if cwd.is_relative_to(REPO_ROOT) else cwd
    logger.info(f"Running in {display_cwd}: {' '.join(command)}")
    env = os.environ.copy()
    if extra_env:
        env.update(extra_env)
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


def git_output(args: Sequence[str], logger: Logger, cwd: Path = REPO_ROOT) -> str:
    completed = run(["git", *args], cwd, logger, capture_output=True)
    return completed.stdout.strip()


def git_lines(args: Sequence[str], logger: Logger, cwd: Path = REPO_ROOT) -> list[str]:
    return [line.strip() for line in git_output(args, logger, cwd).splitlines() if line.strip()]


def iter_files(paths: Iterable[Path]) -> list[Path]:
    files: set[Path] = set()
    ignored_dirs = {"target", "node_modules", ".git", "dist", "build", ".next", ".turbo", ".cache"}
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


def cargo_metadata_digest() -> str:
    global _CARGO_METADATA_DIGEST
    if _CARGO_METADATA_DIGEST is None:
        completed = subprocess.run(
            ["cargo", "metadata", "--no-deps", "--format-version", "1"],
            cwd=WORKSPACE_ROOT,
            capture_output=True,
            text=True,
            check=True,
        )
        _CARGO_METADATA_DIGEST = hashlib.md5(completed.stdout.encode("utf-8")).hexdigest()
    return _CARGO_METADATA_DIGEST


def fingerprint(target: Target, files: list[Path], method: str) -> str:
    digest = hashlib.md5()
    for path in files:
        relative = path.relative_to(REPO_ROOT).as_posix()
        digest.update(relative.encode("utf-8"))
        stat = path.stat()
        if method == "mtime":
            digest.update(f"{stat.st_mtime_ns}:{stat.st_size}".encode("utf-8"))
            continue
        if method == "cargo-metadata":
            digest.update(f"{stat.st_mtime_ns}:{stat.st_size}:{path.parent.name}".encode("utf-8"))
            continue
        digest.update(path.read_bytes())
    if method == "cargo-metadata" and target.kind == "rust":
        digest.update(cargo_metadata_digest().encode("utf-8"))
    return digest.hexdigest()


def load_cache() -> dict:
    if not CACHE_PATH.exists():
        return {"targets": {}}
    try:
        return json.loads(CACHE_PATH.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        return {"targets": {}}


def save_cache(cache: dict) -> None:
    CACHE_PATH.write_text(
        json.dumps(cache, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def resolve_targets(requested: list[str] | None) -> list[str]:
    if not requested:
        return ["codex-cli", "codex-tui", "codex-gui", "codex-gui-x", "extensions"]
    resolved: list[str] = []
    for item in requested:
        for part in [part.strip() for part in item.split(",") if part.strip()]:
            if part == "all":
                resolved.extend(TARGETS)
                continue
            if part not in TARGETS:
                raise SystemExit(f"Unknown target '{part}'. Choose from: {', '.join(sorted(TARGETS))}")
            resolved.append(part)
    return list(dict.fromkeys(resolved))


def detect_changed(target_names: list[str], method: str, logger: Logger) -> tuple[list[str], dict]:
    cache = load_cache()
    cache.setdefault("targets", {})
    changed: list[str] = []
    for name in target_names:
        target = TARGETS[name]
        files = iter_files([*target.watch_roots, *SHARED_WATCH])
        current = fingerprint(target, files, method)
        record = cache["targets"].get(name, {})
        previous = record.get("fingerprint") if record.get("method") == method else None
        logger.info(f"{name}: scanned {len(files)} input files via {method}")
        if current != previous:
            changed.append(name)
        cache["targets"][name] = {
            "description": target.description,
            "fingerprint": current,
            "files": [path.relative_to(REPO_ROOT).as_posix() for path in files],
            "method": method,
            "updated_at": utc_now().isoformat(),
        }
    return changed, cache


def add_common_flags(command: list[str], args: argparse.Namespace) -> list[str]:
    if command[:2] != ["cargo", "build"]:
        return command
    patched = command.copy()
    if "--release" in patched and args.profile != "release":
        patched.remove("--release")
        patched[2:2] = ["--profile", args.profile]
    patched.extend(["-j", str(args.jobs)])
    return patched


def cargo_bin_dir() -> Path:
    cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
    return cargo_home / "bin"


def kill_processes(targets: Iterable[Target], logger: Logger) -> None:
    names = sorted({name for target in targets for name in target.process_names})
    if not names:
        logger.info("No processes configured for selected targets.")
        return
    logger.info(f"Stopping running processes: {', '.join(names)}")
    for name in names:
        if os.name == "nt":
            subprocess.run(
                ["taskkill", "/F", "/IM", binary_name(name)],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                check=False,
            )
        else:
            subprocess.run(
                ["pkill", "-f", name],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                check=False,
            )


def install_targets(target_names: list[str], logger: Logger) -> None:
    install_dir = cargo_bin_dir()
    install_dir.mkdir(parents=True, exist_ok=True)
    chosen = [TARGETS[name] for name in target_names]
    kill_processes(chosen, logger)
    for target in chosen:
        for source, dest_name in target.install_map.items():
            if not source.exists():
                raise FileNotFoundError(f"Missing build artifact for {target.name}: {source}")
            destination = install_dir / dest_name
            shutil.copy2(source, destination)
            logger.info(f"Installed {target.name}: {source.relative_to(REPO_ROOT)} -> {destination}")


def classify_strategy(path: str) -> str:
    for rule in CONFLICT_RULES:
        if fnmatch.fnmatch(path, rule.pattern):
            return rule.strategy
    return "manual"


def classify_paths(paths: Iterable[str]) -> dict[str, list[str]]:
    buckets = {strategy: [] for strategy in REPORT_CATEGORY_TITLES}
    for path in paths:
        strategy = classify_strategy(path)
        buckets.setdefault(strategy, []).append(path)
    return buckets


def version_check(binary: str, logger: Logger) -> str | None:
    try:
        completed = run([binary, "--version"], REPO_ROOT, logger, capture_output=True, check=False)
    except FileNotFoundError:
        logger.warn(f"{binary} is not on PATH yet.")
        return None
    if completed.returncode != 0:
        logger.warn(f"{binary} --version exited with {completed.returncode}")
        return None
    version = completed.stdout.strip()
    logger.info(f"{binary} version: {version}")
    return version


def ensure_branch_ref(branch: str, start_point: str, logger: Logger) -> None:
    existing = git_lines(["branch", "--list", branch], logger)
    if existing:
        logger.info(f"Branch ref already exists: {branch}")
        return
    run(["git", "branch", branch, start_point], REPO_ROOT, logger)
    logger.info(f"Created branch ref {branch} -> {start_point}")


def feature_equivalent(rule: FeatureRule, upstream_ref: str, logger: Logger) -> bool:
    for pattern in rule.upstream_patterns:
        matches = git_lines(["ls-tree", "-r", "--name-only", upstream_ref, "--", pattern], logger)
        if matches:
            return True
    return False


def render_markdown_report(
    *,
    base_branch: str,
    upstream_ref: str,
    current_branch: str,
    candidate_paths: list[str],
    custom_commits: list[str],
    range_diff: str,
    logger: Logger,
) -> str:
    buckets = classify_paths(candidate_paths)
    lines = [
        "# Upstream Sync Analysis",
        "",
        f"- Generated: {utc_now().isoformat()}",
        f"- Current branch: `{current_branch}`",
        f"- Base branch: `{base_branch}`",
        f"- Upstream ref: `{upstream_ref}`",
        "",
        "## Summary",
        "",
        f"- Candidate repo-tracked paths from `main` vs `{upstream_ref}`: **{len(candidate_paths)}**",
        *[
            f"- `{strategy}` paths: **{len(buckets.get(strategy, []))}**"
            for strategy in REPORT_CATEGORY_TITLES
        ],
        f"- Custom commits on `main`: **{len(custom_commits)}**",
        "",
        "## Feature Strategy",
        "",
    ]
    for rule in FEATURE_RULES:
        equivalent = feature_equivalent(rule, upstream_ref, logger)
        strategy = rule.recommendation if equivalent else "keep custom implementation; upstream has no equivalent surface"
        lines.extend(
            [
                f"### {rule.name}",
                "",
                f"- Upstream equivalent: {'yes' if equivalent else 'no'}",
                f"- Strategy: {strategy}",
                "",
            ]
        )
    lines.extend(
        [
            "## Classified Paths",
            "",
        ]
    )
    for strategy, title in REPORT_CATEGORY_TITLES.items():
        lines.extend(
            [
                f"### {title}",
                "",
                *[f"- `{path}`" for path in buckets.get(strategy, [])[:120]],
                "",
            ]
        )
    lines.extend(
        [
            "## Custom Commits",
            "",
            *[f"- `{entry}`" for entry in custom_commits[:80]],
            "",
            "## Range Diff",
            "",
            "```text",
            range_diff or "(no range-diff output)",
            "```",
            "",
        ]
    )
    return "\n".join(lines).rstrip() + "\n"


def write_report(path: Path, content: str, logger: Logger) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    logger.info(f"Wrote report: {path}")


def collect_sync_inputs(args: argparse.Namespace, logger: Logger) -> tuple[str, str, str, list[str], list[str], str]:
    upstream_ref = f"{args.remote}/{args.branch}"
    base_branch = args.base_branch
    current_branch = git_output(["branch", "--show-current"], logger) or "DETACHED"
    candidate_paths = git_lines(["diff", "--name-only", f"{upstream_ref}...main"], logger)
    custom_commits = git_lines(
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
    logger.info(
        f"Running in .: git range-diff {upstream_ref}...main {upstream_ref}...{base_branch}"
    )
    try:
        range_diff_completed = subprocess.run(
            ["git", "range-diff", f"{upstream_ref}...main", f"{upstream_ref}...{base_branch}"],
            cwd=REPO_ROOT,
            text=True,
            encoding="utf-8",
            errors="replace",
            capture_output=True,
            check=False,
            timeout=20,
        )
        range_diff = (range_diff_completed.stdout or range_diff_completed.stderr).strip()
    except subprocess.TimeoutExpired:
        range_diff = "range-diff timed out after 20s; rerun manually for the full diff"
    return current_branch, base_branch, upstream_ref, candidate_paths, custom_commits, range_diff


def cmd_list(_: argparse.Namespace) -> int:
    for target in TARGETS.values():
        print(f"{target.name:12} {target.kind:5} {target.description}")
    return 0


def cmd_build(args: argparse.Namespace) -> int:
    logger = Logger(args.log_file)
    target_names = resolve_targets(args.targets)
    changed, cache = detect_changed(target_names, args.method, logger)
    selected = target_names if args.force or not args.changed_only else changed
    if args.changed_only and not changed and not args.force:
        logger.info("No target inputs changed; skipping build.")
        save_cache(cache)
        return 0
    env = {
        "CARGO_BUILD_JOBS": str(args.jobs),
        "CODEX_FAST_BUILD_JOBS": str(args.jobs),
        "CARGO_INCREMENTAL": "1",
    }
    if args.deny_warnings:
        env["RUSTFLAGS"] = "-D warnings"
    for name in selected:
        run(add_common_flags(TARGETS[name].build_cmd, args), TARGETS[name].cwd, logger, env)
    save_cache(cache)
    return 0


def cmd_install(args: argparse.Namespace) -> int:
    logger = Logger(args.log_file)
    installable = [name for name in resolve_targets(args.targets) if TARGETS[name].install_map]
    if not installable:
        logger.info("Selected targets do not produce installable binaries; skipping install.")
        return 0
    install_targets(installable, logger)
    if args.verify:
        version_check("codex", logger)
    return 0


def cmd_kill(args: argparse.Namespace) -> int:
    logger = Logger(args.log_file)
    kill_processes([TARGETS[name] for name in resolve_targets(args.targets)], logger)
    return 0


def cmd_analyze(args: argparse.Namespace) -> int:
    logger = Logger(args.log_file)
    current_branch, base_branch, upstream_ref, candidate_paths, custom_commits, range_diff = collect_sync_inputs(args, logger)
    report = render_markdown_report(
        base_branch=base_branch,
        upstream_ref=upstream_ref,
        current_branch=current_branch,
        candidate_paths=candidate_paths,
        custom_commits=custom_commits,
        range_diff=range_diff,
        logger=logger,
    )
    write_report(args.output, report, logger)
    return 0


def cmd_sync(args: argparse.Namespace) -> int:
    logger = Logger(args.log_file)
    remotes = ["origin", args.remote] if args.include_origin else [args.remote]
    if not args.no_fetch:
        for remote in remotes:
            run(["git", "fetch", "--prune", remote], REPO_ROOT, logger)
    run(["git", "rev-parse", "--verify", f"{args.remote}/{args.branch}"], REPO_ROOT, logger)
    run(["git", "rev-parse", "--verify", args.base_branch], REPO_ROOT, logger)
    if args.create_branch:
        ensure_branch_ref(args.create_branch, args.base_branch, logger)
    if args.merge:
        merge_target = f"{args.remote}/{args.branch}"
        merge = run(
            ["git", "merge", "--no-commit", "--no-ff", merge_target],
            REPO_ROOT,
            logger,
            check=False,
        )
        if merge.returncode not in {0, 1}:
            raise subprocess.CalledProcessError(merge.returncode, merge.args)
        conflicts = git_lines(["diff", "--name-only", "--diff-filter=U"], logger)
        if conflicts:
            resolver = [sys.executable, str(REPO_ROOT / "scripts" / "resolve_merge_conflicts.py"), *conflicts]
            for rule in args.rule:
                resolver.extend(["--rule", rule])
            run(resolver, REPO_ROOT, logger)
            run(["git", "add", *conflicts], REPO_ROOT, logger)
            logger.warn("Merge conflict markers were rewritten; review staged changes carefully.")
    if args.report:
        analyze_args = argparse.Namespace(
            remote=args.remote,
            branch=args.branch,
            base_branch=args.base_branch,
            output=args.report,
            log_file=args.log_file,
        )
        cmd_analyze(analyze_args)
    return 0


def cmd_full(args: argparse.Namespace) -> int:
    logger = Logger(args.log_file)
    if not args.skip_sync:
        sync_args = argparse.Namespace(
            remote=args.remote,
            branch=args.branch,
            base_branch=args.base_branch,
            create_branch=args.create_branch,
            include_origin=True,
            no_fetch=args.no_fetch,
            merge=args.merge,
            rule=args.rule,
            report=args.report,
            log_file=args.log_file,
        )
        cmd_sync(sync_args)
    if not args.skip_analyze:
        analyze_args = argparse.Namespace(
            remote=args.remote,
            branch=args.branch,
            base_branch=args.base_branch,
            output=args.report,
            log_file=args.log_file,
        )
        cmd_analyze(analyze_args)
    build_args = argparse.Namespace(
        targets=args.targets,
        jobs=args.jobs,
        method=args.method,
        profile=args.profile,
        force=args.force,
        changed_only=args.changed_only,
        deny_warnings=args.deny_warnings,
        log_file=args.log_file,
    )
    cmd_build(build_args)
    if args.install:
        install_args = argparse.Namespace(targets=args.targets, verify=True, log_file=args.log_file)
        cmd_install(install_args)
    else:
        logger.info("Skipping install phase; pass --install to copy artifacts into ~/.cargo/bin.")
    return 0


def add_build_flags(subparser: argparse.ArgumentParser) -> None:
    subparser.add_argument("targets", nargs="*", help="Target names or comma-separated groups")
    subparser.add_argument("--jobs", type=int, default=DEFAULT_JOBS, help="Parallel build jobs (default: 12)")
    subparser.add_argument("--method", choices=["md5", "mtime", "cargo-metadata"], default=DEFAULT_METHOD)
    subparser.add_argument("--profile", default=DEFAULT_PROFILE, help="Cargo profile name")
    subparser.add_argument("--force", action="store_true", help="Build all selected targets regardless of cache")
    subparser.add_argument("--changed-only", action="store_true", help="Only build targets whose inputs changed")
    subparser.add_argument("--no-deny-warnings", dest="deny_warnings", action="store_false")
    subparser.add_argument("--log-file", type=Path, help="Append log output to a file")
    subparser.set_defaults(deny_warnings=True)


def add_sync_flags(subparser: argparse.ArgumentParser) -> None:
    subparser.add_argument("--remote", default="upstream", help="Upstream remote name")
    subparser.add_argument("--branch", default="main", help="Upstream branch name")
    subparser.add_argument("--base-branch", default=DEFAULT_SYNC_BRANCH, help="Integration base branch")
    subparser.add_argument(
        "--create-branch",
        default=f"codex/upstream-sync-automation-{datetime.now().strftime('%Y-%m-%d')}",
        help="Create a non-checked-out branch ref from the base branch",
    )
    subparser.add_argument("--include-origin", action="store_true", help="Fetch origin as well as the upstream remote")
    subparser.add_argument("--no-fetch", action="store_true", help="Skip git fetch before analysis")
    subparser.add_argument("--merge", action="store_true", help="Attempt a no-commit merge before reporting")
    subparser.add_argument("--rule", action="append", default=[], help="Extra resolver rule in glob=strategy form")
    subparser.add_argument("--report", type=Path, default=DEFAULT_REPORT_PATH, help="Markdown report output path")
    if "--log-file" not in subparser._option_string_actions:
        subparser.add_argument("--log-file", type=Path, help="Append log output to a file")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Codex upstream sync, analysis, and fast differential build automation",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser("list-targets", help="List supported build targets")
    list_parser.set_defaults(func=cmd_list)

    build_parser_cmd = subparsers.add_parser(
        "build",
        aliases=["fast-build"],
        help="Run differential builds for selected targets",
    )
    add_build_flags(build_parser_cmd)
    build_parser_cmd.set_defaults(func=cmd_build)

    install_parser = subparsers.add_parser(
        "install",
        aliases=["fast-build-install"],
        help="Install built artifacts into ~/.cargo/bin",
    )
    install_parser.add_argument("targets", nargs="*", help="Target names or comma-separated groups")
    install_parser.add_argument("--verify", action="store_true", help="Run `codex --version` after install")
    install_parser.add_argument("--log-file", type=Path, help="Append log output to a file")
    install_parser.set_defaults(func=cmd_install)

    kill_parser = subparsers.add_parser("kill", help="Stop running processes for selected targets")
    kill_parser.add_argument("targets", nargs="*", help="Target names or comma-separated groups")
    kill_parser.add_argument("--log-file", type=Path, help="Append log output to a file")
    kill_parser.set_defaults(func=cmd_kill)

    analyze_parser = subparsers.add_parser("analyze", help="Generate an upstream/custom merge report")
    analyze_parser.add_argument("--remote", default="upstream", help="Upstream remote name")
    analyze_parser.add_argument("--branch", default="main", help="Upstream branch name")
    analyze_parser.add_argument("--base-branch", default=DEFAULT_SYNC_BRANCH, help="Integration base branch")
    analyze_parser.add_argument("--output", type=Path, default=DEFAULT_REPORT_PATH, help="Markdown report output path")
    analyze_parser.add_argument("--log-file", type=Path, help="Append log output to a file")
    analyze_parser.set_defaults(func=cmd_analyze)

    sync_parser = subparsers.add_parser(
        "sync",
        aliases=["upstream-sync"],
        help="Fetch upstream refs, optionally merge, and write a sync report",
    )
    add_sync_flags(sync_parser)
    sync_parser.set_defaults(func=cmd_sync)

    full_parser = subparsers.add_parser("full", help="Run sync, analyze, build, and optional install")
    add_build_flags(full_parser)
    add_sync_flags(full_parser)
    full_parser.add_argument("--install", action="store_true", help="Install binaries after build")
    full_parser.add_argument("--skip-sync", action="store_true", help="Skip sync phase")
    full_parser.add_argument("--skip-analyze", action="store_true", help="Skip analyze phase")
    full_parser.set_defaults(func=cmd_full)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
