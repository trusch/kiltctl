mod add;
mod revoke;
mod remove;
mod unrevoke;
mod reclaim_deposit;

pub fn command() -> clap::Command {
    clap::Command::new("public-credentials")
        .about("Public credential transactions")
        .subcommand_required(true)
        .subcommands([
            add::command(),
            revoke::command(),
            remove::command(),  
            unrevoke::command(),
            reclaim_deposit::command(),
        ])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("add", matches)) => add::run(matches).await,
        Some(("revoke", matches)) => revoke::run(matches).await,
        Some(("remove", matches)) => remove::run(matches).await,
        Some(("unrevoke", matches)) => unrevoke::run(matches).await,
        Some(("reclaim-deposit", matches)) => reclaim_deposit::run(matches).await,
        _ => unreachable!(),
    }
}
