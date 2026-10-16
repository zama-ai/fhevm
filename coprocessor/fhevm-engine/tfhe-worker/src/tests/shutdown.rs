use std::time::{Duration, Instant};

use fhevm_engine_common::drift_containment::block_ct_computations;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{Database as ListenerDatabase, Handle};

use crate::tests::dependence_chain::chain_state;
use crate::tests::event_helpers::{
    insert_event, insert_trivial_encrypt, next_handle, next_handle_with_type, scalar_flag,
    scalar_u128_handle, setup_event_harness_with, wait_until_computed, zero_address, EventHarness,
};

const FHE_UINT64: i32 = 5;
// Long enough that the batch is still running when the worker is stopped.
const LONG_CHAIN: usize = 1500;

#[tokio::test]
async fn test_stop_waiting_for_containment_releases_chain() {
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness_with(|args| {
        args.disable_dcid_locking = false;
        args.pg_pool_max_connections = 6;
        args.worker_polling_interval_ms = 100;
    })
    .await
    .unwrap();

    // A missing boundary input leaves allowed work pending across iterations,
    // so the worker retains the chain even after its batch transaction ends.
    let chain_id = next_handle();
    let output = next_handle_with_type(FHE_UINT64);
    let mut tx = listener_db.new_transaction().await.unwrap().unwrap();
    insert_event(
        &listener_db,
        &mut tx,
        chain_id,
        TfheContractEvents::FheMul(TfheContract::FheMul {
            caller: zero_address(),
            lhs: next_handle_with_type(FHE_UINT64),
            rhs: scalar_u128_handle(1),
            scalarByte: scalar_flag(true),
            result: output,
        }),
        true,
    )
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at) \
         VALUES ($1, 'updated', NOW())",
    )
    .bind(chain_id.to_vec())
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();

    tokio::time::timeout(Duration::from_secs(30), async {
        while chain_state(&pool, chain_id.as_slice()).await.1.is_none() {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("worker never took the pending chain");

    let mut barrier = pool.begin().await.unwrap();
    tokio::time::timeout(
        Duration::from_secs(120),
        block_ct_computations(&mut barrier),
    )
    .await
    .expect("worker never finished its first batch")
    .unwrap();
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let waiting: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM pg_stat_activity \
                 WHERE datname = current_database() AND pid <> pg_backend_pid() \
                 AND query LIKE '%advisory_xact_lock_shared%' \
                 AND ((state = 'active' AND wait_event = 'advisory') \
                      OR (state = 'idle in transaction' \
                          AND query LIKE '%pg_try_advisory_xact_lock_shared%')))",
            )
            .fetch_one(&pool)
            .await
            .unwrap();
            if waiting {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("worker never tried the barrier again");
    assert!(chain_state(&pool, chain_id.as_slice()).await.1.is_some());

    app.stop_worker();
    tokio::time::timeout(Duration::from_secs(5), async {
        while chain_state(&pool, chain_id.as_slice()).await.1.is_some() {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("stopped worker kept its chain while waiting for containment");
    assert_eq!(chain_state(&pool, chain_id.as_slice()).await.0, "updated");
    let completed: bool =
        sqlx::query_scalar("SELECT is_completed FROM computations WHERE output_handle = $1")
            .bind(output.to_vec())
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(
        !completed,
        "the released chain must still have pending work"
    );
    // Keep containment active until after release: shutdown must not need it
    // to finish before rolling back and handing the chain back.
    barrier.rollback().await.unwrap();
}

/// A worker stopped mid-batch (SIGTERM, scale-in) hands its chain back at
/// once, instead of holding it until the lease expires.
#[tokio::test]
async fn test_stop_mid_batch_releases_chain() {
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness_with(|args| {
        args.disable_dcid_locking = false;
        // The lease manager needs connections beside the listener and the batch.
        args.pg_pool_max_connections = 6;
    })
    .await
    .unwrap();

    // Server-key loading does not yield, so the stop would wait for it.
    // A short chain first puts the key in the worker's cache.
    stage_mul_chain(&listener_db, 2).await;
    wait_until_computed(&app).await.unwrap();

    let chain_id = stage_mul_chain(&listener_db, LONG_CHAIN).await;
    let started = Instant::now();
    while chain_state(&pool, chain_id.as_slice()).await.1.is_none() {
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "worker never took the chain"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    // Let the batch start executing, so the stop has to abandon it.
    tokio::time::sleep(Duration::from_millis(500)).await;

    app.stop_worker();
    let stopped = Instant::now();
    while chain_state(&pool, chain_id.as_slice()).await.1.is_some() {
        assert!(
            stopped.elapsed() < Duration::from_secs(5),
            "stopped worker kept its chain"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let (status, _, _) = chain_state(&pool, chain_id.as_slice()).await;
    assert_eq!(status, "updated", "released chain must be acquirable again");
}

/// One transaction, one chain: `length` sequential 64-bit multiplications,
/// only the last output allowed. Returns the chain id.
async fn stage_mul_chain(listener_db: &ListenerDatabase, length: usize) -> Handle {
    let tx_id = next_handle();
    let mut tx = listener_db
        .new_transaction()
        .await
        .unwrap()
        .expect("new_transaction() returns Some on a live stack");
    let mut counter = next_handle_with_type(FHE_UINT64);
    let factor = next_handle_with_type(FHE_UINT64);
    insert_trivial_encrypt(listener_db, &mut tx, tx_id, 3, FHE_UINT64, counter, false)
        .await
        .unwrap();
    insert_trivial_encrypt(listener_db, &mut tx, tx_id, 1, FHE_UINT64, factor, false)
        .await
        .unwrap();
    for i in 0..length {
        let product = next_handle_with_type(FHE_UINT64);
        insert_event(
            listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheMul(TfheContract::FheMul {
                caller: zero_address(),
                lhs: counter,
                rhs: factor,
                scalarByte: scalar_flag(false),
                result: product,
            }),
            i + 1 == length,
        )
        .await
        .unwrap();
        counter = product;
    }
    sqlx::query(
        "INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at) \
         VALUES ($1, 'updated', NOW())",
    )
    .bind(tx_id.to_vec())
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
    tx_id
}
