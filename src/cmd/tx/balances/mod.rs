mod transfer;

pub fn command() -> clap::Command {
    clap::Command::new("balances")
        .about("Balance transactions")
        .subcommand_required(true)
        .subcommands([transfer::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("transfer", matches)) => transfer::run(matches).await,
        _ => Ok(()),
    }
}
