use std::time::Instant;

use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::core::event::{InputProofRequest, InputProofResponse};
use crate::metrics;
use crate::store::sql::models::input_proof_req_model::InputProofResponseModel;
use crate::store::sql::models::req_status_enum_model::ReqStatus;
use crate::store::sql::{
    client::PgClient,
    error::{SqlError, SqlResult},
};
use alloy::primitives::U256;
use uuid::Uuid;

/// Outcome of completing an input proof request (accept or reject).
#[derive(Debug)]
pub enum InputProofCompletionOutcome {
    /// Request completed successfully in this operation
    Completed { int_request_id: Uuid },
    /// Request was already completed (idempotent duplicate)
    AlreadyCompleted { int_request_id: Uuid },
    /// Request is already in a final failure/timed_out state
    AlreadyInFinalState {
        int_request_id: Uuid,
        current_status: ReqStatus,
    },
    /// Request with this gw_reference_id was not found
    NotFound,
}

pub struct InputProofRepository {
    pool: PgClient,
}

impl InputProofRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // NOTE: We have a query which is performed at the database level in a pg_cron job instead of being called by the internals. and is trigged on this condition:
    // If status == 'receipt_recieved' and now - `updated_at` > 30 min roughly (TBD.)
    // Update status to timed_out with configured timeout message.
    // OR IN THE TIMEOUT REPO.

    // insert ext_job_id, int_request_id (uuidv7), req into input_proof_req table return ext_job_id
    // TODO: Ensure, rows affected was 1, else return an errror ? UUID is not needed in this case. But an error in case of no rows inserted.
    /// Insert ext_job_id, int_request_id, req into input_proof_req table.
    /// Returns the ext_job_id.
    pub async fn insert_new_input_proof(
        &self,
        ext_job_id: Uuid,
        int_request_id: Uuid,
        request: InputProofRequest,
    ) -> SqlResult<Uuid> {
        let req = serde_json::to_value(&request).map_err(|e| {
            SqlError::conversion_error(
                "request",
                "InputProofRequest",
                format!("Failed to serialize: {}", e),
            )
        })?;

        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query_scalar!(
            r#"
            INSERT INTO input_proof_req (
                ext_job_id,
                int_request_id,
                req,
                req_status
            )
            VALUES ($1, $2, $3, 'processing'::req_status)
            RETURNING ext_job_id
            "#,
            ext_job_id,
            int_request_id,
            req
        )
        .fetch_one(&mut *conn)
        .await;
        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }
        let result = result?;

        metrics::increment_req_status_count(
            metrics::RequestType::InputProof,
            ReqStatus::Processing,
        );

        Ok(result)
    }

    /// Update req_status to 'tx_in_flight' by int_request_id.
    /// Returns number of rows affected.
    pub async fn update_status_to_tx_in_flight(&self, int_request_id: Uuid) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM input_proof_req WHERE int_request_id = $1
            ),
            upd AS (
                UPDATE input_proof_req
                SET req_status = 'tx_in_flight'::req_status
                WHERE int_request_id = $1
                  AND req_status = 'processing'::req_status
                RETURNING req_status, updated_at
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            int_request_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::InputProof,
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
            SELECT int_request_id, updated_at
            FROM input_proof_req
            WHERE req_status = 'tx_in_flight'::req_status
            "#
        )
        .fetch_all(&mut *conn)
        .await;

        match &rows {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        let rows = rows?;
        if rows.is_empty() {
            return Ok(0);
        }

        // Perform bulk update (updated_at set by trigger)
        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            UPDATE input_proof_req
            SET req_status = 'processing'::req_status
            WHERE req_status = 'tx_in_flight'::req_status
            "#
        )
        .execute(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        let rows_affected = result?.rows_affected();

        // Update metrics: decrement tx_in_flight, increment processing
        for _ in 0..rows_affected {
            metrics::record_status_transition(
                metrics::RequestType::InputProof,
                ReqStatus::TxInFlight,
                ReqStatus::Processing,
                chrono::Utc::now(),
                chrono::Utc::now(),
            );
        }

        Ok(rows_affected)
    }

    // update the status to 'receipt_recieved' + gw_req_tx_hash + gw_reference_id by int_request_id
    /// Update req_status to 'receipt_received', set tx hash and gw_ref_id by int_request_id.
    /// Returns number of rows affected.
    pub async fn update_input_proof_status_to_receipt_received(
        &self,
        int_request_id: Uuid,
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
                SELECT req_status, updated_at FROM input_proof_req WHERE int_request_id = $3
            ),
            upd AS (
                UPDATE input_proof_req
                SET
                    req_status = 'receipt_received'::req_status,
                    gw_req_tx_hash = $1,
                    gw_reference_id = $2
                WHERE int_request_id = $3
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
            int_request_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::InputProof,
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

    // update status to failure and err_reason, by 'int_request_id'
    /// Update req_status to 'failure' and set err_reason by int_request_id.
    /// Returns number of rows affected.
    pub async fn update_status_to_failure(
        &self,
        int_request_id: Uuid,
        err_reason: &str,
    ) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM input_proof_req WHERE int_request_id = $2
            ),
            upd AS (
                UPDATE input_proof_req
                SET
                    req_status = 'failure'::req_status,
                    err_reason = $1
                WHERE int_request_id = $2
                  AND req_status IN ('processing'::req_status,
                                     'tx_in_flight'::req_status,
                                     'receipt_received'::req_status)
                RETURNING req_status, updated_at
            )
            SELECT 
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            err_reason,
            int_request_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }
        let record = result?;

        if let Some(r) = record {
            metrics::record_status_transition(
                metrics::RequestType::InputProof,
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

    // LISTENER

    // update by gateway_reference_id ->accepted = true res, req_status to 'completed' and gw_response_tx_hash, returns int_request_id
    /// Update res, req_status to 'completed', gw_response_tx_hash, and accepted status.
    /// Returns an outcome enum indicating success, already completed, already in final state, or not found.
    pub async fn accept_and_complete_input_proof_req(
        &self,
        gw_reference_id: U256,
        response: InputProofResponse,
        gw_response_tx_hash: &str,
    ) -> SqlResult<InputProofCompletionOutcome> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();
        let res = serde_json::to_value(&response).map_err(|e| {
            SqlError::conversion_error(
                "response",
                "InputProofResponse",
                format!("Failed to serialize: {}", e),
            )
        })?;

        let mut conn = self.pool.get_app_connection().await?;

        // Step 1: Query current state
        let query_start = Instant::now();
        let current_state = sqlx::query!(
            r#"
            SELECT int_request_id, req_status as "req_status!: ReqStatus"
            FROM input_proof_req
            WHERE gw_reference_id = $1
            "#,
            gw_ref_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &current_state {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        let current_state = current_state?;

        // Step 2: Check state and return appropriate outcome
        let Some(state) = current_state else {
            return Ok(InputProofCompletionOutcome::NotFound);
        };

        match state.req_status {
            ReqStatus::Completed => {
                return Ok(InputProofCompletionOutcome::AlreadyCompleted {
                    int_request_id: state.int_request_id,
                });
            }
            ReqStatus::Failure | ReqStatus::TimedOut => {
                return Ok(InputProofCompletionOutcome::AlreadyInFinalState {
                    int_request_id: state.int_request_id,
                    current_status: state.req_status,
                });
            }
            ReqStatus::ReceiptReceived => {
                // Continue with update
            }
            _ => {
                // Unexpected state (e.g., Processing, TxInFlight) - treat as not ready
                return Ok(InputProofCompletionOutcome::AlreadyInFinalState {
                    int_request_id: state.int_request_id,
                    current_status: state.req_status,
                });
            }
        }

        // Step 3: Attempt update (only for ReceiptReceived state)
        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM input_proof_req WHERE gw_reference_id = $3
            ),
            upd AS (
                UPDATE input_proof_req
                SET
                    res = $1,
                    req_status = 'completed'::req_status,
                    gw_response_tx_hash = $2,
                    accepted = true
                WHERE gw_reference_id = $3
                  AND req_status = 'receipt_received'::req_status
                RETURNING int_request_id, updated_at
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.int_request_id as "int_request_id!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            res,
            gw_response_tx_hash,
            gw_ref_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        let result = result?;

        match result {
            Some(record) => {
                metrics::record_status_transition(
                    metrics::RequestType::InputProof,
                    record.old_status,
                    ReqStatus::Completed,
                    record.old_updated_at,
                    record.new_updated_at,
                );
                Ok(InputProofCompletionOutcome::Completed {
                    int_request_id: record.int_request_id,
                })
            }
            None => {
                // Race condition: state changed between check and update
                Ok(InputProofCompletionOutcome::AlreadyCompleted {
                    int_request_id: state.int_request_id,
                })
            }
        }
    }

    // update accepted to false , req_status=completed, gw_response_tx_hash, and res, return int_request_id
    /// Update accepted to false, req_status to 'completed', set res and tx hash.
    /// Returns an outcome enum indicating success, already completed, already in final state, or not found.
    pub async fn reject_and_complete_input_proof_req(
        &self,
        gw_reference_id: U256,
        rejection_reason: String,
        gw_response_tx_hash: &str,
    ) -> SqlResult<InputProofCompletionOutcome> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();

        let mut conn = self.pool.get_app_connection().await?;

        // Step 1: Query current state
        let query_start = Instant::now();
        let current_state = sqlx::query!(
            r#"
            SELECT int_request_id, req_status as "req_status!: ReqStatus"
            FROM input_proof_req
            WHERE gw_reference_id = $1
            "#,
            gw_ref_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &current_state {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        let current_state = current_state?;

        // Step 2: Check state and return appropriate outcome
        let Some(state) = current_state else {
            return Ok(InputProofCompletionOutcome::NotFound);
        };

        match state.req_status {
            ReqStatus::Completed => {
                return Ok(InputProofCompletionOutcome::AlreadyCompleted {
                    int_request_id: state.int_request_id,
                });
            }
            ReqStatus::Failure | ReqStatus::TimedOut => {
                return Ok(InputProofCompletionOutcome::AlreadyInFinalState {
                    int_request_id: state.int_request_id,
                    current_status: state.req_status,
                });
            }
            ReqStatus::ReceiptReceived => {
                // Continue with update
            }
            _ => {
                // Unexpected state (e.g., Processing, TxInFlight) - treat as not ready
                return Ok(InputProofCompletionOutcome::AlreadyInFinalState {
                    int_request_id: state.int_request_id,
                    current_status: state.req_status,
                });
            }
        }

        // Step 3: Attempt update (only for ReceiptReceived state)
        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM input_proof_req WHERE gw_reference_id = $3
            ),
            upd AS (
                UPDATE input_proof_req
                SET
                    accepted = false,
                    req_status = 'completed'::req_status,
                    gw_response_tx_hash = $1,
                    err_reason = $2
                WHERE gw_reference_id = $3
                  AND req_status = 'receipt_received'::req_status
                RETURNING int_request_id, updated_at
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.int_request_id as "int_request_id!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            gw_response_tx_hash,
            rejection_reason,
            gw_ref_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        let result = result?;

        match result {
            Some(record) => {
                metrics::record_status_transition(
                    metrics::RequestType::InputProof,
                    record.old_status,
                    ReqStatus::Completed,
                    record.old_updated_at,
                    record.new_updated_at,
                );
                Ok(InputProofCompletionOutcome::Completed {
                    int_request_id: record.int_request_id,
                })
            }
            None => {
                // Race condition: state changed between check and update
                Ok(InputProofCompletionOutcome::AlreadyCompleted {
                    int_request_id: state.int_request_id,
                })
            }
        }
    }

    // GET REQUEST.
    // select by ext_job_id and return res, err_reason, accepted, updated_at
    /// Select status, res, err_reason, accepted, and updated_at by ext_job_id.
    pub async fn find_status_by_ext_id(
        &self,
        ext_job_id: Uuid,
    ) -> SqlResult<Option<InputProofResponseModel>> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query_as!(
            InputProofResponseModel,
            r#"
            SELECT
                req_status as "req_status!: ReqStatus",
                res,
                err_reason,
                accepted,
                updated_at
            FROM input_proof_req
            WHERE ext_job_id = $1
            "#,
            ext_job_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        Ok(result?)
    }

    /// Find incomplete requests for startup recovery (queued, processing, tx_in_flight).
    pub async fn find_incomplete_requests(
        &self,
    ) -> SqlResult<Vec<(Uuid, Value, ReqStatus, DateTime<Utc>)>> {
        let result = sqlx::query!(
            r#"
            SELECT int_request_id, req, req_status as "req_status!: ReqStatus", updated_at
            FROM input_proof_req
            WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status)
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(&self.pool.get_app_pool())
        .await?;

        Ok(result
            .into_iter()
            .map(|row| (row.int_request_id, row.req, row.req_status, row.updated_at))
            .collect())
    }

    pub async fn count_by_status(&self) -> SqlResult<Vec<(ReqStatus, i64)>> {
        let result = sqlx::query!(
            r#"
            SELECT req_status as "req_status!: ReqStatus", COUNT(*) as "count!"
            FROM input_proof_req
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
