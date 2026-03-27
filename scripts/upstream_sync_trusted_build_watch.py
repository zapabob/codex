#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import logging
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
CODEX_RS_ROOT = REPO_ROOT / "codex-rs"
CANONICAL_LOG = REPO_ROOT / "_docs" / "2026-03-22_upstream-sync-v3.1.0_completion_log.md"
LOG_FILE = REPO_ROOT / "_docs" / "2026-03-28_upstream_sync_trusted_build_watch.log"
ARTIFACT_DIR = REPO_ROOT / "artifacts" / "trusted-build" / "codex"
ARTIFACT_MANIFEST = ARTIFACT_DIR / "manifest.json"
INSTALL_SCRIPT = CODEX_RS_ROOT / "fast_build_kill_install.py"
ACTIVE_MARKER_EXCLUDES = (
    "_docs/",
    "archive/",
    ".specstory/",
    "docs/",
    "fixtures/",
    "scripts/upstream_sync_trusted_build_watch.py",
    "scripts/resolve_merge_conflicts.py",
)
INTENTIONAL_MARKER_PATTERNS = (
    "contents.contains(\"<<<<<<< HEAD\")",
    "merged.push_str(\"<<<<<<< Agent: \")",
    "r\"^<<<<<<< .*?",
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Monitor upstream-sync trusted-build status and install trusted artifacts when available.",
    )
    parser.add_argument("--workspace-root", default=str(REPO_ROOT))
    parser.add_argument("--canonical-log", default=str(CANONICAL_LOG))
    parser.add_argument("--log-file", default=str(LOG_FILE))
    parser.add_argument("--artifact-dir", default=str(ARTIFACT_DIR))
    parser.add_argument("--artifact-manifest", default=str(ARTIFACT_MANIFEST))
    parser.add_argument("--install-script", default=str(INSTALL_SCRIPT))
    parser.add_argument("--install-path", default=str(Path.home() / ".cargo" / "bin" / "codex.exe"))
    parser.add_argument("--verbose", action="store_true")
    return parser.parse_args()


def configure_logging(log_path: Path, verbose: bool) -> logging.Logger:
    log_path.parent.mkdir(parents=True, exist_ok=True)
    logger = logging.getLogger("upstream_sync_trusted_build_watch")
    logger.setLevel(logging.DEBUG if verbose else logging.INFO)
    logger.handlers.clear()

    formatter = logging.Formatter("%(asctime)s %(levelname)s %(message)s")

    file_handler = logging.FileHandler(log_path, encoding="utf-8")
    file_handler.setFormatter(formatter)
    file_handler.setLevel(logging.DEBUG)
    logger.addHandler(file_handler)

    stream_handler = logging.StreamHandler(sys.stderr)
    stream_handler.setFormatter(formatter)
    stream_handler.setLevel(logging.DEBUG if verbose else logging.INFO)
    logger.addHandler(stream_handler)
    return logger


def run_command(command: list[str], cwd: Path, logger: logging.Logger) -> subprocess.CompletedProcess[str]:
    logger.info("Running command: %s", " ".join(command))
    return subprocess.run(
        command,
        cwd=str(cwd),
        text=True,
        capture_output=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )


def get_git_status(workspace_root: Path, logger: logging.Logger) -> list[str]:
    result = run_command(["git", "status", "--short"], workspace_root, logger)
    if result.returncode != 0:
        logger.warning("git status failed: %s", result.stderr.strip())
        return []
    return [line for line in result.stdout.splitlines() if line.strip()]


def get_unresolved_conflicts(workspace_root: Path, logger: logging.Logger) -> list[str]:
    result = run_command(["git", "diff", "--name-only", "--diff-filter=U"], workspace_root, logger)
    if result.returncode != 0:
        logger.warning("git diff --diff-filter=U failed: %s", result.stderr.strip())
        return []
    return [line for line in result.stdout.splitlines() if line.strip()]


def scan_real_markers(workspace_root: Path, logger: logging.Logger) -> list[str]:
    result = run_command(["git", "grep", "-n", "<<<<<<< ", "--", "."], workspace_root, logger)
    if result.returncode not in (0, 1):
        logger.warning("git grep for markers failed: %s", result.stderr.strip())
        return []

    findings: list[str] = []
    for line in result.stdout.splitlines():
        if not line.strip():
            continue
        if any(part in line for part in ACTIVE_MARKER_EXCLUDES):
            continue
        if any(pattern in line for pattern in INTENTIONAL_MARKER_PATTERNS):
            continue
        findings.append(line)
    return findings


def latest_build_script_failure(logger: logging.Logger) -> str:
    candidates = [
        Path.home() / "AppData" / "Local" / "Temp" / "codex-upstream-sync" / "target" / "debug" / "build",
        Path(r"F:\codex-targets\codex-main-upstream-sync\debug\build"),
        Path(r"H:\codex-targets\codex-main-upstream-sync\debug\build"),
    ]

    for root in candidates:
        if not root.exists():
            continue
        executables = sorted(root.rglob("build-script-build.exe"), key=lambda item: item.stat().st_mtime, reverse=True)
        if not executables:
            continue
        latest = executables[0]
        logger.info("Testing latest build-script executable: %s", latest)
        try:
            subprocess.run(
                [str(latest)],
                cwd=str(latest.parent),
                text=True,
                capture_output=True,
                encoding="utf-8",
                errors="replace",
                check=False,
            )
            return f"latest build-script executable launched successfully from `{latest}`"
        except OSError as exc:
            return f"latest build-script executable still blocked at `{latest}` with `{exc}`"

    return "no build-script executable found to probe"


def load_manifest(path: Path, logger: logging.Logger) -> dict[str, object]:
    if not path.exists():
        logger.info("Artifact manifest not found: %s", path)
        return {}
    with path.open("r", encoding="utf-8") as handle:
        manifest = json.load(handle)
    logger.info("Loaded artifact manifest: %s", path)
    return manifest


def install_trusted_artifact(
    workspace_root: Path,
    install_script: Path,
    artifact_dir: Path,
    artifact_manifest: Path,
    install_path: Path,
    logger: logging.Logger,
) -> tuple[int, str]:
    command = [
        "py",
        "-3",
        str(install_script),
        "--mode",
        "install-only",
        "--artifact-dir",
        str(artifact_dir),
        "--artifact-manifest",
        str(artifact_manifest),
        "--install-path",
        str(install_path),
    ]
    result = run_command(command, workspace_root / "codex-rs", logger)
    combined = "\n".join(filter(None, [result.stdout.strip(), result.stderr.strip()])).strip()
    return result.returncode, combined


def append_markdown_log(path: Path, lines: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    heading = f"2026-03-28 Trusted Build Watch {datetime.now().strftime('%H:%M:%S')}"
    with path.open("a", encoding="utf-8", newline="\n") as handle:
        handle.write(f"\n## {heading}\n")
        for line in lines:
            handle.write(f"- {line}\n")


def main() -> int:
    args = parse_args()
    workspace_root = Path(args.workspace_root)
    canonical_log = Path(args.canonical_log)
    artifact_dir = Path(args.artifact_dir)
    artifact_manifest = Path(args.artifact_manifest)
    install_script = Path(args.install_script)
    install_path = Path(args.install_path)
    logger = configure_logging(Path(args.log_file), args.verbose)

    status_lines = get_git_status(workspace_root, logger)
    unresolved = get_unresolved_conflicts(workspace_root, logger)
    markers = scan_real_markers(workspace_root, logger)
    blocker_probe = latest_build_script_failure(logger)

    manifest = load_manifest(artifact_manifest, logger)
    artifact_binary = artifact_dir / "codex.exe"
    artifact_available = artifact_binary.exists() and artifact_manifest.exists()

    log_lines = [
        f"run_at_utc=`{datetime.now(timezone.utc).isoformat()}`",
        f"workspace_root=`{workspace_root}`",
        f"git_status_entries=`{len(status_lines)}`",
        f"unresolved_conflicts=`{len(unresolved)}`",
        f"real_marker_findings=`{len(markers)}`",
        blocker_probe,
    ]

    if unresolved:
        log_lines.append(f"unresolved_paths=`{'; '.join(unresolved[:10])}`")
    if markers:
        log_lines.append(f"marker_paths=`{'; '.join(markers[:10])}`")

    if artifact_available:
        install_exit, install_output = install_trusted_artifact(
            workspace_root,
            install_script,
            artifact_dir,
            artifact_manifest,
            install_path,
            logger,
        )
        log_lines.append(f"trusted_artifact_status=`ready`")
        log_lines.append(f"install_exit=`{install_exit}`")
        if manifest.get("version_output"):
            log_lines.append(f"expected_version=`{manifest['version_output']}`")
        if install_output:
            log_lines.append(f"install_output=`{install_output[:400].replace(chr(10), ' | ')}`")
        append_markdown_log(canonical_log, log_lines)
        return install_exit

    log_lines.append("trusted_artifact_status=`missing`")
    if manifest:
        log_lines.append("artifact_manifest_present=`true`")
    else:
        log_lines.append("artifact_manifest_present=`false`")
    log_lines.append(f"artifact_dir=`{artifact_dir}`")
    log_lines.append(f"artifact_manifest=`{artifact_manifest}`")
    append_markdown_log(canonical_log, log_lines)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
