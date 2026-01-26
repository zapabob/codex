#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Rustビルド進捗可視化スクリプト (tqdm風)
高速差分ビルドの進捗を視覚化して残り時間と経過時間を表示
"""

import subprocess
import sys
import re
import time
import os
from datetime import datetime, timedelta

# Windows環境での文字エンコーディング対策
if sys.platform == 'win32':
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

def format_time(seconds):
    """秒数を読みやすい形式に変換"""
    if seconds < 60:
        return f"{seconds:.1f}秒"
    elif seconds < 3600:
        minutes = int(seconds // 60)
        secs = int(seconds % 60)
        return f"{minutes}分{secs}秒"
    else:
        hours = int(seconds // 3600)
        minutes = int((seconds % 3600) // 60)
        return f"{hours}時間{minutes}分"

def parse_cargo_output(line):
    """cargoの出力からコンパイル情報を抽出"""
    # "Compiling crate_name v1.2.3" のパターン
    compiling_match = re.search(r'Compiling\s+(\S+)', line)
    if compiling_match:
        return ('compiling', compiling_match.group(1))
    
    # "Finished" のパターン
    if 'Finished' in line:
        return ('finished', None)
    
    # エラーのパターン
    if 'error:' in line.lower():
        return ('error', line)
    
    return (None, None)

def build_with_progress(command, description="ビルド"):
    """進捗を可視化しながらビルドを実行"""
    print(f"\n{'='*60}")
    print(f"🚀 {description}を開始します...")
    print(f"{'='*60}\n")
    
    start_time = time.time()
    current_crate = None
    compiled_crates = []
    total_estimated = 100  # 推定値（実際の値は動的に更新）
    
    process = subprocess.Popen(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
        universal_newlines=True
    )
    
    try:
        while True:
            output = process.stdout.readline()
            if output == '' and process.poll() is not None:
                break
            
            if output:
                line = output.strip()
                event_type, data = parse_cargo_output(line)
                
                if event_type == 'compiling':
                    current_crate = data
                    compiled_crates.append(current_crate)
                    elapsed = time.time() - start_time
                    progress = len(compiled_crates)
                    
                    # 進捗バーを表示
                    bar_length = 40
                    filled = int(bar_length * progress / max(total_estimated, progress))
                    bar = '█' * filled + '░' * (bar_length - filled)
                    percentage = min(100, int(progress * 100 / max(total_estimated, progress)))
                    
                    print(f"\r[{bar}] {percentage}% | コンパイル中: {current_crate[:40]:<40} | 経過: {format_time(elapsed)}", end='', flush=True)
                
                elif event_type == 'finished':
                    elapsed = time.time() - start_time
                    print(f"\r{' ' * 100}", end='')  # 行をクリア
                    print(f"\r✅ ビルド完了！ | 経過時間: {format_time(elapsed)} | コンパイル済み: {len(compiled_crates)}個のクレート")
                    break
                
                elif event_type == 'error':
                    print(f"\n❌ エラー: {data}")
                    break
        
        return_code = process.poll()
        return return_code == 0
    
    except KeyboardInterrupt:
        print("\n\n⚠️  ビルドが中断されました")
        process.terminate()
        return False

if __name__ == "__main__":
    # 作業ディレクトリをcodex-rsに変更
    script_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(script_dir)
    
    # 環境変数を確認
    target_dir = os.environ.get('CARGO_TARGET_DIR', '')
    if target_dir:
        print(f"CARGO_TARGET_DIR: {target_dir}")
    
    # 高速差分ビルド（リリースモード）- ワークスペース全体
    print("高速差分ビルドを開始します... (workspace, custom-features)")
    success = build_with_progress(
        ["cargo", "build", "--workspace", "--features", "custom-features", "--release"],
        "Rustワークスペース (リリースビルド)"
    )
    
    if success:
        print("\n✅ ビルドが正常に完了しました！")
        sys.exit(0)
    else:
        print("\n❌ ビルドに失敗しました")
        sys.exit(1)
