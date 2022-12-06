mod did;

pub fn command() -> clap::Command {
    clap::Command::new("did")
        .about("DID related storage entires")
        .subcommand_required(true)
        .subcommands([did::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("did", matches)) => did::run(matches).await,
        _ => Ok(()),
    }
}
