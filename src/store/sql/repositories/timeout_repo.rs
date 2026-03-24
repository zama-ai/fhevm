use std::time::Instant;

use crate::{
    config::settings::CronConfig,
    metrics,
    store::sql::{client::PgClient, models::req_status_enum_model::ReqStatus},
};
use anyhow::Result;

pub struct TimeoutRepository {
    pool: PgClient,
    cron_config: CronConfig,
}

impl TimeoutRepository {
    pub fn new(pool: PgClient, cron_config: CronConfig) -> Self {
        Self { pool, cron_config }
    }

    /// Returns the total number of rows moved to 'timed_out'.
    /// Updates all requests that have been stuck in 'receipt_received' for longer than configured timeout.
    pub async fn time_out_stale_requests(&self) -> Result<u64> {
        let mut total_affected = 0;
        const ERR_REASON: &str = "Gateway chain did not respond within the expected timeframe";

        // ---------------------------------------------------------------------
        // 1. User Decrypt
        // ---------------------------------------------------------------------
        {
            let mut conn = self.pool.get_cron_connection().await?; // Metrics: Pool Wait
            let query_start = Instant::now();

            let timeout_secs = self.cron_config.user_decrypt_timeout.as_secs_f64();

            // We use a CTE to capture the 'old_updated_at' before the update happens.
            let result = sqlx::query!(
                r#"
                WITH stale_rows AS (
                    SELECT id, updated_at
                    FROM user_decrypt_req
                    WHERE req_status = 'receipt_received'::req_status
                      AND updated_at < NOW() - make_interval(secs => $1)
                    FOR UPDATE SKIP LOCKED -- Prevent conflicts with other workers
                ),
                updated_rows AS (
                    UPDATE user_decrypt_req
                    SET req_status = 'timed_out'::req_status,
                        err_reason = $2,
                        updated_at = NOW()
                    FROM stale_rows
                    WHERE user_decrypt_req.id = stale_rows.id
                    RETURNING user_decrypt_req.updated_at as new_updated_at, stale_rows.updated_at as old_updated_at
                )
                SELECT new_updated_at, old_updated_at FROM updated_rows
                "#,
                timeout_secs,
                ERR_REASON
            )
            .fetch_all(&mut *conn)
            .await;

            // Metrics: Query Duration / Errors
            match &result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed())
                }
                Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
            }

            let rows = result?;
            let count = rows.len() as u64;
            total_affected += count;

            // Metrics: Status Transitions
            for row in rows {
                metrics::record_status_transition(
                    metrics::RequestType::UserDecrypt,
                    ReqStatus::ReceiptReceived, // Old status
                    ReqStatus::TimedOut,        // New status
                    row.old_updated_at,
                    row.new_updated_at,
                );
            }
        }

        // ---------------------------------------------------------------------
        // 2. Public Decrypt
        // ---------------------------------------------------------------------
        {
            let mut conn = self.pool.get_cron_connection().await?;
            let query_start = Instant::now();

            let timeout_secs = self.cron_config.public_decrypt_timeout.as_secs_f64();

            let result = sqlx::query!(
                r#"
                WITH stale_rows AS (
                    SELECT id, updated_at
                    FROM public_decrypt_req
                    WHERE req_status = 'receipt_received'::req_status
                      AND updated_at < NOW() - make_interval(secs => $1)
                    FOR UPDATE SKIP LOCKED
                ),
                updated_rows AS (
                    UPDATE public_decrypt_req
                    SET req_status = 'timed_out'::req_status,
                        err_reason = $2,
                        updated_at = NOW()
                    FROM stale_rows
                    WHERE public_decrypt_req.id = stale_rows.id
                    RETURNING public_decrypt_req.updated_at as new_updated_at, stale_rows.updated_at as old_updated_at
                )
                SELECT new_updated_at, old_updated_at FROM updated_rows
                "#,
                timeout_secs,
                ERR_REASON
            )
            .fetch_all(&mut *conn)
            .await;

            match &result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
                }
                Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
            }

            let rows = result?;
            let count = rows.len() as u64;
            total_affected += count;

            for row in rows {
                metrics::record_status_transition(
                    metrics::RequestType::PublicDecrypt,
                    ReqStatus::ReceiptReceived,
                    ReqStatus::TimedOut,
                    row.old_updated_at,
                    row.new_updated_at,
                );
            }
        }

        // ---------------------------------------------------------------------
        // 3. Input Proof
        // ---------------------------------------------------------------------
        {
            let mut conn = self.pool.get_cron_connection().await?;
            let query_start = Instant::now();

            let timeout_secs = self.cron_config.input_proof_timeout.as_secs_f64();

            let result = sqlx::query!(
                r#"
                WITH stale_rows AS (
                    SELECT id, updated_at
                    FROM input_proof_req
                    WHERE req_status = 'receipt_received'::req_status
                      AND updated_at < NOW() - make_interval(secs => $1)
                    FOR UPDATE SKIP LOCKED
                ),
                updated_rows AS (
                    UPDATE input_proof_req
                    SET req_status = 'timed_out'::req_status,
                        err_reason = $2,
                        updated_at = NOW()
                    FROM stale_rows
                    WHERE input_proof_req.id = stale_rows.id
                    RETURNING input_proof_req.updated_at as new_updated_at, stale_rows.updated_at as old_updated_at
                )
                SELECT new_updated_at, old_updated_at FROM updated_rows
                "#,
                timeout_secs,
                ERR_REASON
            )
            .fetch_all(&mut *conn)
            .await;

            match &result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed())
                }
                Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
            }

            let rows = result?;
            let count = rows.len() as u64;
            total_affected += count;

            for row in rows {
                metrics::record_status_transition(
                    metrics::RequestType::InputProof,
                    ReqStatus::ReceiptReceived,
                    ReqStatus::TimedOut,
                    row.old_updated_at,
                    row.new_updated_at,
                );
            }
        }

        Ok(total_affected)
    }
}
