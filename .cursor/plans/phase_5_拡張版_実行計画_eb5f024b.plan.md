---
name: Phase 5 拡張版 実行計画
overview: Phase 5拡張版計画を実行可能な形に整理し、既存実装の確認と新規実装項目を明確化。ビルド検証から順次実施し、各フェーズで完了基準を確認。
todos:
  - id: phase_5_1_build
    content: "Phase 5.1: 基本ビルド・テスト検証（cargo build, test, clippy, fmt, tsc）"
    status: in_progress
  - id: phase_5_2_types
    content: "Phase 5.2: 型安全性と警告0達成（Rust/TypeScript警告修正、ゼロトラスト検証）"
    status: pending
  - id: phase_5_3_react
    content: "Phase 5.3: React 19.2.1+への安全なアップグレード（CVE-2025-55182対策）"
    status: pending
  - id: phase_5_4_vr_ar
    content: "Phase 5.4: VR/AR機能強化（Apple Glass、VIVE、SteamVR対応）"
    status: pending
  - id: phase_5_5_sandbox
    content: "Phase 5.5.1: macOSライクサンドボックス + cowork統合"
    status: pending
  - id: phase_5_5_research
    content: "Phase 5.5.2: DeepResearchレポート強化"
    status: pending
  - id: phase_5_5_windows
    content: "Phase 5.5.3: Windows 11 25H2 MCP対応"
    status: pending
  - id: phase_5_5_ransomware
    content: "Phase 5.5.4: ランサムウェア対策の追加"
    status: pending
  - id: phase_5_5_parallel_gui
    content: "Phase 5.5.5: 複数AIツール並列実行GUI実装"
    status: pending
  - id: phase_5_6_review
    content: "Phase 5.6: コードレビューとAPI再実装（Git差分分析、非推奨API置き換え）"
    status: pending
  - id: phase_5_7_log
    content: "Phase 5.7: 実装ログ作成（_docs/に保存）"
    status: pending
isProject: false
---

# Phase 5 拡張版: 実行計画

## 現状分析

### 既に実装済みの機能

- ✅ マルウェア検知: `codex-rs/core/src/malware_detector.rs`, `codex-rs/core/src/security/malware_detector.rs`
- ✅ DeepResearch: `codex-rs/cli/src/research_cmd.rs` (CLI統合済み)
- ✅ VR/AR機能: `gui/src/lib/xr/webxr-manager.ts`, `codex-rs/core/src/vr_ar_integration.rs`
- ✅ サンドボックス: `codex-rs/core/src/sandboxing/`, `codex-rs/core/src/seatbelt.rs`
- ✅ Plan機能: `codex-rs/core/src/plan/` (公式プロンプト改善統合済み)
- ✅ Personality機能: `/personality`コマンド実装済み

### 要実装・強化項目

- ⚠️ React 19.2.1+へのアップグレード（現在React 18）
- ⚠️ 型安全性と警告0達成（Rust/TypeScript）
- ⚠️ ゼロトラストセキュリティ実装の検証・強化
- ⚠️ VR/AR追加デバイス対応（Apple Glass、VIVE、SteamVR）
- ⚠️ macOSライクサンドボックス + cowork統合
- ⚠️ Windows 11 25H2 MCP対応
- ⚠️ ランサムウェア対策の追加
- ⚠️ 複数AIツール並列実行GUI

## 実行フェーズ

### Phase 5.1: 基本ビルド・テスト検証（最優先）

**目的**: 現在のコードベースが正常にビルド・テストできることを確認

**実行手順**:

1. Rustビルド検証
   ```powershell
   cd codex-rs
   cargo build --workspace --features custom-features
   ```


   - エラー0件、警告数を記録

2. テスト実行
   ```powershell
   cargo test --workspace --features custom-features
   ```


   - 全テスト通過を確認

3. リンターチェック
   ```powershell
   cargo clippy --workspace --features custom-features -- -D warnings
   ```


   - 警告0件を目標

4. フォーマットチェック
   ```powershell
   cargo fmt --all -- --check
   ```

5. TypeScript型チェック
   ```powershell
   cd gui; npm run type-check  # または tsc --noEmit
   cd ../prism-web; npm run type-check
   ```


**完了基準**:

- [ ] ビルド成功（エラー0件）
- [ ] 全テスト通過
- [ ] Clippy警告0件
- [ ] フォーマットチェック通過
- [ ] TypeScript型エラー0件

### Phase 5.2: 型安全性と警告0達成

**目的**: すべての型定義を完全にし、コンパイル警告を0にする

**実行手順**:

1. Rust側の警告修正

   - `cargo clippy`の結果を分析
   - `#[allow(...)]`の削除と適切な修正
   - 未使用変数・インポートの整理

2. TypeScript側の型エラー修正

   - `tsc --noEmit`の結果を分析
   - `any`型の削除
   - 未定義型の特定と修正

3. ゼロトラストセキュリティ実装の検証

   - `codex-rs/core/src/security/`の実装確認
   - `codex-rs/core/src/mcp/secure_connection.rs`の確認
   - 通信の暗号化検証
   - 認証・認可の多要素化確認

**対象ファイル**:

- `codex-rs/**/*.rs` - すべてのRustファイル
- `gui/**/*.ts`, `gui/**/*.tsx` - GUIのTypeScriptファイル
- `prism-web/**/*.ts`, `prism-web/**/*.tsx` - Prism WebのTypeScriptファイル
- `extensions/**/*.ts`, `extensions/**/*.tsx` - 拡張機能のTypeScriptファイル

**完了基準**:

- [ ] `cargo clippy --workspace --features custom-features -- -D warnings`で警告0
- [ ] `tsc --noEmit`で型エラー0
- [ ] ゼロトラストセキュリティ実装の検証完了

### Phase 5.3: React 19.2.1+への安全なアップグレード

**目的**: React 19の最新パッチ版に更新し、CVE-2025-55182を回避

**注意事項**:

- React 19には重大な脆弱性（CVE-2025-55182）が存在
- 19.0.1、19.1.2、19.2.1で修正済み
- 必ず最新のパッチ版（19.2.1以降）を使用

**実行手順**:

1. 依存関係の確認と更新
   ```powershell
   cd gui
   npm audit
   npm install react@^19.2.1 react-dom@^19.2.1 @types/react@^19 @types/react-dom@^19
   
   cd ../prism-web
   npm audit
   npm install react@^19.2.1 react-dom@^19.2.1 @types/react@^19 @types/react-dom@^19
   
   cd ../extensions/codex-viz-web/frontend
   npm audit
   npm install react@^19.2.1 react-dom@^19.2.1
   
   cd ../../codex-rs/tauri-gui
   npm audit
   npm install react@^19.2.1 react-dom@^19.2.1
   
   cd ../../examples/react-todo
   npm audit
   npm install react@^19.2.1 react-dom@^19.2.1
   ```

2. 破壊的変更への対応

   - React 19の新しいAPIへの移行
   - Server Componentsの適切な使用
   - 非推奨APIの置き換え
   - 型定義の更新

3. ビルドとテスト
   ```powershell
   cd gui; npm run build
   cd ../prism-web; npm run build
   ```


**完了基準**:

- [ ] すべてのpackage.jsonでReact 19.2.1+に更新
- [ ] `npm audit`で脆弱性0件
- [ ] ビルド成功
- [ ] 既存機能の動作確認

### Phase 5.4: VR/AR機能強化

**目的**: Quest 2/3、Apple Glass、VIVE、SteamVRへの対応拡張

**現状**: 既存実装あり（`gui/src/lib/xr/webxr-manager.ts`, `codex-rs/core/src/vr_ar_integration.rs`）

**実行手順**:

1. 既存実装の確認

   - Quest 2/3対応の確認と最適化
   - WebXR APIの拡張状況確認

2. 追加デバイス対応

   - Apple Glass (ARKit) 対応の追加
   - VIVE (OpenXR) 対応の追加
   - SteamVR (OpenVR) 対応の追加

**対象ファイル**:

- `gui/src/components/visualization/VRInterface.tsx`
- `gui/src/lib/xr/webxr-manager.ts`
- `prism-web/lib/xr/webxr-manager.ts`
- `codex-rs/tui/src/git_visualizer.rs`
- `codex-rs/core/src/vr_ar_integration.rs`

**完了基準**:

- [ ] Quest 2/3で動作確認
- [ ] Apple Glass対応実装
- [ ] VIVE対応実装
- [ ] SteamVR対応実装

### Phase 5.5: その他機能強化

#### 5.5.1: macOSライクサンドボックス + cowork統合

**現状**: サンドボックス実装あり（`codex-rs/core/src/sandboxing/`, `codex-rs/core/src/seatbelt.rs`）

**実行手順**:

1. 既存サンドボックス機能の確認
2. cowork統合機能の追加

   - サンドボックス内でのcowork機能の利用
   - ファイル共有とアクセス制御
   - ネットワーク分離と許可リスト

**対象ファイル**:

- `codex-rs/core/src/sandboxing/mod.rs`
- `codex-rs/core/src/cowork_integration.rs`
- `codex-rs/core/src/qc/sandboxed_execution.rs`

#### 5.5.2: DeepResearchレポート強化

**現状**: DeepResearch実装あり（`codex-rs/cli/src/research_cmd.rs`）

**実行手順**:

1. 既存機能の確認
2. Markdownレポート生成の強化

   - レポートテンプレートの改善
   - 図表の追加
   - 引用とソースの整理

**対象ファイル**:

- `codex-rs/cli/src/research_cmd.rs` - `generate_markdown_report()`関数
- `codex-rs/mcp-server/src/deep_research_tool_handler.rs`

#### 5.5.3: Windows 11 25H2 MCP対応

**実行手順**:

1. Windows 11 25H2 APIの調査
2. MCPサーバーの拡張

   - Windows 11 25H2専用MCPツールの追加
   - AI加速機能の統合

**対象ファイル**:

- `codex-rs/mcp-server/src/` - MCPサーバー実装
- `codex-rs/core/src/windows_ai_integration.rs` - Windows AI統合

#### 5.5.4: ランサムウェア対策の追加

**現状**: マルウェア検知実装あり（`codex-rs/core/src/malware_detector.rs`）

**実行手順**:

1. 既存機能の確認
2. ランサムウェア対策の追加

   - ファイル暗号化パターンの検知
   - 異常なファイル変更の監視
   - 自動バックアップ機能

**対象ファイル**:

- `codex-rs/core/src/malware_detector.rs`
- `codex-rs/core/src/security/malware_detector.rs`
- `codex-rs/core/src/security/quarantine.rs`

#### 5.5.5: 複数AIツール並列実行GUI

**実行手順**:

1. 既存機能の確認

   - `codex-rs/core/src/orchestration/parallel_execution.rs`
   - `codex-rs/core/src/ai_tool_manager.rs`

2. GUI実装

   - 複数AIツール選択UI
   - 自然言語プロンプト入力フィールド
   - 並列実行状況の可視化
   - 結果の統合表示

**対象ファイル**:

- `gui/src/components/ai-tools/` - 新規作成
- `codex-rs/core/src/orchestration/parallel_execution.rs`
- `codex-rs/core/src/ai_tool_manager.rs`

### Phase 5.6: コードレビューとAPI再実装

**実行手順**:

1. Git差分分析
   ```powershell
   git diff upstream/main...HEAD --name-only
   git log --oneline --since="2025-01-01" --until="2026-01-26"
   ```

2. コードレビュー

   - 既存コードの品質評価
   - 改善点の特定
   - 非推奨APIの使用箇所の特定
   - セキュリティ問題の特定

3. 最新APIへの再実装

   - 非推奨APIの置き換え
   - 最新のベストプラクティスの適用
   - パフォーマンス最適化

### Phase 5.7: 実装ログ作成

**実行手順**:

1. すべての変更の記録
2. `_docs/`への保存

   - ファイル名: `yyyy-mm-dd_機能名{worktreename}.md`
   - 実装内容、変更点、テスト結果を記録

## 保護対象の確認

以下のモジュールが`#[cfg(feature = "custom-features")]`で保護されていることを確認：

- `codex-rs/core/src/orchestration/`
- `codex-rs/core/src/agents/`
- `codex-rs/core/src/plan/`
- `codex-rs/core/src/qc/`
- `codex-rs/core/src/cowork_integration.rs`
- `codex-rs/core/src/vr_ar_integration.rs`
- `codex-rs/core/src/git4d_accelerated.rs`
- `codex-rs/core/src/superior_git4d_visualizer.rs`

## リスクと対策

1. **React 19アップグレード**

   - リスク: 破壊的変更による動作不良
   - 対策: 段階的なアップグレード、十分なテスト

2. **型定義と警告0**

   - リスク: 大規模なコード変更が必要
   - 対策: 段階的な修正、既存機能の回帰テスト

3. **ゼロトラスト実装**

   - リスク: パフォーマンスへの影響
   - 対策: パフォーマンステスト、段階的な実装

## 完了基準（全体）

- [ ] ビルドが成功する（エラー0件、警告0件）
- [ ] テストが全て通過する
- [ ] リンターエラーがない（警告0件）
- [ ] フォーマットチェックが通過する
- [ ] TypeScript型エラーがない
- [ ] React 19.2.1+に更新済み
- [ ] ゼロトラストセキュリティ実装済み
- [ ] VR/AR機能が追加デバイスで動作する
- [ ] macOSライクサンドボックスが動作する
- [ ] DeepResearchレポートが正常に生成される
- [ ] Windows 11 25H2 MCPが動作する
- [ ] マルウェア検知が正常に動作する
- [ ] 複数AIツール並列実行GUIが動作する
- [ ] コードレビューと再実装が完了している
- [ ] 実装ログを作成済み