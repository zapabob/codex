#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import logging
import os
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from tqdm import tqdm

REPO_ROOT = Path(__file__).resolve().parents[1]
CODEX_RS_ROOT = Path(__file__).resolve().parent
DEFAULT_TARGET_DIR = Path(r"F:\codex-targets\codex-main-upstream-sync")
DEFAULT_CARGO_HOME = Path(r"H:\cargo-home\codex-main-upstream-sync")
FALLBACK_TARGET_DIR = Path(r"H:\codex-targets\codex-main-upstream-sync")
DEFAULT_INSTALL_PATH = Path.home() / ".cargo" / "bin" / "codex.exe"
DEFAULT_BACKUP_DIR = Path.home() / ".cargo" / "bin" / "backups"
DEFAULT_LOG_FILE = REPO_ROOT / "_docs" / "2026-03-27_fast_build_install.log"
DEFAULT_MD_LOG = REPO_ROOT / "_docs" / "2026-03-22_upstream-sync-v3.1.0_completion_log.md"
DEFAULT_ARTIFACT_DIR = REPO_ROOT / "artifacts" / "trusted-build" / "codex"
DEFAULT_ARTIFACT_MANIFEST = DEFAULT_ARTIFACT_DIR / "manifest.json"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Build Codex, export trusted-build artifacts, and install an artifact locally.",
    )
    parser.add_argument(
        "--mode",
        choices=("build-only", "install-only", "build-and-install"),
        default="build-and-install",
        help="Whether to build, install, or do both.",
    )
    parser.add_argument("--jobs", type=int, default=6, help="Cargo parallelism.")
    parser.add_argument("--target-dir", default=str(DEFAULT_TARGET_DIR), help="Requested CARGO_TARGET_DIR value.")
    parser.add_argument("--cargo-home", default=str(DEFAULT_CARGO_HOME), help="CARGO_HOME value.")
    parser.add_argument("--install-path", default=str(DEFAULT_INSTALL_PATH), help="Destination binary path.")
    parser.add_argument("--backup-dir", default=str(DEFAULT_BACKUP_DIR), help="Backup directory for replaced binaries.")
    parser.add_argument("--bin-name", default="codex", help="Cargo binary to build or install.")
    parser.add_argument(
        "--verify-args",
        nargs="*",
        default=["--version"],
        help="Arguments used to verify built and installed binaries.",
    )
    parser.add_argument("--artifact-dir", default=str(DEFAULT_ARTIFACT_DIR), help="Directory containing exported artifacts.")
    parser.add_argument(
        "--artifact-manifest",
        default=str(DEFAULT_ARTIFACT_MANIFEST),
        help="Artifact manifest JSON path.",
    )
    parser.add_argument("--log-file", default=str(DEFAULT_LOG_FILE), help="Detailed execution log file.")
    parser.add_argument(
        "--kill-pattern",
        action="append",
        default=["codex.exe", "codex-tui.exe", "codex-gui.exe", "codex-tauri-gui.exe"],
        help="Executable name pattern to terminate before install.",
    )
    parser.add_argument("--verbose", action="store_true", help="Enable debug logging.")
    return parser.parse_args()


def configure_logging(log_path: Path, verbose: bool) -> logging.Logger:
    log_path.parent.mkdir(parents=True, exist_ok=True)
    logger = logging.getLogger("fast_build_kill_install")
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


def can_execute_from(directory: Path, logger: logging.Logger) -> bool:
    directory.mkdir(parents=True, exist_ok=True)
    if os.name != "nt":
        return True

    probe_dir = directory / ".codex-exec-probe"
    probe_dir.mkdir(parents=True, exist_ok=True)
    system_root = Path(os.environ.get("SystemRoot", r"C:\Windows"))
    source_exe = system_root / "System32" / "where.exe"
    probe_exe = probe_dir / "probe.exe"
    shutil.copy2(source_exe, probe_exe)

    try:
        proc = subprocess.run(
            [str(probe_exe), "cmd.exe"],
            cwd=str(probe_dir),
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            check=False,
        )
        logger.info("Execution probe for %s returned exit=%s", directory, proc.returncode)
        return proc.returncode == 0
    except OSError as exc:
        logger.warning("Execution probe failed for %s: %s", directory, exc)
        return False
    finally:
        shutil.rmtree(probe_dir, ignore_errors=True)


def select_storage_paths(
    requested_target: Path,
    requested_cargo_home: Path,
    logger: logging.Logger,
) -> tuple[Path, Path]:
    temp_root = Path(tempfile.gettempdir()) / "codex-upstream-sync"
    temp_target = temp_root / "target"

    if os.name == "nt":
        target_candidates = [temp_target]
        if requested_target != temp_target:
            target_candidates.append(requested_target)
        if FALLBACK_TARGET_DIR not in target_candidates:
            target_candidates.append(FALLBACK_TARGET_DIR)
    else:
        target_candidates = [requested_target]
        if requested_target != FALLBACK_TARGET_DIR:
            target_candidates.append(FALLBACK_TARGET_DIR)
        if requested_target != temp_target:
            target_candidates.append(temp_target)

    for target_dir in target_candidates:
        if can_execute_from(target_dir, logger):
            requested_cargo_home.mkdir(parents=True, exist_ok=True)
            logger.info(
                "Using storage roots target_dir=%s cargo_home=%s",
                target_dir,
                requested_cargo_home,
            )
            return target_dir, requested_cargo_home

    raise RuntimeError(
        "Unable to find an executable target directory. Checked temp, requested, and fallback target locations."
    )


def hash_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def run_command(
    command: list[str],
    *,
    cwd: Path,
    env: dict[str, str],
    logger: logging.Logger,
) -> tuple[int, int]:
    logger.info("Running command: %s", " ".join(command))
    process = subprocess.Popen(
        command,
        cwd=str(cwd),
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        encoding="utf-8",
        errors="replace",
        bufsize=1,
        universal_newlines=True,
    )

    compiled: set[str] = set()
    progress = tqdm(desc="cargo build", unit="crate", dynamic_ncols=True, leave=True)

    assert process.stdout is not None
    for raw_line in process.stdout:
        line = raw_line.rstrip()
        if line.startswith("   Compiling "):
            crate_name = line.split()[1]
            if crate_name not in compiled:
                compiled.add(crate_name)
                progress.update(1)
                progress.set_postfix_str(crate_name)
        elif line.startswith("    Finished "):
            progress.set_postfix_str("finished")
            logger.info(line)
        elif "warning:" in line.lower():
            logger.warning(line)
        elif "error:" in line.lower():
            logger.error(line)
        else:
            logger.info(line)

    return_code = process.wait()
    progress.close()
    logger.info(
        "Command finished with exit code %s after compiling %s crates.",
        return_code,
        len(compiled),
    )
    return return_code, len(compiled)


def terminate_processes(patterns: list[str], logger: logging.Logger) -> None:
    for pattern in patterns:
        try:
            if sys.platform == "win32":
                proc = subprocess.run(
                    ["taskkill", "/F", "/IM", pattern, "/T"],
                    text=True,
                    capture_output=True,
                    encoding="utf-8",
                    errors="replace",
                )
                if proc.returncode == 0:
                    logger.info("Terminated process pattern %s", pattern)
                else:
                    stderr = proc.stderr.strip()
                    if "not found" not in stderr.lower() and "見つかりません" not in stderr:
                        logger.info(
                            "taskkill returned %s for %s: %s",
                            proc.returncode,
                            pattern,
                            stderr or proc.stdout.strip(),
                        )
            else:
                subprocess.run(["pkill", "-f", pattern], check=False, capture_output=True)
                logger.info("Sent pkill for %s", pattern)
        except Exception as exc:  # pragma: no cover - defensive logging
            logger.warning("Failed to terminate %s: %s", pattern, exc)


def backup_existing_binary(install_path: Path, backup_dir: Path, logger: logging.Logger) -> Path | None:
    if not install_path.exists():
        return None

    backup_dir.mkdir(parents=True, exist_ok=True)
    backup_path = backup_dir / f"{install_path.stem}-{datetime.now().strftime('%Y%m%d-%H%M%S')}{install_path.suffix}"
    shutil.copy2(install_path, backup_path)
    logger.info("Created backup: %s", backup_path)
    return backup_path


def append_markdown_log(path: Path, heading: str, message_lines: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8", newline="\n") as handle:
        handle.write(f"\n## {heading}\n")
        for line in message_lines:
            handle.write(f"- {line}\n")


def verify_binary(binary_path: Path, verify_args: list[str], logger: logging.Logger) -> tuple[int, str]:
    proc = subprocess.run(
        [str(binary_path), *verify_args],
        text=True,
        capture_output=True,
        encoding="utf-8",
        errors="replace",
    )
    output = (proc.stdout or proc.stderr).strip()
    logger.info("Verification exit=%s output=%s", proc.returncode, output)
    return proc.returncode, output


def load_manifest(path: Path, logger: logging.Logger) -> dict[str, Any]:
    if not path.exists():
        logger.warning("Artifact manifest does not exist: %s", path)
        return {}

    with path.open("r", encoding="utf-8") as handle:
        manifest = json.load(handle)
    logger.info("Loaded artifact manifest from %s", path)
    return manifest


def write_manifest(path: Path, manifest: dict[str, Any], logger: logging.Logger) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="\n") as handle:
        json.dump(manifest, handle, ensure_ascii=True, indent=2)
        handle.write("\n")
    logger.info("Wrote artifact manifest to %s", path)


def export_artifact(
    source_binary: Path,
    artifact_dir: Path,
    manifest_path: Path,
    verify_output: str,
    target_dir: Path,
    cargo_home: Path,
    logger: logging.Logger,
) -> tuple[Path, dict[str, Any]]:
    artifact_dir.mkdir(parents=True, exist_ok=True)
    artifact_binary = artifact_dir / source_binary.name
    shutil.copy2(source_binary, artifact_binary)
    manifest = {
        "artifact_binary": str(artifact_binary),
        "source_binary": str(source_binary),
        "version_output": verify_output,
        "source_commit": os.environ.get("GIT_COMMIT", ""),
        "target_dir": str(target_dir),
        "cargo_home": str(cargo_home),
        "built_at_utc": datetime.now(timezone.utc).isoformat(),
        "sha256": hash_file(artifact_binary),
    }
    write_manifest(manifest_path, manifest, logger)
    return artifact_binary, manifest


def resolve_artifact_binary(artifact_dir: Path, bin_name: str, manifest: dict[str, Any], logger: logging.Logger) -> Path:
    artifact_binary = artifact_dir / f"{bin_name}.exe"
    manifest_binary = manifest.get("artifact_binary")
    if manifest_binary:
        candidate = Path(str(manifest_binary))
        if candidate.exists():
            artifact_binary = candidate

    if not artifact_binary.exists():
        logger.error("Artifact binary not found: %s", artifact_binary)
        raise FileNotFoundError(f"Artifact binary not found: {artifact_binary}")

    return artifact_binary


def install_binary(
    source_binary: Path,
    install_path: Path,
    backup_dir: Path,
    kill_patterns: list[str],
    verify_args: list[str],
    manifest: dict[str, Any],
    logger: logging.Logger,
) -> tuple[int, str]:
    install_path.parent.mkdir(parents=True, exist_ok=True)
    source_hash = hash_file(source_binary)
    destination_hash = hash_file(install_path) if install_path.exists() else None
    if destination_hash == source_hash:
        logger.info("Installed binary is already up to date; skipping copy.")
        return verify_binary(install_path, verify_args, logger)

    terminate_processes(kill_patterns, logger)
    backup_path = backup_existing_binary(install_path, backup_dir, logger)

    try:
        shutil.copy2(source_binary, install_path)
        logger.info("Installed %s -> %s", source_binary, install_path)
    except Exception as exc:
        logger.error("Install copy failed: %s", exc)
        if backup_path and backup_path.exists():
            shutil.copy2(backup_path, install_path)
            logger.info("Restored backup after failed install: %s", backup_path)
        raise

    verify_code, verify_output = verify_binary(install_path, verify_args, logger)
    expected_version = str(manifest.get("version_output", "")).strip()
    if expected_version and expected_version not in verify_output:
        raise RuntimeError(
            f"Installed binary version mismatch. expected fragment={expected_version!r} actual={verify_output!r}"
        )
    return verify_code, verify_output


def run_build(
    args: argparse.Namespace,
    logger: logging.Logger,
) -> tuple[int, int, Path, Path, Path, str]:
    requested_target_dir = Path(args.target_dir)
    requested_cargo_home = Path(args.cargo_home)
    target_dir, cargo_home = select_storage_paths(requested_target_dir, requested_cargo_home, logger)
    build_binary = target_dir / "release" / f"{args.bin_name}.exe"

    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(target_dir)
    env["CARGO_HOME"] = str(cargo_home)

    target_dir.mkdir(parents=True, exist_ok=True)
    cargo_home.mkdir(parents=True, exist_ok=True)

    return_code, compiled_count = run_command(
        ["cargo", "build", "--bin", args.bin_name, "--release", "-j", str(args.jobs)],
        cwd=CODEX_RS_ROOT,
        env=env,
        logger=logger,
    )
    if return_code != 0:
        return return_code, compiled_count, requested_target_dir, cargo_home, build_binary, ""

    if not build_binary.exists():
        logger.error("Built binary not found: %s", build_binary)
        return 2, compiled_count, requested_target_dir, cargo_home, build_binary, ""

    verify_code, verify_output = verify_binary(build_binary, args.verify_args, logger)
    if verify_code != 0:
        return verify_code, compiled_count, requested_target_dir, cargo_home, build_binary, verify_output

    export_artifact(
        build_binary,
        Path(args.artifact_dir),
        Path(args.artifact_manifest),
        verify_output,
        target_dir,
        cargo_home,
        logger,
    )
    return 0, compiled_count, requested_target_dir, cargo_home, build_binary, verify_output


def main() -> int:
    args = parse_args()
    logger = configure_logging(Path(args.log_file), args.verbose)

    artifact_dir = Path(args.artifact_dir)
    artifact_manifest_path = Path(args.artifact_manifest)
    install_path = Path(args.install_path)
    backup_dir = Path(args.backup_dir)

    if args.mode in {"build-only", "build-and-install"}:
        build_exit, compiled_count, requested_target_dir, cargo_home, build_binary, verify_output = run_build(args, logger)
        if build_exit != 0:
            append_markdown_log(
                Path(DEFAULT_MD_LOG),
                "2026-03-28 Fast Build Install Pass",
                [
                    f"mode=`{args.mode}`",
                    f"Fast build failed before install. exit_code=`{build_exit}`",
                    f"requested_target_dir=`{requested_target_dir}`",
                    f"cargo_home=`{cargo_home}`",
                    f"log_file=`{Path(args.log_file)}`",
                ],
            )
            return build_exit
    else:
        compiled_count = 0
        requested_target_dir = Path(args.target_dir)
        cargo_home = Path(args.cargo_home)
        build_binary = Path()
        verify_output = ""

    if args.mode == "build-only":
        append_markdown_log(
            Path(DEFAULT_MD_LOG),
            "2026-03-28 Fast Build Install Pass",
            [
                f"mode=`{args.mode}`",
                f"artifact_dir=`{artifact_dir}`",
                f"artifact_manifest=`{artifact_manifest_path}`",
                f"compiled_crates=`{compiled_count}`",
                f"verify_output=`{verify_output}`",
                f"log_file=`{Path(args.log_file)}`",
            ],
        )
        return 0

    manifest = load_manifest(artifact_manifest_path, logger)
    source_binary = build_binary if args.mode == "build-and-install" else resolve_artifact_binary(
        artifact_dir,
        args.bin_name,
        manifest,
        logger,
    )
    if args.mode == "build-and-install" and not manifest:
        manifest = load_manifest(artifact_manifest_path, logger)

    verify_code, installed_output = install_binary(
        source_binary,
        install_path,
        backup_dir,
        args.kill_pattern,
        args.verify_args,
        manifest,
        logger,
    )

    append_markdown_log(
        Path(DEFAULT_MD_LOG),
        "2026-03-28 Fast Build Install Pass",
        [
            f"mode=`{args.mode}`",
            f"requested_target_dir=`{requested_target_dir}`",
            f"cargo_home=`{cargo_home}`",
            f"artifact_dir=`{artifact_dir}`",
            f"artifact_manifest=`{artifact_manifest_path}`",
            f"install_path=`{install_path}`",
            f"compiled_crates=`{compiled_count}`",
            f"verify_exit=`{verify_code}`",
            f"verify_output=`{installed_output}`",
            f"log_file=`{Path(args.log_file)}`",
        ],
    )
    return verify_code


if __name__ == "__main__":
    raise SystemExit(main())
