---
name: クリーンビルドとサブエージェント・スキル作成
overview: クリーン高速ビルドでバイナリをコピーアンドペーストで上書きインストールするスクリプトを作成し、Rust/TSの型定義・警告0・ゼロトラスト・セキュアコーディング用のサブエージェント、Cursor/ClaudeCode風のplanモード作成スキル、DeepresearchをCursorのWEB検索で実行するスキルを作成する
todos:
  - id: clean-build-script
    content: クリーン高速ビルドとコピーアンドペーストインストールスクリプト作成（tqdm風進捗表示付き）
    status: completed
  - id: secure-code-subagent
    content: Rust/TS型定義・警告0・ゼロトラスト・セキュアコーディング用サブエージェント作成
    status: completed
  - id: plan-mode-skill
    content: Cursor/ClaudeCode風のplanモード作成スキル改善（2フェーズアプローチ）
    status: completed
  - id: deepresearch-skill
    content: DeepresearchをCursorのWEB検索で実行できるようにするスキル改善
    status: completed
isProject: false
---

# クリーンビルドとサブエージェント・スキル作成プラン

## 現状分析

### 既存のビルドシステム

- `scripts/build-binary.js`: メインのビルドスクリプト（Node.js）
- `scripts/install_with_kill.ps1`: プロセスキルとインストールスクリプト
- `codex-rs/fast-build-install.ps1`: 高速差分ビルドスクリプト（tqdm風進捗表示）
- バイナリパス: `codex-rs/target/release/codex.exe` (Windows)
- インストール先: `$env:USERPROFILE\.cargo\bin\codex.exe` または `C:\bin\codex.exe`

### 既存のサブエージェント

- `.cursor/agents/rust-error-fixer.md`: Rustコンパイルエラー修正
- `.cursor/agents/unsafe-warning-fixer.md`: Rust 2024 unsafe警告修正

### 既存のスキル

- `.cursor/skills/plan-mode/`: Cursor/ClaudeCode風のplanモード作成スキル（既存）
- `.cursor/skills/web-search-deepresearch/`: Deepresearchスキル（既存）

## 実装タスク

### Task 1: クリーン高速ビルドとコピーアンドペーストインストールスクリプト作成

**目的**: tqdm風の進捗表示付きで、クリーンビルド→バイナリコピー→上書きインストールを一括実行

**実装ファイル**: `scripts/clean-build-install.ps1`

**機能要件**:

1. クリーンビルド実行（`cargo clean` → `cargo build --release -p codex-cli`）
2. tqdm風の進捗表示（残り時間、経過時間、コンパイル中のクレート数）
3. ビルド完了後、バイナリを検出
4. 実行中のプロセスを自動検出・終了
5. バイナリをコピーアンドペーストで上書きインストール
6. インストール先の選択（`$env:USERPROFILE\.cargo\bin\codex.exe` または `C:\bin\codex.exe`）

**参考実装**:

- `scripts/build-binary.js`: ビルドロジック
- `scripts/install_with_kill.ps1`: プロセスキルとインストールロジック
- `codex-rs/fast-build-install.ps1`: tqdm風進捗表示
- `scripts/scripts/monitor-build-tqdm.ps1`: 進捗モニタリング

**実装内容**:

```powershell
# 1. クリーンビルド実行
cargo clean
cargo build --release -p codex-cli

# 2. 進捗表示（tqdm風）
# - コンパイル中のクレート数を監視
# - 残り時間・経過時間を表示
# - プログレスバーを表示

# 3. バイナリ検出
$binaryPath = "codex-rs\target\release\codex.exe"

# 4. プロセスキル
Get-Process codex -ErrorAction SilentlyContinue | Stop-Process -Force

# 5. コピーアンドペーストで上書きインストール
Copy-Item -Path $binaryPath -Destination $installPath -Force
```

### Task 2: Rust/TS型定義・警告0・ゼロトラスト・セキュアコーディング用サブエージェント作成

**目的**: RustとTypeScriptの型定義エラー、警告0達成、ゼロトラスト設計、セキュアコーディングのベストプラクティスを実装する専門サブエージェント

**実装ファイル**: `.cursor/agents/secure-code-expert.md`

**機能要件**:

1. Rust型定義エラー修正
2. TypeScript型定義エラー修正
3. 警告0達成（Rust: `-D warnings`, TypeScript: `strict: true`）
4. ゼロトラスト設計の実装（TLS 1.3、mTLS、Ed25519署名）
5. セキュアコーディングベストプラクティスの適用
6. ソフトウェア工学的ベストプラクティスの適用

**参考実装**:

- `.cursor/agents/rust-error-fixer.md`: Rustエラー修正の構造
- `.cursor/agents/unsafe-warning-fixer.md`: 警告修正の構造

**実装内容**:

```markdown
---
name: secure-code-expert
description: Rust/TS型定義・警告0・ゼロトラスト・セキュアコーディングの専門家。型定義エラー修正、警告0達成、ゼロトラスト設計、セキュアコーディングベストプラクティスを自動的に適用。Use proactively when working with Rust/TypeScript code, security-sensitive features, or zero-trust architecture.
---

# セキュアコーディング専門エージェント

## 主要機能

### 1. 型定義エラー修正
- Rust: 型不一致、ライフタイムエラー、所有権エラー
- TypeScript: 型エラー、strictモード違反、null安全性

### 2. 警告0達成
- Rust: `-D warnings`で全警告をエラー化
- TypeScript: `strict: true`で厳格な型チェック

### 3. ゼロトラスト設計
- TLS 1.3の実装
- mTLS（相互TLS認証）
- Ed25519署名の実装
- 最小権限の原則

### 4. セキュアコーディング
- 入力検証
- SQLインジェクション対策
- XSS対策
- CSRF対策
- セキュアなパスワードハンドリング

### 5. ソフトウェア工学的ベストプラクティス
- SOLID原則
- DRY原則
- テスト駆動開発
- コードレビュー
```

### Task 3: Cursor/ClaudeCode風のplanモード作成スキル改善

**目的**: 既存のplan-modeスキルをCursor/ClaudeCode風の2フェーズアプローチに改善

**実装ファイル**: `.cursor/skills/plan-mode/SKILL.md` (更新)

**改善内容**:

1. 2フェーズアプローチの明確化
  - Phase 1: Intent chat（意図の理解）
  - Phase 2: Implementation chat（実装計画）
2. 質問のバッチ処理（4-10問）
3. 発見可能な事実の探索優先
4. 好み/トレードオフの早期質問

**参考実装**:

- `codex-rs/core/src/plan/mod.rs`: Planモードの実装
- `codex-rs/core/templates/collaboration_mode/plan.md`: 公式Plan Modeテンプレート

### Task 4: DeepresearchをCursorのWEB検索で実行するスキル改善

**目的**: 既存のweb-search-deepresearchスキルをCursorのWEB検索機能と統合

**実装ファイル**: `.cursor/skills/web-search-deepresearch/SKILL.md` (更新)

**改善内容**:

1. CursorのWEB検索機能との統合
2. MCPサーバー経由での検索実行
3. 検索結果の統合と分析
4. 深層研究パイプラインの実行

**参考実装**:

- `codex-rs/deep-research/`: Deep Research実装
- `codex-rs/mcp-server/src/deep_research_tool_handler.rs`: MCPツールハンドラー
- `codex-rs/cli/src/research_cmd.rs`: リサーチコマンド

## 実装順序

1. **Task 1**: クリーン高速ビルドスクリプト作成（最優先）
2. **Task 2**: セキュアコーディング専門サブエージェント作成
3. **Task 3**: Planモードスキル改善
4. **Task 4**: Deepresearchスキル改善

## 成功基準

### Task 1

- クリーンビルドが正常に実行される
- tqdm風の進捗表示が機能する（残り時間・経過時間表示）
- バイナリが正常にコピーアンドペーストでインストールされる
- 実行中のプロセスが自動検出・終了される

### Task 2

- Rust/TS型定義エラーを修正できる
- 警告0を達成できる
- ゼロトラスト設計を実装できる
- セキュアコーディングベストプラクティスを適用できる

### Task 3

- 2フェーズアプローチが明確に実装されている
- 質問のバッチ処理が機能する
- 発見可能な事実の探索が優先される

### Task 4

- CursorのWEB検索機能と統合されている
- MCPサーバー経由で検索が実行できる
- 深層研究パイプラインが正常に動作する

## リスクと対策

**リスク1**: ビルドスクリプトが既存のビルドシステムと競合

- **対策**: 既存のスクリプトを参考にし、互換性を維持

**リスク2**: サブエージェントが既存のエージェントと重複

- **対策**: 既存のエージェントを確認し、機能を明確に分離

**リスク3**: スキルの改善が既存の機能を破壊

- **対策**: 既存のスキルをバックアップし、段階的に改善

