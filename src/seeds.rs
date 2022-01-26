use crate::storage::Storage;
use bip39::{Language, Mnemonic, MnemonicType};

pub fn seed_generate_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let words = matches.value_of("words").unwrap().parse::<usize>().unwrap();
    let seed = generate_seed(words)?;
    if let Some(name) = matches.value_of("name") {
        let storage_key = "seeds/".to_string() + name;
        storage.set(&storage_key, &seed)?;
    } else {
        println!("{}", seed);
    }
    Ok(())
}

pub fn seed_import_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let path = matches.value_of("path").unwrap();
    let seed = std::fs::read_to_string(path)?.trim().to_string();
    let storage_key = "seeds/".to_string() + name;
    storage.set(&storage_key, &seed)?;
    Ok(())
}

pub fn seed_show_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let storage_key = "seeds/".to_string() + name;
    let seed = storage.get(&storage_key)?;
    println!("{}", seed);
    Ok(())
}

pub fn seed_list_cmd<S: Storage>(storage: &S) -> Result<(), Box<dyn std::error::Error>> {
    let seeds = storage.list("seeds/")?;
    for name in seeds {
        println!("{}", name);
    }
    Ok(())
}

pub fn seed_remove_cmd<S: Storage>(
    matches: &clap::ArgMatches,
    storage: &mut S,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("name").unwrap();
    let storage_key = "seeds/".to_string() + name;
    storage.remove(&storage_key)?;
    Ok(())
}

fn generate_seed(words: usize) -> Result<String, Box<dyn std::error::Error>> {
    let m_type = MnemonicType::for_word_count(words)?;
    let mnemonic = Mnemonic::new(m_type, Language::English);
    Ok(mnemonic.phrase().into())
}
