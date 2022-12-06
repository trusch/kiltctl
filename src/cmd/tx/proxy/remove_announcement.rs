use kiltapi::{
    connect,
    kilt::{self},
    AccountIdParser, HashParser,
};
use sp_core::{crypto::AccountId32, H256};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("remove-announcement")
        .about("remove a proxy announcement")
        .arg(
            clap::Arg::new("real")
                .short('r')
                .long("real")
                .help("Proxied account")
                .required(true)
                .value_parser(AccountIdParser)
                .env("REAL"),
        )
        .arg(
            clap::Arg::new("call_hash")
                .long("call_hash")
                .short('c')
                .help("call hash of the call that should be removed")
                .value_parser(HashParser)
                .required(true),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let real = matches.get_one::<AccountId32>("real").unwrap();
    let hash = matches.get_one::<H256>("call_hash").unwrap();

    let tx = kilt::tx()
        .proxy()
        .remove_announcement(real.to_owned().into(), hash.to_owned());

    let cli = connect(matches).await?;

    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
