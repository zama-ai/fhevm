use crate::{
    monitoring::otlp::PropagationContext,
    types::db::{OperationStatus, ParamsTypeDb},
};
use alloy::primitives::{Address, FixedBytes, U256};
use fhevm_gateway_bindings::{
    decryption_registry::DecryptionRegistry::{
        PublicDecryptionRequested, UserDecryptionRequested,
    },
    kms_generation::KMSGeneration::{
        CrsgenRequest, KeyReshareSameSet, KeygenRequest, PRSSInit, PrepKeygenRequest,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct PublicDecryptionRequestV2 {
    pub request_id: U256,
    pub handles: Vec<FixedBytes<32>>,
    pub contract_addresses: Vec<Address>,
    pub chain_id: U256,
    pub timestamp: U256,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserDecryptionRequestV2 {
    pub request_id: U256,
    pub handles: Vec<FixedBytes<32>>,
    pub contract_addresses: Vec<Address>,
    pub user_address: Address,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
    pub chain_id: U256,
    pub timestamp: U256,
}
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
    pub already_sent: bool,
    pub error_counter: i16,
    pub otlp_context: PropagationContext,
}

impl GatewayEvent {
    pub fn new(kind: GatewayEventKind, otlp_context: PropagationContext) -> Self {
        GatewayEvent {
            kind,
            error_counter: 0,
            already_sent: false,
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
                update_public_decryption_status(db, e.request_id, status, already_sent, err_count)
                    .await
            }
            GatewayEventKind::UserDecryption(e) => {
                update_user_decryption_status(db, e.request_id, status, already_sent, err_count)
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
    PublicDecryption(PublicDecryptionRequestV2),
    UserDecryption(UserDecryptionRequestV2),
    PrepKeygen(PrepKeygenRequest),
    Keygen(KeygenRequest),
    Crsgen(CrsgenRequest),
    PrssInit(U256),
    KeyReshareSameSet(KeyReshareSameSet),
}

pub fn from_public_decryption_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let handles: Vec<[u8; 32]> = row.try_get("handles")?;
    let contract_addresses: Vec<[u8; 20]> = row.try_get("contract_addresses")?;

    let kind = GatewayEventKind::PublicDecryption(PublicDecryptionRequestV2 {
        request_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("request_id")?),
        handles: handles.into_iter().map(FixedBytes::from).collect(),
        contract_addresses: contract_addresses.into_iter().map(Address::from).collect(),
        chain_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("chain_id")?),
        timestamp: U256::from_le_bytes(row.try_get::<[u8; 32], _>("timestamp")?),
    });
    Ok(GatewayEvent {
        kind,
        error_counter: row.try_get::<i16, _>("error_counter")?,
        already_sent: row.try_get::<bool, _>("already_sent")?,
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_user_decryption_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let handles: Vec<[u8; 32]> = row.try_get("handles")?;
    let contract_addresses: Vec<[u8; 20]> = row.try_get("contract_addresses")?;

    let kind = GatewayEventKind::UserDecryption(UserDecryptionRequestV2 {
        request_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("request_id")?),
        handles: handles.into_iter().map(FixedBytes::from).collect(),
        contract_addresses: contract_addresses.into_iter().map(Address::from).collect(),
        user_address: row.try_get::<[u8; 20], _>("user_address")?.into(),
        public_key: row.try_get::<Vec<u8>, _>("public_key")?,
        signature: row.try_get::<Vec<u8>, _>("signature")?,
        chain_id: U256::from_le_bytes(row.try_get::<[u8; 32], _>("chain_id")?),
        timestamp: U256::from_le_bytes(row.try_get::<[u8; 32], _>("timestamp")?),
    });
    Ok(GatewayEvent {
        kind,
        error_counter: row.try_get::<i16, _>("error_counter")?,
        already_sent: row.try_get::<bool, _>("already_sent")?,
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
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
        error_counter: 0,
        already_sent: row.try_get::<bool, _>("already_sent")?,
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_keygen_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let kind = GatewayEventKind::Keygen(KeygenRequest {
        prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
        keyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?),
    });
    Ok(GatewayEvent {
        kind,
        error_counter: 0,
        already_sent: row.try_get::<bool, _>("already_sent")?,
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
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
        error_counter: 0,
        already_sent: row.try_get::<bool, _>("already_sent")?,
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_prss_init_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let kind = GatewayEventKind::PrssInit(U256::from_le_bytes(row.try_get::<[u8; 32], _>("id")?));
    Ok(GatewayEvent {
        kind,
        error_counter: 0,
        already_sent: false,
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
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
        error_counter: 0,
        already_sent: false,
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

async fn update_public_decryption_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
    error_counter: i16,
) {
    let query = sqlx::query(
        "UPDATE public_decryption_requests SET status = $1, already_sent = $2, error_counter = $3 \
        WHERE decryption_id = $4",
    )
    .bind(status as OperationStatus)
    .bind(already_sent)
    .bind(error_counter)
    .bind(id.as_le_slice());
    execute_update_event_query_dynamic(db, query).await;
}

async fn update_user_decryption_status(
    db: &Pool<Postgres>,
    id: U256,
    status: OperationStatus,
    already_sent: bool,
    error_counter: i16,
) {
    let query = sqlx::query(
        "UPDATE user_decryption_requests SET status = $1, already_sent = $2, error_counter = $3 \
        WHERE decryption_id = $4",
    )
    .bind(status as OperationStatus)
    .bind(already_sent)
    .bind(error_counter)
    .bind(id.as_le_slice());
    execute_update_event_query_dynamic(db, query).await;
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

async fn execute_update_event_query_dynamic<'a>(
    db: &Pool<Postgres>,
    query: sqlx::query::Query<'a, Postgres, sqlx::postgres::PgArguments>,
) {
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
                write!(f, "PublicDecryptionRequest #{}", e.request_id)
            }
            GatewayEventKind::UserDecryption(e) => {
                write!(f, "UserDecryptionRequest #{}", e.request_id)
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

impl From<PublicDecryptionRequested> for GatewayEventKind {
    fn from(value: PublicDecryptionRequested) -> Self {
        Self::PublicDecryption(PublicDecryptionRequestV2 {
            request_id: value.requestId,
            handles: value.handles,
            contract_addresses: value.contractAddresses,
            chain_id: value.chainId,
            timestamp: value.timestamp,
        })
    }
}

impl From<UserDecryptionRequested> for GatewayEventKind {
    fn from(value: UserDecryptionRequested) -> Self {
        Self::UserDecryption(UserDecryptionRequestV2 {
            request_id: value.requestId,
            handles: value.handles,
            contract_addresses: value.contractAddresses,
            user_address: value.userAddress,
            public_key: value.publicKey.to_vec(),
            signature: value.signature.to_vec(),
            chain_id: value.chainId,
            timestamp: value.timestamp,
        })
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

impl From<PublicDecryptionRequestV2> for GatewayEventKind {
    fn from(value: PublicDecryptionRequestV2) -> Self {
        Self::PublicDecryption(value)
    }
}

impl From<UserDecryptionRequestV2> for GatewayEventKind {
    fn from(value: UserDecryptionRequestV2) -> Self {
        Self::UserDecryption(value)
    }
}

pub const PRSS_INIT_ID: U256 = U256::ONE;
