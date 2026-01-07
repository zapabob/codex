#!/usr/bin/env python3
"""
Build completion waiting & auto installation script
"""

import os
import sys
import time
import subprocess
from pathlib import Path

def wait_for_build_completion():
    """ビルド完了を待機"""
    binary_path = Path("codex-rs/target/x86_64-pc-windows-msvc/release/codex.exe")

    print("[REBUILD] リリースビルド完了を待機中...")

    while not binary_path.exists():
        print(f"[WAIT] ビルド待機中... (30秒ごとに確認) - {time.strftime('%H:%M:%S')}")
        time.sleep(30)

    print("[OK] ビルド完了! バイナリが見つかりました。")
    print(f"[DIR] パス: {binary_path}")
    print(f"[INFO] サイズ: {binary_path.stat().st_size} bytes")

    return binary_path

def run_installation(binary_path):
    """インストールを実行"""
    print("\n[TOOL] プロセスキル＆インストールを実行...")

    # PowerShellスクリプトを実行
    ps_script = Path("scripts/install_with_kill.ps1")
    if ps_script.exists():
        cmd = [
            "powershell", "-ExecutionPolicy", "Bypass", "-File", str(ps_script),
            "-SourcePath", str(binary_path),
            "-TargetPath", "C:\\bin\\codex.exe",
            "-Force"
        ]
        result = subprocess.run(cmd, cwd=Path.cwd())
        if result.returncode == 0:
            print("[OK] インストール完了")
        else:
            print("[ERROR] インストール失敗")
            return False
    else:
        print("[ERROR] インストールスクリプトが見つかりません")
        return False

    return True

def verify_installation():
    """インストールを検証"""
    print("\n[SEARCH] インストール確認...")

    try:
        result = subprocess.run(["codex", "--version"], capture_output=True, text=True, timeout=10)
        if result.returncode == 0:
            print("[OK] インストール成功!")
            print(f"[INFO] バージョン: {result.stdout.strip()}")
            return True
        else:
            print("[ERROR] バージョン確認失敗")
            print(f"エラー: {result.stderr}")
            return False
    except FileNotFoundError:
        print("[ERROR] codexコマンドが見つかりません。PATHにC:\\binが含まれているか確認してください。")
        return False
    except Exception as e:
        print(f"[ERROR] 確認エラー: {e}")
        return False

def main():
    print("[START] Codex リリースビルド完了確認＆自動インストールシステム")
    print("=" * 60)

    # ビルド完了を待機
    binary_path = wait_for_build_completion()

    # インストールを実行
    if run_installation(binary_path):
        # インストールを検証
        verify_installation()
        print("\n[SUCCESS] すべての処理が完了しました!")
        print("[START] 新しいCodexを使用できます")
    else:
        print("\n[CRASH] インストールに失敗しました")
        sys.exit(1)

if __name__ == "__main__":
    main()