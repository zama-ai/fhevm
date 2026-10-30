use uuid::Uuid;

/// Maps to PostgreSQL enum type `catchup_flow`
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "catchup_flow", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatchupFlow {
    Catchup,
    FinalCatchup,
}

impl CatchupFlow {
    /// Value of the `flow` metric label. Catchup metrics are labeled by
    /// `chain_id` and `flow` only — `consumer_id` would make the cardinality
    /// unbounded and belongs in logs.
    pub fn metric_label(self) -> &'static str {
        match self {
            Self::Catchup => "catchup",
            Self::FinalCatchup => "final_catchup",
        }
    }
}

/// Maps to PostgreSQL enum type `catchup_status`
///
/// `Active` is the only non-terminal value; every other variant names the
/// reason the request stopped being current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "catchup_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatchupStatus {
    Active,
    Cancelled,
    Skipped,
    /// Every block this request fanned out has been published. Written
    /// only by [`CatchupRepository::record_coverage`], and only when the
    /// coverage set closes `block_start..fanned_end` — never by a count.
    ///
    /// [`CatchupRepository::record_coverage`]: crate::store::repositories::CatchupRepository::record_coverage
    Completed,
}

/// Input for admitting a catchup request.
///
/// `chain_id` is not carried here: the repository holds it.
#[derive(Debug, Clone)]
pub struct NewCatchupRequest {
    pub catchup_id: Uuid,
    pub flow: CatchupFlow,
    pub consumer_id: String,
    pub block_start: u64,
    pub block_end: u64,
    /// `Skipped` when the split produced no sub-ranges (block_start above the
    /// chain head), `Active` otherwise.
    pub status: CatchupStatus,
    pub sub_ranges: i32,
    /// The last block the fanout actually covered — `block_end` clamped to the
    /// chain head. This is the completion target, and passing the *requested*
    /// `block_end` instead leaves any request reaching past the head
    /// permanently `Active`.
    ///
    /// `None` on the `Skipped` path, where the fanout produced nothing.
    pub fanned_end: Option<u64>,
}

/// Verdict of [`CatchupRepository::admit_request`].
///
/// Three-way rather than a bool because the orchestrator has three genuinely
/// different things to do. Collapsing `AlreadyFannedOut` into `FanOut` would
/// make a duplicate request re-fan-out the whole range.
///
/// [`CatchupRepository::admit_request`]: crate::store::repositories::CatchupRepository::admit_request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestAdmission {
    /// The request is active and has not been fanned out yet.
    ///
    /// `fanned_end` is the completion target stored by the request's first
    /// delivery, which a redelivery must fan out to instead of its own head
    /// read. `None` only for an untracked request, which has no row.
    FanOut { fanned_end: Option<u64> },
    /// The request is active but a previous delivery already published every
    /// sub-range. Publish nothing.
    AlreadyFannedOut,
    /// The request was retired before it started — a cancel beat the
    /// orchestrator here, or the split produced nothing.
    AlreadyTerminal(CatchupStatus),
}

/// What the fetcher's guard reads about a request, from
/// [`CatchupRepository::request_state`].
///
/// [`CatchupRepository::request_state`]: crate::store::repositories::CatchupRepository::request_state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatchupRequestState {
    pub status: CatchupStatus,
    /// The completion target, and the one block a range worker holds back
    /// until every other block of the request is published. `None` on a row
    /// that never fanned out: `Skipped`, or a cancel that beat the request.
    pub fanned_end: Option<u64>,
}

/// Verdict of [`CatchupRepository::cancel_request`].
///
/// `NotOwned` carries the stored `consumer_id` so the handler can log both
/// values. A bare bool would collapse it into `AlreadyTerminal` and make an
/// operator's mistyped `consumer_id` fail silently.
///
/// [`CatchupRepository::cancel_request`]: crate::store::repositories::CatchupRepository::cancel_request
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancelOutcome {
    Cancelled,
    AlreadyTerminal,
    NotOwned { stored: String },
}

/// Verdict of [`CatchupRepository::record_coverage`].
///
/// `NotActive` deliberately folds together "cancelled mid-flight", "already
/// completed" and "no such row". All three mean the merge changed nothing, and
/// the caller's response to all three is identical: the blocks were published,
/// so `Ack` and move on. Splitting them would invite a caller to treat one of
/// them as an error.
///
/// [`CatchupRepository::record_coverage`]: crate::store::repositories::CatchupRepository::record_coverage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageMerge {
    /// This merge closed the span — tick the completion counter exactly once.
    /// The `RETURNING` is a single-fire edge, so a replay of the same merge
    /// afterwards reports `NotActive`, not `Completed` again.
    Completed,
    /// Every block below `fanned_end` is published and the request is still
    /// active: publish `fanned_end`, flagged as the last block, then merge it.
    ///
    /// Read from the row's state, not fired once. Any merge that finds the
    /// body covered reports it again, so a flagged publish that failed is
    /// re-sent by the retry instead of being lost.
    LastBlockDue,
    /// Progress recorded; blocks below `fanned_end` are still outstanding.
    Progressed,
    /// The request is no longer `Active`, or never existed. No-op.
    NotActive,
}
