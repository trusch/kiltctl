use kiltapi::{
    connect, kilt::runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec, unwrap_or_stdin,
};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("claim")
        .about("Claim a Web3Name for a DID")
        .arg(
            clap::Arg::new("name")
                .short('n')
                .long("name")
                .help("Web3Name to claim")
                .env("NAME"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let name = unwrap_or_stdin(matches.get_one::<String>("name").map(|e| e.to_owned()))?;

    let tx = crate::kilt::tx()
        .web3_names()
        .claim(BoundedVec(name.into_bytes()));

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
