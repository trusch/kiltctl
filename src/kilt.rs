use subxt::{ClientBuilder, Config, DefaultConfig, DefaultExtra};

// This generates the kilt runtime api for us
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod kilt {}

// Unfortunately there is a but in there which causes that the DidEncryptionKey type doesn't implement PartialOrd, so we need to add it manually
const _: () = {
    use kilt::runtime_types::did::did_details::DidEncryptionKey;

    impl PartialEq for DidEncryptionKey {
        fn eq(&self, other: &Self) -> bool {
            self == other
        }
    }

    impl Eq for DidEncryptionKey {}

    impl PartialOrd for DidEncryptionKey {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self {
                Self::X25519(x) => match other {
                    Self::X25519(y) => x.as_ref().partial_cmp(y.as_ref()),
                },
            }
        }
    }

    impl Ord for DidEncryptionKey {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self {
                Self::X25519(x) => match other {
                    Self::X25519(y) => x.as_ref().cmp(y.as_ref()),
                },
            }
        }
    }
};

// re-export all the auto generated code
pub use kilt::*;

// This is the runtime config for kilt. It only differs in the Index type from the default
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct KiltConfig;
impl Config for KiltConfig {
    type Index = u64;
    type BlockNumber = <DefaultConfig as Config>::BlockNumber;
    type Hash = <DefaultConfig as Config>::Hash;
    type Hashing = <DefaultConfig as Config>::Hashing;
    type AccountId = <DefaultConfig as Config>::AccountId;
    type Address = <DefaultConfig as Config>::Address;
    type Header = <DefaultConfig as Config>::Header;
    type Signature = <DefaultConfig as Config>::Signature;
    type Extrinsic = <DefaultConfig as Config>::Extrinsic;
}

/// connect to a websocket endpoint using the KiltConfig
pub async fn connect<U: Into<String>>(
    url: U,
) -> Result<kilt::RuntimeApi<KiltConfig, DefaultExtra<KiltConfig>>, subxt::BasicError> {
    let api = ClientBuilder::new()
        .set_url(url)
        .build()
        .await?
        .to_runtime_api::<kilt::RuntimeApi<KiltConfig, DefaultExtra<KiltConfig>>>();
    Ok(api)
}
