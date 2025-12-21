---
name: release-2-7-0-align-skills-planmode
overview: 公式のSkills/Plan（docs/skills.md・docs/plan/*）に整合させつつ独自要素は互換の範囲で残し、全パッケージを2.7.0へ更新→高速差分ビルド→上書きインストール→GUITUICLI実機スモークテストまで行う。
todos:
  - id: audit-official-alignment
    content: docs/skills.md・docs/plan/* と現行実装（Rust core/CLI/GUI + sample plan skill）のギャップを列挙し、合わせ方針（docsを正）を確定する
    status: completed
  - id: bump-versions-2-7-0
    content: Rust workspace + kernel-extensions + 全 package.json の version を 2.7.0 に統一し、必要な lockfile/CHANGELOG を更新する
    status: completed
  - id: replace-plan-skill-helpers
    content: sample plan skill の Python scripts を削除し、Rust/TypeScript で置換（公式 Plan Mode 導線と整合）する
    status: completed
  - id: fast-incremental-build
    content: Windowsで高速差分ビルド（os error 5対策込み）を実行し、releaseバイナリを生成する
    status: in_progress
  - id: force-install-binaries
    content: cargo install --force で codex / codex-tui を上書きインストールし、2.7.0を確認する
    status: pending
  - id: full-smoke-test
    content: CLI+TUI+Node+GUI Next.js+Tauri の実機スモークテストを実施し、結果を整理する
    status: pending
  - id: write-impl-log
    content: MCPで現在日時を取得し、_docs に yyyy-mm-dd_release-2.7.0{worktreename}.md 形式で実装ログを残す
    status: pending
---

# 2.

7.0 リリース準備（公式Skills/Plan整合 + 置換ヘルパー + 高速差分ビルド + 実機テスト）

## ゴール

- **公式整合**:
- Skills: `docs/skills.md` と実装（`codex-rs/core/src/skills/*`）の前提に沿う
- Plan Mode: `docs/plan/*` を正として、CLI/GUI/保存先/Export の挙動と説明を揃える
- **独自機能の扱い**: 公式セマンティクス（保存先/コマンド/状態遷移）を壊さない範囲で残す
- **バージョン**: リポジトリ全体の SemVer を **2.7.0** に統一
- **ビルド/導入**: Windowsで高速差分ビルド → `cargo install --force` 等でバイナリ上書き → 実機スモーク
- **ログ**: 実装/テスト結果を `_docs/yyyy-mm-dd_機能名{worktreename}.md` で残す

## スコープ

- In:
- version bump（Cargo workspace / npm packages / extensions / sdk / prism / gui / tauri など）
- plan skill の Python ヘルパーを **Rust/TypeScript** に置換（Pythonは削除）しつつ、公式 Plan Mode 導線（`/Plan`, `/approve`, `/Plan export`）と矛盾しないように整備
- Plan Mode の保存先/Export先/命名の不整合があれば最小差分で統一
- Windowsでの高速差分ビルド手順（`CARGO_TARGET_DIR` 等）を確立し、上書きインストール
- GUITUICLI（CLI+TUI+Node+GUI Next.js+Tauri）実機スモーク
- Out:
- Plan/Skills の仕様自体を独自に変更する（docs/plan を正とする）

## 主要な現状（把握済み）

- Rust（`codex-rs/Cargo.toml`）は `workspace.package.version = "2.6.0"`、editionは workspaceで **2024**
- npm は複数 `package.json` に `"version": "2.6.0"` が存在
- Plan Mode 実装がRust側に存在（例: `codex-rs/core/src/plan/persist.rs`, `codex-rs/cli/src/plan_commands.rs`, `codex-rs/tauri-gui/src/pages/Plans.tsx`）
- sample plan skill は `codex-rs/core/src/skills/assets/samples/plan/SKILL.md` で、Pythonスクリプト前提の記述がある

## 実装方針（公式優先）

- docs を正:
- Plan Mode: `docs/plan/README.md` / `docs/plan/slash-commands.md` の CLI 例・保存先・Export先に実装を寄せる
- Skills: `docs/skills.md` のスコープ/ロード仕様に合わせる
- 互換性:
- 既存の Plan persister/export（`docs/Plans`, `logs/Plan`）は公式docsと齟齬が出ないように整理
- GUI(Tauri)のPlan画面とCLIのPlan操作が同じ状態遷移/用語を使う

## 変更対象（候補）

- Rust:
- `codex-rs/Cargo.toml`（`[workspace.package].version`）
- `codex-rs/Cargo.lock`（更新）
- `kernel-extensions/**/Cargo.toml`（2.7.0へ）
- Plan/skills関連の必要最小限の修正（保存先/命名の統一など）
- Node/TS:
- ルート `package.json`
- `codex-cli/package.json` + lock
- `gui/package.json`, `codex-rs/tauri-gui/package.json`, `extensions/**/package.json`, `sdk/**/package.json`, `prism-*/package.json` ほか（2.7.0へ）
- plan skill の `scripts/*.py` を削除し、TypeScriptヘルパーを追加（必要ならビルド済みJSを同梱してNode無しでも使える形にする）

## 手順

### 1) 公式仕様とのギャップ洗い出し（read-only）

- `docs/plan/*` と現行実装（CLI/GUI/コア）で、保存先・Export先・コマンド表記の差分を列挙
- `docs/skills.md` と skills loader の挙動（repo/user/system/adminの優先順位、frontmatter制約）を確認

### 2) version 2.7.0 への統一

- Rust:
- `codex-rs/Cargo.toml` の workspace version を 2.7.0
- kernel extensions も 2.7.0
- Node/TS:
- 全 `package.json` の version を 2.7.0
- lockfiles を更新（pnpm/npm の運用に合わせる）
- ドキュメント:
- `CHANGELOG.md` など、バージョン記載がある箇所を整合

### 3) plan skill の公式整合 + Python排除

- `codex-rs/core/src/skills/assets/samples/plan/SKILL.md` から Python導線を削除し、公式 Plan Mode の導線へ置換
- `scripts/*.py` は削除
- 置換ヘルパー（TypeScript/Rust）を追加し、以下を提供:
- Plan一覧/検索（frontmatter要約）
- Plan frontmatter 読み取り
- Plan作成（テンプレ生成 + overwrite）
- ただし保存先は docs/plan の記述と一致させる

### 4) 高速差分ビルド（Windows）

- `CARGO_TARGET_DIR` を固定（高速差分）
- 既知の Windows `os error 5`（ファイルロック）回避のための手順を組み込み:
- 競合プロセス（`codex.exe`, `codex-tui.exe`）の停止
- 生成物の一時退避

/クリーン（必要時）

- `RUSTC_WRAPPER`（sccache）周りの取り扱い整理
- `cargo build -p codex -p codex-tui --release` 相当を実行

### 5) バイナリ上書きインストール

- `cargo install --path codex-rs/cli --bin codex --force`
- `cargo install --path codex-rs/tui --bin codex-tui --force`
- `where codex; codex --version` / `where codex-tui; codex-tui --version` で 2.7.0 を確認

### 6) GUITUICLI 実機スモークテスト（フル）

- CLI:
- 起動、`/Plan on`, `/Plan "..."`, `/Plan export`, `/approve`, 実行導線
- TUI:
- 起動、入力、スラッシュコマンド（`/skills` 含む）
- Node(codex-cli):
- `node codex-cli/bin/codex.js --version` / `--help` が Rustバイナリを見つけて動作
- GUI Next.js（`gui/`）:
- `npm ci` → `npm run dev` → localhost疎通
- Tauri（`codex-rs/tauri-gui`）:
- `npm ci` → `npm run tauri:dev` 起動確認

### 7) 実装ログの作成

- MCPで現在日時を取得（可能なら）し、
- `_docs/yyyy-mm-dd_release-2.7.0{merge-upstream-2025-12-20}.md` のように保存
- 実行コマンド、結果、エラー、対処、次の課題を簡潔に記載

## 受け入れ条件

- 主要バージョン表記が 2.7.0 で揃っている（Rust workspace + npm packages）
- `codex --version` / `codex-tui --version` が 2.7.0
- フル実機スモーク（CLI+TUI+Node+GUI+Tauri）で致命的ブロック無し
- plan skill のドキュメントが Python 非依存かつ `docs/plan` と矛盾しない

## リスク

- Windows `os error 5` が再発する可能性（AV/ファイルロック）
- lockfile 更新で差分が大きくなる
- docs と実装の差分が大きい場合、最小差分で合わせるために「docs側の補正」が必要になる