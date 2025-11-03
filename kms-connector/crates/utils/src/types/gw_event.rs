use crate::{
    monitoring::otlp::PropagationContext,
    types::db::{ParamsTypeDb, SnsCiphertextMaterialDbItem},
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
use tracing::{error, info, warn};

/// The events emitted by the Gateway which are monitored by the KMS Connector.
#[derive(Clone, Debug, PartialEq)]
pub struct GatewayEvent {
    pub kind: GatewayEventKind,
    pub otlp_context: PropagationContext,
}

impl GatewayEvent {
    pub fn new(kind: GatewayEventKind, otlp_context: PropagationContext) -> Self {
        GatewayEvent { kind, otlp_context }
    }

    /// Sets the `under_process` field of the event as `FALSE` in the database.
    pub async fn mark_as_pending(&self, db: &Pool<Postgres>) {
        match &self.kind {
            GatewayEventKind::PublicDecryption(e) => {
                mark_public_decryption_as_pending(db, e.decryptionId).await
            }
            GatewayEventKind::UserDecryption(e) => {
                mark_user_decryption_as_pending(db, e.decryptionId).await
            }
            GatewayEventKind::PrepKeygen(e) => {
                mark_prep_keygen_as_pending(db, e.prepKeygenId).await
            }
            GatewayEventKind::Keygen(e) => mark_keygen_as_pending(db, e.keyId).await,
            GatewayEventKind::Crsgen(e) => mark_crsgen_as_pending(db, e.crsId).await,
            GatewayEventKind::PrssInit(_) => mark_prss_init_as_pending(db, PRSS_INIT_ID).await,
            GatewayEventKind::KeyReshareSameSet(e) => {
                mark_key_reshare_same_set_as_pending(db, e.keyId).await
            }
        }
    }

    pub async fn delete_from_db(&self, db: &Pool<Postgres>) {
        match &self.kind {
            GatewayEventKind::PublicDecryption(e) => {
                delete_public_decryption_from_db(db, e.decryptionId).await
            }
            GatewayEventKind::UserDecryption(e) => {
                delete_user_decryption_from_db(db, e.decryptionId).await
            }
            GatewayEventKind::PrepKeygen(e) => delete_prep_keygen_from_db(db, e.prepKeygenId).await,
            GatewayEventKind::Keygen(e) => delete_keygen_from_db(db, e.keyId).await,
            GatewayEventKind::Crsgen(e) => delete_crsgen_from_db(db, e.crsId).await,
            GatewayEventKind::PrssInit(id) => delete_prss_init_from_db(db, *id).await,
            GatewayEventKind::KeyReshareSameSet(e) => {
                delete_key_reshare_same_set_from_db(db, e.keyId).await
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
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
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
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

pub fn from_prss_init_row(row: &PgRow) -> anyhow::Result<GatewayEvent> {
    let kind = GatewayEventKind::PrssInit(U256::from_le_bytes(row.try_get::<[u8; 32], _>("id")?));
    Ok(GatewayEvent {
        kind,
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
        otlp_context: bc2wrap::deserialize(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
    })
}

/// Sets the `under_process` field of the `PublicDecryptionRequest` as `FALSE` in the database.
pub async fn mark_public_decryption_as_pending(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "UPDATE public_decryption_requests SET under_process = FALSE WHERE decryption_id = $1",
        id.as_le_slice()
    );
    execute_free_event_query(db, query).await;
}

/// Sets the `under_process` field of the `UserDecryptionRequest` as `FALSE` in the database.
pub async fn mark_user_decryption_as_pending(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "UPDATE user_decryption_requests SET under_process = FALSE WHERE decryption_id = $1",
        id.as_le_slice()
    );
    execute_free_event_query(db, query).await;
}

/// Sets the `under_process` field of the `PrepKeygenRequest` as `FALSE` in the database.
pub async fn mark_prep_keygen_as_pending(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "UPDATE prep_keygen_requests SET under_process = FALSE WHERE prep_keygen_id = $1",
        id.as_le_slice()
    );
    execute_free_event_query(db, query).await;
}

/// Sets the `under_process` field of the `KeygenRequest` as `FALSE` in the database.
pub async fn mark_keygen_as_pending(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "UPDATE keygen_requests SET under_process = FALSE WHERE key_id = $1",
        id.as_le_slice()
    );
    execute_free_event_query(db, query).await;
}

/// Sets the `under_process` field of the `CrsgenRequest` as `FALSE` in the database.
pub async fn mark_crsgen_as_pending(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "UPDATE crsgen_requests SET under_process = FALSE WHERE crs_id = $1",
        id.as_le_slice()
    );
    execute_free_event_query(db, query).await;
}

/// Sets the `under_process` field of the `PrssInit` as `FALSE` in the database.
pub async fn mark_prss_init_as_pending(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "UPDATE prss_init SET under_process = FALSE WHERE id = $1",
        id.as_le_slice()
    );
    execute_free_event_query(db, query).await;
}

/// Sets the `under_process` field of the `KeyReshareSameSet` as `FALSE` in the database.
pub async fn mark_key_reshare_same_set_as_pending(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "UPDATE key_reshare_same_set SET under_process = FALSE WHERE key_id = $1",
        id.as_le_slice()
    );
    execute_free_event_query(db, query).await;
}

/// Executes the free event query and checks its result.
async fn execute_free_event_query(db: &Pool<Postgres>, query: Query<'_, Postgres, PgArguments>) {
    warn!("Failed to process event. Restoring `under_process` field to `FALSE` in DB...");
    let query_result = match query.execute(db).await {
        Ok(result) => result,
        Err(e) => return warn!("Failed to restore `under_process` field to `FALSE`: {e}"),
    };

    if query_result.rows_affected() == 1 {
        info!("Successfully restore `under_process` field to `FALSE` in DB!");
    } else {
        warn!(
            "Unexpected query result while restoring `under_process` field to `FALSE`: {:?}",
            query_result
        )
    }
}

pub async fn delete_public_decryption_from_db(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "DELETE FROM public_decryption_requests WHERE decryption_id = $1",
        id.as_le_slice()
    );
    execute_delete_event_query(db, query).await;
}

pub async fn delete_user_decryption_from_db(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "DELETE FROM user_decryption_requests WHERE decryption_id = $1",
        id.as_le_slice()
    );
    execute_delete_event_query(db, query).await;
}

pub async fn delete_prep_keygen_from_db(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "DELETE FROM prep_keygen_requests WHERE prep_keygen_id = $1",
        id.as_le_slice()
    );
    execute_delete_event_query(db, query).await;
}

pub async fn delete_keygen_from_db(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "DELETE FROM keygen_requests WHERE key_id = $1",
        id.as_le_slice()
    );
    execute_delete_event_query(db, query).await;
}

pub async fn delete_crsgen_from_db(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "DELETE FROM crsgen_requests WHERE crs_id = $1",
        id.as_le_slice()
    );
    execute_delete_event_query(db, query).await;
}

pub async fn delete_prss_init_from_db(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!("DELETE FROM prss_init WHERE id = $1", id.as_le_slice());
    execute_delete_event_query(db, query).await;
}

pub async fn delete_key_reshare_same_set_from_db(db: &Pool<Postgres>, id: U256) {
    let query = sqlx::query!(
        "DELETE FROM key_reshare_same_set WHERE key_id = $1",
        id.as_le_slice()
    );
    execute_delete_event_query(db, query).await;
}

async fn execute_delete_event_query(db: &Pool<Postgres>, query: Query<'_, Postgres, PgArguments>) {
    warn!("Removing event from DB...");
    let query_result = match query.execute(db).await {
        Ok(result) => result,
        Err(e) => return error!("Failed to remove event from DB: {e}"),
    };

    if query_result.rows_affected() == 1 {
        info!("Successfully deleted event from DB!");
    } else {
        warn!(
            "Unexpected query result while deleting event from DB: {:?}",
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
