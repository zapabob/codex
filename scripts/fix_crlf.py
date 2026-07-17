#!/usr/bin/env python3
"""Rustソースファイルの bare CR (\r) を除去してLFに正規化する"""

import sys

files_to_fix = [
    r"C:\Users\downl\Desktop\codex-main\codex-rs\core\src\agents\runtime.rs",
]

for path in files_to_fix:
    with open(path, "rb") as f:
        raw = f.read()

    # CRLF -> LF
    normalized = raw.replace(b"\r\n", b"\n").replace(b"\r", b"\n")

    if normalized != raw:
        with open(path, "wb") as f:
            f.write(normalized)
        print(f"Fixed CRLF in: {path}")
    else:
        print(f"No CRLF found: {path}")

    # UTF-8として読めるか確認
    try:
        normalized.decode("utf-8")
        print(f"  -> UTF-8 OK")
    except UnicodeDecodeError as e:
        print(f"  -> UTF-8 ERROR: {e}")
