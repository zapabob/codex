#!/usr/bin/env python3
from __future__ import annotations

import argparse
import fnmatch
from dataclasses import dataclass
from pathlib import Path
import re

DEFAULT_REPO_ROOT = Path(__file__).resolve().parents[1]


@dataclass(frozen=True)
class Rule:
    pattern: str
    strategy: str


DEFAULT_RULES = [
    Rule("codex-rs/deep-research/**", "upstream-plus-reinject"),
    Rule("codex-rs/core/src/agents/**", "upstream-plus-reinject"),
    Rule("codex-rs/core/src/orchestration/**", "upstream-plus-reinject"),
    Rule("codex-rs/core/src/plan/**", "upstream-plus-reinject"),
    Rule("gui/**", "plugin-migrate"),
    Rule("codex-gui-x/**", "plugin-migrate"),
    Rule("codex-rs/gui/**", "plugin-migrate"),
    Rule("codex-rs/tauri-gui/**", "plugin-migrate"),
    Rule("gui/src/app/virtual-os/**", "retire-after-parity"),
    Rule("gui/src/components/virtual-os/**", "retire-after-parity"),
    Rule("codex-rs/**/virtual-os/**", "retire-after-parity"),
    Rule(".agents/plugins/**", "upstream"),
    Rule("plugins/**", "upstream"),
    Rule("codex-rs/**", "upstream"),
    Rule("docs/**", "upstream"),
    Rule("CHANGELOG.md", "upstream-reinject"),
    Rule("justfile", "upstream-reinject"),
    Rule("codex-rs/justfile", "upstream-reinject"),
]

CONFLICT_RE = re.compile(
    r"^<<<<<<< .*?\n(?P<local>.*?)^=======\n(?P<upstream>.*?)^>>>>>>> .*?$",
    re.MULTILINE | re.DOTALL,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Resolve merge conflicts with upstream-first reinjection rules")
    parser.add_argument("paths", nargs="+", help="Conflicted paths to resolve")
    parser.add_argument("--rule", action="append", default=[], help="Extra rule in glob=strategy form")
    parser.add_argument(
        "--repo-root",
        type=Path,
        help="Repository/worktree root used to resolve conflicted paths (defaults to current working directory)",
    )
    return parser.parse_args()


def load_rules(extra: list[str]) -> list[Rule]:
    rules = DEFAULT_RULES.copy()
    for entry in extra:
        pattern, _, strategy = entry.partition("=")
        if not pattern or not strategy:
            raise SystemExit(f"Invalid rule '{entry}'. Expected glob=strategy")
        rules.insert(0, Rule(pattern, strategy))
    return rules


def choose_strategy(path: str, rules: list[Rule]) -> str:
    for rule in rules:
        if fnmatch.fnmatch(path, rule.pattern):
            return rule.strategy
    return "upstream-plus-reinject"


def unique_local_lines(local: str, upstream: str) -> list[str]:
    upstream_lines = {line.rstrip() for line in upstream.splitlines() if line.strip()}
    seen: set[str] = set()
    kept: list[str] = []
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
    if strategy in {"upstream", "plugin-migrate", "retire-after-parity"}:
        return upstream
    if strategy == "upstream-plus-reinject":
        strategy = "upstream-reinject"
    reinjected = unique_local_lines(local, upstream)
    if not reinjected:
        return upstream
    body = upstream.rstrip("\n")
    trailer = "\n" if body else ""
    trailer += "\n".join(reinjected)
    if not trailer.endswith("\n"):
        trailer += "\n"
    return body + trailer


def resolve_file(path: Path, relative: str, rules: list[Rule]) -> None:
    text = path.read_text(encoding="utf-8")
    strategy = choose_strategy(relative, rules)

    def repl(match: re.Match[str]) -> str:
        return resolve_block(match.group("local"), match.group("upstream"), strategy)

    updated, count = CONFLICT_RE.subn(repl, text)
    if count == 0:
        print(f"skip {relative}: no merge markers found")
        return
    path.write_text(updated, encoding="utf-8")
    print(f"resolved {relative}: strategy={strategy}, blocks={count}")


def main() -> int:
    args = parse_args()
    rules = load_rules(args.rule)
    repo_root = (args.repo_root or Path.cwd()).resolve()
    for raw_path in args.paths:
        path = (repo_root / raw_path).resolve()
        if not path.exists():
            fallback = (DEFAULT_REPO_ROOT / raw_path).resolve()
            if fallback.exists():
                path = fallback
            else:
                print(f"skip {raw_path}: conflicted path not found in {repo_root}")
                continue
        try:
            relative = path.relative_to(repo_root).as_posix()
        except ValueError:
            try:
                relative = path.relative_to(DEFAULT_REPO_ROOT).as_posix()
            except ValueError:
                relative = raw_path.replace("\\", "/")
        resolve_file(path, relative, rules)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
