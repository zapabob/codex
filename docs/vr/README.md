# VR/AR Support

**Status**: Experimental | **Version**: 2.8.3

Meta Questおよびその他のVR/ARデバイスでの没入型開発環境。

## 🎯 概要

VR/ARサポートにより、3D空間でのコード可視化と直感的な開発体験を提供します。

## 🎮 対応デバイス

### サポート対象

- **Meta Quest 2/3/3S/Pro**
- **Meta Quest Link (PC VR)**
- **Oculus Rift S**
- **HTC Viveシリーズ**
- **Valve Index**

### システム要件

- **VRヘッドセット**: Meta Quest 2以上推奨
- **PC要件**: RTX 3060以上、16GB RAM以上
- **ソフトウェア**: Oculus/Metaアプリ、SteamVR (PC VRの場合)

## 🚀 起動方法

### Meta Questネイティブ

```bash
# Questアプリから起動
# 1. Meta Questブラウザで http://codex-vr.local にアクセス
# 2. アプリをサイドロードインストール
# 3. コントローラーで操作開始
```

### PC VR (Link)

```bash
# PC接続モード
codex vr serve --platform=quest-link --port=8080

# SteamVR統合
codex vr steamvr --auto-launch
```

### WebXRブラウザ

```bash
# ブラウザベースVR
codex vr webxr --port=3000

# Chrome/Edgeで https://localhost:3000 にアクセス
# VRヘッドセットを接続
```

## 🎨 主要機能

### 3Dコード可視化

- **コードブロック**: 物理的な3Dオブジェクトとして表示
- **依存関係グラフ**: 空間的な接続表示
- **変更履歴**: 時間軸でのコード進化の可視化
- **エラー箇所**: 赤いハイライトで問題個所を強調

### 直感的操作

- **ハンドトラッキング**: 自然なジェスチャーでコード操作
- **音声コマンド**: 音声によるPlan実行
- **アイトラッキング**: 視線によるフォーカス制御
- **モーションコントローラー**: 精密な3D操作

### コラボレーション

- **共有スペース**: 複数ユーザーの同時作業
- **アバター表示**: チームメンバーの位置表示
- **リアルタイム同期**: 変更の即時反映
- **音声会議**: 空間オーディオによるコミュニケーション

## 🛠️ 技術仕様

### エンジン

- **Rendering**: WebGL + Three.js
- **Physics**: Cannon.js (3D物理演算)
- **Audio**: Web Audio API + Spatial Audio
- **Networking**: WebRTC + WebSocket

### パフォーマンス

- **Frame Rate**: 72-90 FPS (Quest 2)
- **Latency**: <50ms (入力から表示まで)
- **Memory**: <200MB (VRアプリ)
- **Battery Impact**: +15%消費 (通常使用時)

## 🔧 設定

### VR設定

```json
{
  "codex.vr.enabled": true,
  "codex.vr.platform": "quest-native",
  "codex.vr.quality": "high",
  "codex.vr.handTracking": true,
  "codex.vr.eyeTracking": false,
  "codex.vr.spatialAudio": true
}
```

### デバイス設定

```json
{
  "codex.vr.devices.quest": {
    "resolution": "1832x1920",
    "refreshRate": 90,
    "guardian": true
  },
  "codex.vr.devices.controllers": {
    "haptics": true,
    "batteryAlerts": true
  }
}
```

## 🎮 使用例

### コードレビュー in VR

```bash
# VR空間でコードレビュー
codex vr review --file=src/auth.js --mode=immersive

# 3Dでの変更点ハイライト
codex vr diff --commit=abc123 --spatial
```

### Plan実行監視

```bash
# VRダッシュボード
codex vr dashboard

# 3Dでの実行フロー可視化
codex vr plan-monitor --plan-id=bp-123
```

### ペアプログラミング

```bash
# 共有VRセッション
codex vr collaborate --session=team-alpha

# コードの同時編集
codex vr edit --shared --file=src/main.js
```

## 📊 開発状況

### ✅ 実装済み機能

- [x] 基本3Dコード可視化
- [x] Meta Quest対応
- [x] WebXRサポート
- [x] ハンドトラッキング
- [x] 音声コマンド

### 🔄 開発中機能

- [ ] 高度なコラボレーション
- [ ] AIアシスタントのアバター化
- [ ] ジェスチャー認識の拡張
- [ ] マルチデバイス同期

### 📋 計画機能

- [ ] ARグラス対応
- [ ] ホログラム投影
- [ ] 脳波インターフェース
- [ ] 拡張現実デバッグ

## 🎯 ユースケース

### 没入型コード理解

```bash
# 大規模コードベースの3Dナビゲーション
codex vr explore --repo=. --scale=building

# アーキテクチャの空間把握
codex vr architecture --format=3d-model
```

### 創造的プログラミング

```bash
# 3Dでのアルゴリズム構築
codex vr algorithm --interactive

# ビジュアルプログラミング
codex vr visual-program --blocks
```

### 教育・トレーニング

```bash
# プログラミング学習
codex vr tutorial --language=javascript

# コードレビュートレーニング
codex vr training --scenario=code-review
```

## 🎮 詳細ガイド

- [VR開発](./development.md) - カスタムVR機能開発
- [デバイス設定](./devices.md) - 各種VRデバイスの設定
- [パフォーマンス最適化](./performance.md) - VR特化の最適化

## 📚 関連リンク

- [GUI Support](../gui/README.md) - 2Dインターフェース
- [Benchmarks](../benchmarks/README.md) - パフォーマンス測定
- [Security](../SECURITY.md) - VRセキュリティ

---

**3D空間での革新的な開発体験を提供します** 🥽