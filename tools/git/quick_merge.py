#!/usr/bin/env python3
"""Simple merge conflict resolver."""

import subprocess
import re


def run(cmd):
    r = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return r.returncode == 0, r.stdout, r.stderr


print("Resolving conflicts...")

# Get conflicted files
_, out, _ = run("git diff --name-only --diff-filter=U")
files = [f for f in out.strip().split("\n") if f]
print(f"Files: {files}")

for f in files:
    print(f"  Processing: {f}")

    # Read file
    with open(f, "r", encoding="utf-8", errors="replace") as fp:
        content = fp.read()

    if "<<<<<<< HEAD" not in content:
        continue

    # Get both versions
    run(f"git show HEAD:{f} > /tmp/ours_{f.replace('/', '_')}.txt")
    run(f"git show upstream/main:{f} > /tmp/theirs_{f}.txt")

    with open(f"/tmp/ours_{f.replace('/', '_')}.txt", "r", encoding="utf-8") as fp:
        ours = fp.read()
    with open(f"/tmp/theirs_{f}.txt", "r", encoding="utf-8") as fp:
        theirs = fp.read()

    # Merge: theirs + unique imports from ours
    merged = theirs

    # Add imports from ours not in theirs
    for line in ours.split("\n"):
        line = line.strip()
        if line.startswith("use ") and line not in theirs:
            if "use crate::slash_command::SlashCommand;" in merged:
                merged = merged.replace(
                    "use crate::slash_command::SlashCommand;",
                    f"use crate::slash_command::SlashCommand;\n{line}",
                    1,
                )

    # Remove conflict markers
    merged = re.sub(
        r"<<<<<<< HEAD\n.*?\n=======\n.*?\n>>>>>>> .*?\n", "", merged, flags=re.DOTALL
    )

    # Write merged content
    with open(f, "w", encoding="utf-8") as fp:
        fp.write(merged)

    run(f'git add "{f}"')
    print(f"    Done: {f}")

# Complete merge
print("Completing merge...")
ok, out, err = run("git merge --continue --no-edit")
if ok:
    print("SUCCESS!")
else:
    print(f"Error: {err}")
