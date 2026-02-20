---
name: yukkuri-movie
description: "ゆっくりMovieMaker4 (YMM4) 動画制作自動化: キャラクターアニメーション・音声合成・字幕・エフェクト管理。"
---

# ゆっくりMovieMaker Agent Skill

## Overview

ゆっくりMovieMaker4 (YMM4) を使ったVTuber動画・ゆっくり解説動画制作の自動化スキル。
YMM4のJSONプロジェクト形式・YMM4 API・AviUtl連携に対応。

## Capabilities

### キャラクター管理
- **ゆっくり霊夢/魔理沙**: デフォルトキャラクター設定
- **カスタムキャラクター**: 立ち絵・差分・衣装管理
- **表情制御**: 通常/笑い/怒り/悲しみ等の感情切り替え
- **立ち位置**: 左/中央/右 + 前後レイヤー

### 音声合成
- **VOICEVOX**: 四国めたん/ずんだもん/春日部つむぎ等 (無料)
- **AivisSpeech**: AIベース高品質音声
- **CoeFont**: クラウド音声合成
- **棒読みちゃん**: 汎用TTS
- **感情パラメータ**: 速度/音量/ピッチ/抑揚の細かい調整

### タイムライン管理
- **字幕**: 自動字幕生成・タイミング調整
- **BGM**: 音楽トラック・フェードイン/アウト
- **効果音 (SE)**: タイミング同期
- **テロップ**: テキストエフェクト・アニメーション

### エフェクト & 映像
- **背景**: 静止画/動画/スクリーンショット
- **トランジション**: フェード・ワイプ・フラッシュ
- **フィルター**: ぼかし・色調補正・ビネット
- **字幕デザイン**: フォント・色・縁取り・影

### 出力
- **MP4 (H.264/H.265)**: YouTube向け
- **AVI**: 高品質中間ファイル
- **GIF**: SNS用短尺クリップ
- **AviUtl連携**: 高度なエフェクト処理

## YMM4 プロジェクト構造

```json
{
  "TimeLine": {
    "VideoInfo": {
      "FPS": 60.0,
      "Width": 1920,
      "Height": 1080
    },
    "Items": [
      {
        "$type": "YukkuriMovieMaker.Project.Items.SpeechItem, YukkuriMovieMaker",
        "CharacterName": "ゆっくり霊夢",
        "VoiceParameter": {
          "Speed": 1.0,
          "Volume": 1.0,
          "Pitch": 0.0,
          "Emphasis": 1.0
        },
        "Text": "こんにちは、霊夢です！",
        "Frame": 0,
        "Length": 120,
        "Layer": 1,
        "IsHide": false,
        "FaceParameter": {
          "Expression": "Normal"
        }
      }
    ]
  }
}
```

## Python 自動化スクリプト

### プロジェクト生成

```python
import json
import os
from dataclasses import dataclass, field
from typing import List, Optional

@dataclass
class YMM4Item:
    """YMM4タイムラインアイテム"""
    character: str
    text: str
    frame: int
    length: int = 120
    layer: int = 1
    expression: str = "Normal"
    speed: float = 1.0
    volume: float = 1.0

def create_ymm4_project(
    items: List[YMM4Item],
    fps: float = 60.0,
    width: int = 1920,
    height: int = 1080,
    output_path: str = "project.ymmp"
) -> dict:
    """YMM4プロジェクトを生成"""
    timeline_items = []
    
    for item in items:
        timeline_items.append({
            "$type": "YukkuriMovieMaker.Project.Items.SpeechItem, YukkuriMovieMaker",
            "CharacterName": item.character,
            "VoiceParameter": {
                "Speed": item.speed,
                "Volume": item.volume,
                "Pitch": 0.0,
                "Emphasis": 1.0
            },
            "Text": item.text,
            "Frame": item.frame,
            "Length": item.length,
            "Layer": item.layer,
            "IsHide": False,
            "FaceParameter": {
                "Expression": item.expression
            }
        })
    
    project = {
        "Version": "4.0.0",
        "TimeLine": {
            "VideoInfo": {
                "FPS": fps,
                "Width": width,
                "Height": height
            },
            "Items": timeline_items
        }
    }
    
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(project, f, ensure_ascii=False, indent=2)
    
    return project
```

### VOICEVOX 音声合成連携

```python
import requests
import subprocess
from pathlib import Path

VOICEVOX_API = "http://localhost:50021"

def synthesize_voicevox(
    text: str,
    speaker_id: int = 3,  # ずんだもん
    speed: float = 1.0,
    output_wav: str = "voice.wav"
) -> bool:
    """VOICEVOX で音声合成"""
    # Audio Query 生成
    query_response = requests.post(
        f"{VOICEVOX_API}/audio_query",
        params={"text": text, "speaker": speaker_id}
    )
    
    if query_response.status_code != 200:
        print(f"Audio Query 失敗: {query_response.status_code}")
        return False
    
    audio_query = query_response.json()
    audio_query["speedScale"] = speed
    
    # 音声合成
    synthesis_response = requests.post(
        f"{VOICEVOX_API}/synthesis",
        params={"speaker": speaker_id},
        json=audio_query
    )
    
    if synthesis_response.status_code == 200:
        with open(output_wav, 'wb') as f:
            f.write(synthesis_response.content)
        return True
    
    return False

# VOICEVOX スピーカーID
SPEAKERS = {
    "四国めたん": 2,
    "ずんだもん": 3,
    "春日部つむぎ": 8,
    "雨晴はう": 10,
    "波音リツ": 9,
    "玄野武宏": 11,
    "白上虎太郎": 12,
    "青山龍星": 13,
}
```

### スクリプト → YMM4 変換

```python
def script_to_ymm4(
    script_lines: List[tuple],  # [(character, text, emotion), ...]
    fps: float = 60.0,
    seconds_per_line: float = 3.0
) -> List[YMM4Item]:
    """台本からYMM4アイテムリストを生成"""
    items = []
    frame = 0
    length = int(fps * seconds_per_line)
    
    for i, (character, text, emotion) in enumerate(script_lines):
        items.append(YMM4Item(
            character=character,
            text=text,
            frame=frame,
            length=length,
            layer=1 if "霊夢" in character else 2,
            expression=emotion
        ))
        frame += length
    
    return items

# 使用例
script = [
    ("ゆっくり霊夢", "みなさん、こんにちは！", "Normal"),
    ("ゆっくり魔理沙", "今日はPythonについて解説するぜ！", "Happy"),
    ("ゆっくり霊夢", "早速始めましょう。", "Normal"),
]
```

## Usage

```bash
# 台本からプロジェクト生成
codex $yukkuri-movie "以下の台本でゆっくり解説動画プロジェクトを生成して: ..."

# VOICEVOXで音声生成
codex $yukkuri-movie "VOICEVOXでずんだもんの音声を合成してYMM4に組み込んで"

# テンプレート作成
codex $yukkuri-movie "プログラミング解説動画のYMM4テンプレートを作成して"

# 字幕自動生成
codex $yukkuri-movie "MP4から字幕を抽出してYMM4プロジェクトに変換して"
```

## Workflow

1. **台本作成**: キャラクター・セリフ・感情を定義
2. **音声合成**: VOICEVOX/AivisSpeech でWAV生成
3. **プロジェクト生成**: YMM4 JSON形式でプロジェクト作成
4. **素材配置**: 背景・BGM・SEをタイムラインに配置
5. **エフェクト追加**: トランジション・字幕デザイン設定
6. **レンダリング**: YMM4でMP4出力
7. **後処理**: AviUtlで追加エフェクト (オプション)

## References

- [ゆっくりMovieMaker4 公式](https://manjubox.net/ymm4/)
- [VOICEVOX](https://voicevox.hiroshiba.jp/)
- [AivisSpeech](https://aivis-project.com/)
- [YMM4 Plugin 開発](https://github.com/manju-summoner/YukkuriMovieMaker)

---

**Version**: 2.0.0  
**Target**: YMM4 (ゆっくりMovieMaker4) + VOICEVOX  
**Compatibility**: Windows 11 / Cursor IDE + Codex
