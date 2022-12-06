mod account;
mod fetch_metadata;
mod hash;
mod keys;
mod seed;
mod current_block;

pub fn command() -> clap::Command {
    clap::Command::new("util")
        .about("Utility commands")
        .subcommand_required(true)
        .subcommands([
            account::command(),
            keys::command(),
            seed::command(),
            fetch_metadata::command(),
            hash::command(),
            current_block::command(),
        ])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("account", matches)) => account::run(matches).await,
        Some(("keys", matches)) => keys::run(matches),
        Some(("seed", matches)) => seed::run(matches),
        Some(("fetch-metadata", matches)) => fetch_metadata::run(matches).await,
        Some(("hash", matches)) => hash::run(matches).await,
        Some(("current-block", matches)) => current_block::run(matches).await,
        _ => unreachable!(),
    }
}
