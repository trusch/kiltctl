use clap::ArgAction;
use codec::Decode;
use kiltapi::{connect, kilt::RuntimeCall};
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
    let mut txs = matches
        .get_many::<String>("tx")
        .unwrap_or_default()
        .map(|e| e.to_owned())
        .map(|tx| hex::decode(tx.trim_start_matches("0x").trim()))
        .collect::<Result<Vec<Vec<u8>>, _>>()?
        .into_iter()
        .map(|tx| RuntimeCall::decode(&mut &tx[..]))
        .collect::<Result<Vec<_>, _>>()?;

    log::debug!("got {} txs from the flags", txs.len());

    if txs.is_empty() {
        // read lines and parse them as Calls
        loop {
            let mut buf = String::new();
            let n = std::io::stdin().read_line(&mut buf)?;
            if n == 0 {
                break;
            }
            let tx = hex::decode(buf.trim_start_matches("0x").trim())?;
            let tx = RuntimeCall::decode(&mut &tx[..])?;
            log::debug!("found tx from stdin: {:?}", tx);
            txs.push(tx);
        }
    }
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
