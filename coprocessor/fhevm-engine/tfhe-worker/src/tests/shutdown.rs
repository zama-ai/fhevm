use std::time::{Duration, Instant};

use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{Database as ListenerDatabase, Handle};

use crate::tests::event_helpers::{
    insert_event, insert_trivial_encrypt, next_handle, next_handle_with_type, scalar_flag,
    setup_event_harness_with, wait_until_computed, zero_address, EventHarness,
};

const FHE_UINT64: i32 = 5;
// Long enough that the batch is still running when the worker is stopped.
const LONG_CHAIN: usize = 1500;

/// A worker stopped mid-batch (SIGTERM, scale-in) rolls the batch back and
/// hands its chain back at once, instead of holding it until the lease expires.
#[tokio::test]
async fn test_stop_mid_batch_releases_chain_without_committing() {
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
    stage_mul_chain(&listener_db, &pool, 2).await;
    wait_until_computed(&app).await.unwrap();

    let (chain_id, output) = stage_mul_chain(&listener_db, &pool, LONG_CHAIN).await;
    let started = Instant::now();
    while lease_holder(&pool, &chain_id).await.is_none() {
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "worker never took the chain"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    tokio::time::sleep(Duration::from_millis(500)).await;

    app.stop_worker();
    let stopped = Instant::now();
    while lease_holder(&pool, &chain_id).await.is_some() {
        assert!(
            stopped.elapsed() < Duration::from_secs(5),
            "stopped worker kept its chain"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let status: String =
        sqlx::query_scalar("SELECT status FROM dependence_chain WHERE dependence_chain_id = $1")
            .bind(chain_id.to_vec())
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(status, "updated", "released chain must be acquirable again");

    // Nothing from the abandoned batch may land after the release either.
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert!(
        !is_completed(&pool, &chain_id, &output).await,
        "the abandoned batch must not commit"
    );
}

/// One transaction, one chain: `length` sequential 64-bit multiplications,
/// only the last output allowed. Returns the chain id and that output.
async fn stage_mul_chain(
    listener_db: &ListenerDatabase,
    pool: &sqlx::PgPool,
    length: usize,
) -> (Handle, Handle) {
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
    sqlx::query("SELECT pg_notify('work_available', '')")
        .execute(pool)
        .await
        .unwrap();
    (tx_id, counter)
}

async fn lease_holder(pool: &sqlx::PgPool, chain_id: &Handle) -> Option<sqlx::types::Uuid> {
    sqlx::query_scalar("SELECT worker_id FROM dependence_chain WHERE dependence_chain_id = $1")
        .bind(chain_id.to_vec())
        .fetch_optional(pool)
        .await
        .unwrap()
        .flatten()
}

async fn is_completed(pool: &sqlx::PgPool, tx_id: &Handle, output: &Handle) -> bool {
    sqlx::query_scalar(
        "SELECT is_completed FROM computations WHERE output_handle = $1 AND transaction_id = $2",
    )
    .bind(output.to_vec())
    .bind(tx_id.to_vec())
    .fetch_one(pool)
    .await
    .unwrap()
}
