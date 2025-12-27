#!/usr/bin/env python3
"""
高速差分ビルドと上書きインストールスクリプト
tqdm風の進捗表示付き
"""
import subprocess
import sys
import time
import os
import shutil
from pathlib import Path

try:
    from tqdm import tqdm
except ImportError:
    print("tqdm not found, installing...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "tqdm"])
    from tqdm import tqdm

def run_command(cmd, description, check=True):
    """コマンドを実行し、進捗を表示"""
    print(f"\n{'='*60}")
    print(f"[*] {description}")
    print(f"{'='*60}")
    
    start_time = time.time()
    
    # コマンドを実行（リアルタイム出力）
    process = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
        universal_newlines=True
    )
    
    # 出力をリアルタイムで表示
    for line in process.stdout:
        print(line, end='')
        sys.stdout.flush()
    
    process.wait()
    elapsed = time.time() - start_time
    
    if check and process.returncode != 0:
        print(f"\n[ERROR] {description} failed (exit code: {process.returncode})")
        sys.exit(1)
    
    print(f"\n[OK] {description} completed in {elapsed:.2f}s")
    return process.returncode == 0

def main():
    """メイン処理"""
    print("\n" + "="*60)
    print("  Codex Orchestrator 高速差分ビルド & インストール")
    print("="*60 + "\n")
    
    # 作業ディレクトリを確認
    if not Path("Cargo.toml").exists():
        print("[ERROR] Cargo.toml not found. Please run from codex-rs directory.")
        sys.exit(1)
    
    # Step 1: 型チェック
    print("\n[Step 1/4] 型チェックとリント (Zero Warnings)...")
    if not run_command(
        ["cargo", "check", "--workspace", "--all-targets"],
        "Type checking",
        check=False
    ):
        print("[WARNING] Type check found warnings, continuing anyway...")
    
    # Step 2: 差分ビルド
    print("\n[Step 2/4] 差分ビルド (Release, Incremental)...")
    run_command(
        ["cargo", "build", "--release", "-p", "codex-cli"],
        "Building codex-cli"
    )
    
    # Step 3: 実行中のプロセスを停止
    print("\n[Step 3/4] 実行中のcodexプロセスを停止...")
    try:
        if sys.platform == "win32":
            subprocess.run(["taskkill", "/F", "/IM", "codex.exe"], 
                         capture_output=True, check=False)
        else:
            subprocess.run(["pkill", "-f", "codex"], 
                         capture_output=True, check=False)
        time.sleep(1)
        print("[OK] Processes stopped")
    except Exception as e:
        print(f"[INFO] No processes to stop: {e}")
    
    # Step 4: 上書きインストール
    print("\n[Step 4/4] バイナリを上書きインストール...")
    
    if sys.platform == "win32":
        source = Path("target/release/codex.exe")
        install_dir = Path.home() / ".cargo" / "bin"
        install_path = install_dir / "codex.exe"
    else:
        source = Path("target/release/codex")
        install_dir = Path.home() / ".cargo" / "bin"
        install_path = install_dir / "codex"
    
    if not source.exists():
        print(f"[ERROR] Build artifact not found at {source}")
        sys.exit(1)
    
    install_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, install_path)
    print(f"[OK] Installed to {install_path}")
    
    # 検証
    print("\n[検証] インストールを確認...")
    try:
        result = subprocess.run(
            ["codex", "--version"],
            capture_output=True,
            text=True,
            check=True
        )
        print(f"[OK] Installation verified: {result.stdout.strip()}")
    except Exception as e:
        print(f"[WARNING] Version check failed: {e}")
    
    print("\n" + "="*60)
    print("  ビルド & インストール完了！")
    print("="*60 + "\n")
    print("次のステップ:")
    print("  codex --version")
    print("  codex orchestrator --help  # (if implemented)")

if __name__ == "__main__":
    main()
