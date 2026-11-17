use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use fhevm_engine_common::{
    chain_id::ChainId,
    pg_pool::{PostgresPoolManager, ServiceError},
};
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, PgPool};
use test_harness::{
    db_utils::{insert_ciphertext64, insert_into_pbs_computations},
    instance::{setup_test_db, ImportMode},
};
use tokio::sync::Barrier;
use tokio_util::sync::CancellationToken;

use crate::{BigCiphertext, Ciphertext128Format, ExecutionError, HandleItem};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn upload_task(handle: Vec<u8>) -> HandleItem {
    // Only the production enqueue method runs: these bytes are not decrypted
    // or uploaded. It needs a non-empty CT128 with a known format.
    HandleItem {
        handle,
        host_chain_id: ChainId::try_from(1_i64).unwrap(),
        key_id_gw: vec![7; 32],
        ct64_compressed: Arc::new(vec![1]),
        ct128: Arc::new(BigCiphertext::new(
            vec![2],
            Ciphertext128Format::CompressedOnCpu,
        )),
        ct64_digest: None,
        ct128_digest: None,
        s3_format_version: None,
        transaction_id: None,
        span: tracing::Span::none(),
    }
}

async fn enqueue_batch(
    pool: &PgPool,
    handles: [Vec<u8>; 2],
    first_insert_done: Option<&Barrier>,
) -> Result<(), ExecutionError> {
    let mut transaction = pool.begin().await?;
    sqlx::query("SET LOCAL statement_timeout = '10s'")
        .execute(transaction.as_mut())
        .await?;
    // executor::enqueue_upload_tasks calls this method once per handle while
    // retaining the batch transaction. Pause after the first statement so
    // both batches hold their first stripe before requesting the other.
    for (index, handle) in handles.into_iter().enumerate() {
        let enqueued = upload_task(handle)
            .enqueue_upload_task(&mut transaction)
            .await;
        match enqueued {
            Ok(true) => {}
            Ok(false) => {
                transaction.rollback().await?;
                return Err(ExecutionError::InternalError(
                    "fixture provenance missing".into(),
                ));
            }
            Err(error) => {
                transaction.rollback().await?;
                return Err(error);
            }
        }
        if index == 0 {
            if let Some(barrier) = first_insert_done {
                tokio::time::timeout(Duration::from_secs(5), barrier.wait())
                    .await
                    .map_err(|_| {
                        ExecutionError::InternalError("other batch did not reach barrier".into())
                    })?;
            }
        }
    }
    transaction.commit().await?;
    Ok(())
}

async fn enqueue_batch_with_retry(
    manager: &PostgresPoolManager,
    handles: [Vec<u8>; 2],
    barrier: &Barrier,
    attempts: &AtomicUsize,
    deadlocks: &AtomicUsize,
) -> Result<(), ServiceError> {
    manager
        .blocking_with_db_retry(
            |pool, _| {
                let handles = handles.clone();
                // Synchronize only the initial attempts. The deadlock victim
                // retries the whole transaction after PostgreSQL rolls it back.
                let barrier = (attempts.fetch_add(1, Ordering::SeqCst) == 0).then_some(barrier);
                async move {
                    match enqueue_batch(&pool, handles, barrier).await {
                        Ok(()) => Ok(()),
                        Err(ExecutionError::DbError(error)) => {
                            let db_error = error
                                .as_database_error()
                                .expect("expected a PostgreSQL deadlock")
                                .downcast_ref::<sqlx::postgres::PgDatabaseError>();
                            assert_eq!(db_error.code(), "40P01");
                            let detail = db_error.detail().expect("deadlock wait cycle");
                            assert_eq!(detail.matches("advisory lock").count(), 2, "{detail}");
                            println!(
                                "SQLSTATE {}: {}\n{detail}",
                                db_error.code(),
                                db_error.message()
                            );
                            println!("CONTEXT: {}", db_error.r#where().unwrap_or_default());
                            deadlocks.fetch_add(1, Ordering::SeqCst);
                            Err(ExecutionError::DbError(error).into())
                        }
                        Err(error) => panic!("unexpected enqueue error: {error:?}"),
                    }
                }
            },
            "sns-deadlock-test",
        )
        .await
}

#[tokio::test]
#[serial(db)]
async fn normal_sns_batches_retry_deadlock_without_backoff() -> TestResult {
    let instance = setup_test_db(ImportMode::None).await?;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await?;
    // Compute the hashes in PostgreSQL, matching the real installed trigger.
    // Each result supplies two different handles sharing one stripe; the two
    // results have different stripes. No advisory lock is acquired manually.
    let stripes: Vec<(Vec<u8>, Vec<u8>, i64)> = sqlx::query_as(
        "WITH candidates AS (
             SELECT lpad(to_hex(n), 64, '0') AS handle_hex
             FROM generate_series(1, 1024) n
         ), striped AS (
             SELECT handle_hex, abs(hashtextextended(handle_hex, 0) % 256) AS stripe
             FROM candidates
         )
         SELECT decode(min(handle_hex), 'hex'), decode(max(handle_hex), 'hex'), stripe
         FROM striped GROUP BY stripe HAVING count(*) > 1 ORDER BY stripe LIMIT 2",
    )
    .fetch_all(&pool)
    .await?;
    assert_eq!(stripes.len(), 2);
    let (a, d, x) = stripes[0].clone();
    let (c, b, y) = stripes[1].clone();
    assert_ne!(x, y);
    // A/B belong to batch 1, C/D to batch 2: no row is shared between batches.
    for handle in [&a, &b, &c, &d] {
        insert_ciphertext64(&pool, handle, &vec![0xAB; 32]).await?;
        insert_into_pbs_computations(&pool, 1, handle).await?;
    }
    println!(
        "Batch 1: A={} stripe {x}, B={} stripe {y}",
        hex::encode(&a),
        hex::encode(&b)
    );
    println!(
        "Batch 2: C={} stripe {y}, D={} stripe {x}",
        hex::encode(&c),
        hex::encode(&d)
    );
    let manager = PostgresPoolManager::connect_pool(
        CancellationToken::new(),
        instance.db_url(),
        Duration::from_secs(5),
        2,
        // A generic error would sleep 60s. Both batches must finish within
        // 10s, including PostgreSQL's own deadlock detection (normally 1s).
        Duration::from_secs(30),
        None,
    )
    .await
    .expect("test database pool");
    let barrier = Barrier::new(2);
    let first_attempts = AtomicUsize::new(0);
    let second_attempts = AtomicUsize::new(0);
    let deadlocks = AtomicUsize::new(0);
    tokio::time::timeout(Duration::from_secs(10), async {
        tokio::try_join!(
            enqueue_batch_with_retry(
                &manager,
                [a.clone(), b.clone()],
                &barrier,
                &first_attempts,
                &deadlocks,
            ),
            enqueue_batch_with_retry(
                &manager,
                [c.clone(), d.clone()],
                &barrier,
                &second_attempts,
                &deadlocks,
            ),
        )
    })
    .await
    .expect("deadlock retry must not sleep for the 60s error backoff")?;
    assert_eq!(deadlocks.load(Ordering::SeqCst), 1);
    let mut attempts = [
        first_attempts.load(Ordering::SeqCst),
        second_attempts.load(Ordering::SeqCst),
    ];
    attempts.sort_unstable();
    assert_eq!(attempts, [1, 2], "only the deadlock victim must retry");
    let committed: i64 =
        sqlx::query_scalar("SELECT count(*) FROM ciphertext_digest WHERE handle = ANY($1)")
            .bind(vec![a, b, c, d])
            .fetch_one(&pool)
            .await?;
    assert_eq!(committed, 4, "both batches must be fully committed");
    println!("Confirmed: two normal SNS batches deadlock on shared stripes; the victim retries without backoff and all four handles commit. No S3 migration needed.");
    manager.pool().close().await;
    pool.close().await;
    Ok(())
}
