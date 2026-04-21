use crate::{
    monitoring::otlp::PropagationContext,
    types::db::{OperationStatus, ParamsTypeDb, SnsCiphertextMaterialDbItem},
};
use alloy::primitives::{Address, FixedBytes, U256};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::{
    Decryption::{
        DecryptionEvents, HandleEntry, PublicDecryptionRequest, SnsCiphertextMaterial,
        UserDecryptionRequest_0 as UserDecryptionRequest,
        UserDecryptionRequest_1 as UserDecryptionRequestV2,
    },
    IDecryption::{RequestValiditySeconds, UserDecryptionRequestPayload},
};
use fhevm_host_bindings::kms_generation::KMSGeneration::{
    CrsgenRequest, KMSGenerationEvents, KeygenRequest, PrepKeygenRequest,
};
use sqlx::{
    Pool, Postgres, Row,
    postgres::{PgArguments, PgRow},
    query::Query,
    types::chrono::{DateTime, Utc},
};
use std::fmt::Display;
use tracing::{info, warn};

/// The events emitted by the Zama Protocol which are monitored by the KMS Connector.
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolEvent {
    pub kind: ProtocolEventKind,
    pub tx_hash: Option<FixedBytes<32>>,
    pub already_sent: bool,
    pub error_counter: i16,
    pub created_at: DateTime<Utc>,
    pub otlp_context: PropagationContext,
}

impl ProtocolEvent {
    pub fn new(
        kind: ProtocolEventKind,
        tx_hash: Option<FixedBytes<32>>,
        otlp_context: PropagationContext,
    ) -> Self {
        ProtocolEvent {
            kind,
            tx_hash,
            already_sent: false,
            error_counter: 0,
            created_at: Utc::now(),
            otlp_context,
        }
    }

    /// Sets the event's `status` field to `pending` in the database.
    pub async fn mark_as_pending(&self, db: &Pool<Postgres>) {
        warn!("Failed to process event. Restoring `status` field to `pending` in DB...");
        self.update_status(db, OperationStatus::Pending).await
    }

    /// Sets the event's `status` field to `completed` in the database.
    pub async fn mark_as_completed(&self, db: &Pool<Postgres>) {
        info!("Event successfully processed. Setting its `status` field to `completed` in DB...");
        self.update_status(db, OperationStatus::Completed).await
    }

    /// Sets the event's `status` field to `failed` in the database.
    pub async fn mark_as_failed(&self, db: &Pool<Postgres>) {
        warn!("Failed to process event. Restoring `status` field to `failed` in DB...");
        self.update_status(db, OperationStatus::Failed).await
    }

    async fn update_status(&self, db: &Pool<Postgres>, status: OperationStatus) {
        let already_sent = self.already_sent;
        let err_count = self.error_counter;
        match &self.kind {
            ProtocolEventKind::PublicDecryption(e) => {
                update_public_decryption_status(db, e.decryptionId, status, already_sent, err_count)
                    .await
            }
            ProtocolEventKind::UserDecryption(e) => {
                update_user_decryption_status(db, e.decryptionId, status, already_sent, err_count)
                    .await
            }
            ProtocolEventKind::UserDecryptionV2(e) => {
                update_user_decryption_status(db, e.decryptionId, status, already_sent, err_count)
                    .await
            }
            ProtocolEventKind::PrepKeygen(e) => {
                update_prep_keygen_status(db, e.prepKeygenId, status, already_sent).await
            }
            ProtocolEventKind::Keygen(e) => {
                update_keygen_status(db, e.keyId, status, already_sent).await
            }
            ProtocolEventKind::Crsgen(e) => {
                update_crsgen_status(db, e.crsId, status, already_sent).await
            }
        }
    }
}

// `Debug` and `PartialEq` are implemented by hand below because
// `fhevm_gateway_bindings::decryption::Decryption::UserDecryptionRequest_1` (aliased as
// `UserDecryptionRequestV2`) doesn't derive them: `alloy::sol!` skips the automatic derives on any
// struct that references a type defined in a different `sol!` module, and this event's `payload`
// field reaches into `IDecryption::UserDecryptionRequestPayload`. All of the event's fields
// individually do implement `Debug` and `PartialEq`, so we forward to them field-by-field.
#[derive(Clone)]
pub enum ProtocolEventKind {
    PublicDecryption(PublicDecryptionRequest),
    /// Legacy `UserDecryptionRequest` event (split direct / delegated at the calldata level).
    UserDecryption(UserDecryptionRequest),
    /// RFC016 `UserDecryptionRequest` event — carries the full unified payload (handles, signed
    /// fields, signature) directly in the event, so processing does not need to re-fetch calldata.
    UserDecryptionV2(UserDecryptionRequestV2),
    PrepKeygen(PrepKeygenRequest),
    Keygen(KeygenRequest),
    Crsgen(CrsgenRequest),
}

impl std::fmt::Debug for ProtocolEventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PublicDecryption(e) => f.debug_tuple("PublicDecryption").field(e).finish(),
            Self::UserDecryption(e) => f.debug_tuple("UserDecryption").field(e).finish(),
            Self::UserDecryptionV2(e) => f
                .debug_struct("UserDecryptionV2")
                .field("decryptionId", &e.decryptionId)
                .field("snsCtMaterials", &e.snsCtMaterials)
                .field("handles", &e.handles)
                .field("payload", &e.payload)
                .finish(),
            Self::PrepKeygen(e) => f.debug_tuple("PrepKeygen").field(e).finish(),
            Self::Keygen(e) => f.debug_tuple("Keygen").field(e).finish(),
            Self::Crsgen(e) => f.debug_tuple("Crsgen").field(e).finish(),
        }
    }
}

impl PartialEq for ProtocolEventKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::PublicDecryption(a), Self::PublicDecryption(b)) => a == b,
            (Self::UserDecryption(a), Self::UserDecryption(b)) => a == b,
            (Self::UserDecryptionV2(a), Self::UserDecryptionV2(b)) => {
                a.decryptionId == b.decryptionId
                    && a.snsCtMaterials == b.snsCtMaterials
                    && a.handles == b.handles
                    && a.payload == b.payload
            }
            (Self::PrepKeygen(a), Self::PrepKeygen(b)) => a == b,
            (Self::Keygen(a), Self::Keygen(b)) => a == b,
            (Self::Crsgen(a), Self::Crsgen(b)) => a == b,
            _ => false,
        }
    }
}

pub fn from_public_decryption_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    let sns_ct_materials = row
        .try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?
        .iter()
        .map(SnsCiphertextMaterial::from)
        .collect();

    let kind = ProtocolEventKind::PublicDecryption(PublicDecryptionRequest {
        decryptionId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
        snsCtMaterials: sns_ct_materials,
        extraData: row.try_get::<Vec<u8>, _>("extra_data")?.into(),
    });
    Ok(ProtocolEvent {
        kind,
        tx_hash: row
            .try_get::<Vec<u8>, _>("tx_hash")
            .ok()
            .and_then(|h| FixedBytes::try_from(h.as_slice()).ok()),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: row.try_get::<i16, _>("error_counter")?,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_user_decryption_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    let sns_ct_materials = row
        .try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?
        .iter()
        .map(SnsCiphertextMaterial::from)
        .collect();
    let decryption_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?);
    let user_address: Address = row.try_get::<[u8; 20], _>("user_address")?.into();
    let public_key: Vec<u8> = row.try_get("public_key")?;
    let extra_data: Vec<u8> = row.try_get("extra_data")?;

    // `signature IS NULL` is the sole variant discriminator for `user_decryption_requests` rows.
    // See migration 20260421092426_unified_user_decryption.sql. `try_get(...).ok().flatten()`
    // tolerates both missing column (older SELECT queries) and NULL value → both map to the legacy
    // variant.
    let signature = row
        .try_get::<Option<Vec<u8>>, _>("signature")
        .ok()
        .flatten();

    let kind = match signature {
        None => ProtocolEventKind::UserDecryption(UserDecryptionRequest {
            decryptionId: decryption_id,
            snsCtMaterials: sns_ct_materials,
            userAddress: user_address,
            publicKey: public_key.into(),
            extraData: extra_data.into(),
        }),
        Some(signature) => {
            let owner_addresses: Vec<Vec<u8>> = row.try_get("handle_owner_addresses")?;
            let contract_addresses: Vec<Vec<u8>> = row.try_get("handle_contract_addresses")?;
            let allowed_contracts: Vec<Vec<u8>> = row.try_get("allowed_contracts")?;
            let start_timestamp: i64 = row.try_get("start_timestamp")?;
            let duration_seconds: i64 = row.try_get("duration_seconds")?;

            if owner_addresses.len() != sns_ct_materials.len()
                || contract_addresses.len() != sns_ct_materials.len()
            {
                anyhow::bail!(
                    "handle owner/contract array length mismatch for RFC016 user decryption row"
                );
            }

            let handles = sns_ct_materials
                .iter()
                .zip(owner_addresses.iter())
                .zip(contract_addresses.iter())
                .map(|((m, owner), contract)| {
                    Ok::<_, anyhow::Error>(HandleEntry {
                        handle: m.ctHandle,
                        contractAddress: Address::try_from(contract.as_slice())?,
                        ownerAddress: Address::try_from(owner.as_slice())?,
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?;

            let allowed_contracts = allowed_contracts
                .iter()
                .map(|a| Address::try_from(a.as_slice()).map_err(anyhow::Error::from))
                .collect::<anyhow::Result<Vec<_>>>()?;

            ProtocolEventKind::UserDecryptionV2(UserDecryptionRequestV2 {
                decryptionId: decryption_id,
                snsCtMaterials: sns_ct_materials,
                handles,
                payload: UserDecryptionRequestPayload {
                    userAddress: user_address,
                    publicKey: public_key.into(),
                    allowedContracts: allowed_contracts,
                    requestValidity: RequestValiditySeconds {
                        startTimestamp: U256::from(start_timestamp as u64),
                        durationSeconds: U256::from(duration_seconds as u64),
                    },
                    extraData: extra_data.into(),
                    signature: signature.into(),
                },
            })
        }
    };

    Ok(ProtocolEvent {
        kind,
        tx_hash: row
            .try_get::<Vec<u8>, _>("tx_hash")
            .ok()
            .and_then(|h| FixedBytes::try_from(h.as_slice()).ok()),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: row.try_get::<i16, _>("error_counter")?,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_prep_keygen_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    let kind = ProtocolEventKind::PrepKeygen(PrepKeygenRequest {
        prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
        paramsType: row.try_get::<ParamsTypeDb, _>("params_type")? as u8,
        extraData: row.try_get::<Vec<u8>, _>("extra_data")?.into(),
    });
    Ok(ProtocolEvent {
        kind,

        tx_hash: row
            .try_get::<Vec<u8>, _>("tx_hash")
            .ok()
            .and_then(|h| FixedBytes::try_from(h.as_slice()).ok()),

        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_keygen_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    let kind = ProtocolEventKind::Keygen(KeygenRequest {
        prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
        keyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?),
        extraData: row.try_get::<Vec<u8>, _>("extra_data")?.into(),
    });
    Ok(ProtocolEvent {
        kind,

        tx_hash: row
            .try_get::<Vec<u8>, _>("tx_hash")
            .ok()
            .and_then(|h| FixedBytes::try_from(h.as_slice()).ok()),

        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_crsgen_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    let kind = ProtocolEventKind::Crsgen(CrsgenRequest {
        crsId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("crs_id")?),
        maxBitLength: U256::from_le_bytes(row.try_get::<[u8; 32], _>("max_bit_length")?),
        paramsType: row.try_get::<ParamsTypeDb, _>("params_type")? as u8,
        extraData: row.try_get::<Vec<u8>, _>("extra_data")?.into(),
    });
    Ok(ProtocolEvent {
        kind,

        tx_hash: row
            .try_get::<Vec<u8>, _>("tx_hash")
            .ok()
            .and_then(|h| FixedBytes::try_from(h.as_slice()).ok()),

        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

async fn update_public_decryption_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
    error_counter: i16,
) {
    let query = sqlx::query!(
        "UPDATE public_decryption_requests SET status = $1, already_sent = $2, error_counter = $3 \
        WHERE decryption_id = $4",
        status as OperationStatus,
        already_sent,
        error_counter,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await;
}

async fn update_user_decryption_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
    error_counter: i16,
) {
    let query = sqlx::query!(
        "UPDATE user_decryption_requests SET status = $1, already_sent = $2, error_counter = $3 \
        WHERE decryption_id = $4",
        status as OperationStatus,
        already_sent,
        error_counter,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await;
}

async fn update_prep_keygen_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
) {
    let query = sqlx::query!(
        "UPDATE prep_keygen_requests SET status = $1, already_sent = $2 \
        WHERE prep_keygen_id = $3",
        status as OperationStatus,
        already_sent,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await;
}

async fn update_keygen_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
) {
    let query = sqlx::query!(
        "UPDATE keygen_requests SET status = $1, already_sent = $2 \
        WHERE key_id = $3",
        status as OperationStatus,
        already_sent,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await;
}

async fn update_crsgen_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
) {
    let query = sqlx::query!(
        "UPDATE crsgen_requests SET status = $1, already_sent = $2 \
        WHERE crs_id = $3",
        status as OperationStatus,
        already_sent,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await;
}

async fn execute_update_event_query(db: &Pool<Postgres>, query: Query<'_, Postgres, PgArguments>) {
    let query_result = match query.execute(db).await {
        Ok(result) => result,
        Err(e) => return warn!("Failed to update event: {e}"),
    };

    if query_result.rows_affected() == 1 {
        info!("Successfully updated event in DB!");
    } else {
        warn!(
            "Unexpected query result while updating event: {:?}",
            query_result
        )
    }
}

impl Display for ProtocolEventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolEventKind::PublicDecryption(e) => {
                write!(f, "PublicDecryptionRequest #{}", e.decryptionId)
            }
            ProtocolEventKind::UserDecryption(e) => {
                write!(f, "UserDecryptionRequest #{}", e.decryptionId)
            }
            ProtocolEventKind::UserDecryptionV2(e) => {
                write!(f, "UserDecryptionRequest #{}", e.decryptionId)
            }
            ProtocolEventKind::PrepKeygen(e) => {
                write!(f, "PrepKeygenRequest #{}", e.prepKeygenId)
            }
            ProtocolEventKind::Keygen(e) => write!(f, "KeygenRequest #{}", e.keyId),
            ProtocolEventKind::Crsgen(e) => write!(f, "CrsgenRequest #{}", e.crsId),
        }
    }
}

impl From<PublicDecryptionRequest> for ProtocolEventKind {
    fn from(value: PublicDecryptionRequest) -> Self {
        Self::PublicDecryption(value)
    }
}

impl From<UserDecryptionRequest> for ProtocolEventKind {
    fn from(value: UserDecryptionRequest) -> Self {
        Self::UserDecryption(value)
    }
}

impl From<UserDecryptionRequestV2> for ProtocolEventKind {
    fn from(value: UserDecryptionRequestV2) -> Self {
        Self::UserDecryptionV2(value)
    }
}

impl From<PrepKeygenRequest> for ProtocolEventKind {
    fn from(value: PrepKeygenRequest) -> Self {
        Self::PrepKeygen(value)
    }
}

impl From<KeygenRequest> for ProtocolEventKind {
    fn from(value: KeygenRequest) -> Self {
        Self::Keygen(value)
    }
}

impl From<CrsgenRequest> for ProtocolEventKind {
    fn from(value: CrsgenRequest) -> Self {
        Self::Crsgen(value)
    }
}

impl TryFrom<DecryptionEvents> for ProtocolEventKind {
    type Error = anyhow::Error;

    fn try_from(value: DecryptionEvents) -> Result<Self, Self::Error> {
        match value {
            // `UserDecryptionRequest_0` is the legacy event; `UserDecryptionRequest_1` is the
            // RFC016 overload.
            DecryptionEvents::PublicDecryptionRequest(e) => Ok(e.into()),
            DecryptionEvents::UserDecryptionRequest_0(e) => Ok(e.into()),
            DecryptionEvents::UserDecryptionRequest_1(e) => Ok(e.into()),
            _ => Err(anyhow!("Unexpected Decryption event")),
        }
    }
}

impl TryFrom<KMSGenerationEvents> for ProtocolEventKind {
    type Error = anyhow::Error;

    fn try_from(value: KMSGenerationEvents) -> Result<Self, Self::Error> {
        match value {
            KMSGenerationEvents::PrepKeygenRequest(e) => Ok(e.into()),
            KMSGenerationEvents::KeygenRequest(e) => Ok(e.into()),
            KMSGenerationEvents::CrsgenRequest(e) => Ok(e.into()),
            // `KMSGenerationEvents` does not currently implement `Debug` unfortunately
            _ => Err(anyhow!("Unexpected KMSGeneration event")),
        }
    }
}
