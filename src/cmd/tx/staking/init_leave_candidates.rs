use kiltapi::{connect, kilt};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("init-leave-candidates").about("Initialize a network exit of a collator")
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let tx = kilt::tx().parachain_staking().init_leave_candidates();

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
