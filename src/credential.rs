use base58::{FromBase58, ToBase58};
use serde_json::json;

use rand::RngCore;

use blake2::{digest::consts::U32, Blake2b, Digest};
type Blake2b256 = Blake2b<U32>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Credential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    #[serde(rename = "type")]
    pub type_: Vec<String>,
    pub id: String,
    #[serde(rename = "nonTransferable")]
    pub non_transferable: bool,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: serde_json::Map<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "issuanceDate", skip_serializing_if = "Option::is_none")]
    pub issuance_date: Option<String>,
    #[serde(rename = "federatedTrustModel")]
    pub federated_trust_model: Vec<FederatedTrustModel>,
    #[serde(rename = "credentialStatus")]
    pub credential_status: Status,
    #[serde(rename = "credentialSchema")]
    pub credential_schema: Schema,
    pub proof: Proof,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FederatedTrustModel {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Status {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Schema {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Proof {
    #[serde(rename = "type")]
    pub type_: String,
    pub block: String,
    pub commitments: Vec<String>,
    #[serde(rename = "revealProof")]
    pub reveal_proof: Vec<String>,
}

pub struct CredentialBuilder {
    context: Vec<String>,
    type_: Vec<String>,
    id: Option<String>,
    ctype: Option<String>,
    non_transferable: bool,
    credential_subject: Option<serde_json::Map<String, serde_json::Value>>,
    issuer: Option<String>,
    issuance_date: Option<String>,
    federated_trust_model: Vec<FederatedTrustModel>,
    credential_status: Option<Status>,
    credential_schema: Option<Schema>,
    proof: Option<Proof>,
}

impl Default for CredentialBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialBuilder {
    pub fn new() -> Self {
        Self {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_string(),
                "https://www.kilt.io/contexts/credentials".to_string(),
            ],
            type_: vec![
                "VerifiableCredential".to_string(),
                "KiltCredentialV1".to_string(),
            ],
            id: None,
            ctype: None,
            non_transferable: true,
            credential_subject: None,
            issuer: None,
            issuance_date: None,
            federated_trust_model: vec![],
            credential_status: None,
            credential_schema: None,
            proof: None,
        }
    }

    pub fn with_ctype(mut self, ctype: &str) -> Self {
        self.ctype = Some(ctype.to_string());
        self.credential_status = Some(Status {
            id: format!("kilt:ctype:{}", ctype),
            type_: "JsonSchemaValidator2018".to_string(),
        });
        self
    }

    pub fn with_credential_subject(
        mut self,
        credential_subject: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        let mut subject = credential_subject;
        if let Some(ctype) = &self.ctype {
            subject.insert(
                "@context".to_string(),
                json!({ "@vocab": format!("kilt:ctype:{}#", ctype) }),
            );
        }
        self.credential_subject = Some(subject);
        self
    }

    pub fn with_issuer(mut self, issuer: String) -> Self {
        self.issuer = Some(issuer);
        self
    }

    pub fn with_issuance_date(mut self, issuance_date: String) -> Self {
        self.issuance_date = Some(issuance_date);
        self
    }

    pub fn with_federated_trust_model(
        mut self,
        federated_trust_model: Vec<FederatedTrustModel>,
    ) -> Self {
        self.federated_trust_model = federated_trust_model;
        self
    }

    pub fn create_proof(mut self, block: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut proof = Proof {
            type_: "KiltAttestationProofV1".to_string(),
            block: block.to_string(),
            commitments: vec![],
            reveal_proof: vec![],
        };

        let mut credential_subject = self
            .credential_subject
            .clone()
            .expect("credential subject is required");

        // respect @vocab
        if let Some(context) = credential_subject.get("@context") {
            if let Some(ctx_obj) = context.as_object() {
                if let Some(vocab) = ctx_obj.get("@vocab") {
                    if let Some(vocab_str) = vocab.as_str() {
                        let mut new_credential_subject = serde_json::Map::new();
                        credential_subject.iter().for_each(|(key, value)| {
                            if key != "@context" {
                                let mut key = key.to_owned();
                                if !key.starts_with('@') {
                                    key = format!("{}{}", vocab_str, key);
                                }
                                new_credential_subject.insert(key, value.clone());
                            }
                        });
                        credential_subject = new_credential_subject;
                    }
                }
            }
        }

        // compute prehashes for all claims
        let mut pre_hashes = vec![];
        for (key, value) in credential_subject.iter() {
            let obj = serde_json::to_vec(&json!({ key: value }))?;
            let mut hasher = Blake2b256::new();
            hasher.update(&obj);
            let hash = hasher.finalize().to_vec();
            pre_hashes.push(hash);
        }

        // sort numerically
        pre_hashes.sort();

        // collect data for the root hash on the fly
        let mut root_hash_hasher = Blake2b256::new();
        let mut rng = rand::thread_rng();

        // for each pre_hash create a salt, concat the salt with 0x3078 and the pre_hash, and hash it again
        // the result is your commitment.
        // save commitment and salt to the proof
        // add the commitment to the root hash
        pre_hashes.iter().for_each(|pre_hash| {
            // create salt
            let mut salt = [0u8; 36];
            rng.fill_bytes(&mut salt);
            proof.reveal_proof.push(salt.to_base58());

            // hash salt + pre_hash
            let mut hasher = Blake2b256::new();
            hasher.update(salt);
            hasher.update(pre_hash);
            let hash = hasher.finalize().to_vec();

            // add commitment to proof
            proof.commitments.push(hash.to_base58());

            // add commitment to root hash
            root_hash_hasher.update(&hash);
        });

        // finalize root hash and save credential id
        let root_hash = root_hash_hasher.finalize().to_vec();
        self.id = format!("kilt:credential:{}", root_hash.to_base58()).into();

        // prepare the credential status field
        self.credential_status = Some(Status {
            id: format!("kilt:attestation:{}", root_hash.to_base58()),
            type_: "KiltRevocationStatusV1".to_string(),
        });

        self.credential_schema = Some(Schema {
            id: self.ctype.clone().expect("ctype is required"),
            type_: "JsonSchemaValidator2018".to_string(),
        });

        // set the proof
        self.proof = Some(proof);

        Ok(self)
    }

    pub fn build(self) -> Result<Credential, Box<dyn std::error::Error>> {
        Ok(Credential {
            context: self.context,
            type_: self.type_,
            id: self.id.ok_or("id is required")?,
            non_transferable: self.non_transferable,
            credential_subject: self
                .credential_subject
                .ok_or("credential_subject is required")?,
            issuer: self.issuer,
            issuance_date: self.issuance_date,
            federated_trust_model: self.federated_trust_model,
            credential_status: self
                .credential_status
                .ok_or("credential_status is required")?,
            credential_schema: self
                .credential_schema
                .ok_or("credential_schema is required")?,
            proof: self.proof.ok_or("proof is required")?,
        })
    }
}

impl Credential {
    pub fn verify(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.proof.type_ != "KiltAttestationProofV1" {
            return Err("invalid proof type".into());
        }

        let mut credential_subject = self.credential_subject.clone();

        // respect @vocab
        if let Some(context) = credential_subject.get("@context") {
            if let Some(ctx_obj) = context.as_object() {
                if let Some(vocab) = ctx_obj.get("@vocab") {
                    if let Some(vocab_str) = vocab.as_str() {
                        let mut new_credential_subject = serde_json::Map::new();
                        credential_subject.iter().for_each(|(key, value)| {
                            if key != "@context" {
                                let mut key = key.to_owned();
                                if !key.starts_with('@') {
                                    key = format!("{}{}", vocab_str, key);
                                }
                                new_credential_subject.insert(key, value.clone());
                            }
                        });
                        credential_subject = new_credential_subject;
                    }
                }
            }
        }

        // compute prehashes for all claims
        let mut pre_hashes = vec![];
        for (key, value) in credential_subject.iter() {
            let obj = serde_json::to_vec(&json!({ key: value }))?;
            let mut hasher = Blake2b256::new();
            hasher.update(&obj);
            let hash = hasher.finalize().to_vec();
            pre_hashes.push(hash);
        }

        // sort numerically
        pre_hashes.sort();

        if pre_hashes.len() != self.proof.reveal_proof.len() {
            return Err("reveal proof length does not match number of claims".into());
        }

        for (i, pre_hash) in pre_hashes.iter().enumerate() {
            // hash salt + 0x3078 + pre_hash
            let mut hasher = Blake2b256::new();
            hasher.update(
                &self.proof.reveal_proof[i]
                    .from_base58()
                    .map_err(|_| "failed to parse reveal proof")?,
            );
            hasher.update(pre_hash);
            let hash = hasher.finalize().to_vec();
            if !self.proof.commitments.contains(&hash.to_base58()) {
                return Err("commitment does not match reveal proof".into());
            }
        }

        // check root hash
        let mut root_hash_hasher = Blake2b256::new();
        self.proof.commitments.iter().try_for_each(
            |commitment| -> Result<(), Box<dyn std::error::Error>> {
                root_hash_hasher.update(
                    &commitment
                        .from_base58()
                        .map_err(|_| "failed to parse commitment")?,
                );
                Ok(())
            },
        )?;
        let root_hash = root_hash_hasher.finalize().to_vec();

        let id = format!("kilt:credential:{}", root_hash.to_base58());
        if id != self.id {
            return Err("root hash does not match id".into());
        }

        Ok(())
    }
}

mod test {

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        use super::*;

        pretty_env_logger::init();

        let cred = CredentialBuilder::new()
            .with_ctype(
                "kilt:ctype:0x0586412d7b8adf811c288211c9c704b3331bb3adb61fba6448c89453568180f6",
            )
            .with_credential_subject(
                json!({
                    "@id": "kilt:did:123456789",
                    "val_1": 1,
                    "val_2": 2,
                })
                .as_object()
                .unwrap()
                .clone(),
            )
            .with_issuer("kilt:did:123456789".to_string())
            .with_issuance_date("2021-01-01T00:00:00Z".to_string())
            .create_proof("123456789")?
            .build()?;

        cred.verify().expect("verify failed");

        println!("{}", serde_json::to_string_pretty(&cred)?);

        Ok(())
    }
}
