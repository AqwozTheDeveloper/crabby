$ErrorActionPreference = 'Stop'

# Crabby Git Installer for Windows
$RepoUrl = "https://github.com/AqwozTheDeveloper/crabby.git"
$InstallDir = "$env:USERPROFILE\.crabby"

Write-Host "🦀 Crabby Git Installer" -ForegroundColor Cyan
Write-Host "=======================" -ForegroundColor Cyan
Write-Host ""

# Check if git is installed
if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Error "Error: 'git' is not installed. Please install git from https://git-scm.com/ first."
}

# Check if cargo is installed
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "Error: 'cargo' is not installed. Please install Rust from https://rustup.rs/ first."
}

# Clone or update repository
if (Test-Path $InstallDir) {
    Write-Host "📦 Updating existing Crabby installation..." -ForegroundColor Yellow
    Set-Location $InstallDir
    git pull origin main
} else {
    Write-Host "📦 Cloning Crabby repository..." -ForegroundColor Yellow
    git clone $RepoUrl $InstallDir
    Set-Location $InstallDir
}

Write-Host ""
Write-Host "🔨 Building and installing Crabby..." -ForegroundColor Yellow
Write-Host ""

# Run the install script
& .\install.ps1

Write-Host ""
Write-Host "✅ Crabby installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Run 'crabby --help' to get started." -ForegroundColor Cyan
