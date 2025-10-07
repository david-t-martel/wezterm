#!/usr/bin/env pwsh
# Build script for wezterm-fs-explorer

param(
    [switch]$Release,
    [switch]$Fast,
    [switch]$Install,
    [string]$Target = "x86_64-pc-windows-msvc"
)

$ErrorActionPreference = "Stop"

Write-Host "Building wezterm-fs-explorer..." -ForegroundColor Cyan

# Determine build profile
$profile = "dev"
if ($Fast) {
    $profile = "release-fast"
    Write-Host "Using fast release profile (optimized for size)" -ForegroundColor Yellow
} elseif ($Release) {
    $profile = "release"
    Write-Host "Using release profile" -ForegroundColor Yellow
}

# Build command
$buildArgs = @("build")
if ($Release -or $Fast) {
    $buildArgs += "--release"
}
if ($Fast) {
    $buildArgs += "--profile", "release-fast"
}
$buildArgs += "--target", $Target

Write-Host "Running: cargo $($buildArgs -join ' ')" -ForegroundColor Gray
cargo @buildArgs

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

# Determine binary path
$binaryName = "wezterm-fs-explorer.exe"
$targetDir = "target\$Target"
if ($Release -or $Fast) {
    $binaryPath = Join-Path $targetDir "release\$binaryName"
} else {
    $binaryPath = Join-Path $targetDir "debug\$binaryName"
}

# Get binary info
if (Test-Path $binaryPath) {
    $fileInfo = Get-Item $binaryPath
    $sizeKB = [math]::Round($fileInfo.Length / 1KB, 2)
    $sizeMB = [math]::Round($fileInfo.Length / 1MB, 2)

    Write-Host "`nBuild successful!" -ForegroundColor Green
    Write-Host "Binary: $binaryPath" -ForegroundColor White
    Write-Host "Size: $sizeKB KB ($sizeMB MB)" -ForegroundColor White

    # Install if requested
    if ($Install) {
        $installDir = "$env:USERPROFILE\.local\bin"
        if (-not (Test-Path $installDir)) {
            New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        }

        $installPath = Join-Path $installDir $binaryName
        Copy-Item $binaryPath $installPath -Force

        Write-Host "`nInstalled to: $installPath" -ForegroundColor Green

        # Check if in PATH
        if ($env:PATH -notlike "*$installDir*") {
            Write-Host "`nWARNING: $installDir is not in your PATH!" -ForegroundColor Yellow
            Write-Host "Add it with:" -ForegroundColor Yellow
            Write-Host "`$env:PATH += ';$installDir'" -ForegroundColor Cyan
        }

        # Test installation
        Write-Host "`nTesting installation..." -ForegroundColor Cyan
        & $installPath --version
    }
} else {
    Write-Host "Binary not found at: $binaryPath" -ForegroundColor Red
    exit 1
}

Write-Host "`nUsage:" -ForegroundColor Cyan
Write-Host "  wezterm-fs-explorer [DIRECTORY]" -ForegroundColor White
Write-Host "  wezterm-fs-explorer --help" -ForegroundColor White
Write-Host "  wezterm-fs-explorer --json /path/to/dir" -ForegroundColor White