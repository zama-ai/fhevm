use std::sync::Arc;
use uuid::Uuid;

use crate::store::client::PgClient;
use crate::store::error::SqlResult;
use crate::store::models::{
    CancelOutcome, CatchupFlow, CatchupRequestState, CatchupStatus, CoverageMerge,
    NewCatchupRequest, RequestAdmission,
};
use primitives::utils::saturating_u64_to_i64;

/// Repository for the `catchup_requests` table.
///
/// One row per catchup request, keyed by this repository's `chain_id` and the
/// `catchup_id` the consumer minted. Every query is scoped to `chain_id`, so
/// the same id sent to two chains names two independent requests.
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
    /// would be fanned out a second time. It also returns the stored
    /// `fanned_end`, which a redelivery must fan out to rather than to its own
    /// head read.
    pub async fn admit_request(&self, request: &NewCatchupRequest) -> SqlResult<RequestAdmission> {
        let mut conn = self.client.get_app_connection().await?;

        sqlx::query!(
            r#"
            INSERT INTO catchup_requests (catchup_id, flow, chain_id, consumer_id,
                                          block_start, block_end, status, sub_ranges,
                                          fanned_end, terminal_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9,
                    CASE WHEN $7 = 'ACTIVE'::catchup_status THEN NULL ELSE NOW() END)
            ON CONFLICT (chain_id, catchup_id) DO NOTHING
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
            SELECT status as "status: CatchupStatus", fanned_out_at, fanned_end
            FROM catchup_requests
            WHERE chain_id = $1 AND catchup_id = $2
            "#,
            self.chain_id,
            request.catchup_id,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(match (row.status, row.fanned_out_at) {
            (CatchupStatus::Active, None) => RequestAdmission::FanOut {
                fanned_end: row.fanned_end.map(|end| end as u64),
            },
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
             WHERE chain_id = $1 AND catchup_id = $2 AND fanned_out_at IS NULL
            "#,
            self.chain_id,
            catchup_id,
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    /// Merge one published range into the request's coverage set, and
    /// complete the request if that closed the span.
    ///
    /// `range` is inclusive, matching the payload; the half-open conversion
    /// happens here so no call site has to remember it. `None` merges nothing
    /// and only reads the state back: a range worker whose sub-range is just
    /// `fanned_end` publishes no body, and still has to learn whether the
    /// last block is due.
    ///
    /// Three properties carry this, and none of them are incidental:
    ///
    /// - **The union is idempotent.** A redelivered sub-range merges the same
    ///   range twice and the second merge changes nothing. This is the whole
    ///   reason coverage is a set and not a counter: a counter double-counts
    ///   the redelivery, reaches its target early, and marks the request
    ///   terminal while sub-ranges are still outstanding — which the fetcher
    ///   guard then reads as "discard", turning a retry into a silent data gap.
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
    ///
    /// Returns [`CoverageMerge::LastBlockDue`] whenever the merge leaves every
    /// block below `fanned_end` covered with the request still active. The row
    /// lock serializes concurrent merges, so the merge that closes that body
    /// sees it, and so does any retry after it until the last block is merged.
    /// `fanned_end = block_start` is spelled out rather than left to the
    /// containment of an empty range: a one-block request has an empty body.
    pub async fn record_coverage(
        &self,
        catchup_id: Uuid,
        range: Option<(u64, u64)>,
    ) -> SqlResult<CoverageMerge> {
        let mut conn = self.client.get_app_connection().await?;

        // An empty `int8range(0, 0)` unions as nothing.
        let (lower, upper) = match range {
            Some((start, end)) => (
                saturating_u64_to_i64(start),
                saturating_u64_to_i64(end.saturating_add(1)),
            ),
            None => (0, 0),
        };

        let row = sqlx::query!(
            r#"
            UPDATE catchup_requests
               SET covered = covered + int8multirange(int8range($3, $4)),
                   status = CASE
                              WHEN covered + int8multirange(int8range($3, $4))
                                   @> int8range(block_start, fanned_end + 1)
                              THEN 'COMPLETED'::catchup_status
                              ELSE status
                            END,
                   terminal_at = CASE
                                   WHEN covered + int8multirange(int8range($3, $4))
                                        @> int8range(block_start, fanned_end + 1)
                                   THEN NOW()
                                   ELSE terminal_at
                                 END
             WHERE chain_id = $1 AND catchup_id = $2 AND status = 'ACTIVE'
            RETURNING (status = 'COMPLETED') AS "just_completed!",
                      COALESCE(
                          fanned_end = block_start
                          OR covered @> int8range(block_start, fanned_end),
                          FALSE
                      ) AS "body_done!"
            "#,
            self.chain_id,
            catchup_id,
            lower,
            upper,
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(match row {
            Some(r) if r.just_completed => CoverageMerge::Completed,
            Some(r) if r.body_done => CoverageMerge::LastBlockDue,
            Some(_) => CoverageMerge::Progressed,
            None => CoverageMerge::NotActive,
        })
    }

    /// The fetcher's guard, run once per sub-range before any RPC. Returns the
    /// status and the completion target, which the range worker holds back.
    ///
    /// Primary key only: no `consumer_id` predicate. The `consumer_id` in a
    /// sub-range payload was stamped by the orchestrator off this very row, and
    /// a mismatch would return `None`, which the caller maps to *run the
    /// sub-range*. Ownership is checked on the cancel path, where the id
    /// genuinely arrives from outside.
    pub async fn request_state(&self, catchup_id: Uuid) -> SqlResult<Option<CatchupRequestState>> {
        let mut conn = self.client.get_app_connection().await?;

        let row = sqlx::query!(
            r#"
            SELECT status as "status: CatchupStatus", fanned_end
            FROM catchup_requests
            WHERE chain_id = $1 AND catchup_id = $2
            "#,
            self.chain_id,
            catchup_id,
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(row.map(|r| CatchupRequestState {
            status: r.status,
            fanned_end: r.fanned_end.map(|end| end as u64),
        }))
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
            ON CONFLICT (chain_id, catchup_id) DO UPDATE
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
            WHERE chain_id = $1 AND catchup_id = $2
            "#,
            self.chain_id,
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
