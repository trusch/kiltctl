use clap::ArgAction;
use codec::Decode;
use kiltapi::{connect, kilt::runtime_types};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("batch")
        .about("Batch some other transactions together")
        .arg(
            clap::Arg::new("tx")
                .long("tx")
                .help("tx data")
                .action(ArgAction::Append),
        )
        .arg(
            clap::Arg::new("mode")
                .long("mode")
                .short('m')
                .value_parser(["default", "all", "force"])
                .help("batch mode")
                .default_value("default")
                .required(false),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let txs = matches
        .get_many::<String>("tx")
        .expect("need tx data")
        .map(|e| e.to_owned())
        .map(|tx| hex::decode(tx.trim_start_matches("0x").trim()))
        .collect::<Result<Vec<Vec<u8>>, _>>()?
        .into_iter()
        .map(|tx| runtime_types::spiritnet_runtime::Call::decode(&mut &tx[..]))
        .collect::<Result<Vec<_>, _>>()?;

    let mode = matches.get_one::<String>("mode").expect("expect a mode");

    let cli = connect(matches).await?;

    let payload = match mode.as_str() {
        "default" => crate::kilt::tx()
            .utility()
            .batch(txs)
            .encode_call_data(&cli.metadata())?,
        "all" => crate::kilt::tx()
            .utility()
            .batch_all(txs)
            .encode_call_data(&cli.metadata())?,
        "force" => crate::kilt::tx()
            .utility()
            .force_batch(txs)
            .encode_call_data(&cli.metadata())?,
        _ => unreachable!(),
    };

    println!("0x{}", hex::encode(payload));

    Ok(())
}
