use codec::Encode;
use kiltapi::{connect, unwrap_or_stdin, kilt::{runtime_types::{public_credentials::credentials::Credential, sp_runtime::bounded::bounded_vec::BoundedVec, runtime_common::authorization::{AuthorizationId, PalletAuthorize}, delegation::access_control::DelegationAc, peregrine_runtime::Runtime}, self}, AccountIdParser};
use sp_core::{H256, crypto::AccountId32};
use subxt::tx::TxPayload;
use blake2::{digest::consts::U32, Blake2b, Digest};
type Blake2b256 = Blake2b<U32>;

use crate::ctype::CType;

pub fn command() -> clap::Command {
    clap::Command::new("get-credential-id")
        .about("Compute id for a public credential")
        .arg(
            clap::Arg::new("ctype")
                .short('c')
                .long("ctype")
                .help("CType hash")
                .env("CTYPE"),
        )
        .arg(
            clap::Arg::new("subject")
                .short('s')
                .long("subject")
                .help("DID subject")
                .env("SUBJECT"),
        )
        .arg(
            clap::Arg::new("claims")
                .long("claims")
                .help("Claims")
                .env("CLAIMS"),
        )
        .arg(
            clap::Arg::new("attester")
                .long("attester")
                .help("Attester")
                .value_parser(AccountIdParser)
                .env("ATTESTER"),
        )
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let ctype_hash_str = matches.get_one::<String>("ctype").unwrap().to_owned();
    let ctype_hash_bytes = hex::decode(ctype_hash_str.trim_start_matches("0x").trim())?;
    let ctype_hash = H256::from_slice(&ctype_hash_bytes);
    let did = matches.get_one::<String>("subject").unwrap().to_owned();
    let claims = matches.get_one::<String>("claims").unwrap().to_owned();
    let attester = matches.get_one::<AccountId32>("attester").unwrap().to_owned();

    let cred = Credential{
        ctype_hash,
        subject: BoundedVec(did.into_bytes()),
        claims: BoundedVec(claims.into()),
        authorization: None::<PalletAuthorize<DelegationAc<Runtime>>>,
    };

    let mut hasher = Blake2b256::new();
    hasher.update(&(cred, attester).encode());
    let result = hasher.finalize();

    println!("0x{}", hex::encode(result));

    Ok(())
}
