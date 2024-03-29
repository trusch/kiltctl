use kiltapi::{connect, format_balance, AccountIdParser};
use subxt::ext::sp_core;
use subxt::ext::sp_core::crypto::Ss58Codec;
use subxt::utils::AccountId32;

pub fn command() -> clap::Command {
    clap::Command::new("info")
        .about("Show information about an account")
        .arg(
            clap::Arg::new("account")
                .short('a')
                .long("account")
                .help("Account to inspect")
                .value_parser(AccountIdParser)
                .env("ACCOUNT"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let account: &AccountId32 = matches.get_one("account").expect("need account");

    let addr = kiltapi::kilt::storage().system().account(account);

    let cli = connect(matches).await?;

    let details = cli
        .storage()
        .at_latest()
        .await?
        .fetch(&addr)
        .await?
        .expect("not found");

    println!(
        "Account ID: {}",
        sp_core::crypto::AccountId32::from(account.0).to_ss58check_with_version(38u16.into())
    );
    println!("Free: {}", format_balance(details.data.free));
    println!("Reserved: {}", format_balance(details.data.reserved));
    println!("Misc Frozen: {}", format_balance(details.data.misc_frozen));
    println!("Nonce: {}", details.nonce);
    println!("Consumers: {}", details.consumers);
    println!("Providers: {}", details.providers);
    println!("Sufficients: {}", details.sufficients);

    Ok(())
}
