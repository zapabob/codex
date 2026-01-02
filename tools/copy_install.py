#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Codex Build Artifact Copy & Install Script
ビルド成果物を直接コピーして上書きインストール
"""

import os
import shutil
import time
from tqdm import tqdm

def wait_for_build_completion(timeout=1800):  # 30分タイムアウト
    """ビルド完了を待つ"""
    print("Waiting for build completion...")

    start_time = time.time()
    with tqdm(total=timeout, desc="[WAIT] Build", unit="s") as pbar:
        while time.time() - start_time < timeout:
            cargo_count = len(os.popen('powershell "Get-Process -Name cargo -ErrorAction SilentlyContinue | Measure-Object | Select-Object -ExpandProperty Count" 2>nul').read().strip())

            if cargo_count == "0":
                print("Build completed!")
                return True

            pbar.n = int(time.time() - start_time)
            pbar.refresh()
            time.sleep(5)

    print("Timeout waiting for build completion")
    return False

def copy_and_install():
    """ビルド成果物をコピーしてインストール"""
    print("Codex Build Artifact Copy & Install")
    print("=" * 50)

    # ビルド成果物のパス
    source_path = os.path.join(os.path.dirname(__file__), "codex-rs", "target", "release", "codex.exe")

    # インストール先パス
    dest_path = r"C:\Users\downl\.cargo\bin\codex.exe"

    print(f"Source: {source_path}")
    print(f"Destination: {dest_path}")
    print()

    # ビルド成果物の存在確認
    if not os.path.exists(source_path):
        print(f"[ERROR] Build artifact not found: {source_path}")
        print("[INFO] Build may not have completed yet")
        return False

    # バックアップ作成
    backup_path = dest_path + ".backup"
    if os.path.exists(dest_path):
        print("[BACKUP] Creating backup of current installation...")
        shutil.copy2(dest_path, backup_path)
        print(f"[BACKUP] Backup created: {backup_path}")

    # 上書きコピー
    print("[COPY] Copying build artifact...")
    try:
        shutil.copy2(source_path, dest_path)
        print("[SUCCESS] Build artifact copied successfully!")
        return True
    except Exception as e:
        print(f"[ERROR] Failed to copy: {e}")
        return False

def verify_installation():
    """インストール結果の確認"""
    print("[VERIFY] Verifying installation...")

    try:
        import subprocess
        result = subprocess.run(
            ["codex", "--version"],
            capture_output=True,
            text=True,
            timeout=10
        )

        if result.returncode == 0:
            version = result.stdout.strip()
            print(f"[VERSION] {version}")

            if "2.8.3" in version:
                print("[SUCCESS] Codex v2.8.3 successfully installed!")
                print("[COMPLETE] Semantic versioning issue resolved!")
                return True
            else:
                print(f"[INFO] Version is {version} (expected 2.8.3)")
                return False
        else:
            print("[ERROR] Version check failed")
            return False
    except Exception as e:
        print(f"[ERROR] Verification failed: {e}")
        return False

def main():
    print("Starting Codex Copy & Install Process")
    print()

    # ビルド完了を待つ
    if not wait_for_build_completion():
        print("[ERROR] Build did not complete within timeout")
        return

    print()

    # コピー&インストール
    if copy_and_install():
        print()

        # 検証
        if verify_installation():
            print()
            print("🎉 Installation completed successfully!")
        else:
            print()
            print("⚠️ Installation completed but verification failed")
    else:
        print("[ERROR] Copy and install failed")

if __name__ == "__main__":
    main()