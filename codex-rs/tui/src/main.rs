use clap::Parser;
use codex_common::CliConfigOverrides;
use codex_tui::Cli;
use codex_tui::run_main;

#[derive(Parser, Debug)]
struct TopCli {
    #[clap(flatten)]
    config_overrides: CliConfigOverrides,

    #[clap(flatten)]
    inner: Cli,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let top_cli = TopCli::parse();
    let mut inner = top_cli.inner;
    inner
        .config_overrides
        .raw_overrides
        .splice(0..0, top_cli.config_overrides.raw_overrides);

    let exit_info = run_main(inner, None).await?;
    let token_usage = exit_info.token_usage;
    if !token_usage.is_zero() {
        println!("{}", codex_core::protocol::FinalOutput::from(token_usage),);
    }
    Ok(())
}
