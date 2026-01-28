# 2026-01-29 実装ログ（Cursor VSIX拡張機能統合）

## 取り組み内容

### VSIXでCursorでCodexのCowork、Deep Research、Plan Mode、macOS風サンドボックスを使えるようにする

#### 1. 実装概要

既存のVS Code拡張（`extensions/vscode-codex/`）を拡張して、以下の機能を統合：

- **Cowork機能**: ブラウザ自動化、ドキュメント生成（Excel/Word/PowerPoint）、外部サービス統合（Asana/Notion）
- **Deep Research**: 既存実装を強化（承認ダイアログ、Gemini統合、履歴管理）
- **Plan Mode**: 既存のBlueprint Modeを拡張（承認ゲート、実行モード、予算管理）
- **macOS風サンドボックス**: macOS Seatbelt、Linux seccomp、Windows Restricted Token対応

#### 2. 実装内容

##### Cowork統合モジュール

- **ファイル**: `extensions/vscode-codex/src/cowork/manager.ts`
- **機能**:
  - ブラウザ自動化（`codex.cowork.browser.navigate`, `codex.cowork.browser.automate`）
  - ドキュメント生成（`codex.cowork.document.excel`, `codex.cowork.document.word`, `codex.cowork.document.powerpoint`）
  - 外部サービス統合（`codex.cowork.connector.asana`, `codex.cowork.connector.notion`）
  - セッション管理（`codex.cowork.session.create`, `codex.cowork.session.list`）

##### Sandbox管理モジュール

- **ファイル**: `extensions/vscode-codex/src/sandbox/manager.ts`
- **機能**:
  - サンドボックス設定（`codex.sandbox.configure`）
  - ステータス表示（`codex.sandbox.status`）
  - 有効化/無効化（`codex.sandbox.enable`, `codex.sandbox.disable`）
  - パス許可/拒否（`codex.sandbox.allowPath`, `codex.sandbox.denyPath`）
  - プラットフォーム自動検出（macOS Seatbelt、Linux seccomp、Windows Restricted Token）

##### Deep Research強化

- **ファイル**: `extensions/vscode-codex/src/views/researchProvider.ts`, `src/extension.ts`
- **変更内容**:
  - 研究履歴の実際の読み込み実装（`research.history` RPC呼び出し）
  - 承認ダイアログ統合（`codex.research.requireApproval`設定）
  - Gemini統合オプション（`codex.gemini.authMethod`設定）
  - 深度選択UI（1-5段階）

##### Plan Mode強化

- **既存実装**: `extensions/vscode-codex/src/plan/commands.ts`
- **確認済み機能**:
  - Plan作成、承認、拒否、エクスポート
  - 実行モード設定（single/orchestrated/competition）
  - Deep Research統合（承認ダイアログ付き）

#### 3. package.json更新

##### コマンド追加

```json
{
  "commands": [
    // Cowork (8コマンド)
    "codex.cowork.browser.navigate",
    "codex.cowork.browser.automate",
    "codex.cowork.document.excel",
    "codex.cowork.document.word",
    "codex.cowork.document.powerpoint",
    "codex.cowork.connector.asana",
    "codex.cowork.connector.notion",
    "codex.cowork.session.create",
    "codex.cowork.session.list",
    // Sandbox (6コマンド)
    "codex.sandbox.configure",
    "codex.sandbox.status",
    "codex.sandbox.enable",
    "codex.sandbox.disable",
    "codex.sandbox.allowPath",
    "codex.sandbox.denyPath"
  ]
}
```

##### ビュー追加

```json
{
  "views": {
    "codex-sidebar": [
      "codex.coworkTasks",
      "codex.sandboxStatus"
    ]
  }
}
```

##### 設定追加

```json
{
  "codex.cowork.enabled": true,
  "codex.cowork.pythonPath": "python3",
  "codex.sandbox.enabled": true,
  "codex.sandbox.networkAccess": false,
  "codex.sandbox.internetAccess": false,
  "codex.sandbox.filesystemAccess": "read-write"
}
```

#### 4. extension.ts統合

- `CoworkManager`と`SandboxManager`を初期化
- コマンド登録を統合
- 既存のOrchestratorClientと連携

#### 5. VSIXパッケージング

- **スクリプト**: `extensions/vscode-codex/package-vsix.ps1`（既存）
- **使用方法**:
  ```powershell
  cd extensions/vscode-codex
  .\package-vsix.ps1 -Version 2.12.0 -Install
  ```

## 技術的詳細

### Cowork統合アーキテクチャ

```
VS Code Extension
    ↓
CoworkManager
    ↓
OrchestratorClient (RPC)
    ↓
Codex Rust Core (cowork_integration.rs)
    ↓
Python Scripts (scripts/cowork_*.py)
    ↓
External Services (Playwright, Office Libraries, APIs)
```

### Sandbox統合アーキテクチャ

```
VS Code Extension
    ↓
SandboxManager
    ↓
OrchestratorClient (RPC)
    ↓
Codex Rust Core (sandboxing/mod.rs)
    ↓
Platform-specific Sandbox
    ├─ macOS: Seatbelt (sandbox-exec)
    ├─ Linux: seccomp/Landlock
    └─ Windows: Restricted Token
```

### Deep Research統合

- **RPCメソッド**: `research.execute`, `research.history`
- **設定**: `codex.research.maxDepth`, `codex.research.maxSources`, `codex.research.requireApproval`
- **Gemini統合**: `codex.gemini.authMethod` (oauth/api-key)

### Plan Mode統合

- **既存実装**: `src/plan/commands.ts`で完全実装済み
- **RPCメソッド**: `Plan.create`, `Plan.approve`, `Plan.reject`, `Plan.export`, `Plan.setMode`
- **状態管理**: `PlanStateManager`で管理

## 使用方法

### Cowork機能

1. **ブラウザ自動化**:
   - Command Palette → `Codex: Cowork: Navigate Browser`
   - URLを入力してブラウザを開く

2. **ドキュメント生成**:
   - Command Palette → `Codex: Cowork: Generate Excel/Word/PowerPoint`
   - データを入力して保存先を選択

3. **外部サービス統合**:
   - Command Palette → `Codex: Cowork: Connect Asana/Notion`
   - APIキーを入力

### Sandbox機能

1. **サンドボックス設定**:
   - Command Palette → `Codex: Sandbox: Configure`
   - ネットワークアクセス、ファイルシステムアクセス、インターネットアクセスを設定

2. **ステータス確認**:
   - Command Palette → `Codex: Sandbox: Show Status`
   - 現在のサンドボックス設定を表示

3. **パス許可/拒否**:
   - Command Palette → `Codex: Sandbox: Allow Path` / `Deny Path`
   - フォルダを選択して許可/拒否リストに追加

### Deep Research

1. **研究実行**:
   - Command Palette → `Codex: Deep Research`
   - クエリと深度を入力
   - 承認ダイアログで承認

2. **履歴確認**:
   - サイドバー → `Research History`ビュー
   - 過去の研究結果を確認

### Plan Mode

1. **Plan作成**:
   - Command Palette → `Codex: Create Blueprint`
   - タイトルとゴールを入力

2. **承認/実行**:
   - Command Palette → `Codex: Approve Blueprint`
   - 承認後に実行

## 今後の拡張

- [ ] CoworkタスクのTree View実装
- [ ] SandboxステータスのTree View実装
- [ ] ブラウザ自動化のワークフローエディタ
- [ ] ドキュメント生成のテンプレート管理
- [ ] サンドボックスの詳細ログ表示

## 関連ファイル

- `extensions/vscode-codex/src/cowork/manager.ts` - Cowork統合
- `extensions/vscode-codex/src/sandbox/manager.ts` - Sandbox管理
- `extensions/vscode-codex/src/plan/commands.ts` - Plan Mode（既存）
- `extensions/vscode-codex/src/views/researchProvider.ts` - Deep Research（強化）
- `extensions/vscode-codex/package.json` - コマンド・設定定義
- `extensions/vscode-codex/src/extension.ts` - 統合エントリーポイント
- `extensions/vscode-codex/package-vsix.ps1` - VSIXパッケージングスクリプト

---

**実装者**: zapabob  
**バージョン**: 2.12.0  
**日付**: 2026-01-29
