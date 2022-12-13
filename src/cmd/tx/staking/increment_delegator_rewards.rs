use kiltapi::{connect, BalanceParser, kilt};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("increment-delegator-rewards")
        .about("Actively increment the delegator rewards")
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let amount = matches.get_one::<u128>("amount").unwrap();

    let tx = kilt::tx().parachain_staking().increment_delegator_rewards();

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
