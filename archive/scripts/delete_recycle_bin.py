# -*- coding: utf-8 -*-
"""ごみ箱を削除"""
import os
import shutil

user_recycle_bin = r'C:\$Recycle.Bin\S-1-5-21-1744958584-1862580099-149994256-1001'

if os.path.exists(user_recycle_bin):
    print(f"削除中: {user_recycle_bin}")
    try:
        # フォルダ内の全ファイルを削除
        for root, dirs, files in os.walk(user_recycle_bin):
            for file in files:
                try:
                    file_path = os.path.join(root, file)
                    os.chmod(file_path, 0o777)  # 権限を変更
                    os.remove(file_path)
                except (OSError, PermissionError) as e:
                    print(f"  ファイル削除エラー: {file_path} - {e}")
            
            for dir in dirs:
                try:
                    dir_path = os.path.join(root, dir)
                    os.chmod(dir_path, 0o777)
                    os.rmdir(dir_path)
                except (OSError, PermissionError) as e:
                    print(f"  ディレクトリ削除エラー: {dir_path} - {e}")
        
        # フォルダ自体を削除
        try:
            os.rmdir(user_recycle_bin)
            print("削除完了")
        except (OSError, PermissionError) as e:
            print(f"フォルダ削除エラー: {e}")
            print("一部のファイルが削除できませんでした（権限の問題の可能性があります）")
    except Exception as e:
        print(f"エラー: {e}")
else:
    print("ごみ箱フォルダが見つかりません")
