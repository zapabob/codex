#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
高速差分ビルドとインストールスクリプト
git index修復 + Rust高速差分ビルド + バイナリ上書きインストール
"""

import os
import sys
import subprocess
import time
from pathlib import Path
from datetime import datetime

def run_cmd(cmd, cwd=None, check=False):
    """コマンドを実行して結果を返す"""
    print(f"[実行] {cmd}", flush=True)
    result = subprocess.run(
        cmd,
        shell=True,
        cwd=cwd,
        capture_output=True,
        text=True,
        encoding='utf-8',
        errors='replace'
    )
    if result.stdout:
        print(result.stdout, flush=True)
    if result.stderr:
        print(result.stderr, file=sys.stderr, flush=True)
    if check and result.returncode != 0:
        raise subprocess.CalledProcessError(result.returncode, cmd)
    return result

def fix_git_index():
    """git indexを修復"""
    print("\n" + "="*50)
    print("🔧 Git Index修復")
    print("="*50)
    
    repo_root = Path.cwd()
    git_index = repo_root / ".git" / "index"
    
    if git_index.exists():
        print(f"  git index削除: {git_index}")
        try:
            git_index.unlink()
            print("  ✅ git index削除完了")
        except Exception as e:
            print(f"  ⚠️  git index削除失敗: {e}")
    
    # git resetでindex再構築を試みる
    print("  git reset実行中...")
    result = run_cmd("git reset", check=False)
    if result.returncode == 0:
        print("  ✅ git index修復完了")
    else:
        print("  ⚠️  git reset失敗（続行します）")

def kill_codex_processes():
    """実行中のcodexプロセスをキル"""
    print("\n" + "="*50)
    print("🔪 プロセスキル")
    print("="*50)
    
    try:
        # PowerShellでcodexプロセスを検索してキル
        cmd = 'Get-Process | Where-Object {$_.ProcessName -match "codex"} | Stop-Process -Force -ErrorAction SilentlyContinue'
        result = run_cmd(f'powershell -Command "{cmd}"', check=False)
        if result.returncode == 0:
            print("  ✅ codexプロセスキル完了")
        else:
            print("  ℹ️  実行中のcodexプロセスなし")
    except Exception as e:
        print(f"  ⚠️  プロセスキル失敗: {e}")

def build_codex():
    """高速差分ビルド実行"""
    print("\n" + "="*50)
    print("🦀 Rust高速差分ビルド")
    print("="*50)
    
    codex_rs = Path("codex-rs")
    if not codex_rs.exists():
        print(f"  ❌ {codex_rs}が見つかりません")
        return False
    
    print(f"  ディレクトリ: {codex_rs.absolute()}")
    print("  パッケージ: codex-cli")
    print("  プロファイル: release")
    print("  インクリメンタル: 有効")
    print()
    
    start_time = time.time()
    
    # 環境変数設定
    env = os.environ.copy()
    env["CARGO_INCREMENTAL"] = "1"
    
    # ビルド実行
    cmd = "cargo build --release -p codex-cli"
    result = run_cmd(cmd, cwd=codex_rs, check=False)
    
    build_time = time.time() - start_time
    
    if result.returncode == 0:
        print(f"\n  ✅ ビルド完了 ({build_time:.2f}秒)")
        return True
    else:
        print(f"\n  ❌ ビルド失敗 ({build_time:.2f}秒)")
        return False

def install_codex():
    """バイナリを上書きインストール"""
    print("\n" + "="*50)
    print("📦 バイナリ上書きインストール")
    print("="*50)
    
    codex_rs = Path("codex-rs")
    if not codex_rs.exists():
        return False
    
    print("  パス: cli")
    print("  モード: --force (上書き)")
    print()
    
    start_time = time.time()
    
    cmd = "cargo install --path cli --force"
    result = run_cmd(cmd, cwd=codex_rs, check=False)
    
    install_time = time.time() - start_time
    
    if result.returncode == 0:
        print(f"\n  ✅ インストール完了 ({install_time:.2f}秒)")
        return True
    else:
        print(f"\n  ❌ インストール失敗 ({install_time:.2f}秒)")
        return False

def verify_installation():
    """インストール確認"""
    print("\n" + "="*50)
    print("🔍 インストール確認")
    print("="*50)
    
    result = run_cmd("codex --version", check=False)
    if result.returncode == 0:
        print("  ✅ インストール確認完了")
        return True
    else:
        print("  ⚠️  バージョン確認失敗")
        return False

def main():
    """メイン処理"""
    print("\n" + "="*50)
    print("🚀 Codex 高速差分ビルド & インストール")
    print("="*50)
    print(f"開始時刻: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    
    # 1. git index修復
    fix_git_index()
    
    # 2. プロセスキル
    kill_codex_processes()
    
    # 3. ビルド
    if not build_codex():
        print("\n❌ ビルド失敗で終了")
        sys.exit(1)
    
    # 4. インストール
    if not install_codex():
        print("\n❌ インストール失敗で終了")
        sys.exit(1)
    
    # 5. 確認
    verify_installation()
    
    print("\n" + "="*50)
    print("✅ 完了！")
    print("="*50)
    print(f"終了時刻: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")

if __name__ == "__main__":
    main()
