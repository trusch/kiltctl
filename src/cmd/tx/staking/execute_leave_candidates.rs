use kiltapi::{connect, kilt, AccountIdParser};
use subxt::utils::AccountId32;
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("execute-leave-candidates")
        .about("Execute the network exit of a candidate")
        .arg(
            clap::Arg::new("collator")
                .short('c')
                .long("collator")
                .help("Collator to finally kick")
                .required(true)
                .value_parser(AccountIdParser),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let collator = matches.get_one::<AccountId32>("collator").unwrap();

    let tx = kilt::tx()
        .parachain_staking()
        .execute_leave_candidates(collator.to_owned().into());

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
