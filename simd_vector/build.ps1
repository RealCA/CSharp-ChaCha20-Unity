# build_windows.ps1
# -----------------
# Build SimdVector Rust library for Windows (x64 and x86) and copy DLLs to dist folder.

# Stop on error
$ErrorActionPreference = "Stop"

# Paths
$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$DistRoot = Join-Path $ProjectRoot "dist/windows"
$X64Dist = Join-Path $DistRoot "x86_64"
$X86Dist = Join-Path $DistRoot "x86"

# Create output directories
New-Item -ItemType Directory -Force -Path $X64Dist | Out-Null
New-Item -ItemType Directory -Force -Path $X86Dist | Out-Null

Write-Host "Building x64..."
cargo build --release --target x86_64-pc-windows-msvc
Copy-Item "target/x86_64-pc-windows-msvc/release/SimdVector.dll" $X64Dist -Force

Write-Host "Building x86..."
cargo build --release --target i686-pc-windows-msvc
Copy-Item "target/i686-pc-windows-msvc/release/SimdVector.dll" $X86Dist -Force

Write-Host "Build complete. DLLs copied to dist/windows/x86_64 and dist/windows/x86"