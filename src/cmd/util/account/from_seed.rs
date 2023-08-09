use kiltapi::unwrap_or_stdin;
use subxt::ext::sp_core::{
    crypto::{Ss58AddressFormat, Ss58Codec},
    ecdsa, ed25519, sr25519, Pair,
};

pub fn command() -> clap::Command {
    clap::Command::new("from-seed")
        .about("Show account address given a seed")
        .arg(
            clap::Arg::new("seed")
                .short('s')
                .long("seed")
                .help("Seed to use")
                .env("SEED"),
        )
        .arg(
            clap::Arg::new("derive")
                .short('d')
                .long("derive")
                .required(false)
                .env("DERIVE"),
        )
        .arg(
            clap::Arg::new("prefix")
                .short('p')
                .long("prefix")
                .default_value("38"),
        )
        .arg(
            clap::Arg::new("type")
                .short('t')
                .long("type")
                .default_value("sr25519")
                .value_parser(["sr25519", "ed25519", "ecdsa"])
                .env("TYPE"),
        )
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let seed = unwrap_or_stdin(matches.get_one("seed").map(|e: &String| e.to_owned()))?
        .trim()
        .to_string();
    let prefix: &String = matches.get_one("prefix").expect("need prefix");

    let seed = {
        if let Some(derive) = matches.get_one::<String>("derive") {
            seed + derive.trim()
        } else {
            seed
        }
    };

    let key_type: &String = matches.get_one("type").expect("need type");

    let address = match key_type.as_str() {
        "sr25519" => sr25519::Pair::from_string_with_seed(&seed, None)
            .map_err(|_| "bad seed")?
            .0
            .public()
            .to_ss58check_with_version(Ss58AddressFormat::custom(prefix.parse::<u16>()?)),
        "ed25519" => ed25519::Pair::from_string_with_seed(&seed, None)
            .map_err(|_| "bad seed")?
            .0
            .public()
            .to_ss58check_with_version(Ss58AddressFormat::custom(prefix.parse::<u16>()?)),
        "ecdsa" => ecdsa::Pair::from_string_with_seed(&seed, None)
            .map_err(|_| "bad seed")?
            .0
            .public()
            .to_ss58check_with_version(Ss58AddressFormat::custom(prefix.parse::<u16>()?)),
        _ => panic!("unknown key type"),
    };

    println!("{address}");

    Ok(())
}
