//! SQL-sourced queue accounting for the `Retry-After` ETA.
//!
//! The ETA needs a request's *position* (how many are ahead of it) and a queue's *size* (for a
//! request joining the back). Both used to come from the in-memory throttlers, which are fed and
//! drained only on the pod holding the dispatcher lock - on the passive pod they are empty, not
//! stale, so every ETA there floored to `min_seconds`.
//!
//! `req_status` already encodes the two stages the ETA models: `queued` is the readiness queue,
//! `processing` is the TX queue. Reading both numbers from it makes them a property of the row
//! set rather than of a pod, and lets them fold into the SELECT/INSERT each handler already
//! issues. Adding a round trip to the HTTP path is not an acceptable alternative.
//!
//! Input proofs use only the second stage; they are inserted straight as `processing`.
//!
//! # The status list in the position subqueries is not redundant
//!
//! A position count is correlated - it compares against the polled row's own status - and still
//! carries `IN ('queued', 'processing', 'tx_in_flight')`. Both jobs it does are load-bearing:
//!
//! - It scopes the count. Without it, a GET on a finished request counts every row sharing its
//!   terminal status. That is the steady-state common case, since clients poll until completion:
//!   on a million-row table, 998 000 rows in 397 ms against 0.098 ms.
//! - It is what makes the partial index usable. Postgres only picks a partial index when the
//!   query provably implies the index predicate, and a correlated `q.req_status = r.req_status`
//!   proves nothing at plan time. Without the literal list the planner falls back to the primary
//!   key: 378 ms against 0.28 ms.
//!
//! Keep the list identical to the predicate in the `..._add_active_index_...` migrations. Drift
//! shows up only as latency.
//!
//! Size counts need none of this - a literal status implies the predicate on its own.

/// Cap on index entries walked per queue count.
///
/// Cost is proportional to the depth being measured, and a backlog is when the ETA path is under
/// most pressure. Chosen above the point where the ETA is already clamped to `max_seconds` at the
/// shipped settings - 5 000 for the TX queue, 15 625 for readiness - so capping cannot change a
/// value a client sees. Raising `max_seconds` far past its shipped value would make it visible.
pub const QUEUE_SCAN_CAP: i64 = 20_000;
