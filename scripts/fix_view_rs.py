#!/usr/bin/env python3
"""
feedback/view.rsの重複関数定義を削除し、importパスを修正する
"""

import sys
import re

path = (
    r"C:\Users\downl\Desktop\codex-main\codex-rs\tui\src\bottom_pane\feedback\view.rs"
)

with open(path, "r", encoding="utf-8") as f:
    content = f.read()

original = content

# 1. 重複インライン関数定義を削除 (lines 329-396 相当)
# gutter() から slack_feedback_url() まで削除
pattern_to_remove = r"\nfn gutter\(\).*?fn slack_feedback_url\([^)]*\)[^}]*\}\n"
content = re.sub(pattern_to_remove, "\n", content, flags=re.DOTALL)

# 2. 未使用のBASE_BUG_ISSUE_URL参照を修正 (utils.rsのBASE_ISSUE_URLを使う)
content = content.replace(
    "BASE_BUG_ISSUE_URL", "crate::bottom_pane::feedback::utils::BASE_ISSUE_URL"
)

# 3. super::popup_consts の修正
content = content.replace(
    "use super::popup_consts::standard_popup_hint_line;",
    "use crate::bottom_pane::popup_consts::standard_popup_hint_line;",
)

# 4. super::SelectionViewParams → crate::bottom_pane::SelectionViewParams
content = content.replace(
    "super::SelectionViewParams", "crate::bottom_pane::SelectionViewParams"
)
content = content.replace("super::SelectionItem", "crate::bottom_pane::SelectionItem")
content = content.replace(
    "super::SelectionAction", "crate::bottom_pane::SelectionAction"
)

# 5. BASE_BUG_ISSUE_URL を utils.rsの BASE_ISSUE_URL から import するよう修正
# 既にutils::BASE_ISSUE_URLとして参照されるので不要

with open(path, "w", encoding="utf-8") as f:
    f.write(content)

if content != original:
    print(f"Fixed: {path}")
    # 削除された内容を確認
    import difflib

    diff = list(
        difflib.unified_diff(
            original.splitlines(), content.splitlines(), lineterm="", n=2
        )
    )
    for line in diff[:40]:
        print(line)
else:
    print("No changes made")
