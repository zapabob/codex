#!/usr/bin/env python3
"""
Blueprint → Plan 変数名修正スクリプト
残りの大文字Plan変数を小文字planに一括置換
"""

import re
from pathlib import Path
from typing import List


def fix_file(file_path: Path) -> bool:
    """ファイル内の Plan 変数を plan に修正"""
    try:
        content = file_path.read_text(encoding="utf-8")
        original = content

        # パターン1: ", Plan," → ", plan,"
        content = re.sub(r",\s*Plan\s*,", ", plan,", content)

        # パターン2: "Plan.xxx" → "plan.xxx"
        content = re.sub(r"\bPlan\.", "plan.", content)

        # パターン3: "&Plan." → "&plan."
        content = re.sub(r"&Plan\.", "&plan.", content)

        # パターン4: "(Plan)" → "(plan)" 関数引数など
        content = re.sub(r"\(Plan\)", "(plan)", content)

        # パターン5: "Plan:" → "plan:" パラメータ
        content = re.sub(r"\bPlan:", "plan:", content)

        # パターン6: "plan.title," の後にPlanが来るパターン
        content = re.sub(r"(\w+\(.*?)Plan\b", r"\1plan", content)

        if content != original:
            file_path.write_text(content, encoding="utf-8")
            return True
        return False

    except Exception as e:
        print(f"❌ Error processing {file_path}: {e}")
        return False


def main():
    base_path = Path(__file__).parent.parent

    # 修正対象ファイル
    target_files = [
        base_path / "codex-rs/core/src/orchestration/plan_orchestrator.rs",
        base_path / "codex-rs/core/src/execution/engine.rs",
        base_path / "codex-rs/core/src/agents/competition.rs",
    ]

    print("🔧 Plan変数修正スクリプト")
    print("=" * 50)

    fixed_count = 0
    for file_path in target_files:
        if file_path.exists():
            if fix_file(file_path):
                print(f"✓ {file_path.relative_to(base_path)}")
                fixed_count += 1
            else:
                print(f"  {file_path.relative_to(base_path)} (no changes)")
        else:
            print(f"✗ Not found: {file_path}")

    print("=" * 50)
    print(f"🎉 完了！{fixed_count} ファイル修正")


if __name__ == "__main__":
    main()
