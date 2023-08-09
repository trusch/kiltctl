use kiltapi::{connect, AccountIdParser};
use subxt::utils::AccountId32;

pub fn command() -> clap::Command {
    clap::Command::new("account")
        .about("Access the account information")
        .arg(
            clap::Arg::new("account")
                .short('a')
                .long("account")
                .help("Account to query")
                .required(true)
                .value_parser(AccountIdParser)
                .env("ACCOUNT"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let account = matches.get_one::<AccountId32>("account").unwrap();

    let addr = kiltapi::kilt::storage().system().account(account);

    let cli = connect(matches).await?;
    let details = cli
        .storage()
        .at_latest()
        .await?
        .fetch(&addr)
        .await?
        .expect("not found");
    println!("{details:#?}");

    Ok(())
}
