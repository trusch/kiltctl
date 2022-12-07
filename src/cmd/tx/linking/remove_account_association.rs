use kiltapi::{connect, kilt, AccountIdParser};
use sp_core::crypto::AccountId32;
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("remove-account-association")
        .about("Remove the link between an Account and a DID")
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

    let tx = kilt::tx()
        .did_lookup()
        .remove_account_association(account.to_owned());
    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;
    println!("0x{}", hex::encode(payload));
    Ok(())
}
