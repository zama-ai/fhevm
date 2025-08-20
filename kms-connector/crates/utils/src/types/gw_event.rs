use crate::types::db::SnsCiphertextMaterialDbItem;
use alloy::primitives::U256;
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::{
        PublicDecryptionRequest, SnsCiphertextMaterial, UserDecryptionRequest,
    },
    kms_management::KmsManagement::{
        CrsgenRequest, KeygenRequest, KskgenRequest, PreprocessKeygenRequest,
        PreprocessKskgenRequest,
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
pub enum GatewayEvent {
    PublicDecryption(PublicDecryptionRequest),
    UserDecryption(UserDecryptionRequest),
    PreprocessKeygen(PreprocessKeygenRequest),
    PreprocessKskgen(PreprocessKskgenRequest),
    Keygen(KeygenRequest),
    Kskgen(KskgenRequest),
    Crsgen(CrsgenRequest),
}

impl GatewayEvent {
    /// Create a new `GatewayEvent::PublicDecryption` from a `PgRow`.
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

    /// Create a new `GatewayEvent::UserDecryption` from a `PgRow`.
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

    /// Create a new `GatewayEvent::PreprocessKeygen` from a `PgRow`.
    pub fn from_pre_keygen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::PreprocessKeygen(PreprocessKeygenRequest {
            preKeygenRequestId: U256::from_le_bytes(
                row.try_get::<[u8; 32], _>("pre_keygen_request_id")?,
            ),
            fheParamsDigest: row.try_get::<[u8; 32], _>("fhe_params_digest")?.into(),
        }))
    }

    /// Create a new `GatewayEvent::PreprocessKskgen` from a `PgRow`.
    pub fn from_pre_kskgen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::PreprocessKskgen(PreprocessKskgenRequest {
            preKskgenRequestId: U256::from_le_bytes(
                row.try_get::<[u8; 32], _>("pre_kskgen_request_id")?,
            ),
            fheParamsDigest: row.try_get::<[u8; 32], _>("fhe_params_digest")?.into(),
        }))
    }

    /// Create a new `GatewayEvent::Keygen` from a `PgRow`.
    pub fn from_keygen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::Keygen(KeygenRequest {
            preKeyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_key_id")?),
            fheParamsDigest: row.try_get::<[u8; 32], _>("fhe_params_digest")?.into(),
        }))
    }

    /// Create a new `GatewayEvent::Kskgen` from a `PgRow`.
    pub fn from_kskgen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::Kskgen(KskgenRequest {
            preKskId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_ksk_id")?),
            fheParamsDigest: row.try_get::<[u8; 32], _>("fhe_params_digest")?.into(),
            sourceKeyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("source_key_id")?),
            destKeyId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("dest_key_id")?),
        }))
    }

    /// Create a new `GatewayEvent::Crsgen` from a `PgRow`.
    pub fn from_crsgen_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(GatewayEvent::Crsgen(CrsgenRequest {
            crsgenRequestId: U256::from_le_bytes(row.try_get::<[u8; 32], _>("crsgen_request_id")?),
            fheParamsDigest: row.try_get::<[u8; 32], _>("fhe_params_digest")?.into(),
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
            GatewayEvent::PreprocessKeygen(e) => {
                Self::mark_pre_keygen_as_pending(db, e.preKeygenRequestId).await
            }
            GatewayEvent::PreprocessKskgen(e) => {
                Self::mark_pre_kskgen_as_pending(db, e.preKskgenRequestId).await
            }
            GatewayEvent::Keygen(e) => Self::mark_keygen_as_pending(db, e.preKeyId).await,
            GatewayEvent::Kskgen(e) => Self::mark_kskgen_as_pending(db, e.preKskId).await,
            GatewayEvent::Crsgen(e) => Self::mark_crsgen_as_pending(db, e.crsgenRequestId).await,
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

    /// Sets the `under_process` field of the `PreprocessKeygenRequest` as `FALSE` in the database.
    pub async fn mark_pre_keygen_as_pending(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "UPDATE preprocess_keygen_requests SET under_process = FALSE WHERE pre_keygen_request_id = $1",
            id.as_le_slice()
        );
        Self::execute_free_event_query(db, query).await;
    }

    /// Sets the `under_process` field of the `PreprocessKskgenRequest` as `FALSE` in the database.
    pub async fn mark_pre_kskgen_as_pending(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "UPDATE preprocess_kskgen_requests SET under_process = FALSE WHERE pre_kskgen_request_id = $1",
            id.as_le_slice()
        );
        Self::execute_free_event_query(db, query).await;
    }

    /// Sets the `under_process` field of the `KeyRequest` as `FALSE` in the database.
    pub async fn mark_keygen_as_pending(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "UPDATE keygen_requests SET under_process = FALSE WHERE pre_key_id = $1",
            id.as_le_slice()
        );
        Self::execute_free_event_query(db, query).await;
    }

    /// Sets the `under_process` field of the `KskgenRequest` as `FALSE` in the database.
    pub async fn mark_kskgen_as_pending(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "UPDATE kskgen_requests SET under_process = FALSE WHERE pre_ksk_id = $1",
            id.as_le_slice()
        );
        Self::execute_free_event_query(db, query).await;
    }

    /// Sets the `under_process` field of the `CrsgenRequest` as `FALSE` in the database.
    pub async fn mark_crsgen_as_pending(db: &Pool<Postgres>, id: U256) {
        let query = sqlx::query!(
            "UPDATE crsgen_requests SET under_process = FALSE WHERE crsgen_request_id = $1",
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
            GatewayEvent::PreprocessKeygen(e) => {
                write!(f, "PreprocessKeygenRequest #{}", e.preKeygenRequestId)
            }
            GatewayEvent::PreprocessKskgen(e) => {
                write!(f, "PreprocessKskgenRequest #{}", e.preKskgenRequestId)
            }
            GatewayEvent::Keygen(e) => write!(f, "KeygenRequest #{}", e.preKeyId),
            GatewayEvent::Kskgen(e) => write!(f, "KskgenRequest #{}", e.preKskId),
            GatewayEvent::Crsgen(e) => write!(f, "CrsgenRequest #{}", e.crsgenRequestId),
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

impl From<PreprocessKeygenRequest> for GatewayEvent {
    fn from(value: PreprocessKeygenRequest) -> Self {
        Self::PreprocessKeygen(value)
    }
}

impl From<PreprocessKskgenRequest> for GatewayEvent {
    fn from(value: PreprocessKskgenRequest) -> Self {
        Self::PreprocessKskgen(value)
    }
}

impl From<KeygenRequest> for GatewayEvent {
    fn from(value: KeygenRequest) -> Self {
        Self::Keygen(value)
    }
}

impl From<KskgenRequest> for GatewayEvent {
    fn from(value: KskgenRequest) -> Self {
        Self::Kskgen(value)
    }
}

impl From<CrsgenRequest> for GatewayEvent {
    fn from(value: CrsgenRequest) -> Self {
        Self::Crsgen(value)
    }
}
