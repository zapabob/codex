---
name: 公式機能統合とupstream/mainマージ
overview: upstream/mainの最新変更を取り込み、Personality機能の統合とPlan機能の改善を実施し、独自機能を保護しながら公式機能を統合する
todos:
  - id: merge_preparation
    content: upstream/mainのマージ準備（リモート設定確認、独自機能保護確認）
    status: completed
  - id: merge_execution
    content: upstream/mainのマージ実行とコンフリクト解決
    status: completed
  - id: personality_integration
    content: Personality機能の統合（/personalityスラッシュコマンドの実装）
    status: completed
  - id: plan_improvement
    content: Plan機能の改善（公式のプロンプト改善を取り込み）
    status: completed
  - id: build_test_verification
    content: ビルド・テスト検証と独自機能の動作確認
    status: in_progress
isProject: false
---

# 公式機能統合とupstream/mainマージ計画

## 概要

upstream/main（OpenAI公式リポジトリ）の最新変更を取り込み、以下の作業を実施します：

1. upstream/mainのマージ
2. Personality機能の統合（`/personality`スラッシュコマンドの実装）
3. Plan機能の改善（公式のプロンプト改善を取り込み）
4. ビルド・テスト検証

## 実装フェーズ

### Phase 1: upstream/mainのマージ準備

**目的**: マージ前に独自機能を保護し、コンフリクトを最小化

**作業内容**:

1. 現在のブランチ状態を確認

   - `git status`で変更状況を確認
   - 未コミットの変更があればコミット

2. upstreamリモートの設定確認

   - `git remote -v`でupstreamが設定されているか確認
   - 未設定の場合は追加: `git remote add upstream <upstream-url>`

3. upstream/mainの最新取得

   - `git fetch upstream main`
   - 最新コミットを確認

4. 独自機能の保護確認

   - `#[cfg(feature = "custom-features")]`が適切に設定されているか確認
   - 保護対象ディレクトリ（`.codex/`, `.cursor/`, `.serena/`, `.specstory/`, `archive/`）の存在確認

**対象ファイル**:

- `codex-rs/core/src/lib.rs` - 条件付きコンパイルの確認

### Phase 2: upstream/mainのマージ実行

**目的**: 公式の最新変更を取り込む

**作業内容**:

1. マージ実行

   - `git merge upstream/main` または `git rebase upstream/main`
   - コンフリクトが発生した場合は解決

2. コンフリクト解決時の注意事項

   - 独自機能ディレクトリは削除しない
   - 独自機能モジュールは`#[cfg(feature = "custom-features")]`で保護
   - 公式のセキュリティ修正・バグフィクスは優先的に取り込む

3. マージ後の検証

   - `cargo check --workspace`でビルドエラーがないか確認
   - 基本的なコンパイルエラーを修正

**対象ファイル**:

- コンフリクトが発生した全ファイル

### Phase 3: Personality機能の統合

**目的**: 公式の`/personality`スラッシュコマンドを実装し、既存のpersonalityフィールドと統合

**作業内容**:

1. スラッシュコマンドの追加

   - `codex-rs/tui/src/slash_command.rs`の`SlashCommand` enumに`Personality`を追加
   - `description()`メソッドに説明を追加
   - `available_during_task()`メソッドで利用可能性を設定

2. コマンドハンドラーの実装

   - `codex-rs/tui/src/chatwidget.rs`の`dispatch_command()`メソッドに`Personality`ケースを追加
   - パーソナリティ選択用のポップアップを表示
   - 選択されたパーソナリティを`SessionConfiguration`に反映

3. 既存実装との統合

   - `codex-rs/core/src/codex.rs`の既存の`personality`フィールドを使用
   - `codex-rs/protocol/src/config_types.rs`の`Personality` enum（Friendly, Pragmatic）を使用

4. UI実装

   - パーソナリティ選択用のポップアップUIを実装
   - 現在のパーソナリティを表示
   - 選択肢: Friendly, Pragmatic

**対象ファイル**:

- `codex-rs/tui/src/slash_command.rs` - `Personality`コマンドの追加
- `codex-rs/tui/src/chatwidget.rs` - コマンドハンドラーの実装
- `codex-rs/tui/src/bottom_pane/` - パーソナリティ選択UI（必要に応じて）

**参考実装**:

- 公式の`/personality`実装（マージ後に確認）
- 既存の`/model`コマンドの実装パターン

### Phase 4: Plan機能の改善

**目的**: 公式のプロンプト改善を取り込み、独自実装に統合

**作業内容**:

1. 公式のプロンプト改善を確認

   - マージ後の公式コードからPlan関連のプロンプトを確認
   - コミット #9877, #9874 の変更内容を確認

2. 独自実装への統合

   - `codex-rs/core/src/plan/`の既存実装を確認
   - 公式のプロンプト改善を独自実装に統合
   - 既存の高度な機能（budget管理、execution log、orchestration等）を維持

3. プロンプトファイルの更新

   - Plan生成用のプロンプトテンプレートを更新
   - 公式の改善点を反映

**対象ファイル**:

- `codex-rs/core/src/plan/` - Plan機能の実装
- Plan関連のプロンプトファイル（存在する場合）

**注意事項**:

- 独自実装の高度な機能は維持
- 公式のプロンプト改善のみを取り込む

### Phase 5: ビルド・テスト検証

**目的**: 統合後の動作確認と品質保証

**作業内容**:

1. ビルド検証
   ```powershell
   cd codex-rs
   cargo build --workspace --features custom-features
   ```

2. テスト実行
   ```powershell
   cargo test --workspace --features custom-features
   ```

3. リンター実行
   ```powershell
   cargo clippy --workspace --features custom-features -- -D warnings
   ```

4. フォーマットチェック
   ```powershell
   cargo fmt --all -- --check
   ```

5. 独自機能の動作確認

   - Personality機能の動作確認
   - Plan機能の動作確認
   - 既存の独自機能（Git4D、QC、Orchestration等）の動作確認

**検証項目**:

- [ ] ビルドが成功する
- [ ] テストが全て通過する
- [ ] リンターエラーがない
- [ ] `/personality`コマンドが動作する
- [ ] Plan機能が正常に動作する
- [ ] 独自機能が正常に動作する

## 保護対象

以下のディレクトリ・モジュールは削除しないよう保護：

- **独自機能ディレクトリ**:
  - `.codex/` - 独自機能・スキル・設定
  - `.cursor/` - Cursor設定・計画・ルール
  - `.serena/` - Serena設定
  - `.specstory/` - 仕様・ストーリー
  - `archive/` - アーカイブ

- **Rustモジュール**（`#[cfg(feature = "custom-features")]`で保護）:
  - `codex-rs/core/src/orchestration/` - オーケストレーション
  - `codex-rs/core/src/agents/` - エージェントシステム
  - `codex-rs/core/src/plan/` - 計画モード（独自実装）
  - `codex-rs/core/src/qc/` - QC機能
  - `codex-rs/core/src/cowork_integration.rs` - Cowork統合
  - `codex-rs/core/src/vr_ar_integration.rs` - VR/AR統合
  - `codex-rs/core/src/git4d_accelerated.rs` - Git4D加速
  - `codex-rs/core/src/superior_git4d_visualizer.rs` - Git4D可視化

## リスクと対策

1. **マージコンフリクト**

   - 対策: 事前に独自機能を保護し、コンフリクト解決時に独自機能を優先

2. **ビルドエラー**

   - 対策: 段階的にマージし、各段階でビルド検証

3. **機能の破壊**

   - 対策: 各フェーズでテストを実行し、既存機能の動作を確認

## 完了確認

- [ ] upstream/mainのマージが完了
- [ ] `/personality`コマンドが実装され、動作確認済み
- [ ] Plan機能の改善が完了
- [ ] ビルド・テストが全て通過
- [ ] 独自機能が正常に動作
- [ ] 実装ログを作成