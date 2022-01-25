use clap::{arg, App, AppSettings};

mod storage;
use storage::{GitStorage, GpgStorage};

mod accounts;
use accounts::*;

mod seeds;
use seeds::*;

mod chain;
use chain::*;

mod credentials;
use credentials::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("kiltctl")
        .about("kilt command line client")
        .arg(arg!(-s --storage <STORAGE> "Path to the storage root directory")
            .default_value("~/.kiltctl")
            .required(false),
        )
        .arg(arg!(-e --endpoint <ENDPOINT> "chain endpoint")
            .default_value("wss://spiritnet.kilt.io")
            .required(false),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("seed")
                .about("seed handling")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
            App::new("generate")
                    .arg(arg!(-w --words <WORDS> "Number of words to generate").default_value("12").required(false))
                    .arg(arg!(<NAME> "Name of the seed").name("name").required(false))
                    .about("generate seed")
                )
                .subcommand(
                    App::new("list")
                        .about("list seeds")
                )
                .subcommand(
                    App::new("import")
                        .arg(arg!(<NAME> "Name of the seed").name("name"))
                        .arg(arg!(--path <PATH> "Path to the seed file"))
                        .about("import seed")
                )
                .subcommand(
                    App::new("show")
                        .arg(arg!(<NAME> "Name of the seed").name("name"))
                        .about("show seed")
                )
        )
        .subcommand(
            App::new("account")
                .about("account handling")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
            App::new("generate")
                    .about("generate account")
                    .arg(arg!(-s --seed <SEED> "The seed to use for the account"))
                    .arg(arg!(-d --derive <DERIVE> "The derive path to use, i.e. '//kilt/123'").required(false))
                    .arg(arg!(-p --password <PASSWORD> "The password to use for the account").required(false))
                    .arg(arg!(--suffix <SUFFIX> "Optionally a suffix that the account has to have").required(false))
                    .arg(arg!(-a --algorithm <ALGORITHM> "The signature algorithm to use, i.e. 'sr25519', 'ed25519' or 'ecdsa'").default_value("sr25519").required(false))
                    .arg(arg!(<NAME> "Name of the account").name("name").required(false))
                    .setting(AppSettings::ArgRequiredElseHelp),
                )
                .subcommand(App::new("import")
                    .arg(arg!(<NAME> "Name of the account").name("name"))
                    .arg(arg!(-a --algorithm <ALGORITHM> "The signature algorithm to use, i.e. 'sr25519', 'ed25519' or 'ecdsa'").required(true))
                    .arg(arg!(-a --address <ADDRESS> "The address to use for the account").required(true))
                    .about("import account")
                )
                .subcommand(
                    App::new("list")
                        .about("list accounts")
                )
                .subcommand(App::new("show")
                    .arg(arg!(<NAME> "Name of the account").name("name"))
                    .about("show account")
                )
                .subcommand(App::new("sign")
                    .arg(arg!(<NAME> "Name of the account").name("name"))
                    .about("sign message")
                )
                .subcommand(App::new("verify")
                    .arg(arg!(<NAME> "Name of the account").name("name"))
                    .arg(arg!(-s --signature <SIGNATURE> "Signature to verify"))
                    .about("verify message")
                )
                .subcommand(App::new("info")
                    .arg(arg!(<NAME> "Name of the account").name("name"))
                    .about("get on-chain info about the account")
                )
                .subcommand(App::new("send")
                    .arg(arg!(--from <FROM> "source account"))
                    .arg(arg!(--to <TO> "target account"))
                    .arg(arg!(--amount <AMOUNT> "amount"))
                    .about("get on-chain info about the account")
                )

        )
        .subcommand(App::new("chain")
            .about("chain handling")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(App::new("metadata")
                .about("get metadata")
                .arg(arg!(--json "Print the metadata in JSON format"))
            )
            .subcommand(App::new("runtime-version")
                .about("get runtime version")
            )
        )
        .subcommand(App::new("credential")
            .about("credential handling")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(App::new("save")
                .arg(arg!(<NAME> "Name of the credential").name("name"))
                .arg(arg!(--path <PATH> "Path to the credential file").default_value("-").required(false))
            )
            .subcommand(App::new("list")
                .about("list credentials")
                .arg(arg!(--prefix <PREFIX> "filter credentials by prefix").required(false))
            )
            .subcommand(App::new("show")
                .arg(arg!(<NAME> "Name of the credential").name("name"))
            )
            .subcommand(App::new("delete")
                .alias("rm")
                .arg(arg!(<NAME> "Name of the credential").name("name"))
            )
        )
        .get_matches();

    let storage_root = shellexpand::tilde(matches.value_of("storage").unwrap());
    let gpg_storage = GpgStorage::new(&storage_root);
    let mut storage = GitStorage::new(gpg_storage, &storage_root);
    
    let url = matches.value_of("endpoint").unwrap();

    match matches.subcommand() {
        Some(("seed", sub_matches)) => match sub_matches.subcommand() {
            Some(("generate", sub_sub_matches)) => {
                seed_generate_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("list", _sub_sub_matches)) => {
                seed_list_cmd(&mut storage)?;
            }
            Some(("import", sub_sub_matches)) => {
                seed_import_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("show", sub_sub_matches)) => {
                seed_show_cmd(sub_sub_matches, &mut storage)?;
            }
            _ => unreachable!(),
        },
        Some(("account", sub_matches)) => match sub_matches.subcommand() {
            Some(("generate", sub_sub_matches)) => {
                account_generate_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("list", _sub_sub_matches)) => {
                account_list_cmd(&mut storage)?;
            }
            Some(("show", sub_sub_matches)) => {
                account_show_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("import", sub_sub_matches)) => {
                account_import_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("sign", sub_sub_matches)) => {
                account_sign_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("verify", sub_sub_matches)) => {
                account_verify_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("info", sub_sub_matches)) => {
                account_info_cmd(sub_sub_matches, &mut storage, url)?;
            }
            Some(("send", sub_sub_matches)) => {
                account_send_cmd(sub_sub_matches, &mut storage, url)?;
            }

            _ => unreachable!(),
        },
        Some(("chain", sub_matches)) => match sub_matches.subcommand() {
            Some(("metadata", sub_sub_matches)) => {
                let json = sub_sub_matches.is_present("json");
                chain_metadata_cmd(url, json)?;
            }
            Some(("runtime-version", _sub_sub_matches)) => {
                chain_runtime_version_cmd(url)?;
            }
            _ => unreachable!(),
        },
        Some(("credential", sub_matches)) => match sub_matches.subcommand() {
            Some(("save", sub_sub_matches)) => {
                credential_save_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("list", sub_sub_matches)) => {
                credential_list_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("show", sub_sub_matches)) => {
                credential_show_cmd(sub_sub_matches, &mut storage)?;
            }
            Some(("delete", sub_sub_matches)) => {
                credential_delete_cmd(sub_sub_matches, &mut storage)?;
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Ok(())
}
