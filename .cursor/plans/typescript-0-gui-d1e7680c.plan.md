<!-- d1e7680c-cc20-47da-a63c-d4adaa1434c2 4f0bcbc4-ec4b-4c21-b363-0aea45f6be2f -->
# バージョンアップとビルドインストール計画

## 1. プロセスキル

### 1.1 CLI/TUI/GUIプロセスの終了

- `codex`プロセスを検索して終了
- `codex-tui`プロセスを検索して終了
- `codex-gui`プロセスを検索して終了
- `codex-tauri-gui`プロセスを検索して終了

**実装**: PowerShellスクリプトで`Get-Process`と`Stop-Process`を使用

## 2. セマンティックバージョンのアップ

### 2.1 バージョンファイルの更新

- `VERSION`: v1.2.0 → v1.3.0
- `codex-rs/Cargo.toml`: workspace versionを更新
- `codex-rs/tauri-gui/package.json`: versionを2.1.0 → 2.3.0に更新（メジャーバージョンは別管理）

**理由**: セマンティックバージョニングに従い、マイナーバージョンをインクリメント

## 3. GUIショートカットの更新

### 3.1 ショートカットスクリプトの実行

- `scripts/create-gui-shortcut.ps1`を実行
- デスクトップショートカットを更新
- アイコンとパスを最新版に更新

## 4. アーキテクチャ図の更新

### 4.1 最新機能の反映

- セキュリティ機能（マルウェア検知、パスワード管理、リアルタイム監視）を追加
- GUI統合テストの追加
- CI/CDパイプラインの更新

**ファイル**: `docs/architecture/architecture-v2.3.0.mmd`を新規作成

## 5. README.mdの更新

### 5.1 バージョン情報の更新

- バージョン番号をv2.2.0 → v2.3.0に更新
- 新機能セクションにセキュリティ機能を追加
- インストール手順の更新

## 6. 高速差分ビルド

### 6.1 sccacheの確認と使用

- sccacheがインストールされているか確認
- `RUSTC_WRAPPER=sccache`を設定
- 差分ビルドを実行（変更されたクレートのみ再ビルド）

**コマンド**:

```powershell
cd codex-rs
$env:RUSTC_WRAPPER = "sccache"
cargo build --release -p codex-cli
```

## 7. 強制インストール

### 7.1 CLIのインストール

- `cargo install --path cli --force`を実行
- インストール後のバージョン確認

### 7.2 GUIのビルドとインストール（オプション）

- Tauri GUIのビルド（必要に応じて）
- MSIインストーラーの生成

## 実装順序

1. プロセスキル
2. バージョンアップ（VERSION, Cargo.toml, package.json）
3. GUIショートカット更新
4. アーキテクチャ図更新
5. README.md更新
6. 高速差分ビルド
7. 強制インストール
8. バージョン確認

## 検証項目

- すべてのプロセスが正常に終了したか
- バージョン番号が正しく更新されたか
- ショートカットが正しく作成されたか
- ビルドが成功したか
- インストールが成功したか
- `codex --version`で正しいバージョンが表示されるか

### To-dos

- [x] 
- [x] 
- [x] 
- [x] 
- [x] 
- [ ] 高速差分ビルド（reqwestのfeature名をrustls-tlsに修正完了）
- [ ] 