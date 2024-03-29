use kiltapi::{
    connect,
    kilt::runtime_types::{
        public_credentials::credentials::Credential, sp_core::bounded::bounded_vec::BoundedVec,
    },
};
use subxt::ext::sp_core::H256;
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("add")
        .about("Add a new public credential to the chain")
        .arg(
            clap::Arg::new("ctype")
                .short('c')
                .long("ctype")
                .help("CType hash")
                .env("CTYPE"),
        )
        .arg(
            clap::Arg::new("subject")
                .short('s')
                .long("subject")
                .help("DID subject")
                .env("SUBJECT"),
        )
        .arg(
            clap::Arg::new("claims")
                .long("claims")
                .help("Claims")
                .env("CLAIMS"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let ctype_hash_str = matches.get_one::<String>("ctype").unwrap().to_owned();
    let ctype_hash_bytes = hex::decode(ctype_hash_str.trim_start_matches("0x").trim())?;
    let ctype_hash = H256::from_slice(&ctype_hash_bytes);
    let did = matches.get_one::<String>("subject").unwrap().to_owned();
    let claims = matches.get_one::<String>("claims").unwrap().to_owned();
    let claims_bytes = if claims.starts_with("0x") {
        hex::decode(claims.trim_start_matches("0x"))?
    } else {
        claims.into_bytes()
    };

    let tx = crate::kilt::tx().public_credentials().add(Credential {
        ctype_hash,
        subject: BoundedVec(did.into_bytes()),
        claims: BoundedVec(claims_bytes),
        authorization: None,
    });

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
