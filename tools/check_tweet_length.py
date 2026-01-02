#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
X投稿用テキストの文字数チェック
"""

candidates = [
    "🚀Codex v2.8.3：世界初のAI-Native OS。CUDA加速でGit解析100倍高速、4D可視化、VR/AR対応、マルチエージェント並列で2.6倍高速化。完全オープンソース。https://github.com/zapabob/codex",
    "世界初のAI-Native OS「Codex v2.8.3」🚀 CUDA加速でGit解析100倍高速、4D可視化、VR/AR対応、マルチエージェント並列で2.6倍高速。完全オープンソース・無料。https://github.com/zapabob/codex",
    "【Codex v2.8.3】世界初のAI-Native OS🚀 CUDA加速でGit解析100倍高速、4D可視化、VR/AR対応、マルチエージェント並列で2.6倍高速化。Rust製・完全オープンソース。https://github.com/zapabob/codex",
    "世界初のAI-Native OS「Codex v2.8.3」🚀 CUDA加速でGit解析100倍高速、4D可視化（Kamui4D超え）、VR/AR対応、マルチエージェント並列で2.6倍高速。完全オープンソース。https://github.com/zapabob/codex",
]

# 139文字に調整
target = 139

for i, text in enumerate(candidates, 1):
    length = len(text)
    diff = length - target
    print(f"候補{i}: {length}文字 (差: {diff:+d})")
    if diff == 0:
        print(f"  ✅ 完璧！")
    elif diff > 0:
        print(f"  ⚠️ {diff}文字多い")
    else:
        print(f"  ⚠️ {abs(diff)}文字少ない")
    print(f"  {text}\n")

# 139文字に調整したバージョン
final = "🚀Codex v2.8.3：世界初のAI-Native OS。CUDA加速でGit解析100倍高速、4D可視化、VR/AR対応、マルチエージェント並列で2.6倍高速化。完全オープンソース。https://github.com/zapabob/codex"
print(f"\n最終推奨: {len(final)}文字")
print(final)
