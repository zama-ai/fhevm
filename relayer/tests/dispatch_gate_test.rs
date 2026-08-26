//! The dispatch gate.
//!
//! These are the only tests that put a *running relayer* on the non-dispatcher side of the
//! gate, which is the state the whole HA design is about and the one a single-instance harness
//! cannot otherwise reach. The trick is `dispatcher_lock.key_override`: the test takes the
//! advisory lock on that key itself, on its own connection, before the relayer starts. The
//! relayer then polls forever without acquiring - exactly as a standby pod does while a peer
//! holds the lock - and releasing the test's own lock hands it over for real.
//!
//! What is deliberately *not* asserted here: two live relayers actually contending for the
//! same lock.

mod common;

use common::flows::public_decrypt;
use common::utils::{fast_timing, row_state, TestSetup};
use fhevm_relayer::http::endpoints::v2::types::error::ApiResponseStatus;
use fhevm_relayer::orchestrator::DISPATCHER_LOCK_CLASSID;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

/// An `objid` unlikely to collide with any schema-derived key from a concurrently running test
/// (those are SHA256-derived from `test_<uuid>`), and distinct per test process.
fn test_lock_key() -> i32 {
    // Derived from the pid rather than random: reproducible within a run, and nextest gives
    // each test its own process, so two tests in this file cannot collide either.
    (std::process::id() as i32).saturating_abs() | 0x4000_0000
}

/// Holds `pg_advisory_lock` on the test's key for as long as the pool lives, standing in for
/// the peer pod that owns the lock. Dropping the pool closes the session, which is how
/// Postgres releases a session-level advisory lock - the same mechanism a crashed peer relies
/// on.
async fn hold_lock(database_url: &str, objid: i32) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .min_connections(1)
        .connect(database_url)
        .await
        .expect("Failed to connect the lock-holding pool");

    let acquired: bool = sqlx::query_scalar("SELECT pg_try_advisory_lock($1, $2)")
        .bind(DISPATCHER_LOCK_CLASSID)
        .bind(objid)
        .fetch_one(&pool)
        .await
        .expect("Failed to take the advisory lock");
    assert!(
        acquired,
        "the test must hold the lock before the relayer starts, or it is not testing a standby"
    );

    pool
}

/// A relayer that is not the dispatcher still accepts an HTTP
/// request and persists it, drives nothing at all, and once it becomes the dispatcher its sweep
/// picks that request up and carries it to completion.
///
/// The unowned row is what connects the two halves: intake stamps `owner_epoch = NULL` because
/// the gate was closed, and `NULL` is exactly what the sweep claims on sight.
#[tokio::test]
async fn test_a_standby_accepts_without_driving_then_the_sweep_drives_it_after_handover() {
    let objid = test_lock_key();

    // The lock has to be held before the relayer's first poll, so this cannot use the schema
    // the harness creates - it takes the key on the shared database instead, which is where
    // advisory locks live anyway (they are database-wide, never schema-scoped).
    let base_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5433/relayer_db".to_string());
    let peer = hold_lock(&base_url, objid).await;

    let setup = TestSetup::new_with_settings(|settings| {
        settings.dispatcher_lock.key_override = Some(objid);
        fast_timing(settings);
    })
    .await
    .expect("Failed to start the relayer");

    let db = PgPoolOptions::new()
        .max_connections(1)
        .connect(&setup.settings.storage.sql_database_url)
        .await
        .expect("Failed to connect the assertion pool");

    // Accepted, not driven.
    let payload = public_decrypt::create_public_decrypt_payload();
    let handles = public_decrypt::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = public_decrypt::random_plaintext_values(handles.len());
    setup.fhevm_mock.on_public_decrypt_success(
        handles,
        plaintext_values,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let ext_job_id = public_decrypt::submit_request(&setup, &payload).await;

    // Long enough that a dispatching relayer would have moved the row well past `queued` (the
    // same flow completes in about a second when the pod holds the lock), and long enough for
    // several sweep ticks to have run and declined to claim.
    tokio::time::sleep(Duration::from_secs(3)).await;

    let (status, owner_epoch, attempts) = row_state(&db, &ext_job_id).await;
    assert_eq!(
        status, "queued",
        "a pod that is not the dispatcher must not drive the request it accepted"
    );
    assert_eq!(
        owner_epoch, None,
        "intake must leave the row unowned, which is what makes it claimable on sight"
    );
    assert_eq!(
        attempts, 0,
        "the sweep must not claim anything while the gate is closed"
    );

    // Hand the lock over. Closing the peer's session releases it exactly as a dead pod would.
    peer.close().await;

    // The relayer acquires within a poll interval, mints an epoch, and its sweep claims the
    // unowned row and re-dispatches it.
    let (http_status, body) = public_decrypt::poll_until_terminal(&setup, &ext_job_id).await;
    assert_eq!(
        http_status,
        reqwest::StatusCode::OK,
        "the request accepted while standing by must complete once this pod is the dispatcher"
    );
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(
        body.result.is_some(),
        "the decryption result must be stored"
    );

    let (_, owner_epoch, attempts) = row_state(&db, &ext_job_id).await;
    assert!(
        owner_epoch.is_some(),
        "the claim must stamp the epoch it dispatched under"
    );
    assert_eq!(
        attempts, 1,
        "one claim, not a re-dispatch per tick: the row is under the sweep's own epoch after \
         the first claim, and its own epoch is what the claim never matches"
    );

    setup.shutdown().await;
}
