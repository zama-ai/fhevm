use crate::types::db::{ParamsTypeDb, SnsCiphertextMaterialDbItem};
use alloy::primitives::U256;
use fhevm_gateway_bindings::{
    decryption::Decryption::{
        PublicDecryptionRequest, SnsCiphertextMaterial, UserDecryptionRequest,
    },
    kms_management::KmsManagement::{CrsgenRequest, KeygenRequest, PrepKeygenRequest},
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
pub enum GatewayEvent {
    PublicDecryption(PublicDecryptionRequest),
    UserDecryption(UserDecryptionRequest),
    PrepKeygen(PrepKeygenRequest),
    Keygen(KeygenRequest),
    Crsgen(CrsgenRequest),
}

impl GatewayEvent {
    pub fn from_public_decryption_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let sns_ct_materials = row
            .try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?
            .iter()
            .map(SnsCiphertextMaterial::from)
            .collect();

        Ok(GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            snsCtMaterials: sns_ct_materials,
            extraData: row.try_get::<Vec<u8>, _>("extra_data")?.into(),
        }))
    }

    pub fn from_user_decryption_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let sns_ct_materials = row
            .try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?
            .iter()
            .map(SnsCiphertextMaterial::from)
            .collect();

        Ok(GatewayEvent::UserDecryption(UserDecryptionRequest {
            decryptionId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
            snsCtMaterials: sns_ct_materials,
            userAddress: row.try_get::<[u8; 20], _>("user_address")?.into(),
            publicKey: row.try_get::<Vec<u8>, _>("public_key")?.into(),
            extraData: row.try_get::<Vec<u8>, _>("extra_data")?.into(),
        }))
    }

    pub fn from_prep_keygen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::PrepKeygen(PrepKeygenRequest {
            prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
            epochId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("epoch_id")?),
            paramsType: row.try_get::<ParamsTypeDb, _>("params_type")? as u8,
        }))
    }

    pub fn from_keygen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::Keygen(KeygenRequest {
            prepKeygenId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?),
            keyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?),
        }))
    }

    pub fn from_crsgen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::Crsgen(CrsgenRequest {
            crsId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("crs_id")?),
            maxBitLength: U256::from_le_bytes(row.try_get::<[u8; 32], _>("max_bit_length")?),
            paramsType: row.try_get::<ParamsTypeDb, _>("params_type")? as u8,
        }))
    }

    /// Sets the `under_process` field of the event as `FALSE` in the database.
    pub async fn mark_as_pending(&self, db: &Pool<Postgres>) {
        match self {
            GatewayEvent::PublicDecryption(e) => {
                Self::mark_public_decryption_as_pending(db, e.decryptionId).await
            }
            GatewayEvent::UserDecryption(e) => {
                Self::mark_user_decryption_as_pending(db, e.decryptionId).await
            }
            GatewayEvent::PrepKeygen(e) => {
                Self::mark_pre_keygen_as_pending(db, e.prepKeygenId).await
            }
            GatewayEvent::Keygen(e) => Self::mark_keygen_as_pending(db, e.prepKeygenId).await,
            GatewayEvent::Crsgen(e) => Self::mark_crsgen_as_pending(db, e.crsId).await,
        }
    }

    /// Sets the `under_process` field of the `PublicDecryptionRequest` as `FALSE` in the database.
    pub async fn mark_public_decryption_as_pending(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "UPDATE public_decryption_requests SET under_process = FALSE WHERE decryption_id = $1",
            id.as_le_slice()
        );
        Self::execute_free_event_query(db, query).await;
    }

    /// Sets the `under_process` field of the `UserDecryptionRequest` as `FALSE` in the database.
    pub async fn mark_user_decryption_as_pending(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "UPDATE user_decryption_requests SET under_process = FALSE WHERE decryption_id = $1",
            id.as_le_slice()
        );
        Self::execute_free_event_query(db, query).await;
    }

    /// Sets the `under_process` field of the `PrepKeygenRequest` as `FALSE` in the database.
    pub async fn mark_pre_keygen_as_pending(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "UPDATE prep_keygen_requests SET under_process = FALSE WHERE prep_keygen_id = $1",
            id.as_le_slice()
        );
        Self::execute_free_event_query(db, query).await;
    }

    /// Sets the `under_process` field of the `KeyRequest` as `FALSE` in the database.
    pub async fn mark_keygen_as_pending(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "UPDATE keygen_requests SET under_process = FALSE WHERE prep_keygen_id = $1",
            id.as_le_slice()
        );
        Self::execute_free_event_query(db, query).await;
    }

    /// Sets the `under_process` field of the `CrsgenRequest` as `FALSE` in the database.
    pub async fn mark_crsgen_as_pending(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "UPDATE crsgen_requests SET under_process = FALSE WHERE crs_id = $1",
            id.as_le_slice()
        );
        Self::execute_free_event_query(db, query).await;
    }

    /// Executes the free event query and checks its result.
    async fn execute_free_event_query(
        db: &Pool<Postgres>,
        query: Query<'_, Postgres, PgArguments>,
    ) {
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

    pub async fn delete_from_db(&self, db: &Pool<Postgres>) {
        match self {
            GatewayEvent::PublicDecryption(e) => {
                Self::delete_public_decryption_from_db(db, e.decryptionId).await
            }
            GatewayEvent::UserDecryption(e) => {
                Self::delete_user_decryption_from_db(db, e.decryptionId).await
            }
            GatewayEvent::PreprocessKeygen(e) => {
                Self::delete_pre_keygen_from_db(db, e.preKeygenRequestId).await
            }
            GatewayEvent::PreprocessKskgen(e) => {
                Self::delete_pre_kskgen_from_db(db, e.preKskgenRequestId).await
            }
            GatewayEvent::Keygen(e) => Self::delete_keygen_from_db(db, e.preKeyId).await,
            GatewayEvent::Kskgen(e) => Self::delete_kskgen_from_db(db, e.preKskId).await,
            GatewayEvent::Crsgen(e) => Self::delete_crsgen_from_db(db, e.crsgenRequestId).await,
        }
    }

    pub async fn delete_public_decryption_from_db(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "DELETE FROM public_decryption_requests WHERE decryption_id = $1",
            id.as_le_slice()
        );
        Self::execute_delete_event_query(db, query).await;
    }

    pub async fn delete_user_decryption_from_db(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "DELETE FROM user_decryption_requests WHERE decryption_id = $1",
            id.as_le_slice()
        );
        Self::execute_delete_event_query(db, query).await;
    }

    pub async fn delete_pre_keygen_from_db(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "DELETE FROM preprocess_keygen_requests WHERE pre_keygen_request_id = $1",
            id.as_le_slice()
        );
        Self::execute_delete_event_query(db, query).await;
    }

    pub async fn delete_pre_kskgen_from_db(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "DELETE FROM preprocess_kskgen_requests WHERE pre_kskgen_request_id = $1",
            id.as_le_slice()
        );
        Self::execute_delete_event_query(db, query).await;
    }

    pub async fn delete_keygen_from_db(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "DELETE FROM keygen_requests WHERE pre_key_id = $1",
            id.as_le_slice()
        );
        Self::execute_delete_event_query(db, query).await;
    }

    pub async fn delete_kskgen_from_db(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "DELETE FROM kskgen_requests WHERE pre_ksk_id = $1",
            id.as_le_slice()
        );
        Self::execute_delete_event_query(db, query).await;
    }

    pub async fn delete_crsgen_from_db(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "DELETE FROM crsgen_requests WHERE crsgen_request_id = $1",
            id.as_le_slice()
        );
        Self::execute_delete_event_query(db, query).await;
    }

    async fn execute_delete_event_query(
        db: &Pool<Postgres>,
        query: Query<'_, Postgres, PgArguments>,
    ) {
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
}

impl Display for GatewayEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GatewayEvent::PublicDecryption(e) => {
                write!(f, "PublicDecryptionRequest #{}", e.decryptionId)
            }
            GatewayEvent::UserDecryption(e) => {
                write!(f, "UserDecryptionRequest #{}", e.decryptionId)
            }
            GatewayEvent::PrepKeygen(e) => {
                write!(f, "PrepKeygenRequest #{}", e.prepKeygenId)
            }
            GatewayEvent::Keygen(e) => write!(f, "KeygenRequest #{}", e.keyId),
            GatewayEvent::Crsgen(e) => write!(f, "CrsgenRequest #{}", e.crsId),
        }
    }
}

impl From<PublicDecryptionRequest> for GatewayEvent {
    fn from(value: PublicDecryptionRequest) -> Self {
        Self::PublicDecryption(value)
    }
}

impl From<UserDecryptionRequest> for GatewayEvent {
    fn from(value: UserDecryptionRequest) -> Self {
        Self::UserDecryption(value)
    }
}

impl From<PrepKeygenRequest> for GatewayEvent {
    fn from(value: PrepKeygenRequest) -> Self {
        Self::PrepKeygen(value)
    }
}

impl From<KeygenRequest> for GatewayEvent {
    fn from(value: KeygenRequest) -> Self {
        Self::Keygen(value)
    }
}

impl From<CrsgenRequest> for GatewayEvent {
    fn from(value: CrsgenRequest) -> Self {
        Self::Crsgen(value)
    }
}
