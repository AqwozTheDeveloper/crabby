# Crabby Package Manager Installer
# Works on Windows with PowerShell

Write-Host "Installing Crabby Package Manager..." -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
$rustInstalled = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $rustInstalled) {
    Write-Host "[ERROR] Rust is not installed!" -ForegroundColor Red
    Write-Host "Please install Rust from: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Or run this command in PowerShell:" -ForegroundColor Yellow
    Write-Host "winget install Rustlang.Rustup" -ForegroundColor White
    exit 1
}

$rustVersion = rustc --version
Write-Host "[OK] Rust found: $rustVersion" -ForegroundColor Green
Write-Host ""

# Build Crabby
Write-Host "Building Crabby..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "[OK] Build successful!" -ForegroundColor Green
Write-Host ""

# Determine install location
$installDir = "$env:USERPROFILE\.crabby\bin"
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

# Copy binary
Write-Host "Installing to $installDir..." -ForegroundColor Yellow
Copy-Item "target\release\crabby.exe" "$installDir\" -Force

Write-Host "[OK] Crabby installed!" -ForegroundColor Green
Write-Host ""

# Check if already in PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -like "*$installDir*") {
    Write-Host "[OK] $installDir is already in PATH" -ForegroundColor Green
} else {
    Write-Host "[INFO] Adding Crabby to your PATH..." -ForegroundColor Yellow
    
    # Add to user PATH
    $newPath = "$installDir;$currentPath"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    
    Write-Host "[OK] Added to PATH! Please restart your terminal." -ForegroundColor Green
}

Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Cyan
Write-Host ""
Write-Host "Key Features:" -ForegroundColor White
Write-Host "  - Standalone (No Node.js required!)" -ForegroundColor Green
Write-Host "  - Fast TypeScript execution with tsx" -ForegroundColor Green
Write-Host "  - Full npm ecosystem support" -ForegroundColor Green
Write-Host "  - Global package support (install -g)" -ForegroundColor Green
Write-Host "  - Security auditing (audit)" -ForegroundColor Green
Write-Host ""
Write-Host "Get started:" -ForegroundColor White
Write-Host "  crabby init              # Initialize a new project" -ForegroundColor Cyan
Write-Host "  crabby add react         # Add a package" -ForegroundColor Cyan
Write-Host "  crabby exec tsc --init   # Run binaries (or use 'x')" -ForegroundColor Cyan
Write-Host "  crabby install -g tool   # Install global CLI tools" -ForegroundColor Cyan
Write-Host "  crabby audit             # Check vulnerabilities" -ForegroundColor Cyan
Write-Host "  crabby run src/index.ts  # Run TypeScript files" -ForegroundColor Cyan
Write-Host "  crabby upgrade --self    # Update crabby to latest" -ForegroundColor Cyan
Write-Host ""
Write-Host "Learn more: https://github.com/AqwozTheDeveloper/crabby" -ForegroundColor White
