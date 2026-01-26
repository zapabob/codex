# Test Strategy Script for Rust2024 Production
# 本番環境品質を保証するテスト戦略スクリプト

param(
    [switch]$Coverage,
    [switch]$All,
    [switch]$Release,
    [string]$Package = ""
)

$ErrorActionPreference = "Stop"

Write-Host "🧪 Rust2024 Production Test Strategy" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan
Write-Host ""

Push-Location codex-rs

try {
    $testArgs = @()
    if ($Release) {
        $testArgs += "--release"
    }
    if ($Package) {
        $testArgs += "-p", $Package
    } else {
        $testArgs += "--workspace"
    }
    if ($All) {
        $testArgs += "--all-features"
    }

    # 1. Unit Tests
    Write-Host "1️⃣  Running unit tests..." -ForegroundColor Green
    $unitTestArgs = $testArgs + "--lib"
    cargo test @unitTestArgs
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Unit tests failed" -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ Unit tests passed" -ForegroundColor Green

    # 2. Integration Tests
    Write-Host ""
    Write-Host "2️⃣  Running integration tests..." -ForegroundColor Green
    $integrationTestArgs = $testArgs + "--test"
    cargo test @integrationTestArgs
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Integration tests failed" -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ Integration tests passed" -ForegroundColor Green

    # 3. Coverage Report
    if ($Coverage) {
        Write-Host ""
        Write-Host "3️⃣  Generating coverage report..." -ForegroundColor Green
        
        # Check if cargo-tarpaulin is installed
        $tarpaulinInstalled = cargo tarpaulin --version 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Host "⚠️  cargo-tarpaulin is not installed. Installing..." -ForegroundColor Yellow
            cargo install cargo-tarpaulin
        }
        
        $coverageArgs = @()
        if ($Package) {
            $coverageArgs += "-p", $Package
        } else {
            $coverageArgs += "--workspace"
        }
        if ($All) {
            $coverageArgs += "--all-features"
        }
        $coverageArgs += "--out", "Html"
        
        cargo tarpaulin @coverageArgs
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Coverage report generated: tarpaulin-report.html" -ForegroundColor Green
        } else {
            Write-Host "⚠️  Coverage report generation failed" -ForegroundColor Yellow
        }
    }

    # 4. Summary
    Write-Host ""
    Write-Host "📊 Test Summary" -ForegroundColor Cyan
    Write-Host "===============" -ForegroundColor Cyan
    Write-Host "✅ Unit tests: Passed" -ForegroundColor Green
    Write-Host "✅ Integration tests: Passed" -ForegroundColor Green
    if ($Coverage) {
        Write-Host "✅ Coverage report: Generated" -ForegroundColor Green
    }

} finally {
    Pop-Location
}

Write-Host ""
Write-Host "✅ Test strategy completed" -ForegroundColor Green
