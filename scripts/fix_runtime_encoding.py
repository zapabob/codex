#!/usr/bin/env python3
"""runtime.rsのエンコーディング問題を修正する"""

import sys

path = r"C:\Users\downl\Desktop\codex-main\codex-rs\core\src\agents\runtime.rs"

with open(path, "rb") as f:
    raw = f.read()

try:
    content = raw.decode("utf-8")
    print("Already UTF-8 valid")
    # findとreplaceだけ実行
    content2 = content.replace("find_model_info_for_slug", "model_info_from_slug")
    if content2 != content:
        with open(path, "w", encoding="utf-8") as f:
            f.write(content2)
        print("Replaced find_model_info_for_slug -> model_info_from_slug")
except UnicodeDecodeError as e:
    print(f"UTF-8 error at position {e.start}: {e.reason}")
    # latin-1でデコードして置換
    content = raw.decode("latin-1")
    content = content.replace("find_model_info_for_slug", "model_info_from_slug")
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    print("Fixed and saved as UTF-8")
