use std::str::FromStr;

use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::Decryption::{
    PublicDecryptionRequest_1 as PublicDecryptionRequest,
    UserDecryptionRequest_2 as UserDecryptionRequest,
    UserDecryptionRequest_3 as UserDecryptionRequestV2,
};
use serde::{Deserialize, Deserializer, Serializer};
use std::fmt;

#[derive(Clone)]
pub enum DecryptionRequest {
    Public(PublicDecryptionRequest),
    User(UserDecryptionRequest),
    UserV2(UserDecryptionRequestV2),
}

// `UserDecryptionRequest_3` (the RFC-016 event) doesn't derive `Debug` in the generated bindings,
// so we can't `#[derive(Debug)]` the enum — a compact manual impl keyed on the variant is enough
// for the tool's tracing needs.
impl fmt::Debug for DecryptionRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DecryptionRequest::{}", self.type_str())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DecryptionType {
    Public,
    User,
    /// RFC-016 unified user decryption (`userDecryptionRequest` with a `HandleEntry[]` payload).
    UserV2,
}

impl FromStr for DecryptionType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        // Check `v2` first: "user-v2"/"user_v2" also start with "user".
        if s.contains("v2") {
            Ok(DecryptionType::UserV2)
        } else if s == "u" || s.starts_with("user") {
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
        DecryptionType::UserV2 => s.serialize_str("user_v2"),
    }
}

impl DecryptionRequest {
    pub fn type_str(&self) -> String {
        match self {
            DecryptionRequest::Public(_) => "PublicDecryptionRequest".to_string(),
            DecryptionRequest::User(_) => "UserDecryptionRequest".to_string(),
            DecryptionRequest::UserV2(_) => "UserDecryptionRequestV2".to_string(),
        }
    }
}
