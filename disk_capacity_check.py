# -*- coding: utf-8 -*-
"""Cドライブ容量チェック"""
import shutil
import os

def get_disk_usage():
    """ディスク使用状況を取得"""
    total, used, free = shutil.disk_usage('C:/')

    total_gb = total / (1024 ** 3)
    used_gb = used / (1024 ** 3)
    free_gb = free / (1024 ** 3)
    used_percent = (used / total) * 100

    return {
        'total_gb': round(total_gb, 2),
        'used_gb': round(used_gb, 2),
        'free_gb': round(free_gb, 2),
        'used_percent': round(used_percent, 1)
    }

def main():
    try:
        usage = get_disk_usage()
        print("Cドライブ容量状況:")
        print(f"  総容量: {usage['total_gb']} GB")
        print(f"  使用量: {usage['used_gb']} GB")
        print(f"  空き容量: {usage['free_gb']} GB")
        print(f"  使用率: {usage['used_percent']}%")

        if usage['used_percent'] > 90:
            print("\n⚠️  WARNING: 容量が非常に少ない状態です！")
        elif usage['used_percent'] > 80:
            print("\n⚠️  注意: 容量が不足しつつあります")

    except Exception as e:
        print(f"エラー: {e}")

if __name__ == '__main__':
    main()
