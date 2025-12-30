# Cドライブの容量分析スクリプト
$ErrorActionPreference = 'SilentlyContinue'

Write-Host "Cドライブの容量分析を開始します..." -ForegroundColor Cyan

$results = @()

# 主要なディレクトリをチェック
$dirs = @(
    'C:\Users',
    'C:\Program Files',
    'C:\Program Files (x86)',
    'C:\Windows',
    'C:\ProgramData',
    'C:\Temp',
    'C:\Windows\Temp',
    'C:\$Recycle.Bin',
    'C:\pagefile.sys',
    'C:\hiberfil.sys',
    'C:\swapfile.sys'
)

foreach ($dir in $dirs) {
    if (Test-Path $dir) {
        Write-Host "分析中: $dir" -ForegroundColor Yellow
        try {
            if (Test-Path $dir -PathType Leaf) {
                # ファイルの場合
                $item = Get-Item $dir -ErrorAction SilentlyContinue
                if ($item) {
                    $size = $item.Length
                    $sizeGB = [math]::Round($size / 1GB, 2)
                    $results += [PSCustomObject]@{
                        Path = $dir
                        Type = 'File'
                        SizeGB = $sizeGB
                        SizeBytes = $size
                    }
                }
            } else {
                # ディレクトリの場合
                $size = (Get-ChildItem -Path $dir -Recurse -ErrorAction SilentlyContinue | 
                         Measure-Object -Property Length -Sum -ErrorAction SilentlyContinue).Sum
                if ($size) {
                    $sizeGB = [math]::Round($size / 1GB, 2)
                    $results += [PSCustomObject]@{
                        Path = $dir
                        Type = 'Directory'
                        SizeGB = $sizeGB
                        SizeBytes = $size
                    }
                }
            }
        } catch {
            Write-Host "  エラー: $_" -ForegroundColor Red
        }
    }
}

# C:\Users配下の各ユーザーディレクトリもチェック
if (Test-Path 'C:\Users') {
    Write-Host "C:\Users配下のユーザーディレクトリを分析中..." -ForegroundColor Yellow
    $userDirs = Get-ChildItem -Path 'C:\Users' -Directory -ErrorAction SilentlyContinue
    foreach ($userDir in $userDirs) {
        $userPath = $userDir.FullName
        Write-Host "  分析中: $userPath" -ForegroundColor Gray
        try {
            $size = (Get-ChildItem -Path $userPath -Recurse -ErrorAction SilentlyContinue | 
                     Measure-Object -Property Length -Sum -ErrorAction SilentlyContinue).Sum
            if ($size) {
                $sizeGB = [math]::Round($size / 1GB, 2)
                $results += [PSCustomObject]@{
                    Path = $userPath
                    Type = 'User Directory'
                    SizeGB = $sizeGB
                    SizeBytes = $size
                }
            }
        } catch {
            Write-Host "    エラー: $_" -ForegroundColor Red
        }
    }
}

# 結果をソート
$sortedResults = $results | Sort-Object -Property SizeBytes -Descending

# 合計サイズを計算
$totalSize = ($results | Measure-Object -Property SizeBytes -Sum).Sum
$totalSizeGB = [math]::Round($totalSize / 1GB, 2)

# 出力ファイル名
$outputFile = "C:\Users\downl\Desktop\codex-main\_docs\Cドライブ容量分析_$(Get-Date -Format 'yyyy-MM-dd_HHmmss').md"

# Markdown形式で出力
$mdContent = @"
# Cドライブ容量分析レポート

**分析日時**: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
**合計サイズ**: $totalSizeGB GB ($([math]::Round($totalSize / 1TB, 2)) TB)

## ディレクトリ/ファイル別サイズ一覧

| 順位 | パス | タイプ | サイズ (GB) | サイズ (TB) |
|------|------|--------|-------------|-------------|
"@

$rank = 1
foreach ($item in $sortedResults) {
    $sizeTB = [math]::Round($item.SizeBytes / 1TB, 3)
    $mdContent += "`n| $rank | ``$($item.Path)`` | $($item.Type) | $($item.SizeGB) GB | $sizeTB TB |"
    $rank++
}

$mdContent += @"

## 詳細情報

### トップ10

"@

$rank = 1
foreach ($item in ($sortedResults | Select-Object -First 10)) {
    $sizeTB = [math]::Round($item.SizeBytes / 1TB, 3)
    $mdContent += @"

#### $rank. $($item.Path)

- **タイプ**: $($item.Type)
- **サイズ**: $($item.SizeGB) GB ($sizeTB TB)
- **サイズ (バイト)**: $($item.SizeBytes) bytes

"@
    $rank++
}

$mdContent += @"

## 推奨アクション

1. **一時ファイルの削除**: C:\Windows\Temp や C:\Temp をクリーンアップ
2. **ユーザーディレクトリの整理**: 大きなファイルや不要なデータを削除
3. **プログラムのアンインストール**: 使用していないアプリケーションを削除
4. **ディスククリーンアップ**: Windowsのディスククリーンアップツールを実行

---

*このレポートは自動生成されました。*
"@

# ファイルに出力
$mdContent | Out-File -FilePath $outputFile -Encoding UTF8

Write-Host ""
Write-Host "分析完了！" -ForegroundColor Green
Write-Host "結果を保存しました: $outputFile" -ForegroundColor Green
Write-Host ""
Write-Host "トップ5:" -ForegroundColor Cyan
$sortedResults | Select-Object -First 5 | Format-Table -Property Path, Type, @{Label='Size (GB)'; Expression={$_.SizeGB}} -AutoSize
