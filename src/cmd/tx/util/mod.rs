mod batch;

pub fn command() -> clap::Command {
    clap::Command::new("util")
        .about("Utility transactions")
        .subcommand_required(true)
        .subcommands([batch::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("batch", matches)) => batch::run(matches).await,
        _ => unreachable!(),
    }
}
