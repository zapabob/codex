# CI/CD 修正実装ログ - 2026-02-20

## 作業概要

GitHub Actions の全ワークフロー失敗を調査・修正した。

## 根本原因分析

### 1. 最大の問題: Linux 255バイト制限を超えるファイル名

**症状**: 全CIワークフローが "Checkout" ステップで失敗
**エラー**:
```
error: unable to create file .specstory/history/2026-01-28_17-09Z-@公式統合...
(477バイトのファイル名).md: File name too long
```

**原因**: `.specstory/history/` に 477バイト (UTF-8) のファイル名が存在。
Linux の `ext4/xfs` ファイルシステムは 255バイト制限あり。
Windows の NTFS では制限がないため、ローカルでは問題なかった。

**修正手順**:
1. `git ls-tree -z HEAD` でバイナリパスを取得
2. `git mktree` で問題ファイルを除いた新しいtreeオブジェクト作成
3. `git commit-tree` で新しいコミット作成（force pushなし）
4. `git reset --soft` でHEADを更新
5. `.gitignore` に `2026-01-28_17-09Z-@*` パターン追加

**使用スクリプト**: `scripts/create_fix_commit.py`, `scripts/delete_long_file.py`

### 2. Codespell 誤検知

**症状**: `Codespell` ワークフロー失敗
**エラー**: 以下の誤検知
- `LOD` (Level of Detail) → "LOAD, LOUD, LODE" に誤検知
- `playwright-report/` の生成ファイル内のコードトークン
- `.specstory/history/` の AI 会話ログ内の断片語
- `archive/scripts/` のビルドログ内のエラーメッセージ

**修正**:
- `check-hidden = false` → 隠しディレクトリ(`.specstory/`, `.codex/`)をスキップ
- `archive,logs` をskipリストに追加
- `*/playwright-report/*` などの生成ディレクトリをskip
- `ignore-words-list` に `LOD,lod,arent,gitar,ENew` 等を追加

### 3. cargo-deny ライセンスチェック失敗

**症状**: `cargo-deny` ワークフロー失敗
**エラー**: 37 個のワークスペースクレートが "unlicensed" と判定

**原因**: ワークスペースの `Cargo.toml` に `license = "Apache-2.0"` は定義されているが、
各クレートの `Cargo.toml` に `license = { workspace = true }` がなかった。

**修正**: `scripts/fix_cargo_license.py` で 37 クレートに自動追加

### 4. pnpm lockfile ミスマッチ

**症状**: CI の `build-test` ジョブが `ERR_PNPM_LOCKFILE_CONFIG_MISMATCH` で失敗
**エラー**: `package.json` の `resolutions` と `pnpm-lock.yaml` の `overrides` が不一致

**原因**: upstream sync 時に `package.json` の `resolutions` に
`ws, path-to-regexp, cookie, ip` が追加されたが、`pnpm-lock.yaml` が更新されなかった。

**修正**: `pnpm-lock.yaml` の `overrides` セクションに不足エントリを手動追加

## 修正コミット一覧

| コミット | 内容 |
|---------|------|
| `ab5f230a4` | fix: git tree から 477バイトファイル名削除 |
| `2d1705f5a` | fix: codespell LOD/package-lock、37クレートにlicense追加 |
| `48c86ac99` | fix: codespell playwright-report、pnpm lockfile、gitignore |
| `522898cfd` | fix: check-hidden=false で .specstory/ をスキップ |
| `8df09f09c` | fix: archive/ と logs/ を codespell スキップ |

## CI 結果

| ワークフロー | 修正前 | 修正後 |
|------------|--------|--------|
| rust-clippy analyze | failure | **success** |
| cargo-deny | failure | **success** |
| Codespell | failure | 修正中 |
| ci (build-test) | failure | 修正中 |
| sdk | failure | 修正中 |

## 作成ファイル

- `scripts/ci_check.py` - CI失敗分析スクリプト
- `scripts/create_fix_commit.py` - gitツリー修正コミット作成
- `scripts/delete_long_file.py` - Windowsの長いファイル名削除
- `scripts/fix_cargo_license.py` - Cargoライセンス一括修正
- `scripts/fix_long_git_files.py` - gitインデックスから長いファイル名削除
- `scripts/remove_long_file_from_git.py` - gitツリー操作スクリプト
