<#
.SYNOPSIS
    Compiles only the goxlr-daemon executable for rapid testing.
.DESCRIPTION
    Builds goxlr-daemon with latest web-content assets without rebuilding
    the full workspace, installer, or other binaries.
    Supports -Debug for fastest compilation and -Restart to auto-relaunch.
#>
param(
    [switch]$Debug,
    [switch]$Restart,
    [string]$OutputDir = "$PSScriptRoot\build-output"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Push-Location $PSScriptRoot
try {
    $mode = if ($Debug) { "debug" } else { "release" }
    Write-Host "[*] Building goxlr-daemon ($mode)..." -ForegroundColor Cyan
    $sw = [System.Diagnostics.Stopwatch]::StartNew()

    $prevEAP = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    if ($Debug) {
        cargo build -p goxlr-daemon 2>&1 | ForEach-Object { Write-Host $_ }
    } else {
        cargo build --release -p goxlr-daemon 2>&1 | ForEach-Object { Write-Host $_ }
    }
    $ErrorActionPreference = $prevEAP

    if ($LASTEXITCODE -ne 0) {
        Write-Host "[!] Build FAILED (exit code $LASTEXITCODE)" -ForegroundColor Red
        exit $LASTEXITCODE
    }

    $sw.Stop()
    Write-Host "[+] Built goxlr-daemon in $([math]::Round($sw.Elapsed.TotalSeconds, 1))s" -ForegroundColor Green

    # Handle running daemon process
    $running = Get-Process "goxlr-daemon" -ErrorAction SilentlyContinue
    if ($running) {
        Write-Host "[*] Stopping running goxlr-daemon to overwrite binary..." -ForegroundColor Yellow
        $running | Stop-Process -Force
        Start-Sleep -Milliseconds 500
    }

    if (!(Test-Path $OutputDir)) {
        New-Item -ItemType Directory -Path $OutputDir | Out-Null
    }

    $srcExe = "$PSScriptRoot\target\$mode\goxlr-daemon.exe"
    $destExe = "$OutputDir\goxlr-daemon.exe"

    if (Test-Path $srcExe) {
        Copy-Item -Path $srcExe -Destination $destExe -Force
        Write-Host "[+] Copied $srcExe -> $destExe" -ForegroundColor Green
    } else {
        Write-Host "[!] $srcExe not found!" -ForegroundColor Red
        exit 1
    }

    if ($Restart) {
        Write-Host "[*] Launching updated goxlr-daemon..." -ForegroundColor Cyan
        Start-Process -FilePath $destExe
        Write-Host "[+] goxlr-daemon running." -ForegroundColor Green
    }
} finally {
    Pop-Location
}
