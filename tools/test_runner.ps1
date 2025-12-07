# Test Runner for Crabby
# This script tests all major features of Crabby

Write-Host "🧪 Testing Crabby Package Manager" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Continue"
$testsPassed = 0
$testsFailed = 0

function Test-Command {
    param(
        [string]$Name,
        [string]$Command
    )
    
    Write-Host "Testing: $Name" -ForegroundColor Yellow
    Write-Host "Command: $Command" -ForegroundColor Gray
    
    try {
        Invoke-Expression $Command
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ PASSED" -ForegroundColor Green
            $script:testsPassed++
        } else {
            Write-Host "❌ FAILED (Exit code: $LASTEXITCODE)" -ForegroundColor Red
            $script:testsFailed++
        }
    } catch {
        Write-Host "❌ FAILED (Exception: $_)" -ForegroundColor Red
        $script:testsFailed++
    }
    Write-Host ""
}

# Build Crabby first
Write-Host "Building Crabby..." -ForegroundColor Cyan
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed! Cannot run tests." -ForegroundColor Red
    exit 1
}
Write-Host ""

$crabby = ".\target\release\crabby.exe"

# Test 1: Version
Test-Command "Version Command" "$crabby --version"

# Test 2: Help
Test-Command "Help Command" "$crabby --help"

# Test 3: Init
Write-Host "Testing: Init Command" -ForegroundColor Yellow
Remove-Item -Path "test_project" -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path "test_project" | Out-Null
Push-Location "test_project"
& $crabby init
if ($LASTEXITCODE -eq 0 -and (Test-Path "package.json")) {
    Write-Host "✅ PASSED" -ForegroundColor Green
    $testsPassed++
} else {
    Write-Host "❌ FAILED" -ForegroundColor Red
    $testsFailed++
}
Pop-Location
Write-Host ""

# Test 4: Install specific package
Write-Host "Testing: Install Specific Package" -ForegroundColor Yellow
Push-Location "test_project"
& $crabby install left-pad
if ($LASTEXITCODE -eq 0 -and (Test-Path "node_modules\left-pad")) {
    Write-Host "✅ PASSED" -ForegroundColor Green
    $testsPassed++
} else {
    Write-Host "❌ FAILED" -ForegroundColor Red
    $testsFailed++
}
Pop-Location
Write-Host ""

# Test 5: List packages
Test-Command "List Command" "Push-Location test_project; $crabby list; Pop-Location"

# Test 6: Info command
Test-Command "Info Command" "$crabby info express"

# Test 7: Run TypeScript
Write-Host "Testing: Run TypeScript File" -ForegroundColor Yellow
Set-Content -Path "test_project\test.ts" -Value "console.log('TypeScript works!');"
Push-Location "test_project"
& $crabby run test.ts
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ PASSED" -ForegroundColor Green
    $testsPassed++
} else {
    Write-Host "❌ FAILED" -ForegroundColor Red
    $testsFailed++
}
Pop-Location
Write-Host ""

# Test 8: Run JavaScript
Write-Host "Testing: Run JavaScript File" -ForegroundColor Yellow
Set-Content -Path "test_project\test.js" -Value "console.log('JavaScript works!');"
Push-Location "test_project"
& $crabby run test.js
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ PASSED" -ForegroundColor Green
    $testsPassed++
} else {
    Write-Host "❌ FAILED" -ForegroundColor Red
    $testsFailed++
}
Pop-Location
Write-Host ""

# Test 9: Remove package
Write-Host "Testing: Remove Package" -ForegroundColor Yellow
Push-Location "test_project"
& $crabby remove left-pad
if ($LASTEXITCODE -eq 0 -and !(Test-Path "node_modules\left-pad")) {
    Write-Host "✅ PASSED" -ForegroundColor Green
    $testsPassed++
} else {
    Write-Host "❌ FAILED" -ForegroundColor Red
    $testsFailed++
}
Pop-Location
Write-Host ""

# Cleanup
Write-Host "Cleaning up test project..." -ForegroundColor Gray
Remove-Item -Path "test_project" -Recurse -Force -ErrorAction SilentlyContinue

# Summary
Write-Host ""
Write-Host "=================================" -ForegroundColor Cyan
Write-Host "Test Results" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host "Passed: $testsPassed" -ForegroundColor Green
Write-Host "Failed: $testsFailed" -ForegroundColor Red
Write-Host ""

if ($testsFailed -eq 0) {
    Write-Host "🎉 All tests passed!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "⚠️  Some tests failed" -ForegroundColor Yellow
    exit 1
}
