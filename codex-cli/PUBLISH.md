# npm Package Publishing Guide

このドキュメントでは、`@openai/codex` npm パッケージを公開する手順を説明します。

## 📋 前提条件

### 必須ツール

- **Node.js** >= 16
- **npm** (または yarn/pnpm)
- **Rust** (stable, edition 2024)
- **cargo**
- **rustup** (複数ターゲットのビルド用)

### 必須アカウント

- npm アカウント (https://www.npmjs.com/)
- 組織アクセス権限 (`@openai` スコープ)

### 環境確認

```bash
# Node.js
node --version  # v16以上

# npm
npm --version

# Rust
rustc --version
cargo --version

# rustup
rustup --version
```

---

## 🏗️ ビルド手順

### 1. Rust ターゲットの追加

```bash
# Linux (musl for static linking)
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl

# macOS
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Windows
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
```

### 2. バイナリのビルド

#### オプション A: 全プラットフォームビルド (推奨、CI/CD用)

```bash
cd codex-cli

# 全ターゲットをビルド
BUILD_TARGETS=x86_64-unknown-linux-musl,aarch64-unknown-linux-musl,x86_64-apple-darwin,aarch64-apple-darwin,x86_64-pc-windows-msvc,aarch64-pc-windows-msvc npm run build:rust
```

#### オプション B: 現在のプラットフォームのみ (開発・テスト用)

```bash
cd codex-cli

# 現在のプラットフォームのみビルド
npm run build:rust
```

#### オプション C: 手動ビルド

```bash
cd codex-rs

# 特定ターゲット向けビルド
cargo build --release --target x86_64-unknown-linux-musl --bin codex-tui

# バイナリを vendor/ にコピー
cp target/x86_64-unknown-linux-musl/release/codex ../codex-cli/vendor/x86_64-unknown-linux-musl/codex/
```

### 3. ビルド確認

```bash
cd codex-cli

# vendor ディレクトリの確認
ls -R vendor/

# 期待される構造:
# vendor/
# ├── x86_64-unknown-linux-musl/codex/codex
# ├── aarch64-unknown-linux-musl/codex/codex
# ├── x86_64-apple-darwin/codex/codex
# ├── aarch64-apple-darwin/codex/codex
# ├── x86_64-pc-windows-msvc/codex/codex.exe
# └── aarch64-pc-windows-msvc/codex/codex.exe
```

---

## 🧪 ローカルテスト

### 1. パッケージのローカルインストール

```bash
cd codex-cli

# pack でパッケージを作成
npm pack

# 生成された .tgz ファイルをグローバルインストール
npm install -g openai-codex-0.1.0.tgz
```

### 2. 動作確認

```bash
# バージョン確認
codex --version

# ヘルプ表示
codex --help

# 実際に使ってみる
codex --profile workspace
```

### 3. テストスクリプト実行

```bash
cd codex-cli

# テストスクリプト
npm test
```

### 4. アンインストール

```bash
npm uninstall -g @openai/codex
```

---

## 📦 公開手順

### 1. npmログイン

```bash
npm login

# ログイン確認
npm whoami
```

### 2. バージョン更新

```bash
cd codex-cli

# package.json のバージョンを更新
# "version": "0.1.0" → "0.1.1" など

# または npm version コマンドを使用
npm version patch  # 0.1.0 → 0.1.1
npm version minor  # 0.1.0 → 0.2.0
npm version major  # 0.1.0 → 1.0.0
```

### 3. ドライラン (推奨)

```bash
# 公開せずに確認
npm publish --dry-run

# 出力を確認:
# - パッケージに含まれるファイル
# - パッケージサイズ
# - 警告やエラー
```

### 4. 公開

```bash
# 本番公開
npm publish --access public

# スコープ付きパッケージは --access public が必要
```

### 5. 公開確認

```bash
# npmjsで確認
open https://www.npmjs.com/package/@openai/codex

# インストールテスト
npm install -g @openai/codex
codex --version
```

---

## 🤖 CI/CD による自動公開

### GitHub Actions ワークフロー例

```yaml
name: Publish to npm

on:
  release:
    types: [published]

jobs:
  publish-npm:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target:
          - x86_64-unknown-linux-musl
          - aarch64-unknown-linux-musl
          - x86_64-apple-darwin
          - aarch64-apple-darwin
          - x86_64-pc-windows-msvc
          - aarch64-pc-windows-msvc

    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install target
        run: rustup target add ${{ matrix.target }}

      - name: Build binary
        working-directory: codex-rs
        run: cargo build --release --target ${{ matrix.target }} --bin codex-tui

      - name: Upload binary
        uses: actions/upload-artifact@v4
        with:
          name: codex-${{ matrix.target }}
          path: codex-rs/target/${{ matrix.target }}/release/codex*

  publish:
    needs: publish-npm
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '18'
          registry-url: 'https://registry.npmjs.org'

      - name: Download all binaries
        uses: actions/download-artifact@v4
        with:
          path: codex-cli/vendor

      - name: Publish to npm
        working-directory: codex-cli
        run: npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

---

## 🔒 セキュリティチェックリスト

公開前に以下を確認してください:

- [ ] すべてのテストがパス (`cargo test --all`)
- [ ] セキュリティテストがパス (`cargo test --test sandbox_escape_tests`)
- [ ] リンターエラーなし
- [ ] `cargo audit` でセキュリティ脆弱性なし
- [ ] バイナリのウイルススキャン完了
- [ ] `.npmignore` または `package.json` の `files` フィールドが適切
- [ ] 不要なファイルが含まれていない (`.git`, `.env`, etc.)
- [ ] LICENSE ファイルが含まれている
- [ ] README.md が最新
- [ ] CHANGELOG.md が更新されている

```bash
# セキュリティチェック
cd codex-rs
cargo audit
cargo test --all

# パッケージ内容確認
cd ../codex-cli
npm pack --dry-run
```

---

## 📝 バージョニング規則

[Semantic Versioning 2.0.0](https://semver.org/) に従います:

- **MAJOR** (1.0.0): 後方互換性のない変更
- **MINOR** (0.1.0): 後方互換性のある新機能追加
- **PATCH** (0.0.1): 後方互換性のあるバグ修正

### プレリリースバージョン

```bash
# alpha
npm version prerelease --preid=alpha  # 0.1.0-alpha.0

# beta
npm version prerelease --preid=beta   # 0.1.0-beta.0

# rc (release candidate)
npm version prerelease --preid=rc     # 0.1.0-rc.0
```

---

## 🚨 トラブルシューティング

### エラー: Binary not found

```bash
# vendor/ ディレクトリが存在するか確認
ls -R codex-cli/vendor/

# 不足しているターゲットをビルド
npm run build:rust
```

### エラー: Permission denied (EACCES)

```bash
# npm パーミッション修正
sudo chown -R $(whoami) ~/.npm
sudo chown -R $(whoami) /usr/local/lib/node_modules

# または nvm 使用を推奨
```

### エラー: You must be logged in to publish

```bash
npm login
npm whoami  # ログイン確認
```

### エラー: Package size too large

```bash
# バイナリのstrip (サイズ削減)
cd codex-rs
cargo build --release
strip target/release/codex-tui

# または Rust の設定で strip を有効化 (Cargo.toml)
[profile.release]
strip = true
opt-level = "z"  # サイズ最適化
```

---

## 📊 公開後の確認

### npmjs.com での確認事項

- [ ] パッケージページが正しく表示される
- [ ] README が正しく表示される
- [ ] バージョンが正しい
- [ ] ダウンロード統計が有効
- [ ] すべてのプラットフォームのバイナリが含まれている

### ユーザーテスト

```bash
# 別のマシンでインストールテスト
npm install -g @openai/codex

# 動作確認
codex --version
codex --help
codex --profile workspace
```

---

## 🔄 更新の公開

### 手順

1. ブランチ作成
```bash
git checkout -b release/v0.1.1
```

2. 変更実装・テスト

3. バージョン更新
```bash
cd codex-cli
npm version patch  # または minor/major
```

4. CHANGELOG更新
```bash
# CHANGELOG.md に変更内容を記載
```

5. コミット・プッシュ
```bash
git add .
git commit -m "chore: release v0.1.1"
git push origin release/v0.1.1
```

6. Pull Request作成

7. マージ後、リリースタグ作成
```bash
git tag v0.1.1
git push origin v0.1.1
```

8. npm公開
```bash
cd codex-cli
npm publish --access public
```

---

## 📞 サポート

質問や問題があれば:

- GitHub Issues: https://github.com/openai/codex/issues
- Email: support@openai.com

---

**最終更新**: 2025-10-08  
**バージョン**: 0.1.0

