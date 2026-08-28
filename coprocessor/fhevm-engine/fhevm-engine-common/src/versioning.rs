//! Consensus-version detection that decides whether a service runs in GCS (green)
//! mode, replacing the deprecated `--gcs-mode` CLI flag.
//!
//! Each binary is compiled with a [`crate::CONSENSUS_PROTOCOL_VERSION`]. On
//! startup a service compares it against the live `versioning.consensus_version`
//! singleton row: a binary newer than the live version is the incoming green
//! deployment and runs in GCS mode; an equal binary is the live (blue) stack and
//! runs normally; an older one belongs to a retired stack and stops working.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use sqlx::postgres::PgListener;
use sqlx::{Connection, PgConnection, Pool, Postgres, Transaction};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::{CONSENSUS_PROTOCOL_VERSION, STACK_VERSION};

/// pg_notify channel emitted by the upgrade-controller during `execute_cutover`,
/// inside the same transaction that bumps `versioning.consensus_version` (so the
/// notification is atomic with the version change — it is only delivered if the
/// cutover commits).
///
/// Every service listens on this channel. When an upgrade is active, a service
/// re-runs [`resolve_gcs_mode`] and transitions its runtime mode:
///   - binary version now == table version AND it was in GCS mode → leave GCS
///     mode (the green stack becomes live), or
///   - binary version != table version AND it was not in GCS mode → pause into
///     no-op mode (the retired blue stack stops processing).
pub const EVENT_STACK_VERSION_UPGRADED: &str = "event_stack_version_upgraded";

/// Parse a `vMAJOR.MINOR[.PATCH]` string into a comparable tuple, tolerating a
/// leading `v`/`V`, a missing patch component, and any pre-release/build
/// suffix (e.g. `v0.14.0-rc1`). Non-numeric components parse as 0.
fn parse_version(s: &str) -> (u64, u64, u64) {
    let s = s.trim();
    let s = s.strip_prefix(['v', 'V']).unwrap_or(s);
    let core = s.split(['-', '+']).next().unwrap_or(s);
    let mut parts = core.split('.').map(|p| p.parse::<u64>().unwrap_or(0));
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

/// True iff this binary's [`STACK_VERSION`] is strictly newer than `live`.
pub fn binary_is_newer_than(live: &str) -> bool {
    parse_version(STACK_VERSION) > parse_version(live)
}

/// True iff this binary's [`STACK_VERSION`] equals `live` (same major.minor.patch).
pub fn binary_matches(live: &str) -> bool {
    parse_version(STACK_VERSION) == parse_version(live)
}

/// True if this binary's [`CONSENSUS_PROTOCOL_VERSION`] is strictly newer than `live`.
fn consensus_is_newer_than(live: i64) -> bool {
    i64::from(CONSENSUS_PROTOCOL_VERSION) > live
}

/// True if this binary's [`CONSENSUS_PROTOCOL_VERSION`] equals `live`.
fn consensus_matches(live: i64) -> bool {
    i64::from(CONSENSUS_PROTOCOL_VERSION) == live
}

/// True if this binary's [`CONSENSUS_PROTOCOL_VERSION`] is strictly older than
/// `live` — i.e. it belongs to a retired stack that should no longer touch the
/// database.
fn consensus_is_older_than(live: i64) -> bool {
    i64::from(CONSENSUS_PROTOCOL_VERSION) < live
}

/// Set the versions for a new database.
///
/// The migration script creates a one-time marker before the first migration.
/// This function requires that marker and removes it after a successful update.
pub async fn bootstrap_versioning(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await?;
    let has_bootstrap_intent: bool =
        sqlx::query_scalar("SELECT to_regclass('public._fhevm_versioning_bootstrap') IS NOT NULL")
            .fetch_one(&mut *transaction)
            .await?;
    anyhow::ensure!(
        has_bootstrap_intent,
        "cannot set initial versions: this is not a new database"
    );

    sqlx::query("LOCK TABLE public._fhevm_versioning_bootstrap IN ACCESS EXCLUSIVE MODE")
        .execute(&mut *transaction)
        .await?;
    let bootstrap_intent_rows: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM public._fhevm_versioning_bootstrap")
            .fetch_one(&mut *transaction)
            .await?;
    anyhow::ensure!(
        bootstrap_intent_rows == 1,
        "cannot set initial versions: expected one setup marker, found {bootstrap_intent_rows}"
    );

    let (live_stack_version, live_consensus_version): (String, i64) = sqlx::query_as(
        "SELECT stack_version, consensus_version
         FROM versioning
         WHERE singleton = TRUE
         FOR UPDATE",
    )
    .fetch_one(&mut *transaction)
    .await?;

    let has_upgrade_history: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM upgrade_state)")
            .fetch_one(&mut *transaction)
            .await?;
    anyhow::ensure!(
        !has_upgrade_history,
        "cannot set initial versions: upgrade history already exists"
    );

    let compiled_consensus_version = i64::from(CONSENSUS_PROTOCOL_VERSION);
    anyhow::ensure!(
        compiled_consensus_version >= live_consensus_version,
        "cannot lower the consensus version from {live_consensus_version} to {compiled_consensus_version}"
    );

    let stored_version = STACK_VERSION.to_string();
    let result = sqlx::query(
        "UPDATE versioning
         SET stack_version = $1, consensus_version = $2, updated_at = NOW()
         WHERE singleton = TRUE",
    )
    .bind(&stored_version)
    .bind(compiled_consensus_version)
    .execute(&mut *transaction)
    .await?;

    anyhow::ensure!(result.rows_affected() == 1, "versioning row is missing");
    sqlx::query("DROP TABLE public._fhevm_versioning_bootstrap")
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;

    tracing::info!(
        previous_stack_version = live_stack_version,
        previous_consensus_version = live_consensus_version,
        stack_version = stored_version,
        consensus_version = CONSENSUS_PROTOCOL_VERSION,
        "set initial database versions"
    );
    Ok(())
}

/// Runtime stack mode, shared between a service's work loop and the
/// version-upgrade listener ([`run_stack_version_listener`]).
///
/// Initialized from the startup [`resolve_gcs_mode`] result. A service reads
/// [`StackMode::is_paused`] at the top of its work loop (skipping work when
/// paused) and [`StackMode::gcs_mode`] wherever it needs the current routing.
#[derive(Debug)]
pub struct StackMode {
    gcs_mode: AtomicBool,
    paused: AtomicBool,
}

impl StackMode {
    /// Create shared state seeded with the startup-resolved `gcs_mode`.
    pub fn new(gcs_mode: bool) -> Arc<Self> {
        Arc::new(Self {
            gcs_mode: AtomicBool::new(gcs_mode),
            paused: AtomicBool::new(false),
        })
    }

    /// Whether the service is currently the green (GCS) stack.
    pub fn gcs_mode(&self) -> bool {
        self.gcs_mode.load(Ordering::SeqCst)
    }

    /// Whether the service has been paused into no-op mode (retired blue stack).
    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::SeqCst)
    }
}

/// Re-read `versioning.consensus_version` and apply the cutover transition rules to
/// `mode`:
///   - binary == live AND currently in GCS mode → leave GCS mode (become live);
///   - binary < live AND not paused → pause into no-op mode;
///   - otherwise no change.
pub async fn reconcile_stack_mode(pool: &Pool<Postgres>, mode: &StackMode) -> anyhow::Result<()> {
    let row: Option<(i64,)> =
        sqlx::query_as("SELECT consensus_version FROM versioning WHERE singleton = TRUE")
            .fetch_optional(pool)
            .await?;
    let Some((live,)) = row else {
        warn!("versioning row missing during reconcile; leaving stack mode unchanged");
        return Ok(());
    };

    let matches = consensus_matches(live);
    let gcs_mode = mode.gcs_mode();
    if matches && gcs_mode {
        mode.gcs_mode.store(false, Ordering::SeqCst);
        info!(
            binary_consensus_version = CONSENSUS_PROTOCOL_VERSION,
            live_consensus_version = live,
            "consensus version matches live; leaving GCS mode (now live stack)"
        );
    } else if consensus_is_older_than(live) && !mode.is_paused() {
        mode.paused.store(true, Ordering::SeqCst);
        info!(
            binary_consensus_version = CONSENSUS_PROTOCOL_VERSION,
            live_consensus_version = live,
            "consensus version is behind live; pausing into no-op mode"
        );
    } else {
        info!(
            binary_consensus_version = CONSENSUS_PROTOCOL_VERSION,
            live_consensus_version = live,
            matches,
            gcs_mode,
            "stack-version-upgraded received; no mode change"
        );
    }
    Ok(())
}

/// Listen for [`EVENT_STACK_VERSION_UPGRADED`] and call [`reconcile_stack_mode`]
/// on every notification. Runs until `cancel` fires; logs and retries on
/// listener errors. Spawn this once per service after startup.
pub async fn run_stack_version_listener(
    pool: Pool<Postgres>,
    mode: Arc<StackMode>,
    cancel: CancellationToken,
) -> anyhow::Result<()> {
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(EVENT_STACK_VERSION_UPGRADED).await?;
    info!(
        channel = EVENT_STACK_VERSION_UPGRADED,
        "stack-version-upgraded listener started"
    );
    loop {
        tokio::select! {
            _ = cancel.cancelled() => return Ok(()),
            recv = listener.recv() => match recv {
                Ok(_) => {
                    if let Err(e) = reconcile_stack_mode(&pool, &mode).await {
                        warn!(error = %e, "failed to reconcile stack mode after version upgrade");
                    }
                }
                Err(e) => {
                    warn!(error = %e, "stack-version listener recv error; sleeping before retry");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}

/// Decide whether this binary should run in GCS (green) mode by comparing its
/// compiled-in [`crate::CONSENSUS_PROTOCOL_VERSION`] against the live
/// `versioning.consensus_version` row.
///
/// Opens a short-lived connection with the default `public` search_path, so it
/// works before the service's main pool — whose search_path may be pinned to
/// `gcs,public` — is built. If the `versioning` row is missing — or the table
/// itself does not exist yet (a fresh deploy where the db-migration Job has not
/// finished) — the service defaults to non-GCS (blue) mode rather than failing
/// startup, so it does not CrashLoop waiting on migration ordering.
pub async fn resolve_gcs_mode(database_url: &str) -> anyhow::Result<bool> {
    // Route through `connect_options_for_database_url` so that when AWS IAM auth is
    // enabled we connect with a freshly minted IAM token instead of the raw,
    // password-less URL (which would bypass IAM auth and fail to authenticate).
    // Never render the token into a URL (`render_database_url_with_auth_token`):
    // the round-trip percent-decodes it and breaks the SigV4 signature.
    let options = crate::database::connect_options_for_database_url(
        &crate::utils::DatabaseURL::from(database_url),
    )
    .await?;
    let mut conn = PgConnection::connect_with(&options).await?;
    let live = live_consensus_version(&mut conn).await?;
    let _ = conn.close().await;

    let live = match live {
        Some(v) => v,
        None => {
            warn!(
                binary_consensus_version = CONSENSUS_PROTOCOL_VERSION,
                "versioning table is empty or not yet created; defaulting to non-GCS (blue) mode"
            );
            return Ok(false);
        }
    };

    let gcs_mode = consensus_is_newer_than(live);
    info!(
        binary_consensus_version = CONSENSUS_PROTOCOL_VERSION,
        live_consensus_version = live,
        gcs_mode,
        "resolved gcs_mode from versioning table"
    );
    Ok(gcs_mode)
}

/// Fail unless the GCS schema exists.
///
/// Green services pin `search_path` to that schema, and Postgres ignores it when
/// missing, so writes would go to `public`. The upgrade-controller creates it at
/// startup; a green service that starts first must wait. Call this before
/// opening a pool.
pub async fn assert_gcs_schema_exists(database_url: &str) -> anyhow::Result<()> {
    let options = crate::database::connect_options_for_database_url(
        &crate::utils::DatabaseURL::from(database_url),
    )
    .await?;
    let mut conn = PgConnection::connect_with(&options).await?;
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_namespace WHERE nspname = $1)")
            .bind(crate::database::GCS_SCHEMA)
            .fetch_one(&mut conn)
            .await?;
    let _ = conn.close().await;
    anyhow::ensure!(
        exists,
        "schema {} does not exist yet; waiting for the upgrade-controller to create it",
        crate::database::GCS_SCHEMA
    );
    Ok(())
}

/// How long a green service waits for the upgrade-controller to create the schema.
///
/// Long enough that a slow or misconfigured controller can be fixed while the
/// services keep waiting, short enough that a deploy that will never work still
/// gives up instead of sitting there.
pub const GCS_SCHEMA_WAIT: Duration = Duration::from_secs(3600);

/// Wait for the GCS schema, then return.
///
/// Every green service starts alongside the upgrade-controller that creates the
/// schema, so losing that race is normal and must not end the process. Give up
/// only after `timeout`, which means the controller never got there.
pub async fn wait_for_gcs_schema(database_url: &str, timeout: Duration) -> anyhow::Result<()> {
    const POLL: Duration = Duration::from_secs(2);
    const LOG_EVERY: Duration = Duration::from_secs(30);
    let mut waited = Duration::ZERO;
    let mut next_log = Duration::ZERO;
    loop {
        match assert_gcs_schema_exists(database_url).await {
            Ok(()) => {
                if !waited.is_zero() {
                    info!(
                        schema = crate::database::GCS_SCHEMA,
                        waited_secs = waited.as_secs(),
                        "GCS schema is present; continuing startup"
                    );
                }
                return Ok(());
            }
            Err(err) if waited >= timeout => {
                return Err(err.context(format!(
                    "schema {} still missing after {}s",
                    crate::database::GCS_SCHEMA,
                    timeout.as_secs()
                )))
            }
            Err(err) => {
                if waited >= next_log {
                    warn!(
                        schema = crate::database::GCS_SCHEMA,
                        waited_secs = waited.as_secs(),
                        timeout_secs = timeout.as_secs(),
                        error = %err,
                        "waiting for the upgrade-controller to create the GCS schema"
                    );
                    next_log = waited + LOG_EVERY;
                }
                tokio::time::sleep(POLL).await;
                waited += POLL;
            }
        }
    }
}

/// Error returned by [`begin_guarded_pool`] / [`begin_guarded_conn`] when this
/// binary belongs to a retired stack — its [`CONSENSUS_PROTOCOL_VERSION`] is
/// strictly older than the live `versioning.consensus_version`.
#[derive(Debug, thiserror::Error)]
#[error(
    "consensus version {binary} is older than the active {live}; access denied (retired stack)"
)]
pub struct StaleStackError {
    pub binary: u32,
    pub live: i64,
}

/// True if `err` is Postgres `undefined_table` (SQLSTATE 42P01) — i.e. the
/// `versioning` table does not exist yet (migrations not applied).
fn is_undefined_table(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db) if db.code().as_deref() == Some("42P01"))
}

/// Fetch the live consensus version singleton, or `None` if the `versioning` row
/// is absent (fresh/unseeded DB). Shared by the retirement checks below.
///
/// A missing `versioning` *table* (SQLSTATE 42P01) is treated the same as a
/// missing row — `None`, not an error — so a service that starts before the
/// db-migration Job has created the table does not fail (see [`resolve_gcs_mode`]
/// and [`assert_not_retired`], which read this as "unseeded → blue / not-retired").
async fn live_consensus_version(conn: &mut PgConnection) -> Result<Option<i64>, sqlx::Error> {
    let row: Option<(i64,)> = match sqlx::query_as(
        "SELECT consensus_version FROM versioning WHERE singleton = TRUE",
    )
    .fetch_optional(conn)
    .await
    {
        Ok(row) => row,
        Err(err) if is_undefined_table(&err) => {
            warn!(
                    binary_consensus_version = CONSENSUS_PROTOCOL_VERSION,
                    "versioning table does not exist yet (migrations not applied?); treating as unseeded"
                );
            None
        }
        Err(err) => return Err(err),
    };
    Ok(row.map(|(v,)| v))
}

/// Re-read the live consensus version on `conn` and report whether this binary
/// belongs to a retired stack (its [`CONSENSUS_PROTOCOL_VERSION`] is strictly
/// older than the live `versioning.consensus_version`). A missing `versioning`
/// row is treated as not-retired, mirroring [`resolve_gcs_mode`]'s permissive
/// default so a fresh/unseeded DB is not locked out.
///
/// This is the single source of truth for "should this stack stop touching the
/// DB" — the same fence used by [`assert_not_retired`], [`resolve_gcs_mode`], and
/// [`reconcile_stack_mode`]. Read it *after* taking the shared cutover lock (see
/// [`cutover_gate`]) to close the begin-time TOCTOU window.
pub async fn is_retired(conn: &mut PgConnection) -> Result<bool, sqlx::Error> {
    Ok(live_consensus_version(conn)
        .await?
        .is_some_and(consensus_is_older_than))
}

/// Re-read the live consensus version on `conn` and fail if this binary is strictly
/// older (a retired stack). A missing `versioning` row is permissive, mirroring
/// [`resolve_gcs_mode`]'s default, so a fresh/unseeded DB is not locked out.
async fn assert_not_retired(conn: &mut PgConnection) -> Result<(), sqlx::Error> {
    if let Some(live) = live_consensus_version(conn).await? {
        if consensus_is_older_than(live) {
            return Err(sqlx::Error::Configuration(Box::new(StaleStackError {
                binary: CONSENSUS_PROTOCOL_VERSION,
                live,
            })));
        }
    }
    Ok(())
}

/// Begin a transaction on `pool` whose first action asserts this binary is not a
/// retired stack (see [`assert_not_retired`]). On rejection the just-opened
/// transaction is dropped (and thus rolled back) before it is returned, so a
/// stale binary can neither read nor write through it.
///
/// Cost: one extra round-trip per transaction (a single indexed singleton read).
pub(crate) async fn begin_guarded_pool(
    pool: &Pool<Postgres>,
) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    assert_not_retired(&mut tx).await?;
    Ok(tx)
}

/// Like [`begin_guarded_pool`] but begins on an already-acquired connection.
pub(crate) async fn begin_guarded_conn(
    conn: &mut PgConnection,
) -> Result<Transaction<'_, Postgres>, sqlx::Error> {
    let mut tx = conn.begin().await?;
    assert_not_retired(&mut tx).await?;
    Ok(tx)
}

/// PostgreSQL advisory-lock key serializing writes against cutover and rollback.
/// The upgrade-controller takes the **exclusive** form; every guarded write
/// transaction takes the **shared** form via [`cutover_gate`]. Chosen to be
/// recognizable in logs (`0x4648_4556_4355_5456` ~ ASCII "FHEVCUTV").
pub const CUTOVER_LOCK_ID: i64 = 0x4648_4556_4355_5456;

/// Result of opening a guarded write transaction.
pub enum WriteGuard<'a> {
    Proceed(Transaction<'a, Postgres>),
    Stop,
    Skip,
}

impl<'a> WriteGuard<'a> {
    /// Returns the transaction when the write can proceed.
    pub fn into_tx(self) -> Option<Transaction<'a, Postgres>> {
        match self {
            WriteGuard::Proceed(tx) => Some(tx),
            WriteGuard::Stop | WriteGuard::Skip => None,
        }
    }
}

/// Controls GCS writes after a rollback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GcsRollbackPolicy {
    /// Used by raw chain ingestion.
    Continue,
    /// Used by derived output workers.
    Skip,
}

#[derive(Clone, Copy)]
enum GateBlock {
    Stop,
    Skip,
}

/// Cutover and rollback safety gate for a write transaction.
///
/// Takes the **shared** advisory lock on `tx`, then checks the stack state while
/// ordered against controller changes, which take the **exclusive** form.
/// Returns `Stop` when cutover has retired a BCS writer and `Skip` when rollback
/// has paused derived GCS writes. The caller rolls back either blocked
/// transaction before returning it.
///
/// Why the lock, not just [`begin_guarded_pool`]'s BEGIN-time check:
/// `assert_not_retired` runs only at BEGIN, so a transaction opened before
/// cutover could otherwise commit *after* it (a time-of-check/time-of-use gap),
/// injecting stale-format rows into the live green tables. The shared lock closes
/// that window: either this tx holds the shared lock and cutover's exclusive
/// request blocks until it commits, or cutover already committed and this check
/// observes the bumped `versioning` and aborts. Shared locks are mutually
/// compatible, so this does **not** serialize BCS worker replicas against each
/// other — only against the one-shot cutover.
///
/// GCS-mode (green) writers also take the shared lock: their writes land in the
/// `gcs` schema, which cutover merges and rollback resets. Without the lock a
/// green write could be lost during cutover or land in the recreated schema after
/// rollback. Holding the shared lock makes either exclusive request wait until
/// the write commits. Raw ingestion continues after rollback, while derived
/// workers skip when the GCS row is `PAUSED`. GCS writers skip only the
/// [`is_retired`] re-check because a green binary is newer than the live stack
/// and cannot be retired.
async fn cutover_gate(
    tx: &mut Transaction<'_, Postgres>,
    gcs_mode: bool,
    rollback_policy: GcsRollbackPolicy,
) -> Result<Option<GateBlock>, sqlx::Error> {
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut **tx)
        .await?;
    if gcs_mode {
        if rollback_policy == GcsRollbackPolicy::Skip {
            let paused: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM upgrade_state WHERE stack_role = 'GCS' AND state = 'PAUSED')",
            )
            .fetch_one(&mut **tx)
            .await?;
            return Ok(paused.then_some(GateBlock::Skip));
        }
        return Ok(None);
    }
    Ok(is_retired(tx).await?.then_some(GateBlock::Stop))
}

/// Begin a **write** transaction fenced against cutover and rollback, in one call.
///
/// Combines [`begin_guarded_pool`] (BEGIN-time `assert_not_retired`) with
/// [`cutover_gate`] (shared advisory lock plus the relevant state check). Returns
/// [`WriteGuard::Stop`] for a retired BCS stack and [`WriteGuard::Skip`] for a
/// derived GCS write after rollback.
///
/// Use this for every BCS or GCS write transaction. Keep [`begin_guarded_pool`]
/// for **read-only** transactions: reads cannot corrupt merged or reset state,
/// so they should not take the shared lock, which would delay cutover or
/// rollback behind every in-flight read.
pub async fn begin_write_guarded(
    pool: &Pool<Postgres>,
    gcs_mode: bool,
    rollback_policy: GcsRollbackPolicy,
) -> Result<WriteGuard<'static>, sqlx::Error> {
    let mut tx = begin_guarded_pool(pool).await?;
    match cutover_gate(&mut tx, gcs_mode, rollback_policy).await? {
        None => Ok(WriteGuard::Proceed(tx)),
        Some(block) => {
            tx.rollback().await?;
            Ok(match block {
                GateBlock::Stop => WriteGuard::Stop,
                GateBlock::Skip => WriteGuard::Skip,
            })
        }
    }
}

/// Like [`begin_write_guarded`] but begins on an already-acquired connection
/// (mirrors [`begin_guarded_conn`]).
pub async fn begin_write_guarded_conn(
    conn: &mut PgConnection,
    gcs_mode: bool,
    rollback_policy: GcsRollbackPolicy,
) -> Result<WriteGuard<'_>, sqlx::Error> {
    let mut tx = begin_guarded_conn(conn).await?;
    match cutover_gate(&mut tx, gcs_mode, rollback_policy).await? {
        None => Ok(WriteGuard::Proceed(tx)),
        Some(block) => {
            tx.rollback().await?;
            Ok(match block {
                GateBlock::Stop => WriteGuard::Stop,
                GateBlock::Skip => WriteGuard::Skip,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        consensus_is_newer_than, consensus_is_older_than, consensus_matches, parse_version,
    };
    use crate::STACK_VERSION;

    #[test]
    fn parses_loose_versions() {
        assert_eq!(parse_version("v0.13"), (0, 13, 0));
        assert_eq!(parse_version("v0.14.0"), (0, 14, 0));
        assert_eq!(parse_version("0.14.2"), (0, 14, 2));
        assert_eq!(parse_version("v1.2.3-rc1"), (1, 2, 3));
    }

    #[test]
    fn orders_versions() {
        assert!(parse_version("v0.14.0") > parse_version("v0.13"));
        assert!(parse_version("v0.14.1") > parse_version("v0.14"));
        // Missing patch component pads to 0, so these compare equal.
        assert_eq!(parse_version("v0.14.0"), parse_version("v0.14"));
        assert!(parse_version("v0.13") <= parse_version("v0.14.0"));
    }

    /// A copy of the released 0.14 comparator. Those services are already deployed and
    /// decide whether they have been replaced by comparing their own release with what
    /// cutover stores, so this copy must not be changed.
    fn released_0_14_parse_version(s: &str) -> (u64, u64, u64) {
        let s = s.trim();
        let s = s.strip_prefix(['v', 'V']).unwrap_or(s);
        let core = s.split(['-', '+']).next().unwrap_or(s);
        let mut parts = core.split('.').map(|p| p.parse::<u64>().unwrap_or(0));
        (
            parts.next().unwrap_or(0),
            parts.next().unwrap_or(0),
            parts.next().unwrap_or(0),
        )
    }

    #[test]
    fn what_we_store_replaces_a_0_14_service() {
        // 0.14 stops working once the stored release is above its own.
        let deployed = released_0_14_parse_version("0.14.0");
        assert!(
            released_0_14_parse_version(STACK_VERSION) > deployed,
            "{STACK_VERSION} must replace 0.14.0"
        );
        // A version it cannot read counts as 0.0.0, which would leave it running.
        assert_eq!(released_0_14_parse_version("not-a-version"), (0, 0, 0));
        assert!(released_0_14_parse_version("not-a-version") < deployed);
    }

    #[test]
    fn consensus_relationships() {
        let live = i64::from(crate::CONSENSUS_PROTOCOL_VERSION);
        assert!(consensus_matches(live));
        assert!(!consensus_is_newer_than(live) && !consensus_is_older_than(live));
        assert!(consensus_is_newer_than(live - 1));
        assert!(consensus_is_older_than(live + 1));
    }
}
