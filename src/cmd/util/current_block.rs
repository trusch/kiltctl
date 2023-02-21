use kiltapi::connect;

pub fn command() -> clap::Command {
    clap::Command::new("current-block").about("Get the latest block number")
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let cli = connect(matches).await?;
    let block = cli.blocks().at(None).await?.number().to_string();
    println!("{block}");
    Ok(())
}
