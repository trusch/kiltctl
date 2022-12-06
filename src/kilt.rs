use subxt::{
    config::Config,
    ext::sp_runtime::traits::{IdentifyAccount, Verify},
    tx::PolkadotExtrinsicParams,
};

#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod kilt {}

// re-export all the auto generated code
pub use kilt::*;

pub type Call = kilt::runtime_types::spiritnet_runtime::Call;
pub type Event = kilt::runtime_types::spiritnet_runtime::Event;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct KiltConfig;
impl Config for KiltConfig {
    type Index = u64;
    type BlockNumber = u64;
    type Hash = subxt::ext::sp_core::H256;
    type Hashing = subxt::ext::sp_runtime::traits::BlakeTwo256;
    type AccountId = <<Self::Signature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = subxt::ext::sp_runtime::MultiAddress<Self::AccountId, ()>;
    type Header = subxt::ext::sp_runtime::generic::Header<Self::BlockNumber, Self::Hashing>;
    type Signature = subxt::ext::sp_runtime::MultiSignature;
    type ExtrinsicParams = PolkadotExtrinsicParams<Self>;
}
