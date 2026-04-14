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

    // Route based on cmdlet name
    match args[0].as_str() {
        "Get-ChildItem" | "gci" | "ls" | "dir" => run_get_childitem(&args[1..], verbose),
        "Get-Process" | "gps" | "ps" => run_get_process(&args[1..], verbose),
        "Get-Service" | "gsv" => run_get_service(&args[1..], verbose),
        "Get-Content" | "gc" | "cat" | "type" => run_get_content(&args[1..], verbose),
        "Set-Location" | "sl" | "cd" => run_set_location(&args[1..], verbose),
        "Get-Location" | "gl" | "pwd" => run_get_location(&args[1..], verbose),
        _ => run_powershell_passthrough(args, verbose),
    }
}

/// Filter Get-ChildItem (ls/dir) output
fn run_get_childitem(args: &[String], verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();

    // Build PowerShell command
    let mut cmd = resolved_command("powershell.exe");
    cmd.arg("-Command").arg("Get-ChildItem");

    // Add user arguments
    for arg in args {
        cmd.arg(arg);
    }

    // Add format for better parsing
    cmd.arg("|").arg("Format-Table").arg("-AutoSize");

    let output = cmd.output().context("Failed to run Get-ChildItem")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("{}", stderr);
        return Ok(crate::core::utils::exit_code_from_output(&output, "Get-ChildItem"));
    }

    let filtered = filter_get_childitem_output(&stdout);

    timer.track("Get-ChildItem", "rtk Get-ChildItem", &stdout, &filtered);
    print!("{}", filtered);

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(0)
}

/// Filter Get-Process output
fn run_get_process(args: &[String], verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();

    let mut cmd = resolved_command("powershell.exe");
    cmd.arg("-Command").arg("Get-Process");

    for arg in args {
        cmd.arg(arg);
    }

    cmd.arg("|").arg("Select-Object").arg("Name,Id,CPU,WorkingSet");

    let output = cmd.output().context("Failed to run Get-Process")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("{}", stderr);
        return Ok(crate::core::utils::exit_code_from_output(&output, "Get-Process"));
    }

    let filtered = filter_get_process_output(&stdout);

    timer.track("Get-Process", "rtk Get-Process", &stdout, &filtered);
    print!("{}", filtered);

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(0)
}

/// Filter Get-Service output
fn run_get_service(args: &[String], verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();

    let mut cmd = resolved_command("powershell.exe");
    cmd.arg("-Command").arg("Get-Service");

    for arg in args {
        cmd.arg(arg);
    }

    cmd.arg("|").arg("Select-Object").arg("Name,DisplayName,Status");

    let output = cmd.output().context("Failed to run Get-Service")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("{}", stderr);
        return Ok(crate::core::utils::exit_code_from_output(&output, "Get-Service"));
    }

    let filtered = filter_get_service_output(&stdout);

    timer.track("Get-Service", "rtk Get-Service", &stdout, &filtered);
    print!("{}", filtered);

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(0)
}

/// Filter Get-Content output
fn run_get_content(args: &[String], verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();

    let mut cmd = resolved_command("powershell.exe");
    cmd.arg("-Command").arg("Get-Content");

    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd.output().context("Failed to run Get-Content")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("{}", stderr);
        return Ok(crate::core::utils::exit_code_from_output(&output, "Get-Content"));
    }

    let filtered = filter_get_content_output(&stdout);

    timer.track("Get-Content", "rtk Get-Content", &stdout, &filtered);
    print!("{}", filtered);

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(0)
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

/// Generic PowerShell passthrough for unsupported cmdlets
fn run_powershell_passthrough(args: &[String], verbose: u8) -> Result<i32> {
    let timer = crate::core::tracking::TimedExecution::start();

    let mut cmd = resolved_command("powershell.exe");
    cmd.arg("-Command");

    let mut command_str = String::new();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            command_str.push(' ');
        }
        command_str.push_str(arg);
    }

    cmd.arg(&command_str);

    let output = cmd.output().context("Failed to run PowerShell command")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("{}", stderr);
        return Ok(crate::core::utils::exit_code_from_output(&output, "powershell"));
    }

    timer.track_passthrough(
        &format!("powershell {}", command_str),
        &format!("rtk powershell {} (passthrough)", command_str),
    );

    print!("{}", stdout);

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(0)
}

/// Filter Get-ChildItem output to reduce tokens
fn filter_get_childitem_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();

    if lines.is_empty() {
        return String::new();
    }

    // Keep header and filter out unnecessary details
    let mut result = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Skip verbose metadata, keep essential file info
        if line.contains("Mode") && line.contains("LastWriteTime") {
            result.push(line.to_string()); // Keep header
        } else if !line.starts_with("Directory:") {
            // Filter file entries - could add more filtering logic here
            result.push(line.to_string());
        }
    }

    result.join("\n")
}

/// Filter Get-Process output
fn filter_get_process_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();

    if lines.len() <= 2 {
        return output.to_string(); // Too short, return as-is
    }

    // Keep header, filter process list
    let mut result = Vec::new();
    let mut header_found = false;

    for line in lines {
        if line.contains("Name") && line.contains("Id") && !header_found {
            result.push(line.to_string());
            header_found = true;
        } else if header_found && !line.trim().is_empty() {
            // Could add filtering for specific processes or memory thresholds
            result.push(line.to_string());
        }
    }

    result.join("\n")
}

/// Filter Get-Service output
fn filter_get_service_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();

    if lines.len() <= 2 {
        return output.to_string();
    }

    // Keep header, filter service list
    let mut result = Vec::new();
    let mut header_found = false;

    for line in lines {
        if line.contains("Name") && line.contains("Status") && !header_found {
            result.push(line.to_string());
            header_found = true;
        } else if header_found && !line.trim().is_empty() {
            // Could filter to only show running services
            result.push(line.to_string());
        }
    }

    result.join("\n")
}

/// Filter Get-Content output
fn filter_get_content_output(output: &str) -> String {
    // For file content, we might want to limit lines or filter
    // For now, return as-is but could add intelligent filtering
    output.to_string()
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
}