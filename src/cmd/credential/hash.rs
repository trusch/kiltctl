use base58::FromBase58;
use kiltapi::{credential::Credential, unwrap_or_stdin};

pub fn command() -> clap::Command {
    clap::Command::new("hash")
        .about("Get the root hash of a credential")
        .arg(
            clap::Arg::new("credential")
                .short('c')
                .long("credential")
                .help("credential to verify")
                .env("CREDENTIAL"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let credential = unwrap_or_stdin(
        matches
            .get_one::<String>("credential")
            .map(|e| e.to_owned()),
    )?;

    let cred: Credential = serde_json::from_str(&credential)?;

    let root_hash = cred
        .id
        .trim_start_matches("kilt:credential:")
        .from_base58()
        .map_err(|_| "failed to parse id")?;

    println!("0x{}", hex::encode(root_hash));
    Ok(())
}
