use std::{io::Read, str::FromStr};

use clap::error::ErrorKind::{Format, InvalidValue};
use kilt::KiltConfig;
use subxt::ext::sp_core::H256;
use subxt::{tx::TxPayload, utils::AccountId32, OnlineClient};

pub mod credential;
pub mod kilt;
#[derive(Debug, Clone)]
pub struct AccountIdParser;

impl clap::builder::TypedValueParser for AccountIdParser {
    type Value = subxt::utils::AccountId32;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let val = value
            .to_os_string()
            .into_string()
            .map_err(|_| clap::Error::new(InvalidValue))?
            .trim_start_matches("did:kilt:")
            .to_owned();
        let account = AccountId32::from_str(&val).map_err(|_| clap::Error::new(Format))?;
        Ok(account)
    }
}

#[derive(Debug, Clone)]
pub struct BalanceParser;

impl clap::builder::TypedValueParser for BalanceParser {
    type Value = u128;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let val = value
            .to_os_string()
            .into_string()
            .map_err(|_| clap::Error::new(InvalidValue))?;
        if val.ends_with('K') || val.ends_with("KILT") {
            let val = val
                .trim_end_matches('K')
                .trim_end_matches("KILT")
                .to_owned();
            let val = val
                .parse::<u128>()
                .map_err(|_| clap::Error::new(InvalidValue))?;
            Ok(val * 1_000_000_000_000_000)
        } else {
            let val = val
                .parse::<u128>()
                .map_err(|_| clap::Error::new(InvalidValue))?;
            Ok(val)
        }
    }
}

#[derive(Debug, Clone)]
pub struct HashParser;

impl clap::builder::TypedValueParser for HashParser {
    type Value = H256;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let val = value
            .to_os_string()
            .into_string()
            .map_err(|_| clap::Error::new(Format))?
            .trim_start_matches("0x")
            .trim()
            .to_string();
        let bytes: [u8; 32] = hex::decode(val)
            .map_err(|_| clap::Error::new(Format))?
            .try_into()
            .map_err(|_| clap::Error::new(Format))?;
        Ok(H256::from(bytes))
    }
}

#[derive(Debug, Clone)]
pub struct CallParser;

impl clap::builder::TypedValueParser for CallParser {
    type Value = RawCall;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let val = value
            .to_os_string()
            .into_string()
            .map_err(|_| clap::Error::new(Format))?
            .trim_start_matches("0x")
            .trim()
            .to_string();
        let bytes = hex::decode(val).map_err(|_| clap::Error::new(Format))?;

        Ok(RawCall { call: bytes })
    }
}

#[derive(Debug, Clone)]
pub struct RawCall {
    pub call: Vec<u8>,
}

impl TxPayload for RawCall {
    fn encode_call_data_to(
        &self,
        _metadata: &subxt::Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), subxt::Error> {
        out.extend_from_slice(&self.call);
        Ok(())
    }
}

pub fn unwrap_or_stdin(data: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    match data {
        Some(data) => Ok(data),
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

pub fn format_balance(b: u128) -> String {
    if b < 1_000_000_000_000_000 {
        let d = b / 1_000_000_000_000;
        let mut r = (b % 1_000_000_000_000).to_string();
        if r.len() < 12 {
            r = format!("{r:0>12}");
        }
        r = r[..4].to_string();
        format!("{d}.{r} mKILT")
    } else {
        let d = b / 1_000_000_000_000_000;
        let mut r = (b % 1_000_000_000_000_000).to_string();
        if r.len() < 15 {
            r = format!("{r:0>15}");
        }
        r = r[..4].to_string();
        format!("{d}.{r} KILT")
    }
}

pub async fn connect(
    matches: &clap::ArgMatches,
) -> Result<OnlineClient<KiltConfig>, Box<dyn std::error::Error>> {
    let endpoint: &String = matches.get_one("endpoint").expect("need endpoint");
    let endpoint_url = match endpoint.as_str() {
        "spiritnet" => "wss://spiritnet.kilt.io:443",
        "peregrine" => "wss://peregrine.kilt.io:443/parachain-public-ws",
        _ => endpoint.as_str(),
    };
    Ok(OnlineClient::<KiltConfig>::from_url(endpoint_url).await?)
}
