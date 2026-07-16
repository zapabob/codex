# 2026-06-25 upstream sync v3.3.0 implementation log

## 基本情報

- 日時: 2026-06-25
- ブランチ: codex/upstream-main-2026-06-25
- 作業ツリー: C:\Users\downl\Desktop\codex-main
- 機能名: upstream sync v3.3.0

## 実施内容

- 公式 upstream を取得し、`rust-v0.142.2` を公式安定版の取り込み基準にした。
- `scripts/upstream_overlay_merge.py` で `rust-v0.133.0` から `rust-v0.142.2` へのPythonベース overlay merge を実行した。
- `VERSION` と `version-metadata.json` を `3.3.0` に更新し、`node scripts\sync-version.mjs` で README と関連バージョン表記を同期した。
- README を v3.3.0 向けに更新し、公式機能、バグ修正、脆弱性・依存更新、4コアWindowsビルド手順、APPを除外する上書きインストール手順を現在の同期内容へ合わせた。
- `codex-rs\app-server-protocol\src\protocol\common.rs` の壊れた Git4D API 断片を修復し、Git4D request / notification / experimental test を公式macro構造へ戻した。
- `codex-rs\app-server\src\lib.rs` の壊れた末尾断片を削除し、`git4d_bridge` module宣言をmodule一覧へ戻した。
- `codex-rs\app-server\tests\suite\v2\plugin_read.rs` の壊れた末尾断片を削除した。
- `codex-rs\core\src\shell_detect.rs` をフォーク側の独自機能として復元した。

## 判断

- 公式に同等機能がある箇所は公式側の最新API形状を優先し、Git4Dなど残すべき独自機能は公式のapp-server protocol構造へ追従させた。
- 4コア差分高速ビルドは `cargo +1.95.0 build --release -p codex-cli -j 4` を使用し、`RUSTUP_HOME`、`CARGO_HOME`、`CARGO_TARGET_DIR`、`TEMP`、`TMP` をHドライブへ逃がしてCドライブ容量不足を回避した。
- release binary が生成されていないため、プロセスkillと `C:\Users\downl\.cargo\bin\codex.exe` への上書きインストールは実行していない。

## 動作確認

### Python overlay merge

- 実装状況: 完了
- 動作確認: `scripts/upstream_overlay_merge.py --apply --allow-dirty` が conflict なしで overlay report を出力した。
- 確認日時: 2026-06-25
- 備考: レポートは `_docs/upstream-overlay-merge-report.md` と `_docs/upstream-overlay-merge-report.json`。

### バージョン同期

- 実装状況: 完了
- 動作確認: `node scripts\sync-version.mjs --check` が `Version artifacts already synced at v3.3.0` で成功した。
- 確認日時: 2026-06-25
- 備考: `scripts\sync-version.mjs` は現在のREADME構造に合わせて更新した。

### フォーマット

- 実装状況: 部分完了
- 動作確認: `cargo +1.95.0 fmt --all` は一度 `common.rs`、`lib.rs`、`plugin_read.rs` の構文崩れを検出した後、修復後に成功した。
- 確認日時: 2026-06-25
- 備考: その後のビルドで `codex-core` の追加統合エラーが判明した。

### 4コアrelease build

- 実装状況: 未完了
- 動作確認: `cargo +1.95.0 build --release -p codex-cli -j 4` は `codex-core` まで進み、未解決importと重複moduleで停止した。
- 確認日時: 2026-06-25
- 備考: 直近の主な残エラーは `review_prompts` 重複、`GoalContext` / `goal_spec` / `SkillsManager` / `AgentsMdManager` / `EventPersistenceMode` / `git2` / `codex_utils_template` など、フォーク独自core機能と公式最新core構成の再接続。

### diff hygiene

- 実装状況: 未完了
- 動作確認: `git diff --check` は複数ファイルで `new blank line at EOF` を検出した。
- 確認日時: 2026-06-25
- 備考: 大量の公式取り込みファイルにまたがるため、次回は機械的に末尾空行を整理してから再実行する。

### 上書きインストール

- 実装状況: 未実行
- 動作確認: release binary 未生成のため、APP除外のプロセスkillとバイナリ上書きは未実行。
- 確認日時: 2026-06-25
- 備考: ビルド成功後に `scripts\install_with_kill.ps1` で実施する想定。
