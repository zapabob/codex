#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Codex Repository Organization Script
徹底的なリポジトリ整理整頓を実行
"""

import os
import shutil
import time
from pathlib import Path
from tqdm import tqdm

class RepositoryOrganizer:
    def __init__(self, root_dir):
        self.root_dir = Path(root_dir)
        self.actions_taken = []

    def log_action(self, action, details=""):
        """アクションをログに記録"""
        self.actions_taken.append(f"{action}: {details}")
        print(f"[ACTION] {action}: {details}")

    def create_directory(self, path):
        """ディレクトリを作成"""
        full_path = self.root_dir / path
        if not full_path.exists():
            full_path.mkdir(parents=True, exist_ok=True)
            self.log_action("CREATE_DIR", str(path))

    def move_file(self, src, dst):
        """ファイルを移動"""
        src_path = self.root_dir / src
        dst_path = self.root_dir / dst

        if src_path.exists():
            # 宛先ディレクトリが存在することを確認
            dst_path.parent.mkdir(parents=True, exist_ok=True)

            shutil.move(str(src_path), str(dst_path))
            self.log_action("MOVE_FILE", f"{src} -> {dst}")
        else:
            self.log_action("SKIP_MISSING", f"Source not found: {src}")

    def cleanup_empty_dirs(self):
        """空のディレクトリを削除"""
        cleaned = 0
        for root, dirs, files in os.walk(str(self.root_dir), topdown=False):
            for dir_name in dirs:
                dir_path = Path(root) / dir_name
                try:
                    # .gitディレクトリなどはスキップ
                    if '.git' in str(dir_path):
                        continue

                    if not any(dir_path.rglob('*')):  # 完全に空か？
                        dir_path.rmdir()
                        self.log_action("REMOVE_EMPTY_DIR", str(dir_path.relative_to(self.root_dir)))
                        cleaned += 1
                except OSError:
                    pass  # 削除できない場合はスキップ

        return cleaned

    def organize_root_files(self):
        """ルートディレクトリのファイルを整理"""
        print("Organizing root directory files...")

        # 作業スクリプトを tools/ に移動
        scripts_to_move = [
            "build_progress.py",
            "check_tweet_length.py",
            "check_version_unity.py",
            "copy_install.py",
            "fast_build_install.py",
            "fix_compilation_errors.py",
            "organize_repository.py"
        ]

        self.create_directory("tools")
        for script in scripts_to_move:
            if (self.root_dir / script).exists():
                self.move_file(script, f"tools/{script}")

        # その他のファイルを適切な場所に移動
        other_files = {
            "CHANGELOG.md": "docs/CHANGELOG.md",
            "NOTICE": "docs/NOTICE",
            "ORGANIZATION_COMPLETE.md": "docs/ORGANIZATION_COMPLETE.md"
        }

        for src, dst in other_files.items():
            if (self.root_dir / src).exists():
                self.move_file(src, dst)

    def organize_archive_files(self):
        """アーカイブファイルを整理"""
        print("Organizing archive files...")

        # .archive/ のサブディレクトリを整理
        archive_mappings = {
            ".archive/artifacts/": "archive/artifacts/",
            ".archive/backups/": "archive/backups/",
            ".archive/build-artifacts/": "archive/build-artifacts/",
            ".archive/code-review-reports/": "archive/code-reviews/",
            ".archive/completions/": "archive/completions/",
            ".archive/debug/": "archive/debug/",
            ".archive/install/": "archive/install/",
            ".archive/legacy-docs/": "archive/legacy-docs/",
            ".archive/old-implementations/": "archive/old-implementations/",
            ".archive/research-reports/": "archive/research/",
            ".archive/security-reports/": "archive/security/",
            ".archive/temp/": "archive/temp/",
            ".archive/test/": "archive/test/",
            ".archive/test-outputs/": "archive/test-outputs/",
            ".archive/test-pack-install/": "archive/test-pack-install/",
            ".archive/util/": "archive/util/"
        }

        for src, dst in archive_mappings.items():
            src_path = self.root_dir / src
            if src_path.exists() and src_path.is_dir():
                self.create_directory(dst)
                for item in src_path.iterdir():
                    if item.is_file():
                        dst_file = dst + item.name
                        self.move_file(str(item.relative_to(self.root_dir)), dst_file)

        # 個別ファイルを移動
        single_files = [
            (".archive/build_err.txt", "archive/build-errors.txt"),
            (".archive/build_errors.txt", "archive/build-errors.txt"),
            (".archive/code-review-report.md", "archive/code-reviews/main-report.md"),
            (".archive/debug-codex.sh", "archive/debug/debug-codex.sh"),
            (".archive/FINAL_COMPLETION_SUMMARY.md", "archive/summaries/final-completion.md"),
            (".archive/PHASE2_COMPLETION_REPORT.md", "archive/summaries/phase2-completion.md"),
            (".archive/PROJECT_COMPLETE_FINAL_SUMMARY.md", "archive/summaries/project-complete-final.md"),
            (".archive/PROJECT_COMPLETE_SUMMARY.md", "archive/summaries/project-complete.md"),
            (".archive/PULL_REQUEST_OPENAI_COMPLETE.md", "archive/pull-requests/openai-complete.md"),
            (".archive/PULL_REQUEST_OPENAI.md", "archive/pull-requests/openai.md"),
            (".archive/PULL_REQUEST.md", "archive/pull-requests/main.md"),
            (".archive/report.md", "archive/reports/main-report.md"),
            (".archive/research-report.md", "archive/research/main-report.md")
        ]

        for src, dst in single_files:
            self.move_file(src, dst)

    def organize_docs(self):
        """ドキュメントファイルを整理"""
        print("Organizing documentation files...")

        # _docs/ のファイルを docs/ に統合
        docs_path = self.root_dir / "_docs"
        if docs_path.exists():
            for item in docs_path.rglob("*"):
                if item.is_file():
                    relative_path = item.relative_to(docs_path)
                    dst_path = f"docs/{relative_path}"
                    self.move_file(str(item.relative_to(self.root_dir)), dst_path)

    def create_summary(self):
        """整理結果のサマリーを作成"""
        summary = f"""
Repository Organization Complete
================================

Actions Taken: {len(self.actions_taken)}

Details:
"""

        for i, action in enumerate(self.actions_taken[:50], 1):  # 最初の50件
            summary += f"{i:3d}. {action}\n"

        if len(self.actions_taken) > 50:
            summary += f"... and {len(self.actions_taken) - 50} more actions\n"

        # サマリーファイルを作成
        summary_file = self.root_dir / "REPOSITORY_ORGANIZATION_SUMMARY.md"
        with open(summary_file, 'w', encoding='utf-8') as f:
            f.write(summary)

        print(f"Summary saved to: {summary_file}")

        return summary

def main():
    print("Codex Repository Organization Tool")
    print("=" * 50)
    print("Thorough repository cleanup and folder organization")
    print()

    # 現在のディレクトリを取得
    current_dir = Path.cwd()

    organizer = RepositoryOrganizer(current_dir)

    # 整理作業を実行
    tasks = [
        ("Create necessary directories", lambda: None),  # プレースホルダー
        ("Organize root directory files", organizer.organize_root_files),
        ("Organize archive files", organizer.organize_archive_files),
        ("Organize documentation", organizer.organize_docs),
        ("Clean up empty directories", lambda: organizer.cleanup_empty_dirs()),
    ]

    with tqdm(total=len(tasks), desc="[ORGANIZE] Repository cleanup", bar_format='{desc}: {percentage:3.0f}%|{bar}| {n_fmt}/{total_fmt}') as pbar:
        for desc, task_func in tasks:
            pbar.set_description(f"[ORGANIZE] {desc}")
            try:
                result = task_func()
                if result is not None:
                    print(f"  Result: {result}")
            except Exception as e:
                print(f"  Error in {desc}: {e}")

            pbar.update(1)
            time.sleep(0.5)

    print()
    organizer.create_summary()

    print("Repository organization completed!")
    print(f"Total actions taken: {len(organizer.actions_taken)}")

if __name__ == "__main__":
    main()