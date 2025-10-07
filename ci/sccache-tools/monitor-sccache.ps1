param([int]$IntervalSeconds = 5)
Write-Host "Monitoring sccache stats (Ctrl+C to stop)..." -ForegroundColor Cyan
while ($true) {
  Clear-Host
  Write-Host "=== SCCACHE STATISTICS ===" -ForegroundColor Green
  Write-Host "Time: $(Get-Date -Format 'HH:mm:ss')" -ForegroundColor Yellow
  sccache --show-stats
  Start-Sleep -Seconds $IntervalSeconds
}
