param(
  [Parameter(Mandatory = $true)]
  [string]$ExtensionId,

  [string]$HostPath,

  [switch]$Edge
)

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptDir "..\..")

if (-not $HostPath) {
  $HostPath = Join-Path $repoRoot "codex-rs\target\release\codex-chrome-host.exe"
}

if (-not (Test-Path $HostPath)) {
  Write-Error "Host binary not found: $HostPath"
  exit 1
}

$templatePath = Join-Path $scriptDir "native-messaging-host.json"
if (-not (Test-Path $templatePath)) {
  Write-Error "Template not found: $templatePath"
  exit 1
}

$outputPath = Join-Path $scriptDir "native-messaging-host.local.json"
$template = Get-Content -Raw $templatePath
$template = $template -replace "__HOST_PATH__", $HostPath
$template = $template -replace "__EXTENSION_ID__", $ExtensionId
Set-Content -Path $outputPath -Value $template -Encoding utf8

if ($Edge) {
  $baseKey = "HKCU:\Software\Microsoft\Edge\NativeMessagingHosts"
} else {
  $baseKey = "HKCU:\Software\Google\Chrome\NativeMessagingHosts"
}

New-Item -Path $baseKey -Force | Out-Null
$hostKey = Join-Path $baseKey "com.codex.chrome"
New-Item -Path $hostKey -Force | Out-Null
Set-ItemProperty -Path $hostKey -Name "(default)" -Value $outputPath

Write-Host "Registered native host at $outputPath"
