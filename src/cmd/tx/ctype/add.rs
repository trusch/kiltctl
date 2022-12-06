use kiltapi::{connect, unwrap_or_stdin};
use subxt::tx::TxPayload;

use crate::ctype::CType;

pub fn command() -> clap::Command {
    clap::Command::new("add")
    .about("Add a new CType to the chain")
    .arg(
        clap::Arg::new("ctype")
            .short('c')
            .long("ctype")
            .help("CType data")
            .env("CTYPE"),
    )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let ctype_str = unwrap_or_stdin(matches.get_one::<String>("ctype").map(|e| e.to_owned()))?;
    let ctype: CType = serde_json::from_str(&ctype_str)?;

    let data = ctype.serialize()?;

    let tx = crate::kilt::tx().ctype().add(data.into_bytes());

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;
    println!("0x{}", hex::encode(payload));
    Ok(())
}
