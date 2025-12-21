---
name: guituicli_smoke_test_v2_6_0_with_unstage
overview: v2.6.0（merge-upstream-2025-12-20）でGUI（Next.js+Tauri）/TUI（codex-tui）/CLI（Rust codex + Node codex-cli）をWindows上でビルド・起動し、最低限の実機スモークテストとバージョン確認、結果ログ（_docs/）作成まで行う。開始前にGitのステージングにある不要ファイルを除外（unstage）する。
todos: []
---

# GUITUICLI実機テスト（v2.6.0 / merge-upstream-2025-12-20）

## 前提と方針

- 対象
- TUI: `codex-rs/tui`（`codex-tui`）
- Rust CLI: `codex-rs/cli`（bin: `codex`）
- Node CLI: `codex-cli`（`node codex-cli/bin/codex.js`）
- GUI (Next.js): `gui`（`next dev/build/start`）
- GUI (Tauri): `codex-rs/tauri-gui`（`npm run tauri:dev/tauri:build`）
- Windows os error 5対策: 短パスの`CARGO_TARGET_DIR`、`CARGO_BUILD_JOBS=1`、ビルド前待機（`Start-Sleep`）を標準化。
- ディレクトリ移動の事故防止: Rustは`--manifest-path`、npmは`--prefix`で実行。
- 依存導入: `gui/`・`codex-cli/`・`codex-rs/tauri-gui/`はいずれも`package-lock.json`があるため、基本は`npm ci`を優先。

## 0) Git: ステージングの不要ファイル除外（unstage）

- 確認
- `git status

`

- `git diff --cached --name-only`
- 方針
- 今回の実機テスト自体はコミット不要なので、原則として「余分にステージされているもの」は作業ツリーは触らずにステージだけ外す。
- 実行例
- 個別に外す: `git restore --staged <path>`
- 全部外す: `git restore --staged .`
- 最後に再確認
- `git status`で意図しない staged が残っていないこと

## 1) 環境確認（1回だけ）

- PowerShellで以下を確認
- `node -v` / `npm -v` / `rustc -V` / `cargo -V`
- 高速差分ビルド用ターゲットを作成
- `C:\Users\downl\.cargo-target\codex`

## 2) Rust: TUI/CLIビルド（高速差分）

- 環境変数（同一PowerShellセッション内で設定）
- `CARGO_TARGET_DIR=C:\Users\downl\.cargo-target\codex`
- `CARGO_PROFILE_RELEASE_INCREMENTAL=true`
- `CARGO_BUILD_JOBS=1`
- ビルド前に待機（ロック回避）
- `Start-Sleep -Seconds 20`
- ビルド（TUI/CLI）
- `cargo build --manifest-path codex-rs/Cargo.toml -p codex-tui --release`
- `cargo build --manifest-path codex-rs/Cargo.toml -p codex-cli --release`
- tqdm風の可視化が必要な場合
- `py -3 codex-rs/build_with_progress.py`（事前に上記環境変数をセットしてから）

## 3) Rust: インストール & 実機確認（TUI/CLI）

- 上書きインストール
- `cargo install --path codex-rs/tui --bin codex-tui --force`
- `cargo install --path codex-rs/cli --bin codex --force`
- 配置確認
- `where codex-tui` / `codex-tui --version`
- `where codex` / `codex --version`
- TUI手動スモーク（必須）
- 入力が即時反映
- `/`でスラッシュコマンド候補
- `/model`等が実行できる

## 4) Node: codex-cli（JS）実機確認

- 依存導入（必要な場合）
- `npm --prefix codex-cli ci`
- 実行（PATH衝突回避のためnode直叩き）
- `node codex-cli/bin/codex.js --version`
- `node codex-cli/bin/codex.js --help`

## 5) GUI: Next.js（gui/）実機確認

- 依存導入（必要な場合）
- `npm --prefix gui ci`
- ビルド＆起動（いずれか）
- 開発: `npm --prefix gui run dev`
- 本番想定: `npm --prefix gui run build` → `npm --prefix gui run start`
- スモーク観点
- 起動後にトップ/主要ページ（例: `/`, `/agents`, `/mcp`）が描画される
- コンソールに致命的エラーが出ない

## 6) GUI: Tauri（codex-rs/tauri-gui）実機確認

- 依存導入（必要な場合）
- `npm --prefix codex-rs/tauri-gui ci`
- 起動
- `npm --prefix codex-rs/tauri-gui run tauri:dev`
- 失敗時の参照
- `codex-rs/tauri-gui/INSTALLATION.md`
- `codex-rs/tauri-gui/QUICK_START.md`
- スモーク観点
- アプリウィンドウが起動し、主要画面が描画される
- 設定/ダッシュボードなど基本遷移が可能

## 7) 結果ログ（_docs/）

- ファイル名
- `_docs/2025-12-19_TUI入力スラッシュコマンド{merge-upstream-2025-12-20}.md`
- 記載
- 実行コマンド（PowerShell）
- ビルド/テスト/起動結果（成功・失敗、エラー全文）
- `where`と`--version`の結果
- 手動スモークの確認項目と結果

## 8) 終了合図（音）

- 既存のPowerShellスクリプトで指定wavを再生する（例: `codex-rs/tauri-gui/play-sound.ps1` または `docs/sound-notification/`配下）。