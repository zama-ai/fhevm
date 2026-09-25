use uuid::Uuid;

/// Maps to PostgreSQL enum type `catchup_flow`
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "catchup_flow", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatchupFlow {
    Catchup,
    FinalCatchup,
}

impl CatchupFlow {
    /// Value of the `flow` metric label (D10: `chain_id` and `flow` only —
    /// `consumer_id` would make the cardinality unbounded and belongs in logs).
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
    /// Every block this request fanned out has been published (D16). Written
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
    /// permanently `Active` (D16).
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
    FanOut,
    /// The request is active but a previous delivery already published every
    /// sub-range. Publish nothing.
    AlreadyFannedOut,
    /// The request was retired before it started — a cancel beat the
    /// orchestrator here, or the split produced nothing.
    AlreadyTerminal(CatchupStatus),
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
    /// This sub-range closed the span — tick the completion counter exactly
    /// once. The `RETURNING` is a single-fire edge, so a replay of the same
    /// sub-range afterwards reports `NotActive`, not `Completed` again.
    Completed,
    /// Progress recorded; blocks are still outstanding.
    Progressed,
    /// The request is no longer `Active`, or never existed. No-op.
    NotActive,
}
