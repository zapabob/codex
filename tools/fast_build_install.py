#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Codex Fast Build & Install Script
高速差分ビルドでCLIをビルドし、上書きインストール
"""

import subprocess
import sys
import time
from tqdm import tqdm
import os

def run_command(cmd, description, timeout=180):
    """コマンドを実行"""
    print(f"[EXEC] {description}")

    try:
        result = subprocess.run(
            cmd,
            shell=True,
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=os.path.join(os.path.dirname(__file__), "codex-rs")
        )

        if result.returncode == 0:
            print(f"[OK] {description} completed")
            return True
        else:
            print(f"[FAIL] {description} failed")
            # エラーログを表示（短く）
            if result.stderr:
                lines = result.stderr.strip().split('\n')
                for line in lines[-5:]:  # 最後の5行
                    if line.strip():
                        print(f"  > {line}")
            return False

    except subprocess.TimeoutExpired:
        print(f"[TIMEOUT] {description} - {timeout}s timeout")
        return False

def main():
    print("Codex Fast Build & Install v2.8.3")
    print("=" * 50)
    print("High-speed incremental build + binary overwrite install")
    print()

    # 高速ビルド手順（改良版）
    build_steps = [
        ("cargo update --quiet", "Update dependencies (quiet)"),
        ("cargo clean --quiet", "Clean all build artifacts"),
        ("cargo check -p codex-cli --quiet", "CLI pre-check (quiet)"),
        ("cargo build --release -p codex-cli --quiet", "Release build (fast, quiet)"),
        ("cargo install --path cli --force --quiet", "Binary install (overwrite, quiet)"),
    ]

    success_count = 0

    print(f"Build steps: {len(build_steps)}")
    print()

    with tqdm(total=len(build_steps), desc="[FAST] Building", bar_format='{desc}: {percentage:3.0f}%|{bar}| {n_fmt}/{total_fmt}') as pbar:
        for i, (cmd, desc) in enumerate(build_steps, 1):
            pbar.set_description(f"[STEP {i}] {desc}")

            if run_command(cmd, desc, timeout=600):  # 10分タイムアウト
                success_count += 1
                pbar.update(1)
                print(f"[OK] Step {i} completed successfully")
            else:
                print(f"[FAIL] Step {i} failed: {desc}")
                break

            time.sleep(0.5)

    print()
    print("=" * 50)

    if success_count == len(build_steps):
        print("[SUCCESS] Fast build & install completed!")

        # バージョン確認
        print("[CHECK] Version verification...")
        for attempt in range(3):  # 3回までリトライ
            try:
                result = subprocess.run(
                    "codex --version",
                    shell=True,
                    capture_output=True,
                    text=True,
                    timeout=5
                )
                if result.returncode == 0:
                    version = result.stdout.strip()
                    print(f"[VERSION] {version}")
                    if "2.8.3" in version:
                        print("[TARGET] Codex v2.8.3 successfully installed!")
                        print("[COMPLETE] Semantic versioning issue resolved!")
                        break
                    else:
                        print(f"[INFO] Version is {version} (expected 2.8.3)")
                        if attempt == 2:
                            print("[WARN] Version not updated to 2.8.3")
                else:
                    print(f"[WARN] Version check failed (attempt {attempt + 1})")
                    time.sleep(2)
            except:
                print(f"[ERROR] Codex command not found (attempt {attempt + 1})")
                time.sleep(2)

    else:
        print(f"[PARTIAL] {success_count}/{len(build_steps)} steps completed")
        print("[INFO] Build incomplete - check logs above")
        print("[RETRY] Run again or check for blocking processes")

if __name__ == "__main__":
    main()