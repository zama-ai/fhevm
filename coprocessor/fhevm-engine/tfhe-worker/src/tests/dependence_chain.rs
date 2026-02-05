use crate::dependence_chain::{LockMngr, LockingReason};
use crate::tests::utils::{setup_test_app, TestInstance};
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
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, schedule_lane)
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
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, schedule_lane)
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
    let test_instance = setup_test_app().await.expect("valid db instance");
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
