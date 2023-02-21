use codec::{Decode, Encode};
use kiltapi::{
    connect,
    kilt::{
        self,
        runtime_types::did::did_details::{DidAuthorizedCallOperation, DidSignature},
        RuntimeCall,
    },
    AccountIdParser,
};
use sp_core::{ecdsa, ed25519, sr25519, Pair};
use subxt::tx::TxPayload;
use subxt::utils::AccountId32;

pub fn command() -> clap::Command {
    clap::Command::new("authorize")
        .about("Authorize a DID transaction")
        .arg(
            clap::Arg::new("did")
                .short('d')
                .long("did")
                .help("DID to authorize")
                .required(true)
                .value_parser(AccountIdParser)
                .env("DID"),
        )
        .arg(
            clap::Arg::new("seed")
                .long("seed")
                .help("Seed to use for signing")
                .required(true)
                .env("SEED"),
        )
        .arg(
            clap::Arg::new("key-type")
                .long("key-type")
                .help("Type of signing key")
                .env("TYPE")
                .value_parser(["sr25519", "ed25519", "ecdsa"])
                .default_value("sr25519"),
        )
        .arg(
            clap::Arg::new("submitter")
                .long("submitter")
                .help("Submitter to authorize")
                .required(true)
                .value_parser(AccountIdParser)
                .env("SUBMITTER"),
        )
        .arg(
            clap::Arg::new("tx")
                .short('t')
                .long("tx")
                .help("Transaction to authorize")
                .env("TX"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let did = matches
        .get_one::<AccountId32>("did")
        .expect("need did")
        .to_owned();
    let submitter = matches
        .get_one::<AccountId32>("submitter")
        .expect("need submitter")
        .to_owned();

    let seed: &String = matches.get_one("seed").expect("need seed");
    let key_type: &String = matches.get_one("key-type").expect("need key type");

    let tx_hex = kiltapi::unwrap_or_stdin(matches.get_one::<String>("tx").map(|t| t.to_owned()))?;
    let tx_bytes = hex::decode(tx_hex.trim_start_matches("0x").trim())?;

    let cli = connect(matches).await?;

    let did_doc_addr = kilt::storage().did().did(&did);
    let tx_counter = cli
        .storage()
        .at(None)
        .await?
        .fetch(&did_doc_addr)
        .await?
        .map(|doc| doc.last_tx_counter + 1)
        .unwrap_or(1u64);

    let block_number = cli
        .rpc()
        .block(None)
        .await
        .map_err(|e| format!("Failed to get block number: {e}"))?
        .ok_or("Failed to get block number")?
        .block
        .header
        .number;

    let op = DidAuthorizedCallOperation {
        did,
        tx_counter,
        call: RuntimeCall::decode(&mut tx_bytes.as_ref())?,
        block_number,
        submitter,
    };

    let sig = match key_type.as_str() {
        "sr25519" => {
            DidSignature::Sr25519(kiltapi::kilt::runtime_types::sp_core::sr25519::Signature(
                sr25519::Pair::from_string_with_seed(seed, None)
                    .map_err(|_| "bad seed")?
                    .0
                    .sign(op.encode().as_ref())
                    .0,
            ))
        }
        "ed25519" => {
            DidSignature::Ed25519(kiltapi::kilt::runtime_types::sp_core::ed25519::Signature(
                ed25519::Pair::from_string_with_seed(seed, None)
                    .map_err(|_| "bad seed")?
                    .0
                    .sign(op.encode().as_ref())
                    .0,
            ))
        }
        "ecdsa" => DidSignature::Ecdsa(kiltapi::kilt::runtime_types::sp_core::ecdsa::Signature(
            ecdsa::Pair::from_string_with_seed(seed, None)
                .map_err(|_| "bad seed")?
                .0
                .sign(op.encode().as_ref())
                .0,
        )),
        _ => panic!("unknown key type"),
    };

    let tx = kiltapi::kilt::tx().did().submit_did_call(op, sig);

    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));

    Ok(())
}
