use codec::Decode;
use kilt_asset_dids::AssetDid;
use kiltapi::{
    connect, format_balance,
    kilt::{
        self,
        runtime_types::{
            public_credentials::credentials::CredentialEntry,
            runtime_common::authorization::AuthorizationId,
        },
    },
    unwrap_or_stdin,
};
use sp_core::{
    crypto::{AccountId32, Ss58Codec},
    H256,
};
use subxt::storage::address::{StorageHasher, StorageMapKey};

type Entry = CredentialEntry<H256, AccountId32, u64, AccountId32, u128, AuthorizationId<H256>>;

pub fn command() -> clap::Command {
    clap::Command::new("credentials")
        .about("Access the credentials for a asset-did list")
        .arg(
            clap::Arg::new("asset-did")
                .long("asset-did")
                .help("asset-did to lookup")
                .env("ASSET_DID"),
        )
        .arg(
            clap::Arg::new("credential-id")
                .long("credential-id")
                .help("credential-id to lookup")
                .env("CREDENTIAL_ID"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let asset_did_str =
        unwrap_or_stdin(matches.get_one::<String>("asset-did").map(|e| e.to_owned()))?;

    let did =
        AssetDid::from_utf8_encoded(asset_did_str).map_err(|_| "failed to parse asset did")?;

    let credential_id = matches
        .get_one::<String>("credential-id")
        .map(|e| e.to_owned());

    let addr = kilt::storage().public_credentials().credentials_root();
    let mut query_key = addr.to_root_bytes();
    StorageMapKey::new(did, StorageHasher::Twox64Concat).to_bytes(&mut query_key);

    let cli = connect(matches).await?;

    if let Some(id) = credential_id {
        let id = H256(
            hex::decode(id.trim_start_matches("0x").trim())
                .map_err(|_| "failed to parse credential id")?
                .try_into()
                .map_err(|_| "failed to parse credential id")?,
        );
        StorageMapKey::new(id, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
        let attestation_data = cli
            .storage()
            .at(None)
            .await?
            .fetch_raw(&query_key)
            .await?
            .ok_or("no attestation found")?;
        let attestation = Entry::decode(&mut &attestation_data[..])?;
        print_credential_entry(attestation);
    } else {
        let keys = cli
            .storage()
            .at(None)
            .await?
            .fetch_keys(&query_key, 10, None)
            .await?;
        for key in keys {
            println!(
                "Credential ID: 0x{}",
                hex::encode(&key.0[key.0.len() - 32..])
            );
            if let Some(storage_data) = cli.storage().at(None).await?.fetch_raw(&key.0).await? {
                let value = Entry::decode(&mut &storage_data[..])?;
                print_credential_entry(value);
            }
        }
    }
    Ok(())
}

fn print_credential_entry(
    entry: CredentialEntry<H256, AccountId32, u64, AccountId32, u128, AuthorizationId<H256>>,
) {
    println!("CType Hash: {:?}", entry.ctype_hash);
    println!(
        "Attester: {}",
        entry.attester.to_ss58check_with_version(38u16.into())
    );
    println!("Revoked: {}", entry.revoked);
    println!("Deposit: {}", format_balance(entry.deposit.amount));
    println!(
        "Deposit Owner: {}",
        entry.deposit.owner.to_ss58check_with_version(38u16.into())
    );
}
