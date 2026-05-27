use crate::{
    core::tx_sender::Error, monitoring::metrics::register_solana_native_response_forwarding_latency,
};
use anyhow::anyhow;
use connector_utils::{monitoring::otlp::PropagationContext, types::db::OperationStatus};
use sqlx::{
    Pool, Postgres, Row,
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
};
use std::{future::Future, time::Duration};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

pub const SOLANA_NATIVE_DECRYPTION_RESPONSE_V0_STR: &str = "solana_native_decryption_response_v0";

#[derive(Clone, Debug, PartialEq)]
pub struct SolanaNativeResponseV0 {
    pub route: SolanaNativeResponseRouteV0,
    pub response_hash: [u8; 32],
    pub response_payload: Vec<u8>,
    pub raw_response_body: Vec<u8>,
    pub certificate: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub otlp_context: PropagationContext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeResponseRouteV0 {
    pub host_chain_id: u64,
    pub solana_cluster_id: [u8; 32],
    pub kms_context_id: [u8; 32],
    pub request_hash: [u8; 32],
    pub request_mode: u8,
    pub response_kind: u8,
    pub response_context: Vec<u8>,
}

/// Interface used to pick verified native-v0 Solana responses from storage.
pub trait SolanaNativeResponsePicker {
    fn pick_responses(
        &mut self,
    ) -> impl Future<Output = anyhow::Result<Vec<SolanaNativeResponseV0>>> + Send;
}

/// Interface used to publish a verified native-v0 Solana response to its final target.
///
/// The branch intentionally keeps this as a trait until the product publication target is fixed:
/// a Solana instruction, a relayer outbox, and a caller callback need different transport details
/// but the same DB claiming/status rules.
pub trait SolanaNativeResponsePublisher: Clone + Send + Sync + 'static {
    fn publish_response(
        &self,
        response: SolanaNativeResponseV0,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

#[derive(Clone, Debug)]
pub struct DbSolanaNativeResponsePicker {
    db_pool: Pool<Postgres>,
    responses_batch_size: u8,
    db_polling: Duration,
}

pub struct SolanaNativeTransactionSender<L, P>
where
    L: SolanaNativeResponsePicker,
    P: SolanaNativeResponsePublisher,
{
    response_picker: L,
    publisher: P,
    db_pool: Pool<Postgres>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SolanaNativeForwardDecision {
    RetryLater,
    FailPermanently,
    CancelAndRetryLater,
    Complete,
}

impl DbSolanaNativeResponsePicker {
    pub fn new(db_pool: Pool<Postgres>, responses_batch_size: u8, db_polling: Duration) -> Self {
        Self {
            db_pool,
            responses_batch_size,
            db_polling,
        }
    }

    pub async fn try_pick_pending_responses(&self) -> anyhow::Result<Vec<SolanaNativeResponseV0>> {
        sqlx::query(
            "\
            UPDATE solana_native_decryption_responses_v0
            SET status = 'under_process'
            FROM (
                SELECT request_hash
                FROM solana_native_decryption_responses_v0
                WHERE status = 'pending'
                ORDER BY updated_at ASC
                LIMIT $1 FOR UPDATE SKIP LOCKED
            ) AS resp
            WHERE solana_native_decryption_responses_v0.request_hash = resp.request_hash
            RETURNING
                resp.request_hash,
                host_chain_id,
                solana_cluster_id,
                kms_context_id,
                request_mode,
                response_kind,
                response_context,
                response_hash,
                response_payload,
                raw_response_body,
                certificate,
                created_at,
                otlp_context\
            ",
        )
        .bind(i16::from(self.responses_batch_size))
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(SolanaNativeResponseV0::from_row)
        .collect()
    }
}

impl SolanaNativeResponsePicker for DbSolanaNativeResponsePicker {
    async fn pick_responses(&mut self) -> anyhow::Result<Vec<SolanaNativeResponseV0>> {
        loop {
            let responses = self.try_pick_pending_responses().await?;
            if !responses.is_empty() {
                info!("Picked {} native Solana responses", responses.len());
                return Ok(responses);
            }
            tokio::time::sleep(self.db_polling).await;
        }
    }
}

impl<L, P> SolanaNativeTransactionSender<L, P>
where
    L: SolanaNativeResponsePicker,
    P: SolanaNativeResponsePublisher,
{
    pub fn new(response_picker: L, publisher: P, db_pool: Pool<Postgres>) -> Self {
        Self {
            response_picker,
            publisher,
            db_pool,
        }
    }

    pub async fn start(self, cancel_token: CancellationToken) {
        info!("Starting SolanaNativeTransactionSender");
        tokio::select! {
            _ = cancel_token.cancelled() => info!("SolanaNativeTransactionSender cancelled..."),
            _ = self.run(&cancel_token) => (),
        }
        info!("SolanaNativeTransactionSender stopped successfully!");
    }

    async fn run(mut self, cancel_token: &CancellationToken) {
        loop {
            match self.response_picker.pick_responses().await {
                Ok(responses) => {
                    self.spawn_response_forwarding_tasks(responses, cancel_token)
                        .await
                }
                Err(e) => break error!("Native Solana response picker is broken: {e}"),
            };
        }
    }

    async fn spawn_response_forwarding_tasks(
        &self,
        responses: Vec<SolanaNativeResponseV0>,
        cancel_token: &CancellationToken,
    ) {
        for response in responses {
            let publisher = self.publisher.clone();
            let db_pool = self.db_pool.clone();
            let cloned_cancel_token = cancel_token.clone();
            connector_utils::tasks::spawn_with_limit(async move {
                Self::forward_response(publisher, db_pool, response, cloned_cancel_token).await
            })
            .await;
        }
    }

    async fn forward_response(
        publisher: P,
        db_pool: Pool<Postgres>,
        response: SolanaNativeResponseV0,
        cancel_token: CancellationToken,
    ) {
        let result = publisher.publish_response(response.clone()).await;

        match SolanaNativeForwardDecision::from_publish_result(&result) {
            SolanaNativeForwardDecision::RetryLater => response.mark_as_pending(&db_pool).await,
            SolanaNativeForwardDecision::FailPermanently => response.mark_as_failed(&db_pool).await,
            SolanaNativeForwardDecision::CancelAndRetryLater => {
                response.mark_as_pending(&db_pool).await;
                cancel_token.cancel();
            }
            SolanaNativeForwardDecision::Complete => {
                response.mark_as_completed(&db_pool).await;
                register_solana_native_response_forwarding_latency(&response);
            }
        }
    }
}

impl SolanaNativeForwardDecision {
    fn from_publish_result(result: &Result<(), Error>) -> Self {
        match result {
            Err(Error::Recoverable(_)) => Self::RetryLater,
            Err(Error::Irrecoverable(_)) => Self::FailPermanently,
            Err(Error::AlloyBackendGone) => Self::CancelAndRetryLater,
            Ok(()) => Self::Complete,
        }
    }
}

impl SolanaNativeResponseV0 {
    fn from_row(row: &PgRow) -> anyhow::Result<Self> {
        Ok(Self {
            route: SolanaNativeResponseRouteV0 {
                host_chain_id: host_chain_id_from_db_bytes(row.try_get("host_chain_id")?)?,
                solana_cluster_id: fixed_bytes(row.try_get("solana_cluster_id")?)?,
                kms_context_id: fixed_bytes(row.try_get("kms_context_id")?)?,
                request_hash: fixed_bytes(row.try_get("request_hash")?)?,
                request_mode: byte_from_db_i16(row.try_get("request_mode")?, "request_mode")?,
                response_kind: byte_from_db_i16(row.try_get("response_kind")?, "response_kind")?,
                response_context: row.try_get("response_context")?,
            },
            response_hash: fixed_bytes(row.try_get("response_hash")?)?,
            response_payload: row.try_get("response_payload")?,
            raw_response_body: row.try_get("raw_response_body")?,
            certificate: row.try_get("certificate")?,
            created_at: row.try_get("created_at")?,
            otlp_context: bc2wrap::deserialize_safe(&row.try_get::<Vec<u8>, _>("otlp_context")?)?,
        })
    }

    pub async fn mark_as_pending(&self, db: &Pool<Postgres>) {
        warn!("Failed to publish native Solana response. Restoring status to pending in DB...");
        self.update_status(db, OperationStatus::Pending).await
    }

    pub async fn mark_as_completed(&self, db: &Pool<Postgres>) {
        info!("Native Solana response successfully published. Marking it completed in DB...");
        self.update_status(db, OperationStatus::Completed).await
    }

    pub async fn mark_as_failed(&self, db: &Pool<Postgres>) {
        warn!("Native Solana response failed permanently. Marking it failed in DB...");
        self.update_status(db, OperationStatus::Failed).await
    }

    async fn update_status(&self, db: &Pool<Postgres>, status: OperationStatus) {
        let query_result = sqlx::query(
            "\
            UPDATE solana_native_decryption_responses_v0
            SET status = $1
            WHERE request_hash = $2\
            ",
        )
        .bind(status)
        .bind(self.route.request_hash.as_slice())
        .execute(db)
        .await;

        match query_result {
            Ok(result) if result.rows_affected() == 1 => {
                info!("Successfully updated native Solana response in DB")
            }
            Ok(result) => warn!(
                "Unexpected query result while updating native Solana response: {:?}",
                result
            ),
            Err(e) => warn!("Failed to update native Solana response: {e}"),
        }
    }
}

fn fixed_bytes<const N: usize>(bytes: Vec<u8>) -> anyhow::Result<[u8; N]> {
    bytes
        .try_into()
        .map_err(|bytes: Vec<u8>| anyhow!("expected {N} bytes, got {}", bytes.len()))
}

fn host_chain_id_from_db_bytes(bytes: Vec<u8>) -> anyhow::Result<u64> {
    Ok(u64::from_le_bytes(fixed_bytes(bytes)?))
}

fn byte_from_db_i16(value: i16, field: &'static str) -> anyhow::Result<u8> {
    u8::try_from(value).map_err(|_| anyhow!("{field} is outside u8 range: {value}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_chain_id_decodes_little_endian_db_bytes() {
        assert_eq!(
            host_chain_id_from_db_bytes(900u64.to_le_bytes().to_vec()).unwrap(),
            900
        );
    }

    #[test]
    fn fixed_bytes_rejects_wrong_length() {
        let err = fixed_bytes::<32>(vec![0; 31]).unwrap_err();
        assert!(err.to_string().contains("expected 32 bytes"));
    }

    #[test]
    fn byte_from_db_i16_rejects_out_of_range_values() {
        let err = byte_from_db_i16(300, "response_kind").unwrap_err();
        assert!(
            err.to_string()
                .contains("response_kind is outside u8 range")
        );
    }

    #[test]
    fn publish_result_maps_to_native_forwarding_decision() {
        assert_eq!(
            SolanaNativeForwardDecision::from_publish_result(&Ok(())),
            SolanaNativeForwardDecision::Complete
        );
        assert_eq!(
            SolanaNativeForwardDecision::from_publish_result(&Err(Error::Recoverable(anyhow!(
                "retry"
            )))),
            SolanaNativeForwardDecision::RetryLater
        );
        assert_eq!(
            SolanaNativeForwardDecision::from_publish_result(&Err(Error::Irrecoverable(anyhow!(
                "stop"
            )))),
            SolanaNativeForwardDecision::FailPermanently
        );
        assert_eq!(
            SolanaNativeForwardDecision::from_publish_result(&Err(Error::AlloyBackendGone)),
            SolanaNativeForwardDecision::CancelAndRetryLater
        );
    }
}
