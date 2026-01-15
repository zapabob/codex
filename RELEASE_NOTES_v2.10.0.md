# Codex v2.9.0 Release Notes

## 🚀 What's New

### Fast Incremental Build System
- **高速差分ビルド**: Cargoのincremental compilationを活用した高速ビルドシステム
- **変更検出**: MD5ハッシュベースのインテリジェントなファイル変更検出
- **ビルドキャッシュ**: 永続化ビルドキャッシュによる再ビルド高速化
- **進捗表示**: tqdmを使用したリアルタイムビルド進捗表示
- **並列処理**: CPUコア数最適化による並列ビルド実行

### Hot Reload Installation System
- **プロセス自動終了**: psutilを使用したクロスプラットフォームプロセス管理
- **上書きインストール**: アトミックなバイナリ置換による安全な更新
- **プラットフォーム自動検出**: Windows/macOS/Linuxでの自動インストール
- **インストール検証**: インストール後のバージョン確認と機能検証
- **PowerShell統合**: Windows固有の高度なインストール機能

### Integrated Release Packaging
- **統合バイナリアーカイブ**: 全プラットフォームのバイナリを含むtgzパッケージ
- **自動インストールスクリプト**: プラットフォーム別自動インストールスクリプト生成
- **包括的なドキュメント**: インストール手順と使用方法の詳細ドキュメント
- **GitHub Actions統合**: 自動化されたクロスプラットフォームリリース

## 📦 Installation

### Quick Install (Recommended)
```bash
# Download and extract
tar -xzf codex-v1.0.0.tgz
cd codex-v1.0.0

# Run installer
./install.sh
```

### Manual Install
1. Choose the appropriate binary for your platform:
   - **Windows**: `bin/codex-windows-x64.exe`
   - **macOS Intel**: `bin/codex-darwin-x64`
   - **macOS Apple Silicon**: `bin/codex-darwin-arm64`
   - **Linux x64**: `bin/codex-linux-x64`

2. Copy to a directory in your PATH
3. Verify: `codex --version`

## 🛠️ Development Tools

### Fast Build Commands
```bash
# Quick debug build
just fast-build

# Release build
just fast-build release

# Build specific packages
just fast-build debug codex-cli codex-tui
```

### Build & Install Commands
```bash
# Build and install to default location
just build-install

# Build and install to custom location
just build-install --install-path /custom/path/codex

# Skip process killing
just build-install --skip-kill
```

## 🔧 Technical Details

### Build Optimizations
- **Incremental Compilation**: Cargoのincremental機能を活用
- **Parallel Builds**: CPUコア数に基づく並列ビルド最適化
- **Change Detection**: MD5ハッシュによる効率的な変更検出
- **Smart Caching**: ビルド成果物のインテリジェントキャッシュ

### Installation Safety
- **Process Detection**: psutilを使用した実行中プロセス検出
- **Graceful Shutdown**: 安全なプロセス終了処理
- **Atomic Operations**: 中断耐性のあるファイル操作
- **Verification**: インストール後の機能検証

## 📈 Performance Improvements

- **ビルド時間**: 平均30-50%の高速化（差分ビルド時）
- **インストール時間**: プロセス終了と検証を含む完全自動化
- **リリースサイズ**: 最適化されたバイナリと統合パッケージ

## 🐛 Bug Fixes

- プロセス終了時の競合状態を修正
- キャッシュ破損時の自動回復機能を追加
- プラットフォーム検出の信頼性向上

## 📚 Documentation

- 詳細なインストール手順
- 開発者向けビルドガイド
- トラブルシューティングガイド

---

## Previous Releases

- See [CHANGELOG.md](CHANGELOG.md) for detailed change history