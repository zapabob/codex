# 2026-01-29 CI/CD Fix

## 概要

CI/CDパイプラインで `actions/setup-node@v4` がロックファイルの欠落および `pnpm` コマンド未検出により失敗していた問題を修正しました。

## 実装内容

### 1. ロックファイルの生成

- プロジェクトルートに `pnpm-lock.yaml` が存在しなかったため、`pnpm install` を実行して生成しました。
- これにより、`setup-node` のキャッシュ機能と `pnpm install --frozen-lockfile` が正しく動作するようになりました。

### 2. ワークフロー設定の修正

- `.github/workflows/integration-tests.yml` の設定を修正しました。
  - `cache: 'npm'` を `cache: 'pnpm'` に変更。
  - `npm ci` コマンドを `pnpm install --frozen-lockfile` に変更。
  - `npm run build` コマンドを `pnpm run build` に変更。
  - **追加**: `actions/setup-node` の前に `pnpm/action-setup@v4` ステップを追加し、`pnpm` コマンドが確実に利用できるようにしました。
- `ci.yml` は既に `cache` 設定が適切（または手動キャッシュ）であり、`pnpm install` を使用していたため、変更不要でした。
- `qa-ci.yml` についても調査を行いましたが、`setup-node` を使用していないため影響なしと判断しました。

## 検証

- ローカル環境で `pnpm-lock.yaml` が生成されたことを確認しました。
- `integration-tests.yml` の各ジョブに `pnpm` セットアップステップが追加されたことを確認しました。
