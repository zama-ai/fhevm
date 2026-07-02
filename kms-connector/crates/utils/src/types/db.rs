use crate::types::ProtocolEventKind;
use alloy::{
    primitives::{Address, B256, U256},
    sol_types::SolEvent,
};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::Decryption::{
    PublicDecryptionRequest, SnsCiphertextMaterial,
    UserDecryptionRequest_0 as UserDecryptionRequest,
    UserDecryptionRequest_1 as UserDecryptionRequestV2,
};
use fhevm_host_bindings::{
    kms_generation::{
        IKMSGeneration::KeyDigest,
        KMSGeneration::{CrsgenRequest, KeygenRequest, PrepKeygenRequest},
    },
    protocol_config::ProtocolConfig::{NewKmsContext, NewKmsEpoch},
};
use sqlx::postgres::PgNotification;
use std::{fmt::Display, str::FromStr};

/// Struct representing how `SnsCiphertextMaterial` are stored in the database.
#[derive(sqlx::Type, Clone, Debug, Default, PartialEq)]
#[sqlx(type_name = "sns_ciphertext_material")]
pub struct SnsCiphertextMaterialDbItem {
    pub ct_handle: [u8; 32],
    pub key_id: [u8; 32],
    pub sns_ciphertext_digest: [u8; 32],
    pub coprocessor_tx_sender_addresses: Vec<[u8; 20]>,
}

impl From<&SnsCiphertextMaterial> for SnsCiphertextMaterialDbItem {
    fn from(value: &SnsCiphertextMaterial) -> Self {
        Self {
            ct_handle: *value.ctHandle,
            key_id: value.keyId.to_le_bytes(),
            sns_ciphertext_digest: *value.snsCiphertextDigest,
            coprocessor_tx_sender_addresses: value
                .coprocessorTxSenderAddresses
                .iter()
                .map(|a| *a.0)
                .collect(),
        }
    }
}

impl From<&SnsCiphertextMaterialDbItem> for SnsCiphertextMaterial {
    fn from(value: &SnsCiphertextMaterialDbItem) -> Self {
        Self {
            ctHandle: value.ct_handle.into(),
            keyId: U256::from_le_bytes(value.key_id),
            snsCiphertextDigest: value.sns_ciphertext_digest.into(),
            coprocessorTxSenderAddresses: value
                .coprocessor_tx_sender_addresses
                .iter()
                .map(Address::from)
                .collect(),
        }
    }
}

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
    /// Reserved by KMS Core's signing enum; no digest uses it yet.
    CompressedPublic,
    /// Digest over a CompressedXofKeySet blob (RFC-029 migration; compressed keygens).
    CompressedKeyset,
}

impl TryFrom<u8> for KeyType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == Self::Server as u8 => Ok(Self::Server),
            v if v == Self::Public as u8 => Ok(Self::Public),
            v if v == Self::CompressedPublic as u8 => Ok(Self::CompressedPublic),
            v if v == Self::CompressedKeyset as u8 => Ok(Self::CompressedKeyset),
            _ => Err(anyhow!("Invalid KeyType value: {value}")),
        }
    }
}

impl FromStr for KeyType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ServerKey" => Ok(Self::Server),
            "PublicKey" => Ok(Self::Public),
            // KMS Core types the digest after the blob it wrote and SIGNS
            // that type inside KeygenVerification — the enum value must
            // round-trip unchanged or on-chain signature verification fails.
            "CompressedXofKeySet" => Ok(Self::CompressedKeyset),
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

/// Enum of all the events monitored by the KMS Connector.
#[derive(sqlx::Type, Copy, Clone, Debug, PartialEq)]
pub enum EventType {
    PublicDecryptionRequest,
    UserDecryptionRequest,
    PrepKeygenRequest,
    KeygenRequest,
    CrsgenRequest,
    NewKmsContext,
    NewKmsEpoch,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::PublicDecryptionRequest => write!(f, "PublicDecryptionRequest"),
            EventType::UserDecryptionRequest => write!(f, "UserDecryptionRequest"),
            EventType::PrepKeygenRequest => write!(f, "PrepKeygenRequest"),
            EventType::KeygenRequest => write!(f, "KeygenRequest"),
            EventType::CrsgenRequest => write!(f, "CrsgenRequest"),
            EventType::NewKmsContext => write!(f, "NewKmsContext"),
            EventType::NewKmsEpoch => write!(f, "NewKmsEpoch"),
        }
    }
}

impl From<&ProtocolEventKind> for EventType {
    fn from(value: &ProtocolEventKind) -> Self {
        match value {
            ProtocolEventKind::PublicDecryption(_) => Self::PublicDecryptionRequest,
            // Both legacy and RFC016 variants share the same `user_decryption_requests` table
            // and the same `UserDecryptionRequest` event type for `last_block_polled` bookkeeping.
            ProtocolEventKind::UserDecryption(_) | ProtocolEventKind::UserDecryptionV2(_) => {
                Self::UserDecryptionRequest
            }
            ProtocolEventKind::PrepKeygen(_) => Self::PrepKeygenRequest,
            // Migration keygen shares the keygen table and notification channel.
            ProtocolEventKind::Keygen(_) | ProtocolEventKind::CompressedKeyMigrationKeygen(_) => {
                Self::KeygenRequest
            }
            ProtocolEventKind::Crsgen(_) => Self::CrsgenRequest,
            ProtocolEventKind::NewKmsContext(_) => Self::NewKmsContext,
            ProtocolEventKind::NewKmsEpoch(_) => Self::NewKmsEpoch,
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
            NEW_KMS_CONTEXT_NOTIFICATION => Ok(Self::NewKmsContext),
            NEW_KMS_EPOCH_NOTIFICATION => Ok(Self::NewKmsEpoch),
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
            Self::NewKmsContext => NEW_KMS_CONTEXT_NOTIFICATION,
            Self::NewKmsEpoch => NEW_KMS_EPOCH_NOTIFICATION,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::PublicDecryptionRequest => "public_decryption_request",
            EventType::UserDecryptionRequest => "user_decryption_request",
            EventType::PrepKeygenRequest => "prep_keygen_request",
            EventType::KeygenRequest => "keygen_request",
            EventType::CrsgenRequest => "crsgen_request",
            EventType::NewKmsContext => "new_kms_context",
            EventType::NewKmsEpoch => "new_kms_epoch",
        }
    }

    pub fn signature_hash(&self) -> B256 {
        match self {
            EventType::PublicDecryptionRequest => PublicDecryptionRequest::SIGNATURE_HASH,
            EventType::UserDecryptionRequest => UserDecryptionRequest::SIGNATURE_HASH,
            EventType::PrepKeygenRequest => PrepKeygenRequest::SIGNATURE_HASH,
            EventType::KeygenRequest => KeygenRequest::SIGNATURE_HASH,
            EventType::CrsgenRequest => CrsgenRequest::SIGNATURE_HASH,
            EventType::NewKmsContext => NewKmsContext::SIGNATURE_HASH,
            EventType::NewKmsEpoch => NewKmsEpoch::SIGNATURE_HASH,
        }
    }

    /// Returns every topic0 hash that maps to this `EventType` in the Gateway ABI.
    ///
    /// `UserDecryptionRequest` currently covers two overloaded events — the legacy shape and the
    /// RFC016 shape — so both topic0 hashes must be listed in the `eth_getLogs` filter. All other
    /// event types map one-to-one to a single topic0.
    pub fn signature_hashes(&self) -> Vec<B256> {
        match self {
            EventType::UserDecryptionRequest => vec![
                UserDecryptionRequest::SIGNATURE_HASH,
                UserDecryptionRequestV2::SIGNATURE_HASH,
            ],
            _ => vec![self.signature_hash()],
        }
    }
}

// Postgres notifications
pub const PUBLIC_DECRYPT_REQUEST_NOTIFICATION: &str = "public_decryption_request_available";
pub const USER_DECRYPT_REQUEST_NOTIFICATION: &str = "user_decryption_request_available";
pub const PREP_KEYGEN_REQUEST_NOTIFICATION: &str = "prep_keygen_request_available";
pub const KEYGEN_REQUEST_NOTIFICATION: &str = "keygen_request_available";
pub const CRSGEN_REQUEST_NOTIFICATION: &str = "crsgen_request_available";
pub const NEW_KMS_CONTEXT_NOTIFICATION: &str = "new_kms_context_available";
pub const NEW_KMS_EPOCH_NOTIFICATION: &str = "new_kms_epoch_available";

#[derive(sqlx::Type, Copy, Clone, Debug, PartialEq)]
#[sqlx(type_name = "operation_status", rename_all = "lowercase")]
pub enum OperationStatus {
    Pending,
    #[sqlx(rename = "under_process")]
    UnderProcess,
    Completed,
    Failed,
}
