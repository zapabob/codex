#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import winsound
import os

audio_path = r'C:\Users\downl\Desktop\SO8T\.cursor\marisa_owattaze.wav'
if os.path.exists(audio_path):
    winsound.PlaySound(audio_path, winsound.SND_FILENAME | winsound.SND_SYNC)
    print('✅ 音声を再生しました: 終わったぜ！')
else:
    print(f'⚠️ 音声ファイルが見つかりません: {audio_path}')
