use crate::{
    monitoring::otlp::PropagationContext,
    types::db::{OperationStatus, ParamsTypeDb, SnsCiphertextMaterialDbItem},
};
use alloy::primitives::U256;
use fhevm_gateway_bindings::{
    decryption::Decryption::{
        PublicDecryptionRequest, SnsCiphertextMaterial, UserDecryptionRequest,
    },
    kms_generation::KMSGeneration::{
        CrsgenRequest, KeyReshareSameSet, KeygenRequest, PRSSInit, PrepKeygenRequest,
    },
};
use sqlx::{
    Pool, Postgres, Row,
    postgres::{PgArguments, PgRow},
    query::Query,
};
use std::fmt::Display;
use tracing::{info, warn};

/// The events emitted by the Gateway which are monitored by the KMS Connector.
#[derive(Clone, Debug, PartialEq)]
pub struct GatewayEvent {
    pub kind: GatewayEventKind,
    pub calldata: Option<Vec<u8>>,
    pub already_sent: bool,
    pub error_counter: i16,
    pub otlp_context: PropagationContext,
}

impl GatewayEvent {
    pub fn new(
        kind: GatewayEventKind,
        calldata: Option<Vec<u8>>,
        otlp_context: PropagationContext,
    ) -> Self {
        GatewayEvent {
            kind,
            calldata,
            already_sent: false,
            error_counter: 0,
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
            GatewayEventKind::PublicDecryption(e) => {
                update_public_decryption_status(db, e.decryptionId, status, already_sent, err_count)
                    .await
            }
            GatewayEventKind::UserDecryption(e) => {
                update_user_decryption_status(db, e.decryptionId, status, already_sent, err_count)
                    .await
            }
            GatewayEventKind::PrepKeygen(e) => {
                update_prep_keygen_status(db, e.prepKeygenId, status, already_sent).await
            }
            GatewayEventKind::Keygen(e) => {
                update_keygen_status(db, e.keyId, status, already_sent).await
            }
            GatewayEventKind::Crsgen(e) => {
                update_crsgen_status(db, e.crsId, status, already_sent).await
            }
            GatewayEventKind::PrssInit(id) => update_prss_init_status(db, *id, status).await,
            GatewayEventKind::KeyReshareSameSet(e) => {
                update_key_reshare_same_set_status(db, e.keyId, status).await
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GatewayEventKind {
    PublicDecryption(PublicDecryptionRequest),
    UserDecryption(UserDecryptionRequest),
    PrepKeygen(PrepKeygenRequest),
    Keygen(KeygenRequest),
    Crsgen(CrsgenRequest),
    PrssInit(U256),
    KeyReshareSameSet(KeyReshareSameSet),
}

pub fn from_public_decryption_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let sns_ct_materials = row
        .try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?
        .iter()
        .map(SnsCiphertextMaterial::from)
        .collect();

    let kind = GatewayEventKind::PublicDecryption(PublicDecryptionRequest {
        decryptionId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
        snsCtMaterials: sns_ct_materials,
        extraData: row.try_get::<Vec<u8>, _>("extra_data")?.into(),
    });
    Ok(GatewayEvent {
        kind,
        calldata: None,
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: row.try_get::<i16, _>("error_counter")?,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_user_decryption_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let sns_ct_materials = row
        .try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?
        .iter()
        .map(SnsCiphertextMaterial::from)
        .collect();

    let kind = GatewayEventKind::UserDecryption(UserDecryptionRequest {
        decryptionId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
        snsCtMaterials: sns_ct_materials,
        userAddress: row.try_get::<[u8; 20], _>("user_address")?.into(),
        publicKey: row.try_get::<Vec<u8>, _>("public_key")?.into(),
        extraData: row.try_get::<Vec<u8>, _>("extra_data")?.into(),
    });

    Ok(GatewayEvent {
        kind,
        calldata: row.try_get::<Option<Vec<u8>>, _>("calldata")?,
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: row.try_get::<i16, _>("error_counter")?,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_prep_keygen_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let kind = GatewayEventKind::PrepKeygen(PrepKeygenRequest {
        prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
        epochId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("epoch_id")?),
        paramsType: row.try_get::<ParamsTypeDb, _>("params_type")? as u8,
    });
    Ok(GatewayEvent {
        kind,
        calldata: None,
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_keygen_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let kind = GatewayEventKind::Keygen(KeygenRequest {
        prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
        keyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?),
    });
    Ok(GatewayEvent {
        kind,
        calldata: None,
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_crsgen_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let kind = GatewayEventKind::Crsgen(CrsgenRequest {
        crsId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("crs_id")?),
        maxBitLength: U256::from_le_bytes(row.try_get::<[u8; 32], _>("max_bit_length")?),
        paramsType: row.try_get::<ParamsTypeDb, _>("params_type")? as u8,
    });
    Ok(GatewayEvent {
        kind,
        calldata: None,
        already_sent: row.try_get::<bool, _>("already_sent")?,
        error_counter: 0,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_prss_init_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let kind = GatewayEventKind::PrssInit(U256::from_le_bytes(row.try_get::<[u8; 32], _>("id")?));
    Ok(GatewayEvent {
        kind,
        calldata: None,
        already_sent: false,
        error_counter: 0,
        otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_key_reshare_same_set_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let kind = GatewayEventKind::KeyReshareSameSet(KeyReshareSameSet {
        prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
        keyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?),
        keyReshareId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_reshare_id")?),
        paramsType: row.try_get::<ParamsTypeDb, _>("params_type")? as u8,
    });
    Ok(GatewayEvent {
        kind,
        calldata: None,
        already_sent: false,
        error_counter: 0,
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

async fn update_prss_init_status(db: &Pool<Postgres>, id: U256, status: OperationStatus) {
    let query = sqlx::query!(
        "UPDATE prss_init SET status = $1 WHERE id = $2",
        status as OperationStatus,
        id.as_le_slice()
    );
    execute_update_event_query(db, query).await;
}

async fn update_key_reshare_same_set_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
) {
    let query = sqlx::query!(
        "UPDATE key_reshare_same_set SET status = $1 WHERE key_id = $2",
        status as OperationStatus,
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

impl Display for GatewayEventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GatewayEventKind::PublicDecryption(e) => {
                write!(f, "PublicDecryptionRequest #{}", e.decryptionId)
            }
            GatewayEventKind::UserDecryption(e) => {
                write!(f, "UserDecryptionRequest #{}", e.decryptionId)
            }
            GatewayEventKind::PrepKeygen(e) => {
                write!(f, "PrepKeygenRequest #{}", e.prepKeygenId)
            }
            GatewayEventKind::Keygen(e) => write!(f, "KeygenRequest #{}", e.keyId),
            GatewayEventKind::Crsgen(e) => write!(f, "CrsgenRequest #{}", e.crsId),
            GatewayEventKind::PrssInit(id) => write!(f, "PrssInit #{id}"),
            GatewayEventKind::KeyReshareSameSet(e) => {
                write!(f, "KeyReshareSameSet #{}", e.keyId)
            }
        }
    }
}

impl From<PublicDecryptionRequest> for GatewayEventKind {
    fn from(value: PublicDecryptionRequest) -> Self {
        Self::PublicDecryption(value)
    }
}

impl From<UserDecryptionRequest> for GatewayEventKind {
    fn from(value: UserDecryptionRequest) -> Self {
        Self::UserDecryption(value)
    }
}

impl From<PrepKeygenRequest> for GatewayEventKind {
    fn from(value: PrepKeygenRequest) -> Self {
        Self::PrepKeygen(value)
    }
}

impl From<KeygenRequest> for GatewayEventKind {
    fn from(value: KeygenRequest) -> Self {
        Self::Keygen(value)
    }
}

impl From<CrsgenRequest> for GatewayEventKind {
    fn from(value: CrsgenRequest) -> Self {
        Self::Crsgen(value)
    }
}

impl From<PRSSInit> for GatewayEventKind {
    fn from(_value: PRSSInit) -> Self {
        Self::PrssInit(PRSS_INIT_ID)
    }
}

impl From<KeyReshareSameSet> for GatewayEventKind {
    fn from(value: KeyReshareSameSet) -> Self {
        Self::KeyReshareSameSet(value)
    }
}

pub const PRSS_INIT_ID: U256 = U256::ONE;
