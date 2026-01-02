# GUI/Web Interface

**Status**: Experimental | **Version**: 2.8.3

ダッシュボードとエージェント管理のためのWebベースGUI。

## 🎯 概要

CodexのGUIはブラウザベースのインターフェースを提供し、直感的な操作とリアルタイム監視を実現します。

## 🚀 起動方法

### 開発サーバー

```bash
# GUIサーバー起動
codex gui serve --port=3000

# ブラウザでアクセス
# http://localhost:3000
```

### Dockerコンテナ

```bash
# Dockerイメージ使用
docker run -p 3000:3000 zapabob/codex-gui:latest

# ボリュームマウント
docker run -v $(pwd):/workspace -p 3000:3000 zapabob/codex-gui:latest
```

## 🎨 主要機能

### ダッシュボード

- **Plan Status**: 実行中の計画のリアルタイム監視
- **Agent Activity**: サブエージェントの稼働状況
- **Performance Metrics**: CPU/メモリ使用率、実行時間
- **Log Stream**: 構造化ログのライブ表示

### エージェント管理

- **Agent Pool**: 利用可能なエージェントの一覧
- **Task Assignment**: 手動タスク割り当て
- **Performance Tuning**: エージェント設定の調整
- **Health Monitoring**: エージェントの健全性チェック

### 可視化

- **Execution Timeline**: タスク実行の時系列グラフ
- **Resource Usage**: リソース消費のチャート
- **Error Patterns**: エラーの傾向分析
- **Success Rates**: タスク完了率の統計

## 🛠️ 技術仕様

### フロントエンド

- **Framework**: React 18 + TypeScript
- **Styling**: Tailwind CSS + shadcn/ui
- **State Management**: Zustand
- **Charts**: Recharts + D3.js

### バックエンドAPI

- **Protocol**: WebSocket + REST
- **Authentication**: JWT tokens
- **Real-time Updates**: Server-Sent Events
- **CORS**: Configurable origins

### パフォーマンス

- **Initial Load**: <2秒
- **Real-time Latency**: <100ms
- **Memory Usage**: <50MB
- **Bundle Size**: <500KB (gzip)

## 🔧 設定

### GUI設定

```json
{
  "codex.gui.enabled": true,
  "codex.gui.port": 3000,
  "codex.gui.host": "localhost",
  "codex.gui.auth.enabled": false,
  "codex.gui.theme": "auto",
  "codex.gui.refreshInterval": 5000
}
```

### セキュリティ設定

```json
{
  "codex.gui.security.cors.enabled": true,
  "codex.gui.security.cors.origins": ["localhost:3000"],
  "codex.gui.security.auth.jwt.secret": "your-secret",
  "codex.gui.security.rateLimit.requests": 100,
  "codex.gui.security.rateLimit.window": 60000
}
```

## 🎮 使用例

### Plan実行監視

```bash
# GUIでPlan作成
# 1. ブラウザで http://localhost:3000 にアクセス
# 2. "Create Plan" ボタンをクリック
# 3. タスク記述を入力
# 4. 実行モードを選択
# 5. "Execute" ボタンをクリック

# CLIでのPlan作成もGUIで監視可能
codex /Plan "Implement user authentication"
```

### エージェント管理

```bash
# エージェントプールの監視
codex gui agents

# 特定エージェントの詳細
codex gui agent backend-agent --details
```

## 📊 開発状況

### ✅ 実装済み機能

- [x] 基本ダッシュボード
- [x] Plan実行監視
- [x] エージェントステータス
- [x] ログストリーミング
- [x] レスポンシブデザイン

### 🔄 開発中機能

- [ ] 高度な可視化チャート
- [ ] コラボレーションツール
- [ ] プラグインシステム
- [ ] モバイル対応

### 📋 計画機能

- [ ] AIチャットインターフェース
- [ ] コードエディタ統合
- [ ] チーム管理機能
- [ ] カスタムダッシュボード

## 🎯 ユースケース

### 開発監視

```bash
# チーム開発の監視
codex gui serve --team-mode

# パフォーマンス監視
codex gui monitor --metrics=cpu,memory,network
```

### デバッグ支援

```bash
# エラーログの可視化
codex gui logs --filter=error --real-time

# Plan実行のデバッグ
codex gui debug plan-123
```

## 🎮 詳細ガイド

- [GUI開発](./development.md) - カスタムGUI開発
- [APIリファレンス](./api.md) - REST/WebSocket API
- [テーマ設定](./themes.md) - カスタマイズ

## 📚 関連リンク

- [Plan Mode](../plan/README.md) - GUIで管理するワークフロー
- [Security](../SECURITY.md) - GUIセキュリティ
- [Benchmarks](../benchmarks/README.md) - パフォーマンス測定

---

**直感的なWebインターフェースでCodexを操作できます** 🖥️