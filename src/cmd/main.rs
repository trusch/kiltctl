mod credential;
mod ctype;
mod storage;
mod tx;
mod util;

use std::io::Write;

use clap_complete::{generate, Generator, Shell};
use kiltapi::kilt::{self};

fn command() -> clap::Command {
    clap::Command::new("kiltctl")
        .about("KILT Protocol command line helper (peregrine edition)")
        .arg(
            clap::Arg::new("endpoint")
                .short('e')
                .long("endpoint")
                .global(true)
                .help("Endpoint to connect to")
                .default_value("peregrine")
                .env("KILT_ENDPOINT"),
        )
        .subcommand_required(true)
        .subcommands([
            tx::command(),
            util::command(),
            storage::command(),
            credential::command(),
            ctype::command(),
        ])
        .subcommand(
            clap::Command::new("completions")
                .hide(true)
                .about("generate shell completion script")
                .arg(
                    clap::Arg::new("shell")
                        .help("shell to generate completion for")
                        .value_parser(clap::value_parser!(Shell))
                        .required(true),
                ),
        )
}

fn print_completions<G: Generator>(gen: G, out: &mut dyn Write) {
    let mut cmd = command();
    let name = cmd.get_name().to_owned();
    generate(gen, &mut cmd, name, out);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let matches = command().get_matches();

    match matches.subcommand() {
        Some(("tx", matches)) => tx::run(matches).await,
        Some(("util", matches)) => util::run(matches).await,
        Some(("storage", matches)) => storage::run(matches).await,
        Some(("credential", matches)) => credential::run(matches).await,
        Some(("ctype", matches)) => ctype::run(matches).await,
        Some(("completions", matches)) => {
            let shell = matches.get_one::<Shell>("shell").unwrap().to_owned();
            print_completions(shell, &mut std::io::stdout());
            Ok(())
        }
        _ => Ok(()),
    }
}
