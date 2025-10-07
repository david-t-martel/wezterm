param([switch]$Confirm)
if (-not $Confirm) {
  Write-Host "Use -Confirm to actually clear the sccache cache" -ForegroundColor Yellow
  return
}
sccache --stop-server 2>$null
$cacheDir = $env:SCCACHE_DIR
if ($cacheDir -and (Test-Path $cacheDir)) {
  Get-ChildItem -Path $cacheDir -Recurse -Force | Remove-Item -Force -Recurse
  Write-Host "Cleared: $cacheDir" -ForegroundColor Green
} else {
  Write-Host "Cache directory not found ($cacheDir)" -ForegroundColor Yellow
}
sccache --start-server
sccache --show-stats
