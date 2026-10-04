<#
.SYNOPSIS
    Builds goxlr-utility in release mode.
.DESCRIPTION
    Compiles the entire workspace with cargo build --release,
    measures elapsed time, and copies resulting binaries to a
    flat output folder for convenience.
#>
param(
    [switch]$Clean,
    [string]$OutputDir = "$PSScriptRoot\build-output"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Push-Location $PSScriptRoot
try {
    # Optional clean
    if ($Clean) {
        Write-Host "[*] Cleaning previous build..." -ForegroundColor Yellow
        cargo clean 2>&1 | Out-Null
    }

    Write-Host "[*] Building goxlr-utility (release)..." -ForegroundColor Cyan
    $sw = [System.Diagnostics.Stopwatch]::StartNew()

    # Temporarily relax ErrorAction so cargo's stderr progress lines
    # don't trigger PowerShell's NativeCommandError false positive.
    $prevEAP = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    cargo build --release 2>&1 | ForEach-Object { Write-Host $_ }
    $ErrorActionPreference = $prevEAP

    if ($LASTEXITCODE -ne 0) {
        Write-Host "[!] Build FAILED (exit code $LASTEXITCODE)" -ForegroundColor Red
        exit $LASTEXITCODE
    }

    $sw.Stop()
    Write-Host "[+] Build succeeded in $([math]::Round($sw.Elapsed.TotalSeconds, 1))s" -ForegroundColor Green

    # Copy binaries to output dir
    if (!(Test-Path $OutputDir)) { New-Item -ItemType Directory -Path $OutputDir | Out-Null }

    $exes = Get-ChildItem "$PSScriptRoot\target\release\*.exe" -ErrorAction SilentlyContinue
    if ($exes) {
        $exes | ForEach-Object {
            Copy-Item $_.FullName -Destination $OutputDir -Force
            Write-Host "  -> $($_.Name)" -ForegroundColor Gray
        }
        Write-Host "[+] $($exes.Count) binary(ies) copied to $OutputDir" -ForegroundColor Green
    } else {
        Write-Host "[!] No .exe found in target\release" -ForegroundColor Yellow
    }
} finally {
    Pop-Location
}
