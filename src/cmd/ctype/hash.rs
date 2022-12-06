use blake2::{digest::consts::U32, Blake2b, Digest};
use kiltapi::unwrap_or_stdin;

use crate::ctype::CType;

type Blake2b256 = Blake2b<U32>;

pub fn command() -> clap::Command {
    clap::Command::new("hash")
        .about("Get the hash of a CType")
        .arg(
            clap::Arg::new("ctype")
                .short('c')
                .long("ctype")
                .help("CType data")
                .env("CTYPE"),
        )
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let ctype_str = unwrap_or_stdin(matches.get_one::<String>("ctype").map(|e| e.to_owned()))?;
    let ctype: CType = serde_json::from_str(&ctype_str)?;

    let data = ctype.serialize()?;

    let mut hasher = Blake2b256::new();
    hasher.update(data);
    let hash = hasher.finalize().to_vec();

    println!("0x{}", hex::encode(hash));
    Ok(())
}
