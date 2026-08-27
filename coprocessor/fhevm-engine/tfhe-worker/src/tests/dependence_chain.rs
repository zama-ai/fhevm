use crate::dependence_chain::{LockMngr, LockingReason};
use crate::tests::utils::{setup_test_db_without_worker, TestInstance};
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use tokio::time::{sleep, Duration};
use tracing::info;
use uuid::Uuid;

const NUM_SAMPLE_CHAINS: usize = 10;

#[tokio::test]
#[serial(db)]
async fn test_acquire_next_lock() {
    let instance = setup().await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let dependence_chain_ids = insert_sample_dcids(&pool, "updated", NUM_SAMPLE_CHAINS)
        .await
        .expect("inserted chains");

    let mut workers = vec![];

    for dependence_chain_id in dependence_chain_ids.iter() {
        info!(target: "deps_chain", ?dependence_chain_id, "Testing acquire_next_lock");
        let mut mgr =
            LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);

        let (acquired, locking) = mgr.acquire_next_lock().await.unwrap();
        assert_eq!(acquired, Some(dependence_chain_id.clone()));
        assert_eq!(locking, LockingReason::UpdatedUnowned);

        let row = sqlx::query!(
            "SELECT status, worker_id FROM dependence_chain WHERE dependence_chain_id = $1",
            dependence_chain_id
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(row.status, "processing".to_string());
        assert_eq!(row.worker_id, Some(mgr.worker_id()));

        workers.push(mgr);
    }

    // Ensure no more locks available
    assert_locks_available(&pool, 0).await;

    for worker in workers.iter_mut() {
        assert_reacquire_lock(&pool, worker).await;
        assert!(worker.get_current_lock().is_none());
    }
}

#[tokio::test]
#[serial(db)]
async fn test_acquire_next_lock_prefers_fast_lane() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let fast_id = vec![1u8];
    let slow_id = vec![2u8];

    sqlx::query!(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, schedule_priority)
        VALUES ($1, 'updated', NOW() - INTERVAL '1 minute', NOW(), 1, 0)
        "#,
        fast_id,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query!(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, schedule_priority)
        VALUES ($1, 'updated', NOW() - INTERVAL '2 minute', NOW(), 2, 1)
        "#,
        slow_id,
    )
    .execute(&pool)
    .await
    .unwrap();

    let mut mgr_fast =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);
    let (acquired_fast, _) = mgr_fast.acquire_next_lock().await.unwrap();
    assert_eq!(acquired_fast, Some(fast_id.clone()));

    let mut mgr_slow =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);
    let (acquired_slow, _) = mgr_slow.acquire_next_lock().await.unwrap();
    assert_eq!(acquired_slow, Some(slow_id.clone()));
}

#[tokio::test]
#[serial(db)]
async fn test_parked_chain_yields_until_lock_expiry() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let stalled_id = vec![1u8];
    let ready_id = vec![2u8];

    sqlx::query(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height)
        VALUES
            ($1, 'updated', NOW() - INTERVAL '2 minutes', NOW(), 1),
            ($2, 'updated', NOW() - INTERVAL '1 minute', NOW(), 2)
        "#,
    )
    .bind(stalled_id.clone())
    .bind(ready_id.clone())
    .execute(&pool)
    .await
    .unwrap();

    let lock_ttl_sec = 5;
    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        lock_ttl_sec,
        false,
        None,
        None,
        None,
    );
    let (acquired, reason) = mgr.acquire_next_lock().await.unwrap();
    assert_eq!(acquired, Some(stalled_id.clone()));
    assert_eq!(reason, LockingReason::UpdatedUnowned);

    let parked_lock = mgr.get_current_lock().unwrap();
    mgr.park_current_lock();
    assert!(mgr.get_current_lock().is_none());

    let parked_row: (
        String,
        Option<Uuid>,
        Option<chrono::DateTime<chrono::Utc>>,
        chrono::DateTime<chrono::Utc>,
    ) = sqlx::query_as(
        "SELECT status, worker_id, lock_expires_at, last_updated_at
         FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&stalled_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(parked_row.0, "processing");
    assert_eq!(parked_row.1, Some(mgr.worker_id()));
    assert_eq!(parked_row.2, parked_lock.lock_expires_at);
    assert_eq!(parked_row.3, parked_lock.last_updated_at);

    let (acquired, reason) = mgr.acquire_next_lock().await.unwrap();
    assert_eq!(acquired, Some(ready_id));
    assert_eq!(reason, LockingReason::UpdatedUnowned);
    mgr.release_current_lock(true, None).await.unwrap();

    assert_eq!(
        mgr.acquire_next_lock().await.unwrap(),
        (None, LockingReason::Missing)
    );

    sleep(Duration::from_secs(lock_ttl_sec as u64 + 1)).await;
    let (acquired, reason) = mgr.acquire_next_lock().await.unwrap();
    assert_eq!(acquired, Some(stalled_id));
    assert_eq!(reason, LockingReason::ExpiredLock);
}

#[tokio::test]
#[serial(db)]
async fn test_acquire_early_lock_ignores_priority() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let fast_id = vec![3u8];
    let slow_id = vec![4u8];

    sqlx::query(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, dependency_count, schedule_priority)
        VALUES ($1, 'updated', NOW() - INTERVAL '1 minute', NOW(), 3, 5, 0)
        "#,
    )
    .bind(fast_id.clone())
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, dependency_count, schedule_priority)
        VALUES ($1, 'updated', NOW() - INTERVAL '2 minute', NOW(), 4, 0, 1)
        "#,
    )
    .bind(slow_id.clone())
    .execute(&pool)
    .await
    .unwrap();

    let mut mgr_slow =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);
    let (acquired_slow, _) = mgr_slow.acquire_early_lock().await.unwrap();
    assert_eq!(acquired_slow, Some(slow_id.clone()));

    let mut mgr_fast =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);
    let (acquired_fast, _) = mgr_fast.acquire_early_lock().await.unwrap();
    assert_eq!(acquired_fast, Some(fast_id.clone()));
}

#[tokio::test]
#[serial(db)]
async fn test_work_stealing() {
    let instance = setup().await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let dependence_chain_ids = insert_sample_dcids(&pool, "updated", NUM_SAMPLE_CHAINS)
        .await
        .expect("inserted chains");

    let mut workers = vec![];
    let lock_ttl_sec = 1;

    for dependence_chain_id in dependence_chain_ids.iter() {
        info!(?dependence_chain_id, "Testing acquire_next_lock");

        let worker = Uuid::new_v4();
        let mut mgr =
            LockMngr::new_with_conf(worker, pool.clone(), lock_ttl_sec, false, None, None, None);
        let acquired = mgr.acquire_next_lock().await.unwrap().0;
        assert_eq!(acquired, Some(dependence_chain_id.clone()));

        // Verify DB state
        let row = sqlx::query!(
            "SELECT status, worker_id FROM dependence_chain WHERE dependence_chain_id = $1",
            dependence_chain_id
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        workers.push(mgr);

        assert_eq!(row.status, "processing".to_string());
        assert_eq!(row.worker_id, Some(worker));
    }

    // Make sure the locks have expired
    tokio::time::sleep(std::time::Duration::from_secs(3 + lock_ttl_sec as u64)).await;

    // Assert that we can re-acquire all locks due to work-stealing
    for _ in 0..NUM_SAMPLE_CHAINS {
        let mut mgr = workers.pop().unwrap();
        let (acquired, locking_reason) = mgr.acquire_next_lock().await.unwrap();
        assert!(acquired.is_some());
        assert_eq!(locking_reason, LockingReason::ExpiredLock);
    }

    assert_locks_available(&pool, 0).await;
}

/// Asserts that after releasing a lock, it can be re-acquired by another worker
async fn assert_reacquire_lock(pool: &sqlx::PgPool, dependence_mgr: &mut LockMngr) {
    let lock = dependence_mgr.get_current_lock().unwrap();
    let dependence_chain_id = lock.dependence_chain_id;

    let row = sqlx::query!(
        "SELECT status, worker_id FROM dependence_chain WHERE dependence_chain_id = $1",
        dependence_chain_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    assert_eq!(row.status, "processing".to_string());

    // Update status for this dependence_chain_id
    // to simulate host-listener marking it as updated again
    sqlx::query!(
        "UPDATE dependence_chain
         SET status = 'updated', last_updated_at = NOW()
         WHERE dependence_chain_id = $1",
        dependence_chain_id
    )
    .execute(pool)
    .await
    .unwrap();

    // Assert that before releasing the lock, it cannot be re-acquired
    assert_eq!(
        LockMngr::new(Uuid::new_v4(), pool.clone())
            .acquire_next_lock()
            .await
            .unwrap()
            .0,
        None
    );
    dependence_mgr.release_all_owned_locks().await.unwrap();

    // Assert that after releasing or expiring, it can be re-acquired by another worker
    assert_eq!(
        LockMngr::new(Uuid::new_v4(), pool.clone())
            .acquire_next_lock()
            .await
            .unwrap()
            .0,
        Some(dependence_chain_id.clone())
    );
}

async fn assert_locks_available(pool: &sqlx::PgPool, expected_locks_count: usize) {
    // Check DB state
    let rows = sqlx::query!(
        "SELECT COUNT(*) as count FROM dependence_chain
                     WHERE (status = 'updated' AND worker_id IS NULL) OR (lock_expires_at < NOW())",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(rows.count, Some(expected_locks_count as i64));

    if expected_locks_count == 0 {
        // Check acquire_next_lock returns None
        let worker = Uuid::new_v4();
        let mut mgr = LockMngr::new(worker, pool.clone());
        let acquired = mgr.acquire_next_lock().await.unwrap().0;
        assert_eq!(acquired, None);
    }
}

async fn insert_sample_dcids(
    pool: &sqlx::PgPool,
    status: &str,
    num_chains: usize,
) -> sqlx::Result<Vec<Vec<u8>>> {
    let mut out = Vec::with_capacity(num_chains);

    for i in 0..num_chains {
        info!("Inserting dcid {}", i);
        let dcid = i.to_le_bytes().to_vec();
        sqlx::query!(
            r#"
            INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at, block_timestamp, block_height)
            VALUES ($1, $2, NOW() - INTERVAL '1 minute', NOW() - INTERVAL '5 minute', $3)
            "#,
            dcid,
            status,
            i as i64,
        )
        .execute(pool)
        .await?;

        out.push(dcid);
    }

    Ok(out)
}

#[tokio::test]
#[serial(db)]
async fn test_extend_or_release_lock() {
    let instance = setup().await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    // Insert a single dependence-chain row
    let dependence_chain_id = insert_sample_dcids(&pool, "updated", 1)
        .await
        .expect("inserted chains")
        .first()
        .cloned()
        .unwrap();

    let lock_timeslice_sec: u32 = 1;

    // Ensure the only available lock can be re-acquired after releasing
    // where mark_as_processed is false
    for _ in 0..10 {
        info!(?dependence_chain_id, "Testing extend_or_release_lock");
        let mut mgr = LockMngr::new_with_conf(
            Uuid::new_v4(),
            pool.clone(),
            2,
            false,
            Some(lock_timeslice_sec),
            None,
            None,
        );
        let acquired = mgr.acquire_next_lock().await.unwrap().0;

        assert_eq!(acquired, Some(dependence_chain_id.clone()));

        // Try to extend the lock after timeslice has been consumed
        // where enable_timeslice_check is TRUE
        sleep(Duration::from_secs(lock_timeslice_sec as u64 + 2)).await;
        let dcid = mgr.extend_or_release_current_lock(true).await.unwrap();

        assert!(dcid.is_none());
        assert!(mgr.get_current_lock().is_none());
    }

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        2,
        false,
        Some(lock_timeslice_sec),
        None,
        None,
    );
    let acquired = mgr.acquire_next_lock().await.unwrap().0;
    assert_eq!(acquired, Some(dependence_chain_id.clone()));

    // Try to extend the lock after timeslice has been consumed
    // where enable_timeslice_check is FALSE
    sleep(Duration::from_secs(2)).await;
    let dcid = mgr.extend_or_release_current_lock(false).await.unwrap();
    assert!(dcid.is_some());
    assert!(mgr.get_current_lock().is_some());
}

#[tokio::test]
#[serial(db)]
async fn test_extend_or_release_lock_2() {
    let instance = setup().await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    // Insert 2 dcids
    let ids = insert_sample_dcids(&pool, "updated", 2)
        .await
        .expect("inserted chains");

    let first_id: Vec<u8> = ids.first().cloned().unwrap();
    let second_id: Vec<u8> = ids.get(1).cloned().unwrap();

    let lock_timeslice_sec: u32 = 1;

    info!(?first_id, "Testing extend_or_release_lock");
    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        2,
        false,
        Some(lock_timeslice_sec),
        None,
        None,
    );
    let acquired = mgr.acquire_next_lock().await.unwrap().0;
    assert_eq!(acquired, Some(first_id.clone()));

    // Try to extend the lock after timeslice has been consumed
    // where enable_timeslice_check is TRUE
    sleep(Duration::from_secs(lock_timeslice_sec as u64 + 2)).await;
    let dcid = mgr.extend_or_release_current_lock(true).await.unwrap();

    assert!(dcid.is_none());
    assert!(mgr.get_current_lock().is_none());

    info!(?second_id, "Testing extend_or_release_lock");
    let acquired = mgr.acquire_next_lock().await.unwrap().0;
    assert_eq!(acquired, Some(first_id.clone()));
}

#[tokio::test]
#[serial(db)]
async fn test_cleanup() {
    let instance = setup().await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let inserted = insert_sample_dcids(&pool, "processed", NUM_SAMPLE_CHAINS)
        .await
        .expect("inserted chains")
        .len();
    let cleanup_age_threshold_sec = Some(30); // 30 seconds
    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        2,
        false,
        None,
        None,
        cleanup_age_threshold_sec,
    );

    let deleted = mgr.do_cleanup().await.expect("cleanup failed");
    assert_eq!(deleted, inserted as u64);
}

async fn setup() -> TestInstance {
    // No worker: these tests own the rows they seed. See setup_test_db_without_worker.
    let test_instance = setup_test_db_without_worker()
        .await
        .expect("valid db instance");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(test_instance.db_url())
        .await
        .unwrap();

    // Insert sample dependence-chain rows
    sqlx::query!("TRUNCATE TABLE dependence_chain")
        .execute(&pool)
        .await
        .unwrap();

    test_instance
}

/// A chain gated on several parents released in ONE batch must receive one
/// decrement per parent. The naive row-match decremented such a chain once
/// per batch, leaving dependency_count > 0 forever (braid-shaped joins
/// deadlocked when both parents completed together).
#[tokio::test]
#[serial(db)]
async fn test_batched_release_decrements_per_parent() {
    let instance = setup().await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let child = b"child-chain-0000".to_vec();
    sqlx::query!(
        r#"
        INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, dependency_count)
        VALUES ($1, 'updated', NOW(), NOW(), 3, 2)
        "#,
        child,
    )
    .execute(&pool)
    .await
    .unwrap();
    for (i, parent) in [b"parent-chain-aaa".to_vec(), b"parent-chain-bbb".to_vec()]
        .into_iter()
        .enumerate()
    {
        sqlx::query!(
            r#"
            INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, dependency_count, dependents)
            VALUES ($1, 'updated', NOW() - INTERVAL '1 minute', NOW(), $2, 0, $3)
            "#,
            parent,
            i as i64,
            &[child.clone()],
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    let mut mgr =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);
    let locks = mgr.acquire_next_locks(10).await.unwrap();
    let acquired: Vec<_> = locks.iter().filter_map(|(id, _)| id.clone()).collect();
    assert_eq!(acquired.len(), 2, "both parents acquired in one batch");
    mgr.release_current_lock(true, None).await.unwrap();

    let count: i32 = sqlx::query_scalar!(
        r#"SELECT dependency_count AS "count!" FROM dependence_chain WHERE dependence_chain_id = $1"#,
        child,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        count, 0,
        "one decrement per released parent, not one per batch"
    );
}

/// A completed DCID must free its batch slot without waiting for an unrelated
/// DCID that still has allowed work. The released parent also decrements its
/// child exactly once, and a second release must be a no-op.
#[tokio::test]
#[serial(db)]
async fn test_release_completed_lock_refills_batch_slot() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let completed = b"completed-chain-00".to_vec();
    let pending = b"pending-chain-000".to_vec();
    let refill = b"refill-chain-0000".to_vec();
    let child = b"dependent-chain-00".to_vec();

    sqlx::query(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height,
             dependency_count, dependents)
        VALUES
            ($1, 'updated', NOW() - INTERVAL '3 minutes', NOW(), 1, 0, $5),
            ($2, 'updated', NOW() - INTERVAL '2 minutes', NOW(), 2, 0, '{}'),
            ($3, 'updated', NOW() - INTERVAL '1 minute', NOW(), 3, 0, '{}'),
            ($4, 'updated', NOW(), NOW(), 4, 1, '{}')
        "#,
    )
    .bind(&completed)
    .bind(&pending)
    .bind(&refill)
    .bind(&child)
    .bind(vec![child.clone()])
    .execute(&pool)
    .await
    .unwrap();

    for (dcid, output_handle, is_completed) in [
        (&completed, b"completed-output".to_vec(), true),
        (&pending, b"pending-output00".to_vec(), false),
    ] {
        sqlx::query(
            r#"
            INSERT INTO computations
                (output_handle, dependencies, fhe_operation, is_scalar,
                 dependence_chain_id, transaction_id, is_allowed, is_completed,
                 is_error, host_chain_id, operand_boundary_mask)
            VALUES ($1, '{}', 0, false, $2, $3, true, $4, false, 1, $5)
            "#,
        )
        .bind(output_handle)
        .bind(dcid)
        .bind(format!("transaction-for-{}", hex::encode(dcid)).into_bytes())
        .bind(is_completed)
        .bind(vec![0_u8; 32])
        .execute(&pool)
        .await
        .unwrap();
    }

    let mut mgr =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);
    let acquired = mgr.acquire_next_locks(2).await.unwrap();
    assert_eq!(
        acquired
            .into_iter()
            .filter_map(|(id, _)| id)
            .collect::<Vec<_>>(),
        vec![completed.clone(), pending.clone()]
    );

    assert_eq!(mgr.release_completed_locks().await.unwrap(), 1);
    assert_eq!(mgr.get_current_lock_ids(), vec![pending.clone()]);
    assert_eq!(mgr.release_completed_locks().await.unwrap(), 0);

    let (completed_status, completed_owner): (String, Option<Uuid>) = sqlx::query_as(
        "SELECT status, worker_id FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&completed)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(completed_status, "processed");
    assert_eq!(completed_owner, None);

    let dependency_count: i32 = sqlx::query_scalar(
        "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&child)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(dependency_count, 0, "completed parent released its child");

    let acquired_refill = mgr.acquire_next_locks(1).await.unwrap();
    assert_eq!(
        acquired_refill.into_iter().find_map(|(id, _)| id),
        Some(refill),
        "a completed DCID frees its slot for the next ready chain"
    );
    assert_eq!(
        mgr.get_current_lock_ids(),
        vec![pending, b"refill-chain-0000".to_vec()]
    );
}

/// A listener refresh that adds work while a DCID is owned must prevent its
/// release until the new allowed computation reaches a terminal state.
#[tokio::test]
#[serial(db)]
async fn test_release_completed_lock_preserves_listener_refresh() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");
    let parent = b"refreshed-parent-00".to_vec();
    let child = b"refreshed-child-000".to_vec();

    sqlx::query(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height,
             dependency_count, dependents)
        VALUES
            ($1, 'updated', NOW(), NOW(), 1, 0, $3),
            ($2, 'updated', NOW(), NOW(), 2, 1, '{}')
        "#,
    )
    .bind(&parent)
    .bind(&child)
    .bind(vec![child.clone()])
    .execute(&pool)
    .await
    .unwrap();

    let mut mgr =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);
    assert_eq!(
        mgr.acquire_next_lock().await.unwrap().0,
        Some(parent.clone())
    );

    // This models listener-side ingestion racing with the worker after it has
    // selected its batch. The computation is still incomplete, so the parent
    // must remain owned and its dependent must stay gated.
    let output_handle = b"refreshed-output-0".to_vec();
    let transaction_id = b"refreshed-transaction".to_vec();
    sqlx::query(
        r#"
        INSERT INTO computations
            (output_handle, dependencies, fhe_operation, is_scalar,
             dependence_chain_id, transaction_id, is_allowed, is_completed,
             is_error, host_chain_id, operand_boundary_mask)
        VALUES ($1, '{}', 0, false, $2, $3, true, false, false, 1, $4)
        "#,
    )
    .bind(&output_handle)
    .bind(&parent)
    .bind(&transaction_id)
    .bind(vec![0_u8; 32])
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(mgr.release_completed_locks().await.unwrap(), 0);
    let (status, owner): (String, Option<Uuid>) = sqlx::query_as(
        "SELECT status, worker_id FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&parent)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(status, "processing");
    assert_eq!(owner, Some(mgr.worker_id()));

    let dependency_count: i32 = sqlx::query_scalar(
        "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&child)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        dependency_count, 1,
        "pending refresh must not unblock child"
    );

    sqlx::query(
        "UPDATE computations SET is_completed = true WHERE output_handle = $1 AND transaction_id = $2",
    )
    .bind(&output_handle)
    .bind(&transaction_id)
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(mgr.release_completed_locks().await.unwrap(), 1);
    let (status, owner): (String, Option<Uuid>) = sqlx::query_as(
        "SELECT status, worker_id FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&parent)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(status, "processed");
    assert_eq!(owner, None);

    let dependency_count: i32 = sqlx::query_scalar(
        "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&child)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(dependency_count, 0, "terminal refresh releases its child");
}

/// A renewal that loses one lease must keep the rest. The statement renews
/// every row this worker still owns, so forgetting the whole set would strand
/// the rows it just extended: nobody can acquire them, this worker included,
/// until the expiry the renewal itself pushed out.
#[tokio::test]
#[serial(db)]
async fn test_extend_keeps_the_locks_it_renewed() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let chains = insert_sample_dcids(&pool, "updated", 2)
        .await
        .expect("inserted chains");
    let (stolen, kept) = (chains[0].clone(), chains[1].clone());

    let mut mgr =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);
    let acquired = mgr.acquire_next_locks(2).await.unwrap();
    assert_eq!(
        acquired
            .into_iter()
            .filter_map(|(id, _)| id)
            .collect::<Vec<_>>(),
        vec![stolen.clone(), kept.clone()]
    );

    // Another worker takes one of the two leases.
    let thief = Uuid::new_v4();
    sqlx::query("UPDATE dependence_chain SET worker_id = $1 WHERE dependence_chain_id = $2")
        .bind(thief)
        .bind(&stolen)
        .execute(&pool)
        .await
        .unwrap();

    let extended = mgr.extend_or_release_current_lock(false).await.unwrap();
    assert!(
        extended.is_some(),
        "a renewal that still owns a lease reports one"
    );
    assert_eq!(
        mgr.get_current_lock_ids(),
        vec![kept.clone()],
        "the stolen lease is dropped and the renewed one retained"
    );

    // The retained lease is still this worker's in the database, so it keeps
    // being worked rather than waiting out a TTL nobody can shorten.
    let owner: Option<Uuid> =
        sqlx::query_scalar("SELECT worker_id FROM dependence_chain WHERE dependence_chain_id = $1")
            .bind(&kept)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(owner, Some(mgr.worker_id()));

    // Losing every lease still reports nothing held.
    sqlx::query("UPDATE dependence_chain SET worker_id = $1 WHERE dependence_chain_id = $2")
        .bind(thief)
        .bind(&kept)
        .execute(&pool)
        .await
        .unwrap();
    assert!(mgr
        .extend_or_release_current_lock(false)
        .await
        .unwrap()
        .is_none());
    assert!(mgr.get_current_lock_ids().is_empty());
}

/// A chain stranded by a corrupted dependency_count ('updated', unowned,
/// count > 0) matches neither normal acquisition predicate, and the
/// no-progress escalation is only reachable through some OTHER chain
/// stalling. The stale-gated repair acquisition must pick it up once it has
/// sat past the age gate — and never before.
#[tokio::test]
#[serial(db)]
async fn test_acquire_stale_gated_lock_recovers_stranded_chain() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let stranded_id = vec![9u8];
    sqlx::query!(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height,
             schedule_priority, dependency_count)
        VALUES ($1, 'updated', NOW() - INTERVAL '10 minutes', NOW(), 1, 0, 1)
        "#,
        stranded_id,
    )
    .execute(&pool)
    .await
    .unwrap();

    let mut mgr =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);

    // Invisible to normal acquisition (dependency_count > 0).
    let (acquired, _) = mgr.acquire_next_lock().await.unwrap();
    assert_eq!(acquired, None);

    // The age gate protects a chain that has not sat long enough.
    let (acquired, _) = mgr.acquire_stale_gated_lock(3600.0).await.unwrap();
    assert_eq!(acquired, None);

    // The probe itself is throttled per manager (at most twice per age
    // window), so an immediate retry on the SAME manager stays silent...
    let (acquired, _) = mgr.acquire_stale_gated_lock(60.0).await.unwrap();
    assert_eq!(acquired, None);

    // ...while another worker past the gate acquires it like any other
    // chain.
    let mut mgr =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);
    let (acquired, locking) = mgr.acquire_stale_gated_lock(60.0).await.unwrap();
    assert_eq!(acquired, Some(stranded_id.clone()));
    assert_eq!(locking, LockingReason::StaleGateRepair);

    let row = sqlx::query!(
        "SELECT status, worker_id, dependency_count FROM dependence_chain WHERE dependence_chain_id = $1",
        stranded_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.status, "processing".to_string());
    assert_eq!(row.worker_id, Some(mgr.worker_id()));
    // Repair resets the count ground truth established as stale, so the
    // chain re-enters normal scheduling (including expired-lock stealing —
    // a crash while holding this lock cannot re-strand it).
    assert_eq!(row.dependency_count, 0);
}

/// dependency_count > 0 is a LIVE same-block gate while a producer chain is
/// still pending: the repair path must not bypass it (e.g. during catchup,
/// where block-derived last_updated_at makes every chain look old).
#[tokio::test]
#[serial(db)]
async fn test_stale_gated_lock_respects_live_producers() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let gated_id = vec![7u8];
    let producer_id = vec![8u8];
    sqlx::query!(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height,
             schedule_priority, dependency_count, dependents)
        VALUES
            ($1, 'updated', NOW() - INTERVAL '10 minutes', NOW(), 1, 0, 1, '{}'),
            ($2, 'updated', NOW() - INTERVAL '10 minutes', NOW(), 1, 0, 0, ARRAY[$1::bytea])
        "#,
        gated_id,
        producer_id,
    )
    .execute(&pool)
    .await
    .unwrap();

    let mut mgr =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);

    // The producer is unprocessed, so the gate is legitimate: the repair
    // path must leave the gated chain alone however old it looks.
    let (acquired, _) = mgr.acquire_stale_gated_lock(60.0).await.unwrap();
    assert_eq!(acquired, None);
    // Fresh manager: sidestep the per-manager probe throttle.
    let mut mgr =
        LockMngr::new_with_conf(Uuid::new_v4(), pool.clone(), 3600, false, None, None, None);

    // Once the producer is processed (its release decremented nothing here,
    // simulating a lost decrement), the gate is provably stale and the
    // repair path recovers the chain.
    sqlx::query!(
        "UPDATE dependence_chain SET status = 'processed' WHERE dependence_chain_id = $1",
        producer_id,
    )
    .execute(&pool)
    .await
    .unwrap();
    let (acquired, _) = mgr.acquire_stale_gated_lock(60.0).await.unwrap();
    assert_eq!(acquired, Some(gated_id));
}
