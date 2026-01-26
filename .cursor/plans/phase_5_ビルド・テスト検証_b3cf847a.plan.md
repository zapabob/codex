---
name: Phase 5 ビルド・テスト検証
overview: 公式機能統合後のビルド・テスト検証を実施し、Personality機能とPlan機能の動作確認、独自機能の保護確認、実装ログの作成を行う
todos:
  - id: build_verification
    content: ビルド検証（cargo build --workspace --features custom-features）
    status: in_progress
  - id: test_execution
    content: テスト実行（cargo test --workspace --features custom-features）
    status: pending
  - id: linter_check
    content: リンター実行（cargo clippy --workspace --features custom-features -- -D warnings）
    status: pending
  - id: format_check
    content: フォーマットチェック（cargo fmt --all -- --check）
    status: pending
  - id: personality_verification
    content: Personality機能の動作確認
    status: pending
  - id: plan_verification
    content: Plan機能の動作確認
    status: pending
  - id: custom_features_verification
    content: その他独自機能の動作確認
    status: pending
  - id: create_implementation_log
    content: 実装ログ作成（_docs/に保存）
    status: pending
isProject: false
---

# Phase 5: ビルド・テスト検証計画

## 現状確認

Phase 1-4は完了済み：

- ✅ upstream/mainのマージ完了
- ✅ Personality機能統合完了（`/personality`コマンド実装済み）
- ✅ Plan機能改善完了（公式のプロンプト改善統合済み）

Phase 5の検証項目：

1. ビルド検証（`cargo build --workspace --features custom-features`）
2. テスト実行（`cargo test --workspace --features custom-features`）
3. リンター実行（`cargo clippy --workspace --features custom-features -- -D warnings`）
4. フォーマットチェック（`cargo fmt --all -- --check`）
5. 独自機能の動作確認
6. 実装ログ作成

## 実装手順

### Step 1: ビルド検証

**目的**: コンパイルエラーがないことを確認

**実行コマンド**:

```powershell
cd codex-rs
cargo build --workspace --features custom-features
```

**確認項目**:

- コンパイルエラーがないこと
- 警告の有無を確認（後で修正可能なものは記録）
- ビルド時間の記録

**対象パッケージ**:

- `codex-core` - 独自機能モジュール（orchestration, agents, plan, qc等）
- `codex-cli` - CLI実行ファイル
- `codex-tui` - TUI（Personality機能含む）
- その他ワークスペースメンバー

### Step 2: テスト実行

**目的**: 既存テストが正常に動作することを確認

**実行コマンド**:

```powershell
cargo test --workspace --features custom-features
```

**確認項目**:

- 全テストが通過すること
- 失敗したテストがあれば原因を記録
- テスト実行時間の記録

**重点確認**:

- Personality機能のテスト（`codex-rs/tui/src/chatwidget/tests.rs`）
- Plan機能のテスト（`codex-rs/core/src/plan/`）
- 独自機能のテスト（orchestration, agents, qc等）

### Step 3: リンター実行

**目的**: コード品質と警告の確認

**実行コマンド**:

```powershell
cargo clippy --workspace --features custom-features -- -D warnings
```

**確認項目**:

- クリッピー警告がないこと（`-D warnings`で警告をエラーとして扱う）
- 修正が必要な警告があれば記録

### Step 4: フォーマットチェック

**目的**: コードフォーマットの統一確認

**実行コマンド**:

```powershell
cargo fmt --all -- --check
```

**確認項目**:

- フォーマット違反がないこと
- 違反があれば自動修正可能（`--check`を外して実行）

### Step 5: 独自機能の動作確認

**目的**: 統合後も独自機能が正常に動作することを確認

**確認項目**:

1. **Personality機能**:

   - `/personality`コマンドが動作する
   - Friendly/Pragmaticの切り替えが正常
   - 設定が永続化される

2. **Plan機能**:

   - Plan生成が正常に動作する
   - 公式のプロンプト改善が反映されている
   - 独自機能（budget管理、execution log等）が維持されている

3. **その他独自機能**:

   - Orchestration機能
   - Agents機能
   - QC機能
   - Git4D機能
   - VR/AR統合機能

**確認方法**:

- 手動テスト（TUI起動してコマンド実行）
- 既存の統合テスト実行

### Step 6: 実装ログ作成

**目的**: 実装完了を記録し、今後の参照用に保存

**保存先**: `_docs/2026-01-26_公式機能統合とupstream_mainマージ完了{main}.md`

**記録内容**:

- 実施日時
- 完了したフェーズのサマリー
- ビルド・テスト結果
- 検証結果
- 残課題（あれば）
- 保護された独自機能の確認

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

1. **ビルドエラー**

   - 対策: エラーメッセージを確認し、段階的に修正

2. **テスト失敗**

   - 対策: 失敗原因を分析し、必要に応じてテストコードを更新

3. **リンター警告**

   - 対策: 重要な警告は修正、軽微なものは記録のみ

4. **機能の破壊**

   - 対策: 各機能を個別にテストし、問題があれば即座に修正

## 完了基準

- [ ] ビルドが成功する（エラー0件）
- [ ] テストが全て通過する
- [ ] リンターエラーがない（警告は記録のみ可）
- [ ] フォーマットチェックが通過する
- [ ] `/personality`コマンドが動作する
- [ ] Plan機能が正常に動作する
- [ ] 独自機能が正常に動作する
- [ ] 実装ログを作成済み