use codec::Decode;
use kiltapi::{
    connect,
    kilt::{self, ProxyType},
    AccountIdParser, CallParser, RawCall,
};
use subxt::utils::AccountId32;
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("proxy")
        .about("Execute a proxy call")
        .arg(
            clap::Arg::new("real")
                .short('r')
                .long("real")
                .help("Proxied account")
                .required(true)
                .value_parser(AccountIdParser)
                .env("REAL"),
        )
        .arg(
            clap::Arg::new("force-type")
                .long("force_type")
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
                .required(false),
        )
        .arg(
            clap::Arg::new("call")
                .long("call")
                .short('c')
                .help("call to submit")
                .value_parser(CallParser)
                .required(true),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let real = matches.get_one::<AccountId32>("real").unwrap();
    let proxy_type = matches
        .get_one::<String>("force-type")
        .map(|t| match t.as_str() {
            "Any" => ProxyType::Any,
            "NonTransfer" => ProxyType::NonTransfer,
            "Governance" => ProxyType::Governance,
            "ParachainStaking" => ProxyType::ParachainStaking,
            "CancelProxy" => ProxyType::CancelProxy,
            "NonDepositClaiming" => ProxyType::NonDepositClaiming,
            _ => panic!("Unknown proxy type"),
        });

    let call = matches.get_one::<RawCall>("call").unwrap();
    let call = kiltapi::kilt::RuntimeCall::decode(&mut call.call.as_ref())?;

    let tx = kilt::tx()
        .proxy()
        .proxy(real.to_owned().into(), proxy_type, call);

    let cli = connect(matches).await?;

    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
