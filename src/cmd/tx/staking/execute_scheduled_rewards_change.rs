use kiltapi::{connect, kilt};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("execute-scheduled-rewards-change")
        .about("Execute the annual reduction of the rewards")
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let tx = kilt::tx()
        .parachain_staking()
        .execute_scheduled_reward_change();

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
