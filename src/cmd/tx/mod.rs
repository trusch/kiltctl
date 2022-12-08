use clap::ArgMatches;

mod attestation;
mod balances;
mod ctype;
mod did;
mod linking;
mod proxy;
mod public_credentials;
mod sign;
mod submit;
mod util;

pub fn command() -> clap::Command {
    clap::Command::new("tx")
        .about("Transaction constructors")
        .subcommand_required(true)
        .subcommands([
            balances::command(),
            sign::command(),
            submit::command(),
            did::command(),
            ctype::command(),
            util::command(),
            attestation::command(),
            proxy::command(),
            linking::command(),
            public_credentials::command(),
        ])
}

pub async fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("balances", matches)) => balances::run(matches).await,
        Some(("did", matches)) => did::run(matches).await,
        Some(("sign", matches)) => sign::run(matches).await,
        Some(("submit", matches)) => submit::run(matches).await,
        Some(("ctype", matches)) => ctype::run(matches).await,
        Some(("util", matches)) => util::run(matches).await,
        Some(("attestation", matches)) => attestation::run(matches).await,
        Some(("proxy", matches)) => proxy::run(matches).await,
        Some(("linking", matches)) => linking::run(matches).await,
        Some(("public-credentials", matches)) => public_credentials::run(matches).await,
        _ => Err("no valid subcommand".into()),
    }
}
