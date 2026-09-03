//! Epoch fencing: see the module doc block in `public_decrypt_repo` for
//! the full rationale, which applies here unchanged. The listener paths deliberately left
//! unfenced in this repository are [`InputProofRepository::accept_and_complete_input_proof_req`]
//! and [`InputProofRepository::reject_and_complete_input_proof_req`] - both record on-chain
//! truth observed by a chain listener, not a send decision this pod's epoch owns.

use std::time::Instant;

use serde_json::Value;
use tracing::error;

use crate::core::event::{InputProofRequest, InputProofResponse};
use crate::core::job_id::JobId;
use crate::metrics;
use crate::orchestrator::{DispatcherLock, UNCLAIMED_EPOCH};
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
    Completed { int_job_id: JobId },
    /// Request was already completed (idempotent duplicate)
    AlreadyCompleted { int_job_id: JobId },
    /// Request is already in a final failure/timed_out state
    AlreadyInFinalState {
        int_job_id: JobId,
        current_status: ReqStatus,
    },
    /// Request with this gw_reference_id was not found
    NotFound,
}

/// Result of attempting to insert a new input proof request
pub enum InputProofInsertResult {
    /// New request inserted into DB
    Inserted { ext_job_id: Uuid },
    /// Duplicate request that already completed
    DuplicateCompleted {
        ext_job_id: Uuid,
        accepted: bool,
        response: Option<InputProofResponse>,
    },
    /// Duplicate request still being processed
    DuplicateProcessing { ext_job_id: Uuid },
}

pub struct InputProofRepository {
    pool: PgClient,
    dispatcher_lock: DispatcherLock,
}

impl InputProofRepository {
    pub fn new(pool: PgClient, dispatcher_lock: DispatcherLock) -> Self {
        Self {
            pool,
            dispatcher_lock,
        }
    }

    /// This pod's dispatch-gate reading: `Some(epoch)` while it is the confirmed dispatcher,
    /// `None` otherwise. HTTP intake reads this **once** and uses the one value for both
    /// decisions it makes about a new request - what to stamp on the row, and whether to
    /// dispatch it in process. See
    /// [`Self::insert_data_on_conflict_and_get_ext_job_id`] for why one read rather than two.
    pub fn dispatch_epoch(&self) -> Option<i64> {
        self.dispatcher_lock.dispatching_epoch()
    }

    // NOTE: We have a query which is performed at the database level in a pg_cron job instead of being called by the internals. and is triggered on this condition:
    // If status == 'receipt_received' and now - `updated_at` > 30 min roughly (TBD.)
    // Update status to timed_out with configured timeout message.
    // OR IN THE TIMEOUT REPO.

    /// Check for an existing request (not failed, not timed_out).
    /// Returns the ext_job_id (Uuid) if found.
    /// Returns None if the request doesn't exist OR is in a terminal failure state.
    /// Used for bounce checking: if any request exists (including completed), skip bounce.
    pub async fn find_active_ext_ref_by_int_job_id(
        &self,
        int_job_id_bytes: &[u8],
    ) -> SqlResult<Option<Uuid>> {
        let query_start = Instant::now();

        let result = sqlx::query_scalar!(
            r#"
            SELECT ext_job_id
            FROM input_proof_req
            WHERE int_job_id = $1
              AND req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            LIMIT 1
            "#,
            int_job_id_bytes
        )
        .fetch_optional(&self.pool.get_app_pool())
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        Ok(result?)
    }

    /// Insert req, ext_job_id, int_job_id.
    /// Returns Inserted for new requests, DuplicateCompleted for completed duplicates,
    /// or DuplicateProcessing for in-progress duplicates.
    ///
    /// Uses ON CONFLICT with partial unique index for atomic deduplication (no race conditions).
    /// The xmax = 0 trick detects if the row was truly inserted or if it hit the ON CONFLICT path.
    /// `dispatch_epoch` must be the same [`crate::orchestrator::DispatchGate::epoch`] reading
    /// the caller decided on - see
    /// [`crate::store::sql::repositories::public_decrypt_repo::PublicDecryptRepository::insert_data_on_conflict_and_get_ext_job_id`].
    pub async fn insert_data_on_conflict_and_get_ext_job_id(
        &self,
        ext_job_id: Uuid,
        int_job_id_bytes: &[u8],
        request: InputProofRequest,
        dispatch_epoch: Option<i64>,
    ) -> SqlResult<InputProofInsertResult> {
        let req = serde_json::to_value(&request).map_err(|e| {
            SqlError::conversion_error(
                "request",
                "InputProofRequest",
                format!("Failed to serialize: {}", e),
            )
        })?;

        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        // Use ON CONFLICT with partial unique index - atomic operation, no race conditions
        // xmax = 0 indicates true INSERT; xmax != 0 indicates ON CONFLICT update path
        let result = sqlx::query!(
            r#"
            INSERT INTO input_proof_req (
                ext_job_id,
                int_job_id,
                req,
                req_status,
                owner_epoch
            )
            VALUES ($1, $2, $3, 'processing'::req_status, $4)
            ON CONFLICT (int_job_id)
            WHERE req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
              AND int_job_id != '\x0000000000000000000000000000000000000000000000000000000000000000'
            DO UPDATE SET updated_at = input_proof_req.updated_at
            RETURNING ext_job_id, (xmax = 0) AS "is_inserted!", req_status AS "req_status!: ReqStatus", accepted, res
            "#,
            ext_job_id,
            int_job_id_bytes,
            req,
            dispatch_epoch.unwrap_or(UNCLAIMED_EPOCH),
        )
        .fetch_one(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        let record = result?;

        // Match on the state and return appropriate enum variant
        let insert_result = match (record.is_inserted, record.req_status) {
            (true, _) => {
                // New request inserted
                metrics::increment_req_status_count(
                    metrics::RequestType::InputProof,
                    ReqStatus::Processing,
                );
                InputProofInsertResult::Inserted {
                    ext_job_id: record.ext_job_id,
                }
            }
            (false, ReqStatus::Completed) => {
                // Duplicate, already completed - return cached response
                let response = if let Some(res_value) = record.res {
                    match serde_json::from_value::<InputProofResponse>(res_value) {
                        Ok(r) => Some(r),
                        Err(e) => {
                            error!(
                                alert = true,
                                ext_job_id = %record.ext_job_id,
                                error = %e,
                                "Failed to deserialize InputProofResponse from database"
                            );
                            None
                        }
                    }
                } else {
                    None
                };
                let accepted = match record.accepted {
                    Some(v) => v,
                    None => {
                        error!(
                            alert = true,
                            ext_job_id = %record.ext_job_id,
                            "Completed request missing accepted field in database"
                        );
                        false
                    }
                };
                InputProofInsertResult::DuplicateCompleted {
                    ext_job_id: record.ext_job_id,
                    accepted,
                    response,
                }
            }
            (false, _) => {
                // Duplicate, still processing (queued, processing, tx_in_flight, etc.)
                InputProofInsertResult::DuplicateProcessing {
                    ext_job_id: record.ext_job_id,
                }
            }
        };

        Ok(insert_result)
    }

    /// Update req_status to 'tx_in_flight' by int_job_id.
    /// Returns number of rows affected.
    pub async fn update_status_to_tx_in_flight(&self, int_job_id_bytes: &[u8]) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;
        let epoch = self.dispatcher_lock.fencing_epoch();

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM input_proof_req WHERE int_job_id = $1
            ),
            upd AS (
                UPDATE input_proof_req
                SET req_status = 'tx_in_flight'::req_status,
                    owner_epoch = $2
                WHERE int_job_id = $1
                  AND req_status = 'processing'::req_status
                  AND owner_epoch <= $2
                RETURNING req_status, updated_at
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            int_job_id_bytes,
            epoch,
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

    /// Update req_status to 'receipt_received', set tx hash and gw_ref_id by int_job_id.
    /// Returns number of rows affected.
    pub async fn update_input_proof_status_to_receipt_received(
        &self,
        int_job_id_bytes: &[u8],
        gw_req_tx_hash: &str,
        gw_reference_id: U256,
    ) -> SqlResult<u64> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();
        let mut conn = self.pool.get_app_connection().await?;
        let epoch = self.dispatcher_lock.fencing_epoch();

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM input_proof_req WHERE int_job_id = $3
            ),
            upd AS (
                UPDATE input_proof_req
                SET
                    req_status = 'receipt_received'::req_status,
                    gw_req_tx_hash = $1,
                    gw_reference_id = $2,
                    owner_epoch = $4
                WHERE int_job_id = $3
                  AND req_status = 'tx_in_flight'::req_status
                  AND owner_epoch <= $4
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
            int_job_id_bytes,
            epoch,
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

    // update status to failure and err_reason, by 'int_job_id'
    /// Update req_status to 'failure' and set err_reason by int_job_id.
    /// Returns number of rows affected.
    pub async fn update_status_to_failure(
        &self,
        int_job_id_bytes: &[u8],
        err_reason: &str,
    ) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;
        let epoch = self.dispatcher_lock.fencing_epoch();

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH old AS (
                SELECT req_status, updated_at FROM input_proof_req WHERE int_job_id = $2
            ),
            upd AS (
                UPDATE input_proof_req
                SET
                    req_status = 'failure'::req_status,
                    err_reason = $1,
                    owner_epoch = $3
                WHERE int_job_id = $2
                  AND req_status IN ('processing'::req_status,
                                     'tx_in_flight'::req_status,
                                     'receipt_received'::req_status)
                  AND owner_epoch <= $3
                RETURNING req_status, updated_at
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.updated_at as "new_updated_at!"
            FROM old, upd
            "#,
            err_reason,
            int_job_id_bytes,
            epoch,
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

    // update by gateway_reference_id ->accepted = true res, req_status to 'completed' and gw_response_tx_hash, returns int_job_id
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
            SELECT int_job_id, req_status as "req_status!: ReqStatus"
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

        // Convert Vec<u8> to JobId for use in outcomes
        let state_int_job_id: JobId = state.int_job_id.clone().try_into().map_err(|_| {
            SqlError::conversion_error(
                "int_job_id",
                format!("Vec<u8> of length {}", state.int_job_id.len()),
                "Expected exactly 32 bytes for int_job_id".to_string(),
            )
        })?;

        match state.req_status {
            ReqStatus::Completed => {
                return Ok(InputProofCompletionOutcome::AlreadyCompleted {
                    int_job_id: state_int_job_id,
                });
            }
            ReqStatus::Failure | ReqStatus::TimedOut => {
                return Ok(InputProofCompletionOutcome::AlreadyInFinalState {
                    int_job_id: state_int_job_id,
                    current_status: state.req_status,
                });
            }
            ReqStatus::ReceiptReceived => {
                // Continue with update
            }
            _ => {
                // Unexpected state (e.g., Processing, TxInFlight) - treat as not ready
                return Ok(InputProofCompletionOutcome::AlreadyInFinalState {
                    int_job_id: state_int_job_id,
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
                RETURNING int_job_id, updated_at
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.int_job_id as "int_job_id!",
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

                // Convert Vec<u8> to JobId
                let int_job_id: JobId = record.int_job_id.try_into().map_err(|_| {
                    SqlError::conversion_error(
                        "int_job_id",
                        "Vec<u8>".to_string(),
                        "Expected exactly 32 bytes for int_job_id".to_string(),
                    )
                })?;

                Ok(InputProofCompletionOutcome::Completed { int_job_id })
            }
            None => {
                // Race condition: state changed between check and update
                Ok(InputProofCompletionOutcome::AlreadyCompleted {
                    int_job_id: state_int_job_id,
                })
            }
        }
    }

    // update accepted to false , req_status=completed, gw_response_tx_hash, and res, return int_job_id
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
            SELECT int_job_id, req_status as "req_status!: ReqStatus"
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

        // Convert Vec<u8> to JobId for use in outcomes
        let state_int_job_id: JobId = state.int_job_id.clone().try_into().map_err(|_| {
            SqlError::conversion_error(
                "int_job_id",
                format!("Vec<u8> of length {}", state.int_job_id.len()),
                "Expected exactly 32 bytes for int_job_id".to_string(),
            )
        })?;

        match state.req_status {
            ReqStatus::Completed => {
                return Ok(InputProofCompletionOutcome::AlreadyCompleted {
                    int_job_id: state_int_job_id,
                });
            }
            ReqStatus::Failure | ReqStatus::TimedOut => {
                return Ok(InputProofCompletionOutcome::AlreadyInFinalState {
                    int_job_id: state_int_job_id,
                    current_status: state.req_status,
                });
            }
            ReqStatus::ReceiptReceived => {
                // Continue with update
            }
            _ => {
                // Unexpected state (e.g., Processing, TxInFlight) - treat as not ready
                return Ok(InputProofCompletionOutcome::AlreadyInFinalState {
                    int_job_id: state_int_job_id,
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
                RETURNING int_job_id, updated_at
            )
            SELECT
                old.req_status as "old_status!: ReqStatus",
                old.updated_at as "old_updated_at!",
                upd.int_job_id as "int_job_id!",
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

                // Convert Vec<u8> to JobId
                let int_job_id: JobId = record.int_job_id.try_into().map_err(|_| {
                    SqlError::conversion_error(
                        "int_job_id",
                        "Vec<u8>".to_string(),
                        "Expected exactly 32 bytes for int_job_id".to_string(),
                    )
                })?;

                Ok(InputProofCompletionOutcome::Completed { int_job_id })
            }
            None => {
                // Race condition: state changed between check and update
                Ok(InputProofCompletionOutcome::AlreadyCompleted {
                    int_job_id: state_int_job_id,
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

    /// Sweep: atomically claim incomplete requests nobody current is driving, stamping
    /// `owner_epoch` and incrementing `attempts` in the same statement that selects them. Only
    /// rows this UPDATE actually touched come back - a row a concurrent claimer already took is
    /// not returned here, so the caller never dispatches it (see
    /// `update_status_to_tx_in_flight`'s doc comment for the CAS-with-discarded-count bug
    /// this is built not to repeat).
    ///
    /// Eligibility, the `tx_in_flight` reset and the lifetime `attempts` budget all work
    /// exactly as they do for public decrypt requests. See
    /// [`crate::store::sql::repositories::public_decrypt_repo::PublicDecryptRepository::claim_incomplete_requests`]
    /// for the reasoning; it is the canonical copy for all three request tables, kept in one
    /// place because three copies of it had already drifted apart once.
    pub async fn claim_incomplete_requests(
        &self,
        epoch: i64,
        max_attempts: i32,
    ) -> SqlResult<Vec<(Vec<u8>, Value, ReqStatus, i32)>> {
        let mut conn = self.pool.get_cron_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            UPDATE input_proof_req
            SET owner_epoch = $1,
                attempts = attempts + 1,
                req_status = CASE
                    WHEN req_status = 'tx_in_flight'::req_status THEN 'processing'::req_status
                    ELSE req_status
                END
            WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status)
              AND attempts < $2
              AND owner_epoch < $1
            RETURNING int_job_id, req, req_status as "req_status!: ReqStatus", attempts
            "#,
            epoch,
            max_attempts,
        )
        .fetch_all(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        Ok(result?
            .into_iter()
            .map(|row| (row.int_job_id, row.req, row.req_status, row.attempts))
            .collect())
    }

    /// Sweep: move requests that have exhausted `max_attempts` to `failure`, so a row that
    /// never completes is not claimed forever. Shares the claim's ownership predicate and
    /// stamps `owner_epoch`; see
    /// [`crate::store::sql::repositories::public_decrypt_repo::PublicDecryptRepository::fail_exhausted_attempts`]
    /// for why the comparison is `<` here and `<=` in every other fenced write, and for why an
    /// epoch can fail a row it never attempted itself.
    pub async fn fail_exhausted_attempts(
        &self,
        epoch: i64,
        max_attempts: i32,
        err_reason: &str,
    ) -> SqlResult<u64> {
        let mut conn = self.pool.get_cron_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH stale AS (
                SELECT id, req_status, updated_at
                FROM input_proof_req
                WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status)
                  AND attempts >= $1
                  AND owner_epoch < $3
                FOR UPDATE SKIP LOCKED
            ),
            updated AS (
                UPDATE input_proof_req
                SET req_status = 'failure'::req_status,
                    err_reason = $2,
                    owner_epoch = $3
                FROM stale
                WHERE input_proof_req.id = stale.id
                RETURNING input_proof_req.updated_at as new_updated_at,
                          stale.req_status as old_status,
                          stale.updated_at as old_updated_at
            )
            SELECT
                old_status as "old_status!: ReqStatus",
                old_updated_at as "old_updated_at!",
                new_updated_at as "new_updated_at!"
            FROM updated
            "#,
            max_attempts,
            err_reason,
            epoch,
        )
        .fetch_all(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }

        let rows = result?;
        let count = rows.len() as u64;

        for row in rows {
            metrics::record_status_transition(
                metrics::RequestType::InputProof,
                row.old_status,
                ReqStatus::Failure,
                row.old_updated_at,
                row.new_updated_at,
            );
        }

        Ok(count)
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
