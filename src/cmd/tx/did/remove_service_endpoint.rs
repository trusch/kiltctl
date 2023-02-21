use kiltapi::{
    connect,
    kilt::{self, runtime_types::sp_core::bounded::bounded_vec::BoundedVec},
};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("remove-service-endpoint")
        .about("Remove a service endpoint from a DID")
        .arg(
            clap::Arg::new("id")
                .long("id")
                .help("The ID of the service endpoint to remove")
                .required(true),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let id = matches.get_one::<String>("id").unwrap();

    let tx = kilt::tx()
        .did()
        .remove_service_endpoint(BoundedVec(id.to_owned().as_bytes().to_vec()));

    let cli = connect(matches).await?;

    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
