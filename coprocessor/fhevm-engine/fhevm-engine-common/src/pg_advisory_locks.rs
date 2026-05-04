//! Postgres advisory-lock keys for cross-process coordination.
//!
//! Each key (or key range, for per-chain locks) is reserved here so the
//! namespaces don't collide across modules.

/// Drift-revert coordination key. Services hold `pg_advisory_lock_shared`
/// on this key while running normally; the runner takes
/// `pg_advisory_lock` (exclusive) before running the revert SQL.
pub const DRIFT_REVERT_LOCK_KEY: i64 = 1_906_000_000;

/// Base for the host-listener's per-chain slow-lane reset lock.
/// The actual key is `BASE + chain_id`.
pub const SLOW_LANE_RESET_LOCK_KEY_BASE: i64 = 1_907_000_000;
