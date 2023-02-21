mod add;
mod announce;
mod proxy;
mod proxy_announced;
mod pure;
mod reject_announcement;
mod remove_announcement;

pub fn command() -> clap::Command {
    clap::Command::new("proxy")
        .about("Proxy transactions")
        .subcommand_required(true)
        .subcommands([
            add::command(),
            announce::command(),
            proxy::command(),
            proxy_announced::command(),
            reject_announcement::command(),
            remove_announcement::command(),
            pure::command(),
        ])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("add", matches)) => add::run(matches).await,
        Some(("announce", matches)) => announce::run(matches).await,
        Some(("proxy", matches)) => proxy::run(matches).await,
        Some(("proxy-announced", matches)) => proxy_announced::run(matches).await,
        Some(("reject-announcement", matches)) => reject_announcement::run(matches).await,
        Some(("remove-announcement", matches)) => remove_announcement::run(matches).await,
        Some(("pure", matches)) => pure::run(matches).await,
        _ => unreachable!(),
    }
}
