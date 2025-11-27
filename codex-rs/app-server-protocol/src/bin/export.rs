use anyhow::Result;
use clap::{Arg, Command};
use std::path::PathBuf;

#[derive(Debug)]
struct Args {
    /// Output directory where generated files will be written
    out_dir: PathBuf,

    /// Optional Prettier executable path to format generated TypeScript files
    prettier: Option<PathBuf>,
}

impl Args {
    fn parse() -> Self {
        let matches = Command::new("codex-app-server-protocol-export")
            .about("Generate TypeScript bindings and JSON Schemas for the Codex app-server protocol")
            .arg(
                Arg::new("out")
                    .short('o')
                    .long("out")
                    .value_name("DIR")
                    .help("Output directory where generated files will be written")
                    .required(true)
                    .value_parser(clap::value_parser!(PathBuf)),
            )
            .arg(
                Arg::new("prettier")
                    .short('p')
                    .long("prettier")
                    .value_name("PRETTIER_BIN")
                    .help("Optional Prettier executable path to format generated TypeScript files")
                    .value_parser(clap::value_parser!(PathBuf)),
            )
            .get_matches();

        Self {
            out_dir: matches.get_one::<PathBuf>("out").unwrap().clone(),
            prettier: matches.get_one::<PathBuf>("prettier").cloned(),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    codex_app_server_protocol::generate_types(&args.out_dir, args.prettier.as_deref())
}
