use kiltapi::{
    connect,
    kilt::{self, ProxyType},
    AccountIdParser,
};
use sp_core::crypto::AccountId32;
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("add")
        .about("Add a new proxy to an account")
        .arg(
            clap::Arg::new("delegate")
                .short('d')
                .long("delegate")
                .help("Account to delegate")
                .required(true)
                .value_parser(AccountIdParser)
                .env("DELEGATE"),
        )
        .arg(
            clap::Arg::new("type")
                .long("type")
                .short('t')
                .value_parser([
                    "Any",
                    "NonTransfer",
                    "Governance",
                    "ParachainStaking",
                    "CancelProxy",
                    "NonDepositClaiming",
                ])
                .help("proxy type")
                .default_value("Any")
                .required(false),
        )
        .arg(
            clap::Arg::new("delay")
                .long("delay")
                .short('l')
                .help("delay in blocks")
                .default_value("0")
                .required(false),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let delegate = matches.get_one::<AccountId32>("delegate").unwrap();
    let proxy_type = matches.get_one::<String>("type").unwrap();
    let delay = matches
        .get_one::<String>("delay")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let proxy_type = match proxy_type.as_str() {
        "Any" => ProxyType::Any,
        "NonTransfer" => ProxyType::NonTransfer,
        "Governance" => ProxyType::Governance,
        "ParachainStaking" => ProxyType::ParachainStaking,
        "CancelProxy" => ProxyType::CancelProxy,
        "NonDepositClaiming" => ProxyType::NonDepositClaiming,
        _ => unreachable!(),
    };

    let tx = kilt::tx()
        .proxy()
        .add_proxy(delegate.to_owned().into(), proxy_type, delay);

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
