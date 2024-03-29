use kiltapi::kilt::runtime_types::pallet_did_lookup::linkable_account::LinkableAccountId;
use kiltapi::{connect, kilt, AccountIdParser};
use subxt::tx::TxPayload;
use subxt::utils::AccountId32;

pub fn command() -> clap::Command {
    clap::Command::new("reclaim-deposit")
        .about("Remove the link between an Account and a DID by the deposit holder")
        .arg(
            clap::Arg::new("account")
                .short('a')
                .long("account")
                .help("Account to unlink")
                .value_parser(AccountIdParser)
                .env("ACCOUNT"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let account = matches
        .get_one::<AccountId32>("account")
        .expect("need account");
    let id = LinkableAccountId::AccountId32(account.to_owned());
    let tx = kilt::tx().did_lookup().reclaim_deposit(id);
    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;
    println!("0x{}", hex::encode(payload));
    Ok(())
}
