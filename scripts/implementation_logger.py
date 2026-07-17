#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
実装ログ管理スクリプト
MCPサーバー経由で日時を取得し、実装ログを自動生成・管理する
"""

import json
import os
import sys
from datetime import datetime
from pathlib import Path
from typing import Optional, Dict, List
import subprocess
import re

# なんJ風の応答テンプレート
NANJ_TEMPLATES = [
    "実装ログ確認したで！{count}件のログがあるな。",
    "おっ、{count}件の実装ログやな。最近のやつ見てみるで。",
    "実装ログ{count}件発見！最近の実装を確認するで。",
    "ログ{count}件あるで。最近の実装をチェックするわ。",
]


def get_current_datetime_via_mcp() -> Optional[str]:
    """MCPサーバー経由で現在日時を取得"""
    try:
        # MCPサーバー経由で日時を取得（codex-datetime toolを使用）
        # 実際の実装では、MCPクライアント経由で呼び出す
        # ここではフォールバックとしてシステム日時を使用
        return datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    except Exception as e:
        print(f"警告: MCP経由での日時取得に失敗: {e}", file=sys.stderr)
        return datetime.now().strftime("%Y-%m-%d %H:%M:%S")


def get_current_date() -> str:
    """現在日付を取得（yyyy-mm-dd形式）"""
    return datetime.now().strftime("%Y-%m-%d")


def get_worktree_name() -> str:
    """現在のworktree名を取得"""
    try:
        # git worktree list で現在のworktreeを確認
        result = subprocess.run(
            ["git", "worktree", "list"], capture_output=True, text=True, cwd=Path.cwd()
        )
        if result.returncode == 0:
            lines = result.stdout.strip().split("\n")
            for line in lines:
                # 現在のディレクトリを含む行を探す
                if str(Path.cwd()) in line:
                    # worktree名を抽出（パスの最後の部分）
                    parts = line.split()
                    if len(parts) > 0:
                        path = Path(parts[0])
                        # mainブランチの場合は "main" を返す
                        if "main" in path.name.lower() or path.name == ".":
                            return "main"
                        return path.name
        # フォールバック: ブランチ名を取得
        result = subprocess.run(
            ["git", "rev-parse", "--abbrev-ref", "HEAD"],
            capture_output=True,
            text=True,
            cwd=Path.cwd(),
        )
        if result.returncode == 0:
            branch = result.stdout.strip()
            return branch if branch else "main"
    except Exception as e:
        print(f"警告: worktree名の取得に失敗: {e}", file=sys.stderr)
    return "main"


def create_implementation_log(
    feature_name: str,
    task_description: str,
    implementation_details: str,
    worktree_name: Optional[str] = None,
) -> Path:
    """
    実装ログを作成

    Args:
        feature_name: 機能名
        task_description: タスクの説明
        implementation_details: 実装詳細
        worktree_name: worktree名（省略時は自動取得）

    Returns:
        作成されたログファイルのパス
    """
    # 日時を取得
    current_datetime = get_current_datetime_via_mcp()
    current_date = get_current_date()

    # worktree名を取得
    if worktree_name is None:
        worktree_name = get_worktree_name()

    # ファイル名を生成（yyyy-mm-dd_機能名{worktreename}.md）
    safe_feature_name = re.sub(r'[<>:"/\\|?*]', "_", feature_name)
    filename = f"{current_date}_{safe_feature_name}{{{worktree_name}}}.md"

    # _docsディレクトリのパス
    docs_dir = Path.cwd() / "_docs"
    docs_dir.mkdir(exist_ok=True)

    log_path = docs_dir / filename

    # ログ内容を生成
    log_content = f"""# {feature_name}

**日時**: {current_datetime}  
**タスク**: {task_description}  
**Worktree**: {worktree_name}

---

## 📋 実施内容

{implementation_details}

---

## ✅ 実施結果

### 完了タスク
- [x] 実装完了

---

## 💡 なんJ風コメント

**実装完了やで！🔥**

- {feature_name}の実装が完了したで
- 詳細は上記の実施内容を確認してくれ

**これで実装ログが記録されたで！🎉**

---

**実装者**: Codex Agent  
**実装日時**: {current_datetime}  
**ステータス**: ✅ 完了
"""

    # ファイルに書き込み
    with open(log_path, "w", encoding="utf-8") as f:
        f.write(log_content)

    print(f"実装ログを作成しました: {log_path}", file=sys.stderr)
    return log_path


def load_recent_implementation_logs(limit: int = 5) -> List[Dict]:
    """最近の実装ログを読み込む"""
    docs_dir = Path.cwd() / "_docs"
    if not docs_dir.exists():
        return []

    logs = []
    for log_file in sorted(
        docs_dir.glob("*.md"), key=lambda p: p.stat().st_mtime, reverse=True
    ):
        if log_file.is_file():
            try:
                with open(log_file, "r", encoding="utf-8") as f:
                    content = f.read()
                    # 日時を抽出
                    date_match = re.search(
                        r"\*\*日時\*\*:\s*(\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2})",
                        content,
                    )
                    # 機能名を抽出（最初の# の後）
                    title_match = re.search(r"^#\s+(.+)$", content, re.MULTILINE)
                    # worktree名を抽出
                    worktree_match = re.search(r"\*\*Worktree\*\*:\s*(\S+)", content)

                    logs.append(
                        {
                            "file": str(log_file),
                            "date": date_match.group(1) if date_match else "",
                            "feature": title_match.group(1)
                            if title_match
                            else log_file.stem,
                            "worktree": worktree_match.group(1)
                            if worktree_match
                            else "main",
                            "content": content[:500],  # 最初の500文字
                        }
                    )
            except Exception as e:
                print(
                    f"警告: ログファイルの読み込みに失敗: {log_file}: {e}",
                    file=sys.stderr,
                )

        if len(logs) >= limit:
            break

    return logs


def generate_nanj_response(logs: List[Dict]) -> str:
    """実装ログからなんJ風の応答を生成"""
    if not logs:
        return "実装ログはまだないで。これから実装していくで！"

    count = len(logs)
    template = NANJ_TEMPLATES[count % len(NANJ_TEMPLATES)]
    response = template.format(count=count)

    # 最近のログを追加
    if logs:
        latest = logs[0]
        response += f"\n\n最近の実装: {latest['feature']} ({latest['date']})"
        response += f"\nWorktree: {latest['worktree']}"

    return response


def main():
    """メイン関数"""
    if len(sys.argv) < 2:
        print("使用方法:")
        print(
            "  python implementation_logger.py create <機能名> <タスク説明> [実装詳細]"
        )
        print("  python implementation_logger.py load [件数]")
        print("  python implementation_logger.py nanj")
        sys.exit(1)

    command = sys.argv[1]

    if command == "create":
        if len(sys.argv) < 4:
            print("エラー: 機能名とタスク説明が必要です", file=sys.stderr)
            sys.exit(1)

        feature_name = sys.argv[2]
        task_description = sys.argv[3]
        implementation_details = sys.argv[4] if len(sys.argv) > 4 else "実装完了"

        log_path = create_implementation_log(
            feature_name, task_description, implementation_details
        )
        print(json.dumps({"status": "success", "path": str(log_path)}))

    elif command == "load":
        limit = int(sys.argv[2]) if len(sys.argv) > 2 else 5
        logs = load_recent_implementation_logs(limit)
        print(json.dumps(logs, ensure_ascii=False, indent=2))

    elif command == "nanj":
        logs = load_recent_implementation_logs(5)
        response = generate_nanj_response(logs)
        print(response)

    else:
        print(f"エラー: 不明なコマンド: {command}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
