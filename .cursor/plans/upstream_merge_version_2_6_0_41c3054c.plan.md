---
name: upstream_merge_version_2_6_0
overview: リポジトリ内のセマンティクスバージョンを 2.6.0 に統一しつつ、upstream取り込み方針（同等機能は公式へ寄せ、無ければ独自を最小差分で維持）で差分を整理し、最終的に `codex-tui` を release ビルド→上書きインストール→入力/スラッシュコマンドを実機確認し、`_docs/` に実装ログを残す。
todos: []
---

# Upstream統合 + バージョン2.6.0統一 + codex-tui実機確認

## スコープ（今回確定）

- **バージョン統一**: リポジトリ内の可能な限り全 `package.json` と、Rust側は **`codex-rs` workspace** を含めて **2.6.0** に統一する
- **動作保証/実機確認**: **`codex-rs` の `codex-tui` / `codex`(CLI) のみ**（DeepResearch/GUI/kernel/malware は“削らず保持 + バージョン更新”までで、今回ビルド保証対象外）

## 重要な前提（安全/ポリシー）

- **AV回避で“アンチ

ウイルスをkillする/無効化する”手順は行わない**。

- 代替策として、**ターゲットディレクトリ分離**・**ビルド並列数抑制**・**ロック回避の待機**・（許可される範囲での）**開発用ビルドディレクトリ除外**を用いる。

## 作業手順

### 0) Gitの状態確認と保全

- `git status` で作業ツリーが汚れていないか確認。
- 汚れている場合は `git stash push` か WIPコミットで退避。

### 1) バージョン 2.6.0 へ統一（リポジトリ全体）

#### 1-a) npm系（全 `package.json`）

- 既に見えている `package.json`（例）:
- [`package.json`](package.json)
- [`codex-cli/package.json`](codex-cli/package.json)
- [`sdk/package.json`](sdk/package.json)
- [`sdk/typescript/package.json`](sdk/typescript/package.json)
- [`gui/package.json`](gui/package.json)
- [`gui-tests/package.json`](gui-tests/package.json)
- [`extensions/*/package.json`](extensions/)
- [`codex-rs/tauri-gui/package.json`](codex-rs/tauri-gui/package.json)
- [`codex-rs/mcp-server/package.json`](codex-rs/mcp-server/package.json)
- [`codex-rs/responses-api-proxy/npm/package.json`](codex-rs/responses-api-proxy/npm/package.json)
- ほか `**/package.json` を列挙し、**全ての `"version"` を `2.6.0`** に更新。
- あわせて、内部依存が **`@zapabob/*` の固定バージョン**を持つ場合は `2.6.0` に揃える（存在する場合のみ）。

#### 1-b) Rust系

- `codex-rs` の workspace version を更新:
- [`codex-rs/Cargo.toml`](codex-rs/Cargo.toml) の `[workspace.package] version = "0.0.0" `を **`"2.6.0"`** に。
- `version.workspace = true` の全crate（例: [`codex-rs/cli/Cargo.toml`](codex-rs/cli/Cargo.toml), [`codex-rs/tui/Cargo.toml`](codex-rs/tui/Cargo.toml)）は自動的に 2.6.0 になる。
- `codex-rs` 外のRust crate（例: [`kernel-extensions/**/Cargo.toml`](kernel-extensions/) など）で `version.workspace` を使っていないものは、個別に `[package] version `を `2.6.0` に更新。

#### 1-c) ドキュメント/チェンジログ

- リリース表記がある場合（例: [`CHANGELOG.md`](CHANGELOG.md)）は、2.6.0 に整合するよう更新（必要箇所のみ）。

### 2) upstream最新機能（skill/plan等）を「公式優先」で整合（差分最小化）

- `codex-tui` / `codex-cli` のビルドに影響する範囲で、以下を点検:
- TUI入力/スラッシュコマンド: [`codex-rs/tui/src/bottom_pane/chat_composer.rs`](codex-rs/tui/src/bottom_pane/chat_composer.rs) など
- plan系: [`codex-rs/cli/src/plan_commands.rs`](codex-rs/cli/src/plan_commands.rs) / [`codex-rs/protocol/src/plan_tool.rs`](codex-rs/protocol/src/plan_tool.rs)
- skill系: `skill` 名称のモジュール/コマンドが upstream 側に存在する場合は **upstream実装に寄せる**。独自側のみなら **最小差分で維持**。
- 既に露出している不整合（例: `ReasoningEffort` importの位置）は upstream 流儀に合わせて修正。
- 例: [`codex-rs/app-server-protocol/src/protocol/v1.rs`](codex-rs/app-server-protocol/src/protocol/v1.rs) の `ReasoningEffort` は `codex_protocol::openai_models::ReasoningEffort` が実体。

### 3) 高速差分ビルド（Windowsのos error 5対策込み）で `codex-tui` を通す

- **固定短パスのターゲットDir**（例: `C:\Users\downl\.cargo-target\codex`）を採用。
- 推奨環境変数（PowerShell想定）:
- `CARGO_TARGET_DIR` を短い固定パスへ
- `CARGO_PROFILE_RELEASE_INCREMENTAL=true`
- `CARGO_BUILD_JOBS=1`
- 追加のロック回避:
- ビルド前に `Start-Sleep -Seconds 20` を入れる（既知の `windows_*` build-script 実行拒否を回避）
- それでも失敗する場合の安全手順:
- **自分の `cargo` / `rustc` だけ**を終了して再実行（AVプロセスは触らない）
- ターゲットDirを一時的に切り替えて再実行
- （許可される場合のみ）開発用ターゲットDirを Defender 除外に追加

### 4) フォーマット/静的チェック/テスト（`codex-tui` 中心）

- Rust変更後:
- 可能なら `just fmt`（`codex-rs`）
- もしWindows環境で `just` が `sh` 不在で失敗するなら `cargo fmt` にフォールバック
- 仕上げ前:
- `just fix -p codex-tui`（必要なら `-p codex-core` も）
- テスト:
- `cargo test -p codex-tui --lib bottom_pane::chat_composer`
- 余力があれば `cargo test -p codex-tui`

### 5) 上書きインストール → 実機確認

- `cargo install --path tui --bin codex-tui --force`
- `where codex-tui` と `codex-tui --version` で差し替え確認（2.6.0が出ること）。
- 手動確認:
- 入力が即時反映
- `"/"` でスラッシュコマンド候補表示
- `/model` 等が実行できる

### 6) 実装ログ（`_docs/`）

- ファイル名: `yyyy-mm-dd_TUI入力スラッシュコマンド{worktreename}.md`
- 記録内容:
- 取り込んだ upstream 範囲
- 解消した競合
- 「置換」した独自機能 / 「維持」した独自機能（今回触った範囲）
- 2.6.0 統一で変更した対象一覧
- 実行コマンド、ビルド/テスト/実機確認結果