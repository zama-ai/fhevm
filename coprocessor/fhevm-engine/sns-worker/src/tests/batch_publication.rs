use super::{enqueue_upload_tasks, query_sns_tasks, Order};
use crate::{BigCiphertext, Ciphertext128Format, ExecutionError};
use fhevm_engine_common::pg_pool::{PostgresPoolManager, ServiceError};
use serial_test::serial;
use sqlx::PgPool;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use test_harness::instance::{setup_test_db, ImportMode};
use tokio_util::sync::CancellationToken;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Observe a single real enqueue. A worker that first acquires new mirror
/// stripes partway through the writes can wait while holding a partial batch.
#[tokio::test]
#[serial(db)]
async fn sns_batch_reserves_its_mirror_locks_before_the_first_digest_write() -> TestResult {
    let db = setup_test_db(ImportMode::None).await?;
    let pool = PgPool::connect(db.db_url()).await?;
    let mut tx = pool.begin().await?;

    // Shadow just the work queue so a persistent test database cannot supply
    // unrelated jobs. The installed digest table and its mirror triggers run as
    // deployed. The observation trigger and all fixture rows roll back together.
    sqlx::raw_sql(
        r#"
        CREATE TEMP TABLE ciphertexts (LIKE public.ciphertexts INCLUDING DEFAULTS);
        CREATE TEMP TABLE pbs_computations (LIKE public.pbs_computations INCLUDING DEFAULTS);
        INSERT INTO ciphertexts (handle, ciphertext, ciphertext_type, ciphertext_version)
            SELECT sha256(int4send(i)), '\x01', 2, 0 FROM generate_series(1, 20) i;
        INSERT INTO pbs_computations (handle, host_chain_id)
            SELECT handle, 11155111 FROM ciphertexts;

        CREATE TEMP TABLE sns_enqueue_observation (locks_before_write bigint);
        CREATE TEMP VIEW sns_held_locks AS
            SELECT count(*) AS n FROM pg_locks
             WHERE pid = pg_backend_pid() AND locktype = 'advisory' AND granted;
        CREATE FUNCTION pg_temp.observe_sns_enqueue() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN
            INSERT INTO pg_temp.sns_enqueue_observation SELECT n FROM pg_temp.sns_held_locks;
            RETURN NEW;
        END $$;
        CREATE TRIGGER observe_sns_enqueue BEFORE INSERT ON public.ciphertext_digest
            FOR EACH ROW EXECUTE FUNCTION pg_temp.observe_sns_enqueue();
        "#,
    )
    .execute(&mut *tx)
    .await?;

    let mut tasks = query_sns_tasks(&mut tx, 20, Order::Asc, &vec![0; 32])
        .await?
        .expect("seeded SNS work");
    for task in &mut tasks {
        task.ct128 = Arc::new(BigCiphertext::new(
            vec![1],
            Ciphertext128Format::CompressedOnCpu,
        ));
    }
    enqueue_upload_tasks(&mut tx, &tasks).await?;
    let (writes, minimum, final_count): (i64, Option<i64>, i64) = sqlx::query_as(
        "SELECT count(*), min(locks_before_write), (SELECT n FROM pg_temp.sns_held_locks)
         FROM pg_temp.sns_enqueue_observation",
    )
    .fetch_one(&mut *tx)
    .await?;
    tx.rollback().await?;

    assert_eq!(writes, 20);
    assert!(
        final_count > 1,
        "the fixture must exercise multiple stripes"
    );
    assert_eq!(
        minimum,
        Some(final_count),
        "the batch acquired additional mirror locks during enqueue"
    );
    Ok(())
}

/// Exercise the SNS error conversion and the actual service retry boundary.
/// PostgreSQL supplies the SQLSTATE without needing a second transaction.
#[tokio::test]
#[serial(db)]
async fn sns_deadlock_victim_reenters_the_service_without_connection_backoff() -> TestResult {
    let db = setup_test_db(ImportMode::None).await?;
    let cancel = CancellationToken::new();
    let manager = PostgresPoolManager::connect_pool(
        cancel.clone(),
        db.db_url(),
        Duration::from_secs(10),
        2,
        Duration::from_secs(60),
        None,
    )
    .await
    .expect("test database pool");
    let error = sqlx::query(
        "DO $$ BEGIN RAISE EXCEPTION 'SNS test victim' USING ERRCODE = '40P01'; END $$",
    )
    .execute(&manager.pool())
    .await
    .expect_err("injected transaction failure");
    assert_eq!(
        error.as_database_error().and_then(|e| e.code()).as_deref(),
        Some("40P01")
    );
    let first_attempt = Mutex::new(Some(ServiceError::from(ExecutionError::DbError(error))));
    let attempts = AtomicUsize::new(0);
    let retry = manager.blocking_with_db_retry(
        |_, _| {
            attempts.fetch_add(1, Ordering::Relaxed);
            std::future::ready(first_attempt.lock().unwrap().take().map_or(Ok(()), Err))
        },
        "sns",
    );

    // Pool initialization and SQL ran before starting the clock. This bounds only
    // the two in-memory service calls, with a configured ordinary backoff of 60s.
    let result = tokio::time::timeout(Duration::from_secs(5), retry).await;
    cancel.cancel();
    result.expect("a deadlock victim waited for the 60s connection backoff")?;
    assert_eq!(attempts.load(Ordering::Relaxed), 2);
    Ok(())
}

/// Exercise independent, ordinary batches through the enqueue entry point.
/// Only the test's handles are visible to the assertion and cleanup. PostgreSQL
/// schedules the competing transactions without test triggers or lock barriers.
#[tokio::test]
#[serial(db)]
async fn parallel_sns_batches_commit_all_digest_rows_without_deadlock() -> TestResult {
    use crate::HandleItem;
    use futures::{stream, StreamExt};

    let db = setup_test_db(ImportMode::None).await?;
    let pool = PgPool::connect(db.db_url()).await?;
    let handles: Vec<Vec<u8>> = sqlx::query_scalar(
        "SELECT sha256(uuid_send(gen_random_uuid())) FROM generate_series(1, 800)",
    )
    .fetch_all(&pool)
    .await?;
    let mut seed = pool.begin().await?;
    sqlx::query(
        "INSERT INTO ciphertexts(handle, ciphertext, ciphertext_type, ciphertext_version)
                 SELECT h, decode('01', 'hex'), 2, 0 FROM unnest($1::bytea[]) h",
    )
    .bind(&handles)
    .execute(&mut *seed)
    .await?;
    sqlx::query(
        "INSERT INTO pbs_computations(handle, host_chain_id)
                 SELECT h, 11155111 FROM unnest($1::bytea[]) h",
    )
    .bind(&handles)
    .execute(&mut *seed)
    .await?;
    seed.commit().await?;

    let host_chain_id = 11155111_i64.try_into()?;
    let tasks: Vec<_> = handles
        .iter()
        .map(|handle| HandleItem {
            host_chain_id,
            handle: handle.clone(),
            key_id_gw: vec![0; 32],
            ct64_compressed: Arc::new(vec![1]),
            ct128: Arc::new(BigCiphertext::new(
                vec![1],
                Ciphertext128Format::CompressedOnCpu,
            )),
            ct64_digest: None,
            ct128_digest: None,
            s3_format_version: None,
            span: tracing::Span::none(),
            transaction_id: None,
        })
        .collect();

    let worker_pool = &pool;
    let enqueue = |batch| async move {
        let mut tx = worker_pool.begin().await?;
        enqueue_upload_tasks(&mut tx, batch).await?;
        tx.commit().await?;
        Ok::<_, ExecutionError>(())
    };
    let errors: Vec<_> = stream::iter(tasks.chunks(20).map(enqueue))
        .buffer_unordered(4)
        .filter_map(|result| std::future::ready(result.err()))
        .collect()
        .await;

    let mut cleanup = pool.begin().await?;
    let committed = sqlx::query("DELETE FROM ciphertext_digest WHERE handle = ANY($1)")
        .bind(&handles)
        .execute(&mut *cleanup)
        .await?
        .rows_affected();
    sqlx::query("DELETE FROM pbs_computations WHERE handle = ANY($1)")
        .bind(&handles)
        .execute(&mut *cleanup)
        .await?;
    sqlx::query("DELETE FROM ciphertexts WHERE handle = ANY($1)")
        .bind(&handles)
        .execute(&mut *cleanup)
        .await?;
    cleanup.commit().await?;
    let mut victims = 0;
    for error in errors {
        match error {
            ExecutionError::DbError(ref db)
                if db.as_database_error().and_then(|e| e.code()).as_deref() == Some("40P01") =>
            {
                victims += 1
            }
            other => return Err(other.into()),
        }
    }
    assert_eq!(
        victims, 0,
        "PostgreSQL reported 40P01 while enqueuing independent SNS batches"
    );
    assert_eq!(committed as usize, handles.len());
    Ok(())
}
