use substrate_api_client::{rpc::WsRpcClient, Api, Metadata};

pub fn chain_metadata_cmd(endpoint: &str, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = WsRpcClient::new(endpoint);
    let api = Api::<sp_core::sr25519::Pair, _>::new(client)?;
    if json {
        println!("{}", Metadata::pretty_format(&api.get_metadata()?).unwrap());
    } else {
        api.metadata.print_overview();
    }
    Ok(())
}

pub fn chain_runtime_version_cmd(endpoint: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = WsRpcClient::new(endpoint);
    let api = Api::<sp_core::sr25519::Pair, _>::new(client)?;
    println!("{}", api.runtime_version);
    Ok(())
}
