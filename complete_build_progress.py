#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Codex v2.8.3 Complete Build Script
Fast incremental build for all crates
"""

import subprocess
import sys
import time
from tqdm import tqdm
import os

def run_build_step(cmd, description, timeout=300):
    """ビルドステップを実行"""
    print(f"[START] {description}")

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
            print(f"[SUCCESS] {description}")
            return True, result.stdout, result.stderr
        else:
            print(f"[ERROR] {description}")
            print("STDOUT:", result.stdout[-2000:])  # 最後の2000文字
            print("STDERR:", result.stderr[-2000:])  # 最後の2000文字
            return False, result.stdout, result.stderr

    except subprocess.TimeoutExpired:
        print(f"[TIMEOUT] {description} - {timeout}秒でタイムアウト")
        return False, "", "Timeout"

def main():
    print("Codex v2.8.3 Complete Build System")
    print("=" * 60)
    print("Fast incremental build for all crates")
    print()

    # ビルド手順
    build_steps = [
        # Phase 1: 基本チェック
        ("cargo check -p mcp-types", "MCP Types check"),
        ("cargo check -p codex-common", "Common crate check"),

        # Phase 2: コアコンポーネント
        ("cargo check -p codex-core", "Core crate check"),
        ("cargo check -p codex-cli", "CLI crate check"),

        # Phase 3: 拡張機能
        ("cargo check -p codex-chrome-mcp-bridge", "Chrome MCP Bridge check"),
        ("cargo check -p codex-deep-research", "Deep Research check"),
        ("cargo check -p codex-tui", "TUI check"),

        # Phase 4: サーバー/プロトコル
        ("cargo check -p codex-protocol", "Protocol check"),
        ("cargo check -p codex-app-server", "App Server check"),

        # Phase 5: 最終リリースビルド
        ("cargo build --release -p codex-cli", "CLI Release build"),
        ("cargo install --path cli --force", "CLI install"),
    ]

    success_count = 0
    total_steps = len(build_steps)

    print(f"Total steps: {total_steps}")
    print()

    with tqdm(total=total_steps, desc="[BUILD] Overall Progress", bar_format='{desc}: {percentage:3.0f}%|{bar}| {n_fmt}/{total_fmt}') as pbar:
        for i, (cmd, desc) in enumerate(build_steps, 1):
            pbar.set_description(f"[STEP {i:2d}/{total_steps}] {desc}")

            success, stdout, stderr = run_build_step(cmd, desc)

            if success:
                success_count += 1
            else:
                print(f"[FAILED] Step {i} failed: {desc}")
                if "cargo build --release" in cmd:
                    print("[INFO] Release build failed - continue with checks")
                else:
                    print("[ERROR] Critical error - stopping build")
                    break

            pbar.update(1)
            time.sleep(0.2)  # visual effect

    print()
    print("=" * 60)
    print(f"[RESULT] Build Results: {success_count}/{total_steps} steps successful")

    if success_count == total_steps:
        print("[SUCCESS] All build steps completed!")
        print("[DONE] Codex v2.8.3 fully built")
        print("[CHECK] Run 'codex --version' to verify")
    elif success_count >= total_steps * 0.8:  # 80%以上成功
        print("[WARNING] Most build steps successful")
        print("[FIX] Fix remaining errors for complete build")
        print("[LOG] Check error logs for details")
    else:
        print("[ERROR] Many build failures occurred")
        print("[DEBUG] Check error logs for root cause")

    # 最終バージョン確認
    print()
    print("[VERIFY] Final version check...")
    try:
        result = subprocess.run(
            "codex --version",
            shell=True,
            capture_output=True,
            text=True,
            timeout=10
        )
        if result.returncode == 0:
            version = result.stdout.strip()
            print(f"[VERSION] {version}")
            if "2.8.3" in version:
                print("[TARGET] v2.8.3 build successful!")
            else:
                print("[WARNING] Version different from expected")
        else:
            print("[ERROR] Version check failed")
    except:
        print("[ERROR] codex command not found")

if __name__ == "__main__":
    main()