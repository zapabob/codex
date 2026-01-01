# -*- coding: utf-8 -*-
"""ごみ箱のサイズを確認"""
import os

path = r'C:\$Recycle.Bin'

if os.path.exists(path):
    total_size = 0
    file_count = 0
    
    try:
        for dirpath, dirnames, filenames in os.walk(path):
            for filename in filenames:
                try:
                    filepath = os.path.join(dirpath, filename)
                    size = os.path.getsize(filepath)
                    total_size += size
                    file_count += 1
                except (OSError, PermissionError):
                    pass
    except (OSError, PermissionError) as e:
        print(f"エラー: {e}")
    
    if total_size > 0:
        size_gb = total_size / (1024 ** 3)
        print(f"C:\\$Recycle.Bin の現在のサイズ: {size_gb:.2f} GB")
        print(f"ファイル数: {file_count:,} 個")
    else:
        print("C:\\$Recycle.Bin は空です")
else:
    print("C:\\$Recycle.Bin が存在しません")
