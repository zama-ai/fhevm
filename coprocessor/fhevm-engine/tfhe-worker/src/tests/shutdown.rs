use std::time::{Duration, Instant};

use fhevm_engine_common::drift_containment::{block_ct_computations, DRIFT_CONTAINMENT_BARRIER};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{Database as ListenerDatabase, Handle};

use crate::tests::dependence_chain::chain_state;
use crate::tests::event_helpers::{
    insert_event_in_chain, insert_trivial_encrypt, next_handle, next_handle_with_type, scalar_flag,
    setup_event_harness_with, wait_until_computed, zero_address, EventHarness,
};

const FHE_UINT64: i32 = 5;

/// A worker stopped during a cycle (SIGTERM, scale-in) finishes and commits
/// that cycle's batch, starts no other, and hands its chain back at once
/// instead of holding it until the lease expires.
#[tokio::test]
async fn test_stop_finishes_batch_and_releases_chain() {
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness_with(|args| {
        args.disable_dcid_locking = false;
        // The lease manager needs connections beside the listener and the batch.
        args.pg_pool_max_connections = 6;
        // One transaction per batch, so the chain outlives the running batch.
        args.work_items_batch_size = 1;
    })
    .await
    .unwrap();

    // Server-key loading does not yield, so the stop would wait for it.
    // A first chain puts the key in the worker's cache.
    stage_chain(&listener_db, false).await;
    wait_until_computed(&app).await.unwrap();

    // Hold every batch at its drift-containment permit, so the stop lands
    // inside a cycle that is past its shutdown check.
    let mut blocker = pool.begin().await.unwrap();
    block_ct_computations(&mut blocker).await.unwrap();
    let (chain, second) = stage_chain(&listener_db, true).await;
    let started = Instant::now();
    while !batch_waits_on_permit(&pool).await {
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "worker never started a batch"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    app.stop_worker();
    let stopped = Instant::now();
    // The close request reaches the worker's token through another task.
    tokio::time::sleep(Duration::from_millis(200)).await;
    blocker.rollback().await.unwrap();
    while chain_state(&pool, chain.as_slice()).await.1.is_some() || !is_computed(&pool, chain).await
    {
        assert!(
            stopped.elapsed() < Duration::from_secs(10),
            "stopped worker did not commit its batch and release its chain"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    assert!(
        !is_computed(&pool, second).await,
        "a stopped worker must not start another batch"
    );
    let (status, _, _) = chain_state(&pool, chain.as_slice()).await;
    assert_eq!(status, "updated", "released chain must be acquirable again");
}

/// A chain whose first transaction does two cheap multiplications of trivial
/// ciphertexts. With `with_second`, a second transaction squares their stored
/// result, a real multiplication. Returns both transaction ids; the first is
/// also the chain id.
async fn stage_chain(listener_db: &ListenerDatabase, with_second: bool) -> (Handle, Handle) {
    let (first, second) = (next_handle(), next_handle());
    let mut tx = listener_db
        .new_transaction()
        .await
        .unwrap()
        .expect("new_transaction() returns Some on a live stack");
    let (mut counter, factor) = (
        next_handle_with_type(FHE_UINT64),
        next_handle_with_type(FHE_UINT64),
    );
    insert_trivial_encrypt(listener_db, &mut tx, first, 3, FHE_UINT64, counter, false)
        .await
        .unwrap();
    insert_trivial_encrypt(listener_db, &mut tx, first, 1, FHE_UINT64, factor, false)
        .await
        .unwrap();
    for last in [false, true] {
        let product = next_handle_with_type(FHE_UINT64);
        let event = TfheContractEvents::FheMul(TfheContract::FheMul {
            caller: zero_address(),
            lhs: counter,
            rhs: factor,
            scalarByte: scalar_flag(false),
            result: product,
        });
        insert_event_in_chain(listener_db, &mut tx, first, first, event, last)
            .await
            .unwrap();
        counter = product;
    }
    if with_second {
        let event = TfheContractEvents::FheMul(TfheContract::FheMul {
            caller: zero_address(),
            lhs: counter,
            rhs: counter,
            scalarByte: scalar_flag(false),
            result: next_handle_with_type(FHE_UINT64),
        });
        insert_event_in_chain(listener_db, &mut tx, second, first, event, true)
            .await
            .unwrap();
    }
    sqlx::query(
        "INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at) \
         VALUES ($1, 'updated', NOW())",
    )
    .bind(first.to_vec())
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
    (first, second)
}

async fn batch_waits_on_permit(pool: &sqlx::PgPool) -> bool {
    sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM pg_locks l JOIN pg_database d ON d.oid = l.database \
         WHERE d.datname = current_database() AND l.locktype = 'advisory' AND NOT l.granted \
         AND l.objid = ($1::bigint & 4294967295)::oid)",
    )
    .bind(DRIFT_CONTAINMENT_BARRIER)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn is_computed(pool: &sqlx::PgPool, tx_id: Handle) -> bool {
    sqlx::query_scalar("SELECT bool_and(is_completed) FROM computations WHERE transaction_id = $1")
        .bind(tx_id.to_vec())
        .fetch_one(pool)
        .await
        .unwrap()
}
