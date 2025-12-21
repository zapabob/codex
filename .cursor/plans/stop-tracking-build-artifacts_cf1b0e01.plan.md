---
name: stop-tracking-build-artifacts
overview: target_locked_backupディレクトリ内のビルド成果物（lib-*.json、lib-*.rlibなど）のGit追跡を停止し、今後追跡されないように.gitignoreに追加します。
todos:
  - id: check-status
    content: target_locked_backup内の追跡ファイル数と.gitignoreの内容を確認
    status: completed
  - id: remove-tracking
    content: git rm --cached -rでtarget_locked_backupの追跡を停止
    status: completed
  - id: update-gitignore
    content: .gitignoreにtarget_locked_backup/を追加
    status: completed
  - id: verify
    content: git statusで追跡が停止されたことを確認
    status: completed
---

# ビルド成果物の追跡停止

## ゴール

- `codex-rs/target_lo

cked_backup/` ディレクトリ内のビルド成果物のGit追跡を停止

- 今後これらのファイルが追跡されないように`.gitignore`に追加

## 実行手順

### 1. 現状確認

- `git ls-files`で`target_locked_backup`内の追跡ファイル数を確認
- `.gitignore`に`target_locked_backup/`が既に含まれているか確認

### 2. 追跡の停止

- 既に追跡されているファイルを追跡から外す:
  ```powershell
        git rm --cached -r codex-rs/target_locked_backup
  ```




- 注意: `--cached`オプションにより、ファイル自体は削除されず、追跡のみが停止されます

### 3. .gitignoreの更新

- `.gitignore`に`codex-rs/target_locked_backup/`を追加（まだ含まれていない場合）
- 適切な場所（ビルド成果物セクションなど）に追加

### 4. 結果確認

- `git status`で`target_locked_backup`が追跡されていないことを確認
- `.gitignore`の内容を確認

## 変更対象ファイル

- `.gitignore`: `codex-rs/target_locked_backup/`を追加

## 注意事項

- `git rm --cached`はファイ