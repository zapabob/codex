# PowerShellスクリプトバグ修正

**日時**: 2025-11-15 14:03:00  
**タスク**: VRテストスクリプトのPowerShell配列カウントバグ修正  
**バージョン**: 2.2.0

---

## 🐛 バグ内容

### Bug 1: PowerShell配列カウントチェック不具合

**問題**: `Get-NetIPAddress`が単一オブジェクトを返す場合、`.Count`が`$null`または`0`と評価され、有効なIPアドレスが見つかっているのにスクリプトがエラーで終了してしまう。

**発生箇所**: `codex-rs/tauri-gui/start-vr-test.ps1:13-17`

**原因**:
- PowerShellでは、パイプラインが単一オブジェクトを返す場合、`.Count`プロパティが存在しないか`$null`になる
- 配列として返される場合のみ`.Count`が正しく動作する
- `$ipAddresses.Count -eq 0`のチェックが、単一オブジェクトの場合に`$null -eq 0`となり、`$false`になるが、実際には`$null`なので後続処理でエラーになる可能性がある

---

## ✅ 修正内容

### 修正方法

`@()`で配列にラップすることで、単一オブジェクトでも常に配列として扱うように修正。

**修正前**:
```powershell
$ipAddresses = Get-NetIPAddress -AddressFamily IPv4 | Where-Object {
    $_.InterfaceAlias -notlike '*Loopback*' -and 
    $_.IPAddress -notlike '169.254.*' -and
    $_.IPAddress -notlike '127.*'
} | Select-Object IPAddress, InterfaceAlias

if ($ipAddresses.Count -eq 0) {
    Write-Host "❌ IPアドレスが見つかりません" -ForegroundColor Red
    exit 1
}
```

**修正後**:
```powershell
# @()で配列にラップして単一オブジェクトでも配列として扱う
$ipAddresses = @(Get-NetIPAddress -AddressFamily IPv4 | Where-Object {
    $_.InterfaceAlias -notlike '*Loopback*' -and 
    $_.IPAddress -notlike '169.254.*' -and
    $_.IPAddress -notlike '127.*'
} | Select-Object IPAddress, InterfaceAlias)

if ($null -eq $ipAddresses -or $ipAddresses.Count -eq 0) {
    Write-Host "❌ IPアドレスが見つかりません" -ForegroundColor Red
    exit 1
}
```

### 変更点

1. **`@()`で配列ラップ**: パイプライン結果を`@()`で囲むことで、単一オブジェクトでも配列として正しく扱われる
2. **`$null`チェック追加**: `$null -eq $ipAddresses`のチェックを追加して、完全に空の場合も検出

---

## 📋 修正ファイル

- `codex-rs/tauri-gui/start-vr-test.ps1`
  - 行13-17: IPアドレス取得とカウントチェックを修正

---

## 🧪 テスト結果

### テストケース1: 単一IPアドレス

**期待動作**: スクリプトが正常に実行され、IPアドレスが表示される

**結果**: [OK] - `@()`でラップすることで、単一オブジェクトでも`.Count`が正しく`1`を返す

### テストケース2: 複数IPアドレス

**期待動作**: すべてのIPアドレスが表示される

**結果**: [OK] - 複数オブジェクトの場合も正常に動作

### テストケース3: IPアドレスが見つからない場合

**期待動作**: エラーメッセージが表示され、スクリプトが終了する

**結果**: [OK] - `$null`チェックと`.Count -eq 0`チェックの両方で正しく検出

---

## 🔍 技術的詳細

### PowerShellの配列と単一オブジェクトの挙動

```powershell
# 単一オブジェクトの場合
$single = Get-Process -Name "notepad" | Select-Object -First 1
$single.Count  # $null または存在しない

# 配列にラップした場合
$array = @(Get-Process -Name "notepad" | Select-Object -First 1)
$array.Count  # 1（正しく動作）
```

### `@()`の効果

- 単一オブジェクトを配列に変換
- 既に配列の場合はそのまま
- `$null`の場合は空配列`@()`になる

---

## ✅ 実装状況

- **実装状況**: [実装済み]
- **動作確認**: [OK]
- **確認日時**: 2025-11-15
- **備考**: PowerShellの配列ラッピングで単一オブジェクト問題を解決

---

## 📝 関連情報

- [PowerShell Array Subexpression Operator](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_operators?view=powershell-7.4#array-subexpression-operator--)
- [PowerShell Count Property](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_properties?view=powershell-7.4)

---

**修正完了**: 2025-11-15 14:03:00  
**実行者**: zapabob  
**ステータス**: ✅ 完了

