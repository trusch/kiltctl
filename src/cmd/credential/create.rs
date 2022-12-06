use chrono::Utc;
use kiltapi::{connect, credential::CredentialBuilder, unwrap_or_stdin};

pub fn command() -> clap::Command {
    clap::Command::new("create")
        .about("Issue a new credential")
        .arg(
            clap::Arg::new("subject")
                .long("subject")
                .required(true)
                .help("subject of the credential")
                .env("SUBJECT"),
        )
        .arg(
            clap::Arg::new("claims")
                .short('c')
                .long("claims")
                .help("json claims to embedd into the credential")
                .env("CLAIMS"),
        )
        .arg(
            clap::Arg::new("ctype")
                .long("ctype")
                .required(true)
                .help("ctype to use for the credential")
                .env("CTYPE"),
        )
        .arg(
            clap::Arg::new("issuer")
                .long("issuer")
                .help("issuer of the credential")
                .env("ISSUER"),
        )
        .arg(
            clap::Arg::new("block")
                .long("block")
                .help("block number to use for the credential")
                .env("BLOCK"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let subject = matches.get_one::<String>("subject").unwrap();
    let ctype = matches.get_one::<String>("ctype").unwrap();
    let claims = unwrap_or_stdin(matches.get_one::<String>("claims").map(|e| e.to_owned()))?;
    let issuer = matches.get_one::<String>("issuer");
    let block = match matches.get_one::<String>("block") {
        Some(block) => block.to_owned(),
        None => {
            let cli = connect(matches).await?;
            cli.blocks().at(None).await.unwrap().number().to_string()
        }
    };

    let mut claim_map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&claims)?;
    claim_map.insert("@id".into(), subject.clone().into());
    let mut builder = CredentialBuilder::new()
        .with_ctype(ctype)
        .with_credential_subject(claim_map);

    if let Some(issuer) = issuer {
        builder = builder
            .with_issuer(issuer.to_owned())
            .with_issuance_date(Utc::now().to_rfc3339());
    }

    let cred = builder.create_proof(&block)?.build()?;

    println!("{}", serde_json::to_string_pretty(&cred)?);

    Ok(())
}
