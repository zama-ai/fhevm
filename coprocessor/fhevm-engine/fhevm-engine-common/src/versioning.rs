//! Consensus-version detection that decides whether a service runs in GCS
//! (green) mode, replacing the deprecated `--gcs-mode` CLI flag.
//!
//! Each binary is compiled with a [`crate::CONSENSUS_PROTOCOL_VERSION`]. On
//! startup a service compares it against the live `versioning.consensus_version`
//! singleton row: a binary newer than the live version is the incoming green
//! deployment and runs in GCS mode; an equal binary is the live (blue) stack and
//! runs normally; an older one belongs to a retired stack and stops working.

use std::cmp::Ordering as CmpOrdering;
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
/// Every service listens on this channel and re-resolves its role: the green
/// stack becomes live, and a stack left behind pauses into no-op mode.
pub const EVENT_STACK_VERSION_UPGRADED: &str = "event_stack_version_upgraded";

/// Parse a `vMAJOR.MINOR[.PATCH]` string into a comparable tuple, tolerating a
/// leading `v`/`V`, a missing patch component, and any pre-release/build
/// suffix (e.g. `v0.14.0-rc1`). Non-numeric components parse as 0.
pub fn parse_version(s: &str) -> (u64, u64, u64) {
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

/// True iff this binary's [`STACK_VERSION`] equals `other` (same major.minor.patch).
pub fn binary_matches(other: &str) -> bool {
    parse_version(STACK_VERSION) == parse_version(other)
}

/// Current role of a running service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RuntimeStackMode {
    Live = 0,
    Candidate = 1,
    Retired = 2,
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

/// Choose a service role from its version and the active version.
pub fn classify_consensus_version(binary: u32, live: i64) -> anyhow::Result<RuntimeStackMode> {
    let live = u32::try_from(live)
        .map_err(|_| anyhow::anyhow!("invalid active consensus version: {live}"))?;
    match binary.cmp(&live) {
        CmpOrdering::Less => Ok(RuntimeStackMode::Retired),
        CmpOrdering::Equal => Ok(RuntimeStackMode::Live),
        // Any newer consensus version runs green. One that skips ahead also runs
        // green: nothing refuses it, but operators on different versions write to
        // different schemas, never agree, and the attempt times out.
        CmpOrdering::Greater => {
            if live.checked_add(1) != Some(binary) {
                warn!(
                    binary,
                    live,
                    expected = live.saturating_add(1),
                    "consensus version skips ahead of the active one; only the release \
                     the proposal names can cut over"
                );
            }
            Ok(RuntimeStackMode::Candidate)
        }
    }
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

/// Re-read `versioning.consensus_version` and apply the cutover transition
/// rules to `mode`:
///   - binary == live → live (the green stack becomes live);
///   - binary > live → green;
///   - binary < live → pause into no-op mode (the retired stack stops).
pub async fn reconcile_stack_mode(pool: &Pool<Postgres>, mode: &StackMode) -> anyhow::Result<()> {
    let row: Option<(i64,)> =
        sqlx::query_as("SELECT consensus_version FROM versioning WHERE singleton = TRUE")
            .fetch_optional(pool)
            .await?;
    let Some((live,)) = row else {
        return Err(anyhow::anyhow!(
            "cannot update service role: versioning row is missing"
        ));
    };

    let runtime_mode = classify_consensus_version(CONSENSUS_PROTOCOL_VERSION, live)?;
    mode.gcs_mode.store(
        runtime_mode == RuntimeStackMode::Candidate,
        Ordering::SeqCst,
    );
    mode.paused
        .store(runtime_mode == RuntimeStackMode::Retired, Ordering::SeqCst);
    info!(
        binary_consensus_version = CONSENSUS_PROTOCOL_VERSION,
        live_consensus_version = live,
        ?runtime_mode,
        "updated service role"
    );
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
/// `gcs,public` — is built. Unlike the release comparison it replaces, a missing
/// version is an error rather than a default to blue: a green binary that
/// assumed blue would start writing as the live stack.
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
    // While the setup marker exists the versions are not final yet. Starting now
    // could pick the wrong role, so fail and let the next start see the result.
    let setup_in_progress: bool =
        sqlx::query_scalar("SELECT to_regclass('public._fhevm_versioning_bootstrap') IS NOT NULL")
            .fetch_one(&mut conn)
            .await?;
    if setup_in_progress {
        let _ = conn.close().await;
        return Err(anyhow::anyhow!(
            "database setup is still in progress; retry startup once it completes"
        ));
    }
    let live = live_consensus_version(&mut conn).await?;
    let _ = conn.close().await;

    let live = match live {
        Some(v) => v,
        None => {
            return Err(anyhow::anyhow!(
                "active consensus version is missing; run database migrations first"
            ));
        }
    };

    let runtime_mode = classify_consensus_version(CONSENSUS_PROTOCOL_VERSION, live)?;
    let gcs_mode = match runtime_mode {
        RuntimeStackMode::Candidate => true,
        RuntimeStackMode::Live => false,
        RuntimeStackMode::Retired => {
            return Err(anyhow::anyhow!(
                "this service uses consensus version {}, but the active version is {live}",
                CONSENSUS_PROTOCOL_VERSION
            ));
        }
    };
    info!(
        binary_consensus_version = CONSENSUS_PROTOCOL_VERSION,
        live_consensus_version = live,
        ?runtime_mode,
        gcs_mode,
        "set service role from consensus version"
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

/// True if `err` is Postgres `undefined_table` (SQLSTATE 42P01) — i.e. the
/// `versioning` table does not exist yet (migrations not applied).
fn is_undefined_table(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db) if db.code().as_deref() == Some("42P01"))
}

fn is_undefined_column(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db) if db.code().as_deref() == Some("42703"))
}

/// Fetch the live consensus version singleton, or `None` if the `versioning`
/// row is absent (fresh/unseeded DB).
///
/// A missing `versioning` *table* (42P01), or a missing `consensus_version`
/// *column* (42703) during the migration window, is treated the same as a
/// missing row — `None`, not an error.
async fn live_consensus_version(conn: &mut PgConnection) -> Result<Option<i64>, sqlx::Error> {
    let row: Option<(i64,)> =
        match sqlx::query_as("SELECT consensus_version FROM versioning WHERE singleton = TRUE")
            .fetch_optional(conn)
            .await
        {
            Ok(row) => row,
            Err(err) if is_undefined_table(&err) || is_undefined_column(&err) => {
                warn!(
                    binary_consensus_version = CONSENSUS_PROTOCOL_VERSION,
                    "active consensus version is not available yet"
                );
                None
            }
            Err(err) => return Err(err),
        };
    Ok(row.map(|(v,)| v))
}

/// Re-read the live consensus version on `conn` and report whether this binary
/// belongs to a retired stack (its [`crate::CONSENSUS_PROTOCOL_VERSION`] is below
/// the live `versioning.consensus_version`). A missing version is an error, not a
/// permissive default: a write guard must not fail open.
///
/// This is the single source of truth for "should this stack stop touching the
/// DB" — the same fence used by [`resolve_gcs_mode`] and [`reconcile_stack_mode`].
/// Read it *after* taking the shared cutover lock (see [`cutover_gate`]) to close
/// the begin-time TOCTOU window.
pub async fn is_retired(conn: &mut PgConnection) -> Result<bool, sqlx::Error> {
    let Some(live) = live_consensus_version(conn).await? else {
        return Err(sqlx::Error::Configuration(
            "active consensus version is missing; refusing writes".into(),
        ));
    };
    Ok(i64::from(CONSENSUS_PROTOCOL_VERSION) < live)
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
/// Returns `Stop` when cutover has retired this writer and `Skip` when rollback
/// has paused derived GCS writes. The caller rolls back either blocked
/// transaction before returning it.
///
/// Why the lock and not a check at BEGIN: a transaction opened before cutover
/// could otherwise commit *after* it, injecting stale-format rows into the live
/// tables. The shared lock closes that window: either this tx holds it and
/// cutover's exclusive request waits for the commit, or cutover already
/// committed and the check below sees the new consensus version and aborts.
/// Shared locks are mutually compatible, so replicas are not serialized against
/// each other — only against the one-shot cutover.
///
/// GCS-mode (green) writers also take the shared lock: their writes land in the
/// `gcs` schema, which cutover merges and rollback resets. Without the lock a
/// green write could be lost during cutover or land in the recreated schema
/// after rollback. Raw ingestion continues after rollback, while derived workers
/// skip when the GCS row is `PAUSED`.
async fn cutover_gate(
    tx: &mut Transaction<'_, Postgres>,
    gcs_mode: bool,
    rollback_policy: GcsRollbackPolicy,
) -> Result<Option<GateBlock>, sqlx::Error> {
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut **tx)
        .await?;

    // Check again after taking the lock because a cutover may have just finished.
    if is_retired(tx).await? {
        return Ok(Some(GateBlock::Stop));
    }

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
    Ok(None)
}

/// Begin a **write** transaction fenced against cutover and rollback, in one call.
///
/// Returns [`WriteGuard::Stop`] for a retired stack and [`WriteGuard::Skip`] for a
/// derived GCS write after rollback.
///
/// Use this for every BCS or GCS write transaction. Read-only transactions should
/// not take the shared lock, which would delay cutover or rollback behind every
/// in-flight read.
pub async fn begin_write_guarded(
    pool: &Pool<Postgres>,
    gcs_mode: bool,
    rollback_policy: GcsRollbackPolicy,
) -> Result<WriteGuard<'static>, sqlx::Error> {
    let mut tx = pool.begin().await?;
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

/// Like [`begin_write_guarded`] but begins on an already-acquired connection.
pub async fn begin_write_guarded_conn(
    conn: &mut PgConnection,
    gcs_mode: bool,
    rollback_policy: GcsRollbackPolicy,
) -> Result<WriteGuard<'_>, sqlx::Error> {
    let mut tx = conn.begin().await?;
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
    use super::{classify_consensus_version, parse_version, RuntimeStackMode};
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
    fn classifies_consensus_relationships() {
        assert_eq!(
            classify_consensus_version(7, 7).unwrap(),
            RuntimeStackMode::Live
        );
        assert_eq!(
            classify_consensus_version(8, 7).unwrap(),
            RuntimeStackMode::Candidate
        );
        assert_eq!(
            classify_consensus_version(6, 7).unwrap(),
            RuntimeStackMode::Retired
        );
    }

    #[test]
    fn runs_green_on_any_newer_consensus_version() {
        // A version that skips ahead still runs green.
        assert_eq!(
            classify_consensus_version(9, 7).unwrap(),
            RuntimeStackMode::Candidate
        );
    }

    #[test]
    fn rejects_an_unreadable_active_consensus_version() {
        assert!(classify_consensus_version(2, -1).is_err());
    }
}
