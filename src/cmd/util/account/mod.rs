mod from_seed;
mod info;

pub fn command() -> clap::Command {
    clap::Command::new("account")
        .about("Some helpers for working with accounts")
        .subcommand_required(true)
        .subcommands([from_seed::command(), info::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("from-seed", matches)) => from_seed::run(matches),
        Some(("info", matches)) => info::run(matches).await,
        _ => unreachable!(),
    }
}
