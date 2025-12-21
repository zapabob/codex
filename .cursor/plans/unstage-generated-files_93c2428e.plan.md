---
name: unstage-generated-files
overview: ステージングされた生成物/ログ系ファイルだけをstagedから外し、ソース変更など必要なステージングは維持します（作業ツリーは変更しません）。
todos:
  - id: unstage-check
    content: staged一覧を取得して不要系を特定
    status: completed
  - id: unstage-run
    content: 生成物/ログ系パスをまとめてgit restore --stagedで解除
    status: completed
  - id: unstage-verify
    content: git status / git diff --cachedで意図どおりになったか確認
    status: completed
---

# ステージング不要ファイルの除外（生成物/ログ系のみ）

## ゴール

- 生成物/ログ系（例: `codex-rs/target_install_verify/**`, `build_err*.txt`, `.cursor/plans/**`, `.specstory/**`, `_docs/**`）だけを **staged から外す**
- 作業ツリー（ファイル内容）は **一切消さない/戻さない**

## 実行手順

- **現状確認**
- `git status -sb`
- `git diff --cached --name-only`
- **生成物/ログ系をまとめて unstage**（日本語パス指定を避けるため、ディレクトリ単位で外す）
- 次を実行（PowerShell; コマンド自体はASCIIのみ）:
    - `git restore --staged -- .cursor/plans .specstory _docs codex-rs/.specstory codex-rs/target_install_verify build_err.txt build_errors.txt codex-rs/build_err.txt`
- **結果確認**
- `git diff --cached --name-only` から上記の系統が消えていること
- `git status -sb` で staged が意図どおり（ソース変更のみ等）になっていること
- **取りこぼしがあれば追加で unstage**
- `git diff --cached --name-only` を見て、残っている不要パスを `git restore --staged --

<path>` で外す

## 変更対象（この操作でstagedから外す想定）

- `.cursor/plans/**`
- `.specstory/**` と `codex-rs/.specstory/**`
- `_docs/**`
- `codex-rs/target_install_verify/**`
- `build_err.txt`, `build_errors.txt`, `codex-rs/build_err.txt`

## 任意の追加対応（必要なら）

- これらを今後ステージしないようにするための `.gitignore` 追記（要望があれば別途）