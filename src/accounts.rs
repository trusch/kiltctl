use serde::{Deserialize, Serialize};
use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::{Decode, Encode};
use sp_runtime::app_crypto::RuntimePublic;
use sp_runtime::{AccountId32, MultiAddress};
use std::io::Read;
use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::{AccountData, AccountInfoGen, Api, XtStatus};

use crate::storage::Storage;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    name: String,
    algorithm: SignatureAlgorithm,
    seed_id: Option<String>,
    derive_path: Option<String>,
    address: String,
}

impl AccountInfo {
    pub fn new(
        name: &str,
        seed_id: Option<String>,
        derive_path: Option<String>,
        address: &str,
        algorithm: SignatureAlgorithm,
    ) -> Self {
        AccountInfo {
            name: name.to_string(),
            algorithm,
            seed_id,
            derive_path,
            address: address.to_string(),
        }
    }
}

pub fn account_generate_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(suffix) = matches.value_of("suffix") {
        account_generate_with_suffix_cmd(matches, storage, suffix)?;
        return Ok(());
    }
    let mut seed = matches.value_of("seed").unwrap().to_string();
    let mut seed_id = None;
    if seed.starts_with('@') {
        seed_id = Some(seed.split_off(1));
        let storage_key = "seeds/".to_string() + seed_id.as_ref().unwrap();
        seed = storage.get(&storage_key)?;
    }
    let password = matches.value_of("password");
    if let Some(derive_path) = matches.value_of("derive") {
        seed += derive_path;
    }
    let account = match matches.value_of("algorithm").unwrap().into() {
        SignatureAlgorithm::Ed25519 => {
            parse_passphrase::<sp_core::ed25519::Pair>(&seed, password)?.0
        }
        SignatureAlgorithm::Sr25519 => {
            parse_passphrase::<sp_core::sr25519::Pair>(&seed, password)?.0
        }
        SignatureAlgorithm::Ecdsa => parse_passphrase::<sp_core::ecdsa::Pair>(&seed, password)?.0,
    };
    if let Some(name) = matches.value_of("name") {
        let storage_key = "accounts/".to_string() + name;
        let algo = matches.value_of("algorithm").unwrap().into();
        let derive_path = matches.value_of("derive");
        let data = AccountInfo::new(
            name,
            seed_id,
            derive_path.map(|e| e.to_string()),
            &account,
            algo,
        );
        let bs = serde_json::to_string(&data)?;
        storage.set(&storage_key, &bs)?;
    }
    println!("{}", account);
    Ok(())
}

pub fn account_generate_with_suffix_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
    suffix: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut seed = matches.value_of("seed").unwrap().to_string();
    let mut seed_id = None;
    if seed.starts_with('@') {
        seed_id = Some(seed.split_off(1));
        let storage_key = "seeds/".to_string() + seed_id.as_ref().unwrap();
        seed = storage.get(&storage_key)?;
    }

    let alg: SignatureAlgorithm = matches.value_of("algorithm").unwrap().into();
    let generator = AccountGenerator::new(&seed, alg);
    let (account, idx) = generator.filter(|e|e.0.ends_with(suffix)).take(1).collect::<Vec<_>>()[0].clone();
    
    if let Some(name) = matches.value_of("name") {
        let storage_key = "accounts/".to_string() + name;
        let algo = matches.value_of("algorithm").unwrap().into();
        let derive_path = matches.value_of("derive");
        let data = AccountInfo::new(
            name,
            seed_id,
            derive_path.map(|e| e.to_string()),
            &account,
            algo,
        );
        let bs = serde_json::to_string(&data)?;
        storage.set(&storage_key, &bs)?;
    }
    
    println!("{}", account);
    println!("//{}", idx);
    Ok(())
}

pub fn account_import_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let algo = matches.value_of("algorithm").unwrap().into();
    let address = matches.value_of("address").unwrap();
    let account = AccountInfo::new(name, None, None, address, algo);
    let storage_key = "accounts/".to_string() + name;
    let bs = serde_json::to_string(&account)?;
    storage.set(&storage_key, &bs)?;
    Ok(())
}

pub fn account_list_cmd<S: Storage>(storage: &mut S) -> Result<(), Box<dyn std::error::Error>> {
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
    storage: &mut S,
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
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let storage_key = "accounts/".to_string() + name;
    let data = storage.get(&storage_key)?;
    let account_info: AccountInfo = serde_json::from_str(&data)?;
    if let Some(seed_id) = account_info.seed_id {
        use sp_core::Pair;
        let mut message = String::new();
        std::io::stdin().read_to_string(&mut message)?;
        // let message = hex::decode(message.trim()).unwrap();
        let storage_key = "seeds/".to_string() + &seed_id;
        let seed = storage.get(&storage_key)?;
        let signature = match account_info.algorithm {
            SignatureAlgorithm::Ed25519 => {
                let pair = parse_passphrase::<sp_core::ed25519::Pair>(&seed, None)?.1;
                pair.sign(message.as_bytes()).encode()
            }
            SignatureAlgorithm::Sr25519 => {
                let pair = parse_passphrase::<sp_core::sr25519::Pair>(&seed, None)?.1;
                pair.sign(message.as_bytes()).encode()
            }
            SignatureAlgorithm::Ecdsa => {
                let pair = parse_passphrase::<sp_core::ecdsa::Pair>(&seed, None)?.1;
                pair.sign(message.as_bytes()).encode()
            }
        };
        println!("{}", hex::encode(signature));
        return Ok(());
    }
    Err("seed_id not found".into())
}

pub fn account_verify_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let storage_key = "accounts/".to_string() + name;
    let data = storage.get(&storage_key)?;
    let account_info: AccountInfo = serde_json::from_str(&data)?;

    // parse message
    let mut message = String::new();
    std::io::stdin().read_to_string(&mut message)?;

    // parse signature
    let bytes = hex::decode(matches.value_of("signature").unwrap())?;

    let public = match account_info.algorithm {
        SignatureAlgorithm::Ed25519 => {
            match <sp_core::ed25519::Public>::from_ss58check_with_version(&account_info.address) {
                Ok(p) => p.0.verify(
                    &message,
                    &<sp_core::ed25519::Signature>::decode(&mut &bytes[..])?,
                ),
                Err(_) => return Err("Invalid address".into()),
            }
        }
        SignatureAlgorithm::Sr25519 => {
            match <sp_core::sr25519::Public>::from_ss58check_with_version(&account_info.address) {
                Ok(p) => p.0.verify(
                    &message,
                    &<sp_core::sr25519::Signature>::decode(&mut &bytes[..])?,
                ),
                Err(_) => return Err("Invalid address".into()),
            }
        }
        SignatureAlgorithm::Ecdsa => {
            match <sp_core::ecdsa::Public>::from_ss58check_with_version(&account_info.address) {
                Ok(p) => p.0.verify(
                    &message,
                    &<sp_core::ecdsa::Signature>::decode(&mut &bytes[..])?,
                ),
                Err(_) => return Err("Invalid address".into()),
            }
        }
    };

    if !public {
        return Err("Signature is invalid".into());
    }

    Ok(())
}

pub fn account_info_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let storage_key = "accounts/".to_string() + name;
    let data = storage.get(&storage_key)?;
    let account_info: AccountInfo = serde_json::from_str(&data)?;

    let client = WsRpcClient::new(endpoint);
    fn get_account<Pair: sp_core::Pair, Public: sp_core::Public + sp_core::Encode>(
        client: WsRpcClient,
        addr: &str,
    ) -> Result<AccountInfoGen<u64, AccountData>, Box<dyn std::error::Error>> {
        let api = Api::<Pair, _>::new(client)?;
        let public = match <Public>::from_ss58check(addr) {
            Ok(p) => p,
            Err(_) => return Err("Invalid address".into()),
        };
        let result: AccountInfoGen<u64, AccountData> = api
            .get_storage_map("System", "Account", public, None)?
            .ok_or::<Box<dyn std::error::Error>>("failed to find account".into())?;
        Ok(result)
    }
    let result: AccountInfoGen<u64, AccountData> = match account_info.algorithm {
        SignatureAlgorithm::Ed25519 => get_account::<
            sp_core::ed25519::Pair,
            sp_core::ed25519::Public,
        >(client, &account_info.address)?,
        SignatureAlgorithm::Sr25519 => get_account::<
            sp_core::sr25519::Pair,
            sp_core::sr25519::Public,
        >(client, &account_info.address)?,
        SignatureAlgorithm::Ecdsa => get_account::<sp_core::ecdsa::Pair, sp_core::ecdsa::Public>(
            client,
            &account_info.address,
        )?,
    };

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

pub fn account_send_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let from = matches.value_of("from").unwrap();
    let to = matches.value_of("to").unwrap();
    let amount: f64 = matches.value_of("amount").unwrap().parse()?;

    println!("from: {}", from);
    println!("to: {}", to);
    println!("amount: {:.4} KILT", amount);

    let from_info = get_account_info(from, storage)?;
    let receiver = get_account_id(&to, storage)?;
    let client = WsRpcClient::new(endpoint);
    let tx_hash = match from_info.algorithm {
        SignatureAlgorithm::Sr25519 => {
            let signer: sp_core::sr25519::Pair = get_signer(&from, None, storage)?;
            let api = Api::new(client)?.set_signer(signer);
            let xt = api.balance_transfer(
                MultiAddress::Id(receiver),
                (amount * 1_000_000_000_000_000_f64) as u128,
            );
            api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)?
        }
        SignatureAlgorithm::Ed25519 => {
            let signer: sp_core::ed25519::Pair = get_signer(&from, None, storage)?;
            let api = Api::new(client)?.set_signer(signer);
            let xt = api.balance_transfer(
                MultiAddress::Id(receiver),
                (amount * 1_000_000_000_000_000_f64) as u128,
            );
            api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)?
        }
        SignatureAlgorithm::Ecdsa => {
            let signer: sp_core::ecdsa::Pair = get_signer(&from, None, storage)?;
            let api = Api::new(client)?.set_signer(signer);
            let xt = api.balance_transfer(
                MultiAddress::Id(receiver),
                (amount * 1_000_000_000_000_000_f64) as u128,
            );
            api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)?
        }
    };

    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);
    Ok(())
}

fn parse_passphrase<P: sp_core::Pair>(
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

fn get_account_info<S: Storage>(
    account_name: &str,
    storage: &mut S,
) -> Result<AccountInfo, Box<dyn std::error::Error>> {
    let storage_key = "accounts/".to_string() + account_name;
    let data = storage.get(&storage_key)?;
    let account_info: AccountInfo = serde_json::from_str(&data)?;
    Ok(account_info)
}

fn get_account_id<S: Storage>(
    account_name: &str,
    storage: &mut S,
) -> Result<AccountId32, Box<dyn std::error::Error>> {
    let account_info = get_account_info(account_name, storage)?;
    let id: AccountId32 = match <AccountId32>::from_ss58check(&account_info.address) {
        Ok(id) => id,
        Err(_) => return Err("Invalid address".into()),
    };
    Ok(id)
}

fn get_signer<P: sp_core::Pair, S: Storage>(
    account_name: &str,
    password: Option<&str>,
    storage: &S,
) -> Result<P, Box<dyn std::error::Error>> {
    let storage_key = "accounts/".to_string() + account_name;
    let data = storage.get(&storage_key)?;
    let account_info: AccountInfo = serde_json::from_str(&data)?;
    if let Some(seed) = account_info.seed_id {
        let storage_key = "seeds/".to_string() + &seed;
        let mut data = storage.get(&storage_key)?;
        if let Some(derive) = account_info.derive_path {
            data = data + &derive;
        }
        if let Ok((pair, _)) = <P>::from_string_with_seed(&data, password) {
            Ok(pair)
        } else {
            Err("Invalid passphrase".into())
        }
    } else {
        Err("No seed".into())
    }
}

struct AccountGenerator {
    seed: String,
    alg: SignatureAlgorithm,
    idx: usize,
}

impl AccountGenerator {
    pub fn new(seed: &str, alg: SignatureAlgorithm) -> Self {
        Self { seed: seed.to_string(), alg, idx: 0 }
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
            SignatureAlgorithm::Ed25519 => {
                Some((parse_passphrase::<sp_core::ed25519::Pair>(&(self.seed.to_string()+"/"+&self.idx.to_string()), None).unwrap().0, self.idx))
            }
            SignatureAlgorithm::Sr25519 => {
                Some((parse_passphrase::<sp_core::sr25519::Pair>(&(self.seed.to_string()+"/"+&self.idx.to_string()), None).unwrap().0, self.idx))
            }
            SignatureAlgorithm::Ecdsa => {
                Some((parse_passphrase::<sp_core::ecdsa::Pair>(&(self.seed.to_string()+"/"+&self.idx.to_string()), None).unwrap().0, self.idx))
            },
        }
    }
}