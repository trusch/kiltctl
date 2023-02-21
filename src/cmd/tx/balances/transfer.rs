use kiltapi::{connect, AccountIdParser, BalanceParser};
use subxt::utils::AccountId32;
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("transfer")
        .about("Transfer KILT tokens")
        .arg(
            clap::Arg::new("to")
                .short('t')
                .long("to")
                .help("Address to send tokens to")
                .required(true)
                .value_parser(AccountIdParser),
        )
        .arg(
            clap::Arg::new("amount")
                .short('a')
                .long("amount")
                .help("Amount of tokens to send")
                .value_parser(BalanceParser),
        )
        .arg(
            clap::Arg::new("mode")
                .short('m')
                .long("mode")
                .help("Mode to use")
                .required(false)
                .default_value("default")
                .value_parser(["default", "all", "keep-alive", "all-keep-alive"]),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let to = matches.get_one::<AccountId32>("to").expect("need address");
    let amount: Option<&u128> = matches.get_one("amount");
    let mode = matches.get_one::<String>("mode").expect("need mode");
    let cli = connect(matches).await?;

    let payload = match mode.as_str() {
        "default" => {
            let amount = amount.expect("need amount");
            let tx = crate::kilt::tx()
                .balances()
                .transfer(to.to_owned().into(), *amount);
            tx.encode_call_data(&cli.metadata())?
        }
        "all" => {
            let tx = crate::kilt::tx()
                .balances()
                .transfer_all(to.to_owned().into(), false);
            tx.encode_call_data(&cli.metadata())?
        }
        "keep-alive" => {
            let amount = amount.expect("need amount");
            let tx = crate::kilt::tx()
                .balances()
                .transfer_keep_alive(to.to_owned().into(), *amount);
            tx.encode_call_data(&cli.metadata())?
        }
        "all-keep-alive" => {
            let tx = crate::kilt::tx()
                .balances()
                .transfer_all(to.to_owned().into(), true);
            tx.encode_call_data(&cli.metadata())?
        }

        _ => {
            panic!("unknown mode");
        }
    };

    println!("0x{}", hex::encode(payload));

    Ok(())
}
