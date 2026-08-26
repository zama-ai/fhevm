//! Epoch fencing (build-order step 8).
//!
//! Every write below that drives a request forward - the intake `INSERT`, and every
//! send-decision status write (`processing`, `tx_in_flight`, `receipt_received`, the
//! `failure`/`timed_out` transitions reachable from a live send) - stamps `owner_epoch` with
//! this pod's [`DispatcherLock::current_epoch`] in the same statement that changes the row, and
//! refuses to apply unless the row's current `owner_epoch` is either `NULL` (nobody has claimed
//! it under the fence yet - true for every pre-migration row and every row a pod not currently
//! holding the lock inserts) or **less than or equal to** that epoch.
//!
//! It is `<=`, not `=`: this is a fencing token, not an ownership check. `owner_epoch` records
//! the *newest* epoch that has ever touched a row, not "the one true current owner" - a write
//! from an epoch older than that is what gets refused (a pod that has not yet noticed it lost
//! the lock, writing against a row a successor has since claimed with a newer epoch), while a
//! write from an epoch *at or above* the row's recorded value always succeeds and advances it.
//! `=` alone would also refuse a legitimate successor claiming a row a predecessor last touched,
//! which is the normal case for startup recovery re-driving a row this same pod owned in its
//! previous incarnation (an older epoch than the fresh one just minted), and for the sweep
//! reclaiming a row from a genuinely dead peer. Two epochs can never disagree about direction,
//! because `owner_epoch` only ever increases: only the current advisory-lock holder ever mints
//! a new epoch, and a pod without one (`current_epoch() == None`) can never overwrite a row a
//! real epoch already claimed - see `mint_epoch`'s doc comment on `DispatcherLock` for why
//! acquisition is what mints, not mere polling.
//!
//! At one replica this changes nothing observable: the pod almost always holds the lock, so its
//! own epoch always matches what it just stamped, and even in the brief pre-acquisition window
//! `current_epoch()` is `None` and the row it just inserted is `NULL`, which still matches.
//!
//! Listener paths that record on-chain truth ([`PublicDecryptRepository::complete_req_with_res`]
//! here) are deliberately NOT fenced: they are not a send decision this pod's epoch owns, chain
//! listeners are not epoch-gated, and the `receipt_received -> completed` status guard already
//! makes them idempotent. Fencing them would risk rejecting a legitimate completion observed by
//! whichever pod's listener got there first.
//!
//! Every fenced method returns `rows_affected` (`u64`, `0` or `1`); callers must treat `0` as
//! "this pod no longer owns the row" and must not read it as if the write applied - see the
//! `on_tx_in_flight`/`on_receipt_received`/`handle_error` call sites in the gateway handlers.

use std::time::Instant;

use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::core::event::{PublicDecryptRequest, PublicDecryptResponse};
use crate::core::job_id::JobId;
use crate::metrics;
use crate::orchestrator::DispatcherLock;
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

pub struct PublicDecryptRepository {
    pool: PgClient,
    /// Source of the epoch stamped on writes this pod makes and checked against writes it
    /// refuses (see the epoch-fencing doc block below). `current_epoch()` is `None` until this
    /// pod holds the dispatcher lock, in which case a fenced write's predicate degrades to
    /// "row is unclaimed" - see the module-level rationale.
    dispatcher_lock: DispatcherLock,
}

impl PublicDecryptRepository {
    pub fn new(pool: PgClient, dispatcher_lock: DispatcherLock) -> Self {
        Self {
            pool,
            dispatcher_lock,
        }
    }

    // NOTE: We have a query which is performed at the database level in a pg_cron job instead of being called by the internals. and is triggered on this condition:
    // If status == 'receipt_received' and now - `updated_at` > 30 min roughly (TBD.)
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
        let epoch = self.dispatcher_lock.current_epoch();

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            INSERT INTO public_decrypt_req (
                ext_job_id,
                int_job_id,
                req,
                req_status,
                owner_epoch,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, 'queued'::req_status, $4, NOW(), NOW())
            ON CONFLICT (int_job_id)
            WHERE req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            DO UPDATE SET updated_at = public_decrypt_req.updated_at
            RETURNING ext_job_id, (xmax = 0) AS "is_inserted!", req_status AS "req_status!: ReqStatus", res
            "#,
            ext_job_id,
            int_job_id_bytes,
            req,
            epoch,
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
        let epoch = self.dispatcher_lock.current_epoch();

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
                SET req_status = 'processing'::req_status,
                    owner_epoch = $2
                WHERE int_job_id = $1
                  AND req_status = 'queued'::req_status
                  AND (owner_epoch IS NULL OR owner_epoch <= $2)
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
        let epoch = self.dispatcher_lock.current_epoch();

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
                    err_reason = $1,
                    owner_epoch = $3
                WHERE int_job_id = $2
                  AND req_status IN ('queued'::req_status, 'receipt_received'::req_status)
                  AND (owner_epoch IS NULL OR owner_epoch <= $3)
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
        let epoch = self.dispatcher_lock.current_epoch();

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
                SET req_status = 'tx_in_flight'::req_status,
                    owner_epoch = $2
                WHERE int_job_id = $1
                  AND req_status = 'processing'::req_status
                  AND (owner_epoch IS NULL OR owner_epoch <= $2)
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
        let epoch = self.dispatcher_lock.current_epoch();

        let query_start = Instant::now();

        // Fenced like every other write (`owner_epoch IS NULL OR owner_epoch <= $epoch`): a
        // fresh epoch is always >= any prior one, so this always wins against a genuinely dead
        // predecessor's rows (including this same pod's own previous incarnation) and never
        // touches a row a currently-live peer still owns - see `public_decrypt_repo`'s module
        // doc for why `<=`, not `=`. If this pod has not yet acquired the lock (`epoch` is
        // `None`), the predicate degrades to `owner_epoch IS NULL` and only unclaimed rows move.

        // Fetch rows to update for metrics
        let rows = sqlx::query!(
            r#"
            SELECT int_job_id, updated_at
            FROM public_decrypt_req
            WHERE req_status = 'tx_in_flight'::req_status
              AND (owner_epoch IS NULL OR owner_epoch <= $1)
            "#,
            epoch,
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
            SET req_status = 'processing'::req_status,
                owner_epoch = $1
            WHERE req_status = 'tx_in_flight'::req_status
              AND (owner_epoch IS NULL OR owner_epoch <= $1)
            "#,
            epoch,
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
        let epoch = self.dispatcher_lock.current_epoch();

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
                    gw_reference_id = $2,
                    owner_epoch = $4
                WHERE int_job_id = $3
                  AND req_status = 'tx_in_flight'::req_status
                  AND (owner_epoch IS NULL OR owner_epoch <= $4)
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

    /// Update req_status to 'failure' from 'queued' state.
    /// Used when failures happen before the request reaches 'processing'
    /// (e.g., readiness check contract errors, enqueue failures).
    pub async fn update_status_to_failure_from_queued(
        &self,
        int_job_id_bytes: &[u8],
        err_reason: &str,
    ) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;
        let epoch = self.dispatcher_lock.current_epoch();

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
                    err_reason = $1,
                    owner_epoch = $3
                WHERE int_job_id = $2
                  AND req_status = 'queued'::req_status
                  AND (owner_epoch IS NULL OR owner_epoch <= $3)
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

    /// update req_status to failure and apply err_reason by internal_indexer_id
    pub async fn update_status_to_failure_on_tx_failed(
        &self,
        int_job_id_bytes: &[u8],
        err_reason: &str,
    ) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;
        let epoch = self.dispatcher_lock.current_epoch();

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
                    err_reason = $1,
                    owner_epoch = $3
                WHERE int_job_id = $2
                  AND req_status IN ('processing'::req_status, 'tx_in_flight'::req_status)
                  AND (owner_epoch IS NULL OR owner_epoch <= $3)
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
                return Ok(PublicDecryptCompletionOutcome::AlreadyCompleted {
                    int_job_id: state_int_job_id,
                });
            }
            ReqStatus::Failure | ReqStatus::TimedOut => {
                return Ok(PublicDecryptCompletionOutcome::AlreadyInFinalState {
                    int_job_id: state_int_job_id,
                    current_status: state.req_status,
                });
            }
            ReqStatus::ReceiptReceived => {
                // Continue with update
            }
            _ => {
                // Unexpected state (e.g., Processing, TxInFlight) - treat as not ready
                return Ok(PublicDecryptCompletionOutcome::AlreadyInFinalState {
                    int_job_id: state_int_job_id,
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
                // Convert Vec<u8> to JobId
                let int_job_id: JobId = r.int_job_id.try_into().map_err(|_| {
                    SqlError::conversion_error(
                        "int_job_id",
                        "Vec<u8>".to_string(),
                        "Expected exactly 32 bytes for int_job_id".to_string(),
                    )
                })?;

                Ok(PublicDecryptCompletionOutcome::Completed { int_job_id })
            }
            None => {
                // Race condition: state changed between check and update
                Ok(PublicDecryptCompletionOutcome::AlreadyCompleted {
                    int_job_id: state_int_job_id,
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

    /// Sweep: atomically claim incomplete requests nobody current is driving, stamping
    /// `owner_epoch` and incrementing `attempts` in the same statement that selects them. Only
    /// rows this UPDATE actually touched come back - a row a concurrent claimer already took is
    /// not returned here, so the caller never dispatches it (see
    /// `update_status_to_tx_in_flight`'s doc comment for the CAS-with-discarded-count bug
    /// this is built not to repeat).
    ///
    /// Two-tier eligibility, both requiring `attempts < max_attempts`
    /// ([`Self::fail_exhausted_attempts`] handles rows past that bound):
    /// - `owner_epoch < $epoch` (strictly older, never `NULL` - see below) - a genuinely dead
    ///   predecessor. Epochs are minted only on actual lock acquisition and monotonic across
    ///   the whole database, so an *older* epoch can never still be the live holder while this
    ///   pod is (see `public_decrypt_repo`'s module doc for why `<`, not `=`, is what "dead"
    ///   means here). Claimed immediately, no staleness window: the epoch fence on every
    ///   subsequent write is what makes this safe even if the predecessor has not noticed it is
    ///   dead yet, not this timing.
    /// - `owner_epoch IS NULL OR owner_epoch = $epoch` - unclaimed, or this pod's own prior
    ///   claim. **`NULL` is not treated as immediately claimable**, unlike the branch above,
    ///   because until dispatch is gated on the lock (step 7) a `NULL` owner can be a *live*
    ///   non-holder pod driving its own accepted traffic in-process - claiming it out from under
    ///   that pod races a live sender, not a dead one, and causes exactly the double-send
    ///   `on_receipt_received`'s fence check exists to catch on the *losing* side, not prevent
    ///   on the winning one. Once non-holders stop dispatching, this branch can drop its
    ///   staleness requirement and merge with the one above. Until then, `claim_after_secs`
    ///   guards it the same as this pod's own prior claim: a row can legitimately sit here for a
    ///   while (the dominant case is a readiness check retrying against `gw_ciphertext_check`'s
    ///   worst case, ~225s at default settings) without `updated_at` moving, and reclaiming it
    ///   early would just re-dispatch live work on every tick.
    ///
    /// A row claimed out of `tx_in_flight` is also reset to `processing` in the same statement,
    /// unconditionally: nothing else ever moves a `tx_in_flight` row back, so without this
    /// `on_tx_in_flight`'s CAS (which requires `processing`) refuses every re-dispatch, the row
    /// is claimed to exhaustion and failed out, and a transaction that may already have
    /// succeeded on chain is orphaned. This used to apply only to the not-mine branch, on the
    /// reasoning that resetting my own active send risks a double-send; it now applies
    /// everywhere, because the *only* way to reach this `UPDATE` at all through the `owner_epoch
    /// = $epoch` branch is `claim_after_secs` of silence.
    ///
    /// **That silence bounds sleeps, not RPC latency, so it is not a hard guarantee.**
    /// `claim_after_secs`'s 300s default exceeds every *sleep*-bound retry budget between here
    /// and a receipt - the 225s readiness-check worst case, and the transaction engine's own gas
    /// estimation and send retries (`tx_engine.retry`: 100 attempts x 500ms each, ~50s per
    /// phase) - but `on_tx_in_flight` claims *before any RPC work* (see `TransactionHelper`'s
    /// doc comment on why), and neither phase's HTTP call carries a client-side timeout anywhere
    /// under `gateway::arbitrum`: a stalled `eth_sendRawTransactionSync` against a degraded
    /// gateway RPC can run past 300s with `updated_at` frozen the whole time, indistinguishable
    /// from a dead send. Resetting it then is the same tolerated failure mode as everywhere else
    /// in this pipeline: a second concurrent send, one `on_tx_in_flight` CAS losing to the
    /// other, one orphaned on-chain request - a wasted attempt and duplicate KMS work, never a
    /// wrong final state (see `startup.rs`'s shutdown-rationale comment for the same tradeoff
    /// made elsewhere).
    ///
    /// `attempts` is never reset - not on this `UPDATE`, not anywhere - including when a claim
    /// changes who owns a row. An earlier version of this method reset it to `1` on ownership
    /// change, so a successor would not inherit a dead predecessor's exhausted budget; that
    /// reset is what let a crash-looping single pod retry a row forever, since every restart
    /// mints a fresh epoch from [`crate::orchestrator::DispatcherLock`]'s sequence and a fresh
    /// epoch looks exactly like a legitimate successor taking over - `max_attempts` never binds
    /// when nothing ever fails to look brand new. `attempts` is a budget on the row's total
    /// redrive count across its whole life, not on any one owner's count: a pod can and will
    /// fail a row out via [`Self::fail_exhausted_attempts`] without ever having attempted it
    /// itself, if earlier owners already spent the budget. That is intended, not a bug -
    /// bounding the total is what makes `max_attempts` mean anything at all.
    pub async fn claim_incomplete_requests(
        &self,
        epoch: i64,
        max_attempts: i32,
        claim_after_secs: f64,
    ) -> SqlResult<Vec<(Vec<u8>, Value, ReqStatus, i32)>> {
        let mut conn = self.pool.get_cron_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            UPDATE public_decrypt_req
            SET owner_epoch = $1,
                attempts = attempts + 1,
                req_status = CASE
                    WHEN req_status = 'tx_in_flight'::req_status THEN 'processing'::req_status
                    ELSE req_status
                END
            WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status)
              AND attempts < $2
              AND (
                    owner_epoch < $1
                    OR (
                         (owner_epoch IS NULL OR owner_epoch = $1)
                         AND updated_at < NOW() - make_interval(secs => $3)
                       )
                  )
            RETURNING int_job_id, req, req_status as "req_status!: ReqStatus", attempts
            "#,
            epoch,
            max_attempts,
            claim_after_secs,
        )
        .fetch_all(&mut *conn)
        .await;

        match &result {
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }

        Ok(result?
            .into_iter()
            .map(|row| (row.int_job_id, row.req, row.req_status, row.attempts))
            .collect())
    }

    /// Sweep: move requests that have exhausted `max_attempts` to `failure`, so a row that
    /// never completes is not claimed forever. Guarded by the same staleness window as
    /// [`Self::claim_incomplete_requests`], so a row gets one full `claim_after` window after
    /// its last attempt before being given up on. Fenced like every other terminal write
    /// (`owner_epoch IS NULL OR owner_epoch <= $epoch`, deliberately `<=` and not an exact
    /// match - see below) and stamps `owner_epoch = $epoch`: this was the one sweep write
    /// outside the fence, and an ex-holder still reading `Held` for up to `heartbeat_interval +
    /// heartbeat_timeout` after a successor has already taken over could otherwise fail a row
    /// the successor is actively driving.
    ///
    /// This can fail a row under a `$epoch` that never itself claimed or drove it - `attempts`
    /// is a global budget on the row's whole life, never reset by [`Self::claim_incomplete_requests`]
    /// even across a change of owner, so whichever epoch's sweep next notices the row is stale
    /// and past `max_attempts` is the one that fails it, regardless of who spent the budget.
    /// The predicate stays `<=`, not narrowed to `owner_epoch = $epoch`: narrowing it would
    /// make an exhausted row invisible to everyone once its owner is a dead epoch (the claim
    /// also requires `attempts < max_attempts`, so it cannot rescue it either), stranding it
    /// forever instead of failing it. See `claim_incomplete_requests`'s doc comment for why
    /// `attempts` is global rather than reset-on-takeover: the reset was tried and reverted
    /// because it let a crash-looping pod retry a row forever.
    pub async fn fail_exhausted_attempts(
        &self,
        epoch: i64,
        max_attempts: i32,
        claim_after_secs: f64,
        err_reason: &str,
    ) -> SqlResult<u64> {
        let mut conn = self.pool.get_cron_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH stale AS (
                SELECT id, req_status, updated_at
                FROM public_decrypt_req
                WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status)
                  AND attempts >= $1
                  AND updated_at < NOW() - make_interval(secs => $2)
                  AND (owner_epoch IS NULL OR owner_epoch <= $4)
                FOR UPDATE SKIP LOCKED
            ),
            updated AS (
                UPDATE public_decrypt_req
                SET req_status = 'failure'::req_status,
                    err_reason = $3,
                    owner_epoch = $4
                FROM stale
                WHERE public_decrypt_req.id = stale.id
                RETURNING public_decrypt_req.updated_at as new_updated_at,
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
            claim_after_secs,
            err_reason,
            epoch,
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

        for row in rows {
            metrics::record_status_transition(
                metrics::RequestType::PublicDecrypt,
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
            FROM public_decrypt_req
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
