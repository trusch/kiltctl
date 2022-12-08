pub fn command() -> clap::Command {
    clap::Command::new("generate")
        .about("Generate a valid asset-did")
        .arg(
            clap::Arg::new("chain-namespace")
                .long("chain-namespace")
                .help("Chain namespace")
                .value_parser(["eip155", "bip122", "cosmos", "polkadot", "kusama"]),
        )
        .arg(clap::Arg::new("chain-reference").long("chain-reference"))
        .arg(
            clap::Arg::new("asset-namespace")
                .long("asset-namespace")
                .help("Asset namespace")
                .value_parser(["erc20", "erc721", "erc1155", "slip44"])
                .required(true),
        )
        .arg(
            clap::Arg::new("asset-reference")
                .long("asset-reference")
                .required(true),
        )
        .arg(clap::Arg::new("asset-id").long("asset-id"))
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let chain_namespace = matches
        .get_one::<String>("chain-namespace")
        .expect("chain-namespace is required");
    let chain_reference = matches
        .get_one::<String>("chain-reference")
        .expect("chain-reference is required");
    let asset_namespace = matches
        .get_one::<String>("asset-namespace")
        .expect("asset-namespace is required");
    let asset_reference = matches
        .get_one::<String>("asset-reference")
        .expect("asset-reference is required");
    let asset_id = matches.get_one::<String>("asset-id");

    let mut did = format!(
        "did:asset:{}:{}.{}:{}",
        chain_namespace, chain_reference, asset_namespace, asset_reference
    );
    if let Some(asset_id) = asset_id {
        did = format!("{}:{}", did, asset_id);
    }

    println!("{}", did);

    Ok(())
}
