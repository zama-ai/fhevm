//! Dispatcher lock tests.
//!
//! Exercises the module directly against a schema-isolated database, rather than through the
//! full relayer harness: this is the real test of the schema-derived key, since many relayers
//! (here, many `DispatcherLock` connections) run concurrently against one shared Postgres in
//! CI. If the key derivation stopped being schema-scoped, these would hang or a lock would
//! never be acquired.

mod common;

use common::test_schema::TestSchema;
use fhevm_relayer::config::settings::DispatcherLockConfig;
use fhevm_relayer::orchestrator::{DispatcherLock, LockState};
use std::time::Duration;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

/// Short intervals so the tests converge quickly without being flaky.
fn fast_config() -> DispatcherLockConfig {
    DispatcherLockConfig {
        poll_interval: Duration::from_millis(50),
        heartbeat_interval: Duration::from_millis(50),
        heartbeat_timeout: Duration::from_secs(2),
        heartbeat_failures_before_exit: 3,
        connect_timeout: Duration::from_secs(5),
        key_override: None,
    }
}

/// Spawn `lock.run(...)` as a background task, returning its shutdown token and join handle.
fn spawn_run(lock: &DispatcherLock) -> (CancellationToken, tokio::task::JoinHandle<()>) {
    let shutdown = CancellationToken::new();
    let handle = tokio::spawn({
        let lock = lock.clone();
        let shutdown = shutdown.clone();
        async move { lock.run(shutdown).await }
    });
    (shutdown, handle)
}

async fn wait_until_held(lock: &DispatcherLock, budget: Duration) {
    timeout(budget, async {
        while lock.state() != LockState::Held {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("lock did not converge on Held within budget");
}

/// Two dedicated connections against the same schema (hence the same lock key) must never
/// both report `Held` at once - Postgres only grants `pg_try_advisory_lock` to one session.
#[tokio::test]
async fn test_two_connections_cannot_both_hold_the_same_key() {
    let schema = TestSchema::new()
        .await
        .expect("Failed to create test schema");
    let config = fast_config();

    let lock_a = DispatcherLock::connect(&config, &schema.database_url())
        .await
        .expect("connect a");
    let lock_b = DispatcherLock::connect(&config, &schema.database_url())
        .await
        .expect("connect b");

    let (shutdown_a, task_a) = spawn_run(&lock_a);
    let (shutdown_b, task_b) = spawn_run(&lock_b);

    let winner_is_a = timeout(Duration::from_secs(5), async {
        loop {
            if lock_a.state() == LockState::Held {
                return true;
            }
            if lock_b.state() == LockState::Held {
                return false;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("neither connection acquired the lock within budget");

    // A few more poll cycles for the loser: it must never also transition to Held.
    tokio::time::sleep(config.poll_interval * 5).await;

    let (winner, loser) = if winner_is_a {
        (&lock_a, &lock_b)
    } else {
        (&lock_b, &lock_a)
    };
    assert_eq!(winner.state(), LockState::Held);
    assert_eq!(
        loser.state(),
        LockState::NotHeld,
        "a second connection must never also hold the same key"
    );

    shutdown_a.cancel();
    shutdown_b.cancel();
    task_a.await.unwrap();
    task_b.await.unwrap();
    lock_a.release_last().await;
    lock_b.release_last().await;
}

/// Once the holder releases, a fresh connection on the same key can acquire it.
#[tokio::test]
async fn test_released_lock_can_be_reacquired() {
    let schema = TestSchema::new()
        .await
        .expect("Failed to create test schema");
    let config = fast_config();

    let lock_a = DispatcherLock::connect(&config, &schema.database_url())
        .await
        .expect("connect a");
    let (shutdown_a, task_a) = spawn_run(&lock_a);
    wait_until_held(&lock_a, Duration::from_secs(5)).await;

    shutdown_a.cancel();
    task_a.await.unwrap();
    lock_a.release_last().await;

    let lock_b = DispatcherLock::connect(&config, &schema.database_url())
        .await
        .expect("connect b");
    let (shutdown_b, task_b) = spawn_run(&lock_b);
    wait_until_held(&lock_b, Duration::from_secs(5)).await;

    shutdown_b.cancel();
    task_b.await.unwrap();
    lock_b.release_last().await;
}

/// `current_epoch()` is `None` before acquisition and `Some` once `Held` - the step-6 sweep
/// (`sweep::run_tick`) treats a `None` epoch while `Held` as "not yet minted, skip this tick".
#[tokio::test]
async fn test_epoch_is_none_before_acquisition_and_some_once_held() {
    let schema = TestSchema::new()
        .await
        .expect("Failed to create test schema");
    let config = fast_config();

    let lock = DispatcherLock::connect(&config, &schema.database_url())
        .await
        .expect("connect");
    assert_eq!(lock.current_epoch(), None, "no epoch before acquisition");

    let (shutdown, task) = spawn_run(&lock);
    wait_until_held(&lock, Duration::from_secs(5)).await;

    assert!(
        lock.current_epoch().is_some(),
        "epoch must be minted once Held"
    );

    shutdown.cancel();
    task.await.unwrap();
    lock.release_last().await;
}

/// `owner_epoch` must be monotonic across acquisitions (the fencing property it exists for):
/// a successor that reacquires the same key after a release always mints a strictly greater
/// value than its predecessor did.
#[tokio::test]
async fn test_epoch_increases_across_reacquire() {
    let schema = TestSchema::new()
        .await
        .expect("Failed to create test schema");
    let config = fast_config();

    let lock_a = DispatcherLock::connect(&config, &schema.database_url())
        .await
        .expect("connect a");
    let (shutdown_a, task_a) = spawn_run(&lock_a);
    wait_until_held(&lock_a, Duration::from_secs(5)).await;
    let epoch_a = lock_a.current_epoch().expect("epoch minted for a");

    shutdown_a.cancel();
    task_a.await.unwrap();
    lock_a.release_last().await;
    assert_eq!(
        lock_a.current_epoch(),
        Some(epoch_a),
        "release_last must not clear current_epoch() - abandoned detached work self-serving \
         it after shutdown still needs this pod's real, last-held epoch to fence its writes \
         correctly (see release_last's doc comment)"
    );

    let lock_b = DispatcherLock::connect(&config, &schema.database_url())
        .await
        .expect("connect b");
    let (shutdown_b, task_b) = spawn_run(&lock_b);
    wait_until_held(&lock_b, Duration::from_secs(5)).await;
    let epoch_b = lock_b.current_epoch().expect("epoch minted for b");

    assert!(
        epoch_b > epoch_a,
        "successor epoch {epoch_b} must be greater than predecessor epoch {epoch_a}"
    );

    shutdown_b.cancel();
    task_b.await.unwrap();
    lock_b.release_last().await;
}
