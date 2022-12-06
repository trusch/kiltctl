mod account;

pub fn command() -> clap::Command {
    clap::Command::new("system")
        .about("System related storage entries")
        .subcommand_required(true)
        .subcommands([account::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("account", matches)) => account::run(matches).await,
        _ => Ok(()),
    }
}
