#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
リポジトリ整理整頓スクリプト
フォルダーへの収納と整理
"""

import os
import shutil
from pathlib import Path
from datetime import datetime

def create_archive_dirs(base_path):
    """アーカイブディレクトリを作成"""
    dirs = [
        "archive/configs",
        "archive/scripts",
        "archive/docs",
        "archive/temp",
    ]
    for dir_path in dirs:
        full_path = base_path / dir_path
        full_path.mkdir(parents=True, exist_ok=True)
        print(f"✅ 作成: {dir_path}")

def move_configs(base_path):
    """設定ファイルを移動"""
    configs = [
        "config-minimal.toml",
        "config-secure.toml",
        "config-ultra-minimal.toml",
        "config.toml.recommended",
    ]
    moved = 0
    for config in configs:
        src = base_path / config
        if src.exists():
            dst = base_path / "archive/configs" / config
            shutil.move(str(src), str(dst))
            print(f"✅ 移動: {config} → archive/configs/")
            moved += 1
    return moved

def move_scripts(base_path):
    """スクリプトファイルを移動"""
    patterns = [
        "build_*.py",
        "test_*.py",
        "check_*.py",
        "delete_*.py",
        "play_*.py",
        "execution_*.rs",
        "plan_*.rs",
        "persist.rs",
        "pnpm*.txt",
        "pnpm*.yaml",
        "turbo.json",
        "VERSION",
        "custom_*.txt",
        "diff_*.txt",
        "errors_*.txt",
        "clippy*.txt",
        "X_TWEET*.md",
    ]
    
    moved = 0
    for pattern in patterns:
        for file in base_path.glob(pattern):
            if file.is_file():
                dst = base_path / "archive/scripts" / file.name
                shutil.move(str(file), str(dst))
                print(f"✅ 移動: {file.name} → archive/scripts/")
                moved += 1
    return moved

def move_docs(base_path):
    """ドキュメントファイルを移動"""
    docs = [
        "README_v2.0.0.md",
        "README_v2.md",
        "RELEASE_NOTES_v0.48.0.md",
        "RELEASE_NOTES_v1.0.0.md",
        "RELEASE_NOTES_v2.4.0.md",
        "RELEASE_NOTES_v2.8.0.md",
    ]
    moved = 0
    for doc in docs:
        src = base_path / doc
        if src.exists():
            dst = base_path / "archive/docs" / doc
            shutil.move(str(src), str(dst))
            print(f"✅ 移動: {doc} → archive/docs/")
            moved += 1
    return moved

def move_directories(base_path):
    """ディレクトリを移動"""
    dirs_to_move = [
        "gui-backup",
        "playwright-report",
        "test-results",
        "prism-mcp-server",
        "prism-web",
        "website",
        "third_party",
    ]
    moved = 0
    for dir_name in dirs_to_move:
        src = base_path / dir_name
        if src.exists() and src.is_dir():
            dst = base_path / "archive" / dir_name
            if dst.exists():
                shutil.rmtree(str(dst))
            shutil.move(str(src), str(dst))
            print(f"✅ 移動: {dir_name}/ → archive/")
            moved += 1
    return moved

def main():
    """メイン処理"""
    print("\n" + "="*50)
    print("🗂️  リポジトリ整理整頓")
    print("="*50)
    print(f"開始時刻: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n")
    
    base_path = Path.cwd()
    print(f"作業ディレクトリ: {base_path}\n")
    
    # アーカイブディレクトリ作成
    print("📁 アーカイブディレクトリ作成中...")
    create_archive_dirs(base_path)
    print()
    
    # 設定ファイル移動
    print("📄 設定ファイル移動中...")
    configs_moved = move_configs(base_path)
    print(f"  移動数: {configs_moved}\n")
    
    # スクリプトファイル移動
    print("📜 スクリプトファイル移動中...")
    scripts_moved = move_scripts(base_path)
    print(f"  移動数: {scripts_moved}\n")
    
    # ドキュメントファイル移動
    print("📚 ドキュメントファイル移動中...")
    docs_moved = move_docs(base_path)
    print(f"  移動数: {docs_moved}\n")
    
    # ディレクトリ移動
    print("📂 ディレクトリ移動中...")
    dirs_moved = move_directories(base_path)
    print(f"  移動数: {dirs_moved}\n")
    
    total = configs_moved + scripts_moved + docs_moved + dirs_moved
    
    print("="*50)
    print("✅ 整理整頓完了！")
    print("="*50)
    print(f"  設定ファイル: {configs_moved}個")
    print(f"  スクリプト: {scripts_moved}個")
    print(f"  ドキュメント: {docs_moved}個")
    print(f"  ディレクトリ: {dirs_moved}個")
    print(f"  合計: {total}個")
    print(f"終了時刻: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n")

if __name__ == "__main__":
    main()
