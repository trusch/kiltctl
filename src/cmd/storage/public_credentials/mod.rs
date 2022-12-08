mod credentials;

pub fn command() -> clap::Command {
    clap::Command::new("public-credentials")
        .about("Public credential related storage entries")
        .subcommand_required(true)
        .subcommands([credentials::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("credentials", matches)) => credentials::run(matches).await,
        _ => Ok(()),
    }
}
