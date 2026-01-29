//! Windows Audio Control
//!
//! Uses PowerShell to control system volume via simulated key presses.
//! POWERSHELL IS JUST A TEMP SOLUTION. THERE IS A BUG IN THE WINDOWS API FOR 
//! .ACTIVATE() AND YOU CANNOT USE IT CURRENTLY FOR VOLUME CONTROL

use std::process::Command;

/// Increase the system volume by one step.
pub fn volume_up() -> Result<(), String> {
    // [char]175 is the Volume Up key scancode
    Command::new("powershell")
        .args([
            "-Command",
            "(New-Object -ComObject WScript.Shell).SendKeys([char]175)"
        ])
        .output()
        .map_err(|e| format!("Failed to run PowerShell: {}", e))?;

    Ok(())
}

/// Decrease the system volume by one step.
pub fn volume_down() -> Result<(), String> {
    // [char]174 is the Volume Down key scancode
    Command::new("powershell")
        .args([
            "-Command",
            "(New-Object -ComObject WScript.Shell).SendKeys([char]174)"
        ])
        .output()
        .map_err(|e| format!("Failed to run PowerShell: {}", e))?;
    Ok(())
}