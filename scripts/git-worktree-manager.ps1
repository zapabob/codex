# Git Worktree Manager for Parallel Development
# Supports AI orchestration and sub-agent development workflows

param(
    [string]$Action = "list",
    [string]$Branch = "",
    [string]$BaseBranch = "main",
    [switch]$AutoMerge,
    [switch]$Clean,
    [switch]$Force
)

# Configuration
$WorktreeRoot = Join-Path $PSScriptRoot "..\worktrees"
$RepoRoot = Join-Path $PSScriptRoot ".."

function Write-Header {
    param([string]$Text)
    Write-Host "🔧 $Text" -ForegroundColor Cyan
    Write-Host ("-" * 50) -ForegroundColor Gray
}

function Get-Worktrees {
    try {
        $worktrees = git worktree list --porcelain | Where-Object { $_ -match "^worktree " } | ForEach-Object {
            $path = $_.Replace("worktree ", "").Trim()
            $branch = ""
            $commit = ""

            # Get branch info
            Push-Location $path
            try {
                $branch = git branch --show-current
                if (!$branch) {
                    $branch = git log -1 --oneline | Select-Object -First 1
                }
                $commit = git rev-parse --short HEAD
            } finally {
                Pop-Location
            }

            [PSCustomObject]@{
                Path = $path
                Branch = $branch
                Commit = $commit
                IsMain = $path -eq (Resolve-Path $RepoRoot)
            }
        }
        return $worktrees
    } catch {
        Write-Error "Failed to list worktrees: $_"
        return @()
    }
}

function New-Worktree {
    param([string]$BranchName, [string]$Base = $BaseBranch)

    if (!$BranchName) {
        Write-Error "Branch name is required for 'new' action"
        return
    }

    Write-Header "Creating new worktree for branch: $BranchName"

    # Ensure worktrees directory exists
    if (!(Test-Path $WorktreeRoot)) {
        New-Item -ItemType Directory -Path $WorktreeRoot | Out-Null
    }

    $worktreePath = Join-Path $WorktreeRoot $BranchName

    # Check if worktree already exists
    if (Test-Path $worktreePath) {
        Write-Warning "Worktree already exists at: $worktreePath"
        if ($Force) {
            Write-Host "Force removing existing worktree..."
            git worktree remove $worktreePath --force 2>$null
            Remove-Item -Recurse -Force $worktreePath 2>$null
        } else {
            return
        }
    }

    try {
        # Create worktree
        git worktree add $worktreePath $Base

        # Create and checkout branch if different from base
        if ($BranchName -ne $Base) {
            Push-Location $worktreePath
            try {
                git checkout -b $BranchName
            } finally {
                Pop-Location
            }
        }

        Write-Host "✅ Worktree created successfully: $worktreePath" -ForegroundColor Green
        Write-Host "📁 Branch: $BranchName" -ForegroundColor Yellow
        Write-Host "🔗 Base: $Base" -ForegroundColor Yellow

        return $worktreePath
    } catch {
        Write-Error "Failed to create worktree: $_"
        return $null
    }
}

function Remove-Worktree {
    param([string]$BranchName)

    if (!$BranchName) {
        Write-Error "Branch name is required for 'remove' action"
        return
    }

    $worktreePath = Join-Path $WorktreeRoot $BranchName

    if (!(Test-Path $worktreePath)) {
        Write-Warning "Worktree does not exist: $worktreePath"
        return
    }

    Write-Header "Removing worktree: $BranchName"

    try {
        # Remove worktree
        git worktree remove $worktreePath 2>$null

        # Clean up directory if it still exists
        if (Test-Path $worktreePath) {
            Remove-Item -Recurse -Force $worktreePath
        }

        Write-Host "✅ Worktree removed successfully: $BranchName" -ForegroundColor Green
    } catch {
        Write-Error "Failed to remove worktree: $_"
    }
}

function Merge-Worktree {
    param([string]$BranchName, [switch]$AutoResolve)

    if (!$BranchName) {
        Write-Error "Branch name is required for 'merge' action"
        return
    }

    $worktreePath = Join-Path $WorktreeRoot $BranchName

    if (!(Test-Path $worktreePath)) {
        Write-Warning "Worktree does not exist: $worktreePath"
        return
    }

    Write-Header "Merging worktree: $BranchName"

    try {
        # Switch to main repo
        Push-Location $RepoRoot

        # Check for conflicts
        $mergeBase = git merge-base HEAD $BranchName 2>$null
        if (!$mergeBase) {
            Write-Warning "No common ancestor found. This might be a complex merge."
        }

        # Attempt merge
        $mergeResult = git merge $BranchName --no-ff --log 2>&1

        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Merge completed successfully" -ForegroundColor Green
            Write-Host $mergeResult
        } else {
            Write-Host "⚠️  Merge conflicts detected" -ForegroundColor Yellow

            if ($AutoResolve) {
                Write-Host "🔄 Attempting auto-resolution with 'ours' strategy..."
                git checkout --ours . 2>$null
                git add . 2>$null

                $commitResult = git commit -m "Auto-merge: resolved conflicts favoring main branch" 2>&1
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "✅ Auto-resolution successful" -ForegroundColor Green
                } else {
                    Write-Host "❌ Auto-resolution failed" -ForegroundColor Red
                    Write-Host $commitResult
                }
            } else {
                Write-Host "Please resolve conflicts manually and commit" -ForegroundColor Yellow
                Write-Host "Use: git add <resolved-files> && git commit" -ForegroundColor Cyan
            }
        }

    } finally {
        Pop-Location
    }
}

function Clean-Worktrees {
    Write-Header "Cleaning orphaned worktrees"

    try {
        # List all worktrees
        $worktrees = Get-Worktrees

        foreach ($wt in $worktrees) {
            if (!(Test-Path $wt.Path)) {
                Write-Host "Removing orphaned worktree reference: $($wt.Path)" -ForegroundColor Yellow
                git worktree prune 2>$null
            } elseif ($wt.Branch -eq "") {
                Write-Host "Cleaning detached worktree: $($wt.Path)" -ForegroundColor Yellow
                try {
                    git worktree remove $wt.Path --force 2>$null
                } catch {
                    Write-Warning "Could not remove worktree: $($wt.Path)"
                }
            }
        }

        Write-Host "✅ Cleanup completed" -ForegroundColor Green
    } catch {
        Write-Error "Failed to clean worktrees: $_"
    }
}

function Show-Help {
    Write-Host @"
Git Worktree Manager for AI Orchestration

USAGE:
    .\git-worktree-manager.ps1 -Action <action> [parameters]

ACTIONS:
    list        List all worktrees (default)
    new         Create new worktree
    remove      Remove worktree
    merge       Merge worktree back to main
    clean       Clean orphaned worktrees

PARAMETERS:
    -Branch <name>      Branch/worktree name
    -BaseBranch <name>  Base branch for new worktrees (default: main)
    -AutoMerge          Auto-resolve merge conflicts
    -Clean              Clean mode (remove merged branches)
    -Force              Force operations

EXAMPLES:
    .\git-worktree-manager.ps1 -Action list
    .\git-worktree-manager.ps1 -Action new -Branch feature-ai-optimization
    .\git-worktree-manager.ps1 -Action merge -Branch feature-ai-optimization -AutoMerge
    .\git-worktree-manager.ps1 -Action clean

AI ORCHESTRATION FEATURES:
- Parallel development across multiple worktrees
- Automated merging with conflict resolution
- Integration with Codex sub-agent system
- Quality control and optimization workflows
"@
}

# Main execution
Push-Location $RepoRoot

try {
    switch ($Action.ToLower()) {
        "list" {
            Write-Header "Active Worktrees"
            $worktrees = Get-Worktrees
            if ($worktrees.Count -eq 0) {
                Write-Host "No worktrees found" -ForegroundColor Yellow
            } else {
                $worktrees | Format-Table -AutoSize -Property @(
                    @{Name="Branch"; Expression={$_.Branch}; Width=30},
                    @{Name="Commit"; Expression={$_.Commit}; Width=10},
                    @{Name="Path"; Expression={$_.Path}}
                )
            }
        }
        "new" {
            New-Worktree -BranchName $Branch -Base $BaseBranch
        }
        "remove" {
            Remove-Worktree -BranchName $Branch
        }
        "merge" {
            Merge-Worktree -BranchName $Branch -AutoResolve:$AutoMerge
        }
        "clean" {
            Clean-Worktrees
        }
        default {
            Show-Help
        }
    }
} finally {
    Pop-Location
}
