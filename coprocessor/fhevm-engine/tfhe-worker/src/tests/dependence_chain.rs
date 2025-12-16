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

    let dependence_chain_ids = insert_dependence_chains(&pool, NUM_SAMPLE_CHAINS)
        .await
        .expect("inserted chains");

    let mut workers = vec![];

    for dependence_chain_id in dependence_chain_ids.iter() {
        info!(target: "deps_chain", ?dependence_chain_id, "Testing acquire_next_lock");
        let mut mgr = LockMngr::new_with_ttl(Uuid::new_v4(), pool.clone(), 3600, false, None);

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
async fn test_work_stealing() {
    let instance = setup().await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .expect("Failed to connect to the database");

    let dependence_chain_ids = insert_dependence_chains(&pool, NUM_SAMPLE_CHAINS)
        .await
        .expect("inserted chains");

    let mut workers = vec![];
    let expiration_duration_secs = 1;

    for dependence_chain_id in dependence_chain_ids.iter() {
        info!(?dependence_chain_id, "Testing acquire_next_lock");

        let worker = Uuid::new_v4();
        let mut mgr =
            LockMngr::new_with_ttl(worker, pool.clone(), expiration_duration_secs, false, None);
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
    tokio::time::sleep(std::time::Duration::from_secs(
        3 + expiration_duration_secs as u64,
    ))
    .await;

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

async fn insert_dependence_chains(
    pool: &sqlx::PgPool,
    num_chains: usize,
) -> sqlx::Result<Vec<Vec<u8>>> {
    let mut out = Vec::with_capacity(num_chains);

    for i in 0..num_chains {
        info!("Inserting dependence chain {}", i);
        let dependence_chain_id = i.to_le_bytes().to_vec();

        sqlx::query!(
            r#"
            INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at)
            VALUES ($1, 'updated', NOW())
            "#,
            dependence_chain_id,
        )
        .execute(pool)
        .await?;

        out.push(dependence_chain_id);

        sleep(Duration::from_millis(100)).await;
    }

    assert_locks_available(pool, num_chains).await;

    Ok(out)
}

async fn setup() -> TestInstance {
    let _ = tracing_subscriber::fmt().json().with_level(true).try_init();
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
