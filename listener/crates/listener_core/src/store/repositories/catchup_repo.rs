use std::sync::Arc;
use uuid::Uuid;

use crate::store::client::PgClient;
use crate::store::error::SqlResult;
use crate::store::models::{
    CancelOutcome, CatchupFlow, CatchupStatus, CoverageMerge, NewCatchupRequest, RequestAdmission,
};
use primitives::utils::saturating_u64_to_i64;

/// Repository for the `catchup_requests` table.
///
/// One row per catchup request, keyed by the `catchup_id` the consumer minted.
/// Sub-ranges have no durable state: they carry the id in their payload and
/// read this row.
#[derive(Clone)]
pub struct CatchupRepository {
    client: Arc<PgClient>,
    chain_id: i64,
}

impl CatchupRepository {
    pub fn new(client: Arc<PgClient>, chain_id: i64) -> Self {
        Self { client, chain_id }
    }

    /// Admit a catchup request and report what the orchestrator should do with it.
    ///
    /// `ON CONFLICT DO NOTHING`, not `DO UPDATE`: if the row already exists it
    /// was written by a cancel that beat us here, or by an earlier delivery of
    /// this same request, and in both cases the stored row wins.
    ///
    /// The read-back is what turns a duplicate request into a true no-op.
    /// `DO NOTHING` prevents a duplicate row, not duplicate work — without
    /// `fanned_out_at` the read-back would return `ACTIVE` and the whole range
    /// would be fanned out a second time.
    pub async fn admit_request(&self, request: &NewCatchupRequest) -> SqlResult<RequestAdmission> {
        let mut conn = self.client.get_app_connection().await?;

        sqlx::query!(
            r#"
            INSERT INTO catchup_requests (catchup_id, flow, chain_id, consumer_id,
                                          block_start, block_end, status, sub_ranges,
                                          fanned_end, terminal_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9,
                    CASE WHEN $7 = 'ACTIVE'::catchup_status THEN NULL ELSE NOW() END)
            ON CONFLICT (catchup_id) DO NOTHING
            "#,
            request.catchup_id,
            request.flow as CatchupFlow,
            self.chain_id,
            request.consumer_id,
            saturating_u64_to_i64(request.block_start),
            saturating_u64_to_i64(request.block_end),
            request.status as CatchupStatus,
            request.sub_ranges,
            request.fanned_end.map(saturating_u64_to_i64),
        )
        .execute(&mut *conn)
        .await?;

        let row = sqlx::query!(
            r#"
            SELECT status as "status: CatchupStatus", fanned_out_at
            FROM catchup_requests
            WHERE catchup_id = $1
            "#,
            request.catchup_id,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(match (row.status, row.fanned_out_at) {
            (CatchupStatus::Active, None) => RequestAdmission::FanOut,
            (CatchupStatus::Active, Some(_)) => RequestAdmission::AlreadyFannedOut,
            (status, _) => RequestAdmission::AlreadyTerminal(status),
        })
    }

    /// Record that every sub-range of this request has been published.
    ///
    /// Deliberately separate from [`Self::admit_request`]: it must run *after*
    /// the publishes, and publishing inside a transaction would hold the row
    /// lock across the broker round trips of an entire fanout.
    pub async fn mark_fanned_out(&self, catchup_id: Uuid) -> SqlResult<()> {
        let mut conn = self.client.get_app_connection().await?;

        sqlx::query!(
            r#"
            UPDATE catchup_requests
               SET fanned_out_at = NOW()
             WHERE catchup_id = $1 AND fanned_out_at IS NULL
            "#,
            catchup_id,
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    /// Merge one published sub-range into the request's coverage set, and
    /// complete the request if that closed the span (D16).
    ///
    /// `block_start`/`block_end` are inclusive, matching the payload; the
    /// half-open conversion happens here so no call site has to remember it.
    ///
    /// Three properties carry this, and none of them are incidental:
    ///
    /// - **The union is idempotent.** A redelivered sub-range merges the same
    ///   range twice and the second merge changes nothing. This is the whole
    ///   reason coverage is a set and not a counter: a counter double-counts
    ///   the redelivery, reaches its target early, and marks the request
    ///   terminal while sub-ranges are still outstanding — which the fetcher
    ///   guard then reads as "discard", turning a retry into a silent data gap
    ///   (D11).
    /// - **The target is `fanned_end`, not `block_end`.** A request reaching
    ///   past the chain head only ever fans out as far as the head, so coverage
    ///   can never reach the requested end and the row would sit `ACTIVE`
    ///   forever.
    /// - **`WHERE status = 'ACTIVE'`.** A sub-range that lands after a cancel
    ///   must not resurrect the request as `COMPLETED`.
    ///
    /// Returns [`CoverageMerge::Completed`] exactly once per request: the
    /// `WHERE` clause stops matching the moment the status flips, so a later
    /// replay reports [`CoverageMerge::NotActive`].
    pub async fn record_coverage(
        &self,
        catchup_id: Uuid,
        block_start: u64,
        block_end: u64,
    ) -> SqlResult<CoverageMerge> {
        let mut conn = self.client.get_app_connection().await?;

        let lower = saturating_u64_to_i64(block_start);
        let upper = saturating_u64_to_i64(block_end.saturating_add(1));

        let row = sqlx::query!(
            r#"
            UPDATE catchup_requests
               SET covered = covered + int8multirange(int8range($2, $3)),
                   status = CASE
                              WHEN covered + int8multirange(int8range($2, $3))
                                   @> int8range(block_start, fanned_end + 1)
                              THEN 'COMPLETED'::catchup_status
                              ELSE status
                            END,
                   terminal_at = CASE
                                   WHEN covered + int8multirange(int8range($2, $3))
                                        @> int8range(block_start, fanned_end + 1)
                                   THEN NOW()
                                   ELSE terminal_at
                                 END
             WHERE catchup_id = $1 AND status = 'ACTIVE'
            RETURNING (status = 'COMPLETED') AS "just_completed!"
            "#,
            catchup_id,
            lower,
            upper,
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(match row {
            Some(r) if r.just_completed => CoverageMerge::Completed,
            Some(_) => CoverageMerge::Progressed,
            None => CoverageMerge::NotActive,
        })
    }

    /// The fetcher's guard, run once per sub-range before any RPC.
    ///
    /// Primary key only: no `consumer_id` predicate. The `consumer_id` in a
    /// sub-range payload was stamped by the orchestrator off this very row, and
    /// a mismatch would return `None`, which the caller maps to *run the
    /// sub-range*. Ownership is checked on the cancel path, where the id
    /// genuinely arrives from outside.
    pub async fn request_status(&self, catchup_id: Uuid) -> SqlResult<Option<CatchupStatus>> {
        let mut conn = self.client.get_app_connection().await?;

        let row = sqlx::query!(
            r#"
            SELECT status as "status: CatchupStatus"
            FROM catchup_requests
            WHERE catchup_id = $1
            "#,
            catchup_id,
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(row.map(|r| r.status))
    }

    /// Cancel a catchup request, creating the row if the cancel beat the
    /// orchestrator here.
    ///
    /// The `consumer_id` predicate on the update branch mirrors the
    /// `InconsistentConsumerId` guard the client applies to filter commands. It
    /// cannot apply to the insert branch: there is no stored row to compare
    /// against.
    pub async fn cancel_request(
        &self,
        catchup_id: Uuid,
        consumer_id: &str,
        flow: CatchupFlow,
    ) -> SqlResult<CancelOutcome> {
        let mut conn = self.client.get_app_connection().await?;

        let flipped = sqlx::query!(
            r#"
            INSERT INTO catchup_requests (catchup_id, flow, chain_id, consumer_id,
                                          block_start, block_end, status, terminal_at)
            VALUES ($1, $2, $3, $4, NULL, NULL, 'CANCELLED', NOW())
            ON CONFLICT (catchup_id) DO UPDATE
                SET status = 'CANCELLED', terminal_at = NOW()
                WHERE catchup_requests.status = 'ACTIVE'
                  AND catchup_requests.consumer_id = $4
            "#,
            catchup_id,
            flow as CatchupFlow,
            self.chain_id,
            consumer_id,
        )
        .execute(&mut *conn)
        .await?
        .rows_affected();

        if flipped > 0 {
            return Ok(CancelOutcome::Cancelled);
        }

        // Zero rows has three causes and only one of them is fine. The
        // read-back is what stops the other two from being silent.
        let row = sqlx::query!(
            r#"
            SELECT status as "status: CatchupStatus", consumer_id
            FROM catchup_requests
            WHERE catchup_id = $1
            "#,
            catchup_id,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(
            if row.status == CatchupStatus::Active && row.consumer_id != consumer_id {
                CancelOutcome::NotOwned {
                    stored: row.consumer_id,
                }
            } else {
                CancelOutcome::AlreadyTerminal
            },
        )
    }

    /// Count the `ACTIVE` requests on this chain, per flow.
    ///
    /// Nothing server-side enforces at most one active request per consumer —
    /// that is the consumer's obligation. This count is how a consumer that
    /// mints ids without cancelling the ones they replace becomes visible:
    /// the value should track the number of consumers, and a climbing line is
    /// the only symptom such a consumer produces.
    pub async fn count_active_by_flow(&self) -> SqlResult<Vec<(CatchupFlow, i64)>> {
        let mut conn = self.client.get_app_connection().await?;

        let rows = sqlx::query!(
            r#"
            SELECT flow as "flow: CatchupFlow", COUNT(*) as "count!"
            FROM catchup_requests
            WHERE chain_id = $1 AND status = 'ACTIVE'
            GROUP BY flow
            "#,
            self.chain_id,
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(rows.into_iter().map(|r| (r.flow, r.count)).collect())
    }
}
