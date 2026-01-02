# npm公開準備: @zapabob/codex

**作成日時**: 2025-12-30  
**ワークツリー**: main  
**パッケージ名**: `@zapabob/codex`  
**バージョン**: 2.8.0

---

## 📋 現在の状況

### パッケージ設定

- **パッケージ名**: `@zapabob/codex` ✅
- **バージョン**: `2.8.0` ✅ (Rustバージョンと一致)
- **公開設定**: `publishConfig.access = "public"` ✅
- **レジストリ**: `https://registry.npmjs.org/` ✅

### 確認事項

- ✅ `package.json`は既に`@zapabob/codex`として設定済み
- ✅ バージョンを2.8.0に更新済み
- ❌ npmレジストリにまだ公開されていない（404エラー）
- ❌ npmアカウントへのログインが必要

---

## 🚀 公開手順

### 1. npmアカウントの準備

#### スコープ`@zapabob`の所有権確認

`@zapabob`スコープを使用するには、npmでスコープの所有権が必要です。

**確認方法**:
```bash
npm org ls zapabob
```

**スコープが存在しない場合**:
1. npmアカウントにログイン: `npm login`
2. スコープを作成（初回公開時に自動作成される場合あり）
3. または、個人アカウントで公開する場合は`@your-username/codex`に変更

### 2. npmログイン

```bash
cd codex-cli
npm login
```

**入力項目**:
- Username: npmアカウント名
- Password: npmパスワード
- Email: 登録済みメールアドレス
- OTP (Two-Factor認証が有効な場合): 認証コード

### 3. パッケージのビルド

#### バイナリを含むパッケージの準備

```bash
# Rustバイナリをビルド（既に完了している場合）
cd ../codex-rs
cargo build --release --bin codex

# npmパッケージをステージング
cd ../codex-cli
python3 scripts/build_npm_package.py \
  --package codex \
  --release-version 2.8.0 \
  --vendor-src ../vendor
```

**注意**: `vendor`ディレクトリにプラットフォーム別のバイナリが必要です。

#### プラットフォーム別バイナリの配置

```
vendor/
├── x86_64-pc-windows-msvc/
│   └── codex/
│       └── codex.exe
├── x86_64-apple-darwin/
│   └── codex/
│       └── codex
├── aarch64-apple-darwin/
│   └── codex/
│       └── codex
├── x86_64-unknown-linux-gnu/
│   └── codex/
│       └── codex
└── ...
```

### 4. パッケージの検証

```bash
cd codex-cli
npm pack --dry-run
```

**確認項目**:
- ✅ `bin/codex.js`が含まれている
- ✅ `vendor/`ディレクトリが含まれている
- ✅ `README.md`と`LICENSE`が含まれている
- ✅ 不要なファイルが除外されている（`.npmignore`を確認）

### 5. 公開前の最終確認

```bash
# パッケージ内容の確認
npm pack
tar -tzf zapabob-codex-2.8.0.tgz | head -20

# ローカルでテストインストール
npm install -g ./zapabob-codex-2.8.0.tgz
codex --version
```

### 6. npm公開

```bash
cd codex-cli
npm publish --access public
```

**公開時の注意事項**:
- スコープパッケージ（`@zapabob/codex`）は`--access public`が必須
- 初回公開後、バージョンは削除できない
- 公開前に必ずバージョンを確認

---

## 📦 パッケージ構成

### ファイル構造

```
@zapabob/codex/
├── bin/
│   └── codex.js          # エントリーポイント（プラットフォーム検出）
├── vendor/               # プラットフォーム別バイナリ
│   ├── x86_64-pc-windows-msvc/
│   │   └── codex/
│   │       └── codex.exe
│   ├── x86_64-apple-darwin/
│   │   └── codex/
│   │       └── codex
│   ├── aarch64-apple-darwin/
│   │   └── codex/
│   │       └── codex
│   ├── x86_64-unknown-linux-gnu/
│   │   └── codex/
│   │       └── codex
│   └── ...
├── README.md
├── LICENSE
└── package.json
```

### package.jsonの主要設定

```json
{
  "name": "@zapabob/codex",
  "version": "2.8.0",
  "bin": {
    "codex": "bin/codex.js"
  },
  "files": [
    "bin",
    "vendor",
    "README.md",
    "LICENSE"
  ],
  "publishConfig": {
    "access": "public",
    "registry": "https://registry.npmjs.org/"
  }
}
```

---

## 🔍 トラブルシューティング

### エラー: "You need to authorize this machine"

**解決方法**:
```bash
npm login
```

### エラー: "You do not have permission to publish"

**原因**: `@zapabob`スコープの所有権がない

**解決方法**:
1. npmアカウントで`@zapabob`スコープを作成
2. または、個人アカウント名を使用（例: `@your-username/codex`）

### エラー: "Package name already exists"

**原因**: 既に同じバージョンが公開されている

**解決方法**:
- バージョンを更新（例: `2.8.1`）
- または、既存のバージョンを確認: `npm view @zapabob/codex versions`

### エラー: "Missing vendor binaries"

**原因**: `vendor/`ディレクトリにバイナリが不足

**解決方法**:
1. Rustバイナリをビルド: `cargo build --release --bin codex`
2. バイナリを`vendor/`に配置
3. `build_npm_package.py`を実行

---

## 📝 公開後の確認

### パッケージ情報の確認

```bash
npm view @zapabob/codex
npm view @zapabob/codex version
npm view @zapabob/codex versions
```

### インストールテスト

```bash
# グローバルインストール
npm install -g @zapabob/codex

# バージョン確認
codex --version

# 動作確認
codex --help
```

### npmページの確認

公開後、以下のURLでパッケージページを確認できます：

```
https://www.npmjs.com/package/@zapabob/codex
```

---

## 🎯 次のステップ

1. ✅ バージョンを2.8.0に更新済み
2. ⏳ npmアカウントにログイン (`npm login`)
3. ⏳ バイナリをビルドして`vendor/`に配置
4. ⏳ `build_npm_package.py`でパッケージを準備
5. ⏳ `npm publish --access public`で公開

### 現在の状況（2025-12-30確認）

- ✅ **package.json**: `@zapabob/codex@2.8.0`として設定済み
- ❌ **npmログイン**: 未ログイン（`npm login`が必要）
- ❌ **vendorディレクトリ**: 存在しない（バイナリビルドが必要）
- ❌ **パッケージ**: まだ準備されていない
- ❌ **npm公開**: 未公開（404エラー確認済み）

### 実行コマンド（順序）

```bash
# 1. npmログイン
cd codex-cli
npm login

# 2. Rustバイナリをビルド（既に完了している場合、このステップはスキップ）
cd ../codex-rs
cargo build --release --bin codex

# 3. vendorディレクトリにバイナリを配置
# （ビルドスクリプトまたは手動で配置）

# 4. npmパッケージを準備
cd ../codex-cli
python3 scripts/build_npm_package.py \
  --package codex \
  --release-version 2.8.0 \
  --vendor-src ../vendor

# 5. パッケージの検証
npm pack --dry-run

# 6. npm公開
npm publish --access public
```

---

## 📚 参考資料

- [npm公式ドキュメント: Publishing scoped packages](https://docs.npmjs.com/cli/v10/commands/npm-publish#publishing-scoped-packages)
- [npm公式ドキュメント: Creating and publishing scoped public packages](https://docs.npmjs.com/creating-and-publishing-scoped-public-packages)
- `codex-cli/scripts/build_npm_package.py` - パッケージビルドスクリプト

---

**作成者**: Codex AI Agent  
**最終更新**: 2025-12-30
