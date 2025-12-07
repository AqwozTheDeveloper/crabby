# Quick test of the git-installer.ps1 logic
$TestDir = "$env:TEMP\crabby-installer-test"

# Clean up any previous test
if (Test-Path $TestDir) {
    Remove-Item -Recurse -Force $TestDir
}

Write-Host "Testing git-installer.ps1 logic..." -ForegroundColor Cyan
Write-Host ""

# Simulate the installer logic
$RepoUrl = "https://github.com/AqwozTheDeveloper/crabby.git"
$InstallDir = "$TestDir\.crabby"

if (Test-Path $InstallDir) {
    Write-Host "Directory exists, would update"
    Set-Location $InstallDir
} else {
    Write-Host "Directory doesn't exist, cloning..."
    git clone $RepoUrl $InstallDir
}

Write-Host "Changing to: $InstallDir"
Set-Location $InstallDir

Write-Host "Current directory: $(Get-Location)" -ForegroundColor Green
Write-Host "Files in directory:" -ForegroundColor Green
Get-ChildItem | Select-Object -First 5 Name

# Check if install.ps1 exists
if (Test-Path ".\install.ps1") {
    Write-Host "`n✅ install.ps1 found! Script would work." -ForegroundColor Green
} else {
    Write-Host "`n❌ install.ps1 NOT found! Script would fail." -ForegroundColor Red
}
