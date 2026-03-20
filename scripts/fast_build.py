#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable

REPO_ROOT = Path(__file__).resolve().parents[1]
WORKSPACE_ROOT = REPO_ROOT / "codex-rs"
CACHE_PATH = REPO_ROOT / ".codex-fast-build-cache.json"
DEFAULT_JOBS = int(os.environ.get("CODEX_FAST_BUILD_JOBS", os.environ.get("FAST_BUILD_JOBS", "6")))
DEFAULT_METHOD = os.environ.get("CODEX_FAST_BUILD_METHOD", "md5")
_CARGO_METADATA_DIGEST: str | None = None
DEFAULT_PROFILE = os.environ.get("CODEX_FAST_BUILD_PROFILE", "release")


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


def now() -> str:
    return datetime.now(timezone.utc).astimezone().strftime("%H:%M:%S")


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


TARGETS: dict[str, Target] = {
    "codex-cli": Target(
        name="codex-cli",
        kind="rust",
        cwd=WORKSPACE_ROOT,
        build_cmd=["cargo", "build", "--release", "-p", "codex-cli", "--features", "custom-features"],
        watch_roots=(WORKSPACE_ROOT / "cli", WORKSPACE_ROOT / "core", WORKSPACE_ROOT / "exec", WORKSPACE_ROOT / "protocol", WORKSPACE_ROOT / "config", WORKSPACE_ROOT / "state", WORKSPACE_ROOT / "mcp-server", WORKSPACE_ROOT / "deep-research", WORKSPACE_ROOT / "utils"),
        install_map={WORKSPACE_ROOT / "target" / "release" / binary_name("codex"): binary_name("codex")},
        process_names=("codex",),
        package="codex-cli",
        description="Rust CLI binary",
    ),
    "codex-tui": Target(
        name="codex-tui",
        kind="rust",
        cwd=WORKSPACE_ROOT,
        build_cmd=["cargo", "build", "--release", "-p", "codex-tui"],
        watch_roots=(WORKSPACE_ROOT / "tui", WORKSPACE_ROOT / "core", WORKSPACE_ROOT / "protocol", WORKSPACE_ROOT / "state", WORKSPACE_ROOT / "utils"),
        install_map={WORKSPACE_ROOT / "target" / "release" / binary_name("codex-tui"): binary_name("codex-tui")},
        process_names=("codex-tui",),
        package="codex-tui",
        description="Rust TUI binary",
    ),
    "codex-gui": Target(
        name="codex-gui",
        kind="rust",
        cwd=WORKSPACE_ROOT / "gui",
        build_cmd=["cargo", "build", "--release", "--manifest-path", str((WORKSPACE_ROOT / "gui" / "Cargo.toml").resolve())],
        watch_roots=(WORKSPACE_ROOT / "gui", WORKSPACE_ROOT / "core", WORKSPACE_ROOT / "protocol", WORKSPACE_ROOT / "state"),
        install_map={WORKSPACE_ROOT / "gui" / "target" / "release" / binary_name("codex-gui"): binary_name("codex-gui")},
        process_names=("codex-gui",),
        description="Custom Rust GUI binary",
    ),
    "codex-gui-x": Target(
        name="codex-gui-x",
        kind="node",
        cwd=REPO_ROOT / "codex-gui-x",
        build_cmd=npm_command("build"),
        watch_roots=(REPO_ROOT / "codex-gui-x" / "src", REPO_ROOT / "codex-gui-x" / "public", REPO_ROOT / "codex-gui-x" / "package.json", REPO_ROOT / "codex-gui-x" / "tsconfig.json", REPO_ROOT / "codex-gui-x" / "vite.config.ts"),
        description="Custom Vite GUI bundle",
    ),
    "extensions": Target(
        name="extensions",
        kind="node",
        cwd=REPO_ROOT / "extensions",
        build_cmd=npm_command("compile"),
        watch_roots=(REPO_ROOT / "extensions" / "src", REPO_ROOT / "extensions" / "package.json", REPO_ROOT / "extensions" / "tsconfig.json"),
        description="Custom extension bundle",
    ),
}

SHARED_WATCH = [
    REPO_ROOT / "justfile",
    WORKSPACE_ROOT / "justfile",
    WORKSPACE_ROOT / "Cargo.toml",
    WORKSPACE_ROOT / "Cargo.lock",
    REPO_ROOT / "package.json",
]


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
        completed = subprocess.run(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=WORKSPACE_ROOT, capture_output=True, text=True, check=True)
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
    CACHE_PATH.write_text(json.dumps(cache, indent=2, ensure_ascii=False, sort_keys=True) + "\n", encoding="utf-8")


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
            "updated_at": datetime.now(timezone.utc).isoformat(),
        }
    return changed, cache


def run(command: list[str], cwd: Path, logger: Logger, extra_env: dict[str, str]) -> None:
    display_cwd = cwd.relative_to(REPO_ROOT) if cwd.is_relative_to(REPO_ROOT) else cwd
    logger.info(f"Running in {display_cwd}: {' '.join(command)}")
    env = os.environ.copy()
    env.update(extra_env)
    subprocess.run(command, cwd=cwd, env=env, check=True)


def cargo_bin_dir() -> Path:
    cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
    return cargo_home / "bin"


def kill_processes(targets: Iterable[Target], logger: Logger) -> None:
    names = sorted({name for target in targets for name in target.process_names})
    if not names:
        return
    logger.info(f"Stopping running processes: {', '.join(names)}")
    for name in names:
        if os.name == "nt":
            subprocess.run(["taskkill", "/F", "/IM", binary_name(name)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)
        else:
            subprocess.run(["pkill", "-f", name], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)


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


def add_common_flags(command: list[str], args: argparse.Namespace) -> list[str]:
    if command[:2] != ["cargo", "build"]:
        return command
    patched = command.copy()
    if "--release" in patched and args.profile != "release":
        patched.remove("--release")
        patched[2:2] = ["--profile", args.profile]
    patched.extend(["-j", str(args.jobs)])
    return patched


def cmd_list(_: argparse.Namespace) -> int:
    for target in TARGETS.values():
        print(f"{target.name:12} {target.kind:5} {target.description}")
    return 0


def cmd_fast_build(args: argparse.Namespace) -> int:
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
        command = add_common_flags(TARGETS[name].build_cmd, args)
        run(command, TARGETS[name].cwd, logger, env)
    save_cache(cache)
    return 0


def cmd_fast_build_install(args: argparse.Namespace) -> int:
    result = cmd_fast_build(args)
    if result != 0:
        return result
    logger = Logger(args.log_file)
    installable = [name for name in resolve_targets(args.targets) if TARGETS[name].install_map]
    if not installable:
        logger.info("Selected targets do not produce installable binaries; skipping install.")
        return 0
    install_targets(installable, logger)
    return 0


def cmd_upstream_sync(args: argparse.Namespace) -> int:
    logger = Logger(args.log_file)
    if not args.no_fetch:
        run(["git", "fetch", args.remote], REPO_ROOT, logger, {})
    merge_target = f"{args.remote}/{args.branch}"
    merge = subprocess.run(["git", "merge", "--no-commit", "--no-ff", merge_target], cwd=REPO_ROOT, text=True)
    if merge.returncode not in {0, 1}:
        raise subprocess.CalledProcessError(merge.returncode, merge.args)
    conflicts = subprocess.run(["git", "diff", "--name-only", "--diff-filter=U"], cwd=REPO_ROOT, capture_output=True, text=True, check=True)
    paths = [line.strip() for line in conflicts.stdout.splitlines() if line.strip()]
    if paths:
        resolver = [sys.executable, str(REPO_ROOT / "scripts" / "resolve_merge_conflicts.py"), *paths]
        if args.rule:
            for rule in args.rule:
                resolver.extend(["--rule", rule])
        run(resolver, REPO_ROOT, logger, {})
        subprocess.run(["git", "add", *paths], cwd=REPO_ROOT, check=True)
    logger.info("Upstream sync flow complete. Review the working tree before committing.")
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Codex fast-build and upstream-sync orchestration")
    subparsers = parser.add_subparsers(dest="command", required=True)

    def add_common(subparser: argparse.ArgumentParser) -> None:
        subparser.add_argument("targets", nargs="*", help="Target names or comma-separated groups (default: core + custom GUI/extensions)")
        subparser.add_argument("--jobs", type=int, default=DEFAULT_JOBS, help="Parallel build jobs (default: 6 or CODEX_FAST_BUILD_JOBS)")
        subparser.add_argument("--method", choices=["md5", "mtime", "cargo-metadata"], default=DEFAULT_METHOD, help="Change detection mode")
        subparser.add_argument("--profile", default=DEFAULT_PROFILE, help="Cargo profile name (default: release)")
        subparser.add_argument("--force", action="store_true", help="Build all selected targets regardless of cache")
        subparser.add_argument("--changed-only", action="store_true", help="Only build targets whose inputs changed")
        subparser.add_argument("--no-deny-warnings", dest="deny_warnings", action="store_false", help="Do not inject RUSTFLAGS=-D warnings")
        subparser.add_argument("--log-file", type=Path, help="Append log output to a file")
        subparser.set_defaults(deny_warnings=True)

    list_parser = subparsers.add_parser("list-targets", help="List supported build targets")
    list_parser.set_defaults(func=cmd_list)

    fast_build = subparsers.add_parser("fast-build", help="Run differential builds for selected targets")
    add_common(fast_build)
    fast_build.set_defaults(func=cmd_fast_build)

    fast_build_install = subparsers.add_parser("fast-build-install", help="Build changed targets and install produced binaries")
    add_common(fast_build_install)
    fast_build_install.set_defaults(func=cmd_fast_build_install)

    upstream_sync = subparsers.add_parser("upstream-sync", help="Fetch/merge upstream and auto-resolve conflicts")
    upstream_sync.add_argument("--remote", default="upstream", help="Upstream remote name")
    upstream_sync.add_argument("--branch", default="main", help="Upstream branch name")
    upstream_sync.add_argument("--no-fetch", action="store_true", help="Skip git fetch before merge")
    upstream_sync.add_argument("--rule", action="append", help="Additional resolver rule in glob=strategy form")
    upstream_sync.add_argument("--log-file", type=Path, help="Append log output to a file")
    upstream_sync.set_defaults(func=cmd_upstream_sync)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
