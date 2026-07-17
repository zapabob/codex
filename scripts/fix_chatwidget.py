#!/usr/bin/env python3
"""
chatwidget.rsの重複UserMessage定義を削除する
"""

import re

path = r"C:\Users\downl\Desktop\codex-main\codex-rs\tui\src\chatwidget.rs"

with open(path, "r", encoding="utf-8") as f:
    content = f.read()

original = content

# 1. インラインUserMessage構造体定義を削除 (lines 476-487)
# pub(crate) struct UserMessage { ... } と From<String>, From<&str> impls
pattern_struct = r"\npub\(crate\) struct UserMessage \{[^}]+\}\n\nimpl From<String> for UserMessage \{[^}]+\}\n\}\n\nimpl From<&str> for UserMessage \{[^}]+\}\n\}\n"
content = re.sub(pattern_struct, "\n", content, flags=re.DOTALL)

# 2. pub(crate) fn create_initial_user_message を削除
pattern_create = r"\npub\(crate\) fn create_initial_user_message\([^{]+\{[\s\S]*?^\}\n"
content = re.sub(pattern_create, "\n", content, flags=re.DOTALL | re.MULTILINE)

# 3. fn remap_placeholders_for_message を削除
# (行頭がfnで始まる)
pattern_remap = r"\n// When merging multiple queued drafts.*?fn remap_placeholders_for_message\([^{]+\{[\s\S]*?^\}\n"
content = re.sub(pattern_remap, "\n", content, flags=re.DOTALL | re.MULTILINE)

with open(path, "w", encoding="utf-8") as f:
    f.write(content)

if content != original:
    print("Fixed chatwidget.rs - removed duplicate definitions")
    # 変更前後の行数確認
    orig_lines = len(original.splitlines())
    new_lines = len(content.splitlines())
    print(
        f"  Lines: {orig_lines} -> {new_lines} (removed {orig_lines - new_lines} lines)"
    )
else:
    print("No changes - patterns not found. Manual check needed.")
