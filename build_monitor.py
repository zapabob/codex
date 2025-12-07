#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Codex ビルド監視スクリプト (tqdm風視覚化)
高速差分ビルドを活用したRustコンパイル監視
"""

import subprocess
import time
import re
from datetime import datetime
from tqdm import tqdm
import psutil
import os
import sys

class BuildMonitor:
    def __init__(self):
        self.start_time = time.time()
        self.last_update = self.start_time
        self.build_progress = {}
        self.total_files = 0
        self.compiled_files = 0

    def parse_build_output(self, line):
        """ビルド出力を解析して進捗を更新"""
        # コンパイル中のファイル数を取得
        compiling_match = re.search(r'Compiling (\d+) packages?', line)
        if compiling_match:
            self.total_files = max(self.total_files, int(compiling_match.group(1)))

        # 完了したファイル数をカウント
        if 'Compiling' in line and 'v' in line:
            self.compiled_files += 1

        # 進捗パーセンテージを計算
        if self.total_files > 0:
            progress = min(100, (self.compiled_files / self.total_files) * 100)
            return progress
        return None

    def format_time(self, seconds):
        """時間を読みやすい形式にフォーマット"""
        hours, remainder = divmod(int(seconds), 3600)
        minutes, seconds = divmod(remainder, 60)
        if hours > 0:
            return "02d"
        elif minutes > 0:
            return "02d"
        else:
            return "05.2f"

    def get_system_info(self):
        """システム情報を取得"""
        try:
            cpu_percent = psutil.cpu_percent(interval=1)
            memory = psutil.virtual_memory()
            return ".1f"
        except:
            return "取得不可"

    def run_build(self, command):
        """ビルドを実行し進捗を監視"""
        print("🚀 Codex ビルド開始")
        print(f"開始時刻: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print(f"コマンド: {' '.join(command)}")
        print("=" * 60)

        # tqdmプログレスバー初期化
        pbar = tqdm(total=100, unit='%', ncols=100,
                   bar_format='{l_bar}{bar}| {n_fmt}/{total_fmt} [{elapsed}<{remaining}, {rate_fmt}]')

        process = subprocess.Popen(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
            universal_newlines=True
        )

        last_progress = 0

        try:
            while True:
                output = process.stdout.readline()
                if output == '' and process.poll() is not None:
                    break

                if output:
                    print(output.strip())

                    # 進捗を解析
                    progress = self.parse_build_output(output)
                    if progress is not None and progress > last_progress:
                        pbar.update(progress - last_progress)
                        last_progress = progress

                    # システム情報を定期的に更新
                    current_time = time.time()
                    if current_time - self.last_update >= 5:  # 5秒ごとに更新
                        system_info = self.get_system_info()
                        pbar.set_description(f"ビルド中 {system_info}")
                        self.last_update = current_time

            # プロセス完了を待つ
            return_code = process.poll()

            # 残りの進捗を完了
            if last_progress < 100:
                pbar.update(100 - last_progress)

            pbar.close()

            # 結果表示
            total_time = time.time() - self.start_time
            print("\n" + "=" * 60)
            print("🏁 ビルド完了")
            print(f"終了時刻: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
            print(f"総時間: {self.format_time(total_time)}")
            print(f"CPU/メモリ使用: {self.get_system_info()}")
            print(f"終了コード: {return_code}")

            if return_code == 0:
                print("✅ ビルド成功!")
                return True
            else:
                print("❌ ビルド失敗!")
                return False

        except KeyboardInterrupt:
            print("\n⚠️  ビルド中断")
            process.terminate()
            pbar.close()
            return False
        except Exception as e:
            print(f"\n💥 エラー発生: {e}")
            process.terminate()
            pbar.close()
            return False

def main():
    """メイン関数"""
    # Rustプロジェクトのルートに移動
    project_root = r"C:\Users\downl\Desktop\codex-main\codex-rs"

    # ビルドコマンド
    build_commands = [
        ["cargo", "clean"],  # クリーン
        ["cargo", "build", "--release", "-p", "codex-cli"],  # CLIビルド
        ["cargo", "build", "--release", "-p", "codex-tui"],  # TUIビルド
        ["cargo", "build", "--release", "-p", "codex-core"], # Coreビルド
    ]

    monitor = BuildMonitor()

    for i, cmd in enumerate(build_commands, 1):
        print(f"\n📦 ステップ {i}/{len(build_commands)}: {' '.join(cmd)}")
        os.chdir(project_root)

        success = monitor.run_build(cmd)
        if not success:
            print(f"❌ ステップ {i} が失敗しました")
            sys.exit(1)

        # ステップ間で少し待機
        time.sleep(1)

    print("\n🎉 すべてのビルドが完了しました!")

if __name__ == "__main__":
    main()
