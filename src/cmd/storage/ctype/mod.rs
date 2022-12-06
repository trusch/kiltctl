mod ctypes;

pub fn command() -> clap::Command {
    clap::Command::new("ctype")
        .about("CType related storage entries")
        .subcommand_required(true)
        .subcommands([ctypes::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("ctypes", matches)) => ctypes::run(matches).await,
        _ => Ok(()),
    }
}
