#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
FAST_BUILD = REPO_ROOT / "scripts" / "fast_build.py"


def main() -> int:
    command = [sys.executable, str(FAST_BUILD), "upstream-sync", *sys.argv[1:]]
    completed = subprocess.run(command, cwd=REPO_ROOT)
    return completed.returncode


if __name__ == "__main__":
    raise SystemExit(main())
