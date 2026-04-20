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
rtk powershell Get-ChildItem          # Optimized directory listing (70% savings)
rtk powershell Get-ChildItem -Recurse # Recursive directory tree  
rtk powershell Get-Item file.txt      # File/directory info
rtk powershell Copy-Item src dst      # Copy with confirmation output
rtk powershell Move-Item src dst      # Move files/folders
rtk powershell New-Item -Name file    # Create files/folders (80% savings)

# Process management
rtk powershell Get-Process            # Process list with CPU/memory (80% savings)
rtk powershell Get-Process chrome     # Filter specific processes
rtk powershell Start-Process notepad  # Launch processes
rtk powershell Stop-Process notepad   # Kill processes

# Service management
rtk powershell Get-Service            # Service status overview (75% savings)
rtk powershell Get-Service -Name sshd # Specific service info

# File content & search
rtk powershell Get-Content file.txt   # File content reading (60% savings for long files)
rtk powershell Get-Content *.log      # Multiple file content
rtk powershell Select-String pattern  # Text search (grep) - groups by file (70% savings)

# File system utilities
rtk powershell Test-Path path         # Check if path exists
rtk powershell Remove-Item file       # Delete files/directories

# Sorting/Formatting
rtk powershell Sort-Object            # Sort pipeline objects
rtk powershell Format-Table           # Format output as table
rtk powershell Join-Path              # Combine path components

# Output/Utilities
rtk powershell Write-Output text      # Write to output stream
rtk powershell Out-String             # Convert to string

# Windows-specific (NEW!)
rtk powershell Get-WinEvent           # Windows event logs (75% savings)
rtk powershell Get-ItemProperty       # Registry operations (70% savings)
rtk powershell Set-ItemProperty       # Registry modifications
rtk powershell Test-NetConnection     # Network connectivity tests (80% savings)
rtk powershell Get-NetAdapter         # Network adapter info (75% savings)
rtk powershell Get-ComputerInfo       # System information (85% savings)
rtk powershell Get-SystemInfo         # System info alias
rtk powershell Get-Package            # Installed packages (70% savings)
rtk powershell Install-Package        # Package installation
rtk powershell Get-EventLog           # Event log entries (80% savings)
rtk powershell Clear-EventLog         # Clear event logs
rtk powershell Get-Counter            # Performance counters (75% savings)
rtk powershell Get-NetTCPConnection   # TCP connections (70% savings)

# Usage in PowerShell pipelines
Get-ChildItem | rtk powershell Format-Table  # Format output from other commands
Get-Process | rtk powershell Select-String   # Search process output
```

#### Common Use Cases

**Remove /truncate verbose .log files**: Get just first & last 20 lines
```powershell
rtk powershell Get-Content huge-file.log
# Returns first 20 + "... [truncated 5923 lines] ..." + last 20 lines
```

**List all running processes but truncated** to first 50 for token savings
```powershell
rtk powershell Get-Process
# Shows Name, Id, CPU, WorkingSet for top 50 processes
# ... [truncated N more processes] ...
```

**Search large text files efficiently**
```powershell
rtk powershell Select-String "error" *.log
# Shows first 200 matches with file locations
# ... [truncated N more matches] ...
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
