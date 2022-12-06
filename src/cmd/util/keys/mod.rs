mod from_seed;

pub fn command() -> clap::Command {
    clap::Command::new("keys")
        .about("Some helpers for working with keys")
        .subcommand_required(true)
        .subcommands([from_seed::command()])
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("from-seed", matches)) => from_seed::run(matches),
        _ => unreachable!(),
    }
}
