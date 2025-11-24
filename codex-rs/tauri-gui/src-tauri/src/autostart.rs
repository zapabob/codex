//! Autostart functionality for Windows, macOS, and Linux
//!
//! Provides cross-platform autostart management using platform-specific APIs.

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{info, warn};

/// Set autostart enabled/disabled
pub fn set_autostart(enabled: bool) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        set_autostart_windows(enabled)
    }
    #[cfg(target_os = "macos")]
    {
        set_autostart_macos(enabled)
    }
    #[cfg(target_os = "linux")]
    {
        set_autostart_linux(enabled)
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        warn!("Autostart not supported on this platform");
        Ok(())
    }
}

/// Check if autostart is enabled
pub fn is_autostart_enabled() -> Result<bool> {
    #[cfg(target_os = "windows")]
    {
        is_autostart_enabled_windows()
    }
    #[cfg(target_os = "macos")]
    {
        is_autostart_enabled_macos()
    }
    #[cfg(target_os = "linux")]
    {
        is_autostart_enabled_linux()
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Ok(false)
    }
}

#[cfg(target_os = "windows")]
fn set_autostart_windows(enabled: bool) -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let exe_path = std::env::current_exe()
        .context("Failed to get current executable path")?;
    let exe_path_str = exe_path.to_string_lossy();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_SET_VALUE | KEY_READ,
    )?;

    if enabled {
        run_key.set_value("Codex", &exe_path_str.to_string())?;
        info!("Autostart enabled: {}", exe_path_str);
    } else {
        run_key.delete_value("Codex").ok(); // Ignore if not exists
        info!("Autostart disabled");
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn is_autostart_enabled_windows() -> Result<bool> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_READ,
    )?;

    match run_key.get_value::<String, _>("Codex") {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(target_os = "macos")]
fn set_autostart_macos(enabled: bool) -> Result<()> {
    use std::process::Command;

    let exe_path = std::env::current_exe()
        .context("Failed to get current executable path")?;

    if enabled {
        // Create Launch Agent plist
        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.codex.autostart</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
            exe_path.to_string_lossy()
        );

        let plist_path = dirs::home_dir()
            .context("Failed to get home directory")?
            .join("Library/LaunchAgents/com.codex.autostart.plist");

        std::fs::write(&plist_path, plist_content)
            .context("Failed to write Launch Agent plist")?;

        // Load the Launch Agent
        Command::new("launchctl")
            .args(&["load", &plist_path.to_string_lossy()])
            .output()
            .context("Failed to load Launch Agent")?;

        info!("Autostart enabled on macOS");
    } else {
        // Unload the Launch Agent
        Command::new("launchctl")
            .args(&["unload", "com.codex.autostart"])
            .output()
            .ok();

        // Remove plist file
        let plist_path = dirs::home_dir()
            .context("Failed to get home directory")?
            .join("Library/LaunchAgents/com.codex.autostart.plist");
        std::fs::remove_file(&plist_path).ok();

        info!("Autostart disabled on macOS");
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn is_autostart_enabled_macos() -> Result<bool> {
    use std::process::Command;

    let output = Command::new("launchctl")
        .args(&["list", "com.codex.autostart"])
        .output()
        .ok();

    Ok(output.map(|o| o.status.success()).unwrap_or(false))
}

#[cfg(target_os = "linux")]
fn set_autostart_linux(enabled: bool) -> Result<()> {
    use std::fs;

    let exe_path = std::env::current_exe()
        .context("Failed to get current executable path")?;

    let autostart_dir = dirs::config_dir()
        .context("Failed to get config directory")?
        .join("autostart");

    fs::create_dir_all(&autostart_dir)
        .context("Failed to create autostart directory")?;

    let desktop_file = autostart_dir.join("codex.desktop");

    if enabled {
        let desktop_content = format!(
            r#"[Desktop Entry]
Type=Application
Name=Codex
Exec={}
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
"#,
            exe_path.to_string_lossy()
        );

        fs::write(&desktop_file, desktop_content)
            .context("Failed to write desktop file")?;

        info!("Autostart enabled on Linux");
    } else {
        fs::remove_file(&desktop_file).ok();
        info!("Autostart disabled on Linux");
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn is_autostart_enabled_linux() -> Result<bool> {
    let autostart_dir = dirs::config_dir()
        .context("Failed to get config directory")?
        .join("autostart");

    let desktop_file = autostart_dir.join("codex.desktop");
    Ok(desktop_file.exists())
}

/// Background process manager
pub struct BackgroundProcessManager {
    process_id: Option<u32>,
}

impl BackgroundProcessManager {
    pub fn new() -> Self {
        Self { process_id: None }
    }

    /// Start background process
    pub fn start(&mut self, command: &str, args: &[&str]) -> Result<()> {
        use std::process::Command;

        #[cfg(target_os = "windows")]
        {
            let mut cmd = Command::new("cmd");
            cmd.args(&["/C", "start", "/B", command]);
            cmd.args(args);
            let child = cmd.spawn()?;
            self.process_id = child.id().into();
        }

        #[cfg(not(target_os = "windows"))]
        {
            let mut cmd = Command::new(command);
            cmd.args(args);
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::null());
            let child = cmd.spawn()?;
            self.process_id = Some(child.id());
        }

        info!("Background process started: {} (PID: {:?})", command, self.process_id);
        Ok(())
    }

    /// Stop background process
    pub fn stop(&mut self) -> Result<()> {
        if let Some(pid) = self.process_id {
            #[cfg(target_os = "windows")]
            {
                use std::process::Command;
                Command::new("taskkill")
                    .args(&["/F", "/PID", &pid.to_string()])
                    .output()?;
            }

            #[cfg(not(target_os = "windows"))]
            {
                use std::process::Command;
                Command::new("kill")
                    .args(&["-9", &pid.to_string()])
                    .output()?;
            }

            info!("Background process stopped (PID: {})", pid);
            self.process_id = None;
        }

        Ok(())
    }

    /// Check if process is running
    pub fn is_running(&self) -> bool {
        self.process_id.is_some()
    }
}

impl Default for BackgroundProcessManager {
    fn default() -> Self {
        Self::new()
    }
}
