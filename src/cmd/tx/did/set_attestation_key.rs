use kiltapi::{
    connect,
    kilt::{
        self,
        runtime_types::{
            did::did_details::DidVerificationKey,
            sp_core::{ecdsa, ed25519, sr25519},
        },
    },
};
use subxt::tx::TxPayload;
pub fn command() -> clap::Command {
    clap::Command::new("set-attestation-key")
        .about("Set a new attestation key to the DID")
        .arg(
            clap::Arg::new("key")
                .short('k')
                .long("key")
                .help("Key to add")
                .required(true)
                .env("KEY"),
        )
        .arg(
            clap::Arg::new("type")
                .short('t')
                .long("type")
                .help("Type of key agreement key to add")
                .env("TYPE")
                .value_parser(["sr25519", "ed25519", "ecdsa"])
                .default_value("sr25519"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let key = matches.get_one::<String>("key").unwrap();
    let key_type = matches.get_one::<String>("type").unwrap();

    let key_bytes = hex::decode(key.trim_start_matches("0x").trim())?;

    let tx = match key_type.as_str() {
        "sr25519" => kilt::tx()
            .did()
            .set_attestation_key(DidVerificationKey::Sr25519(sr25519::Public(
                key_bytes.try_into().map_err(|_| "key malformed")?,
            ))),
        "ed25519" => kilt::tx()
            .did()
            .set_attestation_key(DidVerificationKey::Ed25519(ed25519::Public(
                key_bytes.try_into().map_err(|_| "key malformed")?,
            ))),
        "ecdsa" => kilt::tx()
            .did()
            .set_attestation_key(DidVerificationKey::Ecdsa(ecdsa::Public(
                key_bytes.try_into().map_err(|_| "key malformed")?,
            ))),
        _ => unreachable!("no more types by now"),
    };

    let cli = connect(matches).await?;

    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
