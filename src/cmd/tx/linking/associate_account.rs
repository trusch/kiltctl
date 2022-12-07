use codec::Encode;
use kiltapi::{AccountIdParser, connect, kilt};
use sp_core::{crypto::AccountId32, ecdsa, ed25519, sr25519, Pair};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("associate-account")
        .about("Link an account to a DID")
        .arg(
            clap::Arg::new("account")
                .short('a')
                .long("account")
                .help("Account to link")
                .value_parser(AccountIdParser)
                .env("ACCOUNT"),
        )
        .arg(
            clap::Arg::new("expiration")
                .short('e')
                .long("expiration")
                .help("Block number at which the signatures expires")
                .env("EXPIRATION"),
        )
        .arg(
            clap::Arg::new("seed")
                .short('s')
                .long("seed")
                .help("Seed to use for signing")
                .env("SEED"),
        )
        .arg(
            clap::Arg::new("signature-algorithm")
                .long("signature-algorithm")
                .help("Signature algorithm to use for signing")
                .value_parser(["sr25519", "ed25519", "ecdsa"])
                .default_value("sr25519")
                .env("SIGNATURE_ALGORITHM"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let account = matches
        .get_one::<AccountId32>("account")
        .expect("need account");
    let seed = matches.get_one::<String>("seed").expect("need seed");
    let signature_algorithm = matches
        .get_one::<String>("signature-algorithm")
        .expect("need signature algorithm");
    let expiration = match matches.get_one::<u64>("expiration") {
        Some(expiration) => expiration.to_owned(),
        None => {
            let cli = connect(matches).await?;
            let block_number = cli
                .rpc()
                .block(None)
                .await
                .map_err(|e| format!("Failed to get block number: {}", e))?
                .ok_or("Failed to get block number")?
                .block
                .header
                .number;
            block_number + 100
        }
    };

    let tx = match signature_algorithm.as_str() {
        "sr25519" => {
            let pair = sr25519::Pair::from_string_with_seed(seed, None)
                .expect("failed to parse seed")
                .0;
            kilt::tx().did_lookup().associate_account(
                account.to_owned(),
                expiration,
                kiltapi::kilt::runtime_types::sp_runtime::MultiSignature::Sr25519(
                    kiltapi::kilt::runtime_types::sp_core::sr25519::Signature(
                        pair.sign(&(account, expiration).encode()).0,
                    ),
                ),
            )
        }
        "ed25519" => {
            let pair = ed25519::Pair::from_string_with_seed(seed, None)
                .expect("failed to parse seed")
                .0;
            kilt::tx().did_lookup().associate_account(
                account.to_owned(),
                expiration,
                kiltapi::kilt::runtime_types::sp_runtime::MultiSignature::Ed25519(
                    kiltapi::kilt::runtime_types::sp_core::ed25519::Signature(
                        pair.sign(&(account, expiration).encode()).0,
                    ),
                ),
            )
        }
        "ecdsa" => {
            let pair = ecdsa::Pair::from_string_with_seed(seed, None)
                .expect("failed to parse seed")
                .0;
            kilt::tx().did_lookup().associate_account(
                account.to_owned(),
                expiration,
                kiltapi::kilt::runtime_types::sp_runtime::MultiSignature::Ecdsa(
                    kiltapi::kilt::runtime_types::sp_core::ecdsa::Signature(
                        pair.sign(&(account, expiration).encode()).0,
                    ),
                ),
            )
        }
        _ => unreachable!(),
    };

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;
    println!("0x{}", hex::encode(payload));
    Ok(())
}
