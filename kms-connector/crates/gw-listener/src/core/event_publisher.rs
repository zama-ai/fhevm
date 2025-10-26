use crate::monitoring::metrics::EVENT_STORED_COUNTER;
use alloy::primitives::U256;
use anyhow::anyhow;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    types::{
        GatewayEvent, GatewayEventKind,
        db::{ParamsTypeDb, SnsCiphertextMaterialDbItem},
    },
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
    kms_generation::KMSGeneration::{CrsgenRequest, KeygenRequest, PrepKeygenRequest},
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
    #[tracing::instrument(skip_all)]
    async fn publish(&self, event: GatewayEvent) -> anyhow::Result<()> {
        info!("Storing {:?} in DB...", event.kind);

        let otlp_ctx = event.otlp_context;
        let query_result = match event.kind {
            GatewayEventKind::PublicDecryption(e) => {
                self.publish_public_decryption(e, otlp_ctx).await
            }
            GatewayEventKind::UserDecryption(e) => self.publish_user_decryption(e, otlp_ctx).await,
            GatewayEventKind::PrepKeygen(e) => self.publish_prep_keygen_request(e, otlp_ctx).await,
            GatewayEventKind::Keygen(e) => self.publish_keygen_request(e, otlp_ctx).await,
            GatewayEventKind::Crsgen(e) => self.publish_crsgen_request(e, otlp_ctx).await,
            GatewayEventKind::PrssInit(id) => self.publish_prss_init(id).await,
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
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        let sns_ciphertexts_db = request
            .snsCtMaterials
            .iter()
            .map(SnsCiphertextMaterialDbItem::from)
            .collect::<Vec<SnsCiphertextMaterialDbItem>>();

        sqlx::query!(
            "INSERT INTO public_decryption_requests(decryption_id, sns_ct_materials, extra_data, otlp_context) \
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            request.decryptionId.as_le_slice(),
            sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
            request.extraData.as_ref(),
            bc2wrap::serialize(&otlp_ctx)?,
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_user_decryption(
        &self,
        request: UserDecryptionRequest,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        let sns_ciphertexts_db = request
            .snsCtMaterials
            .iter()
            .map(SnsCiphertextMaterialDbItem::from)
            .collect::<Vec<SnsCiphertextMaterialDbItem>>();

        sqlx::query!(
            "INSERT INTO user_decryption_requests(\
                decryption_id, sns_ct_materials, user_address, public_key, extra_data, otlp_context\
            ) \
            VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
            request.decryptionId.as_le_slice(),
            sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
            request.userAddress.as_slice(),
            request.publicKey.as_ref(),
            request.extraData.as_ref(),
            bc2wrap::serialize(&otlp_ctx)?,
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_prep_keygen_request(
        &self,
        request: PrepKeygenRequest,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        let params_type: ParamsTypeDb = request.paramsType.try_into()?;
        sqlx::query!(
            "INSERT INTO prep_keygen_requests(prep_keygen_id, epoch_id, params_type, otlp_context) \
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            request.prepKeygenId.as_le_slice(),
            request.epochId.as_le_slice(),
            params_type as ParamsTypeDb,
            bc2wrap::serialize(&otlp_ctx)?,
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_keygen_request(
        &self,
        request: KeygenRequest,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO keygen_requests(prep_keygen_id, key_id, otlp_context) \
            VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            request.prepKeygenId.as_le_slice(),
            request.keyId.as_le_slice(),
            bc2wrap::serialize(&otlp_ctx)?,
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_crsgen_request(
        &self,
        request: CrsgenRequest,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        let params_type: ParamsTypeDb = request.paramsType.try_into()?;
        sqlx::query!(
            "INSERT INTO crsgen_requests(crs_id, max_bit_length, params_type, otlp_context) \
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            request.crsId.as_le_slice(),
            request.maxBitLength.as_le_slice(),
            params_type as ParamsTypeDb,
            bc2wrap::serialize(&otlp_ctx)?,
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_prss_init(&self, id: U256) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO prss_init(id) VALUES ($1) ON CONFLICT DO NOTHING",
            id.as_le_slice(),
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }
}
