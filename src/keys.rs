use std::array::TryFromSliceError;

use base58::ToBase58;
use blake2::{digest::consts::U32, Blake2b, Digest};
use serde::{Deserialize, Serialize};
use subxt::{sp_core::{crypto::{SecretStringError, Ss58AddressFormat, Ss58Codec}, ecdsa, ed25519, sr25519, ByteArray, Pair}};

use crate::storage::Storage;

type Blake2b256 = Blake2b<U32>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum KeyType {
    Sr25519,
    Ed25519,
    X25519,
    Ecdsa,
    Ethereum,
}

impl std::str::FromStr for KeyType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sr25519" => Ok(KeyType::Sr25519),
            "ed25519" => Ok(KeyType::Ed25519),
            "x25519" => Ok(KeyType::X25519),
            "ecdsa" => Ok(KeyType::Ecdsa),
            "ethereum" => Ok(KeyType::Ethereum),
            _ => Err(Error::Key("Invalid key type".to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyInfo {
    pub id: String,
    pub key_type: KeyType,
    pub public_key: Vec<u8>,
    pub seed: Option<String>,
    pub derive: Option<String>,
}

impl KeyInfo {
    pub fn from_seed(
        id: &str,
        key_type: KeyType,
        seed: &str,
        derive: Option<String>,
    ) -> Result<KeyInfo, Error> {
        let s = seed.to_string() + &(derive.clone().unwrap_or("".to_string()));

        let public_key = match key_type {
            KeyType::Sr25519 => {
                let (keypair, _) = sr25519::Pair::from_string_with_seed(&s, None)?;
                keypair.public().to_raw_vec()
            }
            KeyType::Ed25519 => {
                let (keypair, _) = ed25519::Pair::from_string_with_seed(&s, None)?;
                keypair.public().to_raw_vec()
            }
            KeyType::Ecdsa => {
                let (keypair, _) = ecdsa::Pair::from_string_with_seed(&s, None)?;
                keypair.public().to_raw_vec()
            }
            KeyType::X25519 => {
                let (keypair, _) = sr25519::Pair::from_string_with_seed(&s, None)?;
                let private = keypair.to_raw_vec();

                // hash the private key
                let mut hasher = Blake2b256::new();
                hasher.update(&private);
                let hashed = hasher.finalize();

                let secret: [u8; 32] = hashed[..32].try_into()?;
                let pair = crypto_box::SecretKey::from(secret);

                pair.public_key().as_bytes().to_vec()
            }
            KeyType::Ethereum => {
                let (keypair, _) = ecdsa::Pair::from_string_with_seed(&s, None)?;
                keypair.public().to_raw_vec()
            }
        };

        Ok(KeyInfo {
            id: id.to_string(),
            seed: Some(seed.to_string()),
            key_type,
            public_key,
            derive,
        })
    }

    pub fn from_pubkey(id: &str, key_type: KeyType, public_key: Vec<u8>) -> Self {
        Self {
            id: id.to_string(),
            key_type,
            public_key,
            seed: None,
            derive: None,
        }
    }

    pub fn save<S: Storage>(&self, storage: &mut S, id: &str) -> Result<(), Error> {
        let key = format!("keys/{}", id);
        storage
            .save(&key, self)
            .map_err(|e| Error::Key(e.to_string()))
    }

    pub fn load<S: Storage>(storage: &S, id: &str) -> Result<KeyInfo, Error> {
        let key = format!("keys/{}", id);
        storage.load(&key).map_err(|e| Error::Key(e.to_string()))
    }

    pub fn get_pair<P: Pair>(&self) -> Result<P, Error> {
        let seed = self
            .seed
            .as_ref()
            .ok_or(Error::Other("no seed available".into()))?
            .to_owned();

        let s = seed + &self.derive.clone().unwrap_or("".to_string());

        let (keypair, _) = P::from_string_with_seed(&s, None)?;

        Ok(keypair)
    }

    pub fn get_secret_key(&self) -> Result<crypto_box::SecretKey, Error> {
        let seed = self
            .seed
            .as_ref()
            .ok_or(Error::Other("no seed available".into()))?
            .to_owned();
        let s = seed + &self.derive.clone().unwrap_or("".to_string());

        let (keypair, _) = sr25519::Pair::from_string_with_seed(&s, None)?;
        let private = keypair.to_raw_vec();

        // hash the private key
        let mut hasher = Blake2b256::new();
        hasher.update(&private);
        let hashed = hasher.finalize();

        let secret: [u8; 32] = hashed[..32].try_into()?;

        Ok(crypto_box::SecretKey::from(secret))
    }

    pub fn to_ss58(&self) -> Result<String, Error> {
        let kilt = Ss58AddressFormat::try_from("kilt").unwrap();
        match self.key_type {
            KeyType::Sr25519 => {
                let public = subxt::sp_core::sr25519::Public(self.public_key[..].try_into()?);
                Ok(public.to_ss58check_with_version(kilt))
            }
            KeyType::Ed25519 => {
                let public = subxt::sp_core::ed25519::Public(self.public_key[..].try_into()?);
                Ok(public.to_ss58check_with_version(kilt))
            }
            KeyType::Ecdsa | KeyType::Ethereum => {
                let public = subxt::sp_core::ecdsa::Public(self.public_key[..].try_into()?);
                Ok(public.to_ss58check_with_version(kilt))
            }
            _ => Err(Error::Key("bad key type".into()))
        }
    }
}

pub fn create_cmd<S: Storage>(matches: &clap::ArgMatches, storage: &mut S) -> Result<(), Error> {
    let key_type = matches
        .value_of("type")
        .ok_or(Error::Other("no key type specified".into()))?
        .parse()?;

    let mut seed = matches
        .value_of("seed")
        .ok_or(Error::Other("no seed specified".into()))?
        .to_string();

    if seed.starts_with("@") {
        let storage_key = "seeds/".to_string() + &seed[1..];
        seed = storage.get(&storage_key)?;
    }

    let name = matches
        .value_of("name")
        .ok_or(Error::Other("no name specified".into()))?
        .to_string();

    let derive = matches.value_of("derive").map(|d| d.to_string());

    let key_info = KeyInfo::from_seed(&name, key_type, &seed, derive)?;

    key_info.save(storage, &name)?;

    Ok(())
}

pub fn list_cmd<S: Storage>(storage: &S) -> Result<(), Error> {
    let keys = storage.list("keys/")?;

    for key in keys {
        let key_info = KeyInfo::load(storage, &key)?;
        println!(
            "{} {} ({:?})",
            key_info.id,
            hex::encode(&key_info.public_key),
            key_info.key_type
        );
    }

    Ok(())
}

pub fn get_cmd<S: Storage>(matches: &clap::ArgMatches, storage: &S) -> Result<(), Error> {
    let id = matches
        .value_of("name")
        .ok_or(Error::Other("no id specified".into()))?;

    let key_info = KeyInfo::load(storage, id)?;
    println!("{:#?}", key_info);

    Ok(())
}

pub fn delete_cmd<S: Storage>(matches: &clap::ArgMatches, storage: &mut S) -> Result<(), Error> {
    let id = matches
        .value_of("name")
        .ok_or(Error::Other("no id specified".into()))?;

    let key = format!("keys/{}", id);
    storage.remove(&key)?;

    Ok(())
}

#[derive(Debug, Clone)]
pub enum Error {
    Key(String),
    Storage(String),
    Other(String),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Key(reason) => write!(f, "Key error: {}", reason),
            Error::Other(reason) => write!(f, "Other error: {}", reason),
            Error::Storage(reason) => write!(f, "Storage error: {}", reason),
        }
    }
}

impl From<SecretStringError> for Error {
    fn from(err: SecretStringError) -> Self {
        Error::Key(format!("{:?}", err))
    }
}

impl From<crate::storage::Error> for Error {
    fn from(err: crate::storage::Error) -> Self {
        Error::Storage(format!("{:?}", err))
    }
}

impl From<TryFromSliceError> for Error {
    fn from(err: TryFromSliceError) -> Self {
        Error::Other(format!("{:?}", err))
    }
}

mod tests {


    use super::*;

    const EXAMPLE_SEED: &str =
        "expose myth amazing approve like place gasp capital topic festival mother reunion";

    #[test]
    fn test_key_info_from_seed() {
        let derive = Some("//derive".to_string());
        let key_type = KeyType::Sr25519;
        let key_info = KeyInfo::from_seed(&"test", key_type, EXAMPLE_SEED, derive.clone()).unwrap();
        assert_eq!(key_info.key_type, key_type);
        assert_eq!(key_info.seed, Some(EXAMPLE_SEED.to_string()));
        assert_eq!(key_info.derive, derive);
    }

    #[test]
    fn test_key_info_save_load() {
        let derive = Some("//derive".to_string());
        let key_type = KeyType::Sr25519;
        let key_info = KeyInfo::from_seed(&"test", key_type, EXAMPLE_SEED, derive).unwrap();
        let id = "id".to_string();
        let mut storage = crate::storage::MemoryStorage::new();
        key_info.save(&mut storage, &id).unwrap();
        let key_info_loaded = KeyInfo::load(&storage, &id).unwrap();
        assert_eq!(key_info, key_info_loaded);
    }

    #[test]
    fn test_key_info_get_pair() {
        let derive = Some("//derive".to_string());
        let key_type = KeyType::Sr25519;
        let key_info = KeyInfo::from_seed(&"test", key_type, EXAMPLE_SEED, derive).unwrap();
        let pair = key_info.get_pair::<sr25519::Pair>().unwrap();
        let public_key = subxt::sp_core::ByteArray::to_raw_vec(&pair.public());
        assert_eq!(key_info.public_key, public_key);

        let sig = pair.sign(b"hello world");
        let ok = <subxt::sp_core::sr25519::Pair as subxt::sp_core::Pair>::verify(
            &sig,
            b"hello world",
            &pair.public(),
        );
        assert_eq!(ok, true);
    }

    #[test]
    fn test_key_info_get_secret_key() {
        use rand_chacha::rand_core::{SeedableRng};
        use crypto_box::aead::Aead;

        let derive = Some("//alice".to_string());
        let key_type = KeyType::X25519;
        let alice_key_info = KeyInfo::from_seed(&"test", key_type, EXAMPLE_SEED, derive).unwrap();
        let alice_secret_key = alice_key_info.get_secret_key().unwrap();

        let derive = Some("//bob".to_string());
        let key_type = KeyType::X25519;
        let bob_key_info = KeyInfo::from_seed(&"test", key_type, EXAMPLE_SEED, derive).unwrap();
        let bob_secret_key = bob_key_info.get_secret_key().unwrap();

        let bob_pk_bytes: [u8; 32] = bob_key_info.public_key.as_slice().try_into().unwrap();
        let bob_pk = crypto_box::PublicKey::from(bob_pk_bytes);
        let alice_box = crypto_box::Box::new(&bob_pk, &alice_secret_key);

        // create "random" nonce
        let mut rng = rand_chacha::ChaChaRng::from_entropy();
        let nonce = crypto_box::generate_nonce(&mut rng);

        let plaintext = "hello world";

        let ciphertext = alice_box.encrypt(&nonce, plaintext.as_bytes()).unwrap();

        let bobs_box = crypto_box::Box::new(&alice_secret_key.public_key(), &bob_secret_key);
        let decrypted = bobs_box.decrypt(&nonce, &ciphertext[..]).unwrap();

        assert_eq!(plaintext.as_bytes(), decrypted);
    }
}
