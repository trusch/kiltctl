mod ctype;
mod did;
mod system;
mod public_credentials;

pub fn command() -> clap::Command {
    clap::Command::new("storage")
        .about("Storage queries")
        .subcommand_required(true)
        .subcommands([
            did::command(),
            system::command(),
            ctype::command(),
            public_credentials::command(),
        ])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("did", matches)) => did::run(matches).await,
        Some(("system", matches)) => system::run(matches).await,
        Some(("ctype", matches)) => ctype::run(matches).await,
        Some(("public-credentials", matches)) => public_credentials::run(matches).await,
        _ => Ok(()),
    }
}
