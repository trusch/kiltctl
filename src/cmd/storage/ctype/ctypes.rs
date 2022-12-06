use kiltapi::{
    connect,
    kilt::{self},
    unwrap_or_stdin,
};
use sp_core::{crypto::Ss58Codec, H256};

pub fn command() -> clap::Command {
    clap::Command::new("ctypes")
    .about("Access the CTypes list")
    .arg(
        clap::Arg::new("hash")
            .long("hash")
            .help("ctype hash to lookup")
            .env("HASH"),
    )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let ctype_hash_str = unwrap_or_stdin(matches.get_one::<String>("hash").map(|e| e.to_owned()))?;
    let ctype_hash = hex::decode(ctype_hash_str.trim_start_matches("0x").trim())
        .map_err(|_| "failed to parse ctype hash")?;
    let addr = kilt::storage().ctype().ctypes(H256(
        TryInto::<[u8; 32]>::try_into(ctype_hash).map_err(|_| "failed to parse ctype hash")?,
    ));

    let cli = connect(matches).await?;
    let ctype = cli.storage().fetch(&addr, None).await?;
    if let Some(holder) = ctype {
        println!(
            "ctype owned by: did:kilt:{}",
            holder.to_ss58check_with_version(38u16.into())
        );
    } else {
        return Err("ctype not found".into());
    }
    Ok(())
}
