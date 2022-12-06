use kiltapi::unwrap_or_stdin;
use sp_core::{ecdsa, ed25519, sr25519, Pair};

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
            clap::Arg::new("type")
                .short('t')
                .long("type")
                .default_value("sr25519")
                .value_parser(["sr25519", "ed25519", "ecdsa", "x25519"])
                .env("TYPE"),
        )
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let seed = unwrap_or_stdin(matches.get_one("seed").map(|e: &String| e.to_owned()))?
        .trim()
        .to_string();
    let seed = {
        if let Some(derive) = matches.get_one::<String>("derive") {
            seed + derive.trim()
        } else {
            seed
        }
    };
    let key_type: &String = matches.get_one("type").expect("need type");

    let key_bytes: Vec<u8> = match key_type.as_str() {
        "sr25519" => sr25519::Pair::from_string_with_seed(&seed, None)
            .map_err(|_| "bad seed")?
            .0
            .public()
            .0
            .try_into()?,
        "ed25519" => ed25519::Pair::from_string_with_seed(&seed, None)
            .map_err(|_| "bad seed")?
            .0
            .public()
            .0
            .try_into()?,
        "ecdsa" => ecdsa::Pair::from_string_with_seed(&seed, None)
            .map_err(|_| "bad seed")?
            .0
            .public()
            .0
            .try_into()?,
        _ => panic!("unknown key type"),
    };

    println!("0x{}", hex::encode(key_bytes));

    Ok(())
}
