use alloy::primitives::U256;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    types::{
        CrsgenResponse, EpochResultResponse, KeygenResponse, KmsResponse, KmsResponseKind,
        NewKmsContextResponse, PrepKeygenResponse, ProtocolEvent, PublicDecryptionResponse,
        UserDecryptionResponse,
        db::{KeyDigestDbItem, OperationStatus, RequestSource},
    },
};
use kms_connector_api::ErrorCode;
use sqlx::{
    Pool, Postgres,
    postgres::PgQueryResult,
    types::chrono::{DateTime, Utc},
};
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

        let created_at = response.created_at;
        let otlp_context = response.otlp_context;
        let source = response.source;
        let query_result = match response.kind {
            KmsResponseKind::PublicDecryption(r) => {
                self.publish_public_decryption(r, created_at, otlp_context, source)
                    .await?
            }
            KmsResponseKind::UserDecryption(r) => {
                self.publish_user_decryption(r, created_at, otlp_context, source)
                    .await?
            }
            KmsResponseKind::PrepKeygen(r) => {
                self.publish_prep_keygen(r, created_at, otlp_context)
                    .await?
            }
            KmsResponseKind::Keygen(r) => self.publish_keygen(r, created_at, otlp_context).await?,
            KmsResponseKind::Crsgen(r) => self.publish_crsgen(r, created_at, otlp_context).await?,
            KmsResponseKind::NewKmsContext(r) => {
                self.publish_new_kms_context(r, created_at, otlp_context)
                    .await?
            }
            KmsResponseKind::EpochResult(r) => {
                self.publish_epoch_result(r, created_at, otlp_context)
                    .await?
            }
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
    /// Stores the response and completes the associated request row in a single statement.
    ///
    /// A retry may override a previous error row, but never a successful response.
    async fn publish_public_decryption(
        &self,
        response: PublicDecryptionResponse,
        created_at: DateTime<Utc>,
        otlp_ctx: PropagationContext,
        source: RequestSource,
    ) -> anyhow::Result<PgQueryResult> {
        // Single statement so the response row and the request status move atomically:
        //   1. upsert the response, but only if no successful payload exists yet (a retry may
        //      override a previous error row, but never a payload);
        //   2. mark the request `completed` only if the response row is actually written.
        // `rows_affected` reflects the outer UPDATE, so it is 1 iff the response was stored.
        sqlx::query!(
            "WITH written_response AS (
                INSERT INTO public_decryption_responses AS existing (
                    decryption_id, decrypted_result, signature, extra_data, created_at,
                    otlp_context, source, status
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (decryption_id) DO UPDATE SET
                    decrypted_result = EXCLUDED.decrypted_result,
                    signature = EXCLUDED.signature,
                    extra_data = EXCLUDED.extra_data,
                    created_at = EXCLUDED.created_at,
                    otlp_context = EXCLUDED.otlp_context,
                    source = EXCLUDED.source,
                    status = EXCLUDED.status,
                    error_code = NULL,
                    error_details = NULL
                WHERE existing.decrypted_result IS NULL
                RETURNING decryption_id
            )
            UPDATE public_decryption_requests AS request SET status = 'completed'
            FROM written_response
            WHERE request.decryption_id = written_response.decryption_id",
            response.decryption_id.as_le_slice(),
            response.decrypted_result,
            response.signature,
            response.extra_data,
            created_at,
            bc2wrap::serialize(&otlp_ctx)?,
            source as RequestSource,
            decryption_insert_status(source) as OperationStatus,
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    /// Stores the response and completes the associated request row in a single statement.
    ///
    /// A retry may override a previous error row, but never a successful response.
    async fn publish_user_decryption(
        &self,
        response: UserDecryptionResponse,
        created_at: DateTime<Utc>,
        otlp_ctx: PropagationContext,
        source: RequestSource,
    ) -> anyhow::Result<PgQueryResult> {
        // Single statement so the response row and the request status move atomically:
        //   1. upsert the response, but only if no successful payload exists yet (a retry may
        //      override a previous error row, never a payload);
        //   2. mark the request `completed` only for the response row is actually written.
        // `rows_affected` reflects the outer UPDATE, so it is 1 iff the response was stored.
        sqlx::query!(
            "WITH written_response AS (
                INSERT INTO user_decryption_responses AS existing (
                    decryption_id, user_decrypted_shares, signature, extra_data, created_at,
                    otlp_context, source, status
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (decryption_id) DO UPDATE SET
                    user_decrypted_shares = EXCLUDED.user_decrypted_shares,
                    signature = EXCLUDED.signature,
                    extra_data = EXCLUDED.extra_data,
                    created_at = EXCLUDED.created_at,
                    otlp_context = EXCLUDED.otlp_context,
                    source = EXCLUDED.source,
                    status = EXCLUDED.status,
                    error_code = NULL,
                    error_details = NULL
                WHERE existing.user_decrypted_shares IS NULL
                RETURNING decryption_id
            )
            UPDATE user_decryption_requests AS request SET status = 'completed'
            FROM written_response
            WHERE request.decryption_id = written_response.decryption_id",
            response.decryption_id.as_le_slice(),
            response.user_decrypted_shares,
            response.signature,
            response.extra_data,
            created_at,
            bc2wrap::serialize(&otlp_ctx)?,
            source as RequestSource,
            decryption_insert_status(source) as OperationStatus,
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_prep_keygen(
        &self,
        response: PrepKeygenResponse,
        created_at: DateTime<Utc>,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO prep_keygen_responses(prep_keygen_id, signature, created_at, otlp_context)
            VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            response.prep_keygen_id.as_le_slice(),
            response.signature,
            created_at,
            bc2wrap::serialize(&otlp_ctx)?
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_keygen(
        &self,
        response: KeygenResponse,
        created_at: DateTime<Utc>,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO keygen_responses(key_id, key_digests, signature, created_at, otlp_context)
            VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
            response.key_id.as_le_slice(),
            response.key_digests as Vec<KeyDigestDbItem>,
            response.signature,
            created_at,
            bc2wrap::serialize(&otlp_ctx)?
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_crsgen(
        &self,
        response: CrsgenResponse,
        created_at: DateTime<Utc>,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO crsgen_responses(crs_id, crs_digest, signature, created_at, otlp_context)
            VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
            response.crs_id.as_le_slice(),
            response.crs_digest,
            response.signature,
            created_at,
            bc2wrap::serialize(&otlp_ctx)?
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_new_kms_context(
        &self,
        response: NewKmsContextResponse,
        created_at: DateTime<Utc>,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO new_kms_context_responses(context_id, created_at, otlp_context)
            VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            response.context_id.as_le_slice(),
            created_at,
            bc2wrap::serialize(&otlp_ctx)?
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn publish_epoch_result(
        &self,
        response: EpochResultResponse,
        created_at: DateTime<Utc>,
        otlp_ctx: PropagationContext,
    ) -> anyhow::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO epoch_result_responses(
                context_id, epoch_id, keys, crs_list, created_at, otlp_context
            )
            VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
            response.context_id.as_le_slice(),
            response.epoch_id.as_le_slice(),
            response.keys,
            response.crs_list,
            created_at,
            bc2wrap::serialize(&otlp_ctx)?
        )
        .execute(&self.db_pool)
        .await
        .map_err(anyhow::Error::from)
    }

    /// Sets the `status` field of the event to `pending` in the database.
    pub async fn mark_event_as_pending(&self, event: &ProtocolEvent) -> anyhow::Result<()> {
        event.mark_as_pending(&self.db_pool).await
    }

    /// Sets the `status` field of the event to `failed` in the database.
    pub async fn mark_event_as_failed(&self, event: &ProtocolEvent) -> anyhow::Result<()> {
        event.mark_as_failed(&self.db_pool).await
    }

    /// Sets the `status` field of the event to `aborted` in the database.
    pub async fn mark_event_as_aborted(&self, event: &ProtocolEvent) -> anyhow::Result<()> {
        event.mark_as_aborted(&self.db_pool).await
    }

    /// Stores the rejection of an HTTP-sourced public decryption request.
    ///
    /// A retry may override a previous error row, but never a successful response.
    pub async fn publish_public_decryption_error(
        &self,
        decryption_id: U256,
        error_code: ErrorCode,
        error_details: &str,
        extra_data: &[u8],
        otlp_ctx: &PropagationContext,
    ) -> anyhow::Result<()> {
        // Same shape as `publish_public_decryption`: upsert the error row unless a successful
        // payload already exists, then mark the request `failed` only if the row was written.
        let query_result = sqlx::query!(
            "WITH written_response AS (
                INSERT INTO public_decryption_responses AS existing (
                    decryption_id, error_code, error_details, extra_data, created_at,
                    otlp_context, source, status
                )
                VALUES ($1, $2, $3, $4, $5, $6, 'http', 'completed')
                ON CONFLICT (decryption_id) DO UPDATE SET
                    error_code = EXCLUDED.error_code,
                    error_details = EXCLUDED.error_details,
                    extra_data = EXCLUDED.extra_data,
                    created_at = EXCLUDED.created_at,
                    otlp_context = EXCLUDED.otlp_context,
                    source = EXCLUDED.source,
                    status = EXCLUDED.status
                WHERE existing.decrypted_result IS NULL
                RETURNING decryption_id
            )
            UPDATE public_decryption_requests AS request SET status = 'failed'
            FROM written_response
            WHERE request.decryption_id = written_response.decryption_id",
            decryption_id.as_le_slice(),
            error_code.as_str(),
            error_details,
            extra_data,
            Utc::now(),
            bc2wrap::serialize(otlp_ctx)?,
        )
        .execute(&self.db_pool)
        .await?;
        log_error_publication_result(query_result, error_code);
        Ok(())
    }

    /// Stores the rejection of an HTTP-sourced user decryption request.
    ///
    /// A retry may override a previous error row, but never a successful response.
    pub async fn publish_user_decryption_error(
        &self,
        decryption_id: U256,
        error_code: ErrorCode,
        error_details: &str,
        extra_data: &[u8],
        otlp_ctx: &PropagationContext,
    ) -> anyhow::Result<()> {
        // Same shape as `publish_public_decryption`: upsert the error row unless a successful
        // payload already exists, then mark the request `failed` only if the row was written.
        let query_result = sqlx::query!(
            "WITH written_response AS (
                INSERT INTO user_decryption_responses AS existing (
                    decryption_id, error_code, error_details, extra_data, created_at,
                    otlp_context, source, status
                )
                VALUES ($1, $2, $3, $4, $5, $6, 'http', 'completed')
                ON CONFLICT (decryption_id) DO UPDATE SET
                    error_code = EXCLUDED.error_code,
                    error_details = EXCLUDED.error_details,
                    extra_data = EXCLUDED.extra_data,
                    created_at = EXCLUDED.created_at,
                    otlp_context = EXCLUDED.otlp_context,
                    source = EXCLUDED.source,
                    status = EXCLUDED.status
                WHERE existing.user_decrypted_shares IS NULL
                RETURNING decryption_id
            )
            UPDATE user_decryption_requests AS request SET status = 'failed'
            FROM written_response
            WHERE request.decryption_id = written_response.decryption_id",
            decryption_id.as_le_slice(),
            error_code.as_str(),
            error_details,
            extra_data,
            Utc::now(),
            bc2wrap::serialize(otlp_ctx)?,
        )
        .execute(&self.db_pool)
        .await?;
        log_error_publication_result(query_result, error_code);
        Ok(())
    }
}

fn decryption_insert_status(source: RequestSource) -> OperationStatus {
    match source {
        // Onchain-sourced decryption are pending until the tx-sender publishes them on-chain.
        RequestSource::OnChain => OperationStatus::Pending,
        // HTTP-sourced decryption have no on-chain step so they are marked as `completed`
        // immediately on insert.
        RequestSource::Http => OperationStatus::Completed,
    }
}

fn log_error_publication_result(query_result: PgQueryResult, error_code: ErrorCode) {
    if query_result.rows_affected() == 1 {
        info!(
            "Successfully stored `{}` error response in DB!",
            error_code.as_str()
        );
    } else {
        warn!(
            "Error response `{}` not stored: a successful response row already exists",
            error_code.as_str()
        );
    }
}
