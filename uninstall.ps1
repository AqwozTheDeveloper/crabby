# Crabby Package Manager Uninstaller
# Works on Windows with PowerShell

Write-Host "Uninstalling Crabby Package Manager..." -ForegroundColor Cyan
Write-Host ""

$installDir = "$env:USERPROFILE\.crabby"
$binDir = "$installDir\bin"

# Ask for confirmation
Write-Host "This will remove:" -ForegroundColor Yellow
Write-Host "  - Crabby binary from $binDir" -ForegroundColor White
Write-Host "  - Global cache from $installDir\cache" -ForegroundColor White
Write-Host "  - Runtime from $installDir\runtime" -ForegroundColor White
Write-Host ""
$confirmation = Read-Host "Continue? (y/n)"

if ($confirmation -ne 'y' -and $confirmation -ne 'Y') {
    Write-Host "[CANCELLED] Uninstall cancelled" -ForegroundColor Yellow
    exit 0
}

Write-Host ""

# Remove Crabby directory
if (Test-Path $installDir) {
    Write-Host "Removing $installDir..." -ForegroundColor Yellow
    Remove-Item -Path $installDir -Recurse -Force
    Write-Host "[OK] Removed Crabby files" -ForegroundColor Green
} else {
    Write-Host "[INFO] Crabby directory not found" -ForegroundColor Yellow
}

# Remove from PATH
Write-Host "Removing from PATH..." -ForegroundColor Yellow
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -like "*$binDir*") {
    $newPath = ($currentPath -split ';' | Where-Object { $_ -ne $binDir }) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-Host "[OK] Removed from PATH" -ForegroundColor Green
} else {
    Write-Host "[INFO] Not found in PATH" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Uninstall complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Note: You may need to restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
Write-Host ""
Write-Host "Thank you for using Crabby!" -ForegroundColor Cyan
