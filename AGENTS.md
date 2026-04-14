# RTK Agents Configuration

@RTK.md

## Quick Diagnostic Commands

When you need to check RTK performance, logs, or troubleshoot:

### Check Current Status
```powershell
# See if RTK is properly installed and configured
rtk --version
rtk config
rtk gain
```

### PowerShell Command Optimization

RTK now supports PowerShell cmdlets for token optimization:

```powershell
# Directory operations
rtk powershell Get-ChildItem          # Optimized directory listing
rtk powershell Get-ChildItem -Recurse # Recursive directory tree

# Process management
rtk powershell Get-Process            # Process list with CPU/memory
rtk powershell Get-Process chrome     # Filter specific processes

# Service management
rtk powershell Get-Service            # Service status overview
rtk powershell Get-Service -Name sshd # Specific service info

# File operations
rtk powershell Get-Content file.txt   # File content reading
rtk powershell Get-Content *.log      # Multiple file content
```

### Full Diagnostics (Recommended)
```powershell
# Add this function to PowerShell profile:
# function Check-RTK-Logs { ... }
# (See RTK.md for full function definition)

# Then run:
Check-RTK-Logs
```

This creates a timestamped log file with complete RTK diagnostics (configuration, token savings, command history).

### With Codex CLI

Ask Codex to analyze RTK logs:

```
@Codex
Run Check-RTK-Logs and show me:
1. Current privacy settings (is tracking disabled?)
2. Total token savings achieved
3. Top 5 most used commands
```

## Referencing Logs in Codex

You can reference diagnostic files directly:

```
@Codex
Here's my RTK diagnostic log: RTK_Diagnostics_2026-04-13_143022.txt

Based on this log, recommend optimizations to save more tokens.
```

## Historical Analysis

Compare logs across time periods:

```
@Codex
Compare:
1. RTK_Diagnostics_2026-04-10_150000.txt (Week 1)
2. RTK_Diagnostics_2026-04-17_150000.txt (Week 2)

Show improvement trends and analysis.
```
