use kiltapi::{connect, kilt};

use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("associate-sender").about("Link the sender account to a DID")
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let tx = kilt::tx().did_lookup().associate_sender();
    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;
    println!("0x{}", hex::encode(payload));
    Ok(())
}
