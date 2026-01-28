# Codex GUI - Unified Web Interface

**Status**: Production Ready | **Version**: 2.7.0

統合されたWebベースGUI。Plan管理、認証、3D/4D可視化、VR/AR機能を統合。

## 🎯 概要

CodexのGUIはブラウザベースのインターフェースを提供し、直感的な操作とリアルタイム監視を実現します。

## 🚀 起動方法

### 開発サーバー

```bash
# 1. Rustバックエンドを起動
cd codex-rs
cargo run -p codex-gui

# 2. GUIフロントエンドを起動（別ターミナル）
cd gui
npm install
npm run dev

# ブラウザでアクセス
# http://localhost:3000
```

### 環境変数設定

```bash
# gui/.env.local を作成
cp gui/.env.example gui/.env.local

# 必要な環境変数を設定
CODEX_GUI_PORT=8787
CODEX_GUI_CLI_PATH=codex
CODEX_GUI_DB_URL=sqlite:codex-gui.db
CODEX_GUI_JWT_SECRET=your-secret-key-change-in-production
NEXT_PUBLIC_API_URL=http://localhost:8787
```

## 🎨 主要機能

### ダッシュボード
- **Plan Status**: 実行中の計画のリアルタイム監視
- **Agent Activity**: サブエージェントの稼働状況
- **Performance Metrics**: CPU/メモリ使用率、実行時間
- **Log Stream**: 構造化ログのライブ表示

### Plan管理
- **Plan作成・編集**: 計画の作成と管理
- **承認ワークフロー**: Planの承認/却下
- **実行管理**: Planの実行と監視
- **エクスポート**: Markdown/JSON形式でのエクスポート

### 認証
- **JWT認証**: Rustバックエンドによる認証
- **セッション管理**: 自動セッション管理
- **ユーザー管理**: ユーザー登録・ログイン

### 可視化
- **3D/4D Git可視化**: 時間軸を含む4次元可視化
- **VR/AR対応**: WebXRによる没入型可視化
- **ハンドトラッキング**: ジェスチャー操作
- **空間オーディオ**: 3D位置音響

### エージェント管理
- **Agent Pool**: 利用可能なエージェントの一覧
- **Task Assignment**: 手動タスク割り当て
- **Performance Tuning**: エージェント設定の調整
- **Health Monitoring**: エージェントの健全性チェック

## 🛠️ 技術仕様

### フロントエンド
- **Framework**: React 19.2.4 + Next.js 14
- **Styling**: Tailwind CSS + shadcn/ui
- **State Management**: Zustand
- **3D/VR**: Three.js + React Three Fiber + @react-three/xr
- **Charts**: Recharts + Chart.js

### バックエンドAPI
- **Protocol**: HTTP REST API
- **Authentication**: JWT tokens
- **Database**: SQLite
- **Real-time Updates**: Server-Sent Events (SSE)
- **CORS**: Configurable origins

### パフォーマンス
- **Initial Load**: <2秒
- **Real-time Latency**: <100ms
- **Memory Usage**: <50MB

## 📁 プロジェクト構造

```
gui/
├── src/
│   ├── app/                    # Next.js App Router
│   │   ├── (auth)/            # 認証ページ
│   │   ├── plans/             # Plan管理
│   │   ├── visualization/     # 3D/4D可視化
│   │   └── vr/                # VR/AR機能
│   ├── components/
│   │   ├── visualization/     # 可視化コンポーネント
│   │   └── vr/                # VR/ARコンポーネント
│   └── lib/
│       ├── api/
│       │   └── client.ts      # 統一APIクライアント
│       ├── context/
│       │   └── AuthContext.tsx # 認証コンテキスト
│       ├── visualization/     # 可視化ライブラリ
│       └── xr/                # WebXRライブラリ
```

## 🔧 開発

### 依存関係のインストール

```bash
npm install
```

### ビルド

```bash
npm run build
```

### テスト

```bash
# E2Eテスト
npm run test

# UIモード
npm run test:ui
```

## 🔐 セキュリティ

- JWT認証によるセキュアな認証
- パスワードはbcryptでハッシュ化
- SQLiteデータベースによるデータ管理
- CORS設定によるアクセス制御

## 📚 APIドキュメント

### 認証API

- `POST /api/auth/login` - ログイン
- `POST /api/auth/register` - ユーザー登録
- `POST /api/auth/logout` - ログアウト
- `GET /api/auth/session` - セッション確認

### Plan管理API

- `GET /api/plans` - Plan一覧取得
- `POST /api/plans` - Plan作成
- `GET /api/plans/{id}` - Plan詳細取得
- `POST /api/plans/{id}/approve` - Plan承認
- `POST /api/plans/{id}/reject` - Plan却下
- `POST /api/plans/{id}/execute` - Plan実行
- `GET /api/plans/{id}/export` - Planエクスポート

### VR/AR API

- `GET /api/vr/status` - VR/ARステータス取得
- `POST /api/vr/session` - VR/ARセッション作成

## 🐛 トラブルシューティング

### バックエンドに接続できない

1. Rustバックエンドが起動しているか確認
2. `CODEX_GUI_PORT`環境変数が正しいか確認
3. `NEXT_PUBLIC_API_URL`が正しく設定されているか確認

### 認証エラー

1. JWTシークレットが設定されているか確認
2. SQLiteデータベースが作成されているか確認
3. ユーザーが登録されているか確認

## 📝 ライセンス

Apache 2.0
