use clap::ArgAction;
use kiltapi::{
    connect,
    kilt::{
        self,
        runtime_types::{
            did::service_endpoints::DidEndpoint, sp_runtime::bounded::bounded_vec::BoundedVec,
        },
    },
};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("add-service-endpoint")
        .about("Add a new service endpoint to the DID")
        .long_about("Add a new service endpoint to the DID. The service endpoint is a URL that can be used to access the DID subject. The URL can include any valid URL scheme, such as HTTP, FTP, or DID. There are also multiple types and urls allowed for a single service endpoint.")
        .arg(clap::Arg::new("id")
            .long("id")
            .help("Id of the service endpoint")
            .required(true)
            .env("ID")
        )
        .arg(clap::Arg::new("type")
            .long("type")
            .help("Type of the service endpoint")
            .required(true)
            .action(ArgAction::Append)
            .env("TYPE")
        )
        .arg(clap::Arg::new("url")
            .long("url")
            .help("Url of the service endpoint")
            .required(true)
            .action(ArgAction::Append)
            .env("URL")
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let id = matches.get_one::<String>("id").unwrap();
    let types = matches.get_many::<String>("type").unwrap();
    let urls = matches.get_many::<String>("url").unwrap();

    let tx = kilt::tx().did().add_service_endpoint(DidEndpoint {
        id: BoundedVec(id.to_owned().into_bytes()),
        service_types: BoundedVec(
            types
                .map(|t| BoundedVec(t.to_owned().into_bytes()))
                .collect(),
        ),
        urls: BoundedVec(
            urls.map(|u| BoundedVec(u.to_owned().into_bytes()))
                .collect(),
        ),
    });

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
