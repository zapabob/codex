#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
高速差分ビルド・上書きインストールスクリプト
tqdm風の進捗表示で残り時間と経過時間を可視化
"""

import subprocess
import sys
import re
import time
import os
import shutil
from datetime import datetime
from pathlib import Path

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

def build_with_progress(command, description="ビルド"):
    """進捗を可視化しながらビルドを実行"""
    print(f"\n{'='*70}")
    print(f"🚀 {description}を開始します...")
    print(f"{'='*70}\n")
    
    start_time = time.time()
    current_crate = None
    compiled_crates = []
    total_estimated = 100
    
    process = subprocess.Popen(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
        universal_newlines=True,
        encoding='utf-8',
        errors='replace'
    )
    
    try:
        while True:
            output = process.stdout.readline()
            if output == '' and process.poll() is not None:
                break
            
            if output:
                line = output.strip()
                
                # "Compiling crate_name" のパターン
                compiling_match = re.search(r'Compiling\s+(\S+)', line)
                if compiling_match:
                    current_crate = compiling_match.group(1)
                    if current_crate not in compiled_crates:
                        compiled_crates.append(current_crate)
                    
                    elapsed = time.time() - start_time
                    progress = len(compiled_crates)
                    
                    # 進捗バーを表示
                    bar_length = 50
                    filled = int(bar_length * progress / max(total_estimated, progress))
                    bar = '█' * filled + '░' * (bar_length - filled)
                    percentage = min(100, int(progress * 100 / max(total_estimated, progress)))
                    
                    # 残り時間の推定
                    if progress > 0:
                        avg_time_per_crate = elapsed / progress
                        remaining_crates = max(0, total_estimated - progress)
                        estimated_remaining = avg_time_per_crate * remaining_crates
                        remaining_str = f" | 残り: {format_time(estimated_remaining)}"
                    else:
                        remaining_str = ""
                    
                    crate_display = current_crate[:35] if current_crate else "初期化中..."
                    print(f"\r[{bar}] {percentage}% | {crate_display:<35} | 経過: {format_time(elapsed)}{remaining_str}", end='', flush=True)
                
                # "Finished" のパターン
                elif 'Finished' in line and ('dev' in line or 'release' in line):
                    elapsed = time.time() - start_time
                    print(f"\r{' ' * 120}", end='')  # 行をクリア
                    print(f"\r✅ ビルド完了！ | 経過時間: {format_time(elapsed)} | コンパイル済み: {len(compiled_crates)}個のクレート\n")
                    break
                
                # エラーのパターン
                elif 'error:' in line.lower():
                    print(f"\n❌ エラー: {line[:100]}")
        
        return_code = process.poll()
        elapsed = time.time() - start_time
        return return_code == 0, elapsed, len(compiled_crates)
    
    except KeyboardInterrupt:
        print("\n\n⚠️  ビルドが中断されました")
        process.terminate()
        elapsed = time.time() - start_time
        return False, elapsed, len(compiled_crates)
    except Exception as e:
        print(f"\n❌ ビルド中にエラーが発生しました: {e}")
        elapsed = time.time() - start_time
        return False, elapsed, len(compiled_crates)

def install_binary(source_path, install_path):
    """バイナリを上書きインストール"""
    print(f"\n📦 バイナリをインストール中...")
    print(f"   ソース: {source_path}")
    print(f"   インストール先: {install_path}")
    
    try:
        # インストール先ディレクトリが存在するか確認
        install_dir = os.path.dirname(install_path)
        if not os.path.exists(install_dir):
            os.makedirs(install_dir, exist_ok=True)
            print(f"   ✅ ディレクトリ作成: {install_dir}")
        
        # 既存のバイナリをバックアップ（オプション）
        if os.path.exists(install_path):
            backup_path = f"{install_path}.backup-{datetime.now().strftime('%Y%m%d-%H%M%S')}"
            shutil.copy2(install_path, backup_path)
            print(f"   📋 バックアップ作成: {backup_path}")
        
        # バイナリをコピー
        shutil.copy2(source_path, install_path)
        file_size = os.path.getsize(install_path) / (1024 * 1024)  # MB
        print(f"   ✅ インストール完了 ({file_size:.2f} MB)")
        
        return True, file_size
    except Exception as e:
        print(f"   ❌ インストール失敗: {e}")
        return False, 0

def main():
    """メイン処理"""
    # 作業ディレクトリをcodex-rsに変更
    script_dir = os.path.dirname(os.path.abspath(__file__))
    
    # プロジェクトルートからcodex-rsディレクトリを探す
    codex_rs_dir = os.path.join(script_dir, "codex-rs")
    if not os.path.exists(codex_rs_dir):
        # 既にcodex-rsディレクトリ内にいる場合
        if os.path.basename(script_dir) == "codex-rs" or "codex-rs" in script_dir:
            codex_rs_dir = script_dir
        else:
            print(f"❌ codex-rsディレクトリが見つかりません: {codex_rs_dir}")
            print(f"   現在のディレクトリ: {script_dir}")
            sys.exit(1)
    
    os.chdir(codex_rs_dir)
    print(f"📁 作業ディレクトリ: {os.getcwd()}")
    
    # 1. 高速差分ビルド（codex-cli）
    print("\n" + "="*70)
    print("📦 Phase 1: 高速差分ビルド (codex-cli)")
    print("="*70)
    
    build_success, build_elapsed, build_count = build_with_progress(
        ["cargo", "build", "--release", "-p", "codex-cli"],
        "codex-cli (リリースビルド)"
    )
    
    if not build_success:
        print("\n❌ ビルドに失敗しました")
        sys.exit(1)
    
    # 2. バイナリの確認
    source_path = os.path.join("target", "release", "codex.exe")
    if not os.path.exists(source_path):
        print(f"\n❌ ビルド成果物が見つかりません: {source_path}")
        sys.exit(1)
    
    file_size = os.path.getsize(source_path) / (1024 * 1024)  # MB
    print(f"\n📦 ビルド成果物: {source_path} ({file_size:.2f} MB)")
    
    # 3. 上書きインストール
    print("\n" + "="*70)
    print("📥 Phase 2: バイナリ上書きインストール")
    print("="*70)
    
    install_path = os.path.join(os.environ.get('USERPROFILE', os.path.expanduser('~')), '.cargo', 'bin', 'codex.exe')
    
    install_success, installed_size = install_binary(source_path, install_path)
    
    if not install_success:
        print("\n❌ インストールに失敗しました")
        sys.exit(1)
    
    # 4. バージョン確認
    print("\n" + "="*70)
    print("🧪 Phase 3: バージョン確認")
    print("="*70)
    
    try:
        result = subprocess.run(
            ["codex", "--version"],
            capture_output=True,
            text=True,
            timeout=10,
            encoding='utf-8',
            errors='replace'
        )
        if result.returncode == 0:
            print(f"✅ バージョン確認成功:")
            print(f"   {result.stdout.strip()}")
        else:
            print(f"⚠️  バージョン確認で警告がありました")
    except Exception as e:
        print(f"⚠️  バージョン確認中にエラー: {e}")
    
    # 完了
    print("\n" + "="*70)
    print("🎉 全ての処理が正常に完了しました！")
    print("="*70)
    print(f"\n📊 実行サマリー:")
    print(f"   - ビルド時間: {format_time(build_elapsed)}")
    print(f"   - コンパイル済みクレート: {build_count}個")
    print(f"   - インストール先: {install_path}")
    print(f"   - ファイルサイズ: {installed_size:.2f} MB")
    
    # 完了音声を再生（Windows環境）
    if sys.platform == 'win32':
        audio_paths = [
            r"C:\Users\downl\Desktop\SO8T\.cursor\marisa_owattaze.wav",
            r"C:\Users\downl\Desktop\新しいフォルダー (4)\marisa_owattaze.wav",
        ]
        
        audio_played = False
        for audio_path in audio_paths:
            if os.path.exists(audio_path):
                try:
                    import winsound
                    print(f"\n🔊 完了音声を再生中: {audio_path}")
                    winsound.PlaySound(audio_path, winsound.SND_FILENAME | winsound.SND_SYNC)
                    print("✅ 音声を再生しました: 終わったぜ！")
                    audio_played = True
                    break
                except Exception as e:
                    print(f"⚠️  音声ファイルの再生に失敗しました: {e}")
                    continue
        
        if not audio_played:
            try:
                import winsound
                print(f"\n🔊 ピープ音を再生中...")
                winsound.Beep(1000, 500)
                print("✅ ピープ音を再生しました")
            except Exception as e:
                print(f"⚠️  音声の再生に失敗しました: {e}")

if __name__ == "__main__":
    main()
