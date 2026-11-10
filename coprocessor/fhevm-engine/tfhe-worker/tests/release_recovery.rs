//! Regression coverage for the batched adaptation of #4498's release protocol.
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, PgPool};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tfhe_worker::dependence_chain::LockMngr;
use tokio::time::{sleep, timeout, Duration};
use uuid::Uuid;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn setup() -> Result<(DBInstance, PgPool, LockMngr)> {
    let instance = setup_test_db(ImportMode::None).await?;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(instance.db_url())
        .await?;
    sqlx::query(
        "INSERT INTO dependence_chain
        (dependence_chain_id, status, dependency_count, dependents)
        VALUES ('\\x01', 'updated', 0, ARRAY['\\x02'::bytea]),
               ('\\x02', 'updated', 1, ARRAY[]::bytea[])",
    )
    .execute(&pool)
    .await?;
    let mut worker = LockMngr::new(Uuid::new_v4(), pool.clone());
    assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![1]));
    Ok((instance, pool, worker))
}

#[tokio::test]
#[serial(db)]
async fn release_drops_partial_locks_before_retrying_contention() -> Result {
    for completed_only in [false, true] {
        let (_instance, pool, mut worker) = setup().await?;
        let mut ingestion = pool.begin().await?;
        sqlx::query(
            "SELECT 1 FROM dependence_chain WHERE dependence_chain_id = '\\x02' FOR UPDATE",
        )
        .execute(ingestion.as_mut())
        .await?;
        let release = tokio::spawn(async move {
            if completed_only {
                worker.release_completed_locks().await
            } else {
                worker.release_current_lock(true, None).await
            }
        });
        // The release encounters the busy child after its parent. It must drop
        // that parent lock on every retry so ingestion can complete its pass.
        sleep(Duration::from_millis(150)).await;
        assert!(!release.is_finished());
        timeout(
            Duration::from_millis(500),
            sqlx::query(
                "SELECT 1 FROM dependence_chain WHERE dependence_chain_id = '\\x01' FOR UPDATE",
            )
            .execute(ingestion.as_mut()),
        )
        .await??;
        ingestion.commit().await?;
        assert_eq!(timeout(Duration::from_secs(5), release).await???, 1);
        let state: (String, Vec<Vec<u8>>) = sqlx::query_as(
            "SELECT status, dependents FROM dependence_chain WHERE dependence_chain_id = '\\x01'",
        )
        .fetch_one(&pool)
        .await?;
        assert_eq!(state, ("processed".into(), vec![]));
        let count: i32 = sqlx::query_scalar(
            "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = '\\x02'",
        )
        .fetch_one(&pool)
        .await?;
        assert_eq!(count, 0);
        pool.close().await;
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn failed_decrement_rolls_back_release_and_pruning() -> Result {
    for completed_only in [false, true] {
        let (_instance, pool, mut worker) = setup().await?;
        sqlx::query(
            "ALTER TABLE dependence_chain ADD CONSTRAINT reject_decrement
            CHECK (dependence_chain_id <> '\\x02'::bytea OR dependency_count > 0)",
        )
        .execute(&pool)
        .await?;
        let result = if completed_only {
            worker.release_completed_locks().await
        } else {
            worker.release_current_lock(true, None).await
        };
        assert!(result.is_err());
        let state: (String, Option<Uuid>, Vec<Vec<u8>>) = sqlx::query_as(
            "SELECT status, worker_id, dependents FROM dependence_chain WHERE dependence_chain_id = '\\x01'",
        ).fetch_one(&pool).await?;
        assert_eq!(
            state,
            ("processing".into(), Some(worker.worker_id()), vec![vec![2]])
        );
        sqlx::query("ALTER TABLE dependence_chain DROP CONSTRAINT reject_decrement")
            .execute(&pool)
            .await?;
        let result = if completed_only {
            worker.release_completed_locks().await?
        } else {
            worker.release_current_lock(true, None).await?
        };
        assert_eq!(result, 1);
        assert_eq!(worker.release_completed_locks().await?, 0);
        let count: i32 = sqlx::query_scalar(
            "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = '\\x02'",
        )
        .fetch_one(&pool)
        .await?;
        assert_eq!(count, 0);
        pool.close().await;
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn diagnostic_is_written_in_the_output_transaction() -> Result {
    let (instance, pool, mut worker) = setup().await?;
    // A single connection is enough: recording/flushing must never request a
    // second connection while a computation transaction owns the only one.
    let one = PgPoolOptions::new()
        .max_connections(1)
        .connect(instance.db_url())
        .await?;
    let mut tx = one.begin().await?;
    worker
        .set_processing_error(Some("operation failed".into()), Some(&[1]))
        .await?;
    worker.flush_processing_errors(&mut tx).await?;
    let message: Option<String> = sqlx::query_scalar(
        "SELECT error_message FROM dependence_chain WHERE dependence_chain_id = '\\x01'",
    )
    .fetch_one(tx.as_mut())
    .await?;
    assert_eq!(message.as_deref(), Some("operation failed"));
    tx.rollback().await?;
    let message: Option<String> = sqlx::query_scalar(
        "SELECT error_message FROM dependence_chain WHERE dependence_chain_id = '\\x01'",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(message, None);
    let mut tx = one.begin().await?;
    worker.flush_processing_errors(&mut tx).await?;
    tx.commit().await?;
    worker.clear_processing_errors();
    Ok(())
}
