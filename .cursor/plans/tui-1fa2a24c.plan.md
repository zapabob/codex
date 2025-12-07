<!-- 1fa2a24c-271c-4a0d-81f8-29e9b344b8ab ec68d012-717b-4323-b1f1-a33083069bbd -->
# TUI公式リポジトリ統合計画

## 概要

OpenAI/codex公式リポジトリ（upstream/main）を上流として、最新のTUI機能を取り込みつつ、以下の独自機能を保持して統合します：

- GPU統計表示機能（`gpu_stats.rs`, Windows AI統合）
- カスタムプロンプト機能（`custom_prompt_view.rs`）
- 独自スラッシュコマンド（Delegate/Orchestrate/Research/Hook/Plan）
- custom_terminalモジュール（独自ターミナル実装）

## 実装ステップ

### Phase 1: 現状確認と差分分析

1. **upstream/mainの最新取得**

- `git fetch upstream main` で最新コミットを取得
- `git log upstream/main --oneline -- codex-rs/tui -30` で最新のTUI変更を確認

2. **独自機能の完全特定**

- `codex-rs/tui/src/gpu_stats.rs` - GPU統計表示（Windows AI統合）
- `codex-rs/tui/src/bottom_pane/custom_prompt_view.rs` - カスタムプロンプトUI
- `codex-rs/tui/src/custom_terminal.rs` - 独自ターミナル実装
- `codex-rs/tui/src/slash_command.rs` - Delegate/Orchestrate/Research/Hook/Planコマンド定義
- `codex-rs/tui/src/chatwidget.rs` - 独自コマンドのハンドリング（1573-1586行目）
- `codex-rs/tui/src/bottom_pane/mod.rs` - GPU統計統合（GpuStatsWidget）

3. **差分確認とコンフリクト予測**

- `git diff upstream/main -- codex-rs/tui` で全体差分を確認
- `git diff upstream/main -- codex-rs/tui/Cargo.toml` で依存関係の差分を確認
- コンフリクトが発生しそうなファイルを特定

### Phase 2: マージ実行（ours優先戦略）

1. **マージ実行**

- `git merge upstream/main -X ours` でours優先マージ
- これにより独自機能が優先的に保持される

2. **コンフリクト解決（予想されるファイル）**

- `codex-rs/tui/src/lib.rs` - メインライブラリ（custom_terminal統合確認）
- `codex-rs/tui/src/main.rs` - エントリーポイント（clapベース実装の確認）
- `codex-rs/tui/src/chatwidget.rs` - Delegateコマンドなど（独自ハンドリング保持）
- `codex-rs/tui/src/bottom_pane/mod.rs` - GPU統計統合（GpuStatsWidget保持）
- `codex-rs/tui/src/slash_command.rs` - スラッシュコマンド定義（独自コマンド保持）
- `codex-rs/tui/Cargo.toml` - 依存関係（codex-windows-aiなど独自依存関係確認）

3. **公式の新機能を手動統合**

- 公式の新機能を確認し、独自機能と競合しない範囲で統合
- 必要に応じて独自機能を公式の新APIに合わせて更新
- `custom_terminal.rs`が公式のratatui::Terminalと競合しないか確認

### Phase 3: 独自機能の統合確認

1. **GPU統計機能の確認**

- `gpu_stats.rs` が正しく統合されているか確認
- `bottom_pane/mod.rs` のGpuStatsWidgetが動作するか確認
- Windows AI統合（`codex-windows-ai`）が正しく動作するか確認
- `Cargo.toml`の`windows-ai` feature flagが維持されているか確認

2. **カスタムプロンプト機能の確認**

- `custom_prompt_view.rs` が正しく統合されているか確認
- `chat_composer.rs` のカスタムプロンプト処理（expand_custom_prompt）を確認
- `bottom_pane/mod.rs` のset_custom_promptsメソッドを確認

3. **独自スラッシュコマンドの確認**

- `slash_command.rs` のDelegate/Orchestrate/Research/Hook/Plan定義を確認
- `chatwidget.rs` の独自コマンドハンドリング（1573-1586行目）を確認
- CLIへのフォールバックメッセージが正しく表示されるか確認

4. **custom_terminalモジュールの確認**

- `custom_terminal.rs` が正しく統合されているか確認
- `tui.rs`, `lib.rs`, `resume_picker.rs` などでの使用箇所を確認
- 公式のratatui::Terminalとの競合がないか確認

### Phase 4: ビルドと依存関係更新

1. **依存関係の確認と更新**

- `codex-rs/tui/Cargo.toml` を確認
- 公式の新機能で追加された依存関係を確認
- 独自機能に必要な依存関係（`codex-windows-ai`など）が維持されているか確認
- バージョン不一致があれば更新

2. **ビルド実行**

- `cd codex-rs && cargo build --release -p codex-tui` でビルド
- ビルドエラーがあれば修正
- `windows-ai` feature flagが正しく機能するか確認

3. **リンター確認**

- `just fix -p codex-tui` でリンターエラーを修正
- `just fmt` でフォーマット
- Clippy警告があれば修正

### Phase 5: テスト実行

1. **ユニットテスト実行**

- `cargo test -p codex-tui` でテストを実行
- 失敗したテストがあれば修正
- スナップショットテスト（insta）の更新が必要か確認

2. **統合テスト確認**

- GPU統計機能のテスト
- カスタムプロンプト機能のテスト
- 独自スラッシュコマンドのテスト

### Phase 6: 起動確認と動作検証

1. **TUI起動確認**

- `codex tui` または `codex-tui` でTUIを起動
- 基本的な画面表示を確認

2. **機能確認**

- GPU統計表示が動作するか確認（Windows環境）
- カスタムプロンプト機能が動作するか確認
- Delegate/Orchestrate/Research/Hook/Planコマンドが適切にフォールバックするか確認
- 公式の新機能（最新のslash commandsなど）が動作するか確認

### Phase 7: 実装ログ作成

1. **ログ作成**

- `_docs/` に `yyyy-mm-dd_TUI公式マージ統合完了.md` を作成
- マージ内容、保持した独自機能、統合した公式機能、ビルド結果、起動確認結果を記録
- コンフリクト解決の詳細を記録

## 注意事項

1. **ours優先マージの影響**

- `-X ours` により、コンフリクト時は常にローカルの変更が優先される
- 公式の新機能は手動で統合する必要がある

2. **独自機能の保持**

- GPU統計機能はWindows AI統合（`codex-windows-ai`）に依存
- カスタムプロンプト機能は公式のカスタムプロンプト機能と統合が必要な可能性
- custom_terminalモジュールは公式のratatui::Terminalと競合しないか注意

3. **依存関係の管理**

- `codex-windows-ai` は独自機能に必要
- `windows-ai` feature flagが正しく設定されているか確認

4. **ビルドエラーの可能性**

- 公式の新機能でAPIが変更されている可能性
- 依存関係のバージョン不一致の可能性
- custom_terminalと公式のTerminal実装の競合

## 実装ファイル

- `codex-rs/tui/src/lib.rs` - メインライブラリ
- `codex-rs/tui/src/main.rs` - エントリーポイント
- `codex-rs/tui/src/chatwidget.rs` - チャットウィジェット（独自コマンド）
- `codex-rs/tui/src/bottom_pane/mod.rs` - ボトムペイン（GPU統計統合）
- `codex-rs/tui/src/gpu_stats.rs` - GPU統計機能
- `codex-rs/tui/src/bottom_pane/custom_prompt_view.rs` - カスタムプロンプトUI
- `codex-rs/tui/src/custom_terminal.rs` - 独自ターミナル実装
- `codex-rs/tui/src/slash_command.rs` - スラッシュコマンド定義
- `codex-rs/tui/Cargo.toml` - 依存関係
- `codex-rs/cli/src/main.rs` - TUI起動ロジック（`launch_tui`関数）

### To-dos

- [ ] upstream/mainの最新取得とTUI変更履歴の確認
- [ ] 独自機能の完全特定（gpu_stats, custom_prompt_view, custom_terminal, slash_commands）
- [ ] 差分確認とコンフリクト予測（git diff upstream/main -- codex-rs/tui）
- [ ] マージ実行（git merge upstream/main -X ours）
- [ ] コンフリクト解決（lib.rs, main.rs, chatwidget.rs, bottom_pane/mod.rs, slash_command.rs, Cargo.toml）
- [ ] 公式の新機能を手動統合（独自機能と競合しない範囲で）
- [ ] GPU統計機能の統合確認（gpu_stats.rs, bottom_pane/mod.rs, Windows AI統合）
- [ ] カスタムプロンプト機能の統合確認（custom_prompt_view.rs, chat_composer.rs）
- [ ] 独自スラッシュコマンドの統合確認（slash_command.rs, chatwidget.rs）
- [ ] custom_terminalモジュールの統合確認（custom_terminal.rs, 使用箇所の確認）
- [ ] 依存関係の確認と更新（Cargo.toml, codex-windows-ai, windows-ai feature）
- [ ] ビルド実行（cargo build --release -p codex-tui）
- [ ] リンター確認（just fix -p codex-tui, just fmt）
- [ ] テスト実行（cargo test -p codex-tui）
- [ ] TUI起動確認と機能検証（codex tui, GPU統計, カスタムプロンプト, 独自コマンド）
- [ ] 実装ログ作成（_docs/yyyy-mm-dd_TUI公式マージ統合完了.md）