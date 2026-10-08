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
    # Build core workspace binaries first so target/release has fresh goxlr-daemon.exe etc.
    cargo build --release --workspace --exclude installer 2>&1 | ForEach-Object { Write-Host $_ }

    # Inject custom icon into goxlr-utility-ui.exe (both build-output and target/release) before packaging
    $injectIconScript = @"
import ctypes, struct
from ctypes import wintypes
kernel32 = ctypes.WinDLL('kernel32', use_last_error=True)
BeginUpdateResourceW = kernel32.BeginUpdateResourceW
BeginUpdateResourceW.argtypes = [wintypes.LPCWSTR, wintypes.BOOL]
BeginUpdateResourceW.restype = wintypes.HANDLE
UpdateResourceW = kernel32.UpdateResourceW
UpdateResourceW.argtypes = [wintypes.HANDLE, wintypes.LPCWSTR, wintypes.LPCWSTR, wintypes.WORD, wintypes.LPVOID, wintypes.DWORD]
UpdateResourceW.restype = wintypes.BOOL
EndUpdateResourceW = kernel32.EndUpdateResourceW
EndUpdateResourceW.argtypes = [wintypes.HANDLE, wintypes.BOOL]
EndUpdateResourceW.restype = wintypes.BOOL

def update_exe_icon(exe_path, ico_path):
    with open(ico_path, 'rb') as f:
        ico_data = f.read()
    res, itype, count = struct.unpack('<HHH', ico_data[:6])
    hUpdate = BeginUpdateResourceW(exe_path, False)
    if not hUpdate: return
    grp_header = bytearray(struct.pack('<HHH', res, itype, count))
    for i in range(count):
        entry_offset = 6 + i * 16
        w, h, col, resv, planes, bpp, size, offset = struct.unpack('<BBBBHHII', ico_data[entry_offset:entry_offset+16])
        icon_bytes = ico_data[offset:offset+size]
        icon_id = i + 1
        res_type = ctypes.cast(ctypes.c_void_p(3), wintypes.LPCWSTR)
        res_name = ctypes.cast(ctypes.c_void_p(icon_id), wintypes.LPCWSTR)
        p_data = ctypes.cast(ctypes.create_string_buffer(icon_bytes), wintypes.LPVOID)
        UpdateResourceW(hUpdate, res_type, res_name, 0x0409, p_data, size)
        grp_header += struct.pack('<BBBBHHIH', w, h, col, resv, planes, bpp, size, icon_id)
    grp_bytes = bytes(grp_header)
    p_grp = ctypes.cast(ctypes.create_string_buffer(grp_bytes), wintypes.LPVOID)
    res_type_grp = ctypes.cast(ctypes.c_void_p(14), wintypes.LPCWSTR)
    for gid in [1, 32512]:
        res_name_grp = ctypes.cast(ctypes.c_void_p(gid), wintypes.LPCWSTR)
        UpdateResourceW(hUpdate, res_type_grp, res_name_grp, 0x0409, p_grp, len(grp_bytes))
    EndUpdateResourceW(hUpdate, False)

import os
ico = 'logo.ico'
for p in ['build-output/goxlr-utility-ui.exe', 'target/release/goxlr-utility-ui.exe', 'build-output/goxlr-daemon.exe', 'target/release/goxlr-daemon.exe', 'build-output/goxlr-launcher.exe', 'target/release/goxlr-launcher.exe']:
    if os.path.exists(p):
        update_exe_icon(p, ico)
        print(f'Icon updated in {p}')
"@
    python -c $injectIconScript

    # Build installer crate next so payload.tar.gz bundles the freshly built binaries
    cargo build --release -p installer 2>&1 | ForEach-Object { Write-Host $_ }
    $ErrorActionPreference = $prevEAP

    if ($LASTEXITCODE -ne 0) {
        Write-Host "[!] Build FAILED (exit code $LASTEXITCODE)" -ForegroundColor Red
        exit $LASTEXITCODE
    }

    $sw.Stop()
    Write-Host "[+] Build succeeded in $([math]::Round($sw.Elapsed.TotalSeconds, 1))s" -ForegroundColor Green

    # Copy binaries to output dir
    if (!(Test-Path $OutputDir)) { New-Item -ItemType Directory -Path $OutputDir | Out-Null }

    # Remove obsolete installer.exe if present from prior builds
    Remove-Item "$PSScriptRoot\target\release\installer.exe" -Force -ErrorAction SilentlyContinue
    Remove-Item "$OutputDir\installer.exe" -Force -ErrorAction SilentlyContinue

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
