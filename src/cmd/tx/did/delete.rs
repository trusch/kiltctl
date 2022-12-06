use kiltapi::{
    connect,
    kilt::{self},
};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("delete").about("Delete a DID").arg(
        clap::Arg::new("endpoints-to-remove")
            .short('e')
            .long("endpoints-to-remove")
            .help("Maximum number of service endpoints that this TX is allowed to remove")
            .default_value("25")
            .env("ENDPOINTS_TO_REMOVE"),
    )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let endpoints_to_remove = matches
        .get_one::<String>("endpoints-to-remove")
        .unwrap()
        .parse::<u32>()?;

    let tx = kilt::tx().did().delete(endpoints_to_remove);

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
