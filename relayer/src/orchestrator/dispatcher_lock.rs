//! The dispatcher lock: a session-level Postgres advisory lock that elects which of N HA
//! replica pods dispatches. Every pod serves HTTP and is Ready; only the holder dispatches,
//! enforced by each subsystem reading the [`DispatchGate`] this module hands out.
//!
//! # Dedicated connection
//!
//! Session-level advisory locks die with their Postgres session, and a pooled connection is
//! recycled - taking the lock with it. This module owns one [`sqlx::PgConnection`], outside
//! both pools, for as long as it tracks or holds the lock.
//!
//! # Lock key
//!
//! The two-argument `pg_try_advisory_lock(classid, objid)` form keeps this key space disjoint
//! from the per-request `pg_advisory_xact_lock(bigint)` in
//! [`crate::store::sql::repositories::utils`]. Advisory locks are database-wide, not
//! schema-scoped, and the test harness isolates concurrent runs by schema on one database, so
//! `objid` is derived from `current_schema()`, read once on the dedicated connection:
//! production sets no `search_path`, resolves `public`, and shares one key.
//!
//! # Loss detection
//!
//! sqlx 0.8.6 sets no TCP keepalive, so a query on a black-holed socket waits out TCP
//! retransmits (minutes); every query on the dedicated connection is wrapped in
//! [`tokio::time::timeout`]. Two failure shapes (see [`DispatcherLock::run`]): lost to a peer
//! or a swapped session exits immediately - a peer is already dispatching; an unhealthy
//! connection (heartbeat error or timeout) retries up to a bounded number of consecutive
//! failures, since it may be transient.
//!
//! The server side needs its own bound: a holder's node dying without closing its socket (no
//! FIN, no RST) leaves Postgres holding the session - and the lock - until server-side TCP
//! keepalive reaps them, over two hours at Linux defaults, while every standby's try-lock
//! returns a clean `false` and every accepted request sits `queued`. So
//! [`DispatcherLock::connect`] sets the session-scoped `idle_session_timeout` GUC (Postgres
//! 14+) on the dedicated connection. A healthy holder never idles past `heartbeat_interval`,
//! and the config rejects a bound below it. A reaped-but-alive ex-holder is the stale writer
//! the `owner_epoch` fences refuse; its own failing heartbeats walk it out through the
//! bounded-failure exit.
//!
//! Loss exits via a hard [`std::process::exit`], not the graceful shutdown path: shutdown
//! would keep application-pool writers alive for seconds while a successor may be acting.
//!
//! # States
//!
//! [`LockState`]: `NotHeld` polls to acquire. `Held` is confirmed by a pid-matching
//! heartbeat. `Unconfirmed` (last heartbeat failed; Postgres may already have released the
//! lock) reads as "not dispatching" but keeps heartbeating rather than re-polling -
//! re-acquiring while still holding would corrupt the re-entrant counter.

use std::sync::Arc;

use anyhow::Context;
use sha2::{Digest, Sha256};
use sqlx::{Connection, PgConnection};
use tokio::sync::{watch, Mutex};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::config::settings::DispatcherLockConfig;
use crate::metrics;

/// Reserves this `classid` for the dispatch lock, keeping it documented and distinct from
/// the per-request `pg_advisory_xact_lock` key space, which packs a SHA256-derived key into
/// the same classid/objid pair. Spells "HA_1" in ASCII.
pub const DISPATCHER_LOCK_CLASSID: i32 = 0x4841_5F31;

/// Mints `owner_epoch` (migration `20260825130000`). Postgres is the only state shared by
/// every pod, so this - not a pod name, hostname, wall clock, or backend pid - is what makes
/// a successor's epoch always compare greater than any predecessor's, across restarts.
const DISPATCHER_EPOCH_SEQUENCE: &str = "dispatcher_epoch_seq";

/// Whether this process currently holds the dispatch lock. Read synchronously via
/// [`DispatcherLock::state`]; the dispatch gate reads it through [`DispatchGate`]. See the
/// module docs for the three states' meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockState {
    Held,
    /// Acquired, but the last heartbeat failed - a gate must treat this as not dispatching.
    Unconfirmed,
    NotHeld,
}

/// Keeps a test gate's watch channels open for its lifetime - see
/// [`DispatchGate::open_for_tests`]. A production gate leaves this `None`, because the lock owns
/// the senders.
type TestGateSenders = Arc<(watch::Sender<LockState>, watch::Sender<Option<i64>>)>;

/// The dispatch gate: a read-only view of "may this pod drive work right now, and under which
/// epoch", handed to every subsystem that would otherwise dispatch - listeners, tx and
/// readiness processors, cron workers, the sweep, HTTP intake. Deliberately not
/// [`DispatcherLock`] itself: no subsystem behind the gate may acquire or release.
///
/// [`Self::epoch`] is `Some` only when the lock is `Held` **and** an epoch has been minted:
/// - `Unconfirmed` reads closed - Postgres may already have released the lock to a peer.
/// - `Held` with no epoch reads closed - intake would stamp `owner_epoch = NULL`, and the
///   holder's own sweep claims unowned rows on sight, re-driving a request already running.
#[derive(Clone)]
pub struct DispatchGate {
    state_rx: watch::Receiver<LockState>,
    epoch_rx: watch::Receiver<Option<i64>>,
    /// Set only by [`DispatchGate::open_for_tests`]: production senders live in the lock, but
    /// a test gate must keep its channels alive itself or `wait_open` would park forever.
    _test_senders: Option<TestGateSenders>,
}

/// Shared by [`DispatchGate::epoch`] and [`DispatcherLock::dispatching_epoch`] so the two can
/// never drift into disagreeing about what "may dispatch" means.
fn dispatching_epoch_of(
    state_rx: &watch::Receiver<LockState>,
    epoch_rx: &watch::Receiver<Option<i64>>,
) -> Option<i64> {
    match *state_rx.borrow() {
        LockState::Held => *epoch_rx.borrow(),
        LockState::Unconfirmed | LockState::NotHeld => None,
    }
}

impl DispatchGate {
    /// A permanently open gate, for tests that exercise a gated subsystem's own behaviour
    /// rather than the gating. Production gates come from [`DispatcherLock::gate`], which is
    /// the only way to get one that ever closes.
    pub fn open_for_tests(epoch: i64) -> Self {
        let (state_tx, state_rx) = watch::channel(LockState::Held);
        let (epoch_tx, epoch_rx) = watch::channel(Some(epoch));
        Self {
            state_rx,
            epoch_rx,
            _test_senders: Some(Arc::new((state_tx, epoch_tx))),
        }
    }

    /// The epoch to stamp on work this pod is about to drive itself, or `None` when the gate is
    /// closed. See the type's doc comment for why both halves of the condition matter.
    ///
    /// Not to be confused with [`DispatcherLock::current_epoch`], which keeps returning this
    /// pod's last real epoch through `Unconfirmed` and past release, and is what the write
    /// fence needs. The distinction: `current_epoch` answers "which epoch does a write of mine
    /// carry", this answers "may I start driving something new".
    pub fn epoch(&self) -> Option<i64> {
        dispatching_epoch_of(&self.state_rx, &self.epoch_rx)
    }

    /// Whether the gate is open, read synchronously (no await).
    pub fn is_open(&self) -> bool {
        self.epoch().is_some()
    }

    /// Resolves once the gate *closes*, immediately if it already is closed. The mirror of
    /// [`Self::wait_open`], for a subsystem holding something it must give up when this pod
    /// stops being the dispatcher - the WebSocket listener's subscription is the case that
    /// needs it, since a subscription cannot be paused, only dropped and re-established.
    pub async fn wait_closed(&self) {
        let mut state_rx = self.state_rx.clone();
        let mut epoch_rx = self.epoch_rx.clone();
        loop {
            if !self.is_open() {
                return;
            }
            let closed_channel = tokio::select! {
                changed = state_rx.changed() => changed.is_err(),
                changed = epoch_rx.changed() => changed.is_err(),
            };
            if closed_channel {
                // The lock handle is gone, so this pod is certainly not dispatching any more.
                return;
            }
        }
    }

    /// Resolves once the gate is open, immediately if it already is. Cancel-safe, and intended
    /// to be raced against a shutdown token by every caller - it never resolves on its own
    /// while the gate stays closed, which for a standby pod is its whole life.
    pub async fn wait_open(&self) {
        let mut state_rx = self.state_rx.clone();
        let mut epoch_rx = self.epoch_rx.clone();
        loop {
            if self.is_open() {
                return;
            }
            // Both channels are watched: `Held` and the mint are two independent sends whose
            // order is not fixed (see `DispatcherLock::subscribe_epoch`), and the gate needs
            // both. The receivers are cloned before the first check, so a send landing between
            // the check and the await marks them changed rather than being missed.
            let closed = tokio::select! {
                changed = state_rx.changed() => changed.is_err(),
                changed = epoch_rx.changed() => changed.is_err(),
            };
            if closed {
                break;
            }
        }
        // A closed channel means the lock handle itself is gone - unreachable while the
        // process runs. Park rather than return: returning would open the gate for a caller
        // about to dispatch, and every caller races this against shutdown.
        warn!("dispatch gate: lock state channel closed, parking rather than opening the gate");
        std::future::pending().await
    }
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
    /// `state_rx` reads `NotHeld`. The sweep is the intended reader, via
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

        // Session-scoped (`set_config`'s third argument is `is_local`), on this one connection
        // and no pooled one. Bounds how long Postgres keeps a session whose client vanished
        // without closing the socket - see the module docs' loss-detection section. Applies to
        // a session idle outside a transaction, which this one always is between its queries.
        let idle_session_timeout = format!("{}ms", config.idle_session_timeout.as_millis());
        tokio::time::timeout(
            config.heartbeat_timeout,
            sqlx::query("SELECT set_config('idle_session_timeout', $1, false)")
                .bind(&idle_session_timeout)
                .execute(&mut conn),
        )
        .await
        .context("dispatcher lock: setting idle_session_timeout timed out")?
        .context("dispatcher lock: failed to set idle_session_timeout")?;
        info!(
            idle_session_timeout,
            "Dispatcher lock session idle timeout applied"
        );

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

    /// A read-only [`DispatchGate`] over this lock, for the subsystems that must only act while
    /// this pod is the confirmed dispatcher.
    pub fn gate(&self) -> DispatchGate {
        DispatchGate {
            state_rx: self.state_rx.clone(),
            epoch_rx: self.epoch_rx.clone(),
            _test_senders: None,
        }
    }

    /// The epoch to stamp on work this pod is about to drive itself - `Some` only while the
    /// gate is open. Same value as [`DispatchGate::epoch`]; this exists so the request
    /// repositories can stamp intake without carrying a second handle. Distinct from
    /// [`Self::current_epoch`], which is what an *already-started* write fences with.
    pub fn dispatching_epoch(&self) -> Option<i64> {
        dispatching_epoch_of(&self.state_rx, &self.epoch_rx)
    }

    /// Subscribe to epoch changes. `startup.rs`'s bounded wait for the first acquisition is
    /// the intended reader, and it must watch this, not [`Self::state`]: `Held` and the mint
    /// are independent sends, and on the `heartbeat_tick` retry path `Held` lands first - a
    /// state-watcher would wake with the epoch still `None` and wait out a whole extra
    /// heartbeat interval, which can outrun a wait budget sized against the fast path.
    pub fn subscribe_epoch(&self) -> watch::Receiver<Option<i64>> {
        self.epoch_rx.clone()
    }

    /// This pod's dispatcher generation, read synchronously. `None` until the first
    /// acquisition; from then on it keeps the last minted value for the rest of the process's
    /// life - through `Unconfirmed` and past [`Self::release_last`]. Only acquisition ever
    /// changes it.
    ///
    /// `Some` therefore never means "still the holder" - a gate checks `state() == Held` for
    /// that. This is the fencing value: the sweep stamps claims with it and every
    /// send-decision write carries it, and the row's own `owner_epoch` predicate - not this
    /// getter - decides whether a successor has since taken the row.
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

    /// Mint the next epoch from the dedicated Postgres sequence. Called wherever a query on
    /// the dedicated connection just succeeded, so a failed first attempt is retried on the
    /// next heartbeat instead of leaving the holder without a working sweep.
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

    /// Try to acquire, once. Never called while already holding: the lock is re-entrant per
    /// session, and a second grab would increment a counter nothing here unlocks twice.
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

    /// Best-effort sanity check that the lock is visible in `pg_locks` under our backend pid.
    /// `objsubid = 2` for the two-argument form is observed behaviour, not documented - hence
    /// best-effort, never a hard failure. Never changes lock state.
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

    /// One acquisition attempt while not holding. Records only the state transition: pid
    /// confirmation happens on the first heartbeat (`held_pid` starts `None`), so a readback
    /// failure here can never trigger a second `try_acquire` on a session that already holds
    /// the key - the re-entrant counter would accept it, and nothing unlocks twice.
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
                // Held by someone else: a completed round trip, which is all this counter
                // measures. Without the reset a standby's transient errors accumulate over
                // its whole life until an unrelated one exits it.
                *consecutive_failures = 0;
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
                std::process::exit(1);
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

    /// Shared bounded-retry accounting for a failed query on the dedicated connection. The
    /// connection is dead either way and a live PID cannot be reconnected into, so past the
    /// threshold the only move is exit. Takes the count, not the guard, so callers keep
    /// their disjoint field borrows.
    fn exit_if_past_failure_bound(&self, consecutive_failures: u32) {
        if consecutive_failures >= self.config.heartbeat_failures_before_exit {
            error!(
                alert = true,
                consecutive_failures,
                "dispatcher lock: dedicated connection unhealthy past the configured bound, \
                 exiting"
            );
            std::process::exit(1);
        }
    }

    /// Release the lock, last, after every other shutdown step. `pg_advisory_unlock` then
    /// close, both bounded so a hung release cannot eat the shutdown grace period. Never
    /// called on the loss path - the lock is already gone there, and that path never returns
    /// (it hard-exits).
    ///
    /// Sets `state()` to `NotHeld` but leaves `current_epoch()` alone: detached work
    /// abandoned at shutdown can still fence a write with it, and clearing it would degrade
    /// that write's predicate to `owner_epoch IS NULL`, refusing this pod's own still-valid
    /// write and forcing a successor to redo it.
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

    /// Build a gate over channels the test drives directly. The two states below are the ones
    /// no integration test can reach: `Unconfirmed` needs the lock's dedicated connection to
    /// fail, and the `Held`-without-epoch window needs a mint to fail, and the harness can
    /// force neither.
    fn gate_at(state: LockState, epoch: Option<i64>) -> DispatchGate {
        let (state_tx, state_rx) = watch::channel(state);
        let (epoch_tx, epoch_rx) = watch::channel(epoch);
        DispatchGate {
            state_rx,
            epoch_rx,
            _test_senders: Some(Arc::new((state_tx, epoch_tx))),
        }
    }

    #[test]
    fn gate_is_open_only_when_held_with_a_minted_epoch() {
        assert_eq!(gate_at(LockState::Held, Some(7)).epoch(), Some(7));
        assert!(gate_at(LockState::Held, Some(7)).is_open());
    }

    #[test]
    fn gate_is_closed_while_unconfirmed_even_with_an_epoch() {
        // The lock may already have been released to a peer by Postgres, so anything newly
        // dispatched here could be a second dispatcher's work.
        let gate = gate_at(LockState::Unconfirmed, Some(7));
        assert_eq!(gate.epoch(), None);
        assert!(!gate.is_open());
    }

    #[test]
    fn gate_is_closed_while_held_without_a_minted_epoch() {
        // Dispatching here would stamp `owner_epoch = NULL` on intake, and the holder's own
        // sweep claims unowned rows on sight - so it would re-drive what it is already driving.
        let gate = gate_at(LockState::Held, None);
        assert_eq!(gate.epoch(), None);
        assert!(!gate.is_open());
    }

    #[test]
    fn gate_is_closed_when_not_held() {
        assert!(!gate_at(LockState::NotHeld, None).is_open());
        // A stale epoch left over from a previous acquisition does not reopen it.
        assert!(!gate_at(LockState::NotHeld, Some(7)).is_open());
    }

    #[tokio::test]
    async fn gate_wait_open_returns_immediately_when_already_open() {
        let gate = gate_at(LockState::Held, Some(1));
        tokio::time::timeout(std::time::Duration::from_millis(100), gate.wait_open())
            .await
            .expect("an open gate must not make a caller wait");
    }

    #[tokio::test]
    async fn gate_wait_open_wakes_on_the_epoch_being_minted() {
        // The `Held`-then-mint order, which is the `heartbeat_tick` retry path: a waiter that
        // watched only lock state would wake here with the epoch still `None`.
        let (state_tx, state_rx) = watch::channel(LockState::Held);
        let (epoch_tx, epoch_rx) = watch::channel(None);
        let gate = DispatchGate {
            state_rx,
            epoch_rx,
            _test_senders: None,
        };
        assert!(!gate.is_open(), "no epoch yet");

        let waiter = tokio::spawn({
            let gate = gate.clone();
            async move { gate.wait_open().await }
        });
        epoch_tx.send(Some(42)).expect("send epoch");

        tokio::time::timeout(std::time::Duration::from_secs(1), waiter)
            .await
            .expect("wait_open must wake when the epoch is minted")
            .expect("waiter task panicked");
        assert_eq!(gate.epoch(), Some(42));
        drop(state_tx);
    }

    #[tokio::test]
    async fn gate_wait_closed_wakes_when_the_lock_is_lost() {
        let (state_tx, state_rx) = watch::channel(LockState::Held);
        let (epoch_tx, epoch_rx) = watch::channel(Some(1));
        let gate = DispatchGate {
            state_rx,
            epoch_rx,
            _test_senders: None,
        };

        let waiter = tokio::spawn({
            let gate = gate.clone();
            async move { gate.wait_closed().await }
        });
        state_tx
            .send(LockState::Unconfirmed)
            .expect("send state change");

        tokio::time::timeout(std::time::Duration::from_secs(1), waiter)
            .await
            .expect("wait_closed must wake when the gate closes")
            .expect("waiter task panicked");
        drop(epoch_tx);
    }
}
