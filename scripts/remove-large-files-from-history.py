#!/usr/bin/env python3
"""
Git履歴から大容量ファイルを削除するスクリプト
"""

import subprocess
import sys
from pathlib import Path


def run_command(cmd, cwd=None):
    """コマンドを実行して結果を返す"""
    try:
        result = subprocess.run(
            cmd,
            shell=True,
            capture_output=True,
            text=True,
            cwd=cwd,
            encoding="utf-8",
            errors="ignore",
        )
        print(f"✓ 実行: {cmd}")
        if result.stdout:
            print(result.stdout)
        if result.stderr:
            print(f"  警告: {result.stderr}", file=sys.stderr)
        return result.returncode == 0
    except Exception as e:
        print(f"✗ エラー: {e}", file=sys.stderr)
        return False


def main():
    repo_path = Path(__file__).parent.parent
    print(f"📁 リポジトリ: {repo_path}\n")

    # 削除対象のファイル
    files_to_remove = [
        "codex-cli/openai-codex-0.52.0.tgz",
        "codex-cli/zapabob-codex-0.52.0.tgz",
    ]

    print("🗑️  以下のファイルを履歴から削除します:")
    for f in files_to_remove:
        print(f"  - {f}")
    print()

    # バックアップ作成
    print("📦 バックアップ作成中...")
    run_command("git branch backup-before-filter-branch", cwd=repo_path)

    # git filter-branchで履歴から削除
    print("\n🔧 Git履歴からファイルを削除中...")
    filter_cmd = (
        "git filter-branch --force --index-filter "
        '"git rm --cached --ignore-unmatch '
        + " ".join(files_to_remove)
        + '" --prune-empty --tag-name-filter cat -- --all'
    )

    if not run_command(filter_cmd, cwd=repo_path):
        print("\n✗ filter-branch失敗。代替方法を試します...", file=sys.stderr)

        # 代替方法: 各ファイルを個別に削除
        for file in files_to_remove:
            print(f"\n🔧 {file} を削除中...")
            alt_cmd = f'git filter-branch --force --index-filter "git rm --cached --ignore-unmatch {file}" --prune-empty --tag-name-filter cat -- --all'
            run_command(alt_cmd, cwd=repo_path)

    # バックアップ参照を削除
    print("\n🧹 バックアップ参照を削除中...")
    run_command(
        "git for-each-ref --format='delete %(refname)' refs/original | git update-ref --stdin",
        cwd=repo_path,
    )

    # ガベージコレクション実行
    print("\n🧹 ガベージコレクション実行中...")
    run_command("git reflog expire --expire=now --all", cwd=repo_path)
    run_command("git gc --prune=now --aggressive", cwd=repo_path)

    print("\n✅ 完了！")
    print("\n📊 リポジトリサイズを確認:")
    run_command("git count-objects -vH", cwd=repo_path)

    print("\n⚠️  注意: 履歴を書き換えたため、強制プッシュが必要です:")
    print("  git push origin main --force")


if __name__ == "__main__":
    main()
