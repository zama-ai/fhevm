//! Two real relayer processes against one shared database schema.
//!
//! `dispatch_gate_test.rs` puts a running relayer on the standby side of the gate by faking
//! the peer: the test itself holds the advisory lock on `dispatcher_lock.key_override`, on its
//! own connection, and no second relayer process ever exists. This file is what that file's
//! own doc comment names as deliberately out of scope for it: two live `TestSetup` instances,
//! joined onto one schema via `TestSetup::join`, genuinely contending for the same
//! schema-derived lock key.
//!
//! # Why probe rather than ask
//!
//! Both instances run in this one test process, and `ensure_global_init` uses a process-wide
//! `OnceLock` metrics registry (`startup.rs`), so metrics cannot tell the two apart - counters
//! from one pod's dispatch are indistinguishable from the other's. Nothing is exposed over HTTP
//! either: `/healthz` reports readiness, never lock state. So "which of the two pods holds the
//! lock right now" is never asked directly; it is inferred from a real request's own behaviour
//! via [`probe_dispatch`], and every assertion below is against database rows or HTTP
//! responses, never a metric.
//!
//! # Teardown order
//!
//! Teardown always shuts the *joined* instance down first and the *owning* instance last - see
//! `TestSetup::join`'s doc comment for why.

mod common;

use common::flows::public_decrypt;
use common::utils::{fast_timing, row_state, TestSetup};
use fhevm_relayer::http::endpoints::v2::types::error::ApiResponseStatus;
use fhevm_relayer::orchestrator::{DISPATCHER_LOCK_CLASSID, UNCLAIMED_EPOCH};
use sha2::{Digest, Sha256};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

/// Mirrors `orchestrator::dispatcher_lock::hash_schema_key`, which is private to the crate and
/// so cannot be called from here directly. Recomputing it is what lets
/// [`test_exactly_one_of_two_instances_becomes_the_dispatcher`] confirm the election directly
/// against `pg_locks` - not just infer it from completed work - restricted to the one schema
/// this test's two instances actually share (`pg_locks` is database-wide, and other test
/// binaries may be holding unrelated locks under the same fixed classid concurrently).
fn hash_schema_key(schema: &str) -> i32 {
    let digest = Sha256::digest(schema.as_bytes());
    i32::from_be_bytes(digest[..4].try_into().unwrap())
}

/// Read the shared schema's own name back and hash it exactly as the dispatcher lock does, so
/// the `objid` used below always matches whatever key the two instances actually resolved.
async fn schema_lock_objid(pool: &PgPool) -> i32 {
    let schema: String = sqlx::query_scalar("SELECT current_schema()")
        .fetch_one(pool)
        .await
        .expect("Failed to read current_schema()");
    hash_schema_key(&schema)
}

/// Count of sessions currently granted the dispatcher lock's key, restricted to `objid` (see
/// [`schema_lock_objid`]) so a concurrently running, unrelated test binary's own advisory lock
/// under the same fixed classid can never be mistaken for one of this test's two instances.
async fn granted_lock_count(pool: &PgPool, objid: i32) -> i64 {
    sqlx::query_scalar(
        r#"
        SELECT count(*) FROM pg_locks
        WHERE locktype = 'advisory' AND classid = $1 AND objid = $2 AND granted = true
        "#,
    )
    .bind(DISPATCHER_LOCK_CLASSID)
    .bind(objid)
    .fetch_one(pool)
    .await
    .expect("Failed to query pg_locks")
}

/// Poll `granted_lock_count(pool, objid)` until it reports exactly one holder, or `budget`
/// elapses. Preferred over a fixed `sleep` wherever the assertion is about something eventually
/// becoming true - see `common::flows::public_decrypt::poll_until_terminal` for the same shape
/// against HTTP.
async fn wait_for_single_holder(pool: &PgPool, objid: i32, budget: Duration) -> bool {
    let deadline = tokio::time::Instant::now() + budget;
    loop {
        if granted_lock_count(pool, objid).await == 1 {
            return true;
        }
        if tokio::time::Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// Submit a fresh public-decrypt request through `setup`'s own HTTP port, drive it to
/// completion, and read back how it got there: `(driven_directly, epoch)`.
///
/// `driven_directly` is `attempts == 0` - the row's `attempts` column is only ever touched by
/// the sweep's claim `UPDATE` (see `sweep.rs`'s module docs), never by the direct-dispatch path
/// an accepting pod takes when it is itself the confirmed dispatcher (`public_decrypt.rs`'s
/// handler calls `orchestrator.dispatch_event` inline in that case, without going through a
/// claim at all). So `attempts == 0` after completion means `setup` *was* the dispatcher at
/// submission time; `attempts == 1` means some peer's sweep had to claim what `setup` merely
/// accepted - meaning `setup` was the standby, and whichever pod's sweep claimed it is the
/// dispatcher.
///
/// `epoch` is the `owner_epoch` the completed row carries either way. That makes this useful
/// for more than identification: called against the pod already established as the dispatcher,
/// it also hands back that pod's current minted epoch - which is exactly what
/// [`test_handover_on_graceful_shutdown_drives_the_leavers_incomplete_work`] needs before
/// taking that pod down, to later prove its successor minted a strictly higher one.
async fn probe_dispatch(setup: &TestSetup, pool: &PgPool) -> (bool, i64) {
    let payload = public_decrypt::create_public_decrypt_payload();
    let handles = public_decrypt::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = public_decrypt::random_plaintext_values(handles.len());
    setup.fhevm_mock.on_public_decrypt_success(
        handles,
        plaintext_values,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let ext_job_id = public_decrypt::submit_request(setup, &payload).await;
    let (http_status, body) = public_decrypt::poll_until_terminal(setup, &ext_job_id).await;
    assert_eq!(
        http_status,
        reqwest::StatusCode::OK,
        "a probe request must complete regardless of which pod ends up driving it"
    );
    assert_eq!(body.status, ApiResponseStatus::Succeeded);

    let (_, owner_epoch, attempts) = row_state(pool, &ext_job_id).await;
    (attempts == 0, owner_epoch)
}

/// A pool connected to the two instances' shared schema, for assertions that read rows or
/// `pg_locks` directly rather than through either pod's HTTP surface.
async fn assertion_pool(setup: &TestSetup) -> PgPool {
    PgPoolOptions::new()
        .max_connections(1)
        .connect(&setup.settings.storage.sql_database_url)
        .await
        .expect("Failed to connect the assertion pool")
}

/// Election: two real relayers pointed at one schema resolve one shared lock key on their own
/// (no `key_override` - see `TestSetup::join`'s doc comment), and Postgres grants it to exactly
/// one of them. Both still serve HTTP and are Ready regardless of which one wins, since
/// the dispatch design is precisely "every pod serves HTTP; only the holder dispatches."
#[tokio::test]
async fn test_exactly_one_of_two_instances_becomes_the_dispatcher() {
    let setup_a = TestSetup::new_with_settings(fast_timing)
        .await
        .expect("Failed to start the first instance");
    let setup_b = TestSetup::join(&setup_a, fast_timing)
        .await
        .expect("Failed to start the joined instance");

    let pool = assertion_pool(&setup_a).await;
    let objid = schema_lock_objid(&pool).await;

    let elected = wait_for_single_holder(&pool, objid, Duration::from_secs(5)).await;
    assert!(
        elected,
        "exactly one of the two instances must hold the shared, schema-derived lock key"
    );

    // Both pods serve HTTP and are Ready immediately, independent of dispatcher role.
    for port in [setup_a.http_port, setup_b.http_port] {
        let resp = reqwest::get(format!("http://localhost:{port}/healthz"))
            .await
            .expect("Failed to GET /healthz");
        assert_eq!(
            resp.status(),
            reqwest::StatusCode::OK,
            "every pod must be Ready regardless of dispatcher role"
        );
    }

    // A few more poll/heartbeat cycles: the count must never flip to two or drop to zero.
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_eq!(
        granted_lock_count(&pool, objid).await,
        1,
        "the election must stay settled on exactly one holder"
    );

    // Joined instance first, owner last - see the module doc comment.
    setup_b.shutdown().await;
    setup_a.shutdown().await;
}

/// The live-standby case: a request submitted to the pod that is *not* the
/// dispatcher is accepted and persisted unowned, same as the faked-peer case in
/// `dispatch_gate_test.rs`, but here it is a second real relayer process's own sweep - polling
/// and heartbeating against the same Postgres, under its own epoch - that claims and completes
/// it.
///
/// Two independent properties pin the single driver down, and neither one covers the other.
/// `attempts == 1` counts claims: the sweep's claim `UPDATE` is the only writer of that column,
/// so a re-dispatch on every tick or a second sweep claiming the same row both show up there.
/// One gateway transaction counts sends, which is the half `attempts` is blind to: the
/// in-process dispatch an accepting pod takes when it is the confirmed dispatcher never claims,
/// so a standby that also drove this row would send a second decryption request while leaving
/// `attempts` at 1.
#[tokio::test]
async fn test_standby_accepts_then_the_holders_sweep_drives_it() {
    let setup_a = TestSetup::new_with_settings(fast_timing)
        .await
        .expect("Failed to start the first instance");
    let setup_b = TestSetup::join(&setup_a, fast_timing)
        .await
        .expect("Failed to start the joined instance");

    let pool = assertion_pool(&setup_a).await;

    let (a_is_holder, _) = probe_dispatch(&setup_a, &pool).await;
    let (_holder, standby) = if a_is_holder {
        (&setup_a, &setup_b)
    } else {
        (&setup_b, &setup_a)
    };

    let payload = public_decrypt::create_public_decrypt_payload();
    let handles = public_decrypt::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = public_decrypt::random_plaintext_values(handles.len());
    standby.fhevm_mock.on_public_decrypt_success(
        handles,
        plaintext_values,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    // Both pods share one mock, so this counts every decryption request either of them sends.
    let sends_before = standby.fhevm_mock.decryption_transaction_count();

    let ext_job_id = public_decrypt::submit_request(standby, &payload).await;

    let (http_status, body) = public_decrypt::poll_until_terminal(standby, &ext_job_id).await;
    assert_eq!(
        http_status,
        reqwest::StatusCode::OK,
        "a request accepted by the standby must still complete, once the holder's sweep claims it"
    );
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(
        body.result.is_some(),
        "the decryption result must be stored"
    );

    // Several more sweep ticks, so a late extra claim or a second driver's send lands before
    // the two counts are read.
    tokio::time::sleep(Duration::from_millis(500)).await;

    let (_, owner_epoch, attempts) = row_state(&pool, &ext_job_id).await;
    assert!(
        owner_epoch > UNCLAIMED_EPOCH,
        "the claim must stamp the epoch the holder dispatched it under"
    );
    assert_eq!(
        attempts, 1,
        "one claim - not a re-dispatch per sweep tick, and not a second sweep claiming the \
         same row"
    );
    assert_eq!(
        standby.fhevm_mock.decryption_transaction_count() - sends_before,
        1,
        "one pod sends the decryption request - a standby that also drove this row in process \
         would send a second while leaving attempts at 1"
    );

    // Joined instance first, owner last - see the module doc comment.
    setup_b.shutdown().await;
    setup_a.shutdown().await;
}

/// Why `dispatcher_lock.release_last()` exists: so a graceful
/// shutdown hands the lock to a peer only after the leaving pod has truly stopped working
/// (`startup.rs`'s shutdown sequence). This exercises that promise end to end with two live
/// processes: the current holder is sent through its real shutdown sequence, and the survivor
/// must pick up the slack - acquire the now-released lock, mint a strictly higher epoch (the
/// epoch sequence is one shared, monotonic counter across the whole database - see
/// `dispatcher_lock.rs`'s module docs), and drive to completion a row the leaver never got to
/// touch.
///
/// The row is submitted through the *survivor's* port, and only after the leaver's shutdown has
/// already been started - not before - so it starts unowned deterministically (the leaver's own
/// sweep never runs another tick once `dequeue_shutdown` cancels it, and the row does not exist
/// yet at the moment that cancellation fires) rather than racing the leaver's last sweep tick
/// for it.
#[tokio::test]
async fn test_handover_on_graceful_shutdown_drives_the_leavers_incomplete_work() {
    let setup_a = TestSetup::new_with_settings(fast_timing)
        .await
        .expect("Failed to start the first instance");
    let setup_b = TestSetup::join(&setup_a, fast_timing)
        .await
        .expect("Failed to start the joined instance");

    let pool = assertion_pool(&setup_a).await;

    // Identify the current holder and, in the same probe, capture its current epoch.
    let (a_is_holder, holder_epoch) = probe_dispatch(&setup_a, &pool).await;
    let (holder, survivor) = if a_is_holder {
        (&setup_a, &setup_b)
    } else {
        (&setup_b, &setup_a)
    };

    // Start the holder's real shutdown sequence - `intake_shutdown`/`dequeue_shutdown` fire
    // immediately, so its sweep will not tick again, and its dispatcher lock loop stops
    // polling/heartbeating. The lock itself is released last, after task drain (`release_last`
    // in `startup.rs`).
    holder.begin_shutdown();

    // Submitted after the shutdown has started, through the survivor: intake there always
    // leaves a row unowned (its own gate is closed until it acquires), so this row is
    // guaranteed to start exactly as "the leaver's incomplete work" would - unowned, undriven -
    // with no dependence on catching the leaver mid-flight.
    let payload = public_decrypt::create_public_decrypt_payload();
    let handles = public_decrypt::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = public_decrypt::random_plaintext_values(handles.len());
    survivor.fhevm_mock.on_public_decrypt_success(
        handles,
        plaintext_values,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );
    let ext_job_id = public_decrypt::submit_request(survivor, &payload).await;

    let (http_status, body) = public_decrypt::poll_until_terminal(survivor, &ext_job_id).await;
    assert_eq!(
        http_status,
        reqwest::StatusCode::OK,
        "the survivor must drive to completion whatever the leaver left behind, once it \
         acquires the lock the leaver released"
    );
    assert_eq!(body.status, ApiResponseStatus::Succeeded);

    let (_, owner_epoch, attempts) = row_state(&pool, &ext_job_id).await;
    assert!(
        owner_epoch > holder_epoch,
        "the survivor must mint a strictly higher epoch than the pod it replaced \
         (survivor epoch {owner_epoch}, leaver epoch {holder_epoch})"
    );
    assert_eq!(
        attempts, 1,
        "one claim by the survivor's sweep - not a re-dispatch per tick"
    );

    // Joined instance first, owner last, regardless of which one ended up as leaver or
    // survivor - see the module doc comment. `shutdown()` on the leaver is a harmless no-op
    // re-cancel of its already-cancelled token, awaiting a handle that has already finished.
    setup_b.shutdown().await;
    setup_a.shutdown().await;
}
