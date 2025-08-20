use crate::monitoring::metrics::EVENT_STORED_COUNTER;
use anyhow::anyhow;
use connector_utils::types::{GatewayEvent, db::SnsCiphertextMaterialDbItem};
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
    kms_management::KmsManagement::{
        CrsgenRequest, KeygenRequest, KskgenRequest, PreprocessKeygenRequest,
        PreprocessKskgenRequest,
    },
};
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use tracing::info;

/// Interface used to publish Gateway's events in some storage.
pub trait EventPublisher: Clone + Send + Sync {
    fn publish(&self, event: GatewayEvent) -> impl Future<Output = anyhow::Result<()>> + Send;
}

/// Struct that stores Gateway's events in a `Postgres` database.
#[derive(Clone)]
pub struct DbEventPublisher {
    db_pool: Pool<Postgres>,
}

impl DbEventPublisher {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }
}

impl EventPublisher for DbEventPublisher {
    #[tracing::instrument(skip(self), fields(event = %event))]
    async fn publish(&self, event: GatewayEvent) -> anyhow::Result<()> {
        info!("Storing {:?} in DB...", event);

        let query_result = match event.clone() {
            GatewayEvent::PublicDecryption(e) => self.publish_public_decryption(e).await,
            GatewayEvent::UserDecryption(e) => self.publish_user_decryption(e).await,
            GatewayEvent::PreprocessKeygen(e) => self.publish_preprocess_keygen_request(e).await,
            GatewayEvent::PreprocessKskgen(e) => self.publish_preprocess_kskgen_request(e).await,
            GatewayEvent::Keygen(e) => self.publish_keygen_request(e).await,
            GatewayEvent::Kskgen(e) => self.publish_kskgen_request(e).await,
            GatewayEvent::Crsgen(e) => self.publish_crsgen_request(e).await,
        }
        .map_err(|err| anyhow!("Failed to publish event: {err}"))?;

        if query_result.rows_affected() > 0 {
            info!("Event successfully stored in DB!");
            EVENT_STORED_COUNTER.inc();
        }
        Ok(())
    }
}

impl DbEventPublisher {
    async fn publish_public_decryption(
        &self,
        request: PublicDecryptionRequest,
    ) -> sqlx::Result<PgQueryResult> {
        let sns_ciphertexts_db = request
            .snsCtMaterials
            .iter()
            .map(SnsCiphertextMaterialDbItem::from)
            .collect::<Vec<SnsCiphertextMaterialDbItem>>();

        sqlx::query!(
            "INSERT INTO public_decryption_requests VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            request.decryptionId.as_le_slice(),
            sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
            request.extraData.as_ref(),
        )
        .execute(&self.db_pool)
        .await
    }

    async fn publish_user_decryption(
        &self,
        request: UserDecryptionRequest,
    ) -> sqlx::Result<PgQueryResult> {
        let sns_ciphertexts_db = request
            .snsCtMaterials
            .iter()
            .map(SnsCiphertextMaterialDbItem::from)
            .collect::<Vec<SnsCiphertextMaterialDbItem>>();

        sqlx::query!(
            "INSERT INTO user_decryption_requests VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
            request.decryptionId.as_le_slice(),
            sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
            request.userAddress.as_slice(),
            request.publicKey.as_ref(),
            request.extraData.as_ref(),
        )
        .execute(&self.db_pool)
        .await
    }

    async fn publish_preprocess_keygen_request(
        &self,
        request: PreprocessKeygenRequest,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO preprocess_keygen_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
            request.preKeygenRequestId.as_le_slice(),
            request.fheParamsDigest.as_slice(),
        )
        .execute(&self.db_pool)
        .await
    }

    async fn publish_preprocess_kskgen_request(
        &self,
        request: PreprocessKskgenRequest,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO preprocess_kskgen_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
            request.preKskgenRequestId.as_le_slice(),
            request.fheParamsDigest.as_slice(),
        )
        .execute(&self.db_pool)
        .await
    }

    async fn publish_keygen_request(&self, request: KeygenRequest) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO keygen_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
            request.preKeyId.as_le_slice(),
            request.fheParamsDigest.as_slice(),
        )
        .execute(&self.db_pool)
        .await
    }

    async fn publish_kskgen_request(&self, request: KskgenRequest) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO kskgen_requests VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            request.preKskId.as_le_slice(),
            request.sourceKeyId.as_le_slice(),
            request.destKeyId.as_le_slice(),
            request.fheParamsDigest.as_slice(),
        )
        .execute(&self.db_pool)
        .await
    }

    async fn publish_crsgen_request(&self, request: CrsgenRequest) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO crsgen_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
            request.crsgenRequestId.as_le_slice(),
            request.fheParamsDigest.as_slice(),
        )
        .execute(&self.db_pool)
        .await
    }
}
