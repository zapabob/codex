# 実装ログ: Upstream Sync v2.16.0 → v2.17.0

**日付**: 2026-02-20  
**ブランチ**: `feature/upstream-sync-v2.17.0`  
**担当**: zapabob (Cursor AI Agent)  
**ステータス**: ✅ 完了

---

## 概要

`zapabob/Codex` フォークを `openai/codex` 上流リポジトリの最新コミットへ同期。  
セマンティクスバージョンを `2.16.0` → `2.17.0` に引き上げ。  
独自機能 (deep-research, supervisor, remote-image-urls 等) を保護しつつ、上流APIの変更にリファクタリング対応。

---

## 作業内容

### 1. バージョンバンプ

| ファイル | 変更内容 |
|----------|---------|
| `codex-rs/Cargo.toml` | `version = "2.16.0"` → `"2.17.0"` |
| `package.json` | `"version": "2.16.0"` → `"2.17.0"` |

### 2. マージコンフリクト解決

`scripts/resolve_conflicts.py` を作成して自動解決。  
解決戦略:
- `ZAPABOB_DIRS` (codex-gui-x, _docs, docs/zapabob, etc.) → **ours**を採用
- `.github/workflows/*` → **theirs** (upstream) を採用
- `codex-rs/core/Cargo.toml` → 両方をマージ、zapabob依存を保持
- `docs/zapabob/AGENTS.md` → ours をベースに theirs のユニーク行を追記

### 3. Rust コンパイルエラー修正 (codex-core)

| エラー | 原因 | 修正 |
|--------|------|------|
| `E0061` `AsyncManagedClient::new` | `codex_apps_tools_cache_context` パラメータ追加 | `None` を追加 |
| `E0433` `CancelErr::Cancelled` | `CancelErr` がenumからstructへ変更 | `Err(_cancelled)` に修正 |
| `E0425` `find_model_info_for_slug` | `model_info_from_slug` にリネーム | 全置換 |
| `E0061` `ToolRouter::from_config` | `app_tools` パラメータ追加 | `None` を追加 |
| `E0061` `run_sampling_request/run_model_turn` | シグネチャ変更 | 引数調整 |
| UTF-8エラー / bare CR | PowerShellのSet-Content使用による文字化け | `scripts/fix_runtime_encoding.py` + `scripts/fix_crlf.py` で修復 |
| `askama` ワークスペース継承エラー | 未定義の依存を継承 | `core/Cargo.toml` から削除 |

### 4. Rust コンパイルエラー修正 (codex-supervisor)

| エラー | 原因 | 修正 |
|--------|------|------|
| `E0061` `ModelsManager::new` | `model_catalog` パラメータ追加 | `None` を追加 |
| `E0061` `ThreadManager::new` | `model_catalog` パラメータ追加 | `None` を追加 |

### 5. Rust コンパイルエラー修正 (codex-tui)

| エラー | 原因 | 修正 |
|--------|------|------|
| `E0761` `history_cell` | flat file と directory module が競合 | `history_cell/` ディレクトリを `git rm -r` |
| `E0255` feedback functions | `view.rs` と `utils.rs` の重複定義 | `scripts/fix_view_rs.py` で重複削除 |
| `E0432` `popup_consts` import | `super::` ではなく `crate::bottom_pane::` が正しいパス | import パス修正 |
| `E0592` `input_height` 重複 | `impl FeedbackNoteView` が2つ定義 | 重複ブロック削除 |
| `E0004` `FeedbackCategory::SafetyCheck` | match非網羅的パターン | `utils.rs` に `SafetyCheck` ハンドリング追加 |
| `E0255/E0252` `UserMessage` 重複 | `chatwidget.rs` とサブモジュール両方に定義 | `scripts/fix_chatwidget.py` で重複削除 |
| `E0308` `runtime_metrics_label` | `&RuntimeMetricsSummary` → owned value | `&` を除去 |
| `E0061` `new_session_info` | `auth_plan` パラメータ追加 | `None` を追加 |
| `E0061` `new_active_web_search_call` | `animations_enabled` パラメータ追加 | `self.config.animations` を追加 |
| `E0063` `UnifiedExecProcessDetails` | `recent_chunks` フィールド欠損 | `recent_chunks: Vec::new()` を追加 |
| `E0609` `animations_enabled` on `StatusIndicatorWidget` | フィールドが廃止 | `if self.animations_enabled` ブロックを削除し `frame_requester` 直呼びに変更 |
| `E0061` `spinner()` 引数2個 | `spinner()` が引数1個に変更 | `self.animations_enabled` 引数を削除 |

### 6. ワークスペースビルド最終確認

```
cargo check --workspace → Exit: 0 (エラーゼロ)
cargo check -p codex-tui → Exit: 0 (エラーゼロ)
```

---

## 保護した独自機能 (zapabob extensions)

| 機能 | ファイル | 状態 |
|------|---------|------|
| Deep Research | `codex-rs/deep-research/` | ✅ 保護済 |
| Supervisor | `codex-rs/supervisor/` | ✅ 保護済 |
| Remote Image URLs | `chatwidget/user_message.rs` | ✅ マージ済 |
| Git4D Feature Gates | `codex-rs/Cargo.toml` features | ✅ 保護済 |
| Web Search | `deep-research/` | ✅ 保護済 |
| codex-gui-x | `codex-gui-x/` | ✅ 保護済 |

---

## 作成したユーティリティスクリプト

| スクリプト | 用途 |
|-----------|------|
| `scripts/resolve_conflicts.py` | Gitマージコンフリクト自動解決 |
| `scripts/fix_runtime_encoding.py` | `runtime.rs` UTF-8エンコーディング修復 |
| `scripts/fix_crlf.py` | Rustソースのビン行終端正規化 |
| `scripts/fix_view_rs.py` | `feedback/view.rs` 重複関数削除 |
| `scripts/fix_chatwidget.py` | `chatwidget.rs` 重複型/関数定義削除 |

---

## 今後の作業

- [ ] `cargo nextest run --no-fail-fast` でテスト実行
- [ ] pnpm audit で Node.js 依存関係のCVEチェック
- [ ] 変更をコミットして `main` にマージ
- [ ] GUI (`gui/`) 側のビルドチェック
