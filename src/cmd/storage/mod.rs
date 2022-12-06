mod ctype;
mod did;
mod system;

pub fn command() -> clap::Command {
    clap::Command::new("storage")
        .about("Storage queries")
        .subcommand_required(true)
        .subcommands([did::command(), system::command(), ctype::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("did", matches)) => did::run(matches).await,
        Some(("system", matches)) => system::run(matches).await,
        Some(("ctype", matches)) => ctype::run(matches).await,
        _ => Ok(()),
    }
}
