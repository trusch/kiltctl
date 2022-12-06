use kiltapi::{
    connect,
    kilt::{self, runtime_types::did::did_details::DidEncryptionKey},
};
use subxt::tx::TxPayload;
pub fn command() -> clap::Command {
    clap::Command::new("add-key-agreement-key")
        .about("Add a new key agreement key to the DID")
        .arg(
            clap::Arg::new("key")
                .short('k')
                .long("key")
                .help("Key agreement key to add")
                .required(true)
                .env("KEY"),
        )
        .arg(
            clap::Arg::new("type")
                .short('t')
                .long("type")
                .help("Type of key agreement key to add")
                .required(true)
                .env("TYPE")
                .value_parser(["x25519"])
                .default_value("x25519"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let key = matches.get_one::<String>("key").unwrap();
    let key_type = matches.get_one::<String>("type").unwrap();

    let key_bytes = hex::decode(key.trim_start_matches("0x").trim())?;

    let tx = match key_type.as_str() {
        "x25519" => kilt::tx()
            .did()
            .add_key_agreement_key(DidEncryptionKey::X25519(
                key_bytes.try_into().map_err(|_| "key malformed")?,
            )),
        _ => unreachable!("no more types by now"),
    };

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
