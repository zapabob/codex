#!/usr/bin/env python3
from __future__ import annotations

import argparse
import fnmatch
import json
import logging
import os
import re
import subprocess
import sys
from dataclasses import asdict, dataclass
from datetime import datetime
from pathlib import Path
from typing import Iterable

from tqdm import tqdm

REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_MD_LOG = REPO_ROOT / "_docs" / "2026-03-22_upstream-sync-v3.1.0_completion_log.md"
DEFAULT_JSONL_LOG = REPO_ROOT / "_docs" / "2026-03-27_merge_resolution.jsonl"
CONFLICT_RE = re.compile(
    r"^<<<<<<< .*?\n(?P<local>.*?)^=======\n(?P<upstream>.*?)^>>>>>>> .*?$",
    re.MULTILINE | re.DOTALL,
)


@dataclass(frozen=True)
class Rule:
    pattern: str
    strategy: str


@dataclass
class ResolutionRecord:
    path: str
    status: str
    strategy: str
    action: str
    detail: str


DEFAULT_RULES = [
    Rule(".github/workflows/shell-tool-mcp*", "upstream"),
    Rule("shell-tool-mcp/**", "upstream"),
    Rule("pnpm-lock.yaml", "upstream"),
    Rule("codex-rs/Cargo.lock", "upstream"),
    Rule("codex-rs/**/Cargo.toml", "upstream-reinject"),
    Rule("codex-rs/app-server/**", "upstream-reinject"),
    Rule("codex-rs/app-server-protocol/**", "upstream-reinject"),
    Rule("codex-rs/exec-server/**", "upstream-reinject"),
    Rule("codex-rs/exec/**", "upstream-reinject"),
    Rule("codex-rs/login/**", "upstream-reinject"),
    Rule("codex-rs/protocol/**", "upstream-reinject"),
    Rule("codex-rs/windows-sandbox-rs/**", "upstream-reinject"),
    Rule("codex-rs/core/src/tools/handlers/multi_agents/**", "custom-reinject"),
    Rule("codex-rs/core/src/tools/handlers/multi_agents.rs", "custom-reinject"),
    Rule("codex-rs/core/src/tools/handlers/multi_agents_tests.rs", "custom-reinject"),
    Rule("codex-rs/core/src/tools/handlers/unified_exec.rs", "custom-reinject"),
    Rule("codex-rs/core/src/tools/spec*.rs", "custom-reinject"),
    Rule("codex-rs/core/tests/common/test_codex.rs", "custom-reinject"),
    Rule("codex-rs/core/tests/suite/view_image.rs", "custom-reinject"),
    Rule("codex-rs/core-skills/**", "custom"),
    Rule("codex-rs/git-utils/**", "custom"),
    Rule("codex-rs/rollout/report.md", "custom"),
    Rule("codex-rs/deep-research/**", "custom-reinject"),
    Rule("AGENTS.md", "custom-reinject"),
    Rule("CHANGELOG.md", "custom-reinject"),
    Rule("CLAUDE.md", "custom-reinject"),
    Rule("README.md", "custom-reinject"),
]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Resolve merge conflicts using upstream/custom rules with logging and tqdm.",
    )
    parser.add_argument("paths", nargs="*", help="Specific conflicted paths to resolve.")
    parser.add_argument("--worktree", default=str(REPO_ROOT), help="Repository root to operate on.")
    parser.add_argument("--upstream-ref", default="upstream/main", help="Reference used for upstream restores.")
    parser.add_argument("--prefer-upstream", action="append", default=[], help="Glob pattern resolved as upstream.")
    parser.add_argument("--prefer-custom", action="append", default=[], help="Glob pattern resolved as custom.")
    parser.add_argument("--log-md", default=str(DEFAULT_MD_LOG), help="Markdown log file to append.")
    parser.add_argument("--log-jsonl", default=str(DEFAULT_JSONL_LOG), help="JSONL log file to append.")
    parser.add_argument("--fail-on-unresolved", action="store_true", help="Exit non-zero if any conflict remains.")
    parser.add_argument("--verbose", action="store_true", help="Enable debug logging.")
    return parser.parse_args()


def configure_logging(verbose: bool) -> logging.Logger:
    logger = logging.getLogger("resolve_merge_conflicts")
    logger.setLevel(logging.DEBUG if verbose else logging.INFO)
    logger.handlers.clear()

    formatter = logging.Formatter("%(asctime)s %(levelname)s %(message)s")
    handler = logging.StreamHandler(sys.stderr)
    handler.setFormatter(formatter)
    handler.setLevel(logging.DEBUG if verbose else logging.INFO)
    logger.addHandler(handler)
    return logger


def run_git(repo_root: Path, args: list[str], check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["git", "-C", str(repo_root), *args],
        check=check,
        text=True,
        capture_output=True,
        encoding="utf-8",
        errors="replace",
    )


def load_rules(prefer_upstream: list[str], prefer_custom: list[str]) -> list[Rule]:
    rules = DEFAULT_RULES.copy()
    for pattern in reversed(prefer_upstream):
        rules.insert(0, Rule(pattern, "upstream"))
    for pattern in reversed(prefer_custom):
        rules.insert(0, Rule(pattern, "custom"))
    return rules


def choose_strategy(path: str, rules: list[Rule]) -> str:
    for rule in rules:
        if fnmatch.fnmatch(path, rule.pattern):
            return rule.strategy
    return "upstream-reinject"


def list_conflicts(repo_root: Path) -> list[str]:
    proc = run_git(repo_root, ["diff", "--name-only", "--diff-filter=U"])
    return [line.strip() for line in proc.stdout.splitlines() if line.strip()]


def get_status_code(repo_root: Path, rel_path: str) -> str:
    proc = run_git(repo_root, ["status", "--porcelain=v1", "--", rel_path])
    line = next((entry for entry in proc.stdout.splitlines() if entry.strip()), "")
    return line[:2] if line else ""


def unique_local_lines(local: str, upstream: str) -> list[str]:
    upstream_lines = {line.rstrip() for line in upstream.splitlines() if line.strip()}
    kept: list[str] = []
    seen: set[str] = set()
    for line in local.splitlines():
        stripped = line.rstrip()
        if not stripped or stripped in upstream_lines or stripped in seen:
            continue
        seen.add(stripped)
        kept.append(line)
    return kept


def resolve_block(local: str, upstream: str, strategy: str) -> str:
    if strategy == "custom":
        return local
    if strategy == "upstream":
        return upstream

    if strategy == "custom-reinject":
        reinjected = unique_local_lines(upstream, local)
        body = local.rstrip("\n")
        if reinjected:
            body = f"{body}\n" + "\n".join(reinjected)
        return f"{body.rstrip()}\n"

    reinjected = unique_local_lines(local, upstream)
    body = upstream.rstrip("\n")
    if reinjected:
        body = f"{body}\n" + "\n".join(reinjected)
    return f"{body.rstrip()}\n"


def resolve_text_conflict(path: Path, strategy: str) -> tuple[str, str]:
    original = path.read_text(encoding="utf-8")

    def repl(match: re.Match[str]) -> str:
        return resolve_block(match.group("local"), match.group("upstream"), strategy)

    updated, count = CONFLICT_RE.subn(repl, original)
    if count == 0:
        return "skip", "no merge markers found"

    path.write_text(updated, encoding="utf-8", newline="\n")
    return "rewrite", f"resolved {count} conflict block(s)"


def resolve_non_text_conflict(repo_root: Path, rel_path: str, status: str, strategy: str, upstream_ref: str) -> tuple[str, str]:
    if status in {"UD", "DU"}:
        if strategy.startswith("upstream"):
            run_git(repo_root, ["rm", "--", rel_path])
            return "git-rm", "accepted upstream deletion"
        run_git(repo_root, ["add", "--", rel_path])
        return "git-add", "kept custom file"

    if status in {"AU", "UA", "AA"}:
        if strategy.startswith("upstream") and not rel_path.startswith("codex-rs/core-skills/") and not rel_path.startswith("codex-rs/git-utils/"):
            proc = run_git(repo_root, ["show", f"{upstream_ref}:{rel_path}"], check=False)
            if proc.returncode == 0:
                (repo_root / rel_path).write_text(proc.stdout, encoding="utf-8", newline="\n")
                run_git(repo_root, ["add", "--", rel_path])
                return "restore-upstream", "restored path from upstream"
        run_git(repo_root, ["add", "--", rel_path])
        return "git-add", "kept relocated/custom-added path"

    return "skip", f"unsupported status {status}"


def append_jsonl(path: Path, records: Iterable[ResolutionRecord]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8", newline="\n") as handle:
        for record in records:
            handle.write(json.dumps(asdict(record), ensure_ascii=False) + "\n")


def append_markdown(path: Path, records: list[ResolutionRecord], unresolved: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    lines = [
        "",
        "## 2026-03-27 Merge Resolution Pass",
        f"- Timestamp: `{timestamp}`",
        f"- Resolved entries: `{len(records)}`",
        f"- Unresolved entries: `{len(unresolved)}`",
        "",
        "| Path | Status | Strategy | Action | Detail |",
        "| --- | --- | --- | --- | --- |",
    ]
    for record in records:
        lines.append(
            f"| `{record.path}` | `{record.status}` | `{record.strategy}` | `{record.action}` | {record.detail} |"
        )
    if unresolved:
        lines.extend(["", "### Remaining Unresolved", ""])
        lines.extend([f"- `{item}`" for item in unresolved])
    with path.open("a", encoding="utf-8", newline="\n") as handle:
        handle.write("\n".join(lines) + "\n")


def main() -> int:
    args = parse_args()
    logger = configure_logging(args.verbose)
    repo_root = Path(args.worktree).resolve()
    rules = load_rules(args.prefer_upstream, args.prefer_custom)

    paths = args.paths or list_conflicts(repo_root)
    if not paths:
        logger.info("No conflicted paths found.")
        return 0

    records: list[ResolutionRecord] = []
    unresolved: list[str] = []

    for rel_path in tqdm(paths, desc="Resolving conflicts", unit="file", dynamic_ncols=True):
        strategy = choose_strategy(rel_path, rules)
        status = get_status_code(repo_root, rel_path)
        path = repo_root / rel_path
        logger.info("Resolving %s with strategy=%s status=%s", rel_path, strategy, status or "??")

        try:
            if path.exists():
                action, detail = resolve_text_conflict(path, strategy)
                if action == "rewrite":
                    run_git(repo_root, ["add", "--", rel_path])
                elif status:
                    action, detail = resolve_non_text_conflict(repo_root, rel_path, status, strategy, args.upstream_ref)
            else:
                action, detail = resolve_non_text_conflict(repo_root, rel_path, status, strategy, args.upstream_ref)

            records.append(
                ResolutionRecord(
                    path=rel_path,
                    status=status,
                    strategy=strategy,
                    action=action,
                    detail=detail,
                )
            )
            logger.info("%s -> %s (%s)", rel_path, action, detail)
        except subprocess.CalledProcessError as exc:
            unresolved.append(rel_path)
            stderr = exc.stderr.strip() or exc.stdout.strip() or str(exc)
            records.append(
                ResolutionRecord(
                    path=rel_path,
                    status=status,
                    strategy=strategy,
                    action="error",
                    detail=stderr[:200],
                )
            )
            logger.error("Failed to resolve %s: %s", rel_path, stderr)

    remaining = list_conflicts(repo_root)
    unresolved = sorted(set(unresolved + remaining))
    append_jsonl(Path(args.log_jsonl), records)
    append_markdown(Path(args.log_md), records, unresolved)
    logger.info("Resolved records=%s unresolved=%s", len(records), len(unresolved))

    if unresolved and args.fail_on_unresolved:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
