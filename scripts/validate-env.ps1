# 環境変数検証スクリプト
# 本番環境デプロイ前に必須環境変数の存在を確認

param(
    [string]$Environment = "production",
    [switch]$Strict
)

$ErrorActionPreference = "Stop"

Write-Host "🔍 環境変数検証 - $Environment" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

$errors = @()
$warnings = @()

# 必須環境変数（全環境共通）
$requiredVars = @(
    "CODEX_API_KEY",
    "OPENAI_API_KEY"
)

# 本番環境で必須
if ($Environment -eq "production") {
    $requiredVars += @(
        "NODE_ENV"
    )
}

# オプション環境変数（警告のみ）
$optionalVars = @(
    "GITHUB_TOKEN",
    "GEMINI_API_KEY"
)

# 必須環境変数のチェック
Write-Host "必須環境変数の確認..." -ForegroundColor Yellow
foreach ($var in $requiredVars) {
    $value = [System.Environment]::GetEnvironmentVariable($var, "Process")
    if ([string]::IsNullOrEmpty($value)) {
        $errors += "❌ $var が設定されていません"
        Write-Host "❌ $var が設定されていません" -ForegroundColor Red
    } else {
        # 機密情報の一部のみ表示（最初の4文字と最後の4文字）
        $masked = if ($value.Length -gt 8) {
            $value.Substring(0, 4) + "..." + $value.Substring($value.Length - 4)
        } else {
            "***"
        }
        Write-Host "✅ $var = $masked" -ForegroundColor Green
    }
}

# オプション環境変数のチェック
Write-Host ""
Write-Host "推奨環境変数の確認..." -ForegroundColor Yellow
foreach ($var in $optionalVars) {
    $value = [System.Environment]::GetEnvironmentVariable($var, "Process")
    if ([string]::IsNullOrEmpty($value)) {
        $warnings += "⚠️  $var が設定されていません（オプション）"
        Write-Host "⚠️  $var が設定されていません（オプション）" -ForegroundColor Yellow
    } else {
        $masked = if ($value.Length -gt 8) {
            $value.Substring(0, 4) + "..." + $value.Substring($value.Length - 4)
        } else {
            "***"
        }
        Write-Host "✅ $var = $masked" -ForegroundColor Green
    }
}

# 環境変数の検証
Write-Host ""
Write-Host "環境変数の値検証..." -ForegroundColor Yellow

# NODE_ENVの検証
if ($Environment -eq "production") {
    $nodeEnv = [System.Environment]::GetEnvironmentVariable("NODE_ENV", "Process")
    if ($nodeEnv -ne "production") {
        $errors += "❌ NODE_ENV は 'production' に設定する必要があります（現在: $nodeEnv）"
        Write-Host "❌ NODE_ENV は 'production' に設定する必要があります（現在: $nodeEnv）" -ForegroundColor Red
    } else {
        Write-Host "✅ NODE_ENV = production" -ForegroundColor Green
    }
}

# 結果サマリー
Write-Host ""
Write-Host "=====================================" -ForegroundColor Cyan
if ($errors.Count -eq 0) {
    Write-Host "✅ すべての必須環境変数が設定されています" -ForegroundColor Green
    if ($warnings.Count -gt 0) {
        Write-Host ""
        Write-Host "⚠️  警告: $($warnings.Count) 個の推奨環境変数が未設定です" -ForegroundColor Yellow
        if ($Strict) {
            Write-Host "Strict モードが有効なため、警告もエラーとして扱います" -ForegroundColor Red
            exit 1
        }
    }
    exit 0
} else {
    Write-Host "❌ エラー: $($errors.Count) 個の必須環境変数が未設定です" -ForegroundColor Red
    Write-Host ""
    Write-Host "設定方法:" -ForegroundColor Yellow
    Write-Host "  1. .env.example を .env にコピー" -ForegroundColor Gray
    Write-Host "  2. .env ファイルに実際の値を設定" -ForegroundColor Gray
    Write-Host "  3. 環境変数を読み込む: Get-Content .env | ForEach-Object { ... }" -ForegroundColor Gray
    exit 1
}
