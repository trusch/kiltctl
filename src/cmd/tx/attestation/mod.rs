mod add;
mod remove;
mod revoke;

pub fn command() -> clap::Command {
    clap::Command::new("attestation")
        .about("Attestation transactions")
        .subcommand_required(true)
        .subcommands([add::command(), revoke::command(), remove::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("add", matches)) => add::run(matches).await,
        Some(("revoke", matches)) => revoke::run(matches).await,
        Some(("remove", matches)) => remove::run(matches).await,
        _ => unreachable!(),
    }
}
