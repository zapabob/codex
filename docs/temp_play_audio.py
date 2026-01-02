#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import winsound
import os
import sys

# UTF-8出力設定
if sys.platform == 'win32':
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')

audio_path = r'C:\Users\downl\Desktop\SO8T\.cursor\marisa_owattaze.wav'
if os.path.exists(audio_path):
    winsound.PlaySound(audio_path, winsound.SND_FILENAME)
    print('✅ 音声を再生しました: 終わったぜ！')
else:
    print(f'⚠️ 音声ファイルが見つかりません: {audio_path}')
