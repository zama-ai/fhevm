//! Epoch fencing.
//!
//! Every send-decision status write below stamps `owner_epoch` with this pod's
//! [`DispatcherLock::fencing_epoch`] in the same statement that changes the row, and applies
//! only when the row's current `owner_epoch` is less than or equal to that epoch.
//! [`UNCLAIMED_EPOCH`] sorts below every minted epoch, so a row never claimed under the
//! fence is open to any writer: every pre-migration row, and every row inserted by a pod
//! that was not the dispatcher.
//!
//! The intake `INSERT` instead stamps the caller's [`crate::orchestrator::DispatchGate`]
//! reading, passed as `dispatch_epoch`: intake decides *whether* this pod drives the row
//! rather than carrying on driving it - see
//! [`PublicDecryptRepository::insert_data_on_conflict_and_get_ext_job_id`].
//!
//! It is `<=`, not `=`: a fencing token, not an ownership check. `owner_epoch` records the
//! newest epoch that ever touched the row; a write from an older epoch - a pod that has not
//! yet noticed it lost the lock - is refused, while `=` alone would also refuse a legitimate
//! successor claiming a predecessor's row, the normal case for restart recovery and for
//! takeover from a dead peer. Epochs only ever increase: only the lock holder mints one.
//!
//! Listener paths that record on-chain truth ([`PublicDecryptRepository::complete_req_with_res`])
//! are deliberately NOT fenced: they are not a send decision, chain listeners are not
//! epoch-gated, and the status guard already makes them idempotent.
//!
//! Every fenced method returns `rows_affected` (`0` or `1`); callers must treat `0` as "this
//! pod no longer owns the row", never as if the write applied.

use std::time::Instant;

use serde_json::Value;

use crate::core::event::{PublicDecryptRequest, PublicDecryptResponse};
use crate::core::job_id::JobId;
use crate::metrics;
use crate::orchestrator::{DispatcherLock, UNCLAIMED_EPOCH};
use crate::store::sql::models::public_decrypt_req_model::{
    PublicDecryptResponseModel, PublicReqStateModelWithOldStatusAndTimestamp,
};
use crate::store::sql::models::req_status_enum_model::ReqStatus;
use crate::store::sql::repositories::queue_depth::QUEUE_SCAN_CAP;
use crate::store::sql::{
    client::PgClient,
    error::{SqlError, SqlResult},
};
use alloy::primitives::U256;
use uuid::Uuid;

/// One row the sweep claimed, in the order [`PublicDecryptRepository::claim_incomplete_requests`]
/// returns them.
pub struct ClaimedRequest {
    pub int_job_id: Vec<u8>,
    pub req: Value,
    /// Post-claim, so `tx_in_flight` never appears here - the claim rewrites it to
    /// `processing`. Chooses which event the row is re-dispatched as.
    pub status: ReqStatus,
}

/// Insert outcome plus the queue depths from the same statement (see
/// [`queue_depth`](crate::store::sql::repositories::queue_depth)). Decrypt needs both stages,
/// unlike input proof's single depth.
pub struct PublicDecryptInsertOutcome {
    pub result: PublicDecryptInsertResult,
    /// Requests in the readiness queue at insert time. Excludes this row: a data-modifying CTE
    /// is invisible to its own statement.
    pub readiness_queue_size: i64,
    /// Requests in the TX queue at insert time, from the same pre-insert snapshot.
    pub tx_queue_size: i64,
}

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
    /// refuses (see the epoch-fencing doc block below). `fencing_epoch()` reads
    /// `UNCLAIMED_EPOCH` until this pod holds the dispatcher lock, which narrows a fenced
    /// write to rows nothing else has claimed - see the module-level rationale.
    dispatcher_lock: DispatcherLock,
}

impl PublicDecryptRepository {
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
    ///
    /// `dispatch_epoch` must be the same [`crate::orchestrator::DispatchGate::epoch`] value
    /// the caller used to decide whether to drive this request in process - not a second
    /// read. Two reads can disagree: an epoch stamped without a dispatch leaves a row nothing
    /// claims until this pod restarts (the sweep skips its own epoch), and `None` stamped
    /// before a dispatch has the sweep re-drive a request already running.
    ///
    /// Also returns both queue depths from the same statement - see
    /// [`PublicDecryptInsertOutcome`].
    pub async fn insert_data_on_conflict_and_get_ext_job_id(
        &self,
        ext_job_id: Uuid,
        int_job_id_bytes: &[u8],
        request: PublicDecryptRequest,
        dispatch_epoch: Option<i64>,
    ) -> SqlResult<PublicDecryptInsertOutcome> {
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
            WITH ins AS (
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
                RETURNING ext_job_id, (xmax = 0) AS is_inserted, req_status, res
            )
            SELECT
                ins.ext_job_id AS "ext_job_id!",
                ins.is_inserted AS "is_inserted!",
                ins.req_status AS "req_status!: ReqStatus",
                ins.res,
                (
                    SELECT COUNT(*)
                    FROM (
                        SELECT 1
                        FROM public_decrypt_req q
                        WHERE q.req_status = 'queued'::req_status
                        LIMIT $5
                    ) depth
                ) AS "readiness_queue_size!",
                (
                    SELECT COUNT(*)
                    FROM (
                        SELECT 1
                        FROM public_decrypt_req q
                        WHERE q.req_status = 'processing'::req_status
                        LIMIT $5
                    ) depth
                ) AS "tx_queue_size!"
            FROM ins
            "#,
            ext_job_id,
            int_job_id_bytes,
            req,
            dispatch_epoch.unwrap_or(UNCLAIMED_EPOCH),
            QUEUE_SCAN_CAP,
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

        // Re-anchor both status gauges on counted values. They are otherwise seeded once at
        // startup and moved by inc/dec, which only fire on the pod that ran the transition - so a
        // pod that is not the dispatcher drifts further from the truth the longer it runs. This
        // lands before the increment below, so the new row is still counted exactly once.
        metrics::set_req_status_count(
            metrics::RequestType::PublicDecrypt,
            ReqStatus::Queued,
            record.readiness_queue_size,
        );
        metrics::set_req_status_count(
            metrics::RequestType::PublicDecrypt,
            ReqStatus::Processing,
            record.tx_queue_size,
        );

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

        Ok(PublicDecryptInsertOutcome {
            result: insert_result,
            readiness_queue_size: record.readiness_queue_size,
            tx_queue_size: record.tx_queue_size,
        })
    }

    // GATEWAY READINESS CHECK.
    /// update public_decrypt_req by int_job_id for to req_status processing
    /// Update req_status to 'processing' by int_job_id.
    /// Returns the number of rows affected (1 if found, 0 if not).
    pub async fn update_status_to_processing(&self, int_job_id_bytes: &[u8]) -> SqlResult<u64> {
        let mut conn = self.pool.get_app_connection().await?;
        let epoch = self.dispatcher_lock.fencing_epoch();

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
        let epoch = self.dispatcher_lock.fencing_epoch();

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
        let epoch = self.dispatcher_lock.fencing_epoch();

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
        let epoch = self.dispatcher_lock.fencing_epoch();

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
        let epoch = self.dispatcher_lock.fencing_epoch();

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
        let epoch = self.dispatcher_lock.fencing_epoch();

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
    /// Select status, res, err_reason, updated_at and queue position by ext_job_id.
    ///
    /// Position is a scalar subquery, not a second query. See
    /// [`queue_depth`](crate::store::sql::repositories::queue_depth) for why the status list
    /// repeats and what bounds the scan.
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
                r.ext_job_id,
                r.req_status as "req_status!: ReqStatus", -- Force Non-Null Enum
                r.res,
                r.err_reason,
                r.updated_at,
                (
                    SELECT COUNT(*)
                    FROM (
                        SELECT 1
                        FROM public_decrypt_req q
                        WHERE q.req_status = r.req_status
                          AND q.req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status)
                          AND q.id < r.id
                        LIMIT $2
                    ) ahead
                ) as "queue_position!",
                (
                    SELECT COUNT(*)
                    FROM (
                        SELECT 1
                        FROM public_decrypt_req q
                        WHERE q.req_status = 'processing'::req_status
                        LIMIT $2
                    ) depth
                ) as "tx_queue_size!"
            FROM public_decrypt_req r
            WHERE r.ext_job_id = $1
            "#,
            ext_job_id,
            QUEUE_SCAN_CAP,
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

    /// Sweep: atomically claim incomplete requests nobody is driving, stamping `owner_epoch`
    /// in the same statement that selects them. Only rows the
    /// UPDATE touched come back, so a row a concurrent claimer took is never dispatched here.
    ///
    /// `batch` bounds one statement rather than the work: what it leaves behind keeps an older
    /// epoch and so is claimable on the next tick (see `sweep::CLAIM_BATCH`).
    /// `SKIP LOCKED` passes over a row an app write holds, leaving it for the next tick rather
    /// than blocking the sweep behind it.
    ///
    /// `ORDER BY owner_epoch, id` matches `idx_*_sweep_claim` exactly, so Postgres walks the
    /// index and stops at `batch` - the ordering costs no sort and reads nothing beyond the
    /// rows it returns. `id` is the insertion sequence, so a failover's inherited backlog,
    /// sharing one epoch, comes back oldest first. Callers dispatch in the order returned;
    /// re-sorting the batch would discard this for an ordering the claim never selected on.
    ///
    /// Eligibility is ownership alone, with no time term: `owner_epoch < $epoch`. A strictly
    /// older epoch is a dead predecessor - epochs are minted only on acquisition and are monotonic
    /// database-wide. `UNCLAIMED_EPOCH` sorts below all of them and means no dispatcher ever
    /// claimed the row: intake stamps it exactly when the accepting pod will not drive what it
    /// inserted. If a predecessor has not yet noticed it is dead, the epoch fence on its later
    /// writes - not the claim's timing - is what keeps the immediate claim safe.
    ///
    /// A row under this pod's own current epoch is never claimed: no query can tell "still
    /// working" from "silently died" - `updated_at` freezes at `on_tx_in_flight`, before RPC
    /// work with no client-side timeout. A row orphaned in-process (panicked handler, send
    /// abandoned at shutdown) therefore waits for the next restart, whose higher epoch makes
    /// it claimable on the first tick.
    ///
    /// A row claimed out of `tx_in_flight` is reset to `processing` in the same statement:
    /// nothing else moves a `tx_in_flight` row back, so without the reset `on_tx_in_flight`'s
    /// CAS refuses every re-dispatch and the row is left with a possibly-successful
    /// transaction orphaned.
    pub async fn claim_incomplete_requests(
        &self,
        epoch: i64,
        batch: i64,
    ) -> SqlResult<Vec<ClaimedRequest>> {
        let mut conn = self.pool.get_cron_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            WITH claimed AS (
                SELECT id
                FROM public_decrypt_req
                WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status)
                  AND owner_epoch < $1
                ORDER BY owner_epoch, id
                LIMIT $2
                FOR UPDATE SKIP LOCKED
            )
            UPDATE public_decrypt_req
            SET owner_epoch = $1,
                req_status = CASE
                    WHEN req_status = 'tx_in_flight'::req_status THEN 'processing'::req_status
                    ELSE req_status
                END
            FROM claimed
            WHERE public_decrypt_req.id = claimed.id
            RETURNING public_decrypt_req.int_job_id, public_decrypt_req.req, public_decrypt_req.req_status as "req_status!: ReqStatus"
            "#,
            epoch,
            batch,
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
            .map(|row| ClaimedRequest {
                int_job_id: row.int_job_id,
                req: row.req,
                status: row.req_status,
            })
            .collect())
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
