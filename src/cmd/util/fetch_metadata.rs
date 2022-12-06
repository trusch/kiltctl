use codec::{Decode, Encode};
use kiltapi::connect;
use sp_core::Bytes;
use subxt::{ext::frame_metadata::RuntimeMetadataPrefixed, rpc::RpcParams};

pub fn command() -> clap::Command {
    clap::Command::new("fetch-metadata")
        .about("Fetch metadata from a node")
        .arg(
            clap::Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file")
                .default_value("metadata.scale"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let output = matches.get_one::<String>("output").unwrap();
    let cli = connect(matches).await?;
    let bytes: Bytes = cli
        .rpc()
        .request("state_getMetadata", RpcParams::new())
        .await?;
    let meta: RuntimeMetadataPrefixed = Decode::decode(&mut &bytes[..])?; // sanity check
    log::info!(
        "Fetched metadata from node {}",
        matches.get_one::<String>("endpoint").unwrap()
    );
    std::fs::write(output, meta.encode())?;
    Ok(())
}
