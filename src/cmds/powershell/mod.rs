//! PowerShell cmdlet routing and filtering.
//!
//! Detects PowerShell cmdlets and applies appropriate filters for token optimization.

use crate::core::utils::resolved_command;
use anyhow::{Context, Result};

/// Route PowerShell cmdlets to appropriate filters
pub fn run(args: &[String], verbose: u8) -> Result<i32> {
    if args.is_empty() {
        return run_powershell_passthrough(&[], verbose);
    }

    match args[0].as_str() {
        // Directory/File operations
        "Get-ChildItem" | "gci" | "ls" | "dir" => run_get_childitem(&args[1..], verbose),
        "Get-Item" | "gi" => run_get_item(&args[1..], verbose),
        "Get-Content" | "gc" | "cat" | "type" => run_get_content(&args[1..], verbose),
        "Copy-Item" | "cp" | "copy" => run_copy_item(&args[1..], verbose),
        "Move-Item" | "mi" | "mv" | "move" => run_move_item(&args[1..], verbose),
        "New-Item" | "ni" => run_new_item(&args[1..], verbose),

        // Process management
        "Get-Process" | "gps" | "ps" => run_get_process(&args[1..], verbose),
        "Start-Process" | "saps" | "start" => run_start_process(&args[1..], verbose),
        "Stop-Process" | "kill" | "spps" => run_stop_process(&args[1..], verbose),

        // Service management
        "Get-Service" | "gsv" => run_get_service(&args[1..], verbose),

        // Search/Text
        "Select-String" | "sls" => run_select_string(&args[1..], verbose),

        // Events/Diagnostics
        "Get-WinEvent" | "gwe" => run_get_winevent(&args[1..], verbose),

        // Navigation
        "Set-Location" | "sl" | "cd" => run_set_location(&args[1..], verbose),
        "Get-Location" | "gl" | "pwd" => run_get_location(&args[1..], verbose),

        // Path utilities
        "Test-Path" | "test" => run_test_path(&args[1..], verbose),
        "Join-Path" => run_join_path(&args[1..], verbose),

        // Sorting/Formatting
        "Sort-Object" | "sort" => run_sort_object(&args[1..], verbose),
        "Format-Table" | "ft" => run_format_table(&args[1..], verbose),

        // Output
        "Write-Output" | "write" | "echo" => run_write_output(&args[1..], verbose),
        "Out-String" => run_out_string(&args[1..], verbose),

        // File system utilities
        "Remove-Item" | "ri" | "rm" | "del" | "erase" => run_remove_item(&args[1..], verbose),

        // 🔥 HIGH-VALUE ADDITIONS (similar to what we just added)

        // Registry operations (Windows-specific, high token savings potential)
        "Get-ItemProperty" | "gp" => run_get_itemproperty(&args[1..], verbose),
        "Set-ItemProperty" | "sp" => run_set_itemproperty(&args[1..], verbose),

        // Networking (similar to curl/wget already supported)
        "Test-NetConnection" | "tnc" => run_test_netconnection(&args[1..], verbose),
        "Get-NetAdapter" => run_get_netadapter(&args[1..], verbose),

        // System information (similar to env/system commands)
        "Get-ComputerInfo" => run_get_computerinfo(&args[1..], verbose),
        "Get-SystemInfo" => run_get_systeminfo(&args[1..], verbose),

        // Package management (similar to pnpm/npm already supported)
        "Get-Package" => run_get_package(&args[1..], verbose),
        "Install-Package" => run_install_package(&args[1..], verbose),

        // Event logs (expand Get-WinEvent)
        "Get-EventLog" => run_get_eventlog(&args[1..], verbose),
        "Clear-EventLog" => run_clear_eventlog(&args[1..], verbose),

        // Performance monitoring (similar to Get-Process)
        "Get-Counter" => run_get_counter(&args[1..], verbose),
        "Get-NetTCPConnection" => run_get_nettcpconnection(&args[1..], verbose),

        // Default passthrough for unsupported cmdlets
        _ => run_powershell_passthrough(args, verbose),
    }
}

/// Filter Get-ChildItem (ls/dir) output
fn run_get_childitem(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = if has_pipeline(args) {
        build_powershell_command("Get-ChildItem", args)
    } else {
        format!(
            "{} | Format-Table -AutoSize",
            build_powershell_command("Get-ChildItem", args)
        )
    };

    let output = execute_powershell_command(&command, "Get-ChildItem")?;
    let filtered = filter_get_childitem_output(&output.stdout);

    timer.track(
        "Get-ChildItem",
        "rtk Get-ChildItem",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-Process output
fn run_get_process(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let base = build_powershell_command("Get-Process", args);
    let command = if has_pipeline(args) {
        base
    } else {
        format!(
            "{} | Select-Object Name,Id,CPU,WorkingSet | Format-Table -AutoSize",
            base
        )
    };

    let output = execute_powershell_command(&command, "Get-Process")?;
    let filtered = filter_get_process_output(&output.stdout);

    timer.track("Get-Process", "rtk Get-Process", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-Service output
fn run_get_service(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let base = build_powershell_command("Get-Service", args);
    let command = if has_pipeline(args) {
        base
    } else {
        format!(
            "{} | Select-Object Name,DisplayName,Status | Format-Table -AutoSize",
            base
        )
    };

    let output = execute_powershell_command(&command, "Get-Service")?;
    let filtered = filter_get_service_output(&output.stdout);

    timer.track("Get-Service", "rtk Get-Service", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Copy-Item output
fn run_copy_item(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Copy-Item", args);

    let output = execute_powershell_command(&command, "Copy-Item")?;
    let filtered = filter_copy_item_output(&output.stdout);

    timer.track("Copy-Item", "rtk Copy-Item", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Move-Item output
fn run_move_item(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Move-Item", args);

    let output = execute_powershell_command(&command, "Move-Item")?;
    let filtered = filter_move_item_output(&output.stdout);

    timer.track("Move-Item", "rtk Move-Item", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter New-Item output
fn run_new_item(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("New-Item", args);

    let output = execute_powershell_command(&command, "New-Item")?;
    let filtered = filter_new_item_output(&output.stdout);

    timer.track("New-Item", "rtk New-Item", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-Content output
fn run_get_content(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Get-Content", args);

    let output = execute_powershell_command(&command, "Get-Content")?;
    let filtered = filter_get_content_output(&output.stdout);

    timer.track("Get-Content", "rtk Get-Content", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-Item output
fn run_get_item(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let base = build_powershell_command("Get-Item", args);
    let command = if has_pipeline(args) {
        base
    } else {
        format!(
            "{} | Select-Object FullName,Length,LastWriteTime,Mode | Format-Table -AutoSize",
            base
        )
    };

    let output = execute_powershell_command(&command, "Get-Item")?;
    let filtered = filter_get_item_output(&output.stdout);

    timer.track("Get-Item", "rtk Get-Item", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Select-String output
fn run_select_string(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Select-String", args);

    let output = execute_powershell_command(&command, "Select-String")?;
    let filtered = filter_select_string_output(&output.stdout);

    timer.track(
        "Select-String",
        "rtk Select-String",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-WinEvent output
fn run_get_winevent(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let base = build_powershell_command("Get-WinEvent", args);
    let command = if has_pipeline(args) {
        base
    } else {
        format!("{} | Select-Object TimeCreated,Id,LevelDisplayName,ProviderName,Message | Format-Table -AutoSize -Wrap", base)
    };

    let output = execute_powershell_command(&command, "Get-WinEvent")?;
    let filtered = filter_get_winevent_output(&output.stdout);

    timer.track(
        "Get-WinEvent",
        "rtk Get-WinEvent",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Handle Set-Location (usually just passthrough)
fn run_set_location(args: &[String], verbose: u8) -> Result<i32> {
    let mut all_args = vec!["Set-Location".to_string()];
    all_args.extend(args.iter().cloned());
    run_powershell_passthrough(&all_args, verbose)
}

/// Handle Get-Location (usually just passthrough)
fn run_get_location(args: &[String], verbose: u8) -> Result<i32> {
    let mut all_args = vec!["Get-Location".to_string()];
    all_args.extend(args.iter().cloned());
    run_powershell_passthrough(&all_args, verbose)
}

/// Filter Start-Process output
fn run_start_process(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Start-Process", args);

    let output = execute_powershell_command(&command, "Start-Process")?;
    let filtered = filter_start_process_output(&output.stdout);

    timer.track(
        "Start-Process",
        "rtk Start-Process",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Stop-Process output
fn run_stop_process(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Stop-Process", args);

    let output = execute_powershell_command(&command, "Stop-Process")?;
    let filtered = filter_stop_process_output(&output.stdout);

    timer.track(
        "Stop-Process",
        "rtk Stop-Process",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Test-Path output
fn run_test_path(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Test-Path", args);

    let output = execute_powershell_command(&command, "Test-Path")?;
    let filtered = filter_test_path_output(&output.stdout);

    timer.track("Test-Path", "rtk Test-Path", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Remove-Item output
fn run_remove_item(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Remove-Item", args);

    let output = execute_powershell_command(&command, "Remove-Item")?;
    let filtered = filter_remove_item_output(&output.stdout);

    timer.track("Remove-Item", "rtk Remove-Item", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Join-Path output
fn run_join_path(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Join-Path", args);

    let output = execute_powershell_command(&command, "Join-Path")?;
    let filtered = filter_join_path_output(&output.stdout);

    timer.track("Join-Path", "rtk Join-Path", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Sort-Object output
fn run_sort_object(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Sort-Object", args);

    let output = execute_powershell_command(&command, "Sort-Object")?;
    let filtered = filter_sort_object_output(&output.stdout);

    timer.track("Sort-Object", "rtk Sort-Object", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Format-Table output
fn run_format_table(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Format-Table", args);

    let output = execute_powershell_command(&command, "Format-Table")?;
    let filtered = filter_format_table_output(&output.stdout);

    timer.track(
        "Format-Table",
        "rtk Format-Table",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Write-Output output
fn run_write_output(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Write-Output", args);

    let output = execute_powershell_command(&command, "Write-Output")?;
    let filtered = filter_write_output_output(&output.stdout);

    timer.track(
        "Write-Output",
        "rtk Write-Output",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Out-String output
fn run_out_string(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Out-String", args);

    let output = execute_powershell_command(&command, "Out-String")?;
    let filtered = filter_out_string_output(&output.stdout);

    timer.track("Out-String", "rtk Out-String", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// 🔥 NEW HIGH-VALUE POWERSHELL CMDLETS (similar to existing pattern)
/// Filter Get-ItemProperty output (Registry operations)
fn run_get_itemproperty(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let base = build_powershell_command("Get-ItemProperty", args);
    let command = if has_pipeline(args) {
        base
    } else {
        format!("{} | Format-Table -AutoSize", base)
    };

    let output = execute_powershell_command(&command, "Get-ItemProperty")?;
    let filtered = filter_get_itemproperty_output(&output.stdout);

    timer.track(
        "Get-ItemProperty",
        "rtk Get-ItemProperty",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Set-ItemProperty output
fn run_set_itemproperty(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Set-ItemProperty", args);

    let output = execute_powershell_command(&command, "Set-ItemProperty")?;
    let filtered = filter_set_itemproperty_output(&output.stdout);

    timer.track(
        "Set-ItemProperty",
        "rtk Set-ItemProperty",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Test-NetConnection output
fn run_test_netconnection(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Test-NetConnection", args);

    let output = execute_powershell_command(&command, "Test-NetConnection")?;
    let filtered = filter_test_netconnection_output(&output.stdout);

    timer.track(
        "Test-NetConnection",
        "rtk Test-NetConnection",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-NetAdapter output
fn run_get_netadapter(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let base = build_powershell_command("Get-NetAdapter", args);
    let command = if has_pipeline(args) {
        base
    } else {
        format!("{} | Select-Object Name,InterfaceDescription,Status,LinkSpeed | Format-Table -AutoSize", base)
    };

    let output = execute_powershell_command(&command, "Get-NetAdapter")?;
    let filtered = filter_get_netadapter_output(&output.stdout);

    timer.track(
        "Get-NetAdapter",
        "rtk Get-NetAdapter",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-ComputerInfo output
fn run_get_computerinfo(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let base = build_powershell_command("Get-ComputerInfo", args);
    let command = if has_pipeline(args) {
        base
    } else {
        format!("{} | Select-Object CsName,CsDomain,CsManufacturer,CsModel,OsName,OsVersion,OsArchitecture | Format-Table -AutoSize", base)
    };

    let output = execute_powershell_command(&command, "Get-ComputerInfo")?;
    let filtered = filter_get_computerinfo_output(&output.stdout);

    timer.track(
        "Get-ComputerInfo",
        "rtk Get-ComputerInfo",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-SystemInfo output (alias for Get-ComputerInfo)
fn run_get_systeminfo(args: &[String], verbose: u8) -> Result<i32> {
    run_get_computerinfo(args, verbose)
}

/// Filter Get-Package output
fn run_get_package(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let base = build_powershell_command("Get-Package", args);
    let command = if has_pipeline(args) {
        base
    } else {
        format!(
            "{} | Select-Object Name,Version,ProviderName,Status | Format-Table -AutoSize",
            base
        )
    };

    let output = execute_powershell_command(&command, "Get-Package")?;
    let filtered = filter_get_package_output(&output.stdout);

    timer.track("Get-Package", "rtk Get-Package", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Install-Package output
fn run_install_package(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Install-Package", args);

    let output = execute_powershell_command(&command, "Install-Package")?;
    let filtered = filter_install_package_output(&output.stdout);

    timer.track(
        "Install-Package",
        "rtk Install-Package",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-EventLog output
fn run_get_eventlog(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let base = build_powershell_command("Get-EventLog", args);
    let command = if has_pipeline(args) {
        base
    } else {
        format!("{} | Select-Object TimeGenerated,EntryType,Source,Message | Format-Table -AutoSize -Wrap", base)
    };

    let output = execute_powershell_command(&command, "Get-EventLog")?;
    let filtered = filter_get_eventlog_output(&output.stdout);

    timer.track(
        "Get-EventLog",
        "rtk Get-EventLog",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Clear-EventLog output
fn run_clear_eventlog(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Clear-EventLog", args);

    let output = execute_powershell_command(&command, "Clear-EventLog")?;
    let filtered = filter_clear_eventlog_output(&output.stdout);

    timer.track(
        "Clear-EventLog",
        "rtk Clear-EventLog",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-Counter output
fn run_get_counter(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command = build_powershell_command("Get-Counter", args);

    let output = execute_powershell_command(&command, "Get-Counter")?;
    let filtered = filter_get_counter_output(&output.stdout);

    timer.track("Get-Counter", "rtk Get-Counter", &output.stdout, &filtered);
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Filter Get-NetTCPConnection output
fn run_get_nettcpconnection(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let base = build_powershell_command("Get-NetTCPConnection", args);
    let command = if has_pipeline(args) {
        base
    } else {
        format!("{} | Select-Object LocalAddress,LocalPort,RemoteAddress,RemotePort,State,OwningProcess | Format-Table -AutoSize", base)
    };

    let output = execute_powershell_command(&command, "Get-NetTCPConnection")?;
    let filtered = filter_get_nettcpconnection_output(&output.stdout);

    timer.track(
        "Get-NetTCPConnection",
        "rtk Get-NetTCPConnection",
        &output.stdout,
        &filtered,
    );
    print!("{}", filtered);

    Ok(output.exit_code)
}

/// Generic PowerShell passthrough for unsupported cmdlets
fn run_powershell_passthrough(args: &[String], _verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();
    let command =
        build_powershell_command(args.first().map(String::as_str).unwrap_or(""), &args[1..]);

    let mut cmd = resolved_command("powershell.exe");
    cmd.arg("-Command").arg(&command);

    let output = cmd.output().context("Failed to run PowerShell command")?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        eprintln!("{}", stderr);
        return Ok(crate::core::utils::exit_code_from_output(
            &output,
            "powershell",
        ));
    }

    timer.track_passthrough(
        &format!("powershell {}", command),
        &format!("rtk powershell {} (passthrough)", command),
    );

    print!("{}", stdout);

    Ok(crate::core::utils::exit_code_from_output(
        &output,
        "powershell",
    ))
}

struct PowerShellOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

fn execute_powershell_command(command: &str, name: &str) -> Result<PowerShellOutput> {
    let mut cmd = resolved_command("powershell.exe");
    cmd.arg("-Command").arg(command);

    let output = cmd
        .output()
        .with_context(|| format!("Failed to run {}", name))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = crate::core::utils::exit_code_from_output(&output, name);

    if !output.status.success() {
        eprintln!("{}", stderr);
    }

    Ok(PowerShellOutput {
        stdout,
        stderr,
        exit_code,
    })
}

fn build_powershell_command(base: &str, args: &[String]) -> String {
    let mut command = base.to_string();

    for arg in args {
        if !command.is_empty() {
            command.push(' ');
        }
        command.push_str(&quote_powershell_arg(arg));
    }

    command
}

fn quote_powershell_arg(arg: &str) -> String {
    if arg.is_empty() {
        return "''".to_string();
    }

    let needs_quotes = arg.chars().any(|c| {
        c.is_whitespace()
            || matches!(
                c,
                '"' | '\'' | '`' | '$' | '&' | '|' | ';' | '(' | ')' | '[' | ']' | '{' | '}' | '*'
            )
    });
    if needs_quotes {
        let escaped = arg.replace('\'', "''");
        return format!("'{}'", escaped);
    }

    arg.to_string()
}

fn has_pipeline(args: &[String]) -> bool {
    args.iter().any(|arg| {
        arg == "|"
            || arg == "Format-Table"
            || arg == "Select-Object"
            || arg == "Where-Object"
            || arg == "Sort-Object"
            || arg == "Group-Object"
    })
}

/// Filter Get-ChildItem output to reduce tokens by extracting key attributes
fn filter_get_childitem_output(output: &str) -> String {
    let lines: Vec<&str> = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    if lines.is_empty() {
        return String::new();
    }

    let mut result = Vec::new();
    let mut header_found = false;

    for line in lines {
        // Skip directory hints and empty lines
        if line.starts_with("Directory:") || line.starts_with("Mode") && !header_found {
            if line.starts_with("Mode") {
                result.push(line.to_string());
                header_found = true;
            }
            continue;
        }

        // Include header and all items after
        if header_found {
            // Skip separator line (----  ------  ---  ----)
            if !line.starts_with("----") && !line.is_empty() {
                result.push(line.to_string());
            }
        }
    }

    result.join("\n")
}

/// Filter Copy-Item output (typically minimal/empty, just compress)
fn filter_copy_item_output(output: &str) -> String {
    output.trim().to_string()
}

/// Filter Move-Item output (typically minimal/empty, just compress)
fn filter_move_item_output(output: &str) -> String {
    output.trim().to_string()
}

/// Filter New-Item output (just keep the created item path)
fn filter_new_item_output(output: &str) -> String {
    let lines: Vec<&str> = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("Mode"))
        .collect();

    if lines.is_empty() {
        return output.to_string();
    }

    // Return only the last line which contains the item info
    lines.last().map(|s| s.to_string()).unwrap_or_default()
}

/// Filter Get-Process output - compact format for better token savings
fn filter_get_process_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 2 {
        return output.to_string();
    }

    let mut result = Vec::new();
    let mut header_found = false;
    let mut line_count = 0;

    for &line in &lines {
        if line.contains("Name") && line.contains("Id") && !header_found {
            result.push(line.to_string());
            header_found = true;
        } else if header_found && !line.trim().is_empty() && !line.starts_with("---") {
            result.push(line.to_string());
            line_count += 1;
            // Limit to 50 processes for token savings (80%+ savings on typical output)
            if line_count >= 50 {
                result.push(format!(
                    "... [truncated {} more processes] ...",
                    lines.len() - result.len()
                ));
                break;
            }
        }
    }

    result.join("\n")
}

/// Filter Get-Service output - compact format for better token savings
fn filter_get_service_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 2 {
        return output.to_string();
    }

    let mut result = Vec::new();
    let mut header_found = false;
    let mut line_count = 0;

    for &line in &lines {
        if (line.contains("Name") && line.contains("Status"))
            || (line.contains("Status") && line.contains("DisplayName")) && !header_found
        {
            result.push(line.to_string());
            header_found = true;
        } else if header_found && !line.trim().is_empty() && !line.starts_with("---") {
            result.push(line.to_string());
            line_count += 1;
            // Limit to 100 services for token savings
            if line_count >= 100 {
                result.push(format!(
                    "... [truncated {} more services] ...",
                    lines.len() - result.len()
                ));
                break;
            }
        }
    }

    result.join("\n")
}

/// Filter Get-Item output
fn filter_get_item_output(output: &str) -> String {
    let lines: Vec<&str> = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    if lines.len() <= 2 {
        return output.to_string();
    }

    let mut result = Vec::new();
    let mut header_found = false;
    for line in lines {
        if line.contains("FullName") && line.contains("LastWriteTime") && !header_found {
            result.push(line.to_string());
            header_found = true;
        } else if header_found && !line.trim().is_empty() {
            result.push(line.to_string());
        }
    }

    if result.is_empty() {
        return output.to_string();
    }

    result.join("\n")
}

/// Filter Select-String output
fn filter_select_string_output(output: &str) -> String {
    let lines: Vec<&str> = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    if lines.len() <= 250 {
        return lines.join("\n");
    }

    let mut result = Vec::new();
    result.extend(lines.iter().take(200).map(|line| (*line).to_string()));
    result.push(format!("... [truncated {} matches] ...", lines.len() - 200));
    result.join("\n")
}

/// Filter Get-WinEvent output
fn filter_get_winevent_output(output: &str) -> String {
    let lines: Vec<&str> = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    if lines.len() <= 200 {
        return lines.join("\n");
    }

    let mut result = Vec::new();
    result.extend(lines.iter().take(150).map(|line| (*line).to_string()));
    result.push(format!("... [truncated {} events] ...", lines.len() - 150));
    result.join("\n")
}

/// Filter Get-Content output
fn filter_get_content_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 500 {
        return output.to_string();
    }

    let mut result = Vec::new();
    result.extend(lines.iter().take(20).map(|line| (*line).to_string()));
    result.push(format!("... [truncated {} lines] ...", lines.len() - 40));
    result.extend(
        lines
            .iter()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .map(|line| (*line).to_string()),
    );
    result.join("\n")
}

/// Filter Start-Process output
fn filter_start_process_output(output: &str) -> String {
    // Start-Process typically produces minimal output, return as-is
    output.to_string()
}

/// Filter Stop-Process output
fn filter_stop_process_output(output: &str) -> String {
    // Stop-Process typically produces minimal output, return as-is
    output.to_string()
}

/// Filter Test-Path output
fn filter_test_path_output(output: &str) -> String {
    // Test-Path returns boolean, minimal output
    output.to_string()
}

/// Filter Remove-Item output
fn filter_remove_item_output(output: &str) -> String {
    // Remove-Item typically produces no output on success
    output.to_string()
}

/// Filter Join-Path output
fn filter_join_path_output(output: &str) -> String {
    // Join-Path returns path string, minimal output
    output.to_string()
}

/// Filter Sort-Object output
fn filter_sort_object_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 100 {
        return output.to_string();
    }

    let mut result = Vec::new();
    result.extend(lines.iter().take(50).map(|line| (*line).to_string()));
    result.push(format!(
        "... [truncated {} sorted items] ...",
        lines.len() - 100
    ));
    result.extend(
        lines
            .iter()
            .rev()
            .take(50)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .map(|line| (*line).to_string()),
    );
    result.join("\n")
}

/// Filter Format-Table output
fn filter_format_table_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 50 {
        return output.to_string();
    }

    let mut result = Vec::new();
    result.extend(lines.iter().take(25).map(|line| (*line).to_string()));
    result.push(format!(
        "... [truncated {} formatted rows] ...",
        lines.len() - 50
    ));
    result.extend(
        lines
            .iter()
            .rev()
            .take(25)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .map(|line| (*line).to_string()),
    );
    result.join("\n")
}

/// Filter Write-Output output
fn filter_write_output_output(output: &str) -> String {
    // Write-Output passes through, minimal filtering needed
    output.to_string()
}

/// Filter Out-String output
fn filter_out_string_output(output: &str) -> String {
    // Out-String converts to string, return as-is
    output.to_string()
}

/// 🔥 NEW FILTER FUNCTIONS FOR HIGH-VALUE CMDLETS
/// Filter Get-ItemProperty output (Registry values)
fn filter_get_itemproperty_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 2 {
        return output.to_string();
    }

    let mut result = Vec::new();
    let mut header_found = false;

    for &line in &lines {
        if line.contains("PSPath") || line.contains("PSParentPath") && !header_found {
            result.push(line.to_string());
            header_found = true;
        } else if header_found && !line.trim().is_empty() && !line.starts_with("---") {
            // Skip empty registry values, keep only meaningful entries
            if !line.trim().is_empty() {
                result.push(line.to_string());
            }
        }
    }

    result.join("\n")
}

/// Filter Set-ItemProperty output (minimal output)
fn filter_set_itemproperty_output(output: &str) -> String {
    output.trim().to_string()
}

/// Filter Test-NetConnection output
fn filter_test_netconnection_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.is_empty() {
        return output.to_string();
    }

    let mut result = Vec::new();
    for &line in &lines {
        // Keep only essential connectivity info
        if line.contains("ComputerName")
            || line.contains("RemoteAddress")
            || line.contains("PingSucceeded")
            || line.contains("TcpTestSucceeded")
        {
            result.push(line.to_string());
        }
    }

    if result.is_empty() {
        return output.to_string();
    }
    result.join("\n")
}

/// Filter Get-NetAdapter output
fn filter_get_netadapter_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 2 {
        return output.to_string();
    }

    let mut result = Vec::new();
    let mut header_found = false;
    let mut line_count = 0;

    for &line in &lines {
        if (line.contains("Name") && line.contains("Status")) && !header_found {
            result.push(line.to_string());
            header_found = true;
        } else if header_found && !line.trim().is_empty() && !line.starts_with("---") {
            result.push(line.to_string());
            line_count += 1;
            // Limit to 20 adapters for token savings
            if line_count >= 20 {
                result.push(format!(
                    "... [truncated {} more adapters] ...",
                    lines.len() - result.len()
                ));
                break;
            }
        }
    }

    result.join("\n")
}

/// Filter Get-ComputerInfo output
fn filter_get_computerinfo_output(output: &str) -> String {
    // Computer info is usually a single object, return key fields only
    let lines: Vec<&str> = output.lines().collect();
    if lines.is_empty() {
        return output.to_string();
    }

    let mut result = Vec::new();
    for &line in &lines {
        // Keep only essential system identification fields
        if line.contains("CsName")
            || line.contains("CsDomain")
            || line.contains("OsName")
            || line.contains("OsVersion")
            || line.contains("OsArchitecture")
        {
            result.push(line.to_string());
        }
    }

    if result.is_empty() {
        return output.to_string();
    }
    result.join("\n")
}

/// Filter Get-Package output
fn filter_get_package_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 2 {
        return output.to_string();
    }

    let mut result = Vec::new();
    let mut header_found = false;
    let mut line_count = 0;

    for &line in &lines {
        if (line.contains("Name") && line.contains("Version")) && !header_found {
            result.push(line.to_string());
            header_found = true;
        } else if header_found && !line.trim().is_empty() && !line.starts_with("---") {
            result.push(line.to_string());
            line_count += 1;
            // Limit to 50 packages for token savings
            if line_count >= 50 {
                result.push(format!(
                    "... [truncated {} more packages] ...",
                    lines.len() - result.len()
                ));
                break;
            }
        }
    }

    result.join("\n")
}

/// Filter Install-Package output
fn filter_install_package_output(output: &str) -> String {
    // Installation output can be verbose, keep only success/failure status
    let lines: Vec<&str> = output.lines().collect();
    if lines.is_empty() {
        return output.to_string();
    }

    let mut result = Vec::new();
    for &line in &lines {
        // Keep only status and completion messages
        if line.contains("Status")
            || line.contains("completed")
            || line.contains("installed")
            || line.contains("failed")
            || line.contains("error")
        {
            result.push(line.to_string());
        }
    }

    if result.is_empty() {
        return output.to_string();
    }
    result.join("\n")
}

/// Filter Get-EventLog output
fn filter_get_eventlog_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 200 {
        return lines.join("\n");
    }

    let mut result = Vec::new();
    result.extend(lines.iter().take(150).map(|line| (*line).to_string()));
    result.push(format!(
        "... [truncated {} more event log entries] ...",
        lines.len() - 150
    ));
    result.join("\n")
}

/// Filter Clear-EventLog output
fn filter_clear_eventlog_output(output: &str) -> String {
    output.trim().to_string()
}

/// Filter Get-Counter output
fn filter_get_counter_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 50 {
        return output.to_string();
    }

    let mut result = Vec::new();
    result.extend(lines.iter().take(25).map(|line| (*line).to_string()));
    result.push(format!(
        "... [truncated {} more performance counters] ...",
        lines.len() - 25
    ));
    result.extend(
        lines
            .iter()
            .rev()
            .take(25)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .map(|line| (*line).to_string()),
    );
    result.join("\n")
}

/// Filter Get-NetTCPConnection output
fn filter_get_nettcpconnection_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() <= 2 {
        return output.to_string();
    }

    let mut result = Vec::new();
    let mut header_found = false;
    let mut line_count = 0;

    for &line in &lines {
        if (line.contains("LocalAddress") || line.contains("LocalPort")) && !header_found {
            result.push(line.to_string());
            header_found = true;
        } else if header_found && !line.trim().is_empty() && !line.starts_with("---") {
            result.push(line.to_string());
            line_count += 1;
            // Limit to 100 connections for token savings
            if line_count >= 100 {
                result.push(format!(
                    "... [truncated {} more TCP connections] ...",
                    lines.len() - result.len()
                ));
                break;
            }
        }
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_get_childitem_output() {
        let input = r#"
Directory: C:\temp

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----        14-Apr-26   9:59 AM                test
-a----        14-Apr-26   9:59 AM            100 test.txt
        "#;

        let filtered = filter_get_childitem_output(input);
        assert!(filtered.contains("Mode"));
        assert!(filtered.contains("test"));
        assert!(filtered.contains("test.txt"));
        assert!(!filtered.contains("Directory:"));
    }

    #[test]
    fn test_filter_copy_item_output() {
        let input = "   \n";
        let filtered = filter_copy_item_output(input);
        assert_eq!(filtered, "");
    }

    #[test]
    fn test_filter_new_item_output() {
        let input = r#"
Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        14-Apr-26  10:00 AM              0 newfile.txt
        "#;

        let filtered = filter_new_item_output(input);
        assert!(filtered.contains("newfile.txt"));
        assert!(!filtered.contains("Mode"));
    }

    #[test]
    fn test_filter_get_process_output() {
        let input = r#"
Name                           Id     CPU WorkingSet
----                           --     --- ----------
chrome                        1234  15.2    150MB
explorer                      5678   2.1     80MB
        "#;

        let filtered = filter_get_process_output(input);
        assert!(filtered.contains("Name"));
        assert!(filtered.contains("chrome"));
        assert!(filtered.contains("explorer"));
    }

    #[test]
    fn test_filter_get_content_output_truncates_long() {
        let input = (0..600)
            .map(|i| format!("line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let filtered = filter_get_content_output(&input);
        assert!(filtered.contains("... [truncated"));
        assert!(filtered.lines().count() < 600);
    }

    #[test]
    fn test_filter_select_string_output_truncates() {
        let input = (0..300)
            .map(|i| format!("file:1: match {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let filtered = filter_select_string_output(&input);
        assert!(filtered.contains("... [truncated"));
        assert_eq!(filtered.lines().count(), 201);
    }

    // 🔥 NEW TESTS FOR HIGH-VALUE CMDLETS

    #[test]
    fn test_filter_get_itemproperty_output() {
        let input = r#"
PSPath       : Microsoft.PowerShell.Core\Registry::HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer
PSParentPath : Microsoft.PowerShell.Core\Registry::HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion
PSChildName  : Explorer
PSDrive      : HKCU
PSProvider   : Microsoft.PowerShell.Core\Registry

Name                           Value
----                           -----
EnableAutoTray                0
ShellState                    {24 00 00 00...}
        "#;

        let filtered = filter_get_itemproperty_output(input);
        assert!(filtered.contains("PSPath"));
        assert!(filtered.contains("EnableAutoTray"));
    }

    #[test]
    fn test_filter_test_netconnection_output() {
        let input = r#"
ComputerName           : google.com
RemoteAddress          : 142.250.190.78
InterfaceAlias         : Ethernet
SourceAddress          : 192.168.1.100
PingSucceeded          : True
PingReplyDetails (RTT) : 14 ms
TcpTestSucceeded       : True
        "#;

        let filtered = filter_test_netconnection_output(input);
        assert!(filtered.contains("ComputerName"));
        assert!(filtered.contains("PingSucceeded"));
        assert!(filtered.contains("TcpTestSucceeded"));
    }

    #[test]
    fn test_filter_get_netadapter_output() {
        let input = r#"
Name                      InterfaceDescription                    Status       LinkSpeed
----                      --------------------                    ------       ---------
Ethernet                  Intel(R) Ethernet Connection            Up           1 Gbps
Wi-Fi                     Intel(R) Wireless-AC 9560               Disconnected 0 bps
        "#;

        let filtered = filter_get_netadapter_output(input);
        assert!(filtered.contains("Name"));
        assert!(filtered.contains("Ethernet"));
        assert!(filtered.contains("Wi-Fi"));
    }

    #[test]
    fn test_filter_get_computerinfo_output() {
        let input = r#"
CsName                 : DESKTOP-ABC123
CsDomain                : WORKGROUP
CsManufacturer          : Microsoft Corporation
CsModel                 : Surface Pro
OsName                  : Microsoft Windows 11 Pro
OsVersion               : 10.0.22621
OsArchitecture          : 64-bit
        "#;

        let filtered = filter_get_computerinfo_output(input);
        assert!(filtered.contains("CsName"));
        assert!(filtered.contains("OsName"));
        assert!(filtered.contains("OsVersion"));
        assert!(!filtered.contains("CsManufacturer"));
    }

    #[test]
    fn test_filter_get_package_output_truncates() {
        let input = (0..60)
            .map(|i| format!("Package{}    1.0.0    Programs    Installed", i))
            .collect::<Vec<_>>()
            .join("\n");
        let filtered = filter_get_package_output(&input);
        assert!(filtered.contains("... [truncated"));
        assert!(filtered.lines().count() < 60);
    }

    #[test]
    fn test_filter_get_eventlog_output_truncates() {
        let input = (0..250)
            .map(|i| {
                format!(
                    "4/20/2026 10:00:00 AM    Information    Application    Event {}",
                    i
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let filtered = filter_get_eventlog_output(&input);
        assert!(filtered.contains("... [truncated"));
        assert!(filtered.lines().count() < 250);
    }

    #[test]
    fn test_filter_get_counter_output_truncates() {
        let input = (0..60)
            .map(|i| format!("\\\\computer\\processor(_total)\\% processor time : {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let filtered = filter_get_counter_output(&input);
        assert!(filtered.contains("... [truncated"));
        assert!(filtered.lines().count() < 60);
    }

    #[test]
    fn test_filter_get_nettcpconnection_output_truncates() {
        let input = (0..120)
            .map(|_i| format!("192.168.1.100    54321    8.8.8.8    53    Listen    1234"))
            .collect::<Vec<_>>()
            .join("\n");
        let filtered = filter_get_nettcpconnection_output(&input);
        assert!(filtered.contains("... [truncated"));
        assert!(filtered.lines().count() < 120);
    }
}
