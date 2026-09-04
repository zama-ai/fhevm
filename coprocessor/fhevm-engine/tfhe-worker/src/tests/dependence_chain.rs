use crate::dependence_chain::{
    delete_old_processed_dependence_chains, rearm_demoted_chains, rearm_demoted_chains_limited,
    LockMngr, LockingReason,
};
use crate::tests::utils::{setup_test_db_without_worker, TestInstance};
use fhevm_engine_common::types::SchedulePriority;
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
        let mut mgr = LockMngr::new_with_conf(
            Uuid::new_v4(),
            pool.clone(),
            3600,
            false,
            None,
            None,
            None,
            3,
        );

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

    let mut mgr_fast = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    let (acquired_fast, _) = mgr_fast.acquire_next_lock().await.unwrap();
    assert_eq!(acquired_fast, Some(fast_id.clone()));

    let mut mgr_slow = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
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
        3,
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

    let mut mgr_slow = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    let (acquired_slow, _) = mgr_slow.acquire_early_lock().await.unwrap();
    assert_eq!(acquired_slow, Some(slow_id.clone()));

    let mut mgr_fast = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
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
        let mut mgr = LockMngr::new_with_conf(
            worker,
            pool.clone(),
            lock_ttl_sec,
            false,
            None,
            None,
            None,
            3,
        );
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
            3,
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
        3,
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
        3,
    );
    let acquired = mgr.acquire_next_lock().await.unwrap().0;
    assert_eq!(acquired, Some(first_id.clone()));

    // Try to extend the lock after timeslice has been consumed
    // where enable_timeslice_check is TRUE
    sleep(Duration::from_secs(lock_timeslice_sec as u64 + 2)).await;
    let dcid = mgr.extend_or_release_current_lock(true).await.unwrap();

    assert!(dcid.is_none());
    assert!(mgr.get_current_lock().is_none());

    // A consumed timeslice ROTATES the chain: it is released with a fresh
    // last_updated_at, so oldest-first acquisition moves on to the next chain
    // instead of handing the same one straight back. Returning it to the FIFO
    // front would make the timeslice pure churn — the worker would re-acquire
    // it microseconds later and never yield to anything younger.
    info!(?second_id, "Testing extend_or_release_lock");
    let acquired = mgr.acquire_next_lock().await.unwrap().0;
    assert_eq!(
        acquired,
        Some(second_id.clone()),
        "the rotated chain yields its slot to the next chain in the FIFO"
    );
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
        3,
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

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
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

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
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

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
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

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
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

/// A lease that lapsed while its owner still holds it in memory must not be
/// stolen back by that same owner. The work-stealing branch matches on expiry
/// alone, so without an explicit exclusion the chain lands in the lock set a
/// second time — and the extend statement, which can only ever return one row
/// per chain, then reports a lock lost on every cycle for the rest of the
/// process's life.
#[tokio::test]
#[serial(db)]
async fn test_worker_does_not_steal_back_its_own_expired_lease() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let chains = insert_sample_dcids(&pool, "updated", 1)
        .await
        .expect("inserted chains");

    let lock_ttl_sec = 1;
    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        lock_ttl_sec,
        false,
        None,
        None,
        None,
        3,
    );
    let acquired = mgr.acquire_next_locks(4).await.unwrap();
    assert_eq!(
        acquired
            .into_iter()
            .filter_map(|(id, _)| id)
            .collect::<Vec<_>>(),
        chains
    );

    // Let the lease lapse without releasing it: the manager still believes it
    // holds the chain, and the row is now visible to work-stealing.
    tokio::time::sleep(std::time::Duration::from_secs(2 + lock_ttl_sec as u64)).await;

    let restolen = mgr.acquire_next_locks(4).await.unwrap();
    assert_eq!(
        restolen
            .into_iter()
            .filter_map(|(id, _)| id)
            .collect::<Vec<_>>(),
        Vec::<Vec<u8>>::new(),
        "a chain already held must not be acquired again by its own owner"
    );
    assert_eq!(
        mgr.get_current_lock_ids(),
        chains,
        "the lock set still holds the chain exactly once"
    );

    // The renewal therefore agrees with the lock set and reports no loss.
    assert!(mgr
        .extend_or_release_current_lock(false)
        .await
        .unwrap()
        .is_some());
    assert_eq!(mgr.get_current_lock_ids(), chains);

    // Another worker can still take it, which is what the expiry is for.
    let mut other = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    sqlx::query("UPDATE dependence_chain SET lock_expires_at = NOW() - INTERVAL '1 second'")
        .execute(&pool)
        .await
        .unwrap();
    let stolen = other.acquire_next_locks(4).await.unwrap();
    assert_eq!(
        stolen
            .into_iter()
            .filter_map(|(id, _)| id)
            .collect::<Vec<_>>(),
        chains,
        "expiry must still hand the chain to a DIFFERENT worker"
    );
}

/// `release_locks(mark_as_processed = true)` must discharge dependents on the
/// status flip that actually happened, not on the caller's intent. A chain the
/// listener refreshed to 'updated' while it was owned has NEW work, so the
/// CASE declines to mark it processed — and its dependents must stay gated.
#[tokio::test]
#[serial(db)]
async fn test_release_locks_discharges_on_the_flip_not_the_parameter() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let parent = vec![0xA1u8; 32];
    let child = vec![0xC1u8; 32];
    sqlx::query(
        "INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height,
             schedule_priority, dependency_count, dependents)
         VALUES ($1, 'updated', NOW(), NOW(), 1, 0, 0, ARRAY[$2::bytea]),
                ($2, 'updated', NOW(), NOW(), 1, 0, 1, ARRAY[]::bytea[])",
    )
    .bind(&parent)
    .bind(&child)
    .execute(&pool)
    .await
    .unwrap();

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    let acquired = mgr.acquire_next_locks(4).await.unwrap();
    assert_eq!(
        acquired
            .into_iter()
            .filter_map(|(id, _)| id)
            .collect::<Vec<_>>(),
        vec![parent.clone()],
        "only the ungated parent is acquirable"
    );

    // The listener refreshes the parent mid-flight: it has new work, so the
    // release below must NOT retire it.
    sqlx::query("UPDATE dependence_chain SET status = 'updated' WHERE dependence_chain_id = $1")
        .bind(&parent)
        .execute(&pool)
        .await
        .unwrap();

    mgr.release_locks(std::slice::from_ref(&parent), true, None)
        .await
        .unwrap();

    let child_gate: i32 = sqlx::query_scalar(
        "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&child)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        child_gate, 1,
        "a refreshed parent has not retired, so its dependent stays gated"
    );

    let parent_status: String =
        sqlx::query_scalar("SELECT status FROM dependence_chain WHERE dependence_chain_id = $1")
            .bind(&parent)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(parent_status, "updated");

    // The same call on a parent that really is 'processing' does discharge.
    let mut mgr2 = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    assert!(!mgr2.acquire_next_locks(4).await.unwrap().is_empty());
    mgr2.release_locks(std::slice::from_ref(&parent), true, None)
        .await
        .unwrap();

    let child_gate: i32 = sqlx::query_scalar(
        "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&child)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(child_gate, 0, "a genuine retirement discharges the gate");
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

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );

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
    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
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

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );

    // The producer is unprocessed, so the gate is legitimate: the repair
    // path must leave the gated chain alone however old it looks.
    let (acquired, _) = mgr.acquire_stale_gated_lock(60.0).await.unwrap();
    assert_eq!(acquired, None);
    // Fresh manager: sidestep the per-manager probe throttle.
    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );

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

/// Insert one chain row with the given gate and dependents.
async fn seed_chain(
    pool: &sqlx::PgPool,
    dcid: &[u8],
    dependency_count: i32,
    dependents: &[Vec<u8>],
) {
    sqlx::query(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height,
             dependency_count, dependents)
        VALUES ($1, 'updated', NOW(), NOW(), 1, $2, $3)
        "#,
    )
    .bind(dcid)
    .bind(dependency_count)
    .bind(dependents)
    .execute(pool)
    .await
    .unwrap();
}

/// Insert one computation row under `dcid`.
fn handle(seed: u8) -> Vec<u8> {
    vec![seed; 32]
}

async fn seed_computation_row(
    pool: &sqlx::PgPool,
    dcid: &[u8],
    output_handle: &[u8],
    is_completed: bool,
    is_error: bool,
    error_message: Option<&str>,
) {
    sqlx::query(
        r#"
        INSERT INTO computations
            (output_handle, dependencies, fhe_operation, is_scalar,
             dependence_chain_id, transaction_id, is_allowed, is_completed,
             is_error, error_message, host_chain_id, operand_boundary_mask)
        VALUES ($1, '{}', 0, false, $2, $3, true, $4, $5, $6, 1, $7)
        "#,
    )
    .bind(output_handle)
    .bind(dcid)
    .bind(format!("tx-for-{}", hex::encode(output_handle)).into_bytes())
    .bind(is_completed)
    .bind(is_error)
    .bind(error_message)
    .bind(vec![0_u8; 32])
    .execute(pool)
    .await
    .unwrap();
}

/// `seed_computation_row` with an explicit transaction, so two rows can share
/// one. The default helper derives the transaction from the output handle,
/// which puts every row in a transaction of its own.
#[allow(clippy::too_many_arguments)]
async fn seed_computation_row_in_transaction(
    pool: &sqlx::PgPool,
    dcid: &[u8],
    output_handle: &[u8],
    transaction_id: &[u8],
    is_completed: bool,
    is_error: bool,
    error_message: Option<&str>,
) {
    seed_computation_row_in_transaction_with_allowed(
        pool,
        dcid,
        output_handle,
        transaction_id,
        true,
        is_completed,
        is_error,
        error_message,
    )
    .await
}

/// As above, but `is_allowed` is a parameter. An internal producer carries
/// `is_allowed = FALSE`: it is never work in its own right, only executed on
/// behalf of an allowed consumer -- and it is stamped like any other row when
/// it fails, so it reaches the demote threshold like any other row too.
#[allow(clippy::too_many_arguments)]
async fn seed_computation_row_in_transaction_with_allowed(
    pool: &sqlx::PgPool,
    dcid: &[u8],
    output_handle: &[u8],
    transaction_id: &[u8],
    is_allowed: bool,
    is_completed: bool,
    is_error: bool,
    error_message: Option<&str>,
) {
    sqlx::query(
        r#"
        INSERT INTO computations
            (output_handle, dependencies, fhe_operation, is_scalar,
             dependence_chain_id, transaction_id, is_allowed, is_completed,
             is_error, error_message, host_chain_id, operand_boundary_mask)
        VALUES ($1, '{}', 0, false, $2, $3, $4, $5, $6, $7, 1, $8)
        "#,
    )
    .bind(output_handle)
    .bind(dcid)
    .bind(transaction_id)
    .bind(is_allowed)
    .bind(is_completed)
    .bind(is_error)
    .bind(error_message)
    .bind(vec![0_u8; 32])
    .execute(pool)
    .await
    .unwrap();
}

/// Demotion has to be TRANSACTION-scoped, because selection is.
///
/// Operation B consumes operation A in the same transaction. A exhausts its
/// attempts and is demoted; B is still pending and, because it defers on the
/// missing input rather than erroring, it is never stamped. Row-scoped
/// demotion made that combination unrecoverable: the window selects
/// transactions, so B kept the transaction selectable and dragged demoted A
/// back in to be retried; B's unstamped `is_error = FALSE` kept the chain out
/// of `processed`; and the slow sweep only looks at processed chains, so the
/// re-arm that was supposed to bound the whole thing never ran. Demotion, the
/// retirement it enables, and the sweep were all inert for this shape.
///
/// The fix is that a transaction holding a demoted row is not selected and
/// does not hold its chain open. This test walks the loop it unblocks.
#[tokio::test]
#[serial(db)]
async fn demotion_is_transaction_scoped_so_a_blocked_sibling_cannot_stall_the_chain() {
    let instance = setup().await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");
    let threshold: i16 = 3;

    let stalled_chain = vec![0xB0u8; 32];
    let control_chain = vec![0xB1u8; 32];
    // A transaction whose rows span two chains, as a fork-exposed database
    // can retain them: the demoted row sits in one chain, the pending row of
    // the same transaction in the other.
    let foreign_demoted_chain = vec![0xB4u8; 32];
    let foreign_pending_chain = vec![0xB5u8; 32];
    // An internal (is_allowed = FALSE) producer and its allowed consumer, in
    // one transaction and one chain.
    let internal_producer_chain = vec![0xB8u8; 32];
    let shared_tx = b"tx-shared-a-and-b".to_vec();
    let control_tx = b"tx-control".to_vec();
    let split_tx = b"tx-split-across-chains".to_vec();
    let internal_tx = b"tx-internal-producer".to_vec();
    for dcid in [
        &stalled_chain,
        &control_chain,
        &foreign_demoted_chain,
        &foreign_pending_chain,
        &internal_producer_chain,
    ] {
        sqlx::query(
            "INSERT INTO dependence_chain
                (dependence_chain_id, status, last_updated_at, block_timestamp, block_height,
                 schedule_priority, dependency_count, dependents)
             VALUES ($1, 'updated', NOW(), NOW(), 1, 0, 0, ARRAY[]::bytea[])",
        )
        .bind(dcid)
        .execute(&pool)
        .await
        .unwrap();
    }

    // A: retryable stamp, attempts spent -> demoted.
    seed_computation_row_in_transaction(
        &pool,
        &stalled_chain,
        &handle(0xA0),
        &shared_tx,
        false,
        true,
        Some("RETRYABLE SchedulerError::ExecutionPanic(sigsegv)"),
    )
    .await;
    sqlx::query("UPDATE computations SET error_retry_count = $1 WHERE output_handle = $2")
        .bind(threshold)
        .bind(handle(0xA0))
        .execute(&pool)
        .await
        .unwrap();
    // B: consumes A, defers on the missing input, never stamped.
    seed_computation_row_in_transaction(
        &pool,
        &stalled_chain,
        &handle(0xB2),
        &shared_tx,
        false,
        false,
        None,
    )
    .await;
    // Control: a pending row in a transaction with NO demoted sibling. It must
    // still hold its chain open, or the scoping is too broad and healthy work
    // would be retired out from under itself.
    seed_computation_row_in_transaction(
        &pool,
        &control_chain,
        &handle(0xB3),
        &control_tx,
        false,
        false,
        None,
    )
    .await;
    // Cross-chain: one chain owns the demoted row, another owns a pending row
    // of the same transaction.
    seed_computation_row_in_transaction(
        &pool,
        &foreign_demoted_chain,
        &handle(0xB6),
        &split_tx,
        false,
        true,
        Some("RETRYABLE SchedulerError::ExecutionPanic(sigsegv)"),
    )
    .await;
    sqlx::query("UPDATE computations SET error_retry_count = $1 WHERE output_handle = $2")
        .bind(threshold)
        .bind(handle(0xB6))
        .execute(&pool)
        .await
        .unwrap();
    seed_computation_row_in_transaction(
        &pool,
        &foreign_pending_chain,
        &handle(0xB7),
        &split_tx,
        false,
        false,
        None,
    )
    .await;
    // An internal producer is executed for its allowed consumer and stamped
    // like any other row when it fails, so it reaches the demote threshold --
    // and it must be discoverable by the sweep, or the chain it retires can
    // never come back.
    seed_computation_row_in_transaction_with_allowed(
        &pool,
        &internal_producer_chain,
        &handle(0xB9),
        &internal_tx,
        false, // is_allowed
        false,
        true,
        Some("RETRYABLE SchedulerError::ExecutionPanic(sigsegv)"),
    )
    .await;
    sqlx::query("UPDATE computations SET error_retry_count = $1 WHERE output_handle = $2")
        .bind(threshold)
        .bind(handle(0xB9))
        .execute(&pool)
        .await
        .unwrap();
    seed_computation_row_in_transaction(
        &pool,
        &internal_producer_chain,
        &handle(0xBA),
        &internal_tx,
        false,
        false,
        None,
    )
    .await;

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        threshold,
    );
    assert!(!mgr.acquire_next_locks(8).await.unwrap().is_empty());
    mgr.release_completed_locks().await.unwrap();

    // 1. The stalled chain retires. B is pending and unstamped, but its
    //    transaction holds a demoted row, so the window will not select it and
    //    the completion test must not count it either.
    let (status, worker, _) = chain_state(&pool, &stalled_chain).await;
    assert_eq!(
        status, "processed",
        "a transaction holding a demoted row must not keep its chain open"
    );
    assert!(worker.is_none(), "the retired chain is released");

    // 2. The control chain does NOT retire: nothing about it is demoted.
    let (status, _, _) = chain_state(&pool, &control_chain).await;
    assert_ne!(
        status, "processed",
        "a pending row with no demoted sibling still holds its chain open"
    );

    // 3. Cross-chain: the completion test is CHAIN-LOCAL, so a chain does not
    //    retire on a demoted row another chain owns. The sweep re-arms chains
    //    that OWN the demoted row, and a 'processed' chain is not acquirable,
    //    so retiring here would strand the pending row until the listener
    //    happened to refresh that chain with new work.
    let (status, _, _) = chain_state(&pool, &foreign_pending_chain).await;
    assert_ne!(
        status, "processed",
        "a chain must not retire on a demoted row it does not own: the sweep \
         would re-arm the other chain and leave this one stranded"
    );
    let (status, _, _) = chain_state(&pool, &foreign_demoted_chain).await;
    assert_eq!(
        status, "processed",
        "the chain that OWNS the demoted row still retires, and the sweep \
         will re-arm it"
    );

    // 4. An internal producer blocks exactly like an allowed one: the chain
    //    retires rather than being held open by its allowed consumer.
    let (status, _, _) = chain_state(&pool, &internal_producer_chain).await;
    assert_eq!(
        status, "processed",
        "a demoted internal producer must block its transaction too"
    );

    // 5. Retiring is what lets the sweep see it -- it only looks at processed
    //    chains -- so the loop closes: re-armed into the slow lane, A's count
    //    reset, and the transaction selectable again on the next pass.
    let rearmed = rearm_demoted_chains(&pool, threshold).await.unwrap();
    assert_eq!(
        rearmed, 3,
        "every chain owning a demoted row is re-armed: the stalled one, the \
         cross-chain owner, and the one whose demoted row is an INTERNAL \
         producer -- `is_allowed` decides what counts as work, not what \
         counts as a blocker"
    );
    let (status, _, _) = chain_state(&pool, &internal_producer_chain).await;
    assert_eq!(
        status, "updated",
        "the internal producer's chain comes back, or its allowed consumer \
         and everything downstream are stranded behind a retired chain"
    );
    let internal_retry_count: i16 =
        sqlx::query_scalar("SELECT error_retry_count FROM computations WHERE output_handle = $1")
            .bind(handle(0xB9))
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        internal_retry_count, 0,
        "and its attempts are reset, so the transaction is selectable again"
    );

    let (status, _, dependency_count) = chain_state(&pool, &stalled_chain).await;
    assert_eq!(status, "updated", "the re-armed chain is acquirable again");
    assert_eq!(dependency_count, 0);
    let (retry_count, priority): (i16, i16) = sqlx::query_as(
        "SELECT c.error_retry_count, dc.schedule_priority
         FROM computations c
         JOIN dependence_chain dc ON dc.dependence_chain_id = c.dependence_chain_id
         WHERE c.output_handle = $1",
    )
    .bind(handle(0xA0))
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        retry_count, 0,
        "the demoted row gets a fresh set of attempts"
    );
    assert_eq!(
        priority,
        i16::from(SchedulePriority::Slow),
        "and comes back in the slow lane"
    );
}

/// The whole demotion loop, end to end: a row that exhausts its attempts stops
/// holding its chain open, the chain retires, the sweep re-arms it into the
/// slow lane with the count reset, and retention does not delete it while the
/// work is still pending — but DOES delete a chain whose only incomplete rows
/// are terminal verdicts.
///
/// Written because every step of this loop is a SQL predicate that has to agree
/// with three others, and the reviewable unit is the behaviour rather than any
/// one clause. It is also the shape that would have caught the retention guard
/// treating a terminal verdict as work still to do: such a row never heals and
/// the sweep never touches it, so its chain would have blocked the TTL delete
/// on every pass, forever.
#[tokio::test]
#[serial(db)]
async fn test_demote_retire_sweep_and_retention_loop() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let threshold: i16 = 3;
    let demoted_chain = vec![0xD0u8; 32];
    let terminal_chain = vec![0x7Eu8; 32];
    for dcid in [&demoted_chain, &terminal_chain] {
        sqlx::query(
            "INSERT INTO dependence_chain
                (dependence_chain_id, status, last_updated_at, block_timestamp, block_height,
                 schedule_priority, dependency_count, dependents)
             VALUES ($1, 'updated', NOW(), NOW(), 1, 0, 0, ARRAY[]::bytea[])",
        )
        .bind(dcid)
        .execute(&pool)
        .await
        .unwrap();
    }

    // One chain holds a demoted row (retryable, attempts spent); the other
    // holds a terminal verdict. Both are incomplete and allowed.
    seed_computation_row(
        &pool,
        &demoted_chain,
        &handle(0xD1),
        false,
        true,
        Some("RETRYABLE SchedulerError::ExecutionPanic(sigsegv)"),
    )
    .await;
    seed_computation_row(
        &pool,
        &terminal_chain,
        &handle(0x7F),
        false,
        true,
        Some("invalid FHE operation: unknown opcode"),
    )
    .await;
    sqlx::query("UPDATE computations SET error_retry_count = $1 WHERE output_handle = $2")
        .bind(threshold)
        .bind(handle(0xD1))
        .execute(&pool)
        .await
        .unwrap();

    // 1. A demoted row must NOT hold its chain open: the work window has
    //    stopped selecting it, so the completion test must stop counting it or
    //    the chain never retires and its dependents stay gated forever.
    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        threshold,
    );
    assert!(!mgr.acquire_next_locks(4).await.unwrap().is_empty());
    mgr.release_completed_locks().await.unwrap();

    for (dcid, label) in [(&demoted_chain, "demoted"), (&terminal_chain, "terminal")] {
        let (status, worker, _) = chain_state(&pool, dcid).await;
        assert_eq!(status, "processed", "{label} chain must retire");
        assert!(worker.is_none(), "{label} chain must be released");
    }

    // 2. The sweep re-arms the demoted chain into the slow lane and resets the
    //    count, so the work window selects the row again. The terminal chain is
    //    left alone — there is nothing to retry.
    let rearmed = rearm_demoted_chains(&pool, threshold).await.unwrap();
    assert_eq!(rearmed, 1, "only the demoted chain is re-armed");

    let (status, _, dependency_count) = chain_state(&pool, &demoted_chain).await;
    assert_eq!(status, "updated", "a re-armed chain is acquirable again");
    assert_eq!(dependency_count, 0);
    let priority: i16 = sqlx::query_scalar(
        "SELECT schedule_priority FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&demoted_chain)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        priority,
        i16::from(SchedulePriority::Slow),
        "re-armed work yields to fresh work"
    );
    let count: i16 =
        sqlx::query_scalar("SELECT error_retry_count FROM computations WHERE output_handle = $1")
            .bind(handle(0xD1))
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 0, "the count means attempts in THIS lane pass");

    assert_eq!(
        chain_state(&pool, &terminal_chain).await.0,
        "processed",
        "a terminal verdict is not re-armed"
    );

    // 3. Retention. Age both chains past the TTL. The demoted chain still owns
    //    pending work and must survive; the terminal one owns only verdicts and
    //    must age out, or it accumulates at the head of the scan forever.
    sqlx::query("UPDATE dependence_chain SET last_updated_at = NOW() - INTERVAL '72 hours'")
        .execute(&pool)
        .await
        .unwrap();
    // Only 'processed' rows are deletable, so put the re-armed chain back —
    // this is the state a sweep leaves behind once its retry also fails.
    sqlx::query("UPDATE dependence_chain SET status = 'processed' WHERE dependence_chain_id = $1")
        .bind(&demoted_chain)
        .execute(&pool)
        .await
        .unwrap();

    delete_old_processed_dependence_chains(&pool, 100, 3600)
        .await
        .unwrap();

    let survives: i64 =
        sqlx::query_scalar("SELECT count(*) FROM dependence_chain WHERE dependence_chain_id = $1")
            .bind(&demoted_chain)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        survives, 1,
        "a chain still owning retryable work must not be deleted — the chain row \
         is the only handle on it, and nothing TTL-deletes computations"
    );

    let terminal_gone: i64 =
        sqlx::query_scalar("SELECT count(*) FROM dependence_chain WHERE dependence_chain_id = $1")
            .bind(&terminal_chain)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        terminal_gone, 0,
        "a chain whose incomplete rows are all verdicts must still age out"
    );
}

async fn chain_state(pool: &sqlx::PgPool, dcid: &[u8]) -> (String, Option<Uuid>, i32) {
    sqlx::query_as(
        "SELECT status, worker_id, dependency_count FROM dependence_chain \
         WHERE dependence_chain_id = $1",
    )
    .bind(dcid)
    .fetch_one(pool)
    .await
    .unwrap()
}

/// A RETRYABLE panic stamp is not terminal: the work window re-selects it and
/// a later success heals it. Releasing the chain as 'processed' would clear
/// `lock_expires_at`, and no acquisition predicate matches
/// (status='processed', lock_expires_at IS NULL) — the computation would be
/// lost outright. A deterministic stamp, by contrast, IS terminal and must
/// let the chain retire.
#[tokio::test]
#[serial(db)]
async fn test_release_completed_lock_keeps_chain_with_retryable_stamp() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let retryable = b"retryable-chain-00".to_vec();
    let terminal = b"terminal-chain-000".to_vec();
    seed_chain(&pool, &retryable, 0, &[]).await;
    seed_chain(&pool, &terminal, 0, &[]).await;
    seed_computation_row(
        &pool,
        &retryable,
        b"retryable-output0",
        false,
        true,
        // The marker, not the error name, is what makes a stamp retryable.
        Some("RETRYABLE SchedulerError::ExecutionPanic(sigsegv in fhe op)"),
    )
    .await;
    seed_computation_row(
        &pool,
        &terminal,
        b"terminal-output00",
        false,
        true,
        Some("operand type mismatch"),
    )
    .await;

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    let acquired: Vec<_> = mgr
        .acquire_next_locks(2)
        .await
        .unwrap()
        .into_iter()
        .filter_map(|(id, _)| id)
        .collect();
    assert_eq!(acquired.len(), 2);

    assert_eq!(
        mgr.release_completed_locks().await.unwrap(),
        1,
        "only the terminally-stamped chain retires"
    );
    assert_eq!(
        mgr.get_current_lock_ids(),
        vec![retryable.clone()],
        "the retryable chain stays owned so its stamp can be retried"
    );

    let (status, owner, _) = chain_state(&pool, &retryable).await;
    assert_eq!(status, "processing");
    assert_eq!(owner, Some(mgr.worker_id()));

    let (status, owner, _) = chain_state(&pool, &terminal).await;
    assert_eq!(status, "processed");
    assert_eq!(owner, None);

    // The heal path: a successful re-execution completes the row, and only
    // then does the chain retire.
    sqlx::query(
        "UPDATE computations SET is_completed = true, is_error = false WHERE output_handle = $1",
    )
    .bind(b"retryable-output0".to_vec())
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(mgr.release_completed_locks().await.unwrap(), 1);
    let (status, owner, _) = chain_state(&pool, &retryable).await;
    assert_eq!(status, "processed");
    assert_eq!(owner, None);
}

/// A listener refresh flips an owned chain's status to 'updated' while the
/// worker holds it. The refresh must be preserved — the chain has NEW work and
/// has to be re-acquired rather than hidden as processed — and it must NOT
/// discharge the dependents' gate: the chain is released as 'updated', is
/// immediately re-acquirable, and retires on a later cycle. Discharging on
/// both would decrement the child TWICE for one arming and open its gate with
/// a sibling producer still unrun.
#[tokio::test]
#[serial(db)]
async fn test_listener_refresh_discharges_a_child_exactly_once() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let parent = b"refresh-once-paren".to_vec();
    let sibling = b"refresh-once-sibli".to_vec();
    let child = b"refresh-once-child".to_vec();
    // The child is gated on TWO producers; only `parent` is exercised here, so
    // its gate must never reach 0.
    seed_chain(&pool, &parent, 0, std::slice::from_ref(&child)).await;
    seed_chain(&pool, &sibling, 0, std::slice::from_ref(&child)).await;
    seed_chain(&pool, &child, 2, &[]).await;
    seed_computation_row(&pool, &parent, b"refresh-once-out0", true, false, None).await;

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    assert_eq!(
        mgr.acquire_next_lock().await.unwrap().0,
        Some(parent.clone())
    );

    // Listener re-arms the owned chain (status only; its ON CONFLICT branch
    // never touches dependency_count) without adding incomplete allowed work.
    sqlx::query("UPDATE dependence_chain SET status = 'updated' WHERE dependence_chain_id = $1")
        .bind(&parent)
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(mgr.release_completed_locks().await.unwrap(), 1);
    let (status, owner, _) = chain_state(&pool, &parent).await;
    assert_eq!(status, "updated", "the listener's re-arm is preserved");
    assert_eq!(owner, None, "and the lease is released");
    let (_, _, child_gate) = chain_state(&pool, &child).await;
    assert_eq!(
        child_gate, 2,
        "a refreshed release is not a retirement and must not discharge"
    );

    // The chain is unowned + 'updated' + ungated, so it is re-acquired and
    // retires on this pass. That is the one discharge.
    assert_eq!(
        mgr.acquire_next_lock().await.unwrap().0,
        Some(parent.clone())
    );
    assert_eq!(mgr.release_completed_locks().await.unwrap(), 1);
    let (status, _, _) = chain_state(&pool, &parent).await;
    assert_eq!(status, "processed");

    let (_, _, child_gate) = chain_state(&pool, &child).await;
    assert_eq!(
        child_gate, 1,
        "exactly one discharge from this parent; the sibling producer still gates the child"
    );
}

/// Releasing a parent and one of its own dependents in the same batch. Folded
/// into one statement, Postgres silently drops the second UPDATE of the child's
/// row and its decrement is lost; the release and the decrement are therefore
/// two statements in one transaction. The child's OTHER producers must still
/// gate it — a release discharges what the parent owed, it does not clear what
/// the child is owed.
#[tokio::test]
#[serial(db)]
async fn test_release_completed_lock_handles_parent_and_child_in_one_batch() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let parent = b"same-batch-parent0".to_vec();
    let child = b"same-batch-child00".to_vec();
    seed_chain(&pool, &parent, 0, std::slice::from_ref(&child)).await;
    // The child is gated but reachable through the repair/early paths, which
    // ignore the gate — so it can legitimately share a batch with its parent.
    seed_chain(&pool, &child, 3, &[]).await;
    seed_computation_row(&pool, &parent, b"same-batch-out-p0", true, false, None).await;
    seed_computation_row(&pool, &child, b"same-batch-out-c0", true, false, None).await;

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    assert_eq!(
        mgr.acquire_next_lock().await.unwrap().0,
        Some(parent.clone())
    );
    assert_eq!(
        mgr.acquire_early_lock().await.unwrap().0,
        Some(child.clone()),
        "the early path ignores the dependency gate"
    );

    assert_eq!(mgr.release_completed_locks().await.unwrap(), 2);

    let (_, _, child_gate) = chain_state(&pool, &child).await;
    assert_eq!(
        child_gate, 2,
        "exactly one of the child's three producers was released, so its \
         decrement must land once and the other two must keep gating it"
    );
}

/// The decrement must be driven by the rows the release actually flipped, not
/// by the ids this worker believes it holds. After a lease is stolen and
/// completed elsewhere, a stale release must decrement nothing — otherwise it
/// would unblock a child whose sibling producer is still outstanding.
#[tokio::test]
#[serial(db)]
async fn test_release_current_lock_does_not_decrement_a_stolen_lease() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let parent = b"stolen-lease-paren".to_vec();
    let child = b"stolen-lease-child".to_vec();
    seed_chain(&pool, &parent, 0, std::slice::from_ref(&child)).await;
    seed_chain(&pool, &child, 2, &[]).await;

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    assert_eq!(
        mgr.acquire_next_lock().await.unwrap().0,
        Some(parent.clone())
    );

    // Another worker steals the lease.
    sqlx::query("UPDATE dependence_chain SET worker_id = $2 WHERE dependence_chain_id = $1")
        .bind(&parent)
        .bind(Uuid::new_v4())
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(
        mgr.release_current_lock(true, None).await.unwrap(),
        0,
        "the stale release matches no rows"
    );

    let (_, _, child_gate) = chain_state(&pool, &child).await;
    assert_eq!(
        child_gate, 2,
        "a release that flipped nothing must decrement nothing"
    );
}

/// The listener arms a cross-block gate by inserting a brand-new child row
/// while the worker's release waits on the parent's row lock. A row created by
/// a transaction that commits after a statement started is invisible to that
/// statement — EvalPlanQual only re-reads rows the snapshot already saw — so
/// folding the decrement into the release's CTE reads the parent's updated
/// `dependents` array (the child IS listed) and then matches no row, silently
/// stranding the child. The release and the decrement are therefore two
/// statements in one transaction, and the second takes a fresh snapshot.
///
/// Timing-tolerant: if the listener happens to commit before the release even
/// starts, the decrement lands through the ordinary path and the assertion
/// still holds — the test simply does not exercise the race that round.
#[tokio::test]
#[serial(db)]
async fn test_release_completed_lock_sees_a_child_armed_during_the_release() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let parent = b"late-armed-parent0".to_vec();
    let child = b"late-armed-child00".to_vec();
    seed_chain(&pool, &parent, 0, &[]).await;

    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        3,
    );
    assert_eq!(
        mgr.acquire_next_lock().await.unwrap().0,
        Some(parent.clone())
    );

    // Listener: lock the parent, arm the gate, hold the lock, then commit.
    let listener = {
        let pool = pool.clone();
        let parent = parent.clone();
        let child = child.clone();
        tokio::spawn(async move {
            let mut tx = pool.begin().await.unwrap();
            sqlx::query(
                "UPDATE dependence_chain SET dependents = ARRAY[$2::bytea] \
                 WHERE dependence_chain_id = $1",
            )
            .bind(&parent)
            .bind(&child)
            .execute(tx.as_mut())
            .await
            .unwrap();
            sqlx::query(
                "INSERT INTO dependence_chain \
                 (dependence_chain_id, status, last_updated_at, block_timestamp, \
                  block_height, dependency_count, dependents) \
                 VALUES ($1, 'updated', NOW(), NOW(), 2, 1, '{}')",
            )
            .bind(&child)
            .execute(tx.as_mut())
            .await
            .unwrap();
            sleep(Duration::from_millis(1500)).await;
            tx.commit().await.unwrap();
        })
    };

    // Start the release while the listener still holds the parent's row lock.
    sleep(Duration::from_millis(300)).await;
    assert_eq!(mgr.release_completed_locks().await.unwrap(), 1);
    listener.await.unwrap();

    let (_, _, child_gate) = chain_state(&pool, &child).await;
    assert_eq!(
        child_gate, 0,
        "a child armed while the release was in flight must still be decremented"
    );
}

/// The timeslice must be measured per lock and must ROTATE what it evicts.
///
/// Measured across the batch, one chain that never finishes drags every
/// healthy sibling out with it each slice. Released with `update_at = None`,
/// the evicted chains keep their block-derived `last_updated_at` and go back
/// to the FIFO front, so the same worker re-acquires them immediately — churn
/// rather than escape. Both together are why a permanently-deferring chain in
/// a productive batch was never evicted.
#[tokio::test]
#[serial(db)]
async fn test_timeslice_rotates_only_the_locks_that_consumed_it() {
    let instance = setup().await;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let stuck = b"timeslice-stuck-00".to_vec();
    let healthy = b"timeslice-healthy0".to_vec();
    seed_chain(&pool, &stuck, 0, &[]).await;
    seed_chain(&pool, &healthy, 0, &[]).await;

    // One-second timeslice so the first lock can outlive it while the second
    // is acquired fresh.
    let mut mgr = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        Some(1),
        None,
        None,
        3,
    );

    assert_eq!(
        mgr.acquire_next_lock().await.unwrap().0,
        Some(stuck.clone())
    );
    let stuck_updated_before: chrono::DateTime<chrono::Utc> = sqlx::query_scalar(
        "SELECT last_updated_at FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&stuck)
    .fetch_one(&pool)
    .await
    .unwrap();

    // Let only the first lock consume its slice, then join a fresh one.
    sleep(Duration::from_millis(1500)).await;
    assert_eq!(
        mgr.acquire_next_lock().await.unwrap().0,
        Some(healthy.clone())
    );

    mgr.extend_or_release_current_lock(true).await.unwrap();

    assert_eq!(
        mgr.get_current_lock_ids(),
        vec![healthy.clone()],
        "only the lock that consumed its slice is evicted"
    );

    let (status, owner, _) = chain_state(&pool, &stuck).await;
    assert_eq!(status, "updated", "evicted chain stays pending");
    assert_eq!(owner, None);

    let stuck_updated_after: chrono::DateTime<chrono::Utc> = sqlx::query_scalar(
        "SELECT last_updated_at FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&stuck)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(
        stuck_updated_after > stuck_updated_before,
        "the evicted chain must rotate to the FIFO back, not return to the front"
    );

    let (status, owner, _) = chain_state(&pool, &healthy).await;
    assert_eq!(status, "processing", "the fresh lock keeps running");
    assert_eq!(owner, Some(mgr.worker_id()));
}

/// The sweep must serve demoted chains OLDEST FIRST.
///
/// It re-arms at most `SLOW_LANE_REARM_BATCH` chains per pass. With an
/// unordered `LIMIT` the plan is free to return the same subset every time,
/// so chains behind it would never get a pass at all -- the opposite of the
/// bounded trickle demotion promises. Ordering by age makes the sweep a
/// queue: a chain re-armed here is stamped `last_updated_at = NOW()`, which
/// sends it to the back.
#[tokio::test]
#[serial(db)]
async fn rearm_demoted_chains_serves_oldest_first() {
    let instance = setup().await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");
    let threshold: i16 = 3;

    // Three demoted chains, distinctly aged. Insert newest first so physical
    // order disagrees with age order -- an unordered LIMIT would follow the
    // former.
    let chains: Vec<(Vec<u8>, i64)> = vec![
        (vec![0xC1u8; 32], 10),
        (vec![0xC2u8; 32], 100),
        (vec![0xC3u8; 32], 1000),
    ];
    for (dcid, age_secs) in &chains {
        sqlx::query(
            "INSERT INTO dependence_chain
                (dependence_chain_id, status, last_updated_at, block_timestamp, block_height,
                 schedule_priority, dependency_count, dependents)
             VALUES ($1, 'processed', NOW() - make_interval(secs => $2), NOW(), 1, 0, 0,
                     ARRAY[]::bytea[])",
        )
        .bind(dcid)
        .bind(*age_secs as f64)
        .execute(&pool)
        .await
        .unwrap();
        seed_computation_row(
            &pool,
            dcid,
            &handle(dcid[0]),
            false,
            true,
            Some("RETRYABLE SchedulerError::ExecutionPanic(sigsegv)"),
        )
        .await;
        sqlx::query("UPDATE computations SET error_retry_count = $1 WHERE output_handle = $2")
            .bind(threshold)
            .bind(handle(dcid[0]))
            .execute(&pool)
            .await
            .unwrap();
    }

    // The oldest chain is 0xC3; it must be the one served.
    let rearmed = rearm_demoted_chains_limited(&pool, threshold, 1)
        .await
        .unwrap();
    assert_eq!(rearmed, 1);
    let (status, _, _) = chain_state(&pool, &[0xC3u8; 32]).await;
    assert_eq!(
        status, "updated",
        "the OLDEST demoted chain is re-armed first"
    );
    for dcid in [vec![0xC1u8; 32], vec![0xC2u8; 32]] {
        let (status, _, _) = chain_state(&pool, &dcid).await;
        assert_eq!(
            status, "processed",
            "younger demoted chains wait their turn"
        );
    }

    // Re-arming stamps NOW(), so the next pass moves on rather than
    // re-serving the same chain.
    let rearmed = rearm_demoted_chains_limited(&pool, threshold, 1)
        .await
        .unwrap();
    assert_eq!(rearmed, 1);
    let (status, _, _) = chain_state(&pool, &[0xC2u8; 32]).await;
    assert_eq!(
        status, "updated",
        "the sweep advances to the next-oldest chain"
    );
}

/// Retention must not delete a chain whose only unfinished row is a demoted
/// INTERNAL producer.
///
/// The two shapes that make this reachable already have tests of their own;
/// the gap was their combination. An internal (`is_allowed = FALSE`) producer
/// is executed for an allowed consumer and stamped like any other row, so it
/// reaches the demote threshold. When the consumer is filed under a DIFFERENT
/// chain -- routine once several listeners with divergent caches split a
/// transaction -- the producer's chain holds no allowed unfinished row of its
/// own, so an allowed-scoped retention guard sees nothing to protect and ages
/// it out at the TTL.
///
/// Nothing TTL-deletes `computations`, so the row outlives its chain. The work
/// window keys its demotion check on transaction_id rather than on a chain
/// still existing, so it goes on excluding the whole transaction; and the
/// sweep joins `dependence_chain`, so with the chain gone it can never reset
/// the stamp. The allowed consumer stalls permanently.
#[tokio::test]
#[serial(db)]
async fn retention_keeps_a_chain_holding_a_demoted_internal_producer() {
    let instance = setup().await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");
    let threshold: i16 = 3;

    let producer_chain = vec![0xE1u8; 32];
    let consumer_chain = vec![0xE2u8; 32];
    let terminal_chain = vec![0xE3u8; 32];
    let shared_tx = b"tx-split-internal-producer".to_vec();
    let terminal_tx = b"tx-terminal-only".to_vec();

    // All three retired long enough ago to be past the retention TTL.
    for dcid in [&producer_chain, &consumer_chain, &terminal_chain] {
        sqlx::query(
            "INSERT INTO dependence_chain
                (dependence_chain_id, status, last_updated_at, block_timestamp, block_height,
                 schedule_priority, dependency_count, dependents)
             VALUES ($1, 'processed', NOW() - make_interval(secs => 100000), NOW(), 1, 0, 0,
                     ARRAY[]::bytea[])",
        )
        .bind(dcid)
        .execute(&pool)
        .await
        .unwrap();
    }

    // The demoted internal producer: its chain's ONLY unfinished row, and not
    // allowed, so an allowed-scoped guard would not see it.
    seed_computation_row_in_transaction_with_allowed(
        &pool,
        &producer_chain,
        &handle(0xE4),
        &shared_tx,
        false, // is_allowed
        false,
        true,
        Some("RETRYABLE SchedulerError::ExecutionPanic(sigsegv)"),
    )
    .await;
    sqlx::query("UPDATE computations SET error_retry_count = $1 WHERE output_handle = $2")
        .bind(threshold)
        .bind(handle(0xE4))
        .execute(&pool)
        .await
        .unwrap();
    // Its allowed consumer, filed under a different chain by another listener.
    seed_computation_row_in_transaction(
        &pool,
        &consumer_chain,
        &handle(0xE5),
        &shared_tx,
        false,
        false,
        None,
    )
    .await;
    // Control: a chain whose only unfinished row is a TERMINAL verdict must
    // still be deletable, or terminal work accumulates at the head of the
    // `last_updated_at ASC` scan forever.
    seed_computation_row_in_transaction(
        &pool,
        &terminal_chain,
        &handle(0xE6),
        &terminal_tx,
        false,
        true,
        Some("invalid FHE operation: unknown opcode"),
    )
    .await;

    let deleted = delete_old_processed_dependence_chains(&pool, 100, 3600)
        .await
        .unwrap();
    assert!(deleted >= 1, "the terminal-only chain must age out");

    let producer_alive: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM dependence_chain WHERE dependence_chain_id = $1")
            .bind(&producer_chain)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        producer_alive, 1,
        "a demoted INTERNAL producer must hold its chain back: the chain row \
         is the sweep's only handle on it, and its allowed consumer lives in \
         another chain"
    );

    let terminal_alive: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM dependence_chain WHERE dependence_chain_id = $1")
            .bind(&terminal_chain)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        terminal_alive, 0,
        "a terminal verdict never heals and must not block deletion, or it \
         accumulates an unbounded residue"
    );
}
