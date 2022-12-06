use bip39::{Language, Mnemonic, MnemonicType};

pub fn command() -> clap::Command {
    clap::Command::new("generate")
        .about("Generate a new seed")
        .arg(
            clap::Arg::new("words")
                .short('w')
                .long("words")
                .help("number of words to use")
                .default_value("12"),
        )
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let words = matches.get_one::<String>("words").expect("need words");
    let words = words.parse::<usize>().expect("need words");
    let m_type = MnemonicType::for_word_count(words)?;
    let mnemonic = Mnemonic::new(m_type, Language::English);
    println!("{}", mnemonic.phrase());
    Ok(())
}
