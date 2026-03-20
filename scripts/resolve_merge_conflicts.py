#!/usr/bin/env python3
from __future__ import annotations

import argparse
import fnmatch
from dataclasses import dataclass
from pathlib import Path
import re

REPO_ROOT = Path(__file__).resolve().parents[1]


@dataclass(frozen=True)
class Rule:
    pattern: str
    strategy: str


DEFAULT_RULES = [
    Rule("codex-rs/**", "upstream-reinject"),
    Rule("codex-rs/gui/**", "upstream-reinject"),
    Rule("codex-gui-x/**", "custom"),
    Rule("extensions/**", "custom"),
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
    return "upstream-reinject"


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
    if strategy == "upstream":
        return upstream
    reinjected = unique_local_lines(local, upstream)
    if not reinjected:
        return upstream
    body = upstream.rstrip("\n")
    trailer = "\n" if body else ""
    trailer += "\n".join(reinjected)
    if not trailer.endswith("\n"):
        trailer += "\n"
    return body + trailer


def resolve_file(path: Path, rules: list[Rule]) -> None:
    text = path.read_text(encoding="utf-8")
    relative = path.relative_to(REPO_ROOT).as_posix()
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
    for raw_path in args.paths:
        path = (REPO_ROOT / raw_path).resolve()
        if not path.exists():
            raise SystemExit(f"Missing conflicted file: {raw_path}")
        resolve_file(path, rules)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
