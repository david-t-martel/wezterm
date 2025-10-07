# Build script for wezterm-watch
# Disables sccache to avoid compilation issues

$env:RUSTC_WRAPPER = ""

Write-Host "Building wezterm-watch..." -ForegroundColor Cyan
Write-Host "Disabling sccache wrapper..." -ForegroundColor Yellow

cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "`nBuild successful!" -ForegroundColor Green
    Write-Host "Binary location: C:\Users\david\.cargo\shared-target\release\wezterm-watch.exe" -ForegroundColor Green

    # Test if binary exists
    if (Test-Path "C:\Users\david\.cargo\shared-target\release\wezterm-watch.exe") {
        Write-Host "`nTesting binary..." -ForegroundColor Cyan
        & "C:\Users\david\.cargo\shared-target\release\wezterm-watch.exe" --version
    }
} else {
    Write-Host "`nBuild failed with exit code $LASTEXITCODE" -ForegroundColor Red
    exit $LASTEXITCODE
}