mod generate;

pub fn command() -> clap::Command {
    clap::Command::new("seed")
        .about("Seed management")
        .subcommand(generate::command())
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("generate", matches)) => generate::run(matches),
        _ => Err("no subcommand".into()),
    }
}
