<!-- 3d46898b-2542-4ccb-9374-710916d9c679 be279566-81e2-499e-b75a-8f43fe0186f2 -->
# Rust 2024警告0高速差分ビルド強制インストール計画

## 問題の概要

- 3つのクレートが`edition = "2021"`のまま（`windows-sandbox-rs`, `utils/pty`, `gui`）
- 型定義の確認と修正が必要
- 警告0を達成する必要がある
- 機能を維持しつつ高速差分ビルドと強制インストールを実行

## 実装手順

### 1. Rust 2024 editionへの統一

**ファイル**: `codex-rs/windows-sandbox-rs/Cargo.toml`, `codex-rs/utils/pty/Cargo.toml`, `codex-rs/gui/Cargo.toml`

**変更内容**:

- `edition = "2021"`を`edition = "2024"`に更新
- 互換性を確認（Rust 2024 editionは後方互換性があるため、通常は問題なし）

### 2. 型定義の確認と修正

**確認対象**:

- `codex-rs/tui/src/bottom_pane/mod.rs`: `Renderable`トレイトの実装確認
- `codex-rs/tui/src/chatwidget.rs`: `layout_areas`の戻り値型確認
- すべてのクレートで型エラーがないか確認

**修正方法**:

- `cargo check --release -p codex-cli -p codex-tui`で型エラーを検出
- エラーがあれば修正

### 3. 警告0の達成

**確認方法**:

- `cargo clippy --release -p codex-cli -p codex-tui -- -W clippy::all`で警告を検出
- 未使用インポート、未使用変数、その他の警告を修正

**修正対象**:

- 未使用の`use`文を削除
- 未使用変数に`_`プレフィックスを付与
- Clippyの警告に従ってコードを改善

### 4. 高速差分ビルドの実行

**ファイル**: `codex-rs/fast-build-install.ps1`（既存スクリプトを使用）

**実行内容**:

- `cargo clean`をスキップ（差分ビルド）
- `cargo build --release -p codex-cli -p codex-tui`を実行
- tqdm風の進捗表示で可視化
- エラー数と警告数を集計

### 5. 強制インストール

**実行内容**:

- 起動中のcodexプロセスを停止
- `cargo install --path cli --force`でCLIを上書きインストール
- 必要に応じてTUIもインストール

### 6. 動作確認

**確認項目**:

- `codex --version`でインストール確認
- エラー数: 0
- 警告数: 0
- ビルド時間の表示

## 期待される結果

- Rust 2024 editionに統一
- 型定義が正しく、エラー0件
- 警告0件
- 機能が維持されている
- CLI/TUIの強制インストール成功
- tqdm風の可視化で進捗が分かりやすい

### To-dos

- [ ] 3つのクレート（windows-sandbox-rs, utils/pty, gui）のeditionを2021から2024に更新
- [ ] cargo checkで型エラーを確認し、必要に応じて修正
- [ ] cargo clippyで警告を確認し、警告0を達成するよう修正
- [ ] fast-build-install.ps1を実行して高速差分ビルド（エラー0・警告0を確認）
- [ ] cargo install --forceでCLI/TUIを上書き強制インストール
- [ ] codex --versionでインストール確認と最終統計表示