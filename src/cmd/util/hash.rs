use kiltapi::unwrap_or_stdin;

use blake2::{digest::consts::U32, Blake2b, Digest};
type Blake2b256 = Blake2b<U32>;

pub fn command() -> clap::Command {
    clap::Command::new("hash")
        .about("Compute Blake2b 256 hash of some data")
        .arg(
            clap::Arg::new("data")
                .short('d')
                .long("data")
                .help("Data to hash")
                .required(false),
        )
        .arg(
            clap::Arg::new("input-format")
                .long("input-format")
                .help("Input format")
                .value_parser(["raw", "hex"])
                .default_value("hex")
                .required(false),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let data = unwrap_or_stdin(matches.get_one::<String>("data").map(|e| e.to_owned()))?;
    let input_format = matches.get_one::<String>("input-format").unwrap();

    let data = match input_format.as_str() {
        "raw" => data.as_bytes().to_vec(),
        "hex" => hex::decode(data.trim_start_matches("0x").trim())?.to_vec(),
        _ => panic!("Invalid input format"),
    };

    let mut hasher = Blake2b256::new();
    hasher.update(&data);
    let result = hasher.finalize();

    println!("0x{}", hex::encode(result));
    Ok(())
}
