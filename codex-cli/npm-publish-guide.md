# @zapabob/codex npm公開ガイド

**パッケージ名**: `@zapabob/codex`  
**バージョン**: `2.8.0`  
**公開先**: npmjs.org (public registry)

---

## 📋 事前準備

### 1. npmアカウントの確認
- npmアカウントを持っていることを確認
- アカウントURL: https://www.npmjs.com/signup

### 2. スコープの確認
- `@zapabob`スコープがnpmで利用可能か確認
- スコープは公開パッケージの場合、自動的に利用可能

---

## 🚀 公開手順

### Step 1: npmにログイン

```bash
npm login
```

**入力情報**:
- Username: `zapabob` (またはnpmアカウント名)
- Password: npmアカウントのパスワード
- Email: npmアカウントのメールアドレス
- One-time password: 2FAが有効な場合は入力

### Step 2: パッケージの確認

```bash
cd codex-cli
npm pack --dry-run
```

これで公開されるファイルの一覧が表示されます。

### Step 3: 公開実行

```bash
npm publish
```

**注意**: 
- `publishConfig`で`access: "public"`が設定されているため、スコープ付きパッケージでも公開されます
- 初回公開時は自動的に公開パッケージとして登録されます

### Step 4: 公開確認

```bash
# パッケージ情報の確認
npm view @zapabob/codex

# バージョン確認
npm view @zapabob/codex version

# インストールテスト
npm install -g @zapabob/codex
codex --version
```

---

## 📦 パッケージ情報

### 含まれるファイル
- `bin/codex.js` - メインのバイナリラッパー
- `vendor/` - プラットフォーム別のバイナリ
- `README.md` - ドキュメント
- `LICENSE` - ライセンスファイル

### サポートプラットフォーム
- Windows (x64, ARM64)
- macOS (x64, ARM64)
- Linux (x64, ARM64, musl)

---

## 🔍 トラブルシューティング

### エラー: "You do not have permission to publish"
- npmアカウントにログインしているか確認
- `@zapabob`スコープの所有権を確認

### エラー: "Package name already exists"
- バージョンを更新して再公開
- または既存のパッケージを確認

### エラー: "Invalid package name"
- `package.json`の`name`フィールドが正しいか確認
- スコープ名がnpmアカウント名と一致しているか確認

---

## 📝 公開後の確認事項

1. ✅ npmjs.orgでパッケージが表示される
   - URL: https://www.npmjs.com/package/@zapabob/codex

2. ✅ インストールが正常に動作する
   ```bash
   npm install -g @zapabob/codex
   codex --version
   ```

3. ✅ READMEが正しく表示される

4. ✅ バイナリが正常に動作する
   ```bash
   codex --help
   ```

---

完了！
