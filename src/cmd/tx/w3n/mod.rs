mod ban;
mod change_deposit_owner;
mod claim;
mod reclaim_deposit;
mod release;
mod unban;
mod update_deposit;

pub fn command() -> clap::Command {
    clap::Command::new("w3n")
        .about("Web3Name transactions")
        .subcommand_required(true)
        .subcommands([
            claim::command(),
            reclaim_deposit::command(),
            ban::command(),
            unban::command(),
            release::command(),
            change_deposit_owner::command(),
            update_deposit::command(),
        ])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("claim", matches)) => claim::run(matches).await,
        Some(("reclaim-deposit", matches)) => reclaim_deposit::run(matches).await,
        Some(("ban", matches)) => ban::run(matches).await,
        Some(("unban", matches)) => unban::run(matches).await,
        Some(("release", matches)) => release::run(matches).await,
        Some(("change-deposit-owner", matches)) => change_deposit_owner::run(matches).await,
        Some(("update-deposit", matches)) => update_deposit::run(matches).await,
        _ => unreachable!(),
    }
}
