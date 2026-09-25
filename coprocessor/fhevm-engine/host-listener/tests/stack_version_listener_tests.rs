//! Regression coverage for the stack-version listener's pool lifecycle
//! (inventory case REG-01-LISTENER-POOL-REBIND).
//!
//! The defect this pins: `run_stack_version_listener` is handed one `Pool`
//! clone and keeps it for its lifetime, while `Database::reconnect` installs a
//! fresh pool and CLOSES the one it displaces -- on every ingestion retry. One
//! transient database error therefore left the listener holding a closed pool,
//! `recv()` could only keep failing, and the loop answered by warning and
//! sleeping a second, forever. Nothing else noticed: the listener is a side
//! task, the process stayed up and reported healthy, and the only consequence
//! was that the stack quietly stopped reacting to cutover.
//!
//! The unit test in `versioning.rs` establishes one thing -- that a closed pool
//! produces `ListenerPoolClosed` rather than a retry -- against a lazy pool
//! that never touches a database. That is the error classification, not the
//! lifecycle. These tests drive the real thing: a real Postgres, the real
//! `Database`, the production `reconnect` path, and the supervisor
//! (`spawn_stack_version_listener`) that is supposed to rebind.
//!
//! Run with a database:
//!   cargo test -p host-listener --test stack_version_listener_tests
use std::sync::Arc;
use std::time::{Duration, Instant};

use fhevm_engine_common::chain_id::ChainId;

use fhevm_engine_common::versioning::{
    StackMode, EVENT_STACK_VERSION_UPGRADED,
};
use fhevm_engine_common::CONSENSUS_PROTOCOL_VERSION;
use host_listener::database::tfhe_event_propagate::{
    spawn_stack_version_listener, Database,
};
use sqlx::{Executor, PgPool};
use test_harness::instance::ImportMode;
use tokio_util::sync::CancellationToken;

const CHAIN_ID: u64 = 12345;

/// Long enough for a respawn (1s backoff) plus a subscribe and a query, short
/// enough that a genuinely wedged listener fails the test rather than hanging
/// it.
const REACTION_TIMEOUT: Duration = Duration::from_secs(20);

/// Ensures the `versioning` singleton exists and sets the live stack version.
///
/// The listener's whole job is to react to this row changing, so a test that
/// did not control it would be asserting against whatever the migrations left.
async fn set_live_consensus_version(pool: &PgPool, version: i64) {
    pool.execute(
        sqlx::query(
            "INSERT INTO versioning (singleton, consensus_version, stack_version) VALUES (TRUE, $1, $2)
             ON CONFLICT (singleton) DO UPDATE SET consensus_version = EXCLUDED.consensus_version",
        )
        .bind(version)
        .bind(fhevm_engine_common::STACK_VERSION),
    )
    .await
    .expect("versioning row should be writable");
}

async fn notify_upgrade(pool: &PgPool) {
    pool.execute(sqlx::query(&format!(
        "NOTIFY {EVENT_STACK_VERSION_UPGRADED}"
    )))
    .await
    .expect("NOTIFY should be accepted");
}

/// Waits for a condition, polling, and fails with a message rather than hanging.
async fn wait_until<F>(what: &str, mut condition: F)
where
    F: FnMut() -> bool,
{
    let deadline = Instant::now() + REACTION_TIMEOUT;
    loop {
        if condition() {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "timed out after {}s waiting for: {what}",
            REACTION_TIMEOUT.as_secs()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

/// Counts the listener's own backend connections, so a leak shows up as a
/// growing number rather than as a vague slowdown.
async fn listener_backends(pool: &PgPool) -> i64 {
    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM pg_stat_activity WHERE datname = current_database() AND query LIKE 'LISTEN%'",
    )
    .fetch_one(pool)
    .await
    .expect("listener connections should be observable")
}

/// The listener reacts, survives its pool being replaced, and reacts again.
///
/// The middle step is the regression: `Database::reconnect` is the production
/// path, and before the fix the listener was left bound to the pool it closed.
#[tokio::test]
async fn rebinds_to_the_pool_that_replaced_its_own() {
    let instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .expect("a test database");
    let url = instance.db_url.clone();
    let mut db = Database::new(&url, ChainId::try_from(CHAIN_ID).unwrap(), 16)
        .await
        .expect("host-listener Database");

    // A control pool of our own, so the test can write and NOTIFY independently
    // of whichever pool the listener happens to hold.
    let control = PgPool::connect(instance.db_url())
        .await
        .expect("control pool");

    // The binary's own version is live, and the mode starts in GCS: the first
    // reconcile must then leave GCS mode, which is an observable transition.
    set_live_consensus_version(&control, i64::from(CONSENSUS_PROTOCOL_VERSION))
        .await;
    let mode = StackMode::new(true);
    let cancel = CancellationToken::new();
    let handle = spawn_stack_version_listener(
        db.clone(),
        Arc::clone(&mode),
        cancel.clone(),
    );

    // Step 1: the listener is live and reacts. It reconciles on startup as well
    // as on notification, so either path proves the subscription exists.
    notify_upgrade(&control).await;
    wait_until("the listener to leave GCS mode on a notification", || {
        !mode.gcs_mode()
    })
    .await;

    // Step 2: replace the pool through the production path. This closes the
    // pool the listener was handed.
    let before = db.pool().await;
    db.reconnect().await;
    let after = db.pool().await;
    assert!(
        before.is_closed(),
        "reconnect must close the pool it displaces; without that this test would not be \
         exercising the defect"
    );
    assert!(!after.is_closed(), "the replacement pool must be open");

    // Step 3: a transition the listener must still see. Before the fix the task
    // was spinning on the closed pool and this never landed.
    set_live_consensus_version(
        &control,
        i64::from(CONSENSUS_PROTOCOL_VERSION) + 1,
    )
    .await;
    notify_upgrade(&control).await;
    wait_until(
        "the rebound listener to pause the stack after the version moved away",
        || mode.is_paused(),
    )
    .await;

    cancel.cancel();
    tokio::time::timeout(REACTION_TIMEOUT, handle)
        .await
        .expect("the supervised listener must exit when cancelled")
        .expect("the listener task must not panic");
}

/// Repeated replacement leaks no listener connections and does not deadlock.
///
/// `reconnect` runs on every ingestion retry, so repeated replacement must keep
/// releasing old listener connections and accepting notifications.
#[tokio::test]
async fn survives_repeated_pool_replacement_without_leaking_listeners() {
    let instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .expect("a test database");
    let url = instance.db_url.clone();
    let mut db = Database::new(&url, ChainId::try_from(CHAIN_ID).unwrap(), 16)
        .await
        .expect("host-listener Database");
    let control = PgPool::connect(instance.db_url())
        .await
        .expect("control pool");
    set_live_consensus_version(&control, i64::from(CONSENSUS_PROTOCOL_VERSION))
        .await;

    let mode = StackMode::new(false);
    let cancel = CancellationToken::new();
    let handle = spawn_stack_version_listener(
        db.clone(),
        Arc::clone(&mode),
        cancel.clone(),
    );
    // Let the first subscription establish before counting.
    notify_upgrade(&control).await;
    tokio::time::sleep(Duration::from_secs(2)).await;
    let baseline = listener_backends(&control).await;

    for round in 0..5 {
        db.reconnect().await;
        // The respawn backoff is one second; give it room and then require the
        // listener to be serving again rather than assuming it.
        set_live_consensus_version(
            &control,
            i64::from(CONSENSUS_PROTOCOL_VERSION),
        )
        .await;
        notify_upgrade(&control).await;
        tokio::time::sleep(Duration::from_secs(2)).await;
        let now = listener_backends(&control).await;
        assert!(
            now <= baseline + 1,
            "round {round}: {now} LISTEN backends against a baseline of {baseline}; each respawn \
             must replace the previous subscription rather than add one"
        );
    }

    // And it is still functional after all that, which a leak check alone would
    // not establish.
    set_live_consensus_version(
        &control,
        i64::from(CONSENSUS_PROTOCOL_VERSION) + 1,
    )
    .await;
    notify_upgrade(&control).await;
    wait_until(
        "the listener to still react after five replacements",
        || mode.is_paused(),
    )
    .await;

    cancel.cancel();
    tokio::time::timeout(REACTION_TIMEOUT, handle)
        .await
        .expect("the supervised listener must exit when cancelled")
        .expect("the listener task must not panic");
}

/// A version transition that happens DURING the disconnect window is still
/// applied.
///
/// This is the gap a notification-only design has, and it is not hypothetical:
/// LISTEN/NOTIFY is not a durable event history, so a NOTIFY delivered while
/// the subscription is being replaced is gone. The listener therefore
/// reconciles once immediately after subscribing; without that, the service
/// runs on a stale mode with nothing left to tell it otherwise -- the same
/// silent failure the closed-pool defect produced.
#[tokio::test]
async fn reconciles_a_transition_that_happened_while_disconnected() {
    let instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .expect("a test database");
    let url = instance.db_url.clone();
    let mut db = Database::new(&url, ChainId::try_from(CHAIN_ID).unwrap(), 16)
        .await
        .expect("host-listener Database");
    let control = PgPool::connect(instance.db_url())
        .await
        .expect("control pool");
    set_live_consensus_version(&control, i64::from(CONSENSUS_PROTOCOL_VERSION))
        .await;

    let mode = StackMode::new(false);
    let cancel = CancellationToken::new();
    let handle = spawn_stack_version_listener(
        db.clone(),
        Arc::clone(&mode),
        cancel.clone(),
    );
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert!(
        !mode.is_paused(),
        "the stack must not be paused while its own version is live"
    );

    // Move the live version and close the listener's pool in the same breath,
    // and deliberately do NOT notify afterwards: the notification is the thing
    // a disconnected listener would have missed.
    set_live_consensus_version(
        &control,
        i64::from(CONSENSUS_PROTOCOL_VERSION) + 1,
    )
    .await;
    db.reconnect().await;

    wait_until(
        "the rebound listener to reconcile the transition it was not notified about",
        || mode.is_paused(),
    )
    .await;

    cancel.cancel();
    tokio::time::timeout(REACTION_TIMEOUT, handle)
        .await
        .expect("the supervised listener must exit when cancelled")
        .expect("the listener task must not panic");
}

/// Cancellation stops the supervisor promptly, and not only the inner task.
#[tokio::test]
async fn cancellation_stops_the_supervisor_promptly() {
    let instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .expect("a test database");
    let url = instance.db_url.clone();
    let db = Database::new(&url, ChainId::try_from(CHAIN_ID).unwrap(), 16)
        .await
        .expect("host-listener Database");
    let control = PgPool::connect(instance.db_url())
        .await
        .expect("control pool");
    set_live_consensus_version(&control, i64::from(CONSENSUS_PROTOCOL_VERSION))
        .await;

    let cancel = CancellationToken::new();
    let handle =
        spawn_stack_version_listener(db, StackMode::new(false), cancel.clone());
    tokio::time::sleep(Duration::from_secs(1)).await;
    cancel.cancel();
    tokio::time::timeout(Duration::from_secs(10), handle)
        .await
        .expect("cancellation must stop the supervisor within ten seconds")
        .expect("the listener task must not panic");
}

/// A socket reconnect on the same pool must also recover a missed notification.
#[tokio::test]
async fn reconciles_after_the_subscription_connection_is_lost() {
    let instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .expect("a test database");
    let db = Database::new(
        &instance.db_url,
        ChainId::try_from(CHAIN_ID).unwrap(),
        16,
    )
    .await
    .expect("host-listener Database");
    let control = PgPool::connect(instance.db_url())
        .await
        .expect("control pool");
    set_live_consensus_version(&control, i64::from(CONSENSUS_PROTOCOL_VERSION))
        .await;
    let mode = StackMode::new(true);
    let cancel = CancellationToken::new();
    let handle = spawn_stack_version_listener(db, mode.clone(), cancel.clone());
    wait_until("initial subscription and reconciliation", || {
        !mode.gcs_mode()
    })
    .await;

    // No NOTIFY: only detection of the broken subscription can trigger the read.
    set_live_consensus_version(
        &control,
        i64::from(CONSENSUS_PROTOCOL_VERSION) + 1,
    )
    .await;
    let terminated: Vec<bool> = sqlx::query_scalar(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity
         WHERE datname = current_database() AND query LIKE 'LISTEN%'",
    )
    .fetch_all(&control)
    .await
    .expect("terminate subscription connection");
    assert_eq!(
        terminated,
        vec![true],
        "must terminate exactly the active subscription"
    );
    wait_until(
        "reconciliation after connection loss without pool replacement",
        || mode.is_paused(),
    )
    .await;
    cancel.cancel();
    tokio::time::timeout(REACTION_TIMEOUT, handle)
        .await
        .unwrap()
        .unwrap();
}

/// A failed initial read must be retried even if no subsequent NOTIFY arrives.
#[tokio::test]
async fn retries_failed_startup_reconciliation_without_a_notification() {
    let instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .expect("a test database");
    let db = Database::new(
        &instance.db_url,
        ChainId::try_from(CHAIN_ID).unwrap(),
        16,
    )
    .await
    .expect("host-listener Database");
    let control = PgPool::connect(instance.db_url())
        .await
        .expect("control pool");
    set_live_consensus_version(&control, i64::from(CONSENSUS_PROTOCOL_VERSION))
        .await;
    control
        .execute("ALTER TABLE versioning RENAME TO unavailable_versioning")
        .await
        .unwrap();
    let mode = StackMode::new(true);
    let cancel = CancellationToken::new();
    let handle = spawn_stack_version_listener(db, mode.clone(), cancel.clone());
    // Prove a read failed before restoring the table, using PostgreSQL's query history.
    let deadline = Instant::now() + REACTION_TIMEOUT;
    loop {
        let attempted: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM pg_stat_activity WHERE datname = current_database()
             AND query = 'SELECT consensus_version FROM versioning WHERE singleton = TRUE')"
        ).fetch_one(&control).await.unwrap();
        if attempted {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "listener did not attempt reconciliation"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(mode.gcs_mode());
    control
        .execute("ALTER TABLE unavailable_versioning RENAME TO versioning")
        .await
        .unwrap();
    wait_until("startup read to recover without a notification", || {
        !mode.gcs_mode()
    })
    .await;
    cancel.cancel();
    tokio::time::timeout(REACTION_TIMEOUT, handle)
        .await
        .unwrap()
        .unwrap();
}
