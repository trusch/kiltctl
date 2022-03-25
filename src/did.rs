use crate::{
    accounts::{get_signer, AccountInfo},
    kilt::{
        kilt::runtime_types::{
            pallet_web3_names::pallet::Call::claim, spiritnet_runtime::Call::Web3Names,
        },
        runtime_types::{
            did::did_details::{DidAuthorizedCallOperation, DidCreationDetails, DidSignature},
            frame_support::storage::{bounded_btree_set::BoundedBTreeSet, bounded_vec::BoundedVec},
        },
    },
    storage::Storage,
};
use codec::Encode;
use serde::{Serialize, Deserialize};
use subxt::sp_core::{crypto::Ss58Codec, Pair};

#[derive(Serialize, Deserialize, Debug)]
pub struct DidInfo {
    id: String,
    auth_key_account: String,
    delegation_key_account: Option<String>,
    attestation_key_account: Option<String>,
    key_agreement_key_accounts: Vec<String>,
}

pub async fn did_claim_web3name<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();

    let payment_account_name = matches.value_of("payment").unwrap();
    let storage_key = "accounts/".to_string() + payment_account_name;
    let data = storage.get(&storage_key)?;
    let payment_account_info: AccountInfo = serde_json::from_str(&data)?;
    let payment_account_id =
        subxt::sp_runtime::AccountId32::from_ss58check(&payment_account_info.address)?;

    let did_name = matches.value_of("did").unwrap();
    let did_info = get_did_info(did_name, storage)?;
    let did_id = subxt::sp_runtime::AccountId32::from_ss58check(&did_info.id)?;

    let auth_account_info = get_account_info(&did_info.auth_key_account, storage)?;

    let api = crate::kilt::connect(endpoint).await?;
    let name = BoundedVec(name.as_bytes().into());

    let did_doc = api
        .storage()
        .did()
        .did(did_id.clone(), None)
        .await?
        .unwrap();

    let did_authorized_call_operation = DidAuthorizedCallOperation {
        did: did_id.clone(),
        tx_counter: did_doc.last_tx_counter + 1,
        call: Web3Names(claim { name }),
        block_number: api.storage().system().number(None).await?,
        submitter: payment_account_id,
    };
    
    let signature = get_did_signature(
        storage,
        &auth_account_info,
        &did_authorized_call_operation,
    )?;

    let tx = api
        .tx()
        .did()
        .submit_did_call(did_authorized_call_operation, signature);

    let res = match payment_account_info.algorithm {
        crate::accounts::SignatureAlgorithm::Ed25519 => {
            let signer: subxt::sp_core::ed25519::Pair =
                get_signer(payment_account_name, None, storage)?;
            tx.sign_and_submit_then_watch(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        crate::accounts::SignatureAlgorithm::Sr25519 => {
            let signer: subxt::sp_core::sr25519::Pair =
                get_signer(payment_account_name, None, storage)?;
            tx.sign_and_submit_then_watch(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        crate::accounts::SignatureAlgorithm::Ecdsa => {
            let signer: subxt::sp_core::ecdsa::Pair =
                get_signer(payment_account_name, None, storage)?;
            tx.sign_and_submit_then_watch(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
    };

    log::info!(
        "[+] Transaction got included. Block: {:?} Extrinsic: {:?}\n",
        res.block_hash(),
        res.extrinsic_hash()
    );

    Ok(())
}

pub fn did_create_cmd<S:Storage>(matches: &clap::ArgMatches, storage: &mut S) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let auth_key_account = matches.value_of("auth").unwrap();
    let delegation_key_account = matches.value_of("delegation");
    let attestation_key_account = matches.value_of("attestation");
    let key_agreement_key_accounts: Vec<String> = matches.values_of("key_agreement").unwrap_or_default().map(|e| e.to_string()).collect();

    let auth_account_info = get_account_info(auth_key_account, storage)?;

    let did_info = DidInfo{
        id: auth_account_info.address.clone(),
        auth_key_account: auth_key_account.to_string(),
        delegation_key_account: delegation_key_account.map(|e| e.to_string()),
        attestation_key_account: attestation_key_account.map(|e| e.to_string()),
        key_agreement_key_accounts,
    };

    let storage_key = "dids/".to_string() + name;
    let bs = serde_json::to_string(&did_info)?;
    storage.set(&storage_key, &bs)?;

    Ok(())
}

pub fn did_show_cmd<S:Storage>(matches: &clap::ArgMatches, storage: &S) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let did_info = get_did_info(name, storage)?;
    println!("{}", serde_json::to_string_pretty(&did_info)?);
    Ok(())
}

pub fn did_list_cmd<S:Storage>(storage: &S) -> Result<(), Box<dyn std::error::Error>> {
    let storage_key = "dids/".to_string();
    let data = storage.list(&storage_key)?;
    for key in data {
        println!("{}", key);
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
    let storage_key = "dids/".to_string() + did_name;    
    let data = storage.get(&storage_key)?;
    let did_info: DidInfo = serde_json::from_str(&data)?;
    let did_id = subxt::sp_runtime::AccountId32::from_ss58check(&did_info.id)?;

    let auth_account_info = get_account_info(&did_info.auth_key_account, storage)?;

    // get payment account from matches
    let payment_account_name = matches.value_of("payment").unwrap();
    let payment_account_info = get_account_info(payment_account_name, storage)?;
    let payment_account_id =
        subxt::sp_runtime::AccountId32::from_ss58check(&payment_account_info.address)?;

    let creation_details = DidCreationDetails {
        did: did_id,
        submitter: payment_account_id,
        // @TODO register all other keys if they exist 
        new_key_agreement_keys: BoundedBTreeSet(vec![]),
        new_attestation_key: None,
        new_delegation_key: None,
        new_service_details: vec![],
    };

    let signature = get_did_signature(storage, &auth_account_info, &creation_details)?;

    let tx = api.tx().did().create(creation_details, signature);

    let res = match payment_account_info.algorithm {
        crate::accounts::SignatureAlgorithm::Ed25519 => {
            let signer: subxt::sp_core::ed25519::Pair =
                get_signer(payment_account_name, None, storage)?;
            tx.sign_and_submit_then_watch(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        crate::accounts::SignatureAlgorithm::Sr25519 => {
            let signer: subxt::sp_core::sr25519::Pair =
                get_signer(payment_account_name, None, storage)?;
            tx.sign_and_submit_then_watch(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
        crate::accounts::SignatureAlgorithm::Ecdsa => {
            let signer: subxt::sp_core::ecdsa::Pair =
                get_signer(payment_account_name, None, storage)?;
            tx.sign_and_submit_then_watch(&subxt::PairSigner::new(signer))
                .await?
                .wait_for_finalized_success()
                .await?
        }
    };

    log::info!(
        "[+] Transaction got included. Block: {:?} Extrinsic: {:?}\n",
        res.block_hash(),
        res.extrinsic_hash()
    );

    Ok(())
}

fn get_did_signature<D: Encode, S: Storage>(
    storage: &S,
    account_info: &AccountInfo,
    data: D,
) -> Result<DidSignature, Box<dyn std::error::Error>> {
    match account_info.algorithm {
        crate::accounts::SignatureAlgorithm::Ed25519 => {
            let signer: subxt::sp_core::ed25519::Pair =
                get_signer(&account_info.name, None, storage)?;
            Ok(DidSignature::Ed25519(
                crate::kilt::runtime_types::sp_core::ed25519::Signature(
                    signer.sign(&data.encode()).0,
                ),
            ))
        }
        crate::accounts::SignatureAlgorithm::Sr25519 => {
            let signer: subxt::sp_core::sr25519::Pair =
                get_signer(&account_info.name, None, storage)?;
            Ok(DidSignature::Sr25519(
                crate::kilt::runtime_types::sp_core::sr25519::Signature(
                    signer.sign(&data.encode()).0,
                ),
            ))
        }
        crate::accounts::SignatureAlgorithm::Ecdsa => {
            let signer: subxt::sp_core::ecdsa::Pair =
                get_signer(&account_info.name, None, storage)?;
            Ok(DidSignature::Ecdsa(
                crate::kilt::runtime_types::sp_core::ecdsa::Signature(
                    signer.sign(&data.encode()).0,
                ),
            ))
        }
    }
}

fn get_account_info<S: Storage>(name: &str, storage: &S) -> Result<AccountInfo, Box<dyn std::error::Error>> {
    let storage_key = "accounts/".to_string() + name;
    let data = storage.get(&storage_key)?;
    Ok(serde_json::from_str(&data)?)
}

fn get_did_info<S: Storage>(name: &str, storage: &S) -> Result<DidInfo, Box<dyn std::error::Error>> {
    let storage_key = "dids/".to_string() + name;
    let data = storage.get(&storage_key)?;
    Ok(serde_json::from_str(&data)?)
}