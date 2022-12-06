mod create;
mod hash;
mod verify;

pub fn command() -> clap::Command {
    clap::Command::new("credential")
        .about("Credential commands")
        .subcommand_required(true)
        .subcommands([create::command(), verify::command(), hash::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("create", matches)) => create::run(matches).await,
        Some(("verify", matches)) => verify::run(matches).await,
        Some(("hash", matches)) => hash::run(matches).await,
        _ => Ok(()),
    }
}
