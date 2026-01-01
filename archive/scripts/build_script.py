#!/usr/bin/env python3
"""
Codex Rust高速差分ビルドスクリプト
tqdm風プログレスバーでビルド進行状況を表示
"""

import subprocess
import time
from tqdm import tqdm
import sys
import os

def main():
    print('[START] 高速差分ビルド開始！なんjだぜ！')
    print('=' * 50)

    # 作業ディレクトリ設定
    script_dir = os.path.dirname(os.path.abspath(__file__))
    codex_rs_dir = os.path.join(script_dir, 'codex-rs')
    os.chdir(codex_rs_dir)

    print(f'作業ディレクトリ: {codex_rs_dir}')

    # ビルドコマンド
    build_cmd = ['cargo', 'build', '--release', '-p', 'codex-cli', '-j', '16']

    print(f'実行コマンド: {" ".join(build_cmd)}')
    print('プログレスバーでビルド進行状況を表示するぜ！')
    print()

    # ビルド開始
    start_time = time.time()

    # tqdmでプログレス表示しながらビルド
    with tqdm(total=100, desc='[BUILD] ビルド進行中', unit='%', ncols=80,
              bar_format='{desc}: {percentage:3.0f}%|{bar}| {n_fmt}/{total_fmt} [{elapsed}<{remaining}, {rate_fmt}]') as pbar:

        # ビルドプロセス開始
        process = subprocess.Popen(
            build_cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            universal_newlines=True,
            bufsize=1
        )

        # 出力をリアルタイムで読み取り
        lines_processed = 0
        total_lines_expected = 200  # 予想される出力行数

        while True:
            output = process.stdout.readline()
            if output == '' and process.poll() is not None:
                break
            if output:
                lines_processed += 1
                # プログレスを更新（0-90%）
                progress = min(90, int((lines_processed / total_lines_expected) * 90))
                pbar.n = progress
                pbar.refresh()

                # 重要な出力のみ表示
                if any(keyword in output.lower() for keyword in ['compiling', 'finished', 'error', 'warning']):
                    print(f'[{time.strftime("%H:%M:%S")}] {output.strip()}')

        # ビルド完了待ち
        return_code = process.poll()

        if return_code == 0:
            pbar.n = 95
            pbar.desc = '[SUCCESS] ビルド成功'
            pbar.refresh()
            print('\n[SUCCESS] ビルド成功！なんj最高だぜ！')
        else:
            pbar.n = 0
            pbar.desc = '[ERROR] ビルド失敗'
            pbar.refresh()
            print(f'\n[ERROR] ビルド失敗！リターンコード: {return_code}')
            return 1

    # ビルド時間計算
    build_time = time.time() - start_time
    print(f'ビルド時間: {build_time:.2f}秒')

    # インストールフェーズ
    print('\n[INSTALL] バイナリ上書きインストール開始...')
    pbar.n = 95
    pbar.desc = '[INSTALL] インストール中'
    pbar.refresh()

    install_cmd = ['cargo', 'install', '--path', 'cli', '--force']
    print(f'実行コマンド: {" ".join(install_cmd)}')

    install_start = time.time()
    install_result = subprocess.run(install_cmd, capture_output=True, text=True)

    install_time = time.time() - install_start

    if install_result.returncode == 0:
        pbar.n = 100
        pbar.desc = '[DONE] インストール完了'
        pbar.refresh()
        print('\n[SUCCESS] インストール成功！codex-cliが上書きインストールされたぜ！')
        print(f'インストール時間: {install_time:.2f}秒')

        # バージョン確認
        version_result = subprocess.run(['codex', '--version'], capture_output=True, text=True)
        if version_result.returncode == 0:
            print(f'現在のバージョン: {version_result.stdout.strip()}')
        else:
            print('バージョン確認に失敗したけど、インストールは成功したはずだぜ！')

    else:
        pbar.desc = '[ERROR] インストール失敗'
        pbar.refresh()
        print('\n[ERROR] インストール失敗！')
        print('STDOUT:', install_result.stdout)
        print('STDERR:', install_result.stderr)
        return 1

    total_time = time.time() - start_time
    print(f'\n[TIME] トータル時間: {total_time:.2f}秒')
    print('=' * 50)
    print('[COMPLETE] 高速差分ビルド＆インストール完了！なんj最高の出来だぜ！！')

    return 0

if __name__ == '__main__':
    sys.exit(main())
