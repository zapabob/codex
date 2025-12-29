# @zapabob/codex npm公開ガイド

## 📋 事前準備

### 1. npmアカウントの確認
- npmアカウントにログイン: https://www.npmjs.com/login
- スコープ`@zapabob`が存在することを確認（存在しない場合は作成）

### 2. ローカルでnpmにログイン
```bash
npm login
# Username: [あなたのnpmユーザー名]
# Password: [あなたのnpmパスワード]
# Email: [あなたのnpmメールアドレス]
```

### 3. スコープの確認
```bash
npm whoami
# あなたのnpmユーザー名が表示されることを確認
```

## 🚀 公開手順

### 1. パッケージの確認
```bash
# 現在のディレクトリで実行
npm pack --dry-run
```

### 2. 公開実行
```bash
npm publish
```

### 3. 公開確認
```bash
# パッケージ情報を確認
npm view @zapabob/codex

# インストールテスト
npm install -g @zapabob/codex
codex --version
```

## 📦 パッケージ情報

- **パッケージ名**: `@zapabob/codex`
- **バージョン**: `2.8.0`
- **レジストリ**: `https://registry.npmjs.org/`
- **公開範囲**: `public`（誰でもインストール可能）

## ⚠️ 注意事項

1. **バージョン管理**: 同じバージョンは再公開できません。バージョンを更新する場合は`package.json`の`version`を変更してください。

2. **スコープパッケージ**: `@zapabob/codex`はスコープパッケージです。公開には`publishConfig.access: "public"`が必要です（既に設定済み）。

3. **バイナリ配布**: `postinstall`スクリプトでGitHub Releasesからバイナリをダウンロードします。バイナリが存在しない場合はインストールが失敗します。

## 🔗 公開後のURL

- **npmパッケージ**: https://www.npmjs.com/package/@zapabob/codex
- **GitHubリポジトリ**: https://github.com/zapabob/codex
