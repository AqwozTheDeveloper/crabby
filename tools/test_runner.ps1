$ErrorActionPreference = "Stop"

function Run-Test {
    param (
        [string]$Name,
        [string]$Command,
        [string]$ExpectedOutput
    )
    
    Write-Host "Running Test: $Name" -ForegroundColor Cyan
    $outputString = ""
    $status = "❌ FAIL"
    
    try {
        # Redirect stdout and stderr to a file or variable
        # For simplicity, we invoke it and capture output
        $process = Start-Process -FilePath "powershell" -ArgumentList "-Command", "$Command" -NoNewWindow -PassThru -Wait -RedirectStandardOutput "temp_stdout.txt" -RedirectStandardError "temp_stderr.txt"
        
        $stdout = Get-Content "temp_stdout.txt" -Raw -ErrorAction SilentlyContinue
        $stderr = Get-Content "temp_stderr.txt" -Raw -ErrorAction SilentlyContinue
        $outputString = "$stdout`n$stderr"
        
        if ($process.ExitCode -eq 0) {
            $status = "✅ PASS"
        }
        
    } catch {
        $outputString = $_.Exception.Message
    }
    
    # Simple formatting for the report
    return @{
        Name = $Name
        Status = $status
        Command = $Command
        Output = $outputString
    }
}

Write-Host "Building Crabby..." -ForegroundColor Yellow
cargo build --quiet

$results = @()

$results += Run-Test -Name "TypeScript File" -Command "cargo run --quiet -- run test.ts"
$results += Run-Test -Name "JavaScript File" -Command "cargo run --quiet -- run simple_test.js"
$results += Run-Test -Name "GUI Spawning" -Command "cargo run --quiet -- run test_gui.ts"

# Generate Markdown Report
$report = "# 🧪 Auto-Generated Test Results`n`n"
$report += "Generated on: $(Get-Date)`n`n"

foreach ($res in $results) {
    $report += "## $($res.Status) - $($res.Name)`n"
    $report += "**Command**: `` $($res.Command) `` `n"
    $report += "### Output`n"
    $report += "````n$($res.Output)`n``` `n`n"
}

$report | Out-File -FilePath "testresults/results.md" -Encoding utf8
Write-Host "Report generated at testresults/results.md" -ForegroundColor Green

Remove-Item "temp_stdout.txt" -ErrorAction SilentlyContinue
Remove-Item "temp_stderr.txt" -ErrorAction SilentlyContinue
