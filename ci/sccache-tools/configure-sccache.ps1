param(
  [string]$CacheDir = "$env:USERPROFILE/sccache-cache",
  [string]$CacheSize = "15G",
  [int]$Port = 4300
)

$env:RUSTC_WRAPPER = "sccache"
$env:SCCACHE_DIR = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($CacheDir)
$env:SCCACHE_CACHE_SIZE = $CacheSize
$env:SCCACHE_SERVER_PORT = $Port

Write-Host "Configured sccache:" -ForegroundColor Cyan
Write-Host "  RUSTC_WRAPPER=$env:RUSTC_WRAPPER" -ForegroundColor Green
Write-Host "  SCCACHE_DIR=$env:SCCACHE_DIR" -ForegroundColor Green
Write-Host "  SCCACHE_CACHE_SIZE=$env:SCCACHE_CACHE_SIZE" -ForegroundColor Green
Write-Host "  SCCACHE_SERVER_PORT=$env:SCCACHE_SERVER_PORT" -ForegroundColor Green

sccache --stop-server 2>$null | Out-Null
sccache --start-server
sccache --show-stats
