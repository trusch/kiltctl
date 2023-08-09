use kiltapi::{connect, unwrap_or_stdin};
use subxt::ext::sp_core::H256;
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("unrevoke")
        .about("Unrevoke an public attestation from the blockchain")
        .arg(
            clap::Arg::new("id")
                .long("id")
                .required(false) // will be read from stdin if not provided
                .help("ID of the credential to unrevoke"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let id_str = unwrap_or_stdin(matches.get_one::<String>("id").map(|e| e.to_owned()))?;
    let id_bytes = hex::decode(id_str.trim_start_matches("0x").trim())?;
    let id = H256::from_slice(&id_bytes);

    let tx = crate::kilt::tx().public_credentials().unrevoke(id, None);

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;
    println!("0x{}", hex::encode(payload));
    Ok(())
}
