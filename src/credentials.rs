use std::io::Read;

use clap::ArgMatches;

use crate::storage::Storage;

pub fn credential_save_cmd<S: Storage>(
    sub_sub_matches: &ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = sub_sub_matches.value_of("name").unwrap();
    let path = sub_sub_matches.value_of("path").unwrap();
    if path == "-" {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        let key = "credentials/".to_string() + name;
        storage.set(&key, &buffer)?;
    } else {
        let mut file = std::fs::File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let key = "credentials/".to_string() + name;
        storage.set(&key, &buffer)?;
    }
    Ok(())
}

pub fn credential_show_cmd<S: Storage>(
    sub_sub_matches: &ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = sub_sub_matches.value_of("name").unwrap();
    let key = "credentials/".to_string() + name;
    let value = storage.get(&key)?;
    println!("{}", value);
    Ok(())
}

pub fn credential_list_cmd<S: Storage>(
    sub_sub_matches: &ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let prefix = sub_sub_matches.value_of("prefix");
    let credentials = storage.list("credentials/")?;
    for name in credentials {
        if prefix.is_none() || name.starts_with(prefix.unwrap()) {
            println!("{}", name);
        }
    }
    Ok(())
}

pub fn credential_delete_cmd<S: Storage>(
    sub_sub_matches: &ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = sub_sub_matches.value_of("name").unwrap();
    let key = "credentials/".to_string() + name;
    storage.remove(&key)?;
    Ok(())
}
