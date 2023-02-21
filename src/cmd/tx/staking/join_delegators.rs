use kiltapi::{connect, kilt, AccountIdParser, BalanceParser};
use subxt::tx::TxPayload;
use subxt::utils::AccountId32;

pub fn command() -> clap::Command {
    clap::Command::new("join-delegators")
        .about("Join the set of delegators")
        .arg(
            clap::Arg::new("amount")
                .short('a')
                .long("amount")
                .help("Amount of tokens to stake")
                .required(true)
                .value_parser(BalanceParser),
        )
        .arg(
            clap::Arg::new("collator")
                .short('c')
                .long("collator")
                .help("Collator to stake for")
                .required(true)
                .value_parser(AccountIdParser),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let amount = matches.get_one::<u128>("amount").unwrap();
    let collator = matches.get_one::<AccountId32>("collator").unwrap();

    let tx = kilt::tx()
        .parachain_staking()
        .join_delegators(collator.to_owned().into(), amount.to_owned());

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
