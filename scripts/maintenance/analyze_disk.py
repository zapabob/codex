# -*- coding: utf-8 -*-
"""
Cドライブの容量分析スクリプト
"""

import os
import subprocess
from datetime import datetime
from pathlib import Path
from typing import List, Tuple


def get_dir_size(path: str) -> int:
    """ディレクトリのサイズを取得（バイト）"""
    total = 0
    try:
        if os.path.isfile(path):
            return os.path.getsize(path)

        for dirpath, dirnames, filenames in os.walk(path):
            for filename in filenames:
                try:
                    filepath = os.path.join(dirpath, filename)
                    total += os.path.getsize(filepath)
                except (OSError, PermissionError):
                    pass
    except (OSError, PermissionError):
        pass
    return total


def format_size(size_bytes: int) -> Tuple[float, float]:
    """サイズをGBとTBで返す"""
    gb = size_bytes / (1024**3)
    tb = size_bytes / (1024**4)
    return round(gb, 2), round(tb, 3)


def analyze_disk():
    """Cドライブを分析"""
    print("Cドライブの容量分析を開始します...")

    results = []

    # 主要なディレクトリをチェック
    dirs = [
        r"C:\Users",
        r"C:\Program Files",
        r"C:\Program Files (x86)",
        r"C:\Windows",
        r"C:\ProgramData",
        r"C:\Temp",
        r"C:\Windows\Temp",
        r"C:\$Recycle.Bin",
    ]

    # システムファイル
    system_files = [
        r"C:\pagefile.sys",
        r"C:\hiberfil.sys",
        r"C:\swapfile.sys",
    ]

    # ディレクトリを分析
    for dir_path in dirs:
        if os.path.exists(dir_path):
            print(f"分析中: {dir_path}")
            try:
                size = get_dir_size(dir_path)
                if size > 0:
                    gb, tb = format_size(size)
                    item_type = "File" if os.path.isfile(dir_path) else "Directory"
                    results.append(
                        {
                            "path": dir_path,
                            "type": item_type,
                            "size_bytes": size,
                            "size_gb": gb,
                            "size_tb": tb,
                        }
                    )
            except Exception as e:
                print(f"  エラー: {e}")

    # システムファイルを分析
    for file_path in system_files:
        if os.path.exists(file_path):
            print(f"分析中: {file_path}")
            try:
                size = os.path.getsize(file_path)
                if size > 0:
                    gb, tb = format_size(size)
                    results.append(
                        {
                            "path": file_path,
                            "type": "System File",
                            "size_bytes": size,
                            "size_gb": gb,
                            "size_tb": tb,
                        }
                    )
            except Exception as e:
                print(f"  エラー: {e}")

    # C:\Users配下の各ユーザーディレクトリもチェック
    users_dir = r"C:\Users"
    if os.path.exists(users_dir):
        print("C:\\Users配下のユーザーディレクトリを分析中...")
        try:
            for item in os.listdir(users_dir):
                user_path = os.path.join(users_dir, item)
                if os.path.isdir(user_path) and not item.startswith("."):
                    print(f"  分析中: {user_path}")
                    try:
                        size = get_dir_size(user_path)
                        if size > 0:
                            gb, tb = format_size(size)
                            results.append(
                                {
                                    "path": user_path,
                                    "type": "User Directory",
                                    "size_bytes": size,
                                    "size_gb": gb,
                                    "size_tb": tb,
                                }
                            )
                    except Exception as e:
                        print(f"    エラー: {e}")
        except Exception as e:
            print(f"  エラー: {e}")

    # 結果をソート
    results.sort(key=lambda x: x["size_bytes"], reverse=True)

    # 合計サイズを計算
    total_size = sum(r["size_bytes"] for r in results)
    total_gb, total_tb = format_size(total_size)

    # 出力ファイル名
    timestamp = datetime.now().strftime("%Y-%m-%d_%H%M%S")
    output_file = (
        Path(r"C:\Users\downl\Desktop\codex-main\_docs")
        / f"Cドライブ容量分析_{timestamp}.md"
    )

    # Markdown形式で出力
    md_lines = []
    md_lines.append("# Cドライブ容量分析レポート\n")
    md_lines.append(f"**分析日時**: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n")
    md_lines.append(f"**合計サイズ**: {total_gb} GB ({total_tb} TB)\n")
    md_lines.append("## ディレクトリ/ファイル別サイズ一覧\n")
    md_lines.append("| 順位 | パス | タイプ | サイズ (GB) | サイズ (TB) |")
    md_lines.append("|------|------|--------|-------------|-------------|")

    for rank, item in enumerate(results, 1):
        path_escaped = item["path"].replace("|", "\\|")
        md_lines.append(
            f"| {rank} | `{path_escaped}` | {item['type']} | {item['size_gb']} GB | {item['size_tb']} TB |"
        )

    md_lines.append("\n## 詳細情報\n")
    md_lines.append("### トップ10\n")

    for rank, item in enumerate(results[:10], 1):
        md_lines.append(f"\n#### {rank}. {item['path']}\n")
        md_lines.append(f"- **タイプ**: {item['type']}")
        md_lines.append(f"- **サイズ**: {item['size_gb']} GB ({item['size_tb']} TB)")
        md_lines.append(f"- **サイズ (バイト)**: {item['size_bytes']:,} bytes\n")

    md_lines.append("\n## 推奨アクション\n")
    md_lines.append(
        "1. **一時ファイルの削除**: C:\\Windows\\Temp や C:\\Temp をクリーンアップ"
    )
    md_lines.append(
        "2. **ユーザーディレクトリの整理**: 大きなファイルや不要なデータを削除"
    )
    md_lines.append(
        "3. **プログラムのアンインストール**: 使用していないアプリケーションを削除"
    )
    md_lines.append(
        "4. **ディスククリーンアップ**: Windowsのディスククリーンアップツールを実行"
    )
    md_lines.append("\n---\n")
    md_lines.append("*このレポートは自動生成されました。*\n")

    # ファイルに出力
    output_file.parent.mkdir(parents=True, exist_ok=True)
    with open(output_file, "w", encoding="utf-8") as f:
        f.write("\n".join(md_lines))

    print(f"\n分析完了！")
    print(f"結果を保存しました: {output_file}")
    print(f"\nトップ5:")
    for rank, item in enumerate(results[:5], 1):
        print(f"  {rank}. {item['path']}: {item['size_gb']} GB")


if __name__ == "__main__":
    analyze_disk()
