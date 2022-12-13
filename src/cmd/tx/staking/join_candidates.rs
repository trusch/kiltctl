use kiltapi::{connect, kilt, BalanceParser};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("join-candidates")
        .about("Join the set of candidates")
        .arg(
            clap::Arg::new("amount")
                .short('a')
                .long("amount")
                .help("Amount of tokens to stake")
                .required(true)
                .value_parser(BalanceParser),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let amount = matches.get_one::<u128>("amount").unwrap();

    let tx = kilt::tx()
        .parachain_staking()
        .join_candidates(amount.to_owned());

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
