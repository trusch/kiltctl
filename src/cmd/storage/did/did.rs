use kiltapi::{connect, AccountIdParser};
use subxt::utils::AccountId32;

pub fn command() -> clap::Command {
    clap::Command::new("did").about("Lookup a DID").arg(
        clap::Arg::new("did")
            .short('d')
            .long("did")
            .help("DID to query")
            .required(true)
            .value_parser(AccountIdParser)
            .env("DID"),
    )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let did = matches.get_one::<AccountId32>("did").unwrap();

    let addr = kiltapi::kilt::storage().did().did(did);

    let cli = connect(matches).await?;
    let details = cli
        .storage()
        .at(None)
        .await?
        .fetch(&addr)
        .await?
        .expect("not found");
    println!("{details:#?}");

    Ok(())
}
