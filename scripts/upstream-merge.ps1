# Upstream Merge Script
# 公式リポジトリの最新機能・脆弱性更新・バグフィクスを取り込み、独自機能を維持

param(
    [switch]$DryRun = $false,  # ドライラン（実際のマージは実行しない）
    [switch]$SkipBuild = $false  # ビルド検証をスキップ
)

$ErrorActionPreference = "Stop"

function Write-Status {
    param([string]$Message, [string]$Color = "Cyan")
    Write-Host "[*] $Message" -ForegroundColor $Color
}

function Write-Success {
    param([string]$Message)
    Write-Host "[OK] $Message" -ForegroundColor Green
}

function Write-ErrorMsg {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

# リポジトリルートの確認
$repoRoot = if ($PSScriptRoot) {
    Split-Path $PSScriptRoot -Parent
} else {
    Get-Location
}

if (-not (Test-Path (Join-Path $repoRoot ".git"))) {
    Write-ErrorMsg "Not a git repository: $repoRoot"
    exit 1
}

Set-Location $repoRoot
Write-Status "Repository root: $repoRoot"

# Step 1: Upstreamの最新取得
Write-Status "Step 1/5: Fetching upstream..."
try {
    git fetch upstream
    Write-Success "Upstream fetched"
} catch {
    Write-ErrorMsg "Failed to fetch upstream: $_"
    exit 1
}

# Step 2: 差分分析
Write-Status "Step 2/5: Analyzing differences..."

$commitCount = (git rev-list --count main..upstream/main 2>$null)
if ($LASTEXITCODE -ne 0) {
    $commitCount = "unknown"
}

Write-Host "  Commits behind upstream/main: $commitCount" -ForegroundColor Gray

# 重要なコミットの特定
Write-Status "Identifying important commits (security fixes, bug fixes, CVE)..."
$allCommits = git log main..upstream/main --oneline 2>$null
$importantCommits = $allCommits | Select-String -Pattern "security|CVE|fix|bug" -CaseSensitive:$false

if ($importantCommits) {
    Write-Host "  Important commits found:" -ForegroundColor Yellow
    $importantCommits | ForEach-Object {
        Write-Host "    $_" -ForegroundColor Gray
    }
} else {
    Write-Host "  No explicitly tagged important commits found" -ForegroundColor Gray
}

# ファイル差分の確認
Write-Status "Checking file differences..."
$fileDiff = git diff --stat main upstream/main 2>$null
if ($fileDiff) {
    Write-Host "  File changes:" -ForegroundColor Gray
    $fileDiff | Select-Object -First 20 | ForEach-Object {
        Write-Host "    $_" -ForegroundColor Gray
    }
    if (($fileDiff | Measure-Object -Line).Lines -gt 20) {
        Write-Host "    ... (more files)" -ForegroundColor Gray
    }
}

# Step 3: マージコンフリクトの予測
Write-Status "Step 3/5: Predicting merge conflicts..."
$mergeTree = git merge-tree $(git merge-base main upstream/main) main upstream/main 2>$null
if ($mergeTree -match "changed in both") {
    Write-Warning "Potential merge conflicts detected"
    Write-Host "  Review merge-tree output above" -ForegroundColor Yellow
} else {
    Write-Success "No obvious merge conflicts predicted"
}

# Step 4: 作業ブランチ作成とマージ実行
if (-not $DryRun) {
    Write-Status "Step 4/5: Creating merge branch and merging..."
    
    $branchName = "upstream-sync-$(Get-Date -Format 'yyyy-MM-dd')"
    Write-Status "Creating branch: $branchName"
    
    try {
        git checkout -b $branchName
        Write-Success "Branch created: $branchName"
    } catch {
        Write-ErrorMsg "Failed to create branch: $_"
        exit 1
    }
    
    Write-Status "Merging upstream/main..."
    try {
        git merge upstream/main --no-ff -m "Merge upstream/main: 公式リポジトリの最新を取り込み"
        Write-Success "Merge completed"
    } catch {
        Write-Warning "Merge conflicts detected. Resolve conflicts and continue."
        Write-Host "  Conflict resolution strategy:" -ForegroundColor Yellow
        Write-Host "    1. Prioritize upstream changes (security fixes, bug fixes)" -ForegroundColor Gray
        Write-Host "    2. Custom features are protected by #[cfg(feature = \"custom-features\")]" -ForegroundColor Gray
        Write-Host "    3. Re-apply custom features in separate commit if needed" -ForegroundColor Gray
        exit 1
    }
} else {
    Write-Status "Step 4/5: Dry run - skipping merge"
    Write-Host "  Would create branch: upstream-sync-$(Get-Date -Format 'yyyy-MM-dd')" -ForegroundColor Gray
    Write-Host "  Would merge: upstream/main" -ForegroundColor Gray
}

# Step 5: ビルド検証
if (-not $SkipBuild -and -not $DryRun) {
    Write-Status "Step 5/5: Verifying build..."
    
    Set-Location (Join-Path $repoRoot "codex-rs")
    
    Write-Status "Building with custom-features..."
    try {
        cargo build --features custom-features 2>&1 | Tee-Object -Variable buildOutput
        
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Build succeeded"
        } else {
            Write-ErrorMsg "Build failed"
            Write-Host $buildOutput -ForegroundColor Red
            exit 1
        }
    } catch {
        Write-ErrorMsg "Build error: $_"
        exit 1
    }
    
    Write-Status "Running tests..."
    try {
        cargo test --features custom-features --workspace 2>&1 | Tee-Object -Variable testOutput
        
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Tests passed"
        } else {
            Write-Warning "Some tests failed (review output above)"
        }
    } catch {
        Write-Warning "Test execution error: $_"
    }
} else {
    Write-Status "Step 5/5: Skipping build verification"
}

Write-Host ""
Write-Success "Upstream merge process completed!"
if ($DryRun) {
    Write-Host "  This was a dry run. Re-run without -DryRun to perform actual merge." -ForegroundColor Yellow
}
