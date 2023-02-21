use kiltapi::{connect, kilt, AccountIdParser};
use subxt::tx::TxPayload;
use subxt::utils::AccountId32;

pub fn command() -> clap::Command {
    clap::Command::new("force-remove-candidate")
        .about("Forcefully kick a collator")
        .arg(
            clap::Arg::new("collator")
                .short('c')
                .long("collator")
                .help("Collator to kick")
                .required(true)
                .value_parser(AccountIdParser),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let collator = matches.get_one::<AccountId32>("collator").unwrap();

    let tx = kilt::tx()
        .parachain_staking()
        .force_remove_candidate(collator.to_owned().into());

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
