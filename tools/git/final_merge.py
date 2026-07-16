#!/usr/bin/env python3
"""Resolve all conflicts."""

import subprocess, re, os, sys


def run(cmd):
    r = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return r.returncode == 0, r.stdout, r.stderr


print("=" * 50)
print("Resolving merge conflicts")
print("=" * 50)

files = [
    "codex-rs/Cargo.lock",
    "codex-rs/Cargo.toml",
    "codex-rs/core/src/event_mapping.rs",
    "codex-rs/tui/src/chatwidget.rs",
]

for f in files:
    print(f"\nProcessing: {f}")

    # Use upstream version
    ok, out, err = run(f"git show upstream/main:{f} > /tmp/theirs.txt")

    with open("/tmp/theirs.txt", "r") as fp:
        content = fp.read()

    # For chatwidget.rs, add our imports
    if "chatwidget" in f:
        ok, ours, _ = run(f"git show HEAD:{f}")
        imports = set()
        for line in ours.split("\n"):
            line = line.strip()
            if line.startswith("use ") and line not in content:
                imports.add(line)

        for imp in sorted(imports):
            if "use crate::slash_command::SlashCommand;" in content:
                content = content.replace(
                    "use crate::slash_command::SlashCommand;",
                    f"use crate::slash_command::SlashCommand;\n{imp}",
                    1,
                )
        print(f"  Added {len(imports)} imports from ours")

    # Remove conflict markers
    content = re.sub(
        r"<<<<<<< HEAD\n.*?\n=======\n.*?\n>>>>>>> .*?\n", "", content, flags=re.DOTALL
    )

    # Write merged content
    with open(f, "w", encoding="utf-8") as fp:
        fp.write(content)

    run(f'git add "{f}"')
    print(f"  Resolved: {f}")

print("\nCompleting merge...")
ok, out, err = run("git merge --continue --no-edit --quiet")
if ok:
    print("\nSUCCESS! Merge completed.")
    sys.exit(0)
else:
    print(f"\nError: {err}")
    sys.exit(1)
