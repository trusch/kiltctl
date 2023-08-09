use kiltapi::{
    connect,
    kilt::{self},
    AccountIdParser, HashParser,
};
use subxt::ext::sp_core::{crypto::AccountId32, H256};
use subxt::{tx::TxPayload, utils::MultiAddress};

pub fn command() -> clap::Command {
    clap::Command::new("reject-announcement")
        .about("Reject a proxy announcement")
        .arg(
            clap::Arg::new("delegate")
                .short('d')
                .long("delegate")
                .help("Proxy account")
                .required(true)
                .value_parser(AccountIdParser)
                .env("DELEGATE"),
        )
        .arg(
            clap::Arg::new("call_hash")
                .long("call_hash")
                .short('c')
                .help("call hash of the call that should be rejected")
                .value_parser(HashParser)
                .required(true),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let delegate = matches.get_one::<AccountId32>("delegate").unwrap();
    let hash = matches.get_one::<H256>("call_hash").unwrap();

    let id = MultiAddress::Address32(delegate.to_owned().into());
    let tx = kilt::tx()
        .proxy()
        .reject_announcement(id, hash.to_owned());

    let cli = connect(matches).await?;

    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
