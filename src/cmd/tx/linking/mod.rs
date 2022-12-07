mod associate_account;
mod remove_sender_association;
mod associate_sender;
mod remove_account_association;
mod reclaim_deposit;

pub fn command() -> clap::Command {
    clap::Command::new("linking")
        .about("Account-linking transactions")
        .subcommand_required(true)
        .subcommands([
            associate_account::command(),
            remove_sender_association::command(),
            associate_sender::command(),
            remove_account_association::command(),
            reclaim_deposit::command(),
        ])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("associate-account", matches)) => associate_account::run(matches).await,
        Some(("remove-sender-association", matches)) => remove_sender_association::run(matches).await,
        Some(("associate-sender", matches)) => associate_sender::run(matches).await,
        Some(("remove-account-association", matches)) => remove_account_association::run(matches).await,
        Some(("reclaim-deposit", matches)) => reclaim_deposit::run(matches).await,
        _ => unreachable!(),
    }
}
