use clap::{arg, Command};

mod storage;
use did::{did_claim_web3name, did_create_cmd, did_register_cmd, did_list_cmd, did_show_cmd, did_resolve_cmd, did_generate_cmd, did_sign_and_submit};
use storage::{GitStorage, GpgStorage};

mod accounts;
use accounts::*;

mod seeds;
use seeds::*;

mod chain;
use chain::*;

mod credentials;
use credentials::*;

pub mod kilt;

mod did;

mod keys;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder().init();
    
    let matches = Command::new("kiltctl")
        .about("kilt command line client")
        .arg(arg!(-s --storage <STORAGE> "Path to the storage root directory")
            .default_value("~/.kiltctl")
            .required(false),
        )
        .arg(arg!(-e --endpoint <ENDPOINT> "chain endpoint")
            .default_value("wss://spiritnet.kilt.io:443")
            .required(false),
        )
        .arg(arg!(--gpg <GPG_ID> "gpg id to use when encrypting/decrypting").required(false))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("seed")
                .about("seed handling")
                .subcommand_required(true)
                .arg_required_else_help(true)  
                .subcommand(
            Command::new("generate")
                    .arg(arg!(-w --words <WORDS> "Number of words to generate").default_value("12").required(false))
                    .arg(arg!(<NAME> "Name of the seed").id("name").required(false))
                    .about("generate seed")
                )
                .subcommand(
                    Command::new("list")
                        .about("list seeds")
                )
                .subcommand(
                    Command::new("import")
                        .arg(arg!(<NAME> "Name of the seed").id("name"))
                        .arg(arg!(--path <PATH> "Path to the seed file"))
                        .about("import seed")
                )
                .subcommand(
                    Command::new("show")
                        .arg(arg!(<NAME> "Name of the seed").id("name"))
                        .about("show seed")
                )
                .subcommand(
                    Command::new("delete")
                        .alias("rm").alias("remove")
                        .arg(arg!(<NAME> "Name of the seed").id("name"))
                        .about("remove seed")
                )
        )
        .subcommand(Command::new("keys")
            .about("key handling")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("create")
                    .arg(arg!(--name <NAME> "Name of the key").id("name").required(true))
                    .arg(arg!(--seed <SEED> "Seed for the key").id("seed").required(true))
                    .arg(arg!(--type <TYPE> "Type of the key").id("type").required(false).default_value("sr25519"))
                    .arg(arg!(--derive <DERIVE> "Derive path for the key").id("derive").required(false))
                    .about("create a key")
            )
            .subcommand(
                Command::new("import")
                    .about("import (public) keys")
                    .arg(arg!(<NAME> "Name of the key").id("name").required(true))
                    .arg(arg!(<PUBKEY> "Public key").id("pubkey").required(true))
            )
            .subcommand(
                Command::new("list")
                    .about("list keys")
            )
            .subcommand(
                Command::new("show")
                    .arg(arg!(<NAME> "Name of the key").id("name"))
                    .about("show key")
            )
            .subcommand(
                Command::new("delete")
                    .alias("rm").alias("remove")
                    .arg(arg!(<NAME> "Name of the key").id("name"))
                    .about("remove key")
            )
        )
        .subcommand(
            Command::new("account")
                .about("account handling")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
            Command::new("create")
                    .about("create account")
                    .arg(arg!(-k --key <KEY> "The key to use for the account"))
                    .arg(arg!(-n --name <NAME> "Name of the account").id("name").required(false))
                )
                .subcommand(
                    Command::new("list")
                        .about("list accounts")
                )
                .subcommand(Command::new("show")
                    .arg(arg!(<NAME> "Name of the account").id("name"))
                    .about("show account")
                )
                .subcommand(Command::new("sign")
                    .arg(arg!(<NAME> "Name of the account").id("name"))
                    .about("sign message")
                )
                .subcommand(Command::new("verify")
                    .arg(arg!(<NAME> "Name of the account").id("name"))
                    .arg(arg!(-s --signature <SIGNATURE> "Signature to verify"))
                    .about("verify message")
                )
                .subcommand(Command::new("info")
                    .arg(arg!(<NAME> "Name of the account").id("name"))
                    .about("get on-chain info about the account")
                )
                .subcommand(Command::new("send")
                    .arg(arg!(--from <FROM> "source account"))
                    .arg(arg!(--to <TO> "target account"))
                    .arg(arg!(--amount <AMOUNT> "amount"))
                    .about("send kilts from one account to another")
                )
                .subcommand(Command::new("send_all")
                    .arg(arg!(--from <FROM> "source account"))
                    .arg(arg!(--to <TO> "target account"))
                    .arg(arg!(--keep-alive "Keep the source account alive").required(false))
                    .about("send all kilts from one account to another")
                )
                .subcommand(Command::new("delete")
                    .alias("rm").alias("remove")
                    .arg(arg!(<NAME> "Name of the account").id("name"))
                    .about("delete account")
                )
        )
        .subcommand(Command::new("chain")
            .about("chain handling")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("metadata")
                .about("get metadata")
                .arg(arg!(--json "Print the metadata in json format").required(false))
            )
            .subcommand(Command::new("runtime-version")
                .about("get runtime version")
            )
        )
        .subcommand(Command::new("credential")
            .about("credential handling")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("save")
                .arg(arg!(<NAME> "Name of the credential").id("name"))
                .arg(arg!(--path <PATH> "Path to the credential file").default_value("-").required(false))
            )
            .subcommand(Command::new("list")
                .about("list credentials")
                .arg(arg!(--prefix <PREFIX> "filter credentials by prefix").required(false))
            )
            .subcommand(Command::new("show")
                .arg(arg!(<NAME> "Name of the credential").id("name"))
            )
            .subcommand(Command::new("delete")
                .alias("rm")
                .arg(arg!(<NAME> "Name of the credential").id("name"))
            )
        )
        .subcommand(Command::new("did")
            .about("DID operations")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("create")
                .about("create a did")
                .arg(arg!(-n --name <NAME> "Name of the DID").id("name"))
                .arg(arg!(--auth <AUTH_ACCOUNT_NAME> "name of the initital auth account"))
                .arg(arg!(--delegation <DELEGATION_ACCOUNT_NAME> "name of the delegation account").required(false))
                .arg(arg!(--attestation <ATTESTATION_ACCOUNT_NAME> "name of the attestation account").required(false))
                .arg(arg!(--key_agreement <KEY_AGREEMENT_ACCOUNT_NAME> "name of the key agreement account").required(false).multiple_occurrences(true))
            )
            .subcommand(Command::new("list")
                .about("list dids")
            )
            .subcommand(Command::new("show")
                .arg(arg!(<NAME> "Name of the did").id("name"))
            )
            .subcommand(Command::new("register")
                .arg(arg!(<NAME> "Name of the did").id("name"))
                .arg(arg!(--payment <PAYMENT_ACCOUNT_NAME> "name of the payment account"))
            )
            .subcommand(Command::new("claim-web3-name")
                .about("claim a web3 name")
                .arg(arg!(--did <DID_NAME> "name of the did"))
                .arg(arg!(--payment <PAYMENT_ACCOUNT_NAME> "name of the payment account"))
                .arg(arg!(--name <NAME> "name to claim"))
            )  
            .subcommand(Command::new("resolve")
                .arg(arg!(<NAME> "Name of the did").id("name"))
            ) 
            .subcommand(Command::new("generate")
                .arg(arg!(<NAME> "Name of the did").id("name"))
                .arg(arg!(--seed <SEED> "seed for the did"))
            )
            .subcommand(Command::new("sign_and_submit")
                .arg(arg!(<NAME> "Name of the did").id("name"))
                .arg(arg!(--data <TX> "did call to sign and submit"))
                .arg(arg!(--payment <ACCOUNT> "payment account"))
            )
        )
        .get_matches();

    let mut storage_root: String;
    if let Ok(res) = std::env::var("KILTCTL_STORAGE") {
        storage_root = res.clone().into();
    }else{
        storage_root = matches.value_of("storage").unwrap().into();
    }
    storage_root = shellexpand::tilde(&storage_root).into();

    ensure_dir(&storage_root)?;

    let gpg_id = matches.value_of("gpg");
    let gpg_storage = GpgStorage::new(&storage_root, gpg_id);
    let mut storage = GitStorage::new(gpg_storage, &storage_root);

    let mut endpoint: String;
    if let Ok(res) = std::env::var("KILTCTL_ENDPOINT") {
        endpoint = res.clone().into();
    }else{
        endpoint = matches.value_of("endpoint").unwrap().into();
    }
    endpoint = match endpoint.as_str() {
        "spiritnet" => "wss://spiritnet.kilt.io:443".into(),
        "peregrine" => "wss://peregrine.kilt.io:443/parachain-public-ws".into(),
        _ => endpoint,
    };

    match matches.subcommand() {
        Some(("seed", sub_matches)) => match sub_matches.subcommand() {
            Some(("generate", sub_sub_matches)) => {
                seed_generate_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("list", _sub_sub_matches)) => {
                seed_list_cmd(&storage)?;
            }
            Some(("import", sub_sub_matches)) => {
                seed_import_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("show", sub_sub_matches)) => {
                seed_show_cmd(sub_sub_matches, &storage)?;
            }
            Some(("delete", sub_sub_matches)) => {
                seed_remove_cmd(sub_sub_matches, &mut storage)?;
            }
            _ => unreachable!(),
        },
        Some(("keys", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_sub_matches)) => {
                keys::create_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("list", _sub_sub_matches)) => {
                keys::list_cmd(&storage)?;
            }
            Some(("show", sub_sub_matches)) => {
                keys::get_cmd(sub_sub_matches, &storage)?;
            }
            Some(("delete", sub_sub_matches)) => {
                keys::delete_cmd(sub_sub_matches, &mut storage)?;
            }
            _ => unreachable!(),
        },
        Some(("account", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_sub_matches)) => {
                account_create_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("list", _sub_sub_matches)) => {
                account_list_cmd(&storage)?;
            }
            Some(("show", sub_sub_matches)) => {
                account_show_cmd(sub_sub_matches, &storage)?;
            }
            Some(("sign", sub_sub_matches)) => {
                account_sign_cmd(sub_sub_matches, &storage)?;
            }
            Some(("verify", sub_sub_matches)) => {
                account_verify_cmd(sub_sub_matches, &storage)?;
            }
            Some(("info", sub_sub_matches)) => {
                account_info_cmd(sub_sub_matches, &storage, &endpoint).await?;
            }
            Some(("send", sub_sub_matches)) => {
                account_send_cmd(sub_sub_matches, &storage, &endpoint).await?;
            }
            Some(("send_all", sub_sub_matches)) => {
                account_send_all_cmd(sub_sub_matches, &storage, &endpoint).await?;
            }
            Some(("delete", sub_sub_matches)) => {
                account_remove_cmd(sub_sub_matches, &mut storage)?;
            }

            _ => unreachable!(),
        },
        Some(("chain", sub_matches)) => match sub_matches.subcommand() {
            Some(("metadata", sub_sub_matches)) => {
                let json = sub_sub_matches.is_present("json");
                chain_metadata_cmd(&endpoint, json)?;
            }
            Some(("runtime-version", _sub_sub_matches)) => {
                chain_runtime_version_cmd(&endpoint)?;
            }
            _ => unreachable!(),
        },
        Some(("credential", sub_matches)) => match sub_matches.subcommand() {
            Some(("save", sub_sub_matches)) => {
                credential_save_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("list", sub_sub_matches)) => {
                credential_list_cmd(sub_sub_matches, &storage)?;
            }
            Some(("show", sub_sub_matches)) => {
                credential_show_cmd(sub_sub_matches, &storage)?;
            }
            Some(("delete", sub_sub_matches)) => {
                credential_delete_cmd(sub_sub_matches, &mut storage)?;
            }
            _ => unreachable!(),
        },
        Some(("did", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_sub_matches)) => {
                did_create_cmd(sub_sub_matches, &mut storage)?;
            },
            Some(("list", _sub_sub_matches)) => {
                did_list_cmd(&storage)?;
            },
            Some(("show", sub_sub_matches)) => {
                did_show_cmd(sub_sub_matches, &storage)?;
            },
            Some(("register", sub_sub_matches)) => {
                did_register_cmd(sub_sub_matches, &storage, &endpoint).await?;
            },
            Some(("claim-web3-name", sub_sub_matches)) => {
                did_claim_web3name(sub_sub_matches, &mut storage, &endpoint).await?;
            },
            Some(("resolve", sub_sub_matches)) => {
                did_resolve_cmd(sub_sub_matches, &storage, &endpoint).await?;
            },
            Some(("generate", sub_sub_matches)) => {
                did_generate_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("sign_and_submit", sub_sub_matches)) => {
                did_sign_and_submit(sub_sub_matches, &mut storage, &endpoint).await?;
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Ok(())
}

fn ensure_dir(path: &str) -> Result<(), std::io::Error> {
    let path = std::path::Path::new(path);
    std::fs::create_dir_all(path)?;
    Ok(())
}
