use kiltapi::{connect, unwrap_or_stdin};
use subxt::ext::sp_core::H256;
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("remove")
        .about("Remove an attestation from the blockchain")
        .arg(
            clap::Arg::new("claim")
                .long("claim")
                .required(false) // will be read from stdin if not provided
                .help("Claim hash"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let claim_hash_str = unwrap_or_stdin(matches.get_one::<String>("claim").map(|e| e.to_owned()))?;
    let claim_hash_bytes = hex::decode(claim_hash_str.trim_start_matches("0x").trim())?;
    let claim_hash = H256::from_slice(&claim_hash_bytes);

    let tx = crate::kilt::tx().attestation().remove(claim_hash, None);

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;
    println!("0x{}", hex::encode(payload));
    Ok(())
}
