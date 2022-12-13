use kiltapi::{connect, kilt};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("force-new-round").about("Forces a new round")
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let tx = kilt::tx().parachain_staking().force_new_round();

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
