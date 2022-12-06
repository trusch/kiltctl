mod create;
mod hash;

pub fn command() -> clap::Command {
    clap::Command::new("ctype")
        .about("CType commands")
        .subcommand_required(true)
        .subcommands([create::command(), hash::command()])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("create", matches)) => create::run(matches).await,
        Some(("hash", matches)) => hash::run(matches),
        _ => Ok(()),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CType {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$schema")]
    pub schema: String,

    pub properties: serde_json::Map<String, serde_json::Value>,

    pub title: String,

    #[serde(rename = "type")]
    pub type_: String,
}

impl CType {
    pub fn new(title: String, properties: serde_json::Map<String, serde_json::Value>) -> Self {
        Self {
            id: None,
            title,
            type_: "object".to_string(),
            schema: "http://kilt-protocol.org/draft-01/ctype#".to_string(),
            properties,
        }
    }

    pub fn serialize(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut copy = self.clone();
        copy.id = None;
        Ok(serde_json::to_string(&copy)?)
    }
}

mod test {
    #[test]
    fn test_ctype() {
        use super::*;

        let ctype = CType::new(
            "test".to_string(),
            serde_json::json!({
                "test": {
                    "z": "string",
                    "a": "string",
                }
            })
            .as_object()
            .unwrap()
            .clone(),
        );
        let sorted = ctype.serialize().unwrap();
        assert_eq!(
            sorted,
            r#"{"$schema":"http://json-schema.org/draft-07/schema#","properties":{"test":{"a":"string","z":"string"}},"title":"test","type":"object"}"#
        );
    }
}
