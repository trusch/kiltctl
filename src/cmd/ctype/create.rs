use kiltapi::unwrap_or_stdin;

use crate::ctype::CType;

pub fn command() -> clap::Command {
    clap::Command::new("create")
        .about("Create a new CType")
        .arg(
            clap::Arg::new("title")
                .short('t')
                .long("title")
                .help("CType title")
                .required(true)
                .env("TITLE"),
        )
        .arg(
            clap::Arg::new("properties")
                .short('p')
                .long("properties")
                .help("CType properties")
                .required(true)
                .env("PROPERTIES"),
        )
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let title = matches
        .get_one::<String>("title")
        .expect("need ctype title");
    let props_str = unwrap_or_stdin(
        matches
            .get_one::<String>("properties")
            .map(|e| e.to_owned()),
    )?;
    let props: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&props_str)?;

    let ctype = CType::new(title.to_owned(), props);

    println!("{}", serde_json::to_string_pretty(&ctype)?);

    Ok(())
}
