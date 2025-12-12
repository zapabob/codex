use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

use crate::shell_snapshot::ShellSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShellType {
    Zsh,
    Bash,
    Sh,
    PowerShell,
    Cmd,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shell {
    pub(crate) shell_type: ShellType,
    pub(crate) shell_path: PathBuf,
    #[serde(skip_serializing, skip_deserializing, default)]
    pub(crate) shell_snapshot: Option<Arc<ShellSnapshot>>,
}

impl Shell {
    pub fn name(&self) -> String {
        self.shell_path
            .file_name()
            .and_then(|s| s.to_str())
            .map(ToString::to_string)
            .unwrap_or_else(|| format!("{shell_type:?}", shell_type = self.shell_type))
    }

    pub fn derive_exec_args(&self, command: &str, use_login_shell: bool) -> Vec<String> {
        let shell_path = self.shell_path.to_string_lossy().to_string();
        match self.shell_type {
            ShellType::Zsh | ShellType::Bash | ShellType::Sh => {
                let flag = if use_login_shell { "-lc" } else { "-c" };
                vec![shell_path, flag.to_string(), command.to_string()]
            }
            ShellType::PowerShell => {
                if use_login_shell {
                    vec![shell_path, "-Command".to_string(), command.to_string()]
                } else {
                    vec![
                        shell_path,
                        "-NoProfile".to_string(),
                        "-Command".to_string(),
                        command.to_string(),
                    ]
                }
            }
            ShellType::Cmd => vec![shell_path, "/C".to_string(), command.to_string()],
        }
    }
}

fn file_exists(path: &PathBuf) -> Option<PathBuf> {
    std::fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.is_file().then_some(path.clone()))
}

fn which_first(candidates: &[&str]) -> Option<PathBuf> {
    for name in candidates {
        if let Ok(path) = which::which(name) {
            return Some(path);
        }
    }
    None
}

fn shell_from_path(shell_type: ShellType, shell_path: PathBuf) -> Shell {
    Shell {
        shell_type,
        shell_path,
        shell_snapshot: None,
    }
}

fn resolve_shell_path(shell_type: ShellType, provided_path: Option<&PathBuf>) -> Option<PathBuf> {
    if let Some(path) = provided_path {
        if let Some(found) = file_exists(path) {
            return Some(found);
        }
    }

    match shell_type {
        ShellType::Zsh => which_first(&["zsh", "/bin/zsh", "/usr/bin/zsh"]),
        ShellType::Bash => which_first(&["bash", "/bin/bash", "/usr/bin/bash", "/usr/local/bin/bash"]),
        ShellType::Sh => which_first(&["sh", "/bin/sh", "/usr/bin/sh"]),
        ShellType::PowerShell => which_first(&["pwsh.exe", "pwsh", "powershell.exe", "powershell"]),
        ShellType::Cmd => which_first(&["cmd.exe", "cmd"]),
    }
}

fn ultimate_fallback_shell() -> Shell {
    if cfg!(windows) {
        shell_from_path(ShellType::Cmd, PathBuf::from("cmd.exe"))
    } else {
        shell_from_path(ShellType::Sh, PathBuf::from("/bin/sh"))
    }
}

pub fn get_shell_by_model_provided_path(shell_path: &PathBuf) -> Shell {
    detect_shell_type(shell_path)
        .and_then(|shell_type| get_shell(shell_type, Some(shell_path)))
        .unwrap_or_else(ultimate_fallback_shell)
}

pub fn get_shell(shell_type: ShellType, path: Option<&PathBuf>) -> Option<Shell> {
    resolve_shell_path(shell_type, path).map(|shell_path| shell_from_path(shell_type, shell_path))
}

pub fn detect_shell_type(shell_path: &PathBuf) -> Option<ShellType> {
    let stem = shell_path.file_stem().and_then(|s| s.to_str())?;
    let stem = stem.to_ascii_lowercase();
    match stem.as_str() {
        "zsh" => Some(ShellType::Zsh),
        "bash" => Some(ShellType::Bash),
        "sh" => Some(ShellType::Sh),
        "pwsh" | "powershell" => Some(ShellType::PowerShell),
        "cmd" => Some(ShellType::Cmd),
        _ => None,
    }
}

#[cfg(unix)]
pub async fn default_user_shell() -> Shell {
    if let Ok(shell) = std::env::var("SHELL") {
        let path = PathBuf::from(shell);
        if let Some(shell_type) = detect_shell_type(&path) {
            if let Some(shell) = get_shell(shell_type, Some(&path)) {
                return shell;
            }
        }
    }

    get_shell(ShellType::Zsh, None)
        .or_else(|| get_shell(ShellType::Bash, None))
        .or_else(|| get_shell(ShellType::Sh, None))
        .unwrap_or_else(ultimate_fallback_shell)
}

#[cfg(target_os = "windows")]
pub async fn default_user_shell() -> Shell {
    if let Some(path) = which_first(&["pwsh.exe", "pwsh"]) {
        return shell_from_path(ShellType::PowerShell, path);
    }
    if let Some(path) = which_first(&["powershell.exe", "powershell"]) {
        return shell_from_path(ShellType::PowerShell, path);
    }
    ultimate_fallback_shell()
}

#[cfg(all(not(target_os = "windows"), not(unix)))]
pub async fn default_user_shell() -> Shell {
    ultimate_fallback_shell()
}
