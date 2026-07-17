#!/usr/bin/env python3
"""カンマ修正スクリプト"""

from pathlib import Path
import re

file_path = Path(
    r"C:\Users\downl\.cursor\worktrees\codex\tBA5Q\codex-rs\core\src\orchestration\plan_orchestrator.rs"
)

print(f"📝 修正ファイル: {file_path.name}")

content = file_path.read_text(encoding="utf-8")
original = content

# 4箇所のカンマ欠落を修正
content = re.sub(
    r"EventType::ExecStart\s+plan,", "EventType::ExecStart, plan,", content
)
content = re.sub(r'"exec\.start"\s+plan,', '"exec.start", plan,', content)
content = re.sub(
    r"EventType::ExecResult\s+plan,", "EventType::ExecResult, plan,", content
)
content = re.sub(r'"exec\.result"\s+plan,', '"exec.result", plan,', content)

if content != original:
    file_path.write_text(content, encoding="utf-8")
    print("✓ カンマ修正完了")

    # 変更箇所を表示
    for i, (old_line, new_line) in enumerate(
        zip(original.split("\n"), content.split("\n")), 1
    ):
        if old_line != new_line:
            print(f"  Line {i}: {old_line.strip()[:60]}...")
            print(f"       →  {new_line.strip()[:60]}...")
else:
    print("  変更なし")
