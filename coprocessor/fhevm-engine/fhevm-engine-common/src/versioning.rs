//! Stack-version detection that decides whether a service runs in GCS (green)
//! mode, replacing the deprecated `--gcs-mode` CLI flag.
//!
//! Each binary is compiled with a [`crate::STACK_VERSION`]. On startup a
//! service compares it against the live `versioning.stack_version` singleton
//! row: a binary strictly newer than the live stack is the incoming green
//! deployment and runs in GCS mode; an equal-or-older binary is the live
//! (blue) stack and runs normally.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use sqlx::postgres::PgListener;
use sqlx::{Connection, PgConnection, Pool, Postgres, Transaction};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::STACK_VERSION;

/// pg_notify channel emitted by the upgrade-controller during `execute_cutover`,
/// inside the same transaction that bumps `versioning.stack_version` (so the
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

/// True iff this binary's [`STACK_VERSION`] is strictly older than `live` — i.e.
/// it belongs to a retired stack that should no longer touch the database.
pub fn binary_is_older_than(live: &str) -> bool {
    parse_version(STACK_VERSION) < parse_version(live)
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

/// Re-read `versioning.stack_version` and apply the cutover transition rules to
/// `mode`:
///   - binary == live AND currently in GCS mode → leave GCS mode (become live);
///   - binary != live AND not in GCS mode → pause into no-op mode;
///   - otherwise no change.
pub async fn reconcile_stack_mode(pool: &Pool<Postgres>, mode: &StackMode) -> anyhow::Result<()> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT stack_version FROM versioning WHERE singleton = TRUE")
            .fetch_optional(pool)
            .await?;
    let Some((live,)) = row else {
        warn!("versioning row missing during reconcile; leaving stack mode unchanged");
        return Ok(());
    };

    let matches = binary_matches(&live);
    let gcs_mode = mode.gcs_mode();
    if matches && gcs_mode {
        mode.gcs_mode.store(false, Ordering::SeqCst);
        info!(
            binary_stack_version = STACK_VERSION,
            live_stack_version = %live,
            "stack version matches live; leaving GCS mode (now live stack)"
        );
    } else if !matches && !gcs_mode {
        mode.paused.store(true, Ordering::SeqCst);
        info!(
            binary_stack_version = STACK_VERSION,
            live_stack_version = %live,
            "stack version no longer matches live; pausing into no-op mode"
        );
    } else {
        info!(
            binary_stack_version = STACK_VERSION,
            live_stack_version = %live,
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
/// compiled-in [`STACK_VERSION`] against the live `versioning.stack_version`
/// row.
///
/// Opens a short-lived connection with the default `public` search_path, so it
/// works before the service's main pool — whose search_path may be pinned to
/// `gcs,public` — is built. If the `versioning` row is missing the service
/// defaults to non-GCS (blue) mode.
pub async fn resolve_gcs_mode(database_url: &str) -> anyhow::Result<bool> {
    let mut conn = PgConnection::connect(database_url).await?;
    let row: Option<(String,)> =
        sqlx::query_as("SELECT stack_version FROM versioning WHERE singleton = TRUE")
            .fetch_optional(&mut conn)
            .await?;
    let _ = conn.close().await;

    let live = match row {
        Some((v,)) => v,
        None => {
            warn!(
                binary_stack_version = STACK_VERSION,
                "versioning table has no row; defaulting to non-GCS (blue) mode"
            );
            return Ok(false);
        }
    };

    let gcs_mode = binary_is_newer_than(&live);
    info!(
        binary_stack_version = STACK_VERSION,
        live_stack_version = %live,
        gcs_mode,
        "resolved gcs_mode from versioning table"
    );
    Ok(gcs_mode)
}

/// Error returned by [`begin_guarded_pool`] / [`begin_guarded_conn`] when this
/// binary belongs to a retired stack — its [`STACK_VERSION`] is strictly older
/// than the live `versioning.stack_version`.
#[derive(Debug, thiserror::Error)]
#[error("stack version {binary} is older than live stack {live}; access denied (retired stack)")]
pub struct StaleStackError {
    pub binary: &'static str,
    pub live: String,
}

/// Re-read the live stack version on `conn` and fail if this binary is strictly
/// older (a retired stack). A missing `versioning` row is permissive, mirroring
/// [`resolve_gcs_mode`]'s default, so a fresh/unseeded DB is not locked out.
async fn assert_not_retired(conn: &mut PgConnection) -> Result<(), sqlx::Error> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT stack_version FROM versioning WHERE singleton = TRUE")
            .fetch_optional(conn)
            .await?;
    if let Some((live,)) = row {
        if binary_is_older_than(&live) {
            return Err(sqlx::Error::Configuration(Box::new(StaleStackError {
                binary: STACK_VERSION,
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
pub async fn begin_guarded_pool(
    pool: &Pool<Postgres>,
) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    assert_not_retired(&mut tx).await?;
    Ok(tx)
}

/// Like [`begin_guarded_pool`] but begins on an already-acquired connection.
pub async fn begin_guarded_conn(
    conn: &mut PgConnection,
) -> Result<Transaction<'_, Postgres>, sqlx::Error> {
    let mut tx = conn.begin().await?;
    assert_not_retired(&mut tx).await?;
    Ok(tx)
}

#[cfg(test)]
mod tests {
    use super::parse_version;

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
        assert!(parse_version("v0.14.0") <= parse_version("v0.14.0"));
        assert!(parse_version("v0.13") <= parse_version("v0.14.0"));
    }
}
