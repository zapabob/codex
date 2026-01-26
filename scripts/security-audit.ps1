# Security Audit Script for Rust2024 Production Environment
# 本番環境向けセキュリティ監査スクリプト

param(
    [switch]$Fix,
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"

Write-Host "🔒 Rust2024 Production Security Audit" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Check if cargo-deny is installed
Write-Host "Checking cargo-deny installation..." -ForegroundColor Yellow
$denyInstalled = cargo deny --version 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "⚠️  cargo-deny is not installed. Installing..." -ForegroundColor Yellow
    cargo install cargo-deny --locked
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to install cargo-deny" -ForegroundColor Red
        exit 1
    }
}

# Check if cargo-audit is installed
Write-Host "Checking cargo-audit installation..." -ForegroundColor Yellow
$auditInstalled = cargo audit --version 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "⚠️  cargo-audit is not installed. Installing..." -ForegroundColor Yellow
    cargo install cargo-audit
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to install cargo-audit" -ForegroundColor Red
        exit 1
    }
}

# Change to codex-rs directory
Push-Location codex-rs

try {
    # 1. Cargo Deny Check
    Write-Host ""
    Write-Host "1️⃣  Running cargo-deny check..." -ForegroundColor Green
    cargo deny check
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ cargo-deny check failed" -ForegroundColor Red
        if (-not $Fix) {
            exit 1
        }
    } else {
        Write-Host "✅ cargo-deny check passed" -ForegroundColor Green
    }

    # 2. Cargo Audit
    Write-Host ""
    Write-Host "2️⃣  Running cargo-audit..." -ForegroundColor Green
    cargo audit
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ cargo-audit found vulnerabilities" -ForegroundColor Red
        if (-not $Fix) {
            exit 1
        }
    } else {
        Write-Host "✅ cargo-audit passed (no vulnerabilities found)" -ForegroundColor Green
    }

    # 3. Check for unsafe blocks
    Write-Host ""
    Write-Host "3️⃣  Checking unsafe blocks..." -ForegroundColor Green
    $unsafeCount = (Select-String -Path "codex-rs\core\src\**\*.rs" -Pattern "unsafe\s*\{" -Exclude "*test*" | Measure-Object).Count
    Write-Host "   Found $unsafeCount unsafe blocks in production code" -ForegroundColor $(if ($unsafeCount -gt 0) { "Yellow" } else { "Green" })
    
    if ($unsafeCount -gt 0 -and $Verbose) {
        Write-Host "   Unsafe blocks found in:" -ForegroundColor Yellow
        Select-String -Path "codex-rs\core\src\**\*.rs" -Pattern "unsafe\s*\{" -Exclude "*test*" | 
            ForEach-Object { Write-Host "     - $($_.Path):$($_.LineNumber)" -ForegroundColor Gray }
    }

    # 4. Check for unwrap/expect in production code
    Write-Host ""
    Write-Host "4️⃣  Checking unwrap()/expect() in production code..." -ForegroundColor Green
    $unwrapCount = (Select-String -Path "codex-rs\core\src\**\*.rs" -Pattern "\.unwrap\(\)|\.expect\(" -Exclude "*test*" | Measure-Object).Count
    Write-Host "   Found $unwrapCount unwrap()/expect() calls in production code" -ForegroundColor $(if ($unwrapCount -gt 0) { "Yellow" } else { "Green" })
    
    if ($unwrapCount -gt 0 -and $Verbose) {
        Write-Host "   Unwrap/expect calls found in:" -ForegroundColor Yellow
        Select-String -Path "codex-rs\core\src\**\*.rs" -Pattern "\.unwrap\(\)|\.expect\(" -Exclude "*test*" | 
            ForEach-Object { Write-Host "     - $($_.Path):$($_.LineNumber)" -ForegroundColor Gray }
    }

    # 5. Summary
    Write-Host ""
    Write-Host "📊 Security Audit Summary" -ForegroundColor Cyan
    Write-Host "=========================" -ForegroundColor Cyan
    Write-Host "✅ cargo-deny: Checked" -ForegroundColor Green
    Write-Host "✅ cargo-audit: Checked" -ForegroundColor Green
    Write-Host "$(if ($unsafeCount -gt 0) { '⚠️' } else { '✅' }) Unsafe blocks: $unsafeCount" -ForegroundColor $(if ($unsafeCount -gt 0) { "Yellow" } else { "Green" })
    Write-Host "$(if ($unwrapCount -gt 0) { '⚠️' } else { '✅' }) Unwrap/expect: $unwrapCount" -ForegroundColor $(if ($unwrapCount -gt 0) { "Yellow" } else { "Green" })
    
} finally {
    Pop-Location
}

Write-Host ""
Write-Host "✅ Security audit completed" -ForegroundColor Green
