use clap::Arg;
use codec::Encode;
use kiltapi::{
    connect,
    kilt::{
        self,
        runtime_types::{
            self,
            did::did_details::{DidCreationDetails, DidSignature, DidVerificationKey},
            sp_runtime::bounded::bounded_btree_set::BoundedBTreeSet,
        },
    },
    AccountIdParser,
};
use sp_core::{crypto::AccountId32, sr25519, Pair};
use subxt::tx::TxPayload;

pub fn command() -> clap::Command {
    clap::Command::new("create")
        .about("Create a DID")
        .arg(
            Arg::new("submitter")
                .long("submitter")
                .required(true)
                .help("Submitter account of this extrinsic")
                .value_parser(AccountIdParser),
        )
        .arg(
            clap::Arg::new("seed")
                .short('s')
                .long("seed")
                .help("Seed to use for auth signing")
                .required(true)
                .env("SEED"),
        )
        .arg(
            clap::Arg::new("attestation-key")
                .long("attestation-key")
                .required(false)
                .help("Attestation public key"),
        )
        .arg(
            clap::Arg::new("delegation-key")
                .long("delegation-key")
                .required(false)
                .help("Delegation public key"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let submitter = matches
        .get_one::<AccountId32>("submitter")
        .expect("need submitter")
        .to_owned();
    let seed: &String = matches.get_one("seed").expect("need seed");
    let pair = sr25519::Pair::from_string_with_seed(seed, None)
        .map_err(|_| "bad seed")?
        .0;
    let pub_key = pair.public();
    let did: AccountId32 = pub_key.into();

    let attestation_key = matches.get_one::<String>("attestation-key").map(|key| {
        let bs: [u8; 32] = hex::decode(key.trim_start_matches("0x").trim())
            .expect("failed to hex decode")
            .try_into()
            .expect("bad attestation key");
        DidVerificationKey::Sr25519(runtime_types::sp_core::sr25519::Public(bs))
    });

    let delegation_key = matches.get_one::<String>("delegation-key").map(|key| {
        let bs: [u8; 32] = hex::decode(key.trim_start_matches("0x").trim())
            .expect("failed to hex decode")
            .try_into()
            .expect("bad attestation key");
        DidVerificationKey::Sr25519(runtime_types::sp_core::sr25519::Public(bs))
    });

    let details = DidCreationDetails {
        did,
        submitter,
        new_attestation_key: attestation_key,
        new_delegation_key: delegation_key,
        new_key_agreement_keys: BoundedBTreeSet(vec![]),
        new_service_details: vec![],
    };

    let sig = pair.sign(&details.encode());
    let did_sig = DidSignature::Sr25519(runtime_types::sp_core::sr25519::Signature(sig.0));
    let tx = kilt::tx().did().create(details, did_sig);

    let cli = connect(matches).await?;
    let payload = tx.encode_call_data(&cli.metadata())?;

    println!("0x{}", hex::encode(payload));
    Ok(())
}
