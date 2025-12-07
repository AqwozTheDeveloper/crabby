#!/bin/bash
# Test Runner for Crabby (Unix)

echo "🧪 Testing Crabby Package Manager"
echo "================================="
echo ""

tests_passed=0
tests_failed=0

test_command() {
    local name=$1
    local command=$2
    
    echo "Testing: $name"
    echo "Command: $command"
    
    if eval "$command"; then
        echo "✅ PASSED"
        ((tests_passed++))
    else
        echo "❌ FAILED (Exit code: $?)"
        ((tests_failed++))
    fi
    echo ""
}

# Build Crabby first
echo "Building Crabby..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Build failed! Cannot run tests."
    exit 1
fi
echo ""

crabby="./target/release/crabby"

# Test 1: Version
test_command "Version Command" "$crabby --version"

# Test 2: Help
test_command "Help Command" "$crabby --help"

# Test 3: Init
echo "Testing: Init Command"
rm -rf test_project
mkdir test_project
cd test_project
$crabby init
if [ $? -eq 0 ] && [ -f "package.json" ]; then
    echo "✅ PASSED"
    ((tests_passed++))
else
    echo "❌ FAILED"
    ((tests_failed++))
fi
cd ..
echo ""

# Test 4: Install
echo "Testing: Install Specific Package"
cd test_project
$crabby install left-pad
if [ $? -eq 0 ] && [ -d "node_modules/left-pad" ]; then
    echo "✅ PASSED"
    ((tests_passed++))
else
    echo "❌ FAILED"
    ((tests_failed++))
fi
cd ..
echo ""

# Test 5: List
test_command "List Command" "cd test_project && $crabby list && cd .."

# Test 6: Info
test_command "Info Command" "$crabby info express"

# Test 7: Run TypeScript
echo "Testing: Run TypeScript File"
echo "console.log('TypeScript works!');" > test_project/test.ts
cd test_project
$crabby run test.ts
if [ $? -eq 0 ]; then
    echo "✅ PASSED"
    ((tests_passed++))
else
    echo "❌ FAILED"
    ((tests_failed++))
fi
cd ..
echo ""

# Test 8: Run JavaScript
echo "Testing: Run JavaScript File"
echo "console.log('JavaScript works!');" > test_project/test.js
cd test_project
$crabby run test.js
if [ $? -eq 0 ]; then
    echo "✅ PASSED"
    ((tests_passed++))
else
    echo "❌ FAILED"
    ((tests_failed++))
fi
cd ..
echo ""

# Cleanup
echo "Cleaning up test project..."
rm -rf test_project

# Summary
echo ""
echo "================================="
echo "Test Results"
echo "================================="
echo "Passed: $tests_passed"
echo "Failed: $tests_failed"
echo ""

if [ $tests_failed -eq 0 ]; then
    echo "🎉 All tests passed!"
    exit 0
else
    echo "⚠️  Some tests failed"
    exit 1
fi
