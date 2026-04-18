use std::path::Path;

use clap::Parser;

const REPO_LOCAL_MARKETPLACE_RELATIVE_PATH: &str = ".agents/plugins/marketplace.json";
const REPO_LOCAL_PLUGIN_RELATIVE_PATH: &str = "plugins/zapabob-legacy-suite";

#[derive(Debug, Parser)]
pub(crate) struct GuiXCommand {
    /// Legacy host flag kept only for migration guidance output
    #[arg(long, value_name = "HOST", default_value = "127.0.0.1")]
    pub(crate) host: String,

    /// Legacy port flag kept only for migration guidance output
    #[arg(long, value_name = "PORT", default_value_t = 5173)]
    pub(crate) port: u16,

    /// Legacy attach flag kept only for migration guidance output
    #[arg(long, default_value_t = false)]
    pub(crate) attached: bool,
}

fn gui_x_deprecation_message(workspace: &Path, cmd: &GuiXCommand) -> String {
    let marketplace_path = workspace.join(REPO_LOCAL_MARKETPLACE_RELATIVE_PATH);
    let plugin_path = workspace.join(REPO_LOCAL_PLUGIN_RELATIVE_PATH);
    let attach_hint = if cmd.attached { "true" } else { "false" };
    let host = &cmd.host;
    let port = cmd.port;
    format!(
        "`gui-x` has been retired and no longer launches `codex-gui-x`.\n\
Use the official desktop flow with `codex app` or the app-server plugin flow instead.\n\
\n\
Recommended migration:\n\
1. Start the official app or app-server: `codex app` or `codex app-server`\n\
2. Discover the repo-local marketplace with `plugin/list`\n\
3. Install or mention `plugin://zapabob-legacy-suite@zapabob-repo-local`\n\
\n\
Repo-local marketplace: {marketplace_path}\n\
Repo-local plugin bundle: {plugin_path}\n\
\n\
Legacy GUI flags were ignored: --host={host}, --port={port}, --attached={attach_hint}",
        marketplace_path = marketplace_path.display(),
        plugin_path = plugin_path.display(),
    )
}

fn run_gui_x_command_in(workspace: &Path, cmd: &GuiXCommand) -> anyhow::Result<()> {
    let message = gui_x_deprecation_message(workspace, cmd);
    anyhow::bail!("{message}")
}

pub(crate) fn run_gui_x_command(cmd: GuiXCommand) -> anyhow::Result<()> {
    let workspace = std::env::current_dir()?;
    run_gui_x_command_in(&workspace, &cmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gui_x_deprecation_message_points_to_official_app_and_plugin_paths() {
        let workspace = Path::new("/tmp/codex-main");
        let cmd = GuiXCommand {
            host: "127.0.0.1".to_string(),
            port: 5173,
            attached: false,
        };

        let message = gui_x_deprecation_message(workspace, &cmd);

        assert!(message.contains("codex app"));
        assert!(message.contains("plugin/list"));
        assert!(message.contains("plugin://zapabob-legacy-suite@zapabob-repo-local"));
        assert!(message.contains(".agents/plugins/marketplace.json"));
        assert!(message.contains("plugins/zapabob-legacy-suite"));
        assert!(message.contains("--host=127.0.0.1"));
    }

    #[test]
    fn run_gui_x_command_in_returns_deprecation_error() {
        let workspace = Path::new("/tmp/codex-main");
        let cmd = GuiXCommand {
            host: "0.0.0.0".to_string(),
            port: 3000,
            attached: true,
        };

        let err = run_gui_x_command_in(workspace, &cmd).expect_err("gui-x should stay disabled");

        assert!(err.to_string().contains("no longer launches `codex-gui-x`"));
        assert!(err.to_string().contains("--attached=true"));
    }
}
