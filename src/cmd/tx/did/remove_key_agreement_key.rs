use kiltapi::{
    connect,
    kilt::{self},
};
use sp_core::H256;
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("remove-key-agreement-key")
        .about("Remove a key agreement key from a DID")
        .arg(
            clap::Arg::new("key-id")
                .short('k')
                .long("key-id")
                .help("The ID of the key agreement key to remove")
                .required(true),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let key_id = matches.get_one::<String>("key-id").unwrap();
    let kid = H256(
        hex::decode(key_id.trim_start_matches("0x").trim())?
            .try_into()
            .map_err(|_| "bad key id")?,
    );

    let tx = kilt::tx().did().remove_key_agreement_key(kid);

    let cli = connect(matches).await?;

    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
