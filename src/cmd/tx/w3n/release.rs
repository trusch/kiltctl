use kiltapi::connect;
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("release").about("Release a Web3Name by the owner")
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let tx = crate::kilt::tx().web3_names().release_by_owner();

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
