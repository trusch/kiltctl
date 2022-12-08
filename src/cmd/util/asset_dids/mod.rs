mod generate;
mod get_credential_id;

pub fn command() -> clap::Command {
    clap::Command::new("asset-dids")
        .about("Some helpers for working with asset-dids")
        .subcommand_required(true)
        .subcommands([generate::command(), get_credential_id::command()])
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("generate", matches)) => generate::run(matches),
        Some(("get-credential-id", matches)) => get_credential_id::run(matches),
        _ => unreachable!(),
    }
}
