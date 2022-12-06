use kiltapi::{
    connect,
    kilt::{self},
};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("remove-delegation-key").about("Remove a delegation key from a DID")
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let tx = kilt::tx().did().remove_delegation_key();

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
