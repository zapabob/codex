#!/bin/bash
# 環境変数検証スクリプト（Bash版）
# 本番環境デプロイ前に必須環境変数の存在を確認

set -euo pipefail

ENVIRONMENT="${1:-production}"
STRICT="${2:-false}"

echo "🔍 環境変数検証 - $ENVIRONMENT"
echo "====================================="
echo ""

ERRORS=()
WARNINGS=()

# 必須環境変数（全環境共通）
REQUIRED_VARS=(
    "CODEX_API_KEY"
    "OPENAI_API_KEY"
)

# 本番環境で必須
if [ "$ENVIRONMENT" = "production" ]; then
    REQUIRED_VARS+=("NODE_ENV")
fi

# オプション環境変数（警告のみ）
OPTIONAL_VARS=(
    "GITHUB_TOKEN"
    "GEMINI_API_KEY"
)

# 必須環境変数のチェック
echo "必須環境変数の確認..."
for var in "${REQUIRED_VARS[@]}"; do
    value="${!var:-}"
    if [ -z "$value" ]; then
        ERRORS+=("❌ $var が設定されていません")
        echo "❌ $var が設定されていません"
    else
        # 機密情報の一部のみ表示
        if [ ${#value} -gt 8 ]; then
            masked="${value:0:4}...${value: -4}"
        else
            masked="***"
        fi
        echo "✅ $var = $masked"
    fi
done

# オプション環境変数のチェック
echo ""
echo "推奨環境変数の確認..."
for var in "${OPTIONAL_VARS[@]}"; do
    value="${!var:-}"
    if [ -z "$value" ]; then
        WARNINGS+=("⚠️  $var が設定されていません（オプション）")
        echo "⚠️  $var が設定されていません（オプション）"
    else
        if [ ${#value} -gt 8 ]; then
            masked="${value:0:4}...${value: -4}"
        else
            masked="***"
        fi
        echo "✅ $var = $masked"
    fi
done

# 環境変数の検証
echo ""
echo "環境変数の値検証..."

# NODE_ENVの検証
if [ "$ENVIRONMENT" = "production" ]; then
    NODE_ENV="${NODE_ENV:-}"
    if [ "$NODE_ENV" != "production" ]; then
        ERRORS+=("❌ NODE_ENV は 'production' に設定する必要があります（現在: $NODE_ENV）")
        echo "❌ NODE_ENV は 'production' に設定する必要があります（現在: $NODE_ENV）"
    else
        echo "✅ NODE_ENV = production"
    fi
fi

# 結果サマリー
echo ""
echo "====================================="
if [ ${#ERRORS[@]} -eq 0 ]; then
    echo "✅ すべての必須環境変数が設定されています"
    if [ ${#WARNINGS[@]} -gt 0 ]; then
        echo ""
        echo "⚠️  警告: ${#WARNINGS[@]} 個の推奨環境変数が未設定です"
        if [ "$STRICT" = "true" ]; then
            echo "Strict モードが有効なため、警告もエラーとして扱います"
            exit 1
        fi
    fi
    exit 0
else
    echo "❌ エラー: ${#ERRORS[@]} 個の必須環境変数が未設定です"
    echo ""
    echo "設定方法:"
    echo "  1. .env.example を .env にコピー"
    echo "  2. .env ファイルに実際の値を設定"
    echo "  3. 環境変数を読み込む: source .env"
    exit 1
fi
