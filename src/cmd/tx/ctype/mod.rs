mod add;

pub fn command() -> clap::Command {
    clap::Command::new("ctype")
        .about("CType transactions")
        .subcommand_required(true)
        .subcommands([add::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("add", matches)) => add::run(matches).await,
        _ => unreachable!(),
    }
}
