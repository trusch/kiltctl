use std::array::TryFromSliceError;

use crate::{
    accounts::{self, AccountInfo},
    keys::{self, KeyInfo, KeyType},
    kilt::{
        runtime_types::{
            did::did_details::{
                DidAuthorizedCallOperation, DidCreationDetails, DidEncryptionKey, DidSignature,
                DidVerificationKey,
            },
            pallet_web3_names::pallet::Call::claim,
            mashnet_node_runtime::{Call, Call::{Web3Names}},
            frame_support::storage::{bounded_btree_set::BoundedBTreeSet, bounded_vec::BoundedVec},
        },
    },
    storage::Storage,
};
use codec::{Encode, Decode};
use serde::{Deserialize, Serialize};
use subxt::{
    sp_core::{
        crypto::{Ss58AddressFormat, Ss58Codec},
        Pair,
    },
    sp_runtime::{MultiSignature},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct DidInfo {
    pub id: String,
    pub auth_key: String,
    pub delegation_key: Option<String>,
    pub attestation_key: Option<String>,
    pub key_agreement_keys: Vec<String>,
}

#[derive(Debug)]
pub enum Error {
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(reason) => write!(f, "DID error: {}", reason),
        }
    }
}

impl std::error::Error for Error {}

impl From<accounts::Error> for Error {
    fn from(err: accounts::Error) -> Self {
        Error::Other(format!("{:?}", err))
    }
}

impl From<keys::Error> for Error {
    fn from(err: keys::Error) -> Self {
        Error::Other(format!("{:?}", err))
    }
}

impl From<TryFromSliceError> for Error {
    fn from(err: TryFromSliceError) -> Self {
        Error::Other(format!("{:?}", err))
    }
}

impl DidInfo {
    pub fn new(
        id: &str,
        auth_key: &str,
        delegation_key: Option<String>,
        attestation_key: Option<String>,
        key_agreement_keys: Vec<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            auth_key: auth_key.to_string(),
            delegation_key,
            attestation_key,
            key_agreement_keys,
        }
    }

    pub fn save<S: Storage>(&self, storage: &mut S, id: &str) -> Result<(), Error> {
        let key = format!("dids/{}", id);
        storage
            .save(&key, self)
            .map_err(|e| Error::Other(e.to_string()))
    }

    pub fn load<S: Storage>(storage: &S, id: &str) -> Result<DidInfo, Error> {
        let key = format!("dids/{}", id);
        storage.load(&key).map_err(|e| Error::Other(e.to_string()))
    }

    fn get_delegation_pub_key<S: Storage>(
        &self,
        storage: &S,
    ) -> Result<Option<DidVerificationKey>, Error> {
        if let Some(delegation_key) = &self.delegation_key {
            let key_info = KeyInfo::load(storage, delegation_key)?;
            match key_info.key_type {
                KeyType::Sr25519 => Ok(Some(DidVerificationKey::Sr25519(
                    crate::kilt::runtime_types::sp_core::sr25519::Public(
                        key_info.public_key[..32].try_into()?,
                    ),
                ))),
                KeyType::Ed25519 => Ok(Some(DidVerificationKey::Ed25519(
                    crate::kilt::runtime_types::sp_core::ed25519::Public(
                        key_info.public_key[..32].try_into()?,
                    ),
                ))),
                KeyType::Ecdsa => Ok(Some(DidVerificationKey::Ecdsa(
                    crate::kilt::runtime_types::sp_core::ecdsa::Public(
                        key_info.public_key[..33].try_into()?,
                    ), // !!! 33 !!!
                ))),
                _ => Err(Error::Other("".to_string())),
            }
        } else {
            Ok(None)
        }
    }

    fn get_attestation_pub_key<S: Storage>(
        &self,
        storage: &S,
    ) -> Result<Option<DidVerificationKey>, Error> {
        if let Some(attestation_key) = &self.attestation_key {
            let key_info = KeyInfo::load(storage, attestation_key)?;
            match key_info.key_type {
                KeyType::Sr25519 => Ok(Some(DidVerificationKey::Sr25519(
                    crate::kilt::runtime_types::sp_core::sr25519::Public(
                        key_info.public_key[..32].try_into()?,
                    ),
                ))),
                KeyType::Ed25519 => Ok(Some(DidVerificationKey::Ed25519(
                    crate::kilt::runtime_types::sp_core::ed25519::Public(
                        key_info.public_key[..32].try_into()?,
                    ),
                ))),
                KeyType::Ecdsa => Ok(Some(DidVerificationKey::Ecdsa(
                    crate::kilt::runtime_types::sp_core::ecdsa::Public(
                        key_info.public_key[..33].try_into()?,
                    ), // !!! 33 !!!
                ))),
                _ => Err(Error::Other("".to_string())),
            }
        } else {
            Ok(None)
        }
    }

    fn get_key_agreement_pub_keys<S: Storage>(
        &self,
        storage: &S,
    ) -> Result<BoundedBTreeSet<DidEncryptionKey>, Error> {
        let mut keys = vec![];
        for key in &self.key_agreement_keys {
            let key_info = KeyInfo::load(storage, key)?;
            match key_info.key_type {
                KeyType::X25519 => {
                    keys.push(DidEncryptionKey::X25519(
                        key_info.public_key[..32].try_into()?,
                    ));
                }
                _ => {
                    return Err(Error::Other(
                        "signing keys cant be used as key agreement keys".to_string(),
                    ))
                }
            }
        }
        Ok(BoundedBTreeSet(keys))
    }
}

pub async fn did_claim_web3name<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();

    let payment_account_name = matches.value_of("payment").unwrap();
    let payment_account_info = AccountInfo::load(storage, &payment_account_name)?;
    let payment_account_id =
        subxt::sp_runtime::AccountId32::from_ss58check(&payment_account_info.address)?;
    let payment_key_info = KeyInfo::load(storage, &payment_account_info.key_id)?;

    let did_name = matches.value_of("did").unwrap();
    let did_info = DidInfo::load(storage, did_name)?;
    let did_id = subxt::sp_runtime::AccountId32::from_ss58check(&did_info.id)?;

    let auth_key_info = KeyInfo::load(storage, &did_info.auth_key)?;

    let api = crate::kilt::connect(endpoint).await?;
    let name = BoundedVec(name.as_bytes().into());

    let did_doc = api.storage().did().did(&did_id, None).await?.unwrap();

    let did_authorized_call_operation = DidAuthorizedCallOperation {
        did: did_id.clone(),
        tx_counter: did_doc.last_tx_counter + 1,
        call: Web3Names(claim { name }),
        block_number: api.storage().system().number(None).await?,
        submitter: payment_account_id,
    };

    let signature = get_did_signature(&auth_key_info, &did_authorized_call_operation)?;

    let tx = api
        .tx()
        .did()
        .submit_did_call(did_authorized_call_operation, signature);

    let res = match payment_key_info.key_type {
        KeyType::Ed25519 => {
            let signer: subxt::sp_core::ed25519::Pair = payment_key_info.get_pair()?;
            tx.sign_and_submit_then_watch_default(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        KeyType::Sr25519 => {
            let signer: subxt::sp_core::sr25519::Pair = payment_key_info.get_pair()?;
            tx.sign_and_submit_then_watch_default(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        KeyType::Ecdsa => {
            let signer: subxt::sp_core::ecdsa::Pair = payment_key_info.get_pair()?;
            tx.sign_and_submit_then_watch_default(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        _ => panic!("Unsupported key type"),
    };

    log::info!(
        "[+] Transaction got included. Block: {:?} Extrinsic: {:?}\n",
        res.block_hash(),
        res.extrinsic_hash()
    );

    Ok(())
}

pub async fn did_resolve_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &S,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let did_id_str = {
        let did_name = matches
            .value_of("name")
            .ok_or(Error::Other("no did specified".to_string()))?;
        if !did_name.starts_with("did:kilt:") {
            DidInfo::load(storage, did_name)?.id
        } else {
            did_name
                .split(":")
                .last()
                .ok_or(Error::Other("can't parse did".to_string()))?
                .to_string()
        }
    };
    let did_id = subxt::sp_runtime::AccountId32::from_ss58check(&did_id_str)?;

    let api = crate::kilt::connect(endpoint).await?;
    let did_doc = api.storage().did().did(&did_id, None).await?.unwrap();

    println!(
        "did: did:kilt:{}",
        did_id.to_ss58check_with_version(Ss58AddressFormat::custom(38))
    );
    println!(
        "authentication key : {}",
        hex::encode(&did_doc.authentication_key)
    );
    println!(
        "attestation key    : {:?}",
        did_doc.attestation_key.map(hex::encode)
    );
    println!(
        "delegation key     : {:?}",
        did_doc.attestation_key.map(hex::encode)
    );
    println!(
        "key agreement keys : {:?}",
        did_doc
            .key_agreement_keys
            .0
            .iter()
            .map(hex::encode)
            .collect::<Vec<String>>()
    );

    Ok(())
}

pub fn did_create_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let auth_key = matches.value_of("auth").unwrap();
    let delegation_key = matches.value_of("delegation");
    let attestation_key = matches.value_of("attestation");
    let key_agreement_keys: Vec<String> = matches
        .values_of("key_agreement")
        .unwrap_or_default()
        .map(|e| e.to_string())
        .collect();

    let auth_info = KeyInfo::load(storage, &auth_key)?;

    let did_info = DidInfo {
        id: auth_info.to_ss58()?,
        auth_key: auth_key.to_string(),
        delegation_key: delegation_key.map(|e| e.to_string()),
        attestation_key: attestation_key.map(|e| e.to_string()),
        key_agreement_keys,
    };

    did_info.save(storage, name)?;

    Ok(())
}

pub async fn did_sign_and_submit<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let api = crate::kilt::connect(endpoint).await?;

    let did_name = matches.value_of("name").unwrap();
    let did_info = DidInfo::load(storage, &did_name)?;
    let did_id = subxt::sp_runtime::AccountId32::from_ss58check(&did_info.id)?;
    let auth_key_info = KeyInfo::load(storage, &did_info.auth_key)?;

    // get payment account from matches
    let payment_account_name = matches.value_of("payment").unwrap();
    let payment_account_info = AccountInfo::load(storage, &payment_account_name)?;
    let payment_key_info = KeyInfo::load(storage, &payment_account_info.key_id)?;
    let payment_account_id =
        subxt::sp_runtime::AccountId32::from_ss58check(&payment_account_info.address)?;

    let did_doc = api.storage().did().did(&did_id, None).await?.unwrap();

    let data = matches.value_of("data").unwrap();
    let data = hex::decode(data)?;

    let call = Call::decode(&mut &data[..])?;

    let did_authorized_call_operation = DidAuthorizedCallOperation {
        did: did_id.clone(),
        tx_counter: did_doc.last_tx_counter + 1,
        call,
        block_number: api.storage().system().number(None).await?,
        submitter: payment_account_id,
    };

    let signature = get_did_signature(&auth_key_info, &did_authorized_call_operation)?;

    let tx = api
        .tx()
        .did()
        .submit_did_call(did_authorized_call_operation, signature);

    let res = match payment_key_info.key_type {
        KeyType::Ed25519 => {
            let signer: subxt::sp_core::ed25519::Pair = payment_key_info.get_pair()?;
            tx.sign_and_submit_then_watch_default(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        KeyType::Sr25519 => {
            let signer: subxt::sp_core::sr25519::Pair = payment_key_info.get_pair()?;
            tx.sign_and_submit_then_watch_default(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        KeyType::Ecdsa => {
            let signer: subxt::sp_core::ecdsa::Pair = payment_key_info.get_pair()?;
            tx.sign_and_submit_then_watch_default(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        _ => panic!("Unsupported key type"),
    };

    log::info!(
        "[+] Transaction got included. Block: {:?} Extrinsic: {:?}\n",
        res.block_hash(),
        res.extrinsic_hash()
    );

    Ok(())
}

pub fn did_generate_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let mut seed = matches.value_of("seed").unwrap().to_string();
    if seed.starts_with("@") {
        let storage_key = "seeds/".to_string() + &seed[1..];
        seed = storage.get(&storage_key)?;
    }

    let auth_key_id = name.to_string() + "_auth";
    let auth_key = KeyInfo::from_seed(
        &auth_key_id,
        KeyType::Sr25519,
        &seed,
        Some("//did//0".to_string()),
    )?;
    auth_key.save(storage, &auth_key_id)?;

    let key_agreement_key_id = name.to_string() + "_key_agreement";
    let key_agreement_key = KeyInfo::from_seed(
        &key_agreement_key_id,
        KeyType::X25519,
        &seed,
        Some("//did//keyAgreement//0".to_string()),
    )?;
    key_agreement_key.save(storage, &key_agreement_key_id)?;

    let did_info = DidInfo {
        id: auth_key.to_ss58()?,
        auth_key: auth_key_id.clone(),
        delegation_key: Some(auth_key_id.clone()),
        attestation_key: Some(auth_key_id.clone()),
        key_agreement_keys: vec![key_agreement_key_id],
    };

    did_info.save(storage, name)?;

    Ok(())
}

pub fn did_show_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let did_info = DidInfo::load(storage, name)?;
    println!("{}", serde_json::to_string_pretty(&did_info)?);
    Ok(())
}

pub fn did_list_cmd<S: Storage>(storage: &S) -> Result<(), Box<dyn std::error::Error>> {
    let storage_key = "dids/".to_string();
    let data = storage.list(&storage_key)?;
    for key in data {
        let info = DidInfo::load(storage, &key)?;
        println!("{}: did:kilt:{}", key, info.id);
    }
    Ok(())
}

pub async fn did_register_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &S,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let api = crate::kilt::connect(endpoint).await?;

    // get did account from matches
    let did_name = matches.value_of("name").unwrap();
    let did_info = DidInfo::load(storage, &did_name)?;
    let did_id = subxt::sp_runtime::AccountId32::from_ss58check(&did_info.id)?;
    let auth_key_info = KeyInfo::load(storage, &did_info.auth_key)?;

    // get payment account from matches
    let payment_account_name = matches.value_of("payment").unwrap();
    let payment_account_info = AccountInfo::load(storage, &payment_account_name)?;
    let payment_key_info = KeyInfo::load(storage, &payment_account_info.key_id)?;
    let payment_account_id =
        subxt::sp_runtime::AccountId32::from_ss58check(&payment_account_info.address)?;

    let creation_details = DidCreationDetails {
        did: did_id,
        submitter: payment_account_id,
        new_key_agreement_keys: did_info.get_key_agreement_pub_keys(storage)?,
        new_attestation_key: did_info.get_attestation_pub_key(storage)?,
        new_delegation_key: did_info.get_delegation_pub_key(storage)?,
        new_service_details: vec![],
    };

    let signature = get_did_signature(&auth_key_info, &creation_details)?;

    let tx = api.tx().did().create(creation_details, signature);

    let res = match payment_key_info.key_type {
        KeyType::Ed25519 => {
            let signer: subxt::sp_core::ed25519::Pair = payment_key_info.get_pair()?;
            tx.sign_and_submit_then_watch_default(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        KeyType::Sr25519 => {
            let signer: subxt::sp_core::sr25519::Pair = payment_key_info.get_pair()?;
            tx.sign_and_submit_then_watch_default(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        KeyType::Ecdsa => {
            let signer: subxt::sp_core::ecdsa::Pair = payment_key_info.get_pair()?;
            tx.sign_and_submit_then_watch_default(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        _ => panic!("Unsupported key type"),
    };

    log::info!(
        "[+] Transaction got included. Block: {:?} Extrinsic: {:?}\n",
        res.block_hash(),
        res.extrinsic_hash()
    );

    Ok(())
}

fn get_did_signature<D: Encode>(
    key_info: &KeyInfo,
    data: D,
) -> Result<DidSignature, Box<dyn std::error::Error>> {
    match key_info.key_type {
        KeyType::Ed25519 => {
            let signer: subxt::sp_core::ed25519::Pair = key_info.get_pair()?;
            Ok(DidSignature::Ed25519(
                crate::kilt::runtime_types::sp_core::ed25519::Signature(
                    signer.sign(&data.encode()).0,
                ),
            ))
        }
        KeyType::Sr25519 => {
            let signer: subxt::sp_core::sr25519::Pair = key_info.get_pair()?;
            Ok(DidSignature::Sr25519(
                crate::kilt::runtime_types::sp_core::sr25519::Signature(
                    signer.sign(&data.encode()).0,
                ),
            ))
        }
        KeyType::Ecdsa => {
            let signer: subxt::sp_core::ecdsa::Pair = key_info.get_pair()?;
            Ok(DidSignature::Ecdsa(
                crate::kilt::runtime_types::sp_core::ecdsa::Signature(
                    signer.sign(&data.encode()).0,
                ),
            ))
        }
        _ => panic!("Unsupported key type"),
    }
}

fn get_multi_signature<D: Encode>(
    key_info: &KeyInfo,
    data: D,
) -> Result<MultiSignature, Box<dyn std::error::Error>> {
    match key_info.key_type {
        KeyType::Ed25519 => {
            let signer: subxt::sp_core::ed25519::Pair = key_info.get_pair()?;
            Ok(MultiSignature::Ed25519(subxt::sp_core::ed25519::Signature(
                signer.sign(&data.encode()).0,
            )))
        }
        KeyType::Sr25519 => {
            let signer: subxt::sp_core::sr25519::Pair = key_info.get_pair()?;
            Ok(MultiSignature::Sr25519(subxt::sp_core::sr25519::Signature(
                signer.sign(&data.encode()).0,
            )))
        }
        KeyType::Ecdsa => {
            let signer: subxt::sp_core::ecdsa::Pair = key_info.get_pair()?;
            Ok(MultiSignature::Ecdsa(subxt::sp_core::ecdsa::Signature(
                signer.sign(&data.encode()).0,
            )))
        }
        _ => panic!("Unsupported key type"),
    }
}
