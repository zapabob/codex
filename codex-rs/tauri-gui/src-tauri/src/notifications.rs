//! Notification system for desktop and system tray
//!
//! Provides cross-platform notifications for various events.

use anyhow::Result;
use tracing::{info, warn};

/// Notification types
#[derive(Debug, Clone)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

/// Notification
#[derive(Debug, Clone)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub notification_type: NotificationType,
}

impl Notification {
    pub fn new(title: String, body: String, notification_type: NotificationType) -> Self {
        Self {
            title,
            body,
            notification_type,
        }
    }

    pub fn info(title: String, body: String) -> Self {
        Self::new(title, body, NotificationType::Info)
    }

    pub fn success(title: String, body: String) -> Self {
        Self::new(title, body, NotificationType::Success)
    }

    pub fn warning(title: String, body: String) -> Self {
        Self::new(title, body, NotificationType::Warning)
    }

    pub fn error(title: String, body: String) -> Self {
        Self::new(title, body, NotificationType::Error)
    }
}

/// Notification manager
pub struct NotificationManager {
    enabled: bool,
}

impl NotificationManager {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Show desktop notification
    pub fn show(&self, notification: &Notification) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        #[cfg(target_os = "windows")]
        {
            self.show_windows(notification)
        }
        #[cfg(target_os = "macos")]
        {
            self.show_macos(notification)
        }
        #[cfg(target_os = "linux")]
        {
            self.show_linux(notification)
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            warn!("Notifications not supported on this platform");
            Ok(())
        }
    }

    /// Show system tray notification (tooltip)
    pub fn show_tray_tooltip(&self, message: &str) -> Result<()> {
        info!("Tray tooltip: {}", message);
        // Tray tooltip is handled by Tauri's tray API
        Ok(())
    }

    /// Enable/disable notifications
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        info!("Notifications {}", if enabled { "enabled" } else { "disabled" });
    }

    /// Check if notifications are enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[cfg(target_os = "windows")]
    fn show_windows(&self, notification: &Notification) -> Result<()> {
        // Use Windows Toast Notifications API
        // For now, use a simple message box as fallback
        use std::process::Command;

        let icon = match notification.notification_type {
            NotificationType::Info | NotificationType::Success => "info",
            NotificationType::Warning => "warning",
            NotificationType::Error => "error",
        };

        // Use PowerShell to show toast notification (Windows 10+)
        let ps_script = format!(
            r#"
            [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
            [Windows.Data.Xml.Dom.XmlDocument, Windows.Data.Xml.Dom.XmlDocument, ContentType = WindowsRuntime] | Out-Null
            
            $template = @"
            <toast>
                <visual>
                    <binding template="ToastGeneric">
                        <text>{}</text>
                        <text>{}</text>
                    </binding>
                </visual>
            </toast>
"@
            $xml = New-Object Windows.Data.Xml.Dom.XmlDocument
            $xml.LoadXml($template)
            $toast = [Windows.UI.Notifications.ToastNotification]::new($xml)
            [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier("Codex").Show($toast)
            "#,
            notification.title, notification.body
        );

        Command::new("powershell")
            .args(&["-Command", &ps_script])
            .output()
            .ok();

        info!("Windows notification: {} - {}", notification.title, notification.body);
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn show_macos(&self, notification: &Notification) -> Result<()> {
        use std::process::Command;

        let sound = match notification.notification_type {
            NotificationType::Info | NotificationType::Success => "Glass",
            NotificationType::Warning => "Basso",
            NotificationType::Error => "Basso",
        };

        Command::new("osascript")
            .args(&[
                "-e",
                &format!(
                    "display notification \"{}\" with title \"{}\" sound name \"{}\"",
                    notification.body, notification.title, sound
                ),
            ])
            .output()
            .context("Failed to show macOS notification")?;

        info!("macOS notification: {} - {}", notification.title, notification.body);
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn show_linux(&self, notification: &Notification) -> Result<()> {
        use std::process::Command;

        let urgency = match notification.notification_type {
            NotificationType::Info | NotificationType::Success => "normal",
            NotificationType::Warning => "normal",
            NotificationType::Error => "critical",
        };

        // Try notify-send first
        if Command::new("notify-send")
            .args(&[
                "--urgency", urgency,
                "--app-name", "Codex",
                &notification.title,
                &notification.body,
            ])
            .output()
            .is_ok()
        {
            info!("Linux notification (notify-send): {} - {}", notification.title, notification.body);
            return Ok(());
        }

        // Fallback to dbus-send
        Command::new("dbus-send")
            .args(&[
                "--session",
                "--dest=org.freedesktop.Notifications",
                "--type=method_call",
                "/org/freedesktop/Notifications",
                "org.freedesktop.Notifications.Notify",
                "string:Codex",
                "uint32:0",
                "string:",
                &format!("string:{}", notification.title),
                &format!("string:{}", notification.body),
                "array:string:",
                "dict:string:",
                "int32:5000",
            ])
            .output()
            .ok();

        info!("Linux notification: {} - {}", notification.title, notification.body);
        Ok(())
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new(true)
    }
}

