#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
from dataclasses import asdict
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
SCRIPTS_DIR = Path(__file__).resolve().parent
if str(SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPTS_DIR))

import fast_build


DEFAULT_BASELINE_REF = "64177aaa222738e2372cdf7f929388883b483094"
DEFAULT_UPSTREAM_REF = "upstream/main"
DEFAULT_REPORT_JSON = REPO_ROOT / "_docs" / "upstream-overlay-merge-report.json"
DEFAULT_REPORT_MD = REPO_ROOT / "_docs" / "upstream-overlay-merge-report.md"

SKIP_STRATEGIES = {
    "keep-fork",
    "plugin-migrate",
    "retire-after-parity",
}

REINJECT_PATHS = {
    "codex-rs/app-server-protocol/src/lib.rs",
    "codex-rs/app-server-protocol/src/protocol/common.rs",
    "codex-rs/app-server-protocol/src/protocol/mod.rs",
    "codex-rs/app-server/src/codex_message_processor.rs",
    "codex-rs/app-server/src/lib.rs",
    "codex-rs/app-server/tests/common/mcp_process.rs",
    "codex-rs/app-server/tests/suite/v2/experimental_api.rs",
    "codex-rs/app-server/tests/suite/v2/mod.rs",
    "codex-rs/app-server/tests/suite/v2/plugin_list.rs",
    "codex-rs/app-server/tests/suite/v2/plugin_read.rs",
    "codex-rs/core/src/lib.rs",
}

KEEP_FORK_PATHS = {
    "README.md",
    "package.json",
}


@dataclass(frozen=True)
class PathPlan:
    path: str
    status: str
    strategy: str
    action: str
    reason: str
    ours_changed: bool
    baseline_exists: bool
    upstream_exists: bool


@dataclass(frozen=True)
class PathOutcome:
    path: str
    action: str
    returncode: int = 0
    conflicted: bool = False
    note: str = ""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Overlay-merge upstream changes using an explicit official baseline, "
            "the current fork tree, and the latest upstream tree."
        )
    )
    parser.add_argument("--baseline-ref", default=DEFAULT_BASELINE_REF)
    parser.add_argument("--upstream-ref", default=DEFAULT_UPSTREAM_REF)
    parser.add_argument("--report-json", type=Path, default=DEFAULT_REPORT_JSON)
    parser.add_argument("--report-md", type=Path, default=DEFAULT_REPORT_MD)
    parser.add_argument("--apply", action="store_true", help="Write planned changes")
    parser.add_argument("--allow-dirty", action="store_true")
    parser.add_argument("--limit", type=int, help="Only process the first N paths")
    return parser.parse_args()


def run_git(args: list[str], *, check: bool = True, input_bytes: bytes | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(
        ["git", *args],
        cwd=REPO_ROOT,
        input=input_bytes,
        capture_output=True,
        check=check,
    )


def git_text(args: list[str]) -> str:
    return run_git(args).stdout.decode("utf-8", errors="replace")


def git_lines(args: list[str]) -> list[str]:
    return [line for line in git_text(args).splitlines() if line.strip()]


def read_ref(ref: str, path: str) -> bytes:
    return run_git(["show", f"{ref}:{path}"]).stdout


def is_binary(data: bytes) -> bool:
    return b"\0" in data[:8192]


def tree_paths(ref: str) -> set[str]:
    return set(git_lines(["ls-tree", "-r", "--name-only", ref]))


def local_changed_paths(baseline_ref: str) -> set[str]:
    return set(git_lines(["diff", "--name-only", baseline_ref, "--"]))


def changed_paths(baseline_ref: str, upstream_ref: str) -> list[tuple[str, str]]:
    rows: list[tuple[str, str]] = []
    for line in git_lines(["diff", "--name-status", "--find-renames", f"{baseline_ref}..{upstream_ref}"]):
        parts = line.split("\t")
        status = parts[0]
        if status.startswith(("R", "C")) and len(parts) >= 3:
            rows.append((status[0], parts[2]))
        elif len(parts) >= 2:
            rows.append((status[0], parts[1]))
    return rows


def strategy_for(path: str) -> str:
    if path in KEEP_FORK_PATHS:
        return "keep-fork"
    if path in REINJECT_PATHS:
        return "upstream-plus-reinject"
    if path.startswith(".codex/skills/"):
        return "upstream-first"
    return fast_build.classify_strategy(path)


def plan_path(
    *,
    status: str,
    path: str,
    baseline_paths: set[str],
    upstream_paths: set[str],
    local_changes: set[str],
) -> PathPlan:
    baseline_exists = path in baseline_paths
    upstream_exists = path in upstream_paths
    ours_changed = path in local_changes
    strategy = strategy_for(path)

    if strategy in SKIP_STRATEGIES:
        return PathPlan(path, status, strategy, "skip", "fork strategy keeps this path", ours_changed, baseline_exists, upstream_exists)
    if status == "D" and ours_changed:
        return PathPlan(path, status, strategy, "skip", "upstream deleted but fork changed it", ours_changed, baseline_exists, upstream_exists)
    if status == "D":
        return PathPlan(path, status, strategy, "delete", "upstream deleted unchanged path", ours_changed, baseline_exists, upstream_exists)
    if not ours_changed:
        return PathPlan(path, status, strategy, "checkout-upstream", "fork did not change this path", ours_changed, baseline_exists, upstream_exists)
    if not baseline_exists or not upstream_exists:
        return PathPlan(path, status, strategy, "checkout-upstream", "added path has no three-way base", ours_changed, baseline_exists, upstream_exists)
    return PathPlan(path, status, strategy, "merge-file", "fork and upstream both changed this path", ours_changed, baseline_exists, upstream_exists)


def checkout_upstream(upstream_ref: str, path: str) -> PathOutcome:
    completed = run_git(["checkout", upstream_ref, "--", path], check=False)
    return PathOutcome(path, "checkout-upstream", completed.returncode, note=completed.stderr.decode("utf-8", errors="replace").strip())


def delete_path(path: str) -> PathOutcome:
    target = REPO_ROOT / path
    if target.exists():
        target.unlink()
    run_git(["rm", "--ignore-unmatch", "--", path], check=False)
    return PathOutcome(path, "delete")


def merge_file(baseline_ref: str, upstream_ref: str, path: str) -> PathOutcome:
    base = read_ref(baseline_ref, path)
    theirs = read_ref(upstream_ref, path)
    ours_path = REPO_ROOT / path
    ours = ours_path.read_bytes() if ours_path.exists() else b""
    if any(is_binary(data) for data in (base, theirs, ours)):
        return checkout_upstream(upstream_ref, path)

    with tempfile.TemporaryDirectory(prefix="codex-overlay-merge-") as tmp:
        tmp_path = Path(tmp)
        ours_file = tmp_path / "ours"
        base_file = tmp_path / "base"
        theirs_file = tmp_path / "theirs"
        ours_file.write_bytes(ours)
        base_file.write_bytes(base)
        theirs_file.write_bytes(theirs)
        completed = subprocess.run(
            ["git", "merge-file", "-p", str(ours_file), str(base_file), str(theirs_file)],
            cwd=REPO_ROOT,
            capture_output=True,
            check=False,
        )
    ours_path.parent.mkdir(parents=True, exist_ok=True)
    ours_path.write_bytes(completed.stdout)
    return PathOutcome(
        path,
        "merge-file",
        completed.returncode,
        conflicted=completed.returncode == 1,
        note=completed.stderr.decode("utf-8", errors="replace").strip(),
    )


def apply_plan(args: argparse.Namespace, plans: list[PathPlan]) -> list[PathOutcome]:
    outcomes: list[PathOutcome] = []
    checkout_paths = [plan.path for plan in plans if plan.action == "checkout-upstream"]
    for path_batch in batches(checkout_paths, 120):
        completed = run_git(["checkout", args.upstream_ref, "--", *path_batch], check=False)
        note = completed.stderr.decode("utf-8", errors="replace").strip()
        outcomes.extend(
            PathOutcome(path, "checkout-upstream", completed.returncode, note=note)
            for path in path_batch
        )

    delete_paths = [plan.path for plan in plans if plan.action == "delete"]
    for path in delete_paths:
        outcomes.append(delete_path(path))

    for plan in plans:
        if plan.action == "skip":
            outcomes.append(PathOutcome(plan.path, "skip", note=plan.reason))
        elif plan.action == "merge-file":
            outcomes.append(merge_file(args.baseline_ref, args.upstream_ref, plan.path))
        elif plan.action not in {"checkout-upstream", "delete"}:
            outcomes.append(PathOutcome(plan.path, plan.action, returncode=1, note="unknown action"))
    conflicts = [outcome.path for outcome in outcomes if outcome.conflicted]
    if conflicts:
        reinject_rules = [
            "--rule"
            for path in sorted(REINJECT_PATHS)
            if path in conflicts
        ]
        reinject_rule_values = [
            f"{path}=upstream-reinject"
            for path in sorted(REINJECT_PATHS)
            if path in conflicts
        ]
        resolver = [
            sys.executable,
            str(REPO_ROOT / "scripts" / "resolve_merge_conflicts.py"),
            *(
                value
                for rule_pair in zip(reinject_rules, reinject_rule_values)
                for value in rule_pair
            ),
            *conflicts,
        ]
        completed = subprocess.run(resolver, cwd=REPO_ROOT, check=False)
        outcomes.append(PathOutcome("scripts/resolve_merge_conflicts.py", "resolve-conflicts", completed.returncode, note=f"{len(conflicts)} conflicted paths"))
    return outcomes


def batches(items: list[str], size: int) -> list[list[str]]:
    return [items[index : index + size] for index in range(0, len(items), size)]


def write_reports(args: argparse.Namespace, plans: list[PathPlan], outcomes: list[PathOutcome]) -> None:
    payload = {
        "baseline_ref": args.baseline_ref,
        "upstream_ref": args.upstream_ref,
        "applied": args.apply,
        "plans": [asdict(plan) for plan in plans],
        "outcomes": [asdict(outcome) for outcome in outcomes],
        "summary": {
            "planned_paths": len(plans),
            "actions": action_counts(plans),
            "conflicted_paths": [outcome.path for outcome in outcomes if outcome.conflicted],
            "failed_paths": [outcome.path for outcome in outcomes if outcome.returncode not in {0, 1}],
        },
    }
    args.report_json.parent.mkdir(parents=True, exist_ok=True)
    args.report_json.write_text(json.dumps(payload, indent=2, ensure_ascii=False, sort_keys=True) + "\n", encoding="utf-8")

    lines = [
        "# Upstream Overlay Merge Report",
        "",
        f"- Baseline ref: `{args.baseline_ref}`",
        f"- Upstream ref: `{args.upstream_ref}`",
        f"- Applied: `{'yes' if args.apply else 'no'}`",
        f"- Planned paths: **{len(plans)}**",
        "",
        "## Actions",
        "",
    ]
    for action, count in action_counts(plans).items():
        lines.append(f"- `{action}`: **{count}**")
    lines.extend(["", "## Conflicts", ""])
    conflicts = [outcome.path for outcome in outcomes if outcome.conflicted]
    lines.extend([f"- `{path}`" for path in conflicts] or ["- none"])
    lines.extend(["", "## Planned Paths", ""])
    for plan in plans[:400]:
        lines.append(f"- `{plan.path}`: `{plan.action}` ({plan.strategy})")
    args.report_md.write_text("\n".join(lines).rstrip() + "\n", encoding="utf-8")


def action_counts(plans: list[PathPlan]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for plan in plans:
        counts[plan.action] = counts.get(plan.action, 0) + 1
    return dict(sorted(counts.items()))


def ensure_clean_worktree(allow_dirty: bool) -> None:
    if allow_dirty:
        return
    status = git_text(["status", "--porcelain"])
    if status.strip():
        raise SystemExit("Working tree is not clean. Commit, stash, or pass --allow-dirty.")


def main() -> int:
    args = parse_args()
    ensure_clean_worktree(args.allow_dirty or args.apply is False)
    rows = changed_paths(args.baseline_ref, args.upstream_ref)
    if args.limit is not None:
        rows = rows[: args.limit]
    baseline_paths = tree_paths(args.baseline_ref)
    upstream_paths = tree_paths(args.upstream_ref)
    local_changes = local_changed_paths(args.baseline_ref)
    plans = [
        plan_path(
            status=status,
            path=path,
            baseline_paths=baseline_paths,
            upstream_paths=upstream_paths,
            local_changes=local_changes,
        )
        for status, path in rows
    ]
    outcomes = apply_plan(args, plans) if args.apply else []
    write_reports(args, plans, outcomes)
    failed = [outcome for outcome in outcomes if outcome.returncode not in {0, 1}]
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
