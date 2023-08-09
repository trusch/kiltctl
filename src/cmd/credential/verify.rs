use base58::FromBase58;
use clap::ArgAction;
use kiltapi::{connect, credential::Credential, unwrap_or_stdin};
use sp_core::H256;

pub fn command() -> clap::Command {
    clap::Command::new("verify")
        .about("Verify a credential")
        .arg(
            clap::Arg::new("credential")
                .short('c')
                .long("credential")
                .help("credential to verify")
                .env("CREDENTIAL"),
        )
        .arg(
            clap::Arg::new("trusted-issuer")
                .long("trusted-issuer")
                .help("trusted issuer")
                .default_value("")
                .action(ArgAction::Append)
                .env("ISSUER"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let mut trusted_issuers = matches
        .get_many::<String>("trusted-issuer")
        .unwrap()
        .map(|e| e.to_owned());

    let credential = unwrap_or_stdin(
        matches
            .get_one::<String>("credential")
            .map(|e| e.to_owned()),
    )?;

    let cred: Credential = serde_json::from_str(&credential)?;

    cred.verify()?;

    let issuer = if let Some(issuer) = cred.issuer {
        issuer
    } else {
        return Err("no issuer specified in credential".into());
    };

    if !trusted_issuers.any(|x| x == issuer) {
        return Err("issuer is not trusted".into());
    }

    let root_hash: [u8; 32] = cred
        .id
        .trim_start_matches("kilt:credential:")
        .from_base58()
        .map_err(|_| "failed to parse id")?
        .try_into()
        .map_err(|_| "failed to parse id")?;
    let addr = kiltapi::kilt::storage()
        .attestation()
        .attestations(H256(root_hash));

    let cli = connect(matches).await?;
    let attestation = cli.storage().at_latest().await?.fetch(&addr).await?;
    if let Some(attestation) = attestation {
        if attestation.revoked {
            return Err("attestation is revoked".into());
        }
    } else {
        return Err("attestation not found".into());
    }

    println!("ok");

    Ok(())
}
