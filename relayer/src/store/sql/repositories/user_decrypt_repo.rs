use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::core::event::UserDecryptResponse;
use crate::metrics;
use crate::store::sql::models::req_status_enum_model::ReqStatus;
use crate::store::sql::models::user_decrypt_req_model::{ConsensusReqState, UserDecryptReqData};
use crate::store::sql::{
    client::PgClient,
    error::{SqlError, SqlResult},
    models::{
        user_decrypt_req_model::{UserDecryptResponseModel, UserDecryptResponseShare},
        user_decrypt_share_model::{ShareInsertParams, UserDecryptShare},
    },
    repositories::utils::compute_advisory_lock_id,
};
use alloy::primitives::U256;
use sqlx::types::{Json, Uuid};
use std::str::FromStr;
use std::time::Instant;

// Import conversion functions privately within this repository
use crate::store::sql::conversion::u256_to_i32;

/// Outcome of inserting a share and checking for threshold completion.
#[derive(Debug)]
pub enum ShareCompletionOutcome {
    /// Threshold not yet reached, request still in progress
    ThresholdNotReached { count: i64 },
    /// Threshold reached and request completed successfully in this operation
    Completed {
        count: i64,
        metadata: ConsensusReqState,
        shares: Vec<UserDecryptShare>,
    },
    /// Threshold reached but request is already completed (duplicate shares)
    /// Includes the completed data so caller can re-dispatch response if needed
    AlreadyCompleted {
        count: i64,
        metadata: ConsensusReqState,
        shares: Vec<UserDecryptShare>,
    },
    /// Threshold reached but request is already in a final failure state
    AlreadyInFinalState {
        count: i64,
        current_status: ReqStatus,
    },
}

pub enum UserDecryptInsertResult {
    /// New request inserted into DB
    Inserted { ext_job_id: Uuid },
    /// Duplicate request that already completed
    DuplicateCompleted {
        ext_job_id: Uuid,
        response: UserDecryptResponse,
    },
    /// Duplicate request still being processed
    DuplicateProcessing { ext_job_id: Uuid },
}

pub struct UserDecryptRepository {
    pool: PgClient,
}

impl UserDecryptRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // NOTE: We have a query which is performed at the database level in a pg_cron job instead of being called by the internals. and is triggered on this condition:
    // If status == 'receipt_received' and now - `updated_at` > 30 min roughly (TBD.)
    // Update status to timed_out with configured timeout message.
    // OR IN THE TIMEOUT REPO.

    /* NOTE: max size for indexes
        B-Tree (Default)	~2,704 bytes	Used for your UNIQUE indexes (int_job_id).
        Hash (USING HASH)	Unlimited (1 GB)	Used for your non-unique lookups (ext_job_id).
    */

    // GENERAL REQUESTS.

    /// Check for an existing *active* request (not failed, not timed_out).
    /// Returns the ext_job_id (Uuid) if found.
    /// Returns None if the request doesn't exist OR if it exists but is in a terminal failure state.
    pub async fn find_active_ext_ref_by_int_job_id(
        &self,
        int_job_id_bytes: &[u8],
    ) -> SqlResult<Option<Uuid>> {
        let query_start = Instant::now();

        let result = sqlx::query_scalar!(
            r#"
            SELECT ext_job_id
            FROM user_decrypt_req
            WHERE int_job_id = $1
              AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            LIMIT 1
            "#,
            int_job_id_bytes
        )
        .fetch_optional(&self.pool.get_app_pool())
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        Ok(result?)
    }

    /// Insert req, ext_job_id, int_job_id.
    /// Returns an enum indicating whether the request was inserted or was a duplicate.
    /// For duplicates, includes the current state (completed with response, or still processing).
    pub async fn insert_data_on_conflict_and_get_ext_job_id(
        &self,
        ext_job_id: Uuid,
        int_job_id_bytes: &[u8],
        request_data: UserDecryptReqData,
    ) -> SqlResult<UserDecryptInsertResult> {
        // Use a transaction to ensure atomic read of status + shares for completed duplicates.
        // This prevents race conditions where shares could be deleted between the INSERT
        // and the subsequent SELECT query.
        let mut tx = self.pool.get_app_pool().begin().await?;

        // Convert typed data to JSON Value and extract type
        let req_type = request_data.req_type();
        let request = request_data.to_value().map_err(|e| {
            SqlError::conversion_error(
                "UserDecryptReqData",
                "Value",
                format!("Failed to serialize request data: {}", e),
            )
        })?;

        let query_start = Instant::now();
        // Logic: Use (xmax=0) to detect if this was a true INSERT or an ON CONFLICT update.
        let result = sqlx::query!(
            r#"
            INSERT INTO user_decrypt_req (
                ext_job_id,
                int_job_id,
                req,
                req_type
            )
            VALUES ($1, $2, $3, $4::user_decrypt_req_type)
            ON CONFLICT (int_job_id)
            WHERE req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            DO UPDATE SET updated_at = user_decrypt_req.updated_at
            RETURNING ext_job_id, (xmax = 0) AS "is_inserted!", req_status AS "req_status!: ReqStatus", gw_reference_id
            "#,
            ext_job_id,
            int_job_id_bytes,
            request,
            req_type as _,
        )
        .fetch_one(&mut *tx)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        let record = result?;

        // Match on the state and return appropriate enum variant
        let insert_result = match (record.is_inserted, record.req_status) {
            (true, _) => {
                // New request inserted
                metrics::increment_req_status_count(
                    metrics::RequestType::UserDecrypt,
                    ReqStatus::Queued,
                );
                UserDecryptInsertResult::Inserted {
                    ext_job_id: record.ext_job_id,
                }
            }
            (false, ReqStatus::Completed) => {
                // Duplicate, already completed - fetch shares to assemble response
                let gw_reference_id = record.gw_reference_id.ok_or_else(|| {
                    SqlError::conversion_error(
                        "gw_reference_id",
                        "BYTEA",
                        "completed request missing gw_reference_id".to_string(),
                    )
                })?;

                // Second query: Fetch shares from user_decrypt_share table
                // Uses same transaction to ensure atomic read of status + shares
                let query_start = Instant::now();
                let shares_result = sqlx::query_as::<_, UserDecryptShare>(
                    r#"
                    SELECT id, gw_reference_id, tx_hash, share_index, share, kms_signature, extra_data, created_at, updated_at
                    FROM user_decrypt_share
                    WHERE gw_reference_id = $1
                    ORDER BY share_index
                    "#,
                )
                .bind(&gw_reference_id)
                .fetch_all(&mut *tx)
                .await;

                match &shares_result {
                    Ok(_) => metrics::observe_query(
                        metrics::Table::UserDecryptShares,
                        query_start.elapsed(),
                    ),
                    Err(_) => metrics::increment_error(metrics::Table::UserDecryptShares),
                }

                let shares = shares_result?;

                if shares.is_empty() {
                    return Err(SqlError::conversion_error(
                        "shares",
                        "Vec<UserDecryptShare>",
                        "completed request has no shares in user_decrypt_share table".to_string(),
                    ));
                }

                // Assemble UserDecryptResponse from shares
                let gateway_request_id = U256::from_be_slice(&gw_reference_id);
                let reencrypted_shares = shares
                    .iter()
                    .map(|s| alloy::primitives::Bytes::from_str(&s.share))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| {
                        SqlError::conversion_error(
                            "share",
                            "Bytes",
                            format!("Failed to parse share: {}", e),
                        )
                    })?;
                let signatures = shares
                    .iter()
                    .map(|s| alloy::primitives::Bytes::from_str(&s.kms_signature))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| {
                        SqlError::conversion_error(
                            "kms_signature",
                            "Bytes",
                            format!("Failed to parse signature: {}", e),
                        )
                    })?;
                let extra_data = shares.first().unwrap().extra_data.clone();

                let response = UserDecryptResponse {
                    gateway_request_id,
                    reencrypted_shares,
                    signatures,
                    extra_data,
                };

                UserDecryptInsertResult::DuplicateCompleted {
                    ext_job_id: record.ext_job_id,
                    response,
                }
            }
            (false, _) => {
                // Duplicate, still processing (queued, processing, etc.)
                UserDecryptInsertResult::DuplicateProcessing {
                    ext_job_id: record.ext_job_id,
                }
            }
        };

        tx.commit().await?;
        Ok(insert_result)
    }

    // GW READINESS LOGIC CHECK.
    /// update user_decrypt_req by int_job_id for to req_status processing
    /// Update req_status to 'processing' by int_job_id.
    /// Returns the number of rows affected (1 if found, 0 if not).
    pub async fn update_status_to_processing(&self, int_job_id_bytes: &[u8]) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM user_decrypt_req
                WHERE int_job_id = $1
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE user_decrypt_req
                SET req_status = 'processing'::req_status
                WHERE int_job_id = $1
                  AND req_status = 'queued'::req_status
                RETURNING req_status, updated_at
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            int_job_id_bytes
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::UserDecrypt,
                r.old_status,
                ReqStatus::Processing,
                r.old_updated_at,
                r.new_updated_at,
            );
            Ok(1)
        } else {
            Ok(0)
        }
    }

    // if not ready after 30min..
    /// Update req_status to 'timed_out' and set err_reason by int_job_id.
    /// Returns the number of rows affected (1 if found, 0 if not).
    pub async fn update_status_to_timed_out(
        &self,
        int_job_id_bytes: &[u8],
        err_reason: &str,
    ) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM user_decrypt_req
                WHERE int_job_id = $2
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE user_decrypt_req
                SET
                    req_status = 'timed_out'::req_status,
                    err_reason = $1
                WHERE int_job_id = $2
                  AND req_status IN ('queued'::req_status, 'receipt_received'::req_status)
                RETURNING req_status, updated_at
            )
            SELECT 
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            err_reason,
            int_job_id_bytes
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::UserDecrypt,
                r.old_status,
                ReqStatus::TimedOut,
                r.old_updated_at,
                r.new_updated_at,
            );
            Ok(1)
        } else {
            Ok(0)
        }
    }

    // TRANSACTION REQUESTS.

    /// Update req_status to 'tx_in_flight' by int_job_id.
    /// Returns the number of rows affected (1 if found, 0 if not).
    pub async fn update_status_to_tx_in_flight(&self, int_job_id_bytes: &[u8]) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM user_decrypt_req
                WHERE int_job_id = $1
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE user_decrypt_req
                SET req_status = 'tx_in_flight'::req_status
                WHERE int_job_id = $1
                  AND req_status = 'processing'::req_status
                RETURNING req_status, updated_at
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            int_job_id_bytes
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::UserDecrypt,
                r.old_status,
                ReqStatus::TxInFlight,
                r.old_updated_at,
                r.new_updated_at,
            );
            Ok(1)
        } else {
            Ok(0)
        }
    }

    /// Reset all tx_in_flight requests to processing status.
    /// Used during startup recovery to ensure clean state transitions.
    /// Returns the number of rows affected.
    pub async fn reset_tx_in_flight_to_processing(&self) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();

        // Fetch rows to update for metrics
        let rows = sqlx::query!(
            r#"
            SELECT int_job_id, updated_at
            FROM user_decrypt_req
            WHERE req_status = 'tx_in_flight'::req_status
            "#
        )
        .fetch_all(&mut *conn)
        .await;

        match &rows {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        let rows = rows?;
        if rows.is_empty() {
            return Ok(0);
        }

        // Perform bulk update (updated_at set by trigger)
        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET req_status = 'processing'::req_status
            WHERE req_status = 'tx_in_flight'::req_status
            "#
        )
        .execute(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        let rows_affected = result?.rows_affected();

        // Update metrics: decrement tx_in_flight, increment processing
        for _ in 0..rows_affected {
            metrics::record_status_transition(
                metrics::RequestType::UserDecrypt,
                ReqStatus::TxInFlight,
                ReqStatus::Processing,
                chrono::Utc::now(),
                chrono::Utc::now(),
            );
        }

        Ok(rows_affected)
    }

    /// Updating the req_status to receipt_received, gw_req_tx_hash, gw_reference_id by int_job_id
    /// Returns the number of rows affected (should be 1 or retry).
    pub async fn update_status_to_receipt_received_on_tx_success(
        &self,
        int_job_id_bytes: &[u8],
        gw_req_tx_hash: &str,
        gw_reference_id: U256,
    ) -> SqlResult<u64> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();

        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM user_decrypt_req
                WHERE int_job_id = $3
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE user_decrypt_req
                SET
                    req_status = 'receipt_received'::req_status,
                    gw_req_tx_hash = $1,
                    gw_reference_id = $2
                WHERE int_job_id = $3
                  AND req_status = 'tx_in_flight'::req_status
                RETURNING req_status, updated_at
            )
            SELECT 
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            gw_req_tx_hash,
            gw_ref_id,
            int_job_id_bytes
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::UserDecrypt,
                r.old_status,
                ReqStatus::ReceiptReceived,
                r.old_updated_at,
                r.new_updated_at,
            );
            Ok(1)
        } else {
            Ok(0)
        }
    }

    /// Update req_status to 'failure' from 'queued' state.
    /// Used when failures happen before the request reaches 'processing'
    /// (e.g., readiness check contract errors, enqueue failures).
    pub async fn update_status_to_failure_from_queued(
        &self,
        int_job_id_bytes: &[u8],
        err_reason: &str,
    ) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM user_decrypt_req
                WHERE int_job_id = $2
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE user_decrypt_req
                SET
                    req_status = 'failure'::req_status,
                    err_reason = $1
                WHERE int_job_id = $2
                  AND req_status = 'queued'::req_status
                RETURNING req_status, updated_at
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            err_reason,
            int_job_id_bytes
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::UserDecrypt,
                r.old_status,
                ReqStatus::Failure,
                r.old_updated_at,
                r.new_updated_at,
            );
            Ok(1)
        } else {
            Ok(0)
        }
    }

    /// update req_status to failure and apply err_reason by internal_indexer_id
    pub async fn update_status_to_failure_on_tx_failed(
        &self,
        int_job_id_bytes: &[u8],
        err_reason: &str,
    ) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM user_decrypt_req
                WHERE int_job_id = $2
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE user_decrypt_req
                SET
                    req_status = 'failure'::req_status,
                    err_reason = $1
                WHERE int_job_id = $2
                  AND req_status IN ('processing'::req_status, 'tx_in_flight'::req_status)
                RETURNING req_status, updated_at
            )
            SELECT 
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            err_reason,
            int_job_id_bytes
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::UserDecrypt,
                r.old_status,
                ReqStatus::Failure,
                r.old_updated_at,
                r.new_updated_at,
            );
            Ok(1)
        } else {
            Ok(0)
        }
    }

    // LISTENER REQUESTS.
    // If we receive consensus reached tx:
    // update user_decrypt_req for gw_consensus_tx_hash by gw_reference_id
    // and if consensus_tx_hash = null and if status = 'receipt_received'
    // but in any case return (status, updated_at, err_reason, int_job_id) for `gw_reference_id`
    /// Attempts to update consensus hash ONLY IF status is 'receipt_received' AND hash is null.
    /// ALWAYS returns the current state of the row (req_status, updated_at, etc.) regardless of update success.
    /// Step 6: Handle Consensus Tx.
    pub async fn update_consensus_hash_and_return_state(
        &self,
        gw_reference_id: U256,
        gw_consensus_tx_hash: &str,
    ) -> SqlResult<Option<ConsensusReqState>> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();

        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query_as!(
            ConsensusReqState,
            r#"
            WITH target AS (
                SELECT id FROM user_decrypt_req WHERE gw_reference_id = $2
            ),
            updated_row AS (
                UPDATE user_decrypt_req
                SET gw_consensus_tx_hash = $1
                WHERE id = (SELECT id FROM target)
                  AND gw_consensus_tx_hash IS NULL
                  AND req_status IN ('receipt_received'::req_status, 'completed'::req_status)
                RETURNING req_status, updated_at, err_reason, int_job_id
            )
            -- 1. Select from updated_row
            SELECT
                req_status as "req_status!: ReqStatus",
                updated_at as "updated_at!",
                err_reason,
                int_job_id as "int_job_id!"
            FROM updated_row

            UNION ALL

            -- 2. Select from original table
            SELECT
                req_status as "req_status!: ReqStatus",
                updated_at as "updated_at!",
                err_reason,
                int_job_id as "int_job_id!"
            FROM user_decrypt_req
            WHERE id = (SELECT id FROM target)
              AND NOT EXISTS (SELECT 1 FROM updated_row)
            "#,
            gw_consensus_tx_hash,
            gw_ref_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        Ok(result?)
    }

    // We receive a share event from the gw.
    /// Insert share and atomically complete request if threshold is reached.
    /// Returns outcome that explicitly indicates whether threshold was reached,
    /// completion succeeded, or request is already in a final state.
    ///
    /// This prevents race conditions between share insertion and request completion
    /// by performing all operations within a single atomic transaction.
    pub async fn insert_share_and_complete_if_threshold_reached(
        &self,
        params: ShareInsertParams<'_>,
        threshold: u32,
    ) -> SqlResult<ShareCompletionOutcome> {
        // u32 → i64: safe widening for DB BIGINT column
        let threshold = i64::from(threshold);
        let id_as_bytes_array: [u8; 32] = params.gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();
        let share_index = u256_to_i32(params.share_index)
            .map_err(|e| SqlError::conversion_error("share_index", params.share_index, e))?;

        // Use advisory locks to serialize share operations per gw_reference_id
        //
        // Advisory locks provide two benefits:
        // 1. Prevents wasted sequence IDs:
        //    Without serialization, concurrent INSERT attempts hitting UNIQUE constraints:
        //    - Multiple transactions call nextval() concurrently
        //    - All but one fail due to UNIQUE constraint on (gw_reference_id, share_index)
        //    - Failed transactions waste their sequence IDs, creating gaps
        //    See: https://www.postgresql.org/docs/current/functions-sequence.html
        //
        // 2. Ensures correct counts by serializing inserts:
        //    When transactions execute sequentially, COUNT(*) sees all previous INSERTTs
        //    for the same gw_reference_id, providing accurate share counts.
        //
        // 3. Prevents race conditions between threshold check and completion:
        //    By including completion logic in the same transaction, we prevent pg_cron
        //    timeout jobs from interfering between share insertion and request completion.
        //
        // Advisory lock automatically released when transaction commits/rollbacks.
        // See: https://www.postgresql.org/docs/current/explicit-locking.html#ADVISORY-LOCKS
        let mut tx = match self.pool.get_app_pool().begin().await {
            Ok(tx) => tx,
            Err(e) => {
                // Failed to acquire connection
                metrics::increment_error(metrics::Table::UserDecryptReq);
                return Err(e.into());
            }
        };

        // 2. METRIC: Start Query Timer
        // We start measuring the actual DB work only after we have the connection.
        let query_start = Instant::now();

        // 3. Execute Business Logic inside the Transaction
        let result: SqlResult<ShareCompletionOutcome> = async {
            // A. Acquire advisory lock (WAITS for other transactions)
            let lock_id = compute_advisory_lock_id(&gw_ref_id);
            sqlx::query!("SELECT pg_advisory_xact_lock($1)", lock_id)
                .execute(&mut *tx)
                .await?;

            // First, do the INSERT
            sqlx::query!(
                r#"
                INSERT INTO user_decrypt_share (
                    gw_reference_id,
                    tx_hash,
                    share_index,
                    share,
                    kms_signature,
                    extra_data
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (gw_reference_id, share_index) DO NOTHING
                "#,
                gw_ref_id,
                params.tx_hash,
                share_index,
                params.share,
                params.kms_signature,
                params.extra_data
            )
            .execute(&mut *tx)
            .await?;

            // Count shares within the same transaction
            let count = sqlx::query_scalar!(
                r#"
                SELECT COUNT(*) as "count!"
                FROM user_decrypt_share
                WHERE gw_reference_id = $1
                "#,
                gw_ref_id
            )
            .fetch_one(&mut *tx)
            .await?;

           // If threshold reached, complete the request atomically
            if count == threshold {
                // Query both old status and attempt update in single atomic operation
                #[derive(sqlx::FromRow, Debug)]
                struct ThresholdCheckResult {
                    old_status: ReqStatus,
                    old_updated_at: DateTime<Utc>,
                    int_job_id: Option<Vec<u8>>,
                    new_status: Option<ReqStatus>,
                    updated_at: Option<DateTime<Utc>>,
                    err_reason: Option<String>,
                }

                let check_result: Option<ThresholdCheckResult> = sqlx::query_as(
                    r#"
                    WITH old AS (
                        SELECT req_status as old_status, updated_at as old_updated_at FROM user_decrypt_req
                        WHERE gw_reference_id = $1
                    ),
                    upd AS (
                        UPDATE user_decrypt_req
                        SET req_status = 'completed'::req_status,
                            resolved_threshold = $2
                        WHERE gw_reference_id = $1
                          AND req_status = 'receipt_received'::req_status
                        RETURNING int_job_id, req_status as new_status, updated_at, err_reason
                    )
                    SELECT
                        old.old_status,
                        old.old_updated_at,
                        upd.int_job_id,
                        upd.new_status,
                        upd.updated_at,
                        upd.err_reason
                    FROM old
                    LEFT JOIN upd ON true
                    "#
                )
                .bind(&gw_ref_id)
                .bind(threshold)
                .fetch_optional(&mut *tx)
                .await?;

                match check_result {
                    Some(result) if result.int_job_id.is_some() => {
                        // Update succeeded - request was in receipt_received and is now completed
                        let int_job_id = result.int_job_id.unwrap();
                        let new_status = result.new_status.unwrap();
                        let updated_at = result.updated_at.unwrap();

                        // Record transition immediately (in-memory metric, safe to do inside tx flow)
                        metrics::record_status_transition(
                            metrics::RequestType::UserDecrypt,
                            result.old_status,
                            ReqStatus::Completed,
                            result.old_updated_at,
                            updated_at,
                        );

                        let metadata = ConsensusReqState {
                            int_job_id,
                            req_status: new_status,
                            updated_at,
                            err_reason: result.err_reason,
                        };

                        // Fetch shares ordered by creation time, limited to threshold
                        let shares: Vec<UserDecryptShare> = sqlx::query_as(
                            r#"
                            SELECT id, gw_reference_id, tx_hash, share_index, share, kms_signature, extra_data, created_at, updated_at
                            FROM user_decrypt_share
                            WHERE gw_reference_id = $1
                            ORDER BY created_at ASC, share_index ASC
                            LIMIT $2
                            "#
                        )
                        .bind(&gw_ref_id)
                        .bind(threshold)
                        .fetch_all(&mut *tx)
                        .await?;

                        tx.commit().await?;

                        Ok(ShareCompletionOutcome::Completed {
                            count,
                            metadata,
                            shares,
                        })
                    }
                    Some(result) => {
                        // Update failed - request is in a final state (not receipt_received)

                        // If already completed, fetch the shares and return AlreadyCompleted
                        if result.old_status == ReqStatus::Completed {
                            // Fetch shares for the completed request
                            let shares: Vec<UserDecryptShare> = sqlx::query_as(
                                r#"
                                SELECT id, gw_reference_id, tx_hash, share_index, share, kms_signature, extra_data, created_at, updated_at
                                FROM user_decrypt_share
                                WHERE gw_reference_id = $1
                                ORDER BY created_at ASC, share_index ASC
                                LIMIT $2
                                "#
                            )
                            .bind(&gw_ref_id)
                            .bind(threshold)
                            .fetch_all(&mut *tx)
                            .await?;

                            // Fetch request metadata
                            #[derive(sqlx::FromRow)]
                            struct CompletedMetadata {
                                int_job_id: Vec<u8>,
                                req_status: ReqStatus,
                                updated_at: DateTime<Utc>,
                                err_reason: Option<String>,
                            }

                            let metadata_row: Option<CompletedMetadata> = sqlx::query_as(
                                r#"
                                SELECT int_job_id, req_status, updated_at, err_reason
                                FROM user_decrypt_req
                                WHERE gw_reference_id = $1
                                "#
                            )
                            .bind(&gw_ref_id)
                            .fetch_optional(&mut *tx)
                            .await?;

                            tx.commit().await?;

                            match metadata_row {
                                Some(meta) => {
                                    let metadata = ConsensusReqState {
                                        int_job_id: meta.int_job_id,
                                        req_status: meta.req_status,
                                        updated_at: meta.updated_at,
                                        err_reason: meta.err_reason,
                                    };

                                    Ok(ShareCompletionOutcome::AlreadyCompleted {
                                        count,
                                        metadata,
                                        shares,
                                    })
                                }
                                None => {
                                    Err(SqlError::Transaction(
                                        "Request disappeared while fetching completed data".to_string(),
                                    ))
                                }
                            }
                        } else {
                            // Other final states (failure, timed_out, etc.)
                            tx.commit().await?;

                            Ok(ShareCompletionOutcome::AlreadyInFinalState {
                                count,
                                current_status: result.old_status,
                            })
                        }
                    }
                    None => {
                        // Request doesn't exist in database - should not happen
                        tx.rollback().await?;
                        Err(SqlError::Transaction(
                            "Request not found when threshold reached".to_string(),
                        ))
                    }
                }
            } else {
                // Threshold not yet reached
                tx.commit().await?;

                Ok(ShareCompletionOutcome::ThresholdNotReached { count })
            }
        }
        .await;

        // 4. METRIC: Record Query Duration / Error
        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        result
    }

    // GET REQUESTS RESULTS.
    /// Select in user_decrypt_req by ext_job_id and get all the shares on gw_reference_id.
    ///
    /// The share LIMIT uses `COALESCE(resolved_threshold, $2)`: if the dynamic threshold
    /// was stored at completion time, it takes precedence; otherwise the static fallback
    /// `$2` is used (backward compatibility for rows created before the migration).
    pub async fn find_req_and_shares_by_ext_job_id(
        &self,
        ext_job_id: Uuid,
        fallback_threshold: u32,
    ) -> SqlResult<Option<UserDecryptResponseModel>> {
        // u32 → i64: safe widening for DB BIGINT column
        let fallback_threshold = i64::from(fallback_threshold);
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query_as!(
            UserDecryptResponseModel,
            r#"
            SELECT
                r.ext_job_id,
                r.req_status as "req_status!: ReqStatus", -- Force non-null Enum type
                r.updated_at,
                r.err_reason,
                r.gw_req_tx_hash,
                r.gw_consensus_tx_hash,
                r.resolved_threshold,
                -- Aggregate shares into a JSON List.
                -- If no shares exist, return an empty JSON array '[]'
                -- Only select needed fields to avoid BYTEA deserialization issues
                COALESCE(
                    jsonb_agg(
                        jsonb_build_object(
                            'share', s.share,
                            'kms_signature', s.kms_signature,
                            'extra_data', s.extra_data
                        )
                        ORDER BY s.share_index
                    )
                    FILTER (WHERE s.id IS NOT NULL),
                    '[]'::jsonb
                ) as "shares!: Json<Vec<UserDecryptResponseShare>>"
            FROM user_decrypt_req r
            LEFT JOIN (
                SELECT * FROM user_decrypt_share
                WHERE gw_reference_id IN (
                    SELECT gw_reference_id FROM user_decrypt_req WHERE ext_job_id = $1
                )
                ORDER BY created_at ASC, share_index ASC
                LIMIT COALESCE(
                    (SELECT resolved_threshold FROM user_decrypt_req WHERE ext_job_id = $1),
                    $2
                )
            ) s ON r.gw_reference_id = s.gw_reference_id
            WHERE r.ext_job_id = $1
            GROUP BY r.id
            "#,
            ext_job_id,
            fallback_threshold
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }

        Ok(result?)
    }

    /// Find incomplete requests for startup recovery (queued, processing, tx_in_flight).
    pub async fn find_incomplete_requests(
        &self,
    ) -> SqlResult<Vec<(Vec<u8>, Value, ReqStatus, DateTime<Utc>)>> {
        let result = sqlx::query!(
            r#"
            SELECT int_job_id, req, req_status as "req_status!: ReqStatus", updated_at
            FROM user_decrypt_req
            WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status)
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(&self.pool.get_app_pool())
        .await?;

        Ok(result
            .into_iter()
            .map(|row| (row.int_job_id, row.req, row.req_status, row.updated_at))
            .collect())
    }

    pub async fn count_by_status(&self) -> SqlResult<Vec<(ReqStatus, i64)>> {
        let result = sqlx::query!(
            r#"
            SELECT req_status as "req_status!: ReqStatus", COUNT(*) as "count!"
            FROM user_decrypt_req
            GROUP BY req_status
            "#
        )
        .fetch_all(&self.pool.get_app_pool())
        .await?;

        Ok(result
            .into_iter()
            .map(|row| (row.req_status, row.count))
            .collect())
    }
}
