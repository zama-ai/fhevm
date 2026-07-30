use crate::types::ProtocolEventKind;
use alloy::{
    primitives::{B256, U256},
    sol_types::SolEvent,
};
use anyhow::anyhow;
// Handle-only overloaded decryption events
use fhevm_gateway_bindings::decryption::Decryption::{
    PublicDecryptionRequest_1 as PublicDecryptionRequest,
    UserDecryptionRequest_2 as UserDecryptionRequest,
    UserDecryptionRequest_3 as UserDecryptionRequestV2,
};
use fhevm_host_bindings::{
    kms_generation::{
        IKMSGeneration::KeyDigest,
        KMSGeneration::{
            AbortCrsgen, AbortKeygen, CrsgenRequest, KeygenRequest, PrepKeygenRequest,
        },
    },
    protocol_config::ProtocolConfig::{
        KmsContextDestroyed, KmsEpochDestroyed, NewKmsContext, NewKmsEpoch,
    },
};
use sqlx::{PgExecutor, postgres::PgNotification, types::chrono::Utc};
use std::{fmt::Display, str::FromStr};
use tracing::info;

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

/// Enum of all the events monitored by the KMS Connector.
#[derive(sqlx::Type, Copy, Clone, Debug, PartialEq)]
pub enum EventType {
    PublicDecryptionRequest,
    UserDecryptionRequest,
    PrepKeygenRequest,
    KeygenRequest,
    CrsgenRequest,
    AbortKeygenRequest,
    AbortCrsgenRequest,
    NewKmsContext,
    NewKmsEpoch,
    KmsContextDestroyed,
    KmsEpochDestroyed,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::PublicDecryptionRequest => write!(f, "PublicDecryptionRequest"),
            EventType::UserDecryptionRequest => write!(f, "UserDecryptionRequest"),
            EventType::PrepKeygenRequest => write!(f, "PrepKeygenRequest"),
            EventType::KeygenRequest => write!(f, "KeygenRequest"),
            EventType::CrsgenRequest => write!(f, "CrsgenRequest"),
            EventType::AbortKeygenRequest => write!(f, "AbortKeygenRequest"),
            EventType::AbortCrsgenRequest => write!(f, "AbortCrsgenRequest"),
            EventType::NewKmsContext => write!(f, "NewKmsContext"),
            EventType::NewKmsEpoch => write!(f, "NewKmsEpoch"),
            EventType::KmsContextDestroyed => write!(f, "KmsContextDestroyed"),
            EventType::KmsEpochDestroyed => write!(f, "KmsEpochDestroyed"),
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
            ProtocolEventKind::Keygen(_) => Self::KeygenRequest,
            ProtocolEventKind::Crsgen(_) => Self::CrsgenRequest,
            ProtocolEventKind::AbortKeygen(_) => Self::AbortKeygenRequest,
            ProtocolEventKind::AbortCrsgen(_) => Self::AbortCrsgenRequest,
            ProtocolEventKind::NewKmsContext(_) => Self::NewKmsContext,
            ProtocolEventKind::NewKmsEpoch(_) => Self::NewKmsEpoch,
            ProtocolEventKind::KmsContextDestroyed(_) => Self::KmsContextDestroyed,
            ProtocolEventKind::KmsEpochDestroyed(_) => Self::KmsEpochDestroyed,
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
            ABORT_KEYGEN_REQUEST_NOTIFICATION => Ok(Self::AbortKeygenRequest),
            ABORT_CRSGEN_REQUEST_NOTIFICATION => Ok(Self::AbortCrsgenRequest),
            NEW_KMS_CONTEXT_NOTIFICATION => Ok(Self::NewKmsContext),
            NEW_KMS_EPOCH_NOTIFICATION => Ok(Self::NewKmsEpoch),
            KMS_CONTEXT_DESTROYED_NOTIFICATION => Ok(Self::KmsContextDestroyed),
            KMS_EPOCH_DESTROYED_NOTIFICATION => Ok(Self::KmsEpochDestroyed),
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
            Self::AbortKeygenRequest => ABORT_KEYGEN_REQUEST_NOTIFICATION,
            Self::AbortCrsgenRequest => ABORT_CRSGEN_REQUEST_NOTIFICATION,
            Self::NewKmsContext => NEW_KMS_CONTEXT_NOTIFICATION,
            Self::NewKmsEpoch => NEW_KMS_EPOCH_NOTIFICATION,
            Self::KmsContextDestroyed => KMS_CONTEXT_DESTROYED_NOTIFICATION,
            Self::KmsEpochDestroyed => KMS_EPOCH_DESTROYED_NOTIFICATION,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::PublicDecryptionRequest => "public_decryption_request",
            EventType::UserDecryptionRequest => "user_decryption_request",
            EventType::PrepKeygenRequest => "prep_keygen_request",
            EventType::KeygenRequest => "keygen_request",
            EventType::CrsgenRequest => "crsgen_request",
            EventType::AbortKeygenRequest => "abort_keygen_request",
            EventType::AbortCrsgenRequest => "abort_crsgen_request",
            EventType::NewKmsContext => "new_kms_context",
            EventType::NewKmsEpoch => "new_kms_epoch",
            EventType::KmsContextDestroyed => "kms_context_destroyed",
            EventType::KmsEpochDestroyed => "kms_epoch_destroyed",
        }
    }

    pub fn signature_hash(&self) -> B256 {
        match self {
            EventType::PublicDecryptionRequest => PublicDecryptionRequest::SIGNATURE_HASH,
            EventType::UserDecryptionRequest => UserDecryptionRequest::SIGNATURE_HASH,
            EventType::PrepKeygenRequest => PrepKeygenRequest::SIGNATURE_HASH,
            EventType::KeygenRequest => KeygenRequest::SIGNATURE_HASH,
            EventType::CrsgenRequest => CrsgenRequest::SIGNATURE_HASH,
            EventType::AbortKeygenRequest => AbortKeygen::SIGNATURE_HASH,
            EventType::AbortCrsgenRequest => AbortCrsgen::SIGNATURE_HASH,
            EventType::NewKmsContext => NewKmsContext::SIGNATURE_HASH,
            EventType::NewKmsEpoch => NewKmsEpoch::SIGNATURE_HASH,
            EventType::KmsContextDestroyed => KmsContextDestroyed::SIGNATURE_HASH,
            EventType::KmsEpochDestroyed => KmsEpochDestroyed::SIGNATURE_HASH,
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
pub const ABORT_KEYGEN_REQUEST_NOTIFICATION: &str = "abort_keygen_request_available";
pub const ABORT_CRSGEN_REQUEST_NOTIFICATION: &str = "abort_crsgen_request_available";
pub const NEW_KMS_CONTEXT_NOTIFICATION: &str = "new_kms_context_available";
pub const NEW_KMS_EPOCH_NOTIFICATION: &str = "new_kms_epoch_available";
pub const KMS_CONTEXT_DESTROYED_NOTIFICATION: &str = "kms_context_destroyed_available";
pub const KMS_EPOCH_DESTROYED_NOTIFICATION: &str = "kms_epoch_destroyed_available";

#[derive(sqlx::Type, Copy, Clone, Debug, PartialEq)]
#[sqlx(type_name = "operation_status", rename_all = "lowercase")]
pub enum OperationStatus {
    Pending,
    #[sqlx(rename = "under_process")]
    UnderProcess,
    Completed,
    Failed,
    Aborted,
}

impl std::fmt::Display for OperationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Pending => "pending",
            Self::UnderProcess => "under_process",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Aborted => "aborted",
        })
    }
}

/// Marks a destroyed KMS context as invalid in the `kms_context` validation cache.
///
/// The invalidation is upserted so the destruction is recorded even if the context was not cached.
pub async fn invalidate_kms_context<'e>(
    executor: impl PgExecutor<'e>,
    context_id: U256,
) -> anyhow::Result<()> {
    let now = Utc::now();
    sqlx::query!(
        "INSERT INTO kms_context(id, is_valid, created_at, updated_at)
        VALUES ($1, FALSE, $2, $2)
        ON CONFLICT (id) DO UPDATE SET is_valid = FALSE, updated_at = $2",
        context_id.as_le_slice(),
        now,
    )
    .execute(executor)
    .await?;

    info!("KMS context #{context_id} marked as destroyed in DB");
    Ok(())
}

/// Marks a destroyed KMS epoch as invalid in the `kms_epoch` validation cache.
///
/// The invalidation is upserted so the destruction is recorded even if the epoch was never cached;
/// `context_id` stays NULL in that case, as the event does not carry it.
pub async fn invalidate_kms_epoch<'e>(
    executor: impl PgExecutor<'e>,
    epoch_id: U256,
) -> anyhow::Result<()> {
    let now = Utc::now();
    sqlx::query!(
        "INSERT INTO kms_epoch(id, is_valid, created_at, updated_at)
        VALUES ($1, FALSE, $2, $2)
        ON CONFLICT (id) DO UPDATE SET is_valid = FALSE, updated_at = $2",
        epoch_id.as_le_slice(),
        now,
    )
    .execute(executor)
    .await?;

    info!("KMS epoch #{epoch_id} marked as destroyed in DB");
    Ok(())
}
