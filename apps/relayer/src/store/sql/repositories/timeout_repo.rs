use std::time::Instant;

use crate::{
    metrics,
    store::sql::{client::PgClient, models::req_status_enum_model::ReqStatus},
};
use anyhow::Result;

pub struct TimeoutRepository {
    pool: PgClient,
}

impl TimeoutRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // TODO: Make the timeout configurable from settings for each of the values here.

    /// Returns the total number of rows moved to 'timed_out'.
    /// Updates all requests that have been stuck in 'receipt_received' for > 30 minutes.
    pub async fn time_out_stale_requests(&self) -> Result<u64> {
        let mut total_affected = 0;
        const ERR_REASON: &str = "Gateway chain did not respond within the expected timeframe";

        // ---------------------------------------------------------------------
        // 1. User Decrypt
        // ---------------------------------------------------------------------
        {
            let mut conn = self.pool.get_connection().await?; // Metrics: Pool Wait
            let query_start = Instant::now();

            // We use a CTE to capture the 'old_updated_at' before the update happens.
            let result = sqlx::query!(
                r#"
                WITH stale_rows AS (
                    SELECT id, updated_at 
                    FROM user_decrypt_req
                    WHERE req_status = 'receipt_received'::req_status
                      AND updated_at < NOW() - INTERVAL '30 minutes'
                    FOR UPDATE SKIP LOCKED -- Prevent conflicts with other workers
                ),
                updated_rows AS (
                    UPDATE user_decrypt_req
                    SET req_status = 'timed_out'::req_status, 
                        err_reason = $1,
                        updated_at = NOW()
                    FROM stale_rows
                    WHERE user_decrypt_req.id = stale_rows.id
                    RETURNING user_decrypt_req.updated_at as new_updated_at, stale_rows.updated_at as old_updated_at
                )
                SELECT new_updated_at, old_updated_at FROM updated_rows
                "#,
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
            total_affected += rows.len() as u64;

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
            let mut conn = self.pool.get_connection().await?;
            let query_start = Instant::now();

            let result = sqlx::query!(
                r#"
                WITH stale_rows AS (
                    SELECT id, updated_at 
                    FROM public_decrypt_req
                    WHERE req_status = 'receipt_received'::req_status
                      AND updated_at < NOW() - INTERVAL '30 minutes'
                    FOR UPDATE SKIP LOCKED
                ),
                updated_rows AS (
                    UPDATE public_decrypt_req
                    SET req_status = 'timed_out'::req_status, 
                        err_reason = $1,
                        updated_at = NOW()
                    FROM stale_rows
                    WHERE public_decrypt_req.id = stale_rows.id
                    RETURNING public_decrypt_req.updated_at as new_updated_at, stale_rows.updated_at as old_updated_at
                )
                SELECT new_updated_at, old_updated_at FROM updated_rows
                "#,
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
            total_affected += rows.len() as u64;

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
            let mut conn = self.pool.get_connection().await?;
            let query_start = Instant::now();

            let result = sqlx::query!(
                r#"
                WITH stale_rows AS (
                    SELECT id, updated_at 
                    FROM input_proof_req
                    WHERE req_status = 'receipt_received'::req_status
                      AND updated_at < NOW() - INTERVAL '30 minutes'
                    FOR UPDATE SKIP LOCKED
                ),
                updated_rows AS (
                    UPDATE input_proof_req
                    SET req_status = 'timed_out'::req_status, 
                        err_reason = $1,
                        updated_at = NOW()
                    FROM stale_rows
                    WHERE input_proof_req.id = stale_rows.id
                    RETURNING input_proof_req.updated_at as new_updated_at, stale_rows.updated_at as old_updated_at
                )
                SELECT new_updated_at, old_updated_at FROM updated_rows
                "#,
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
            total_affected += rows.len() as u64;

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
