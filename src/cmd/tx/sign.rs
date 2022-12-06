use kiltapi::{connect, unwrap_or_stdin, RawCall};
use sp_core::{sr25519, Pair};
use subxt::tx::{Era, PolkadotExtrinsicParamsBuilder};

pub fn command() -> clap::Command {
    clap::Command::new("sign")
        .about("Sign a transaction")
        .arg(
            clap::Arg::new("seed")
                .short('s')
                .long("seed")
                .help("Seed to use for signing")
                .required(true)
                .env("SEED"),
        )
        .arg(
            clap::Arg::new("tx")
                .short('t')
                .long("tx")
                .help("Transaction to sign"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let seed: &String = matches.get_one("seed").expect("need seed");
    let tx = unwrap_or_stdin(matches.get_one("tx").map(|s: &String| s.to_owned()))?;

    let call = RawCall {
        call: hex::decode(tx.trim_start_matches("0x").trim())?,
    };
    let pair = sr25519::Pair::from_string_with_seed(seed, None)
        .map_err(|_| "bad seed")?
        .0;
    let signer = subxt::tx::PairSigner::new(pair);

    let cli = connect(matches).await?;

    let params = PolkadotExtrinsicParamsBuilder::new()
        // .tip(PlainTip::new(20_000_000_000_000))
        .era(Era::Immortal, cli.genesis_hash());

    let signed = cli.tx().create_signed(&call, &signer, params).await?;

    println!("0x{}", hex::encode(signed.encoded()));
    Ok(())
}
