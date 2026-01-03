# Codex 高速ビルド＆インストール手順

## 🚀 自動実行（推奨）
```bash
just build-install
```

## 🔧 手動実行（回避策）
### ステップ1: 高速差分ビルド
```bash
py -3 scripts/fast_build.py release
```

### ステップ2: プロセスキル＆インストール
```bash
py -3 scripts/install_with_kill.ps1 -SourcePath "codex-rs/target/release/codex.exe" -TargetPath "C:\bin\codex.exe" -Force
```

## 📋 確認方法
### インストール確認
```bash
codex --version
```

### ファイル存在確認
```bash
ls C:\bin\codex.exe
```

## ⚠️ 問題解決

ビルドが失敗する場合：
1. Windowsを再起動
2. 再度上記コマンドを実行

## 🎯 実装済み機能

- ✅ **高速差分ビルド**: MD5ハッシュベース変更検出
- ✅ **プロセス自動終了**: psutilクロスプラットフォーム対応
- ✅ **上書きインストール**: アトミックバイナリ置換
- ✅ **進捗可視化**: tqdmリアルタイム表示
- ✅ **統合リリース**: 全プラットフォームtgzパッケージ