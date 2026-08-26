//! The dispatcher lock: a session-level Postgres advisory lock that decides which of N HA
//! replica pods dispatches. Every pod serves HTTP and is Ready immediately; only the lock
//! holder is meant to dispatch (the gate itself is later work - this module only tracks and
//! exposes lock state).
//!
//! # Why a dedicated connection
//!
//! Session-level advisory locks (`pg_try_advisory_lock` / `pg_advisory_unlock`, as opposed to
//! the `_xact_` transaction-scoped form) are tied to the Postgres backend session that took
//! them. A pooled connection is recycled - the app pool idles out and cycles connections in
//! production - and would silently take the lock with it the moment it is returned or closed
//! underneath us. So this owns one [`sqlx::PgConnection`], outside both pools, for the whole
//! time this module tracks or holds the lock.
//!
//! # Lock key
//!
//! The two-argument `pg_try_advisory_lock(classid, objid)` form is used so this stays in a key
//! space disjoint from the per-request `pg_advisory_xact_lock(bigint)` single-argument form
//! used by [`crate::store::sql::repositories::utils`]: `classid` is a fixed constant reserved
//! for this lock, never used by the per-request hash.
//!
//! Advisory locks are database-wide, not schema-scoped, but the integration test harness
//! isolates concurrent test runs by schema on one shared database (each test gets its own
//! `test_<uuid>` schema and a `search_path` set via the connection string's `options`
//! parameter). A fixed `objid` would make every concurrent test relayer a non-holder of
//! someone else's key. So `objid` is derived from `current_schema()`, read once on the
//! dedicated connection right after connecting: production sets no `search_path`, so every
//! pod resolves `public` and shares one key, exactly as intended.
//!
//! # Loss detection
//!
//! There is no TCP keepalive on sqlx 0.8.6, so a query on a black-holed socket does not error
//! promptly - it waits out TCP retransmits (minutes). Every query on the dedicated connection
//! is therefore wrapped in [`tokio::time::timeout`]; the heartbeat (`SELECT pg_backend_pid()`)
//! is the only thing that would ever notice a dead socket while otherwise idle.
//!
//! Two distinct failure shapes are handled differently (see [`DispatcherLock::run`]):
//! - **Lost to a peer / demonstrably gone while healthy**: exit immediately. Another pod is
//!   already dispatching, or (via the pid tripwire below) our session was swapped from under
//!   us - either way, waiting only delays a peer already doing the job.
//! - **Connection unhealthy** (heartbeat error or timeout): retry for a bounded number of
//!   consecutive failures before exiting, since this may be transient.
//!
//! Going passive (stop trying, keep serving, never dispatch again) was considered and
//! rejected: exit is a hard [`std::process::exit`], not the graceful shutdown path - shutdown
//! would keep application-pool writers alive for seconds while a successor may already be
//! acting.
//!
//! # States
//!
//! [`LockState`] has three values. `NotHeld`: not the holder; polls to acquire.
//! `Held`: confirmed - the last heartbeat matched the expected pid. `Unconfirmed`: acquired,
//! but the most recent heartbeat failed or timed out, so Postgres may already have ended the
//! session and released the lock to a peer; treated as "not dispatching" the same as
//! `NotHeld` (a gate reads it that way), but still heartbeats rather than polling to acquire,
//! since re-acquiring while still holding would corrupt the re-entrant counter. A matching
//! heartbeat returns it to `Held`; a bounded run of failures exits instead.

use std::sync::Arc;

use anyhow::Context;
use sha2::{Digest, Sha256};
use sqlx::{Connection, PgConnection};
use tokio::sync::{watch, Mutex};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::config::settings::DispatcherLockConfig;
use crate::metrics;

/// Reserves this `classid` for the whole-database dispatch lock. The per-request
/// `pg_advisory_xact_lock(bigint)` form (`repositories::utils::compute_advisory_lock_id`)
/// packs its SHA256-derived key into the same classid/objid pair internally, so any fixed
/// classid could in principle collide by chance (1 in 2^32 per request); this constant just
/// keeps the two uses documented and distinct rather than colliding by accident. Spells
/// "HA_1" in ASCII.
const DISPATCHER_LOCK_CLASSID: i32 = 0x4841_5F31;

/// Mints `owner_epoch` (migration `20260825130000`). Postgres is the only state shared by
/// every pod, so this - not a pod name, hostname, wall clock, or backend pid - is what makes
/// a successor's epoch always compare greater than any predecessor's, across restarts.
const DISPATCHER_EPOCH_SEQUENCE: &str = "dispatcher_epoch_seq";

/// Whether this process currently holds the dispatch lock. Read synchronously via
/// [`DispatcherLock::state`] or awaited via [`DispatcherLock::subscribe`] - the gate that
/// reads this is later work. See the module docs for the three states' meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockState {
    Held,
    /// Acquired, but the last heartbeat failed - a gate must treat this as not dispatching.
    Unconfirmed,
    NotHeld,
}

/// Derive the schema-scoped `objid` half of the lock key from `current_schema()`. Full `i32`
/// range, uniformly distributed; a collision only means two different schemas would
/// (harmlessly, if it ever happened) serialize on each other's dispatch lock.
fn hash_schema_key(schema: &str) -> i32 {
    let digest = Sha256::digest(schema.as_bytes());
    i32::from_be_bytes(digest[..4].try_into().unwrap())
}

struct Inner {
    /// `None` only after [`DispatcherLock::release_last`] has consumed it.
    conn: Option<PgConnection>,
    /// Set once, at first acquisition; used to detect a swapped session (see module docs).
    held_pid: Option<i32>,
    consecutive_failures: u32,
    /// Mirrors `DispatcherLock::epoch_rx` inside the mutex, so [`DispatcherLock::poll_tick`]
    /// and [`DispatcherLock::heartbeat_tick`] can tell whether minting already succeeded
    /// without a redundant read of the watch channel.
    epoch: Option<i64>,
}

/// Handle to the dispatcher lock. Cheap to clone - clones share the same dedicated connection
/// and state, guarded by an async mutex since [`DispatcherLock::run`] and
/// [`DispatcherLock::release_last`] never execute concurrently (the latter runs only after the
/// former's task has been drained).
#[derive(Clone)]
pub struct DispatcherLock {
    inner: Arc<Mutex<Inner>>,
    state_tx: Arc<watch::Sender<LockState>>,
    state_rx: watch::Receiver<LockState>,
    /// Minted once per successful acquisition (fast path in [`DispatcherLock::poll_tick`],
    /// retried in [`DispatcherLock::heartbeat_tick`] until it succeeds); `None` whenever
    /// `state_rx` reads `NotHeld`. The step-6 sweep is the intended reader, via
    /// [`DispatcherLock::current_epoch`].
    epoch_tx: Arc<watch::Sender<Option<i64>>>,
    epoch_rx: watch::Receiver<Option<i64>>,
    config: DispatcherLockConfig,
    classid: i32,
    objid: i32,
}

impl DispatcherLock {
    /// Connect the dedicated connection and resolve the lock key. Does not attempt
    /// acquisition - call [`DispatcherLock::run`] for that.
    pub async fn connect(
        config: &DispatcherLockConfig,
        database_url: &str,
    ) -> anyhow::Result<Self> {
        let mut conn =
            tokio::time::timeout(config.connect_timeout, PgConnection::connect(database_url))
                .await
                .context("dispatcher lock: connect timed out")?
                .context("dispatcher lock: failed to open dedicated connection")?;

        let schema: Option<String> = tokio::time::timeout(
            config.heartbeat_timeout,
            sqlx::query_scalar::<_, Option<String>>("SELECT current_schema()").fetch_one(&mut conn),
        )
        .await
        .context("dispatcher lock: current_schema() query timed out")?
        .context("dispatcher lock: failed to read current_schema()")?;
        let schema = schema.context(
            "dispatcher lock: current_schema() is NULL - search_path resolves to no \
             accessible schema, refusing to fall back to a shared key",
        )?;

        let objid = config
            .key_override
            .unwrap_or_else(|| hash_schema_key(&schema));
        info!(
            schema,
            classid = DISPATCHER_LOCK_CLASSID,
            objid,
            "Dispatcher lock key resolved"
        );

        let (state_tx, state_rx) = watch::channel(LockState::NotHeld);
        let (epoch_tx, epoch_rx) = watch::channel(None);
        metrics::set_dispatcher_lock_held(false);

        Ok(Self {
            inner: Arc::new(Mutex::new(Inner {
                conn: Some(conn),
                held_pid: None,
                consecutive_failures: 0,
                epoch: None,
            })),
            state_tx: Arc::new(state_tx),
            state_rx,
            epoch_tx: Arc::new(epoch_tx),
            epoch_rx,
            config: config.clone(),
            classid: DISPATCHER_LOCK_CLASSID,
            objid,
        })
    }

    /// Current lock state, read synchronously (no await).
    pub fn state(&self) -> LockState {
        *self.state_rx.borrow()
    }

    /// Subscribe to lock-state changes. Step 7's gate is the intended reader.
    pub fn subscribe(&self) -> watch::Receiver<LockState> {
        self.state_rx.clone()
    }

    /// Subscribe to epoch changes. `startup.rs`'s bounded wait for this pod's first acquisition
    /// (so startup recovery has a real epoch to fence its re-dispatched writes with - see the
    /// call site) is the intended reader - and must watch this, not [`Self::subscribe`]:
    /// `state_tx` and `epoch_tx` are two independent watch channels, and which one changes
    /// first is not fixed. The fast path in `poll_tick` mints the epoch before setting `Held`;
    /// the mint-retry path in `heartbeat_tick` sets `Held` on the *first* confirmed heartbeat
    /// and only mints afterward if minting had not yet succeeded. A waiter watching `state_rx`
    /// on that second path wakes with the epoch still `None` and has to wait for a whole extra
    /// heartbeat interval before the mint's own retry fires - which can outrun a wait budget
    /// sized against the fast path alone.
    pub fn subscribe_epoch(&self) -> watch::Receiver<Option<i64>> {
        self.epoch_rx.clone()
    }

    /// Current dispatcher generation, read synchronously (no await). `None` until this pod has
    /// acquired the lock at least once in this process's lifetime; `Some` from that acquisition
    /// onward for the rest of the process's life - including through `Unconfirmed`, where it
    /// deliberately keeps returning the last real value rather than going stale-safe to `None`,
    /// and through and after [`Self::release_last`], which deliberately does not clear it
    /// either (see that method's doc comment). Acquisition is the only thing that ever changes
    /// it; a failed heartbeat only ever moves `state()`, never this.
    ///
    /// This is why nothing that reads `current_epoch()` may treat `Some` as "I am the current
    /// holder" - a gate must check `state() == Held` for that. What this getter guarantees is
    /// narrower and is what every reader below actually needs: this pod's real epoch from its
    /// last acquisition, monotonic across restarts, regardless of whether it still holds the
    /// lock or has released it. The step-6 sweep stamps a claim with it. The step-8
    /// write-fencing in the request repositories and `ChainCursorRepository` reads it on every
    /// send-decision write, where returning `None` once state moves off `Held` - whether to
    /// `Unconfirmed` or all the way to a released `NotHeld` - would be actively worse, not
    /// safer: a write made in that window carries this pod's real, once-valid epoch either way,
    /// and it is the row's own `owner_epoch` predicate - not this getter - that decides whether
    /// a successor has since claimed it. A row's fence only starts refusing this pod's writes
    /// once some successor's claim has actually landed and stamped a newer epoch onto it; until
    /// then, an ex-holder's own writes keep succeeding and keep the row's `updated_at` fresh,
    /// same as when it genuinely still held the lock - the epoch alone never announces the
    /// loss, only a later write's refusal does.
    pub fn current_epoch(&self) -> Option<i64> {
        *self.epoch_rx.borrow()
    }

    fn set_state(&self, state: LockState) {
        metrics::set_dispatcher_lock_held(matches!(state, LockState::Held));
        // No receiver is an error only once the process is already tearing down.
        let _ = self.state_tx.send(state);
    }

    fn set_epoch(&self, epoch: Option<i64>) {
        let _ = self.epoch_tx.send(epoch);
    }

    /// Mint the next epoch value from the dedicated Postgres sequence. Called opportunistically
    /// wherever a query on the dedicated connection just succeeded (see [`Self::poll_tick`] and
    /// [`Self::heartbeat_tick`]) rather than only once, so a transient failure on the first
    /// attempt is retried on the next heartbeat instead of leaving this holder without an
    /// epoch - and therefore without a working sweep - for the rest of its acquisition.
    async fn mint_epoch(&self, conn: &mut PgConnection) -> anyhow::Result<i64> {
        tokio::time::timeout(
            self.config.heartbeat_timeout,
            sqlx::query_scalar::<_, i64>("SELECT nextval($1)")
                .bind(DISPATCHER_EPOCH_SEQUENCE)
                .fetch_one(&mut *conn),
        )
        .await
        .context("epoch mint timed out")?
        .context("epoch mint query failed")
    }

    /// Try to acquire, once. Only ever called while not already holding - the lock is
    /// re-entrant per session, so a second `pg_try_advisory_lock` on the same key from a
    /// session that already holds it would just increment a counter this module never
    /// balances with a matching unlock.
    async fn try_acquire(&self, conn: &mut PgConnection) -> anyhow::Result<bool> {
        tokio::time::timeout(
            self.config.heartbeat_timeout,
            sqlx::query_scalar::<_, bool>("SELECT pg_try_advisory_lock($1, $2)")
                .bind(self.classid)
                .bind(self.objid)
                .fetch_one(&mut *conn),
        )
        .await
        .context("try-lock timed out")?
        .context("try-lock query failed")
    }

    /// Best-effort sanity check that the lock we just took is visible in `pg_locks` under our
    /// own backend pid, with `objsubid = 2` (verified against a live Postgres 17.7 for the
    /// two-argument form; not documented anywhere, hence "best-effort" rather than a hard
    /// failure). Never changes lock state - Postgres already told us we hold it.
    async fn verify_in_pg_locks(&self, conn: &mut PgConnection, pid: i32) {
        let visible: Result<bool, _> = tokio::time::timeout(
            self.config.heartbeat_timeout,
            sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM pg_locks WHERE locktype = 'advisory' \
                 AND classid = $1 AND objid = $2 AND objsubid = 2 AND pid = $3)",
            )
            .bind(self.classid)
            .bind(self.objid)
            .bind(pid)
            .fetch_one(&mut *conn),
        )
        .await
        .unwrap_or(Ok(false));

        if !visible.unwrap_or(false) {
            error!(
                alert = true,
                classid = self.classid,
                objid = self.objid,
                pid,
                "dispatcher lock: acquired but not visible in pg_locks under our own pid \
                 (tripwire; treating the acquisition as valid since Postgres reported success)"
            );
        }
    }

    /// Heartbeat: confirm the connection is alive and that the session backing it hasn't
    /// changed. Returns the observed pid.
    async fn heartbeat(&self, conn: &mut PgConnection) -> anyhow::Result<i32> {
        tokio::time::timeout(
            self.config.heartbeat_timeout,
            sqlx::query_scalar::<_, i32>("SELECT pg_backend_pid()").fetch_one(&mut *conn),
        )
        .await
        .context("heartbeat timed out")?
        .context("heartbeat query failed")
    }

    /// Long-running loop: while not holding, poll for acquisition every `poll_interval`;
    /// while holding - `Held` or `Unconfirmed`, we still hold the session-level lock either
    /// way - heartbeat every `heartbeat_interval`. `Unconfirmed` must never route back to
    /// polling: re-acquiring on a session that already holds the key would corrupt the
    /// re-entrant counter. Exits the process hard on loss - see the module docs. Stops
    /// (returning normally, lock still held or not) once `shutdown` fires; the caller
    /// releases afterwards via [`DispatcherLock::release_last`].
    pub async fn run(&self, shutdown: CancellationToken) {
        let mut poll_ticker = tokio::time::interval(self.config.poll_interval);
        poll_ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        let mut heartbeat_ticker = tokio::time::interval(self.config.heartbeat_interval);
        heartbeat_ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            let holding = self.state() != LockState::NotHeld;
            tokio::select! {
                _ = poll_ticker.tick(), if !holding => self.poll_tick().await,
                _ = heartbeat_ticker.tick(), if holding => self.heartbeat_tick().await,
                _ = shutdown.cancelled() => {
                    info!("Dispatcher lock loop stopping (shutdown)");
                    return;
                }
            }
        }
    }

    /// One acquisition attempt while not holding. Only records the state transition here -
    /// it does not itself confirm the backend pid, so a readback failure right after
    /// acquiring can never turn into a second `try_acquire` call on a session that already
    /// holds the key (which the re-entrant counter would accept silently, but which nothing
    /// here ever balances with a matching extra unlock). [`Self::heartbeat_tick`] confirms
    /// the pid on the first heartbeat after this transition (`held_pid` starts `None`).
    async fn poll_tick(&self) {
        let mut guard = self.inner.lock().await;
        // Disjoint field borrows: `conn`, `consecutive_failures` and `epoch` need to be usable
        // independently below (`&mut *guard` alone would tie both to one borrow of the
        // whole `Inner`, through the `MutexGuard` deref).
        let Inner {
            conn,
            consecutive_failures,
            epoch,
            ..
        } = &mut *guard;
        let Some(conn) = conn.as_mut() else {
            return;
        };

        match self.try_acquire(conn).await {
            Ok(true) => {
                *consecutive_failures = 0;
                // Best-effort fast path: mint the epoch now, on the same connection, rather
                // than waiting for the first heartbeat. A failure here is not fatal - the row
                // still says we hold the lock - so it just falls back to the retry in
                // `heartbeat_tick`, logged there rather than here.
                if let Ok(minted) = self.mint_epoch(conn).await {
                    *epoch = Some(minted);
                    self.set_epoch(Some(minted));
                }
                drop(guard);
                self.set_state(LockState::Held);
                info!("Dispatcher lock acquired, confirming backend pid on next heartbeat");
            }
            Ok(false) => {
                // Someone else holds it. Not loss - we never held it to begin with.
            }
            Err(e) => {
                warn!(error = %e, "dispatcher lock: try-lock failed");
                *consecutive_failures += 1;
                let failures = *consecutive_failures;
                self.exit_if_past_failure_bound(failures);
            }
        }
    }

    /// One heartbeat while holding (`Held` or `Unconfirmed`). `held_pid == None` means no
    /// heartbeat has ever succeeded since acquiring - possibly because the very first one
    /// failed (`Unconfirmed`, pid still unrecorded) - so this records the pid (and runs the
    /// `pg_locks` sanity check) instead of comparing against one.
    async fn heartbeat_tick(&self) {
        let mut guard = self.inner.lock().await;
        let Inner {
            conn,
            held_pid,
            consecutive_failures,
            epoch,
        } = &mut *guard;
        let Some(conn) = conn.as_mut() else {
            return;
        };
        let expected_pid = *held_pid;

        match self.heartbeat(conn).await {
            Ok(pid) if expected_pid.is_none() => {
                *held_pid = Some(pid);
                *consecutive_failures = 0;
                // Idempotent if already `Held`; recovers from `Unconfirmed` if the very
                // first heartbeat since acquiring had failed.
                self.set_state(LockState::Held);
                self.verify_in_pg_locks(conn, pid).await;
                info!(pid, "Dispatcher lock acquisition confirmed");
                // Retry path for the fast-path mint in `poll_tick`: if that attempt never
                // ran (this is the first heartbeat since a fresh acquisition) or failed, this
                // is still on the dedicated connection and still holding, so it is safe to
                // try again here.
                if epoch.is_none() {
                    match self.mint_epoch(conn).await {
                        Ok(minted) => {
                            *epoch = Some(minted);
                            self.set_epoch(Some(minted));
                        }
                        Err(e) => {
                            warn!(error = %e, "dispatcher lock: epoch mint failed, retrying next heartbeat")
                        }
                    }
                }
            }
            Ok(pid) if Some(pid) == expected_pid => {
                *consecutive_failures = 0;
                // Idempotent if already `Held`; recovers from `Unconfirmed` otherwise.
                self.set_state(LockState::Held);
                if epoch.is_none() {
                    match self.mint_epoch(conn).await {
                        Ok(minted) => {
                            *epoch = Some(minted);
                            self.set_epoch(Some(minted));
                        }
                        Err(e) => {
                            warn!(error = %e, "dispatcher lock: epoch mint failed, retrying next heartbeat")
                        }
                    }
                }
            }
            Ok(pid) => {
                // The session backing this connection changed under us. `PgConnection`
                // does not transparently reconnect, so this should be impossible - it's a
                // cheap tripwire. Whatever the lock table says, we no longer trust that we
                // hold it: exit immediately rather than risk a second active dispatcher.
                error!(
                    alert = true,
                    expected_pid,
                    observed_pid = pid,
                    "dispatcher lock: backend pid changed under an active holder, exiting"
                );
                drop(guard);
                hard_exit();
            }
            Err(e) => {
                warn!(error = %e, "dispatcher lock: heartbeat failed");
                // First failure since holding: demote so a gate stops treating us as the
                // dispatcher, without giving up the session (still heartbeating, not
                // re-polling - see `run`). Postgres may already have released the lock to a
                // peer; we just can't tell from here until a heartbeat succeeds or the bound
                // below is hit.
                self.set_state(LockState::Unconfirmed);
                *consecutive_failures += 1;
                let failures = *consecutive_failures;
                self.exit_if_past_failure_bound(failures);
            }
        }
    }

    /// Shared bounded-retry accounting for a failed query on the dedicated connection,
    /// whether that happened while polling to acquire or while heartbeating as the holder.
    /// The connection is dead or unreachable either way; a live PID can't be reconnected
    /// into (see module docs), so past the threshold there is nothing left to do but exit.
    /// Takes the already-incremented count rather than the guard, so the caller's disjoint
    /// field borrows don't have to be given up just to report it.
    fn exit_if_past_failure_bound(&self, consecutive_failures: u32) {
        if consecutive_failures >= self.config.heartbeat_failures_before_exit {
            error!(
                alert = true,
                consecutive_failures,
                "dispatcher lock: dedicated connection unhealthy past the configured bound, \
                 exiting"
            );
            hard_exit();
        }
    }

    /// Release the lock, last, after every other shutdown step. `pg_advisory_unlock` then
    /// close, both bounded so a hung release cannot eat the shutdown grace period. Never
    /// called on the loss path - the lock is already gone there, and that path never returns
    /// (it hard-exits).
    ///
    /// Sets `state()` to `NotHeld` but deliberately leaves `current_epoch()` alone rather than
    /// clearing it to `None`. Detached work (a tx send, a gateway-response handler) is
    /// abandoned rather than drained at shutdown - see `startup.rs`'s shutdown-rationale
    /// comment - so something can still be self-serving `current_epoch()` for a write after
    /// this call returns. Clearing it would make that write's fence predicate degrade to
    /// `owner_epoch IS NULL`, refusing this pod's own still-valid write against the very row it
    /// legitimately owns and forcing a successor to redo it - a self-inflicted duplicate this
    /// process's own real epoch would otherwise have avoided. The process is exiting either
    /// way, so nothing here ever reads this epoch as if it were current again after release.
    pub async fn release_last(&self) {
        let mut guard = self.inner.lock().await;
        let Some(mut conn) = guard.conn.take() else {
            return;
        };

        if let Some(pid) = guard.held_pid.take() {
            let unlocked = tokio::time::timeout(
                self.config.heartbeat_timeout,
                sqlx::query_scalar::<_, bool>("SELECT pg_advisory_unlock($1, $2)")
                    .bind(self.classid)
                    .bind(self.objid)
                    .fetch_one(&mut conn),
            )
            .await;
            match unlocked {
                Ok(Ok(true)) => info!(pid, "Dispatcher lock released"),
                Ok(Ok(false)) => warn!(pid, "dispatcher lock: unlock reported not held"),
                Ok(Err(e)) => warn!(pid, error = %e, "dispatcher lock: unlock query failed"),
                Err(_) => warn!(pid, "dispatcher lock: unlock timed out"),
            }
        }

        drop(guard);
        self.set_state(LockState::NotHeld);

        if let Err(e) = tokio::time::timeout(self.config.heartbeat_timeout, conn.close()).await {
            warn!(error = %e, "dispatcher lock: close timed out");
        }
    }
}

/// Isolated for tests: swapping this out is exactly the shape a unit test would need to
/// assert the loss path without actually terminating the test process, but nothing here
/// currently does - kept as a named seam rather than an inline call for that reason.
fn hard_exit() -> ! {
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_schema_key_is_deterministic() {
        assert_eq!(hash_schema_key("public"), hash_schema_key("public"));
    }

    #[test]
    fn hash_schema_key_differs_across_schemas() {
        assert_ne!(hash_schema_key("public"), hash_schema_key("test_abc"));
    }

    #[test]
    fn hash_schema_key_spans_the_full_i32_range() {
        let mut has_positive = false;
        let mut has_negative = false;
        for i in 0..1000 {
            match hash_schema_key(&format!("schema_{i}")).cmp(&0) {
                std::cmp::Ordering::Greater => has_positive = true,
                std::cmp::Ordering::Less => has_negative = true,
                std::cmp::Ordering::Equal => {}
            }
        }
        assert!(has_positive);
        assert!(has_negative);
    }

    #[test]
    fn classid_is_disjoint_from_the_per_request_lock_docs() {
        // Not a runtime guarantee (see module docs) - just pins the constant so a future
        // edit doesn't silently pick a value that reads like a mistake.
        assert_eq!(DISPATCHER_LOCK_CLASSID, 0x4841_5F31);
    }
}
