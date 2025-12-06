$ErrorActionPreference = 'Stop'

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "Error: 'cargo' is not installed. Please install Rust from https://rustup.rs/ first."
}

Write-Host "Building Crabby from source..."
cargo build --release

$InstallDir = "$env:LOCALAPPDATA\Programs\crabby"
$SourceExe = "target\release\crabby.exe"

if (-not (Test-Path $SourceExe)) {
    Write-Error "Error: Build failed. $SourceExe not found."
}

Write-Host "Installing to $InstallDir..."
if (Test-Path $InstallDir) {
    Remove-Item -Path $InstallDir -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

Copy-Item -Path $SourceExe -Destination $InstallDir -Force

$BinPath = "$InstallDir"
$ExePath = Join-Path $BinPath "crabby.exe"

if (-not (Test-Path $ExePath)) {
    Write-Error "Error: crabby.exe not found in $InstallDir"
}

# Add to PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$BinPath*") {
    Write-Host "Adding $BinPath to User PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$BinPath", "User")
    Write-Host "Path updated. You may need to restart your terminal."
} else {
    Write-Host "$BinPath is already in PATH."
}

Write-Host "Crabby installed successfully!"
