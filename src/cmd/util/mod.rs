mod account;
mod asset_dids;
mod current_block;
mod fetch_metadata;
mod hash;
mod keys;
mod seed;

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
            asset_dids::command(),
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
        Some(("asset-dids", matches)) => asset_dids::run(matches),
        _ => unreachable!(),
    }
}
