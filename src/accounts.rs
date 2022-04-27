use serde::{Deserialize, Serialize};
use std::io::Read;
use subxt::{
    sp_core::{
        crypto::{Pair, Ss58AddressFormat, Ss58Codec},
        Decode, Encode,
        H160, H256,
    },
    sp_runtime::{app_crypto::RuntimePublic, AccountId32, MultiAddress},
};
use sha3::{Digest, Keccak256};

use crate::{
    keys::{KeyInfo, KeyType},
    storage::Storage,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    Ed25519,
    Sr25519,
    Ecdsa,
}

impl From<&str> for SignatureAlgorithm {
    fn from(s: &str) -> Self {
        match s {
            "ed25519" => SignatureAlgorithm::Ed25519,
            "sr25519" => SignatureAlgorithm::Sr25519,
            "ecdsa" => SignatureAlgorithm::Ecdsa,
            _ => panic!("Unsupported signature algorithm"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub name: String,
    pub key_id: String,
    pub address: String,
}

impl AccountInfo {
    pub fn new(name: &str, address: &str, key_id: &str) -> Self {
        AccountInfo {
            name: name.to_string(),
            key_id: key_id.to_string(),
            address: address.to_string(),
        }
    }
}

impl AccountInfo {
    pub fn from_key(key: &KeyInfo) -> AccountInfo {
        let kilt = Ss58AddressFormat::try_from("kilt").unwrap();

        let address = match key.key_type {
            KeyType::Sr25519 => {
                let mut p = [0u8; 32];
                p.copy_from_slice(&key.public_key[..32]);
                let public = subxt::sp_core::sr25519::Public(p);
                public.to_ss58check_with_version(kilt)
            }
            KeyType::Ed25519 => {
                let mut p = [0u8; 32];
                p.copy_from_slice(&key.public_key[..32]);
                let public = subxt::sp_core::ed25519::Public(p);
                public.to_ss58check_with_version(kilt)
            }
            KeyType::Ecdsa => {
                let mut p = [0u8; 33]; // !!! 33 bytes !!!
                p.copy_from_slice(&key.public_key[..33]);
                let public = subxt::sp_core::ecdsa::Public(p);
                public.to_ss58check_with_version(kilt)
            }
            KeyType::Ethereum => {
                let decompressed = libsecp256k1::PublicKey::parse_slice(
                    &key.public_key,
                    Some(libsecp256k1::PublicKeyFormat::Compressed),
                )
                .expect("Wrong compressed public key provided")
                .serialize();
                let mut m = [0u8; 64];
                m.copy_from_slice(&decompressed[1..65]);
                let account = H160::from(H256::from_slice(Keccak256::digest(&m).as_slice()));
                hex::encode(account.as_bytes())
            }
            _ => panic!("Unsupported key type"),
        };
        AccountInfo::new(&address, &address, &key.id)
    }

    pub fn save<S: Storage>(&self, storage: &mut S) -> Result<(), Error> {
        let key = format!("accounts/{}", &self.name);
        storage
            .save(&key, self)
            .map_err(|e| Error::Other(e.to_string()))
    }

    pub fn load<S: Storage>(storage: &S, id: &str) -> Result<AccountInfo, Error> {
        let key = format!("accounts/{}", id);
        storage.load(&key).map_err(|e| Error::Other(e.to_string()))
    }
}

pub fn account_create_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let key_id = matches.value_of("key").unwrap();
    let name = matches.value_of("name").unwrap();

    let key_info = KeyInfo::load(storage, key_id)?;
    let mut account_info = AccountInfo::from_key(&key_info);
    account_info.name = name.to_string();

    account_info.save(storage)?;

    println!("{}", account_info.address);
    Ok(())
}

pub fn account_list_cmd<S: Storage>(storage: &S) -> Result<(), Box<dyn std::error::Error>> {
    let mut accounts = storage.list("accounts/")?;
    accounts.sort();
    for account in accounts {
        let storage_key = "accounts/".to_string() + &account;
        let data = storage.get(&storage_key)?;
        let account_info: AccountInfo = serde_json::from_str(&data)?;
        println!("{}: {}", account_info.name, account_info.address);
    }
    Ok(())
}

pub fn account_show_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let storage_key = "accounts/".to_string() + name;
    let data = storage.get(&storage_key)?;
    let account_info: AccountInfo = serde_json::from_str(&data)?;
    let output = serde_json::to_string_pretty(&account_info)?;
    println!("{}", output);
    Ok(())
}

pub fn account_sign_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let account_info = AccountInfo::load(storage, name)?;
    let key_info = KeyInfo::load(storage, &account_info.key_id)?;

    let mut message = String::new();
    std::io::stdin().read_to_string(&mut message)?;

    let signature = match key_info.key_type {
        KeyType::Ed25519 => {
            let pair: subxt::sp_core::ed25519::Pair = key_info.get_pair()?;
            pair.sign(message.as_bytes()).encode()
        }
        KeyType::Sr25519 => {
            let pair: subxt::sp_core::sr25519::Pair = key_info.get_pair()?;
            pair.sign(message.as_bytes()).encode()
        }
        KeyType::Ecdsa => {
            let pair: subxt::sp_core::ecdsa::Pair = key_info.get_pair()?;
            pair.sign(message.as_bytes()).encode()
        }
        _ => panic!("Unsupported key type"),
    };
    println!("{}", hex::encode(signature));
    return Ok(());
}

pub fn account_verify_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let account_info = AccountInfo::load(storage, name)?;
    let key_info = KeyInfo::load(storage, &account_info.key_id)?;

    // parse message
    let mut message = String::new();
    std::io::stdin().read_to_string(&mut message)?;

    // parse signature
    let bytes = hex::decode(matches.value_of("signature").unwrap())?;

    let public = match key_info.key_type {
        KeyType::Ed25519 => {
            match <subxt::sp_core::ed25519::Public>::from_ss58check_with_version(
                &account_info.address,
            ) {
                Ok(p) => p.0.verify(
                    &message,
                    &<subxt::sp_core::ed25519::Signature>::decode(&mut &bytes[..])?,
                ),
                Err(_) => return Err("Invalid address".into()),
            }
        }
        KeyType::Sr25519 => {
            match <subxt::sp_core::sr25519::Public>::from_ss58check_with_version(
                &account_info.address,
            ) {
                Ok(p) => p.0.verify(
                    &message,
                    &<subxt::sp_core::sr25519::Signature>::decode(&mut &bytes[..])?,
                ),
                Err(_) => return Err("Invalid address".into()),
            }
        }
        KeyType::Ecdsa => {
            match <subxt::sp_core::ecdsa::Public>::from_ss58check_with_version(
                &account_info.address,
            ) {
                Ok(p) => p.0.verify(
                    &message,
                    &<subxt::sp_core::ecdsa::Signature>::decode(&mut &bytes[..])?,
                ),
                Err(_) => return Err("Invalid address".into()),
            }
        }
        _ => panic!("Unsupported key type"),
    };

    if !public {
        return Err("Signature is invalid".into());
    }

    Ok(())
}

pub async fn account_info_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &S,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let storage_key = "accounts/".to_string() + name;
    let data = storage.get(&storage_key)?;
    let account_info: AccountInfo = serde_json::from_str(&data)?;
    let api = crate::kilt::connect(endpoint).await?;
    let result = api
        .storage()
        .system()
        .account(
            &subxt::sp_runtime::AccountId32::from_ss58check(&account_info.address)?,
            None,
        )
        .await?;

    println!("{:?}", result);

    let free = result.data.free as f64 / 1_000_000_000_000_000_f64;
    let reserved = result.data.reserved as f64 / 1_000_000_000_000_000_f64;
    let total = free + reserved;

    println!("address: {}", account_info.address);
    println!("total: {:.4} KILT", total);
    println!("free: {:.4} KILT", free);
    println!("reserved: {:.4} KILT", reserved);
    println!("nonce: {}", result.nonce);

    Ok(())
}

pub async fn account_send_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &S,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let from = matches.value_of("from").unwrap();
    let to = matches.value_of("to").unwrap();
    let amount: f64 = matches.value_of("amount").unwrap().parse()?;

    println!("from: {}", from);
    println!("to: {}", to);
    println!("amount: {:.4} KILT", amount);

    let from_info = AccountInfo::load(storage, from)?;
    let to_info = AccountInfo::load(storage, to)?;
    let from_key_info = KeyInfo::load(storage, &from_info.key_id)?;
    let receiver = <subxt::sp_runtime::AccountId32>::from_ss58check(&to_info.address)?;

    let api = crate::kilt::connect(endpoint).await?;
    let tx = api.tx().balances().transfer(
        MultiAddress::Id(receiver),
        (amount * 1_000_000_000_000_000_f64) as u128,
    );
    let res = match from_key_info.key_type {
        KeyType::Sr25519 => {
            let signer: subxt::sp_core::sr25519::Pair = from_key_info.get_pair()?;
            let s = subxt::PairSigner::new(signer);
            tx.sign_and_submit_then_watch_default(&s)
                .await?
                .wait_for_in_block()
                .await?
        }
        KeyType::Ed25519 => {
            let signer: subxt::sp_core::ed25519::Pair = from_key_info.get_pair()?;
            let s = subxt::PairSigner::new(signer);
            tx.sign_and_submit_then_watch_default(&s)
                .await?
                .wait_for_in_block()
                .await?
        }
        KeyType::Ecdsa => {
            let signer: subxt::sp_core::ecdsa::Pair = from_key_info.get_pair()?;
            let s = subxt::PairSigner::new(signer);
            tx.sign_and_submit_then_watch_default(&s)
                .await?
                .wait_for_in_block()
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

pub async fn account_send_all_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &S,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let from = matches.value_of("from").unwrap();
    let to = matches.value_of("to").unwrap();
    let keep_alive = matches.is_present("keep-alive");

    println!("from: {}", from);
    println!("to: {}", to);

    let from_info = AccountInfo::load(storage, from)?;
    let to_info = AccountInfo::load(storage, to)?;
    let from_key_info = KeyInfo::load(storage, &from_info.key_id)?;
    let receiver = <subxt::sp_runtime::AccountId32>::from_ss58check(&to_info.address)?;

    let api = crate::kilt::connect(endpoint).await?;
    let tx = api
        .tx()
        .balances()
        .transfer_all(MultiAddress::Id(receiver), keep_alive);
    let res = match from_key_info.key_type {
        KeyType::Sr25519 => {
            let signer: subxt::sp_core::sr25519::Pair = from_key_info.get_pair()?;
            let s = subxt::PairSigner::new(signer);
            tx.sign_and_submit_then_watch_default(&s)
                .await?
                .wait_for_in_block()
                .await?
        }
        KeyType::Ed25519 => {
            let signer: subxt::sp_core::ed25519::Pair = from_key_info.get_pair()?;
            let s = subxt::PairSigner::new(signer);
            tx.sign_and_submit_then_watch_default(&s)
                .await?
                .wait_for_in_block()
                .await?
        }
        KeyType::Ecdsa => {
            let signer: subxt::sp_core::ecdsa::Pair = from_key_info.get_pair()?;
            let s = subxt::PairSigner::new(signer);
            tx.sign_and_submit_then_watch_default(&s)
                .await?
                .wait_for_in_block()
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

pub fn account_remove_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let storage_key = "accounts/".to_string() + name;
    storage.remove(&storage_key)?;
    Ok(())
}

fn parse_passphrase<P: subxt::sp_core::Pair>(
    passphrase: &str,
    password: Option<&str>,
) -> Result<(String, P), String> {
    let kilt = Ss58AddressFormat::try_from("kilt").unwrap();
    if let Ok((pair, _)) = <P>::from_string_with_seed(passphrase, password) {
        Ok((pair.public().to_ss58check_with_version(kilt), pair))
    } else {
        Err("Invalid passphrase".to_string())
    }
}

struct AccountGenerator {
    seed: String,
    alg: SignatureAlgorithm,
    idx: usize,
}

impl AccountGenerator {
    pub fn new(seed: &str, alg: SignatureAlgorithm) -> Self {
        Self {
            seed: seed.to_string(),
            alg,
            idx: 0,
        }
    }
}

impl Iterator for AccountGenerator {
    type Item = (String, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        if self.idx % 1000 == 0 {
            println!("checked {} addresses...", self.idx);
        }
        match self.alg {
            SignatureAlgorithm::Ed25519 => Some((
                parse_passphrase::<subxt::sp_core::ed25519::Pair>(
                    &(self.seed.to_string() + "/" + &self.idx.to_string()),
                    None,
                )
                .unwrap()
                .0,
                self.idx,
            )),
            SignatureAlgorithm::Sr25519 => Some((
                parse_passphrase::<subxt::sp_core::sr25519::Pair>(
                    &(self.seed.to_string() + "/" + &self.idx.to_string()),
                    None,
                )
                .unwrap()
                .0,
                self.idx,
            )),
            SignatureAlgorithm::Ecdsa => Some((
                parse_passphrase::<subxt::sp_core::ecdsa::Pair>(
                    &(self.seed.to_string() + "/" + &self.idx.to_string()),
                    None,
                )
                .unwrap()
                .0,
                self.idx,
            )),
        }
    }
}
