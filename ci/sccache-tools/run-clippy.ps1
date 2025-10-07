param(
  [switch]$NoCache
)

if ($NoCache) {
  Remove-Item Env:RUSTC_WRAPPER -ErrorAction SilentlyContinue
  Write-Host "Running clippy without sccache wrapper (--NoCache)" -ForegroundColor Yellow
} else {
  if (-not $Env:SCCACHE_SERVER_PORT) { $Env:SCCACHE_SERVER_PORT = 4300 }
  sccache --start-server 2>$null | Out-Null
  $Env:RUSTC_WRAPPER = 'sccache'
}

# Pre-flight: ensure rustc version probe succeeds outside wrapper to avoid wrapper probe failures
$probeWrapper = $Env:RUSTC_WRAPPER
Remove-Item Env:RUSTC_WRAPPER -ErrorAction SilentlyContinue
rustc -vV *> $null
if ($probeWrapper) { $Env:RUSTC_WRAPPER = $probeWrapper }

Write-Host "Invoking cargo clippy (workspace, all targets)" -ForegroundColor Cyan
$clippyArgs = @('clippy','--workspace','--all-targets','--','-D','warnings','-A','clippy::type_complexity')

$p = Start-Process cargo -ArgumentList $clippyArgs -NoNewWindow -PassThru -Wait
if ($p.ExitCode -ne 0) {
  Write-Host "clippy failed with exit code $($p.ExitCode)" -ForegroundColor Red
  if ($Env:RUSTC_WRAPPER -eq 'sccache') { sccache --show-stats }
  exit $p.ExitCode
}
if ($Env:RUSTC_WRAPPER -eq 'sccache') { sccache --show-stats }
Write-Host "clippy succeeded" -ForegroundColor Green
