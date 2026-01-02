#!/usr/bin/env python3
"""
Codexビルド進捗可視化スクリプト
tqdmを使ってRustビルドの進捗を視覚化
"""

import subprocess
import sys
import time
from tqdm import tqdm
import threading
import queue
import os

def run_command_with_progress(cmd, description="実行中"):
    """コマンドを実行し、進捗を表示"""
    print(f"[START] {description}を開始します...")

    # ダミーのプログレスバー（実際のビルド時間を見積もって）
    estimated_steps = 100

    with tqdm(total=estimated_steps,
              desc=f"[BUILD] {description}",
              bar_format='{desc}: {percentage:3.0f}%|{bar}| {n_fmt}/{total_fmt} [{elapsed}<{remaining}]') as pbar:

        # コマンドを開始
        process = subprocess.Popen(
            cmd,
            shell=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            encoding='utf-8',
            cwd=os.path.dirname(__file__)
        )

        start_time = time.time()
        lines_processed = 0

        while True:
            output = process.stdout.readline()
            if output == '' and process.poll() is not None:
                break
            if output:
                lines_processed += 1
                # 進捗を更新（出力行数に基づく）
                progress = min(lines_processed * 2, estimated_steps - 10)
                pbar.n = progress
                pbar.refresh()

                # リアルタイムで出力も表示
                print(f"  > {output.strip()}")

        # 最終進捗
        pbar.n = estimated_steps
        pbar.refresh()

        return_code = process.poll()

        if return_code == 0:
            print(f"[SUCCESS] {description}が完了しました！")
            return True
        else:
            print(f"[ERROR] {description}が失敗しました (exit code: {return_code})")
            return False

def main():
    print("Codex Build System v2.0 - zapabob Extended")
    print("=" * 50)

    # ビルドコマンド
    commands = [
        ("cd codex-rs && cargo check --all-features", "Cargo Check (All Features)"),
        ("cd codex-rs && cargo fmt --check", "Code Format Check"),
        ("cd codex-rs && cargo clippy --all-features -- -D warnings", "Clippy Linter (No Warnings)"),
    ]

    success_count = 0

    for cmd, desc in commands:
        if run_command_with_progress(cmd, desc):
            success_count += 1
        else:
            print(f"[WARNING] {desc}で問題が発生しましたが、続行します...")
        print()

    print(f"[RESULT] Build Results: {success_count}/{len(commands)} successful")

    if success_count == len(commands):
        print("[COMPLETE] すべてのビルドチェックが完了しました！")
        print("[NEXT] 次は 'cargo build --release' を実行できます")
    else:
        print("[WARNING] 一部のチェックが失敗しました。詳細を確認してください")

if __name__ == "__main__":
    main()