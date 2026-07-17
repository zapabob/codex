#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tomllib
from dataclasses import asdict
from dataclasses import dataclass
from dataclasses import field
from dataclasses import replace
from pathlib import Path
from typing import Any

REPO_ROOT = Path(__file__).resolve().parents[1]
WORKSPACE_ROOT = REPO_ROOT / "codex-rs"
WORKSPACE_MANIFEST = WORKSPACE_ROOT / "Cargo.toml"
SCRIPTS_DIR = Path(__file__).resolve().parent
if str(SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPTS_DIR))

import fast_build

DEFAULT_BASELINE_REF = "rust-v0.121.0"
DEFAULT_MARKDOWN_REPORT = REPO_ROOT / "_docs" / "upstream-sync-driver-report.md"
DEFAULT_JSON_REPORT = REPO_ROOT / "_docs" / "upstream-sync-driver-report.json"
DEFAULT_CREATE_BRANCH = (
    f"codex/upstream-sync-{fast_build.datetime.now().strftime('%Y-%m-%d')}"
)
DEFAULT_INSTALL_SOURCE = (
    WORKSPACE_ROOT / "target" / "release" / fast_build.binary_name("codex")
)
DEFAULT_INSTALL_TARGET = fast_build.cargo_bin_dir() / fast_build.binary_name("codex")
DEFAULT_INSTALL_HELPER = REPO_ROOT / "scripts" / "install_with_kill.ps1"
DEFAULT_CODEXAPP_EXCLUDE_PREFIX = r"C:\Program Files\WindowsApps\OpenAI.Codex_"

CUSTOM_COMMIT_PATHS = (
    "codex-rs",
    "codex-cli",
    "gui",
    "codex-gui-x",
    "plugins",
    ".agents/plugins",
    "scripts",
    "tools",
    "README.md",
    "STRUCTURE.md",
    "CHANGELOG.md",
    "docs/plan/MERGE_STRATEGY.md",
)

POST_TAG_OVERLAY_MEMBERS = frozenset(
    {
        "plugin",
        "rollout",
        "sandboxing",
        "tools",
        "utils/plugins",
    }
)

WINDOWS_INSTALL_PROCESS_NAMES = (
    "codex",
    "codex-tui",
    "codex-gui",
    "opencode",
)


@dataclass(frozen=True)
class MergeOutcome:
    performed: bool = False
    conflicts: list[str] = field(default_factory=list)
    unresolved_conflicts: list[str] = field(default_factory=list)


@dataclass(frozen=True)
class WorkspaceRepairOutcome:
    performed: bool = False
    success: bool = True
    missing_members: list[str] = field(default_factory=list)
    restored_members: list[str] = field(default_factory=list)
    overlaid_members: list[str] = field(default_factory=list)
    missing_from_baseline: list[str] = field(default_factory=list)


@dataclass(frozen=True)
class StepOutcome:
    name: str
    command: list[str]
    cwd: str
    returncode: int
    stdout_tail: str
    stderr_tail: str
    status: str = "passed"
    failure_kind: str = ""
    failure_summary: str = ""


@dataclass(frozen=True)
class ValidationOutcome:
    performed: bool = False
    success: bool = True
    steps: list[StepOutcome] = field(default_factory=list)
    blocked_on_environment: bool = False
    failure_kind: str = ""
    failure_summary: str = ""


@dataclass(frozen=True)
class BuildOutcome:
    performed: bool = False
    success: bool = True
    command: list[str] = field(default_factory=list)
    cwd: str = ""
    returncode: int = 0
    binary_path: str | None = None
    stdout_tail: str = ""
    stderr_tail: str = ""


@dataclass(frozen=True)
class WindowsInstallOutcome:
    performed: bool = False
    success: bool = True
    command: list[str] = field(default_factory=list)
    returncode: int = 0
    install_path: str | None = None
    resolved_command_path: str | None = None
    codexapp_before: list[dict[str, Any]] = field(default_factory=list)
    codexapp_after: list[dict[str, Any]] = field(default_factory=list)
    surviving_codexapp_pids: list[int] = field(default_factory=list)
    version_output: str = ""
    app_server_help_ok: bool = False
    stdout_tail: str = ""
    stderr_tail: str = ""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Upstream-first sync driver for OpenAI/codex 0.121.0 adoption, "
            "workspace repair, post-tag hardening backports, and Windows install orchestration"
        )
    )
    parser.add_argument("--remote", default="upstream", help="Upstream remote name")
    parser.add_argument("--branch", default="main", help="Upstream branch name")
    parser.add_argument(
        "--baseline-ref",
        default=DEFAULT_BASELINE_REF,
        help="Baseline upstream ref used for upstream delta classification",
    )
    parser.add_argument(
        "--base-branch",
        default=fast_build.DEFAULT_SYNC_BRANCH,
        help="Local integration base branch used for range-diff and branch creation",
    )
    parser.add_argument(
        "--create-branch",
        default=DEFAULT_CREATE_BRANCH,
        help="Create a non-checked-out integration branch ref from --base-branch",
    )
    parser.add_argument(
        "--report-md",
        type=Path,
        default=DEFAULT_MARKDOWN_REPORT,
        help="Write the Markdown report to this path",
    )
    parser.add_argument(
        "--report-json",
        type=Path,
        default=DEFAULT_JSON_REPORT,
        help="Write the JSON report to this path",
    )
    parser.add_argument(
        "--merge", action="store_true", help="Run git merge --no-commit --no-ff"
    )
    parser.add_argument(
        "--repair-workspace",
        action="store_true",
        help="Restore missing workspace members from upstream refs",
    )
    parser.add_argument(
        "--validate",
        action="store_true",
        help="Run repo validation, cargo metadata, formatting, and cargo test",
    )
    parser.add_argument(
        "--build-release",
        action="store_true",
        help="Build the release codex CLI binary",
    )
    parser.add_argument(
        "--windows-install",
        action="store_true",
        help="Overwrite-install codex.exe while excluding CodexApp",
    )
    parser.add_argument(
        "--cargo-target-dir",
        type=Path,
        help="Optional cargo target directory for release builds; defaults to codex-rs/target",
    )
    parser.add_argument(
        "--install-source",
        type=Path,
        default=DEFAULT_INSTALL_SOURCE,
        help="Source binary path for --windows-install",
    )
    parser.add_argument(
        "--install-target",
        type=Path,
        default=DEFAULT_INSTALL_TARGET,
        help="Destination binary path for --windows-install",
    )
    parser.add_argument(
        "--install-helper",
        type=Path,
        default=DEFAULT_INSTALL_HELPER,
        help="PowerShell helper used for path-aware Windows installation",
    )
    parser.add_argument(
        "--codexapp-exclude-prefix",
        action="append",
        default=[DEFAULT_CODEXAPP_EXCLUDE_PREFIX],
        help="Process path prefix that must be excluded from kill/install handling",
    )
    parser.add_argument(
        "--include-origin",
        action="store_true",
        help="Fetch origin in addition to upstream",
    )
    parser.add_argument("--no-fetch", action="store_true", help="Skip git fetch")
    parser.add_argument(
        "--rule",
        action="append",
        default=[],
        help="Extra resolver rule in glob=strategy form",
    )
    parser.add_argument("--log-file", type=Path, help="Append log output to a file")
    return parser.parse_args()


def merge_ref(args: argparse.Namespace) -> str:
    return f"{args.remote}/{args.branch}"


def fetch_refs(args: argparse.Namespace, logger: fast_build.Logger) -> None:
    if args.no_fetch:
        logger.info("Skipping fetch; using locally available refs.")
        return
    remotes = ["origin", args.remote] if args.include_origin else [args.remote]
    for remote in remotes:
        command = ["git", "fetch", "--prune", remote]
        if remote == args.remote:
            command.append("--tags")
        fast_build.run(command, REPO_ROOT, logger)


def verify_refs(args: argparse.Namespace, logger: fast_build.Logger) -> None:
    fast_build.run(["git", "rev-parse", "--verify", merge_ref(args)], REPO_ROOT, logger)
    fast_build.run(
        ["git", "rev-parse", "--verify", args.baseline_ref], REPO_ROOT, logger
    )
    fast_build.run(
        ["git", "rev-parse", "--verify", args.base_branch], REPO_ROOT, logger
    )


def current_branch(logger: fast_build.Logger) -> str:
    return fast_build.git_output(["branch", "--show-current"], logger) or "DETACHED"


def collect_candidate_paths(
    baseline_ref: str,
    target_ref: str,
    logger: fast_build.Logger,
) -> list[str]:
    return fast_build.git_lines(
        ["diff", "--name-only", f"{baseline_ref}..{target_ref}"], logger
    )


def collect_custom_commits(
    upstream_ref: str,
    branch_name: str,
    logger: fast_build.Logger,
) -> list[str]:
    if branch_name == "DETACHED":
        return []
    return fast_build.git_lines(
        [
            "log",
            "--oneline",
            "--no-merges",
            f"{upstream_ref}..{branch_name}",
            "--",
            *CUSTOM_COMMIT_PATHS,
        ],
        logger,
    )


def collect_range_diff(
    upstream_ref: str,
    branch_name: str,
    base_branch: str,
    logger: fast_build.Logger,
    enabled: bool,
) -> str:
    if not enabled:
        return "range-diff skipped; pass --merge to collect it"
    logger.info(
        f"Running in .: git range-diff {upstream_ref}...{branch_name} {upstream_ref}...{base_branch}"
    )
    try:
        completed = subprocess.run(
            [
                "git",
                "range-diff",
                f"{upstream_ref}...{branch_name}",
                f"{upstream_ref}...{base_branch}",
            ],
            cwd=REPO_ROOT,
            text=True,
            encoding="utf-8",
            errors="replace",
            capture_output=True,
            check=False,
            timeout=20,
        )
        return (completed.stdout or completed.stderr).strip()
    except subprocess.TimeoutExpired:
        return "range-diff timed out after 20s; rerun manually for the full diff"


def write_json(path: Path, payload: dict[str, Any], logger: fast_build.Logger) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(payload, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    logger.info(f"Wrote JSON report: {path}")


def tail_text(text: str, lines: int = 40) -> str:
    trimmed = text.strip()
    if not trimmed:
        return ""
    return "\n".join(trimmed.splitlines()[-lines:])


def resolve_install_source(args: argparse.Namespace) -> Path:
    if args.cargo_target_dir and args.install_source == DEFAULT_INSTALL_SOURCE:
        return args.cargo_target_dir / "release" / fast_build.binary_name("codex")
    return args.install_source


def powershell_single_quote(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def powershell_array_literal(values: list[str] | tuple[str, ...]) -> str:
    if not values:
        return "@()"
    entries = ", ".join(powershell_single_quote(value) for value in values)
    return f"@({entries})"


def read_workspace_members(workspace_manifest: Path = WORKSPACE_MANIFEST) -> list[str]:
    data = tomllib.loads(workspace_manifest.read_text(encoding="utf-8"))
    return [str(member) for member in data["workspace"]["members"]]


def find_missing_workspace_members(
    members: list[str], workspace_root: Path = WORKSPACE_ROOT
) -> list[str]:
    return sorted(
        member for member in members if not (workspace_root / member).exists()
    )


def select_post_tag_overlay_members(members: list[str]) -> list[str]:
    return sorted(member for member in members if member in POST_TAG_OVERLAY_MEMBERS)


def member_repo_path(member: str) -> str:
    return f"codex-rs/{member}"


def git_path_exists(ref: str, repo_path: str) -> bool:
    completed = subprocess.run(
        ["git", "cat-file", "-e", f"{ref}:{repo_path}"],
        cwd=REPO_ROOT,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    return completed.returncode == 0


def ref_tree_files(ref: str, repo_path: str) -> list[str]:
    completed = subprocess.run(
        ["git", "ls-tree", "-r", "--name-only", ref, "--", repo_path],
        cwd=REPO_ROOT,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=True,
    )
    return [line.strip() for line in completed.stdout.splitlines() if line.strip()]


def ref_blob_bytes(ref: str, repo_file: str) -> bytes:
    completed = subprocess.run(
        ["git", "show", f"{ref}:{repo_file}"],
        cwd=REPO_ROOT,
        capture_output=True,
        check=True,
    )
    return completed.stdout


def cleanup_empty_directories(root: Path) -> None:
    if not root.exists():
        return
    for path in sorted(
        (item for item in root.rglob("*") if item.is_dir()), reverse=True
    ):
        try:
            path.rmdir()
        except OSError:
            continue


def sync_repo_path_from_ref(
    ref: str, repo_path: str, logger: fast_build.Logger
) -> None:
    files_in_ref = ref_tree_files(ref, repo_path)
    if not files_in_ref:
        raise RuntimeError(f"{repo_path} does not exist in {ref}")

    logger.info(f"Syncing {repo_path} from {ref}")
    target_root = REPO_ROOT / repo_path
    existing_files = set()
    if target_root.exists():
        existing_files = {
            path.relative_to(REPO_ROOT).as_posix()
            for path in target_root.rglob("*")
            if path.is_file()
        }

    files_in_ref_set = set(files_in_ref)
    for repo_file in files_in_ref:
        destination = REPO_ROOT / repo_file
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(ref_blob_bytes(ref, repo_file))

    for stale_file in sorted(existing_files - files_in_ref_set):
        (REPO_ROOT / stale_file).unlink()

    cleanup_empty_directories(target_root)


def checkout_paths_from_ref(
    ref: str, repo_paths: list[str], logger: fast_build.Logger
) -> None:
    if not repo_paths:
        return
    fast_build.run(["git", "checkout", ref, "--", *repo_paths], REPO_ROOT, logger)


def repair_workspace(
    args: argparse.Namespace, logger: fast_build.Logger
) -> WorkspaceRepairOutcome:
    if not args.repair_workspace:
        logger.info("Skipping workspace repair.")
        return WorkspaceRepairOutcome()

    members = read_workspace_members()
    missing_members = find_missing_workspace_members(members)
    if not missing_members:
        logger.info("Workspace is already complete; nothing to restore.")
        return WorkspaceRepairOutcome(performed=True, missing_members=[])

    missing_from_baseline: list[str] = []
    restored_members: list[str] = []
    restore_paths: list[str] = []
    for member in missing_members:
        repo_path = member_repo_path(member)
        if not git_path_exists(args.baseline_ref, repo_path):
            missing_from_baseline.append(member)
            continue
        restore_paths.append(repo_path)
        restored_members.append(member)

    if missing_from_baseline:
        logger.error(
            f"Missing from baseline ref {args.baseline_ref}: {', '.join(missing_from_baseline)}"
        )
        return WorkspaceRepairOutcome(
            performed=True,
            success=False,
            missing_members=missing_members,
            restored_members=restored_members,
            missing_from_baseline=missing_from_baseline,
        )

    checkout_paths_from_ref(args.baseline_ref, restore_paths, logger)

    overlaid_members: list[str] = []
    overlay_paths: list[str] = []
    for member in select_post_tag_overlay_members(restored_members):
        repo_path = member_repo_path(member)
        if not git_path_exists(merge_ref(args), repo_path):
            continue
        overlay_paths.append(repo_path)
        overlaid_members.append(member)

    checkout_paths_from_ref(merge_ref(args), overlay_paths, logger)

    return WorkspaceRepairOutcome(
        performed=True,
        success=True,
        missing_members=missing_members,
        restored_members=restored_members,
        overlaid_members=overlaid_members,
        missing_from_baseline=[],
    )


def perform_merge(args: argparse.Namespace, logger: fast_build.Logger) -> MergeOutcome:
    if not args.merge:
        logger.info(
            "Skipping merge; pass --merge to run git merge --no-commit --no-ff."
        )
        return MergeOutcome()

    target_ref = merge_ref(args)
    merge_result = fast_build.run(
        ["git", "merge", "--no-commit", "--no-ff", target_ref],
        REPO_ROOT,
        logger,
        check=False,
    )
    if merge_result.returncode not in {0, 1}:
        raise subprocess.CalledProcessError(merge_result.returncode, merge_result.args)

    conflicts = fast_build.git_lines(["diff", "--name-only", "--diff-filter=U"], logger)
    if conflicts:
        resolver = [
            sys.executable,
            str(REPO_ROOT / "scripts" / "resolve_merge_conflicts.py"),
            *conflicts,
        ]
        for rule in args.rule:
            resolver.extend(["--rule", rule])
        fast_build.run(resolver, REPO_ROOT, logger)
        fast_build.run(["git", "add", *conflicts], REPO_ROOT, logger)
        logger.warn(
            "Merge conflict markers were rewritten; review staged changes carefully."
        )

    unresolved_conflicts = fast_build.git_lines(
        ["diff", "--name-only", "--diff-filter=U"], logger
    )
    return MergeOutcome(
        performed=True, conflicts=conflicts, unresolved_conflicts=unresolved_conflicts
    )


def run_step(
    name: str, command: list[str], cwd: Path, logger: fast_build.Logger
) -> StepOutcome:
    logger.info(f"Validation step [{name}] in {cwd}: {' '.join(command)}")
    completed = subprocess.run(
        command,
        cwd=cwd,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=False,
    )
    if completed.returncode == 0:
        logger.info(f"Validation step [{name}] passed")
    else:
        logger.error(
            f"Validation step [{name}] failed with exit code {completed.returncode}"
        )
    return StepOutcome(
        name=name,
        command=command,
        cwd=str(cwd),
        returncode=completed.returncode,
        stdout_tail=tail_text(completed.stdout),
        stderr_tail=tail_text(completed.stderr),
        status="passed" if completed.returncode == 0 else "failed",
    )


def detect_validation_environment_blocker(step: StepOutcome) -> tuple[str, str] | None:
    if step.name != "cargo-test" or step.returncode == 0 or sys.platform != "win32":
        return None

    combined = "\n".join(part for part in (step.stdout_tail, step.stderr_tail) if part)
    lowered = combined.lower()
    has_v8 = (
        "failed to run custom build command for `v8" in lowered or "rusty_v8" in lowered
    )
    has_symlink = (
        "failed to create symlink" in lowered or "symlink_dir failed" in lowered
    )
    has_privilege = (
        "1314" in lowered
        or "requested privilege is not held" in lowered
        or "要求された特権を保有していません" in combined
    )
    if not (has_v8 and has_symlink and has_privilege):
        return None

    return (
        "environment_prerequisite_missing",
        (
            "Native Windows full workspace validation requires Developer Mode or equivalent "
            "symlink privilege because the v8 build script creates symlinks."
        ),
    )


def validate_repo(
    args: argparse.Namespace, logger: fast_build.Logger
) -> ValidationOutcome:
    if not args.validate:
        logger.info("Skipping validation.")
        return ValidationOutcome()

    steps = [
        (
            "python-unittest",
            [sys.executable, "-m", "unittest", "scripts.test.test_upstream_sync"],
            REPO_ROOT,
        ),
        (
            "python-py-compile",
            [
                sys.executable,
                "-m",
                "py_compile",
                "scripts/fast_build.py",
                "scripts/resolve_merge_conflicts.py",
                "scripts/upstream_sync.py",
                "scripts/test/test_upstream_sync.py",
            ],
            REPO_ROOT,
        ),
        (
            "upstream-sync-help",
            [sys.executable, "scripts/upstream_sync.py", "--help"],
            REPO_ROOT,
        ),
        (
            "resolve-merge-conflicts-help",
            [sys.executable, "scripts/resolve_merge_conflicts.py", "--help"],
            REPO_ROOT,
        ),
        (
            "cargo-metadata",
            ["cargo", "metadata", "--no-deps", "--format-version", "1"],
            WORKSPACE_ROOT,
        ),
        ("cargo-fmt-check", ["cargo", "fmt", "--all", "--check"], WORKSPACE_ROOT),
        ("cargo-test", ["cargo", "test", "--workspace"], WORKSPACE_ROOT),
    ]

    outcomes: list[StepOutcome] = []
    success = True
    blocked_on_environment = False
    failure_kind = ""
    failure_summary = ""
    for name, command, cwd in steps:
        outcome = run_step(name, command, cwd, logger)
        blocker = detect_validation_environment_blocker(outcome)
        if blocker is not None:
            failure_kind, failure_summary = blocker
            blocked_on_environment = True
            outcome = replace(
                outcome,
                status="environment-prerequisite-missing",
                failure_kind=failure_kind,
                failure_summary=failure_summary,
            )
            logger.warn(
                f"Validation step [{name}] is blocked on environment prerequisites: {failure_summary}"
            )
        elif outcome.returncode != 0:
            failure_kind = "step_failure"
            failure_summary = f"Validation step {name} failed."
        outcomes.append(outcome)
        if outcome.returncode != 0:
            success = False
            break
    return ValidationOutcome(
        performed=True,
        success=success,
        steps=outcomes,
        blocked_on_environment=blocked_on_environment,
        failure_kind=failure_kind,
        failure_summary=failure_summary,
    )


def build_release(args: argparse.Namespace, logger: fast_build.Logger) -> BuildOutcome:
    if not args.build_release:
        logger.info("Skipping release build.")
        return BuildOutcome()

    command = ["cargo", "build", "--release", "-p", "codex-cli"]
    if args.cargo_target_dir:
        command.extend(["--target-dir", str(args.cargo_target_dir)])
    logger.info(f"Release build in {WORKSPACE_ROOT}: {' '.join(command)}")
    completed = subprocess.run(
        command,
        cwd=WORKSPACE_ROOT,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=False,
    )
    install_source = resolve_install_source(args)
    binary_path = install_source if install_source.exists() else None
    success = completed.returncode == 0 and binary_path is not None
    if success:
        logger.info(f"Release build produced {binary_path}")
    else:
        logger.error(f"Release build failed with exit code {completed.returncode}")
    return BuildOutcome(
        performed=True,
        success=success,
        command=command,
        cwd=str(WORKSPACE_ROOT),
        returncode=completed.returncode,
        binary_path=str(binary_path) if binary_path else None,
        stdout_tail=tail_text(completed.stdout),
        stderr_tail=tail_text(completed.stderr),
    )


def run_powershell_json(script: str) -> list[dict[str, Any]]:
    completed = subprocess.run(
        ["powershell", "-NoProfile", "-Command", script],
        cwd=REPO_ROOT,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=True,
    )
    payload = completed.stdout.strip()
    if not payload:
        return []
    parsed = json.loads(payload)
    if isinstance(parsed, list):
        return parsed
    if isinstance(parsed, dict):
        return [parsed]
    return []


def capture_codexapp_processes() -> list[dict[str, Any]]:
    script = (
        "$items = @("
        "Get-Process | "
        "Where-Object { $_.Path -like 'C:\\Program Files\\WindowsApps\\OpenAI.Codex_*' } | "
        "Select-Object ProcessName, Id, Path"
        "); "
        "$items | ConvertTo-Json -Compress"
    )
    return run_powershell_json(script)


def resolve_command_source(command_name: str) -> str | None:
    completed = subprocess.run(
        [
            "powershell",
            "-NoProfile",
            "-Command",
            f"$cmd = Get-Command {command_name} -ErrorAction SilentlyContinue; "
            "if ($cmd) { $cmd.Source | ConvertTo-Json -Compress }",
        ],
        cwd=REPO_ROOT,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=False,
    )
    payload = completed.stdout.strip()
    if not payload:
        return None
    return json.loads(payload)


def build_windows_install_command(
    args: argparse.Namespace, install_source: Path
) -> list[str]:
    script = " ".join(
        [
            f"& {powershell_single_quote(str(args.install_helper))}",
            f"-SourcePath {powershell_single_quote(str(install_source))}",
            f"-TargetPath {powershell_single_quote(str(args.install_target))}",
            "-Force",
            f"-ProcessNames {powershell_array_literal(WINDOWS_INSTALL_PROCESS_NAMES)}",
            f"-ExcludePathPrefixes {powershell_array_literal(args.codexapp_exclude_prefix)}",
        ]
    )
    return [
        "powershell",
        "-NoProfile",
        "-ExecutionPolicy",
        "Bypass",
        "-Command",
        script,
    ]


def install_on_windows(
    args: argparse.Namespace, logger: fast_build.Logger
) -> WindowsInstallOutcome:
    if not args.windows_install:
        logger.info("Skipping Windows install.")
        return WindowsInstallOutcome()
    if sys.platform != "win32":
        raise RuntimeError("--windows-install is only supported on Windows")
    install_source = resolve_install_source(args)
    if not install_source.exists():
        raise FileNotFoundError(f"Install source does not exist: {install_source}")
    if not args.install_helper.exists():
        raise FileNotFoundError(f"Install helper does not exist: {args.install_helper}")

    codexapp_before = capture_codexapp_processes()
    command = build_windows_install_command(args, install_source)

    logger.info(f"Windows install command: {' '.join(command)}")
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=False,
    )
    codexapp_after = capture_codexapp_processes()
    before_ids = {int(item["Id"]) for item in codexapp_before}
    after_ids = {int(item["Id"]) for item in codexapp_after}
    surviving = sorted(before_ids & after_ids)

    resolved_command_path = resolve_command_source("codex")
    version_output = ""
    app_server_help_ok = False
    success = completed.returncode == 0
    if success:
        version_proc = subprocess.run(
            ["codex", "--version"],
            cwd=REPO_ROOT,
            text=True,
            encoding="utf-8",
            errors="replace",
            capture_output=True,
            check=False,
        )
        version_output = (version_proc.stdout or version_proc.stderr).strip()
        help_proc = subprocess.run(
            ["codex", "app-server", "--help"],
            cwd=REPO_ROOT,
            text=True,
            encoding="utf-8",
            errors="replace",
            capture_output=True,
            check=False,
        )
        app_server_help_ok = help_proc.returncode == 0
        success = (
            success
            and args.install_target.exists()
            and resolved_command_path == str(args.install_target)
            and app_server_help_ok
            and (not before_ids or before_ids == set(surviving))
        )

    if success:
        logger.info(f"Windows install verified at {args.install_target}")
    else:
        logger.error("Windows install verification failed")

    return WindowsInstallOutcome(
        performed=True,
        success=success,
        command=command,
        returncode=completed.returncode,
        install_path=str(args.install_target),
        resolved_command_path=resolved_command_path,
        codexapp_before=codexapp_before,
        codexapp_after=codexapp_after,
        surviving_codexapp_pids=surviving,
        version_output=version_output,
        app_server_help_ok=app_server_help_ok,
        stdout_tail=tail_text(completed.stdout),
        stderr_tail=tail_text(completed.stderr),
    )


def build_markdown_report(payload: dict[str, Any]) -> str:
    validation_status = (
        "n/a"
        if not payload["validation"]["performed"]
        else ("yes" if payload["validation"]["success"] else "no")
    )
    build_status = (
        "n/a"
        if not payload["build_release"]["performed"]
        else ("yes" if payload["build_release"]["success"] else "no")
    )
    install_status = (
        "n/a"
        if not payload["windows_install"]["performed"]
        else ("yes" if payload["windows_install"]["success"] else "no")
    )
    lines = [
        "# Upstream-First Sync Driver Report",
        "",
        f"- Generated: `{payload['generated_at']}`",
        f"- Current branch: `{payload['current_branch']}`",
        f"- Integration base branch: `{payload['base_branch']}`",
        f"- Integration branch ref: `{payload['create_branch']}`",
        f"- Baseline ref: `{payload['baseline_ref']}`",
        f"- Merge ref: `{payload['merge_ref']}`",
        "",
        "## Summary",
        "",
        f"- Candidate upstream paths: **{payload['summary']['candidate_path_count']}**",
        *[
            f"- `{strategy}`: **{count}**"
            for strategy, count in payload["summary"]["classification_counts"].items()
        ],
        f"- Custom commits on `{payload['current_branch']}`: **{payload['summary']['custom_commit_count']}**",
        f"- Workspace members restored: **{payload['summary']['restored_member_count']}**",
        f"- Post-tag overlays applied: **{payload['summary']['overlay_member_count']}**",
        f"- Validation success: **{validation_status}**",
        f"- Release build success: **{build_status}**",
        f"- Windows install success: **{install_status}**",
        f"- Merge requested: **{'yes' if payload['merge']['performed'] else 'no'}**",
        f"- Initial conflicts: **{payload['summary']['initial_conflict_count']}**",
        f"- Unresolved conflicts: **{payload['summary']['unresolved_conflict_count']}**",
        "",
        "## Workspace Repair",
        "",
        f"- Performed: {'yes' if payload['workspace_repair']['performed'] else 'no'}",
        f"- Missing members detected: {len(payload['workspace_repair']['missing_members'])}",
        f"- Restored members: {len(payload['workspace_repair']['restored_members'])}",
        f"- Overlaid members: {len(payload['workspace_repair']['overlaid_members'])}",
        "",
    ]
    if payload["workspace_repair"]["restored_members"]:
        lines.extend(
            [
                *[
                    f"- `{member}`"
                    for member in payload["workspace_repair"]["restored_members"]
                ],
                "",
            ]
        )
    if payload["workspace_repair"]["missing_from_baseline"]:
        lines.extend(
            [
                "### Missing From Baseline",
                "",
                *[
                    f"- `{member}`"
                    for member in payload["workspace_repair"]["missing_from_baseline"]
                ],
                "",
            ]
        )
    lines.extend(
        [
            "## Classified Paths",
            "",
        ]
    )
    for strategy, title in fast_build.REPORT_CATEGORY_TITLES.items():
        lines.extend(
            [
                f"### {title}",
                "",
                *[
                    f"- `{path}`"
                    for path in payload["classifications"].get(strategy, [])[:160]
                ],
                "",
            ]
        )
    lines.extend(
        [
            "## Validation",
            "",
            f"- Performed: {'yes' if payload['validation']['performed'] else 'no'}",
            f"- Success: {'yes' if payload['validation']['success'] else 'no'}",
            f"- Blocked on environment: {'yes' if payload['validation']['blocked_on_environment'] else 'no'}",
            "",
        ]
    )
    if payload["validation"]["failure_summary"]:
        lines.extend(
            [
                "### Validation Note",
                "",
                f"- Kind: `{payload['validation']['failure_kind']}`",
                f"- Summary: {payload['validation']['failure_summary']}",
                "",
            ]
        )
    for step in payload["validation"]["steps"]:
        lines.extend(
            [
                f"### {step['name']}",
                "",
                f"- Status: `{step['status']}`",
                f"- Return code: `{step['returncode']}`",
                f"- Command: `{' '.join(step['command'])}`",
                "",
            ]
        )
        if step["failure_summary"]:
            lines.extend(
                [
                    f"- Failure kind: `{step['failure_kind']}`",
                    f"- Failure summary: {step['failure_summary']}",
                    "",
                ]
            )
    lines.extend(
        [
            "## Release Build",
            "",
            f"- Performed: {'yes' if payload['build_release']['performed'] else 'no'}",
            f"- Success: {'yes' if payload['build_release']['success'] else 'no'}",
        ]
    )
    if payload["build_release"]["binary_path"]:
        lines.append(f"- Binary: `{payload['build_release']['binary_path']}`")
    lines.extend(
        [
            "",
            "## Windows Install",
            "",
            f"- Performed: {'yes' if payload['windows_install']['performed'] else 'no'}",
            f"- Success: {'yes' if payload['windows_install']['success'] else 'no'}",
        ]
    )
    if payload["windows_install"]["install_path"]:
        lines.append(f"- Install path: `{payload['windows_install']['install_path']}`")
    if payload["windows_install"]["resolved_command_path"]:
        lines.append(
            f"- `Get-Command codex`: `{payload['windows_install']['resolved_command_path']}`"
        )
    if payload["windows_install"]["codexapp_before"]:
        lines.append(
            f"- CodexApp PIDs preserved: `{payload['windows_install']['surviving_codexapp_pids']}`"
        )
    if payload["windows_install"]["version_output"]:
        lines.append(
            f"- `codex --version`: `{payload['windows_install']['version_output']}`"
        )
    lines.extend(
        [
            "",
            "## Custom Commits",
            "",
            *[f"- `{entry}`" for entry in payload["custom_commits"][:120]],
            "",
            "## Range Diff",
            "",
            "```text",
            payload["range_diff"] or "(no range-diff output)",
            "```",
            "",
            "## Merge Outcome",
            "",
            f"- Merge performed: {'yes' if payload['merge']['performed'] else 'no'}",
            f"- Conflicts touched by resolver: {len(payload['merge']['conflicts'])}",
            f"- Remaining unresolved conflicts: {len(payload['merge']['unresolved_conflicts'])}",
            "",
        ]
    )
    if payload["merge"]["conflicts"]:
        lines.extend(
            [
                "### Resolved Conflict Paths",
                "",
                *[f"- `{path}`" for path in payload["merge"]["conflicts"]],
                "",
            ]
        )
    if payload["merge"]["unresolved_conflicts"]:
        lines.extend(
            [
                "### Unresolved Conflict Paths",
                "",
                *[f"- `{path}`" for path in payload["merge"]["unresolved_conflicts"]],
                "",
            ]
        )
    return "\n".join(lines).rstrip() + "\n"


def build_report_payload(
    *,
    args: argparse.Namespace,
    branch_name: str,
    candidate_paths: list[str],
    custom_commits: list[str],
    range_diff: str,
    merge: MergeOutcome,
    workspace_repair: WorkspaceRepairOutcome,
    validation: ValidationOutcome,
    build_release_outcome: BuildOutcome,
    windows_install: WindowsInstallOutcome,
) -> dict[str, Any]:
    classifications = fast_build.classify_paths(candidate_paths)
    classification_counts = {
        strategy: len(classifications.get(strategy, []))
        for strategy in fast_build.REPORT_CATEGORY_TITLES
    }
    return {
        "generated_at": fast_build.utc_now().isoformat(),
        "baseline_ref": args.baseline_ref,
        "merge_ref": merge_ref(args),
        "base_branch": args.base_branch,
        "create_branch": args.create_branch,
        "current_branch": branch_name,
        "candidate_paths": candidate_paths,
        "classifications": classifications,
        "custom_commits": custom_commits,
        "range_diff": range_diff,
        "workspace_repair": asdict(workspace_repair),
        "validation": asdict(validation),
        "build_release": asdict(build_release_outcome),
        "windows_install": asdict(windows_install),
        "summary": {
            "candidate_path_count": len(candidate_paths),
            "classification_counts": classification_counts,
            "custom_commit_count": len(custom_commits),
            "restored_member_count": len(workspace_repair.restored_members),
            "overlay_member_count": len(workspace_repair.overlaid_members),
            "initial_conflict_count": len(merge.conflicts),
            "unresolved_conflict_count": len(merge.unresolved_conflicts),
        },
        "merge": asdict(merge),
    }


def main() -> int:
    args = parse_args()
    logger = fast_build.Logger(args.log_file)

    branch_name = "UNKNOWN"
    candidate_paths: list[str] = []
    custom_commits: list[str] = []
    range_diff = ""
    merge = MergeOutcome()
    workspace_repair = WorkspaceRepairOutcome()
    validation = ValidationOutcome()
    build_release_outcome = BuildOutcome()
    windows_install = WindowsInstallOutcome()
    exit_code = 0

    try:
        fetch_refs(args, logger)
        verify_refs(args, logger)
        fast_build.ensure_branch_ref(args.create_branch, args.base_branch, logger)

        branch_name = current_branch(logger)
        target_ref = merge_ref(args)
        candidate_paths = collect_candidate_paths(args.baseline_ref, target_ref, logger)
        custom_commits = collect_custom_commits(target_ref, branch_name, logger)
        range_diff = collect_range_diff(
            target_ref, branch_name, args.base_branch, logger, enabled=args.merge
        )

        workspace_repair = repair_workspace(args, logger)
        if workspace_repair.performed and not workspace_repair.success:
            exit_code = 1

        if exit_code == 0:
            merge = perform_merge(args, logger)

        if exit_code == 0:
            validation = validate_repo(args, logger)
            if validation.performed and not validation.success:
                exit_code = 1

        if exit_code == 0:
            build_release_outcome = build_release(args, logger)
            if build_release_outcome.performed and not build_release_outcome.success:
                exit_code = 1

        if exit_code == 0:
            windows_install = install_on_windows(args, logger)
            if windows_install.performed and not windows_install.success:
                exit_code = 1
    except Exception as exc:
        logger.error(str(exc))
        exit_code = 1

    payload = build_report_payload(
        args=args,
        branch_name=branch_name,
        candidate_paths=candidate_paths,
        custom_commits=custom_commits,
        range_diff=range_diff,
        merge=merge,
        workspace_repair=workspace_repair,
        validation=validation,
        build_release_outcome=build_release_outcome,
        windows_install=windows_install,
    )
    fast_build.write_report(args.report_md, build_markdown_report(payload), logger)
    write_json(args.report_json, payload, logger)
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
