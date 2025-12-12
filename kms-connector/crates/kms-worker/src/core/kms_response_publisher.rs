use connector_utils::{
    monitoring::otlp::PropagationContext,
    types::{
        CrsgenResponse, GatewayEvent, KeygenResponse, KmsResponse, KmsResponseKind,
        PrepKeygenResponse, PublicDecryptionResponse, UserDecryptionResponse, db::KeyDigestDbItem,
    },
};
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use tracing::{info, warn};

/// Interface used to publish KMS Core's responses in some storage.
pub trait KmsResponsePublisher {
    fn publish_response(
        &self,
        response: KmsResponse,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

/// Struct that stores KMS Core's responses in a `Postgres` database.
#[derive(Clone)]
pub struct DbKmsResponsePublisher {
    db_pool: Pool<Postgres>,
}

impl DbKmsResponsePublisher {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }
}

impl KmsResponsePublisher for DbKmsResponsePublisher {
    #[tracing::instrument(skip_all)]
    async fn publish_response(&self, response: KmsResponse) -> anyhow::Result<()> {
        info!("Storing response in DB...");

        let otlp_context = response.otlp_context;
        let query_result = match response.kind {
            KmsResponseKind::PublicDecryption(r) => {
                self.publish_public_decryption(r, otlp_context).await?
            }
            KmsResponseKind::UserDecryption(r) => {
                self.publish_user_decryption(r, otlp_context).await?
            }
            KmsResponseKind::PrepKeygen(r) => self.publish_prep_keygen(r, otlp_context).await?,
            KmsResponseKind::Keygen(r) => self.publish_keygen(r, otlp_context).await?,
            KmsResponseKind::Crsgen(r) => self.publish_crsgen(r, otlp_context).await?,
        };

        if query_result.rows_affected() == 1 {
            info!("Successfully stored response in DB!");
        } else {
            warn!("Unexpected query result while publishing response: {query_result:?}");
        }
        Ok(())
    }
}

impl DbKmsResponsePublisher {
    async fn publish_public_decryption(
        &self,
        response: PublicDecryptionResponse,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO public_decryption_responses(decryption_id, decrypted_result, signature, extra_data, otlp_context) \
            VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
            response.decryption_id.as_le_slice(),
            response.decrypted_result,
            response.signature,
            response.extra_data,
            bc2wrap::serialize(&otlp_ctx)?
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_user_decryption(
        &self,
        response: UserDecryptionResponse,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO user_decryption_responses(decryption_id, user_decrypted_shares, signature, extra_data, otlp_context) \
            VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
            response.decryption_id.as_le_slice(),
            response.user_decrypted_shares,
            response.signature,
            response.extra_data,
            bc2wrap::serialize(&otlp_ctx)?
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_prep_keygen(
        &self,
        response: PrepKeygenResponse,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO prep_keygen_responses(prep_keygen_id, signature, otlp_context) \
            VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            response.prep_keygen_id.as_le_slice(),
            response.signature,
            bc2wrap::serialize(&otlp_ctx)?
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_keygen(
        &self,
        response: KeygenResponse,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO keygen_responses(key_id, key_digests, signature, otlp_context) \
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            response.key_id.as_le_slice(),
            response.key_digests as Vec<KeyDigestDbItem>,
            response.signature,
            bc2wrap::serialize(&otlp_ctx)?
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_crsgen(
        &self,
        response: CrsgenResponse,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO crsgen_responses(crs_id, crs_digest, signature, otlp_context) \
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            response.crs_id.as_le_slice(),
            response.crs_digest,
            response.signature,
            bc2wrap::serialize(&otlp_ctx)?
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    /// Sets the `status` field of the event to `pending` in the database.
    pub async fn mark_event_as_pending(&self, event: GatewayEvent) {
        event.mark_as_pending(&self.db_pool).await
    }
}
