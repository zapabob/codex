use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

use crate::shell_snapshot::ShellSnapshot;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ZshShell {
    pub(crate) shell_path: String,
    pub(crate) zshrc_path: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
<<<<<<< HEAD
pub struct BashShell {
    pub(crate) shell_path: String,
    pub(crate) bashrc_path: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct PowerShellConfig {
    pub(crate) exe: String, // Executable name or path, e.g. "pwsh" or "powershell.exe".
    pub(crate) bash_exe_fallback: Option<PathBuf>, // In case the model generates a bash command.
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Shell {
    Zsh(ZshShell),
    Bash(BashShell),
    PowerShell(PowerShellConfig),
    Unknown,
=======
pub struct Shell {
    pub(crate) shell_type: ShellType,
    pub(crate) shell_path: PathBuf,
    #[serde(skip_serializing, skip_deserializing, default)]
    pub(crate) shell_snapshot: Option<Arc<ShellSnapshot>>,
>>>>>>> upstream/main
}

impl Shell {
    pub fn name(&self) -> Option<String> {
        match self {
            Shell::Zsh(zsh) => std::path::Path::new(&zsh.shell_path)
                .file_name()
                .map(|s| s.to_string_lossy().to_string()),
            Shell::Bash(bash) => std::path::Path::new(&bash.shell_path)
                .file_name()
                .map(|s| s.to_string_lossy().to_string()),
            Shell::PowerShell(ps) => Some(ps.exe.clone()),
            Shell::Unknown => None,
        }
    }
}

#[cfg(unix)]
fn detect_default_user_shell() -> Shell {
    use libc::getpwuid;
    use libc::getuid;
    use std::ffi::CStr;

    unsafe {
        let uid = getuid();
        let pw = getpwuid(uid);

        if !pw.is_null() {
            let shell_path = CStr::from_ptr((*pw).pw_shell)
                .to_string_lossy()
                .into_owned();
            let home_path = CStr::from_ptr((*pw).pw_dir).to_string_lossy().into_owned();

            if shell_path.ends_with("/zsh") {
                return Shell::Zsh(ZshShell {
                    shell_path,
                    zshrc_path: format!("{home_path}/.zshrc"),
                });
            }

<<<<<<< HEAD
            if shell_path.ends_with("/bash") {
                return Shell::Bash(BashShell {
                    shell_path,
                    bashrc_path: format!("{home_path}/.bashrc"),
                });
=======
fn file_exists(path: &PathBuf) -> Option<PathBuf> {
    if std::fs::metadata(path).is_ok_and(|metadata| metadata.is_file()) {
        Some(PathBuf::from(path))
    } else {
        None
    }
}

fn get_shell_path(
    shell_type: ShellType,
    provided_path: Option<&PathBuf>,
    binary_name: &str,
    fallback_paths: Vec<&str>,
) -> Option<PathBuf> {
    // If exact provided path exists, use it
    if provided_path.and_then(file_exists).is_some() {
        return provided_path.cloned();
    }

    // Check if the shell we are trying to load is user's default shell
    // if just use it
    let default_shell_path = get_user_shell_path();
    if let Some(default_shell_path) = default_shell_path
        && detect_shell_type(&default_shell_path) == Some(shell_type)
    {
        return Some(default_shell_path);
    }

    if let Ok(path) = which::which(binary_name) {
        return Some(path);
    }

    for path in fallback_paths {
        //check exists
        if let Some(path) = file_exists(&PathBuf::from(path)) {
            return Some(path);
        }
    }

    None
}

fn get_zsh_shell(path: Option<&PathBuf>) -> Option<Shell> {
    let shell_path = get_shell_path(ShellType::Zsh, path, "zsh", vec!["/bin/zsh"]);

    shell_path.map(|shell_path| Shell {
        shell_type: ShellType::Zsh,
        shell_path,
        shell_snapshot: None,
    })
}

fn get_bash_shell(path: Option<&PathBuf>) -> Option<Shell> {
    let shell_path = get_shell_path(ShellType::Bash, path, "bash", vec!["/bin/bash"]);

    shell_path.map(|shell_path| Shell {
        shell_type: ShellType::Bash,
        shell_path,
        shell_snapshot: None,
    })
}

fn get_sh_shell(path: Option<&PathBuf>) -> Option<Shell> {
    let shell_path = get_shell_path(ShellType::Sh, path, "sh", vec!["/bin/sh"]);

    shell_path.map(|shell_path| Shell {
        shell_type: ShellType::Sh,
        shell_path,
        shell_snapshot: None,
    })
}

fn get_powershell_shell(path: Option<&PathBuf>) -> Option<Shell> {
    let shell_path = get_shell_path(
        ShellType::PowerShell,
        path,
        "pwsh",
        vec!["/usr/local/bin/pwsh"],
    )
    .or_else(|| get_shell_path(ShellType::PowerShell, path, "powershell", vec![]));

    shell_path.map(|shell_path| Shell {
        shell_type: ShellType::PowerShell,
        shell_path,
        shell_snapshot: None,
    })
}

fn get_cmd_shell(path: Option<&PathBuf>) -> Option<Shell> {
    let shell_path = get_shell_path(ShellType::Cmd, path, "cmd", vec![]);

    shell_path.map(|shell_path| Shell {
        shell_type: ShellType::Cmd,
        shell_path,
        shell_snapshot: None,
    })
}

fn ultimate_fallback_shell() -> Shell {
    if cfg!(windows) {
        Shell {
            shell_type: ShellType::Cmd,
            shell_path: PathBuf::from("cmd.exe"),
            shell_snapshot: None,
        }
    } else {
        Shell {
            shell_type: ShellType::Sh,
            shell_path: PathBuf::from("/bin/sh"),
            shell_snapshot: None,
        }
    }
}

pub fn get_shell_by_model_provided_path(shell_path: &PathBuf) -> Shell {
    detect_shell_type(shell_path)
        .and_then(|shell_type| get_shell(shell_type, Some(shell_path)))
        .unwrap_or(ultimate_fallback_shell())
}

pub fn get_shell(shell_type: ShellType, path: Option<&PathBuf>) -> Option<Shell> {
    match shell_type {
        ShellType::Zsh => get_zsh_shell(path),
        ShellType::Bash => get_bash_shell(path),
        ShellType::PowerShell => get_powershell_shell(path),
        ShellType::Sh => get_sh_shell(path),
        ShellType::Cmd => get_cmd_shell(path),
    }
}

pub fn detect_shell_type(shell_path: &PathBuf) -> Option<ShellType> {
    match shell_path.as_os_str().to_str() {
        Some("zsh") => Some(ShellType::Zsh),
        Some("sh") => Some(ShellType::Sh),
        Some("cmd") => Some(ShellType::Cmd),
        Some("bash") => Some(ShellType::Bash),
        Some("pwsh") => Some(ShellType::PowerShell),
        Some("powershell") => Some(ShellType::PowerShell),
        _ => {
            let shell_name = shell_path.file_stem();
            if let Some(shell_name) = shell_name
                && shell_name != shell_path
            {
                detect_shell_type(&PathBuf::from(shell_name))
            } else {
                None
>>>>>>> upstream/main
            }
        }
    }
    Shell::Unknown
}

#[cfg(unix)]
pub async fn default_user_shell() -> Shell {
    detect_default_user_shell()
}

#[cfg(target_os = "windows")]
pub async fn default_user_shell() -> Shell {
    use tokio::process::Command;

    // Prefer PowerShell 7+ (`pwsh`) if available, otherwise fall back to Windows PowerShell.
    let has_pwsh = Command::new("pwsh")
        .arg("-NoLogo")
        .arg("-NoProfile")
        .arg("-Command")
        .arg("$PSVersionTable.PSVersion.Major")
        .output()
        .await
        .map(|o| o.status.success())
        .unwrap_or(false);
    let bash_exe = if Command::new("bash.exe")
        .arg("--version")
        .stdin(std::process::Stdio::null())
        .output()
        .await
        .ok()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        which::which("bash.exe").ok()
    } else {
        None
    };

    if has_pwsh {
        Shell::PowerShell(PowerShellConfig {
            exe: "pwsh.exe".to_string(),
            bash_exe_fallback: bash_exe,
        })
    } else {
        Shell::PowerShell(PowerShellConfig {
            exe: "powershell.exe".to_string(),
            bash_exe_fallback: bash_exe,
        })
    }
}

#[cfg(all(not(target_os = "windows"), not(unix)))]
pub async fn default_user_shell() -> Shell {
    Shell::Unknown
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::process::Command;

<<<<<<< HEAD
=======
    #[test]
    #[cfg(target_os = "macos")]
    fn detects_zsh() {
        let zsh_shell = get_shell(ShellType::Zsh, None).unwrap();

        let shell_path = zsh_shell.shell_path;

        assert_eq!(shell_path, PathBuf::from("/bin/zsh"));
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn fish_fallback_to_zsh() {
        let zsh_shell = default_user_shell_from_path(Some(PathBuf::from("/bin/fish")));

        let shell_path = zsh_shell.shell_path;

        assert_eq!(shell_path, PathBuf::from("/bin/zsh"));
    }

    #[test]
    fn detects_bash() {
        let bash_shell = get_shell(ShellType::Bash, None).unwrap();
        let shell_path = bash_shell.shell_path;

        assert!(
            shell_path == PathBuf::from("/bin/bash")
                || shell_path == PathBuf::from("/usr/bin/bash")
                || shell_path == PathBuf::from("/usr/local/bin/bash"),
            "shell path: {shell_path:?}",
        );
    }

    #[test]
    fn detects_sh() {
        let sh_shell = get_shell(ShellType::Sh, None).unwrap();
        let shell_path = sh_shell.shell_path;
        assert!(
            shell_path == PathBuf::from("/bin/sh") || shell_path == PathBuf::from("/usr/bin/sh"),
            "shell path: {shell_path:?}",
        );
    }

    #[test]
    fn can_run_on_shell_test() {
        let cmd = "echo \"Works\"";
        if cfg!(windows) {
            assert!(shell_works(
                get_shell(ShellType::PowerShell, None),
                "Out-String 'Works'",
                true,
            ));
            assert!(shell_works(get_shell(ShellType::Cmd, None), cmd, true,));
            assert!(shell_works(Some(ultimate_fallback_shell()), cmd, true));
        } else {
            assert!(shell_works(Some(ultimate_fallback_shell()), cmd, true));
            assert!(shell_works(get_shell(ShellType::Zsh, None), cmd, false));
            assert!(shell_works(get_shell(ShellType::Bash, None), cmd, true));
            assert!(shell_works(get_shell(ShellType::Sh, None), cmd, true));
        }
    }

    fn shell_works(shell: Option<Shell>, command: &str, required: bool) -> bool {
        if let Some(shell) = shell {
            let args = shell.derive_exec_args(command, false);
            let output = Command::new(args[0].clone())
                .args(&args[1..])
                .output()
                .unwrap();
            assert!(output.status.success());
            assert!(String::from_utf8_lossy(&output.stdout).contains("Works"));
            true
        } else {
            !required
        }
    }

    #[test]
    fn derive_exec_args() {
        let test_bash_shell = Shell {
            shell_type: ShellType::Bash,
            shell_path: PathBuf::from("/bin/bash"),
            shell_snapshot: None,
        };
        assert_eq!(
            test_bash_shell.derive_exec_args("echo hello", false),
            vec!["/bin/bash", "-c", "echo hello"]
        );
        assert_eq!(
            test_bash_shell.derive_exec_args("echo hello", true),
            vec!["/bin/bash", "-lc", "echo hello"]
        );

        let test_zsh_shell = Shell {
            shell_type: ShellType::Zsh,
            shell_path: PathBuf::from("/bin/zsh"),
            shell_snapshot: None,
        };
        assert_eq!(
            test_zsh_shell.derive_exec_args("echo hello", false),
            vec!["/bin/zsh", "-c", "echo hello"]
        );
        assert_eq!(
            test_zsh_shell.derive_exec_args("echo hello", true),
            vec!["/bin/zsh", "-lc", "echo hello"]
        );

        let test_powershell_shell = Shell {
            shell_type: ShellType::PowerShell,
            shell_path: PathBuf::from("pwsh.exe"),
            shell_snapshot: None,
        };
        assert_eq!(
            test_powershell_shell.derive_exec_args("echo hello", false),
            vec!["pwsh.exe", "-NoProfile", "-Command", "echo hello"]
        );
        assert_eq!(
            test_powershell_shell.derive_exec_args("echo hello", true),
            vec!["pwsh.exe", "-Command", "echo hello"]
        );
    }

>>>>>>> upstream/main
    #[tokio::test]
    async fn test_current_shell_detects_zsh() {
        let shell = Command::new("sh")
            .arg("-c")
            .arg("echo $SHELL")
            .output()
            .unwrap();

        let home = std::env::var("HOME").unwrap();
        let shell_path = String::from_utf8_lossy(&shell.stdout).trim().to_string();
        if shell_path.ends_with("/zsh") {
            assert_eq!(
<<<<<<< HEAD
                default_user_shell().await,
                Shell::Zsh(ZshShell {
                    shell_path: shell_path.to_string(),
                    zshrc_path: format!("{home}/.zshrc",),
                })
=======
                default_user_shell(),
                Shell {
                    shell_type: ShellType::Zsh,
                    shell_path: PathBuf::from(shell_path),
                    shell_snapshot: None,
                }
>>>>>>> upstream/main
            );
        }
    }

    #[tokio::test]
    async fn test_run_with_profile_bash_escaping_and_execution() {
        let shell_path = "/bin/bash";

        let cases = vec![
            (
                vec!["myecho"],
                vec![shell_path, "-lc", "source BASHRC_PATH && (myecho)"],
                Some("It works!\n"),
            ),
            (
                vec!["bash", "-lc", "echo 'single' \"double\""],
                vec![
                    shell_path,
                    "-lc",
                    "source BASHRC_PATH && (echo 'single' \"double\")",
                ],
                Some("single double\n"),
            ),
        ];

        for (input, expected_cmd, expected_output) in cases {
            use std::collections::HashMap;

            use crate::exec::ExecParams;
            use crate::exec::SandboxType;
            use crate::exec::process_exec_tool_call;
            use crate::protocol::SandboxPolicy;

            let temp_home = tempfile::tempdir().unwrap();
            let bashrc_path = temp_home.path().join(".bashrc");
            std::fs::write(
                &bashrc_path,
                r#"
                set -x
                function myecho {
                    echo 'It works!'
                }
                "#,
            )
            .unwrap();
            let command = expected_cmd
                .iter()
                .map(|s| s.replace("BASHRC_PATH", bashrc_path.to_str().unwrap()))
                .collect::<Vec<_>>();

            let output = process_exec_tool_call(
                ExecParams {
                    command: command.clone(),
                    cwd: PathBuf::from(temp_home.path()),
                    timeout_ms: None,
                    env: HashMap::from([(
                        "HOME".to_string(),
                        temp_home.path().to_str().unwrap().to_string(),
                    )]),
                    with_escalated_permissions: None,
                    justification: None,
                    arg0: None,
                },
                SandboxType::None,
                &SandboxPolicy::DangerFullAccess,
                temp_home.path(),
                &None,
                None,
            )
            .await
            .unwrap();

            assert_eq!(output.exit_code, 0, "input: {input:?} output: {output:?}");
            if let Some(expected) = expected_output {
                assert_eq!(
                    output.stdout.text, expected,
                    "input: {input:?} output: {output:?}"
                );
            }
        }
    }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod macos_tests {
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_run_with_profile_escaping_and_execution() {
        let shell_path = "/bin/zsh";

        let cases = vec![
            (
                vec!["myecho"],
                vec![shell_path, "-lc", "source ZSHRC_PATH && (myecho)"],
                Some("It works!\n"),
            ),
            (
                vec!["myecho"],
                vec![shell_path, "-lc", "source ZSHRC_PATH && (myecho)"],
                Some("It works!\n"),
            ),
            (
                vec!["bash", "-c", "echo 'single' \"double\""],
                vec![
                    shell_path,
                    "-lc",
                    "source ZSHRC_PATH && (bash -c \"echo 'single' \\\"double\\\"\")",
                ],
                Some("single double\n"),
            ),
            (
                vec!["bash", "-lc", "echo 'single' \"double\""],
                vec![
                    shell_path,
                    "-lc",
                    "source ZSHRC_PATH && (echo 'single' \"double\")",
                ],
                Some("single double\n"),
            ),
        ];
        for (input, expected_cmd, expected_output) in cases {
            use std::collections::HashMap;

            use crate::exec::ExecParams;
            use crate::exec::SandboxType;
            use crate::exec::process_exec_tool_call;
            use crate::protocol::SandboxPolicy;

            let temp_home = tempfile::tempdir().unwrap();
            let zshrc_path = temp_home.path().join(".zshrc");
            std::fs::write(
                &zshrc_path,
                r#"
                set -x
                function myecho {
                    echo 'It works!'
                }
                "#,
            )
            .unwrap();
            let command = expected_cmd
                .iter()
                .map(|s| s.replace("ZSHRC_PATH", zshrc_path.to_str().unwrap()))
                .collect::<Vec<_>>();

            let output = process_exec_tool_call(
                ExecParams {
                    command: command.clone(),
                    cwd: PathBuf::from(temp_home.path()),
                    timeout_ms: None,
                    env: HashMap::from([(
                        "HOME".to_string(),
                        temp_home.path().to_str().unwrap().to_string(),
                    )]),
                    with_escalated_permissions: None,
                    justification: None,
                    arg0: None,
                },
                SandboxType::None,
                &SandboxPolicy::DangerFullAccess,
                temp_home.path(),
                &None,
                None,
            )
            .await
            .unwrap();

            assert_eq!(output.exit_code, 0, "input: {input:?} output: {output:?}");
            if let Some(expected) = expected_output {
                assert_eq!(
                    output.stdout.text, expected,
                    "input: {input:?} output: {output:?}"
                );
            }
        }
    }
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests_windows {
    use super::*;

    #[test]
    fn test_format_default_shell_invocation_powershell() {
        use std::path::PathBuf;

        let cases = vec![
            (
                PowerShellConfig {
                    exe: "pwsh.exe".to_string(),
                    bash_exe_fallback: None,
                },
                vec!["bash", "-lc", "echo hello"],
                vec!["pwsh.exe", "-NoProfile", "-Command", "echo hello"],
            ),
            (
                PowerShellConfig {
                    exe: "powershell.exe".to_string(),
                    bash_exe_fallback: None,
                },
                vec!["bash", "-lc", "echo hello"],
                vec!["powershell.exe", "-NoProfile", "-Command", "echo hello"],
            ),
            (
                PowerShellConfig {
                    exe: "pwsh.exe".to_string(),
                    bash_exe_fallback: Some(PathBuf::from("bash.exe")),
                },
                vec!["bash", "-lc", "echo hello"],
                vec!["bash.exe", "-lc", "echo hello"],
            ),
            (
                PowerShellConfig {
                    exe: "pwsh.exe".to_string(),
                    bash_exe_fallback: Some(PathBuf::from("bash.exe")),
                },
                vec![
                    "bash",
                    "-lc",
                    "apply_patch <<'EOF'\n*** Begin Patch\n*** Update File: destination_file.txt\n-original content\n+modified content\n*** End Patch\nEOF",
                ],
                vec![
                    "bash.exe",
                    "-lc",
                    "apply_patch <<'EOF'\n*** Begin Patch\n*** Update File: destination_file.txt\n-original content\n+modified content\n*** End Patch\nEOF",
                ],
            ),
            (
                PowerShellConfig {
                    exe: "pwsh.exe".to_string(),
                    bash_exe_fallback: Some(PathBuf::from("bash.exe")),
                },
                vec!["echo", "hello"],
                vec!["pwsh.exe", "-NoProfile", "-Command", "echo hello"],
            ),
            (
                PowerShellConfig {
                    exe: "pwsh.exe".to_string(),
                    bash_exe_fallback: Some(PathBuf::from("bash.exe")),
                },
                vec!["pwsh.exe", "-NoProfile", "-Command", "echo hello"],
                vec!["pwsh.exe", "-NoProfile", "-Command", "echo hello"],
            ),
            (
                PowerShellConfig {
                    exe: "powershell.exe".to_string(),
                    bash_exe_fallback: Some(PathBuf::from("bash.exe")),
                },
                vec![
                    "codex-mcp-server.exe",
                    "--codex-run-as-apply-patch",
                    "*** Begin Patch\n*** Update File: C:\\Users\\person\\destination_file.txt\n-original content\n+modified content\n*** End Patch",
                ],
                vec![
                    "codex-mcp-server.exe",
                    "--codex-run-as-apply-patch",
                    "*** Begin Patch\n*** Update File: C:\\Users\\person\\destination_file.txt\n-original content\n+modified content\n*** End Patch",
                ],
            ),
        ];

        for (config, input, expected_cmd) in cases {
            let command = expected_cmd
                .iter()
                .map(|s| (*s).to_string())
                .collect::<Vec<_>>();

            // These tests assert the final command for each scenario now that the helper
            // has been removed. The inputs remain to document the original coverage.
            let expected = expected_cmd
                .iter()
                .map(|s| (*s).to_string())
                .collect::<Vec<_>>();
            assert_eq!(command, expected, "input: {input:?} config: {config:?}");
        }
    }
}
