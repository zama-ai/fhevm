use crate::{
    monitoring::otlp::PropagationContext,
    types::db::{OperationStatus, ParamsTypeDb, SnsCiphertextMaterialDbItem},
};
use alloy::{
    primitives::{Address, FixedBytes, U256},
    sol_types::SolValue,
};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::{
    Decryption::{
        DecryptionEvents, HandleEntry, PublicDecryptionRequest, SnsCiphertextMaterial,
        UserDecryptionRequest_0 as UserDecryptionRequest,
        UserDecryptionRequest_1 as UserDecryptionRequestV2,
    },
    IDecryption::{RequestValiditySeconds, UserDecryptionRequestPayload},
};
use fhevm_host_bindings::{
    kms_generation::KMSGeneration::{
        AbortCrsgen, AbortKeygen, CrsgenRequest, KMSGenerationEvents, KeygenRequest,
        PrepKeygenRequest,
    },
    protocol_config::{
        IProtocolConfig::KmsThresholds,
        ProtocolConfig::{
            KmsNodeParams, NewKmsContext, NewKmsEpoch, PcrValues, ProtocolConfigEvents,
        },
    },
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
    pub async fn mark_as_pending(&self, db: &Pool<Postgres>) -> anyhow::Result<()> {
        warn!("Failed to process event. Restoring `status` field to `pending` in DB...");
        self.update_status(db, OperationStatus::Pending).await
    }

    /// Sets the event's `status` field to `completed` in the database.
    pub async fn mark_as_completed(&self, db: &Pool<Postgres>) -> anyhow::Result<()> {
        info!("Event successfully processed. Setting its `status` field to `completed` in DB...");
        self.update_status(db, OperationStatus::Completed).await
    }

    /// Sets the event's `status` field to `failed` in the database.
    pub async fn mark_as_failed(&self, db: &Pool<Postgres>) -> anyhow::Result<()> {
        warn!("Failed to process event. Restoring `status` field to `failed` in DB...");
        self.update_status(db, OperationStatus::Failed).await
    }

    /// Sets the event's `status` field to `aborted` in the database.
    pub async fn mark_as_aborted(&self, db: &Pool<Postgres>) -> anyhow::Result<()> {
        info!(
            "Event was aborted on the KMS Core. Setting its `status` field to `aborted` in DB..."
        );
        self.update_status(db, OperationStatus::Aborted).await
    }

    async fn update_status(
        &self,
        db: &Pool<Postgres>,
        status: OperationStatus,
    ) -> anyhow::Result<()> {
        let already_sent = self.already_sent;
        let err_count = self.error_counter;
        let result = match &self.kind {
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
                update_keygen_status(db, e.requestId, status, already_sent).await
            }
            ProtocolEventKind::Crsgen(e) => {
                update_crsgen_status(db, e.crsId, status, already_sent).await
            }
            ProtocolEventKind::AbortKeygen(e) => {
                update_abort_keygen_status(db, e.prepKeygenId, status, already_sent).await
            }
            ProtocolEventKind::AbortCrsgen(e) => {
                update_abort_crsgen_status(db, e.crsId, status, already_sent).await
            }
            ProtocolEventKind::NewKmsContext(e) => {
                update_new_kms_context_status(db, e.contextId, status, already_sent).await
            }
            ProtocolEventKind::NewKmsEpoch(e) => {
                update_new_kms_epoch_status(db, e.epochId, status, already_sent).await
            }
        };
        result.map_err(|e| {
            anyhow!(
                "Failed to set {} status to `{status}` in DB: {e:#}",
                self.kind
            )
        })
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
    AbortKeygen(AbortKeygen),
    AbortCrsgen(AbortCrsgen),
    NewKmsContext(NewKmsContext),
    NewKmsEpoch(NewKmsEpoch),
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
            Self::AbortKeygen(e) => f.debug_tuple("AbortKeygen").field(e).finish(),
            Self::AbortCrsgen(e) => f.debug_tuple("AbortCrsgen").field(e).finish(),
            Self::NewKmsContext(e) => f.debug_tuple("NewKmsContext").field(e).finish(),
            Self::NewKmsEpoch(e) => f
                .debug_struct("NewKmsEpoch")
                .field("kmsContextId", &e.kmsContextId)
                .field("epochId", &e.epochId)
                .field("previousContextId", &e.previousContextId)
                .field("previousEpochId", &e.previousEpochId)
                .field("materialBlockNumber", &e.materialBlockNumber)
                .finish(),
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
            (Self::AbortKeygen(a), Self::AbortKeygen(b)) => a == b,
            (Self::AbortCrsgen(a), Self::AbortCrsgen(b)) => a == b,
            (Self::NewKmsContext(a), Self::NewKmsContext(b)) => a == b,
            (Self::NewKmsEpoch(a), Self::NewKmsEpoch(b)) => {
                a.kmsContextId == b.kmsContextId
                    && a.epochId == b.epochId
                    && a.previousContextId == b.previousContextId
                    && a.previousEpochId == b.previousEpochId
                    && a.materialBlockNumber == b.materialBlockNumber
            }
            _ => false,
        }
    }
}

fn tx_hash_from_row(row: &PgRow) -> Option<FixedBytes<32>> {
    row.try_get::<Vec<u8>, _>("tx_hash")
        .ok()
        .and_then(|h| FixedBytes::try_from(h.as_slice()).ok())
}

fn otlp_context_from_row(row: &PgRow) -> anyhow::Result<PropagationContext> {
    Ok(bc2wrap::deserialize_slice(
        &row.try_get::<Vec<u8>, _>("otlp_context")?,
    )?)
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
        tx_hash: tx_hash_from_row(row),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: row.try_get::<i16, _>("error_counter")?,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: otlp_context_from_row(row)?,
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
    let signature: Option<Vec<u8>> = match row.try_get::<Option<Vec<u8>>, _>("signature") {
        Ok(v) => v,
        Err(sqlx::Error::ColumnNotFound(_)) => None,
        Err(e) => return Err(anyhow::Error::from(e)),
    };

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
                    "handle owner/contract array length mismatch for user decryption row"
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
                        startTimestamp: U256::try_from(start_timestamp).map_err(|_| {
                            anyhow!("start_timestamp is negative: {start_timestamp}")
                        })?,
                        durationSeconds: U256::try_from(duration_seconds).map_err(|_| {
                            anyhow!("duration_seconds is negative: {duration_seconds}")
                        })?,
                    },
                    extraData: extra_data.into(),
                    signature: signature.into(),
                },
            })
        }
    };

    Ok(ProtocolEvent {
        kind,
        tx_hash: tx_hash_from_row(row),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: row.try_get::<i16, _>("error_counter")?,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: otlp_context_from_row(row)?,
    })
}

pub fn from_prep_keygen_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    let kind = ProtocolEventKind::PrepKeygen(PrepKeygenRequest {
        prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
        paramsType: row.try_get::<ParamsTypeDb, _>("params_type")? as u8,
        requestKind: 0,
        keyId: U256::ZERO,
        extraData: row.try_get::<Vec<u8>, _>("extra_data")?.into(),
    });
    Ok(ProtocolEvent {
        kind,
        tx_hash: tx_hash_from_row(row),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: otlp_context_from_row(row)?,
    })
}

pub fn from_keygen_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    let request_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?);
    let migration_key_id = row.try_get::<Option<[u8; 32]>, _>("migration_key_id")?;
    let kind = ProtocolEventKind::Keygen(KeygenRequest {
        prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
        requestId: request_id,
        requestKind: if migration_key_id.is_some() { 1 } else { 0 },
        keyId: migration_key_id
            .map(U256::from_le_bytes)
            .unwrap_or(request_id),
        extraData: row.try_get::<Vec<u8>, _>("extra_data")?.into(),
    });
    Ok(ProtocolEvent {
        kind,
        tx_hash: tx_hash_from_row(row),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: otlp_context_from_row(row)?,
    })
}

pub fn from_new_kms_context_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    // Nested arrays are stored as ABI-encoded BYTEA by the gw-listener (see
    // `publish_new_kms_context`); decoding here returns lossless `Vec<KmsNodeParams>` etc.
    let kms_node_params: Vec<KmsNodeParams> =
        SolValue::abi_decode(&row.try_get::<Vec<u8>, _>("kms_node_params")?)?;
    let thresholds: KmsThresholds =
        SolValue::abi_decode(&row.try_get::<Vec<u8>, _>("thresholds")?)?;
    let pcr_values: Vec<PcrValues> =
        SolValue::abi_decode(&row.try_get::<Vec<u8>, _>("pcr_values")?)?;

    let kind = ProtocolEventKind::NewKmsContext(NewKmsContext {
        contextId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("context_id")?),
        previousContextId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("previous_context_id")?),
        kmsNodeParams: kms_node_params,
        thresholds,
        softwareVersion: row.try_get::<String, _>("software_version")?,
        pcrValues: pcr_values,
    });
    Ok(ProtocolEvent {
        kind,
        tx_hash: tx_hash_from_row(row),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: otlp_context_from_row(row)?,
    })
}

pub fn from_new_kms_epoch_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    let kind = ProtocolEventKind::NewKmsEpoch(NewKmsEpoch {
        kmsContextId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("context_id")?),
        epochId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("epoch_id")?),
        previousContextId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("previous_context_id")?),
        previousEpochId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("previous_epoch_id")?),
        materialBlockNumber: U256::from_le_bytes(
            row.try_get::<[u8; 32], _>("material_block_number")?,
        ),
    });
    Ok(ProtocolEvent {
        kind,
        tx_hash: tx_hash_from_row(row),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: otlp_context_from_row(row)?,
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
        tx_hash: tx_hash_from_row(row),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: otlp_context_from_row(row)?,
    })
}

pub fn from_abort_keygen_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    let kind = ProtocolEventKind::AbortKeygen(AbortKeygen {
        prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
    });
    Ok(ProtocolEvent {
        kind,
        tx_hash: tx_hash_from_row(row),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: otlp_context_from_row(row)?,
    })
}

pub fn from_abort_crsgen_row(row: &PgRow) -> anyhow::Result<ProtocolEvent> {
    let kind = ProtocolEventKind::AbortCrsgen(AbortCrsgen {
        crsId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("crs_id")?),
    });
    Ok(ProtocolEvent {
        kind,
        tx_hash: tx_hash_from_row(row),
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        otlp_context: otlp_context_from_row(row)?,
    })
}

async fn update_public_decryption_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
    error_counter: i16,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        "UPDATE public_decryption_requests SET status = $1, already_sent = $2, error_counter = $3
        WHERE decryption_id = $4",
        status as OperationStatus,
        already_sent,
        error_counter,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await
}

async fn update_user_decryption_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
    error_counter: i16,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        "UPDATE user_decryption_requests SET status = $1, already_sent = $2, error_counter = $3
        WHERE decryption_id = $4",
        status as OperationStatus,
        already_sent,
        error_counter,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await
}

async fn update_prep_keygen_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        "UPDATE prep_keygen_requests SET status = $1, already_sent = $2 WHERE prep_keygen_id = $3",
        status as OperationStatus,
        already_sent,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await
}

async fn update_keygen_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        "UPDATE keygen_requests SET status = $1, already_sent = $2 WHERE key_id = $3",
        status as OperationStatus,
        already_sent,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await
}

async fn update_crsgen_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        "UPDATE crsgen_requests SET status = $1, already_sent = $2 WHERE crs_id = $3",
        status as OperationStatus,
        already_sent,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await
}

async fn update_abort_keygen_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        "UPDATE abort_keygen_requests SET status = $1, already_sent = $2 WHERE prep_keygen_id = $3",
        status as OperationStatus,
        already_sent,
        id.as_le_slice(),
    );
    execute_update_event_query(db, query).await
}

async fn update_abort_crsgen_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        "UPDATE abort_crsgen_requests SET status = $1, already_sent = $2 WHERE crs_id = $3",
        status as OperationStatus,
        already_sent,
        id.as_le_slice(),
    );
    execute_update_event_query(db, query).await
}

async fn update_new_kms_context_status(
    db: &Pool<Postgres>,
    context_id: U256,
    status: OperationStatus,
    already_sent: bool,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        "UPDATE new_kms_context SET status = $1, already_sent = $2 WHERE context_id = $3",
        status as OperationStatus,
        already_sent,
        context_id.as_le_slice()
    );
    execute_update_event_query(db, query).await
}

async fn update_new_kms_epoch_status(
    db: &Pool<Postgres>,
    epoch_id: U256,
    status: OperationStatus,
    already_sent: bool,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        "UPDATE new_kms_epoch SET status = $1, already_sent = $2 WHERE epoch_id = $3",
        status as OperationStatus,
        already_sent,
        epoch_id.as_le_slice()
    );
    execute_update_event_query(db, query).await
}

async fn execute_update_event_query(
    db: &Pool<Postgres>,
    query: Query<'_, Postgres, PgArguments>,
) -> anyhow::Result<()> {
    let query_result = query.execute(db).await?;
    if query_result.rows_affected() == 1 {
        info!("Successfully updated event in DB!");
        Ok(())
    } else {
        Err(anyhow!("unexpected query result: {query_result:?}"))
    }
}

impl Display for ProtocolEventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolEventKind::PublicDecryption(e) => {
                write!(f, "PublicDecryptionRequest #{:#066x}", e.decryptionId)
            }
            ProtocolEventKind::UserDecryption(e) => {
                write!(f, "UserDecryptionRequest #{:#066x}", e.decryptionId)
            }
            ProtocolEventKind::UserDecryptionV2(e) => {
                write!(f, "UserDecryptionRequest #{:#066x}", e.decryptionId)
            }
            ProtocolEventKind::PrepKeygen(e) => {
                write!(f, "PrepKeygenRequest #{:#066x}", e.prepKeygenId)
            }
            ProtocolEventKind::Keygen(e) => {
                write!(f, "KeygenRequest #{:#066x}", e.requestId)
            }
            ProtocolEventKind::Crsgen(e) => {
                write!(f, "CrsgenRequest #{:#066x}", e.crsId)
            }
            ProtocolEventKind::AbortKeygen(e) => {
                write!(f, "AbortKeygen #{:#066x}", e.prepKeygenId)
            }
            ProtocolEventKind::AbortCrsgen(e) => {
                write!(f, "AbortCrsgen #{:#066x}", e.crsId)
            }
            ProtocolEventKind::NewKmsContext(e) => {
                write!(f, "NewKmsContext #{:#066x}", e.contextId)
            }
            ProtocolEventKind::NewKmsEpoch(e) => {
                write!(f, "NewKmsEpoch #{:#066x}", e.epochId)
            }
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

impl From<AbortKeygen> for ProtocolEventKind {
    fn from(value: AbortKeygen) -> Self {
        Self::AbortKeygen(value)
    }
}

impl From<AbortCrsgen> for ProtocolEventKind {
    fn from(value: AbortCrsgen) -> Self {
        Self::AbortCrsgen(value)
    }
}

impl From<NewKmsContext> for ProtocolEventKind {
    fn from(value: NewKmsContext) -> Self {
        Self::NewKmsContext(value)
    }
}

impl From<NewKmsEpoch> for ProtocolEventKind {
    fn from(value: NewKmsEpoch) -> Self {
        Self::NewKmsEpoch(value)
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
            KMSGenerationEvents::AbortKeygen(e) => Ok(e.into()),
            KMSGenerationEvents::AbortCrsgen(e) => Ok(e.into()),
            _ => Err(anyhow!("Unexpected KMSGeneration event")),
        }
    }
}

impl TryFrom<ProtocolConfigEvents> for ProtocolEventKind {
    type Error = anyhow::Error;

    fn try_from(value: ProtocolConfigEvents) -> Result<Self, Self::Error> {
        match value {
            ProtocolConfigEvents::NewKmsContext(e) => Ok(e.into()),
            ProtocolConfigEvents::NewKmsEpoch(e) => Ok(e.into()),
            _ => Err(anyhow!("Unexpected ProtocolConfig event")),
        }
    }
}
