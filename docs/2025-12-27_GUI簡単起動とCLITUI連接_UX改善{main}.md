# GUI簡単起動とCLI/TUI連接・UX改善

**日時**: 2025-12-27 19:30:58
**ワークツリー**: main
**タスク**: GUIを簡単に呼び出せるようにし、CLI/TUIと連接して使えるようにし、サイドバーやホバーなどのUXを改善

---

## 🎯 実装内容

### 1. CLIにGUI起動コマンドを追加

**ファイル**: `codex-rs/cli/src/main.rs`

**追加内容**:
- `GuiCommand`構造体を追加
- `codex gui`コマンドを実装
- オプション:
  - `--port <PORT>`: フロントエンドポート（デフォルト: 3000）
  - `--backend-port <PORT>`: バックエンドAPIポート（デフォルト: 8787）
  - `--no-browser`: ブラウザを自動で開かない

**使用方法**:
```bash
# 基本的な起動
codex gui

# カスタムポートで起動
codex gui --port 3001 --backend-port 8788

# ブラウザを開かない
codex gui --no-browser
```

**実装詳細**:
- バックエンドAPIサーバーを自動起動
- フロントエンド開発サーバーを自動起動
- `node_modules`が存在しない場合は自動で`npm install`を実行
- ブラウザを自動で開く（`--no-browser`で無効化可能）
- 複数のGUIディレクトリ検索パスを試行

### 2. サイドバーのUX改善

**ファイル**: `gui/src/components/organisms/Sidebar.tsx`

**改善内容**:

#### ホバー効果の強化
- アイテムホバー時に右に4px移動するアニメーション
- アイコンがホバー時に1.1倍にスケール
- アクティブアイテムの左側にインジケーターバーを表示
- スムーズなトランジション（cubic-bezier）

#### ツールチップの追加
- 各ナビゲーションアイテムにツールチップを追加
- ツールチップに表示:
  - アイテム名
  - 説明文
  - キーボードショートカット（チップ形式）
- 300msの遅延で表示（誤操作防止）

#### キーボードショートカットの表示
- 各アイテムにショートカットキーを表示
- デスクトップ表示のみ（モバイルでは非表示）
- チップ形式で表示

#### ナビゲーションアイテムの拡張
- `shortcut`: キーボードショートカット
- `description`: 説明文

### 3. キーボードショートカットの実装

**ファイル**: `gui/src/components/templates/DashboardLayout.tsx`

**実装内容**:
- Next.jsの`useRouter`を使用してナビゲーション
- キーボードショートカット:
  - `Ctrl+D`: ダッシュボード
  - `Ctrl+C`: コード実行
  - `Ctrl+A`: エージェント
  - `Ctrl+T`: タスク管理
  - `Ctrl+Q`: QC管理
  - `Ctrl+S`: セキュリティ
  - `Ctrl+V`: 仮想OS
  - `Ctrl+I`: AIツール統合
  - `Ctrl+R`: Deep Research
  - `Ctrl+M`: MCPサーバー
  - `Ctrl+,`: 設定
  - `Ctrl+B`: サイドバーの表示/非表示

**特徴**:
- 入力フィールドフォーカス時は無効化（誤操作防止）
- `useKeyboardShortcuts`フックを使用
- スムーズなナビゲーション

### 4. CLI/TUIとの連接強化

**ファイル**: `gui/src/lib/bridge/dual-bridge.ts`, `gui/src/lib/context/CodexContext.tsx`

**改善内容**:

#### DualBridgeの拡張
- `getSystemMetrics()`メソッドを追加
- `codex system metrics --json`コマンドを実行
- タイムアウトを30秒に延長（システムコマンド用）

#### CodexContextの改善
- `loadMetrics()`でCLI/TUI経由の取得を優先
- フォールバック: CLI失敗時はAPI経由で取得
- リアルタイム更新に対応

### 5. 簡単起動スクリプトの追加

**ファイル**: `scripts/launch-gui-simple.ps1`

**内容**:
- `codex gui`コマンドを実行するシンプルなラッパー
- エラーハンドリング
- 使用方法の表示

---

## 📊 UX改善の詳細

### サイドバー
- **ホバー効果**: スムーズなアニメーション、アイコンスケール
- **ツールチップ**: 詳細情報とショートカット表示
- **視覚的フィードバック**: アクティブ状態の明確な表示
- **キーボードショートカット**: 各アイテムに表示

### キーボードナビゲーション
- **ショートカットキー**: 全主要ページに割り当て
- **入力フィールド保護**: 入力中はショートカット無効化
- **スムーズな遷移**: Next.jsルーターを使用

### CLI/TUI連接
- **優先順位**: CLI/TUI → API → フォールバック
- **リアルタイム更新**: WebSocket + CLIコマンド
- **エラーハンドリング**: 複数の取得方法で確実に情報取得

---

## 🔌 接続方法

### 1. CLIコマンド経由（推奨）
```bash
# 基本的な起動
codex gui

# カスタム設定
codex gui --port 3001 --backend-port 8788
```

### 2. PowerShellスクリプト経由
```powershell
.\scripts\launch-gui-simple.ps1
```

### 3. 手動起動
```bash
# バックエンド
cd codex-rs
cargo run -p codex-gui

# フロントエンド（別ターミナル）
cd gui
npm run dev
```

---

## ✅ 完了したタスク

1. ✅ CLIにGUI起動コマンドを追加（`codex gui`）
2. ✅ サイドバーのホバー効果とツールチップを改善
3. ✅ キーボードショートカットのサポートを追加
4. ✅ CLI/TUIとの連接を強化（DualBridge改善）
5. ✅ 簡単起動スクリプトの追加

---

## 🎉 改善結果

### GUI起動の簡素化
- **ワンコマンド起動**: `codex gui`でGUIを起動
- **自動セットアップ**: `npm install`を自動実行
- **ブラウザ自動起動**: デフォルトでブラウザを開く

### UX改善
- **ホバー効果**: 視覚的フィードバックの強化
- **ツールチップ**: 詳細情報の表示
- **キーボードショートカット**: 効率的なナビゲーション
- **スムーズなアニメーション**: プロフェッショナルな見た目

### CLI/TUI連接
- **優先順位付き取得**: CLI/TUI → API → フォールバック
- **リアルタイム更新**: WebSocket + CLIコマンド
- **エラーハンドリング**: 複数の取得方法で確実に情報取得

---

## 📝 使用方法

### GUI起動
```bash
# 最も簡単な方法
codex gui

# カスタムポート
codex gui --port 3001

# ブラウザを開かない
codex gui --no-browser
```

### キーボードショートカット
- `Ctrl+D`: ダッシュボード
- `Ctrl+C`: コード実行
- `Ctrl+A`: エージェント
- `Ctrl+T`: タスク管理
- `Ctrl+B`: サイドバー表示/非表示
- その他: サイドバーのツールチップを参照

### CLI/TUIとの連携
- GUIは自動的にCLI/TUIと接続を試みます
- システム情報はCLI/TUI経由で優先的に取得
- DualBridge経由でコマンドを実行可能

---

## 🔧 技術スタック

- **CLI**: Rust (clap)
- **フロントエンド**: React, TypeScript, Next.js
- **UIライブラリ**: Material-UI, Framer Motion
- **キーボードショートカット**: カスタムフック
- **CLI/TUI連接**: DualBridge, WebSocket RPC

---

完了！
