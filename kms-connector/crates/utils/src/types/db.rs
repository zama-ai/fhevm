use crate::types::GatewayEventKind;
use anyhow::anyhow;
use fhevm_gateway_bindings::kms_generation::IKMSGeneration::KeyDigest;
use sqlx::postgres::PgNotification;
use std::{fmt::Display, str::FromStr};

/// Struct representing the `ParamsType` enum in the database.
#[derive(sqlx::Type, Copy, Clone, Debug, PartialEq)]
#[sqlx(type_name = "params_type")]
pub enum ParamsTypeDb {
    Default,
    Test,
}

impl TryFrom<u8> for ParamsTypeDb {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == Self::Default as u8 {
            Ok(Self::Default)
        } else if value == Self::Test as u8 {
            Ok(Self::Test)
        } else {
            Err(anyhow!("Invalid ParamsType value: {value}"))
        }
    }
}

/// Struct representing the `KeyType` enum in the database.
#[derive(sqlx::Type, Copy, Clone, Debug, Default, PartialEq)]
#[sqlx(type_name = "key_type")]
pub enum KeyType {
    Server,
    #[default]
    Public,
}

impl TryFrom<u8> for KeyType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == Self::Server as u8 {
            Ok(Self::Server)
        } else if value == Self::Public as u8 {
            Ok(Self::Public)
        } else {
            Err(anyhow!("Invalid KeyType value: {value}"))
        }
    }
}

impl FromStr for KeyType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ServerKey" => Ok(Self::Server),
            "PublicKey" => Ok(Self::Public),
            _ => Err(anyhow!("Invalid KeyType value: {s}")),
        }
    }
}

/// Struct representing how `KeyDigest` are stored in the database.
#[derive(sqlx::Type, Clone, Debug, Default, PartialEq)]
#[sqlx(type_name = "key_digest")]
pub struct KeyDigestDbItem {
    pub key_type: KeyType,
    pub digest: Vec<u8>,
}

impl From<KeyDigestDbItem> for KeyDigest {
    fn from(value: KeyDigestDbItem) -> Self {
        Self {
            keyType: value.key_type as u8,
            digest: value.digest.into(),
        }
    }
}

/// Struct representing the `EventType` enum in the database.
#[derive(sqlx::Type, Copy, Clone, Debug, PartialEq)]
#[sqlx(type_name = "event_type")]
pub enum EventType {
    PublicDecryptionRequest,
    UserDecryptionRequest,
    PrepKeygenRequest,
    KeygenRequest,
    CrsgenRequest,
    PrssInit,
    KeyReshareSameSet,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::PublicDecryptionRequest => write!(f, "PublicDecryptionRequest"),
            EventType::UserDecryptionRequest => write!(f, "UserDecryptionRequest"),
            EventType::PrepKeygenRequest => write!(f, "PrepKeygenRequest"),
            EventType::KeygenRequest => write!(f, "KeygenRequest"),
            EventType::CrsgenRequest => write!(f, "CrsgenRequest"),
            EventType::PrssInit => write!(f, "PrssInit"),
            EventType::KeyReshareSameSet => write!(f, "KeyReshareSameSet"),
        }
    }
}

impl From<&GatewayEventKind> for EventType {
    fn from(value: &GatewayEventKind) -> Self {
        match value {
            GatewayEventKind::PublicDecryption(_) => Self::PublicDecryptionRequest,
            GatewayEventKind::UserDecryption(_) => Self::UserDecryptionRequest,
            GatewayEventKind::PrepKeygen(_) => Self::PrepKeygenRequest,
            GatewayEventKind::Keygen(_) => Self::KeygenRequest,
            GatewayEventKind::Crsgen(_) => Self::CrsgenRequest,
            GatewayEventKind::PrssInit(_) => Self::PrssInit,
            GatewayEventKind::KeyReshareSameSet(_) => Self::KeyReshareSameSet,
        }
    }
}

impl TryFrom<PgNotification> for EventType {
    type Error = anyhow::Error;

    fn try_from(value: PgNotification) -> Result<Self, Self::Error> {
        match value.channel() {
            PUBLIC_DECRYPT_REQUEST_NOTIFICATION => Ok(Self::PublicDecryptionRequest),
            USER_DECRYPT_REQUEST_NOTIFICATION => Ok(Self::UserDecryptionRequest),
            PREP_KEYGEN_REQUEST_NOTIFICATION => Ok(Self::PrepKeygenRequest),
            KEYGEN_REQUEST_NOTIFICATION => Ok(Self::KeygenRequest),
            CRSGEN_REQUEST_NOTIFICATION => Ok(Self::CrsgenRequest),
            PRSS_INIT_NOTIFICATION => Ok(Self::PrssInit),
            KEY_RESHARE_SAME_SET_NOTIFICATION => Ok(Self::KeyReshareSameSet),
            s => Err(anyhow!("Unknown notification channel: {s}")),
        }
    }
}

impl EventType {
    pub fn pg_notification(&self) -> &'static str {
        match self {
            Self::PublicDecryptionRequest => PUBLIC_DECRYPT_REQUEST_NOTIFICATION,
            Self::UserDecryptionRequest => USER_DECRYPT_REQUEST_NOTIFICATION,
            Self::PrepKeygenRequest => PREP_KEYGEN_REQUEST_NOTIFICATION,
            Self::KeygenRequest => KEYGEN_REQUEST_NOTIFICATION,
            Self::CrsgenRequest => CRSGEN_REQUEST_NOTIFICATION,
            Self::PrssInit => PRSS_INIT_NOTIFICATION,
            Self::KeyReshareSameSet => KEY_RESHARE_SAME_SET_NOTIFICATION,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::PublicDecryptionRequest => "public_decryption_request",
            EventType::UserDecryptionRequest => "user_decryption_request",
            EventType::PrepKeygenRequest => "prep_keygen_request",
            EventType::KeygenRequest => "keygen_request",
            EventType::CrsgenRequest => "crsgen_request",
            EventType::PrssInit => "prss_init",
            EventType::KeyReshareSameSet => "key_reshare_same_set",
        }
    }
}

// Postgres notifications
pub const PUBLIC_DECRYPT_REQUEST_NOTIFICATION: &str = "public_decryption_request_available";
pub const USER_DECRYPT_REQUEST_NOTIFICATION: &str = "user_decryption_request_available";
pub const PREP_KEYGEN_REQUEST_NOTIFICATION: &str = "prep_keygen_request_available";
pub const KEYGEN_REQUEST_NOTIFICATION: &str = "keygen_request_available";
pub const CRSGEN_REQUEST_NOTIFICATION: &str = "crsgen_request_available";
pub const PRSS_INIT_NOTIFICATION: &str = "prss_init_available";
pub const KEY_RESHARE_SAME_SET_NOTIFICATION: &str = "key_reshare_same_set_available";

#[derive(sqlx::Type, Copy, Clone, Debug, PartialEq)]
#[sqlx(type_name = "operation_status", rename_all = "lowercase")]
pub enum OperationStatus {
    Pending,
    #[sqlx(rename = "under_process")]
    UnderProcess,
    Completed,
    Failed,
}
