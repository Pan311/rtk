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

## PowerShell Support

RTK now supports PowerShell cmdlets for Windows environments:

```powershell
rtk powershell Get-ChildItem          # Directory listing (ls/dir)
rtk powershell Get-Process            # Process list (ps)
rtk powershell Get-Service            # Service status
rtk powershell Get-Content file.txt   # File content (cat/type)
rtk powershell Get-Item file.txt      # File info
rtk powershell Select-String pattern  # Text search (grep)
rtk powershell Get-WinEvent           # Windows event logs
rtk powershell Start-Process          # Launch processes
rtk powershell Stop-Process           # Kill processes
rtk powershell Test-Path path         # Check if path exists
rtk powershell Remove-Item            # Delete files/directories
rtk powershell Join-Path              # Combine path components
rtk powershell Sort-Object            # Sort pipeline objects
rtk powershell Format-Table           # Format output as table
rtk powershell Write-Output           # Write to output stream
rtk powershell Out-String             # Convert to string
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
