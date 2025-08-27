use crate::monitoring::metrics::EVENT_STORED_COUNTER;
use anyhow::anyhow;
use connector_utils::types::{
    GatewayEvent,
    db::{ParamsTypeDb, SnsCiphertextMaterialDbItem},
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
    kms_management::KmsManagement::{CrsgenRequest, KeygenRequest, PrepKeygenRequest},
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
            GatewayEvent::PrepKeygen(e) => self.publish_prep_keygen_request(e).await,
            GatewayEvent::Keygen(e) => self.publish_keygen_request(e).await,
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
    ) -> anyhow::Result<PgQueryResult> {
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
        .map_err(anyhow::Error::from)
    }

    async fn publish_user_decryption(
        &self,
        request: UserDecryptionRequest,
    ) -> anyhow::Result<PgQueryResult> {
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
        .map_err(anyhow::Error::from)
    }

    async fn publish_prep_keygen_request(
        &self,
        request: PrepKeygenRequest,
    ) -> anyhow::Result<PgQueryResult> {
        let params_type: ParamsTypeDb = request.paramsType.try_into()?;
        sqlx::query!(
            "INSERT INTO prep_keygen_requests VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            request.prepKeygenId.as_le_slice(),
            request.epochId.as_le_slice(),
            params_type as ParamsTypeDb,
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_keygen_request(
        &self,
        request: KeygenRequest,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO keygen_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
            request.prepKeygenId.as_le_slice(),
            request.keyId.as_le_slice(),
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_crsgen_request(
        &self,
        request: CrsgenRequest,
    ) -> anyhow::Result<PgQueryResult> {
        let params_type: ParamsTypeDb = request.paramsType.try_into()?;
        sqlx::query!(
            "INSERT INTO crsgen_requests VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            request.crsId.as_le_slice(),
            request.maxBitLength.as_le_slice(),
            params_type as ParamsTypeDb,
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }
}
