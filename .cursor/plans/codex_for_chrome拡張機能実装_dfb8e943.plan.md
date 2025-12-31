---
name: Codex for Chrome拡張機能実装
overview: Claude for Chromeと同様に、Codex CLIとChrome/Edge拡張機能をNative Messaging APIで接続し、ブラウザ内でDeep Researchやコード生成、ブラウザ操作（DOM読み取り、コンソールログ、ネットワークリクエスト）を可能にする。
todos:
  - id: "1"
    content: "Native Messaging Host実装: codex-rs/chrome-hostプロジェクト作成、stdin/stdout JSONメッセージ処理、Codex CLI呼び出し機能"
    status: in_progress
  - id: "2"
    content: "Chrome拡張機能基本構造: manifest.json作成、バックグラウンドスクリプト実装、Native Messaging API接続"
    status: pending
  - id: "3"
    content: "メッセージプロトコル定義: リクエスト/レスポンス形式定義、エラーハンドリング仕様"
    status: pending
  - id: "4"
    content: "Deep Research UI実装: ポップアップUI作成、検索クエリ入力、結果表示"
    status: pending
  - id: "5"
    content: "Deep Research統合: 拡張機能からDeep Research呼び出し、結果の表示・保存"
    status: pending
  - id: "6"
    content: "DOM読み取り機能: コンテンツスクリプト実装、DOM要素取得・操作、ページ情報取得"
    status: pending
  - id: "7"
    content: "コンソールログ取得: コンソールAPI監視、ログ収集・送信"
    status: pending
  - id: "8"
    content: "ネットワークリクエスト監視: chrome.webRequest API使用、リクエスト/レスポンス情報取得"
    status: pending
  - id: "9"
    content: "Codex CLI /chromeコマンド実装: DOM読み取り、コンソールログ取得、ネットワーク監視コマンド"
    status: pending
  - id: "10"
    content: "インストールスクリプト作成: Windows/Linux/macOS用Native Messaging Host登録スクリプト"
    status: pending
  - id: "11"
    content: "統合テスト: エンドツーエンドテスト、クロスプラットフォームテスト"
    status: pending
---

# Codex for Chrome拡張機能実装計画

## 背景

Claude CodeがClaude for Chrome拡張機能と連携し、ターミナルからブラウザ操作が可能になった。同様に、Codex CLIとChrome/Edge拡張機能を接続し、ブラウザ内でDeep Researchやコード生成を可能にする。

## 実装範囲

- **Chrome/Edge拡張機能**: ブラウザ拡張機能UIとバックグラウンドスクリプト
- **Native Messaging Host**: Codex CLIとブラウザ拡張機能を接続するネイティブメッセージングホスト
- **Deep Research統合**: ブラウザ拡張機能からDeep Research機能を呼び出し
- **ブラウザ操作**: DOM読み取り、コンソールログ取得、ネットワークリクエスト監視
- **コード生成・実行**: ブラウザ内でコード生成とテスト

## アーキテクチャ

```
┌─────────────────────────────────────────────────────────┐
│                    Chrome/Edge Browser                   │
│  ┌──────────────────────────────────────────────────┐   │
│  │         Codex Extension (Content Script)         │   │
│  │  - DOM読み取り                                    │   │
│  │  - コンソールログ監視                             │   │
│  │  - ネットワークリクエスト監視                      │   │
│  └──────────────────┬───────────────────────────────┘   │
│                     │                                     │
│  ┌──────────────────▼───────────────────────────────┐   │
│  │      Background Script (Extension)                │   │
│  │  - Native Messaging API接続                       │   │
│  │  - メッセージルーティング                          │   │
│  └──────────────────┬───────────────────────────────┘   │
└──────────────────────┼───────────────────────────────────┘
                       │ Native Messaging API
                       │ (stdin/stdout JSON)
                       ▼
┌─────────────────────────────────────────────────────────┐
│              Native Messaging Host                      │
│  (codex-chrome-host.exe / codex-chrome-host)            │
│  - JSONメッセージのパース                                │   │
│  - Codex CLIへのコマンド変換                             │   │
│  - レスポンスのJSON化                                    │   │
└──────────────────┬──────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────┐
│                  Codex CLI                              │
│  - Deep Research実行                                     │   │
│  - コード生成                                            │   │
│  - ブラウザ操作コマンド処理                               │   │
└─────────────────────────────────────────────────────────┘
```

## 実装ファイル

### 1. Chrome/Edge拡張機能

**新規作成ファイル:**

- `extensions/chrome-codex/manifest.json` - 拡張機能マニフェスト
- `extensions/chrome-codex/background/background.js` - バックグラウンドスクリプト
- `extensions/chrome-codex/content/content.js` - コンテンツスクリプト
- `extensions/chrome-codex/popup/popup.html` - ポップアップUI
- `extensions/chrome-codex/popup/popup.js` - ポップアップロジック
- `extensions/chrome-codex/styles/popup.css` - スタイル

**主要機能:**

- Native Messaging API接続
- DOM読み取り・操作
- コンソールログ取得
- ネットワークリクエスト監視
- Deep Research UI
- コード生成・実行UI

### 2. Native Messaging Host

**新規作成ファイル:**

- `codex-rs/chrome-host/Cargo.toml` - Rustプロジェクト設定
- `codex-rs/chrome-host/src/main.rs` - メインホスト実装
- `codex-rs/chrome-host/src/message.rs` - メッセージ処理
- `codex-rs/chrome-host/src/cli_bridge.rs` - Codex CLIブリッジ

**主要機能:**

- stdin/stdout経由のJSONメッセージ処理
- Codex CLIコマンド実行
- レスポンスのJSON化
- エラーハンドリング

### 3. Codex CLI拡張

**修正ファイル:**

- `codex-rs/cli/src/main.rs` - `/chrome`コマンド追加
- `codex-rs/cli/src/chrome_cmd.rs` - Chrome操作コマンド実装（新規）
- `codex-rs/core/src/chrome/` - Chrome操作モジュール（新規）

**主要機能:**

- `/chrome`コマンド実装
- DOM読み取りコマンド
- コンソールログ取得コマンド
- ネットワークリクエスト監視コマンド
- Deep Research統合

### 4. 設定ファイル

**新規作成ファイル:**

- `extensions/chrome-codex/native-messaging-host.json` - Windows用ホスト登録設定
- `extensions/chrome-codex/native-messaging-host-linux.json` - Linux用ホスト登録設定
- `extensions/chrome-codex/native-messaging-host-macos.json` - macOS用ホスト登録設定
- `extensions/chrome-codex/install-host.ps1` - Windows用インストールスクリプト
- `extensions/chrome-codex/install-host.sh` - Linux/macOS用インストールスクリプト

## 実装ステップ

### Phase 1: 基盤構築

1. **Native Messaging Host実装**

   - `codex-rs/chrome-host`プロジェクト作成
   - stdin/stdout JSONメッセージ処理
   - Codex CLI呼び出し機能

2. **Chrome拡張機能基本構造**

   - `manifest.json`作成
   - バックグラウンドスクリプト実装
   - Native Messaging API接続

3. **メッセージプロトコル定義**

   - リクエスト/レスポンス形式定義
   - エラーハンドリング仕様

### Phase 2: Deep Research統合

4. **Deep Research UI実装**

   - ポップアップUI作成
   - 検索クエリ入力
   - 結果表示

5. **Deep Research統合**

   - 拡張機能からDeep Research呼び出し
   - 結果の表示・保存

### Phase 3: ブラウザ操作機能

6. **DOM読み取り機能**

   - コンテンツスクリプト実装
   - DOM要素取得・操作
   - ページ情報取得

7. **コンソールログ取得**

   - コンソールAPI監視
   - ログ収集・送信

8. **ネットワークリクエスト監視**

   - `chrome.webRequest` API使用
   - リクエスト/レスポンス情報取得

### Phase 3: コード生成・実行

9. **コード生成機能**

   - ブラウザコンテキストに基づくコード生成
   - 生成コードのプレビュー・実行

10. **統合テスト**

    - エンドツーエンドテスト
    - クロスプラットフォームテスト

## 技術詳細

### Native Messaging API仕様

**メッセージ形式:**

```json
{
  "type": "deep_research",
  "query": "Rust async best practices",
  "options": {
    "depth": 3,
    "breadth": 10
  }
}
```

**レスポンス形式:**

```json
{
  "type": "deep_research_result",
  "success": true,
  "data": {
    "summary": "...",
    "sources": [...],
    "report_path": "/path/to/report.md"
  }
}
```

### Codex CLI `/chrome`コマンド

```bash
# Deep Research実行
codex /chrome research "query" --depth 3

# DOM読み取り
codex /chrome dom --selector "#main-content"

# コンソールログ取得
codex /chrome console --filter "error"

# ネットワークリクエスト監視
codex /chrome network --filter "api"
```

### セキュリティ考慮事項

- Native Messaging Hostのパス検証
- メッセージの署名・検証（将来実装）
- 権限の最小化
- サンドボックス化

## 参考実装

- Claude for Chrome: Chrome Native Messaging API使用
- Codex VS Code拡張機能: 既存の拡張機能実装パターン
- Codex Deep Research: 既存のDeep Research実装

## 依存関係

- Chrome/Edge拡張機能: Manifest V3
- Native Messaging Host: Rust (既存Codex CLIと統合)
- Codex CLI: 既存実装を拡張

## テスト戦略

- 単体テスト: Native Messaging Host
- 統合テスト: 拡張機能 ↔ Host ↔ CLI
- E2Eテスト: ブラウザ操作フロー
- クロスプラットフォームテスト: Windows/macOS/Linux

## 実装優先順位

1. **高**: Native Messaging Host + 基本拡張機能
2. **高**: Deep Research統合
3. **中**: DOM読み取り・コンソールログ取得
4. **中**: ネットワークリクエスト監視
5. **低**: コード生成・実行機能