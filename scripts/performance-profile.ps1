# Performance Profiling Script for Rust2024 Production
# 本番環境向けパフォーマンスプロファイリングスクリプト

param(
    [string]$Target = "codex-cli",
    [switch]$Flamegraph,
    [switch]$Benchmark
)

$ErrorActionPreference = "Stop"

Write-Host "⚡ Rust2024 Performance Profiling" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host ""

Push-Location codex-rs

try {
    # 1. Build with release profile
    Write-Host "1️⃣  Building release binary..." -ForegroundColor Green
    cargo build --release -p $Target
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Build failed" -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ Build completed" -ForegroundColor Green

    # 2. Generate flamegraph if requested
    if ($Flamegraph) {
        Write-Host ""
        Write-Host "2️⃣  Generating flamegraph..." -ForegroundColor Green
        
        # Check if flamegraph is installed
        $flamegraphInstalled = cargo flamegraph --version 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Host "⚠️  cargo-flamegraph is not installed. Installing..." -ForegroundColor Yellow
            cargo install flamegraph
        }
        
        Write-Host "   Running with flamegraph profiling..." -ForegroundColor Yellow
        cargo flamegraph --release -p $Target -- --help
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Flamegraph generated: flamegraph.svg" -ForegroundColor Green
        }
    }

    # 3. Run benchmarks if requested
    if ($Benchmark) {
        Write-Host ""
        Write-Host "3️⃣  Running benchmarks..." -ForegroundColor Green
        cargo bench -p $Target
        if ($LASTEXITCODE -ne 0) {
            Write-Host "⚠️  Benchmarks failed or not available" -ForegroundColor Yellow
        } else {
            Write-Host "✅ Benchmarks completed" -ForegroundColor Green
        }
    }

    # 4. Performance metrics
    Write-Host ""
    Write-Host "4️⃣  Performance Metrics" -ForegroundColor Green
    $binaryPath = "target\release\$Target.exe"
    if (Test-Path $binaryPath) {
        $fileInfo = Get-Item $binaryPath
        $sizeMB = [math]::Round($fileInfo.Length / 1MB, 2)
        Write-Host "   Binary size: $sizeMB MB" -ForegroundColor Cyan
        Write-Host "   Binary path: $($fileInfo.FullName)" -ForegroundColor Gray
    }

    Write-Host ""
    Write-Host "✅ Performance profiling completed" -ForegroundColor Green

} finally {
    Pop-Location
}
