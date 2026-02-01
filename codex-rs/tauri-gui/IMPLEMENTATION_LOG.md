# GUI バージョン管理システム実装ログ

## 実装日

2026-02-02

## 概要

Codex Tauri GUIのバージョン表記をハードコードから環境変数ベースの一元管理システムに移行しました。

## 問題点

- バージョンが複数のファイルにハードコードされていた
- package.json、tauri.conf.json、Cargo.toml、GUIレイアウトファイルでバージョンが不一致だった
- バージョン更新時に複数ファイルを手動で修正する必要があった

## 解決策

Viteの`define`機能を使用して、package.jsonのバージョンをビルド時に環境変数として注入し、すべてのコンポーネントから一元管理されるようにしました。

## 変更ファイル一覧

### 1. 新規作成ファイル

#### `src/version.ts`

バージョン定数の一元管理ファイル

- `APP_VERSION`: 純粋なバージョン文字列（例: "2.13.0"）
- `APP_VERSION_DISPLAY`: 表示用（例: "v2.13.0"）
- `APP_NAME`: アプリ名
- `DEFAULT_VERSION`: フォールバック用バージョン

#### `src/vite-env.d.ts`

TypeScript型定義ファイル

- `__APP_VERSION__` グローバル定数の型宣言
- `__APP_NAME__` グローバル定数の型宣言

### 2. 更新ファイル

#### `vite.config.ts`

- `fs`モジュールを使用してpackage.jsonを読み込み
- `define`オプションでグローバル定数を定義:
  ```typescript
  define: {
    __APP_VERSION__: JSON.stringify(version),
    __APP_NAME__: JSON.stringify(packageJson.name),
  }
  ```

#### `package.json`

- version: "2.7.0" → "2.13.0"

#### `src-tauri/tauri.conf.json`

- version: "2.3.2" → "2.13.0"

#### `src-tauri/Cargo.toml`

- version: "2.7.0" → "2.13.0"

#### `src/App.tsx`

- インポート追加: `import { APP_VERSION_DISPLAY } from "./version";`
- 変更: `<p className="version">v1.5.0</p>` → `<p className="version">{APP_VERSION_DISPLAY}</p>`

#### `src/pages/Settings.tsx`

- インポート追加: `import { APP_VERSION } from "../version";`
- 変更: `<p>Version: 0.1.0</p>` → `<p>Version: {APP_VERSION}</p>`

#### `src/pages/Dashboard.tsx`

- インポート追加: `import { DEFAULT_VERSION } from "../version";`
- 変更: `status={status?.version || "0.1.0"}` → `status={status?.version || DEFAULT_VERSION}`

## データフロー

```
package.json (version: "2.13.0")
    ↓
vite.config.ts (readFileSync, define)
    ↓
__APP_VERSION__ (global constant)
    ↓
src/version.ts (export const APP_VERSION)
    ↓
各コンポーネント (import { APP_VERSION })
```

## バージョン更新手順

1. package.jsonのversionフィールドを更新
2. ビルド実行（npm run tauri:build または npm run tauri:dev）
3. 完了 - すべてのコンポーネントが自動的に新しいバージョンを使用

## メリット

- **一元管理**: バージョンはpackage.jsonのみで管理
- **自動同期**: ビルド時にすべてのコンポーネントに自動反映
- **型安全**: TypeScript型定義により、型安全性を保持
- **柔軟性**: 表示形式（v2.13.0 vs 2.13.0）を容易に変更可能

## 今後の改善案

1. CI/CDパイプラインでpackage.jsonのバージョンを自動更新
2. ビルド時にGitタグからバージョンを動的に取得
3. ナイトリービルド用にタイムスタンプ付きバージョン番号の生成

## 検証コマンド

```bash
# ハードコードされたバージョンが残っていないか確認
grep -r "2\.13\.0\|2\.7\.0\|2\.3\.2\|1\.5\.0\|0\.1\.0" codex-rs/tauri-gui/src/ --include="*.tsx" --include="*.ts"

# 環境変数を使用しているか確認
grep -r "APP_VERSION\|DEFAULT_VERSION" codex-rs/tauri-gui/src/ --include="*.tsx" --include="*.ts"
```

## 関連ファイルパス

- `codex-rs/tauri-gui/package.json`
- `codex-rs/tauri-gui/vite.config.ts`
- `codex-rs/tauri-gui/src/version.ts`
- `codex-rs/tauri-gui/src/vite-env.d.ts`
- `codex-rs/tauri-gui/src/App.tsx`
- `codex-rs/tauri-gui/src/pages/Settings.tsx`
- `codex-rs/tauri-gui/src/pages/Dashboard.tsx`
- `codex-rs/tauri-gui/src-tauri/tauri.conf.json`
- `codex-rs/tauri-gui/src-tauri/Cargo.toml`
