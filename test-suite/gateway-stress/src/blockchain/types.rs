use std::str::FromStr;

use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::Decryption::{
    PublicDecryptionRequest, UserDecryptionRequest,
};
use serde::{Deserialize, Deserializer, Serializer};

#[derive(Debug, Clone)]
pub enum DecryptionRequest {
    Public(PublicDecryptionRequest),
    User(UserDecryptionRequest),
}

#[derive(Copy, Clone, Debug)]
pub enum DecryptionType {
    Public,
    User,
}

impl FromStr for DecryptionType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "u" || s.starts_with("user") {
            Ok(DecryptionType::User)
        } else if s == "p" || s.starts_with("public") {
            Ok(DecryptionType::Public)
        } else {
            Err(anyhow!("Invalid decryption type"))
        }
    }
}

pub fn decryption_type_from_str<'de, D>(deserializer: D) -> Result<DecryptionType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.to_lowercase();
    s.parse().map_err(serde::de::Error::custom)
}

pub fn decryption_type_serialize<S>(d: &DecryptionType, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match d {
        DecryptionType::Public => s.serialize_str("public"),
        DecryptionType::User => s.serialize_str("user"),
    }
}

impl DecryptionRequest {
    pub fn type_str(&self) -> String {
        match self {
            DecryptionRequest::Public(_) => "PublicDecryptionRequest".to_string(),
            DecryptionRequest::User(_) => "UserDecryptionRequest".to_string(),
        }
    }
}
