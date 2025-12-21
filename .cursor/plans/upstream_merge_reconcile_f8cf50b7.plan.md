---
name: upstream_merge_reconcile
overview: fork(origin/main)に公式(upstream/main)の新機能/脆弱性修正/バグ修正を取り込みつつ、独自機能は「公式に同等機能が入ったら置き換え、無ければ維持」の方針でマージコンフリクトを解消し、最後にcodex-tuiをビルド→上書きインストール→実機確認→実装ログを残す。
todos:
  - id: sync-upstream-merge
    content: 作業ブランチで upstream/main を merge し、コンフリクトを解消する
    status: completed
  - id: reconcile-custom-features
    content: 独自機能を upstream 同等機能へ置換/維持に仕分けし、差分を最小化する
    status: in_progress
  - id: build-install-test-tui
    content: codex-tui を release でビルド→上書きインストール→入力/スラッシュコマンドを実機確認する
    status: pending
  - id: write-impl-log
    content: _docs/ に所定フォーマットで実装ログを残す
    status: pending
---

## 方針（今回の回答に基づく）

- **統合方式**: `merge`（`upstream/main` を取り込むマージコミット）
- **作業場所**: **作業ブランチ**で統合作業 → 最後に `main` へ反映
- **独自機能の扱い**: 
- upstream に **同等機能が入っている**: upstream 実装へ寄せ、独自差分は削除/縮小
- upstream に **同等機能が無い**: 独自実装を維持しつつ、upstream の設計に合わせて最小差分で乗せ直す

## 事前確認（安全策）

- 作業ツリーが汚れている可能性があるので、まず現状を保全する
- `git status` で変更の有無を確認
- 未コミットがあれば、作業ブランチ作成前に `git stash push` か「WIPコミット」で退避

## 1) upstream の最新を取り込む（作業ブランチ）

- `git fetch upstream` と `git fetch origin` で参照を最新化
- `main` から作業ブランチ作成（例: `merge-upstream-2025-12-20`）
- `upstream/main` を `merge --no-ff` で取り込み
- 競合が出たら、ファイル単位で以下の優先順位で解消
- **A: upstream の新仕様/セキュリティ修正が優先**
- **B: 既存の独自機能は“必要な最小差分”で再適用**

## 2) 独自機能 vs upstream 同等機能の仕分け（差分を削る）

- `upstream/main..origin/main`（自分らだけのコミット）を一覧化し、テーマ別に分類
- TUI入力/スラッシュコマンド（例: `codex-rs/tui/src/bottom_pane/chat_composer.rs`）
- Windows sandbox/exec（例: `codex-rs/core/src/exec.rs`, `codex-rs/windows-sandbox-rs/src/lib.rs`）
- モデル/設定周り（例: `codex-rs/core/src/*model*`）
- CI/GUI/テスト（例: `.github/workflows/*`, `gui-tests/*` など）
- upstream 側に同等機能があるか、該当箇所を確認して以下のいずれかで決着
- **置換**: upstream 実装へ寄せ、独自差分を削除
- **維持**: upstream の流儀に合わせて独自機能を再適用

## 3) ビルドを通す（codex-tui をゴールに合わせる）

- Windows で詰まりやすいので、作業中は次を前提にする
- `CARGO_TARGET_DIR` を短い固定パスへ
- `CARGO_PROFILE_RELEASE_INCREMENTAL=true`
- `CARGO_BUILD_JOBS=1`（os error 5 の回避）
- `cargo build -p codex-tui --release --bin codex-tui` を通し、エラーは上から順に修正

## 4) フォーマット/静的チェック/テスト

- Rust変更が入ったら `just fmt`（`codex-rs`）
- 仕上げ前に `just fix -p codex-tui`（必要なら `-p codex-core` も）
- テスト
- `cargo test -p codex-tui --lib bottom_pane::chat_composer`
- 余力があれば `cargo test -p codex-tui`
- 共通/コアを触った場合は（確認を挟んで）`cargo test --all-features`

## 5) 上書きインストール → 実機確認

- `cargo install --path tui --bin codex-tui --force`
- `where codex-tui` と `codex-tui --version` で差し替え確認
- 手動確認
- 入力が即時反映される
- `"/"` でスラッシュコマンド候補が出る
- `/model` 等が実行できる

## 6) 実装ログ

- `_docs/` に `yyyy-mm-dd_TUI入力スラッシュコマンド{worktreename}.md` を作成し、
- 取り込んだ upstream 範囲
- 解消した競合
- 「置換」した独自機能 / 「維持」した独自機能
- 実行したコマンド、ビルド/テスト結果

を記録する