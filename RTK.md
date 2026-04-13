# RTK - Rust Token Killer (Codex CLI)

**Usage**: Token-optimized CLI proxy for shell commands.

## Rule

Always prefix shell commands with `rtk`.

Examples:

```bash
rtk git status
rtk cargo test
rtk npm run build
rtk pytest -q
```

## Meta Commands

```bash
rtk gain            # Token savings analytics
rtk gain --history  # Recent command savings history
rtk proxy <cmd>     # Run raw command without filtering
```

## Verification

```bash
rtk --version
rtk gain
which rtk
```

## RTK Diagnostics & Logging

### Setup: Add PowerShell Diagnostic Function

Add this to your PowerShell profile (`$PROFILE`):

```powershell
function Check-RTK-Logs {
    $timestamp = Get-Date -Format "yyyy-MM-dd_HHmmss"
    $logFile = "RTK_Diagnostics_$timestamp.txt"
    
    Write-Host "=== RTK Configuration ===" -ForegroundColor Green | Tee-Object -FilePath $logFile -Append
    rtk config | Tee-Object -FilePath $logFile -Append
    
    Write-Host "`n=== RTK Token Savings ===" -ForegroundColor Green | Tee-Object -FilePath $logFile -Append
    rtk gain | Tee-Object -FilePath $logFile -Append
    
    Write-Host "`n=== RTK Full History ===" -ForegroundColor Green | Tee-Object -FilePath $logFile -Append
    rtk gain --history | Tee-Object -FilePath $logFile -Append
    
    Write-Host "`nLog saved to: $PWD\$logFile" -ForegroundColor Cyan
    return $logFile
}
```

### Usage

**Anytime you need to check RTK logs:**

```powershell
Check-RTK-Logs
```

This creates a timestamped log file (e.g., `RTK_Diagnostics_2026-04-13_143022.txt`) containing:
- ✅ RTK configuration (tracking enabled/disabled, log locations)
- ✅ Token savings statistics (total saved, efficiency %)
- ✅ Complete command history (all executed commands with per-command savings)

### Querying Old Logs

RTK data is **permanent** in the database. You can:

1. **Compare logs across days:**
   ```
   @Codex
   Compare these two RTK diagnostic files:
   - RTK_Diagnostics_2026-04-10_150000.txt (Week 1)
   - RTK_Diagnostics_2026-04-17_150000.txt (Week 2)
   
   Show me:
   - Token savings growth
   - Most used commands
   - Performance trends
   ```

2. **Query by date range:**
   ```powershell
   # All logs from today
   Get-ChildItem RTK_Diagnostics_2026-04-13*.txt
   
   # Archive all logs
   Get-ChildItem RTK_Diagnostics_*.txt | Move-Item -Destination ".\logs\"
   ```

### Storage Locations

- **Database**: `$APPDATA\rtk\tracking.db` (permanent, never loses data)
- **Diagnostic logs**: `RTK_Diagnostics_YYYY-MM-DD_HHMMSS.txt` (timestamped snapshots)
- **Raw output**: `$APPDATA\rtk\tee\` (fallback output when filters fail)
