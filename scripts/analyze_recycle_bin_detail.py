# -*- coding: utf-8 -*-
"""ごみ箱の詳細分析"""

import os

recycle_bin_path = r"C:\$Recycle.Bin"

if not os.path.exists(recycle_bin_path):
    print("ごみ箱が見つかりません")
    exit()

print("C:\\$Recycle.Bin 配下のユーザー別サイズ:")
print("-" * 60)

total_size = 0
user_sizes = []

try:
    for item in os.listdir(recycle_bin_path):
        item_path = os.path.join(recycle_bin_path, item)
        if os.path.isdir(item_path):
            size = 0
            file_count = 0
            try:
                for dirpath, dirnames, filenames in os.walk(item_path):
                    for filename in filenames:
                        try:
                            filepath = os.path.join(dirpath, filename)
                            file_size = os.path.getsize(filepath)
                            size += file_size
                            file_count += 1
                        except (OSError, PermissionError):
                            pass
            except (OSError, PermissionError):
                pass

            if size > 0:
                size_gb = size / (1024**3)
                user_sizes.append((item, size, size_gb, file_count))
                total_size += size
except (OSError, PermissionError) as e:
    print(f"エラー: {e}")

# サイズ順にソート
user_sizes.sort(key=lambda x: x[1], reverse=True)

for user_id, size_bytes, size_gb, file_count in user_sizes:
    print(f"ユーザーID: {user_id}")
    print(f"  サイズ: {size_gb:.2f} GB ({size_bytes:,} bytes)")
    print(f"  ファイル数: {file_count:,} 個")
    print()

print("-" * 60)
print(f"合計サイズ: {total_size / (1024**3):.2f} GB")
