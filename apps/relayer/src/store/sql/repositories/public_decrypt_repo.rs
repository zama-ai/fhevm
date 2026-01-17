use std::time::Instant;

use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::core::event::{PublicDecryptRequest, PublicDecryptResponse};
use crate::metrics;
use crate::store::sql::models::public_decrypt_req_model::{
    PublicDecryptResponseModel, PublicReqStateModelWithOldStatusAndTimestamp,
};
use crate::store::sql::models::req_status_enum_model::ReqStatus;
use crate::store::sql::{
    client::PgClient,
    error::{SqlError, SqlResult},
};
use alloy::primitives::U256;
use uuid::Uuid;

pub enum PublicDecryptInsertResult {
    /// New request inserted into DB
    Inserted { ext_job_id: Uuid },
    /// Duplicate request that already completed
    DuplicateCompleted {
        ext_job_id: Uuid,
        response: PublicDecryptResponse,
    },
    /// Duplicate request still being processed
    DuplicateProcessing { ext_job_id: Uuid },
}

/// Outcome of completing a public decrypt request with response.
#[derive(Debug)]
pub enum PublicDecryptCompletionOutcome {
    /// Request completed successfully in this operation
    Completed { int_job_id: Vec<u8> },
    /// Request was already completed (idempotent duplicate)
    AlreadyCompleted { int_job_id: Vec<u8> },
    /// Request is already in a final failure/timed_out state
    AlreadyInFinalState {
        int_job_id: Vec<u8>,
        current_status: ReqStatus,
    },
    /// Request with this gw_reference_id was not found
    NotFound,
}

pub struct PublicDecryptRepository {
    pool: PgClient,
}

impl PublicDecryptRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // NOTE: We have a query which is performed at the database level in a pg_cron job instead of being called by the internals. and is trigged on this condition:
    // If status == 'receipt_recieved' and now - `updated_at` > 30 min roughly (TBD.)
    // Update status to timed_out with configured timeout message.
    // OR IN THE TIMEOUT REPO.

    /* NOTE: max size for indexes
        B-Tree (Default)	~2,704 bytes	Used for your UNIQUE indexes (int_job_id).
        Hash (USING HASH)	Unlimited (1 GB)	Used for your non-unique lookups (ext_job_id).
    */

    // INITIAL POST REQUEST:

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
            FROM public_decrypt_req
            WHERE int_job_id = $1
              AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            LIMIT 1
            "#,
            int_job_id_bytes
        )
        .fetch_optional(&self.pool.get_app_pool())
        .await;

        match &result {
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
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
        request: PublicDecryptRequest,
    ) -> SqlResult<PublicDecryptInsertResult> {
        let req = serde_json::to_value(&request).map_err(|e| {
            SqlError::conversion_error(
                "request",
                "PublicDecryptRequest",
                format!("Failed to serialize: {}", e),
            )
        })?;

        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            INSERT INTO public_decrypt_req (
                ext_job_id,
                int_job_id,
                req,
                req_status,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, 'queued'::req_status, NOW(), NOW())
            ON CONFLICT (int_job_id)
            WHERE req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            DO UPDATE SET updated_at = public_decrypt_req.updated_at
            RETURNING ext_job_id, (xmax = 0) AS "is_inserted!", req_status AS "req_status!: ReqStatus", res
            "#,
            ext_job_id,
            int_job_id_bytes,
            req
        )
        .fetch_one(&mut *conn)
        .await;

        match &result {
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        let record = result?;

        // Match on the state and return appropriate enum variant
        let insert_result = match (record.is_inserted, record.req_status) {
            (true, _) => {
                // New request inserted
                metrics::increment_req_status_count(
                    metrics::RequestType::PublicDecrypt,
                    ReqStatus::Queued,
                );
                PublicDecryptInsertResult::Inserted {
                    ext_job_id: record.ext_job_id,
                }
            }
            (false, ReqStatus::Completed) => {
                // Duplicate, already completed - res must exist
                let response = record
                    .res
                    .ok_or_else(|| {
                        SqlError::conversion_error(
                            "res",
                            "PublicDecryptResponse",
                            "completed request missing response".to_string(),
                        )
                    })
                    .and_then(|res_value| {
                        serde_json::from_value::<PublicDecryptResponse>(res_value).map_err(|e| {
                            SqlError::conversion_error(
                                "res",
                                "PublicDecryptResponse",
                                format!("Failed to deserialize: {}", e),
                            )
                        })
                    })?;

                PublicDecryptInsertResult::DuplicateCompleted {
                    ext_job_id: record.ext_job_id,
                    response,
                }
            }
            (false, _) => {
                // Duplicate, still processing (queued, processing, etc.)
                PublicDecryptInsertResult::DuplicateProcessing {
                    ext_job_id: record.ext_job_id,
                }
            }
        };

        Ok(insert_result)
    }

    // GATEWAY READINESS CHECK.
    /// update public_decrypt_req by int_job_id for to req_status processing
    /// Update req_status to 'processing' by int_job_id.
    /// Returns the number of rows affected (1 if found, 0 if not).
    pub async fn update_status_to_processing(&self, int_job_id_bytes: &[u8]) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM public_decrypt_req
                WHERE int_job_id = $1
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE public_decrypt_req
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
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::PublicDecrypt,
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
                SELECT req_status, updated_at FROM public_decrypt_req
                WHERE int_job_id = $2
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE public_decrypt_req
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
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::PublicDecrypt,
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
                SELECT req_status, updated_at FROM public_decrypt_req
                WHERE int_job_id = $1
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE public_decrypt_req
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
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::PublicDecrypt,
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
            FROM public_decrypt_req
            WHERE req_status = 'tx_in_flight'::req_status
            "#
        )
        .fetch_all(&mut *conn)
        .await;

        match &rows {
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        let rows = rows?;
        if rows.is_empty() {
            return Ok(0);
        }

        // Perform bulk update (updated_at set by trigger)
        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            UPDATE public_decrypt_req
            SET req_status = 'processing'::req_status
            WHERE req_status = 'tx_in_flight'::req_status
            "#
        )
        .execute(&mut *conn)
        .await;

        match &result {
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        let rows_affected = result?.rows_affected();

        // Update metrics: decrement tx_in_flight, increment processing
        for _ in 0..rows_affected {
            metrics::record_status_transition(
                metrics::RequestType::PublicDecrypt,
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
                SELECT req_status, updated_at FROM public_decrypt_req
                WHERE int_job_id = $3
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE public_decrypt_req
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
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::PublicDecrypt,
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
                SELECT req_status, updated_at FROM public_decrypt_req
                WHERE int_job_id = $2
                  AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            ),
            upd AS (
                UPDATE public_decrypt_req
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
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::PublicDecrypt,
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

    // LISTENER QUERIES:

    // update by gw_reference_id, res, and status completed, where status != 'timed_out' or 'failure', returns int_job_id, status, updated_at, err_reason
    /// Update res, req_status to 'completed', and gw_response_tx_hash.
    /// Returns an outcome enum indicating success, already completed, already in final state, or not found.
    pub async fn complete_req_with_res(
        &self,
        gw_reference_id: U256,
        response: PublicDecryptResponse,
        gw_response_tx_hash: &str,
    ) -> SqlResult<PublicDecryptCompletionOutcome> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();
        let res = serde_json::to_value(&response).map_err(|e| {
            SqlError::conversion_error(
                "response",
                "PublicDecryptResponse",
                format!("Failed to serialize: {}", e),
            )
        })?;

        let mut conn = self.pool.get_app_connection().await?;

        // Step 1: Query current state
        let query_start = Instant::now();
        let current_state = sqlx::query!(
            r#"
            SELECT int_job_id, req_status as "req_status!: ReqStatus"
            FROM public_decrypt_req
            WHERE gw_reference_id = $1
            "#,
            gw_ref_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &current_state {
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        let current_state = current_state?;

        // Step 2: Check state and return appropriate outcome
        let Some(state) = current_state else {
            return Ok(PublicDecryptCompletionOutcome::NotFound);
        };

        match state.req_status {
            ReqStatus::Completed => {
                return Ok(PublicDecryptCompletionOutcome::AlreadyCompleted {
                    int_job_id: state.int_job_id,
                });
            }
            ReqStatus::Failure | ReqStatus::TimedOut => {
                return Ok(PublicDecryptCompletionOutcome::AlreadyInFinalState {
                    int_job_id: state.int_job_id,
                    current_status: state.req_status,
                });
            }
            ReqStatus::ReceiptReceived => {
                // Continue with update
            }
            _ => {
                // Unexpected state (e.g., Processing, TxInFlight) - treat as not ready
                return Ok(PublicDecryptCompletionOutcome::AlreadyInFinalState {
                    int_job_id: state.int_job_id,
                    current_status: state.req_status,
                });
            }
        }

        // Step 3: Attempt update (only for ReceiptReceived state)
        let query_start = Instant::now();
        let result = sqlx::query_as!(
            PublicReqStateModelWithOldStatusAndTimestamp,
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM public_decrypt_req
                WHERE gw_reference_id = $3
                AND req_status = 'receipt_received'::req_status
            ),
            upd AS (
                UPDATE public_decrypt_req
                SET
                    res = $1,
                    req_status = 'completed'::req_status,
                    gw_response_tx_hash = $2
                WHERE gw_reference_id = $3
                  AND req_status = 'receipt_received'::req_status
                RETURNING
                    int_job_id,
                    req_status,
                    updated_at,
                    err_reason
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.int_job_id as "int_job_id!",
                upd.req_status as "req_status!: ReqStatus",
                upd.updated_at as "updated_at!",
                upd.err_reason
            FROM old, upd
            "#,
            res,
            gw_response_tx_hash,
            gw_ref_id,
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        let result = result?;

        match result {
            Some(r) => {
                metrics::record_status_transition(
                    metrics::RequestType::PublicDecrypt,
                    r.old_status,
                    ReqStatus::Completed,
                    r.old_updated_at,
                    r.updated_at,
                );
                Ok(PublicDecryptCompletionOutcome::Completed {
                    int_job_id: r.int_job_id,
                })
            }
            None => {
                // Race condition: state changed between check and update
                Ok(PublicDecryptCompletionOutcome::AlreadyCompleted {
                    int_job_id: state.int_job_id,
                })
            }
        }
    }

    // select in `public_decrypt_req` by `ext_job_id` (need status `res` and `err_reason` and `updated_at` and `ext_request_id`)
    /// Select status, res, err_reason, and updated_at by ext_job_id.
    pub async fn find_status_and_res_by_ext_id(
        &self,
        ext_job_id: Uuid,
    ) -> SqlResult<Option<PublicDecryptResponseModel>> {
        let mut conn = self.pool.get_app_connection().await?;
        let query_start = Instant::now();
        let result = sqlx::query_as!(
            PublicDecryptResponseModel,
            r#"
            SELECT
                ext_job_id,
                req_status as "req_status!: ReqStatus", -- Force Non-Null Enum
                res,
                err_reason,
                updated_at
            FROM public_decrypt_req
            WHERE ext_job_id = $1
            "#,
            ext_job_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
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
            FROM public_decrypt_req
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
}
