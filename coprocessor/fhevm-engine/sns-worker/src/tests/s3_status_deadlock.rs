use std::{sync::Arc, time::Duration};

use fhevm_engine_common::chain_id::ChainId;
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, PgPool};
use test_harness::{
    db_utils::{insert_ciphertext64, insert_into_pbs_computations},
    instance::{setup_test_db, ImportMode},
};
use tokio::sync::Barrier;

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
    // Reproduce the previous enqueue order without the production batch
    // prelock. Pause after the first per-handle statement so both batches
    // hold their first stripe before requesting the other.
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

async fn colliding_batches(pool: &PgPool) -> TestResult<[[Vec<u8>; 2]; 2]> {
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
    .fetch_all(pool)
    .await?;
    assert_eq!(stripes.len(), 2);
    let (a, d, x) = stripes[0].clone();
    let (c, b, y) = stripes[1].clone();
    assert_ne!(x, y);
    // A/B belong to batch 1, C/D to batch 2: no row is shared between batches.
    for handle in [&a, &b, &c, &d] {
        insert_ciphertext64(pool, handle, &vec![0xAB; 32]).await?;
        insert_into_pbs_computations(pool, 1, handle).await?;
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
    Ok([[a, b], [c, d]])
}

#[tokio::test]
#[serial(db)]
async fn normal_sns_batches_deadlock_without_stripe_ordering() -> TestResult {
    let instance = setup_test_db(ImportMode::None).await?;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await?;
    let [first_handles, second_handles] = colliding_batches(&pool).await?;
    let all_handles: Vec<_> = first_handles
        .iter()
        .chain(second_handles.iter())
        .cloned()
        .collect();
    let barrier = Barrier::new(2);
    let results = tokio::time::timeout(Duration::from_secs(10), async {
        tokio::join!(
            enqueue_batch(&pool, first_handles, Some(&barrier)),
            enqueue_batch(&pool, second_handles, Some(&barrier)),
        )
    })
    .await
    .expect("PostgreSQL must detect the unordered stripe cycle");
    let error = match results {
        (Ok(()), Err(ExecutionError::DbError(error)))
        | (Err(ExecutionError::DbError(error)), Ok(())) => error,
        results => panic!("expected one successful batch and one deadlock victim: {results:?}"),
    };
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
    let committed: i64 =
        sqlx::query_scalar("SELECT count(*) FROM ciphertext_digest WHERE handle = ANY($1)")
            .bind(all_handles)
            .fetch_one(&pool)
            .await?;
    assert_eq!(committed, 2, "only the surviving batch must commit");
    println!("Confirmed: unordered SNS batches deadlock on shared stripes; PostgreSQL rolls back one batch and lets the other commit. No retry wrapper or S3 migration needed.");
    pool.close().await;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn normal_sns_batches_prelock_shared_stripes_without_deadlock() -> TestResult {
    let instance = setup_test_db(ImportMode::None).await?;
    // Two writers and an observer of their actual PostgreSQL lock state.
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(instance.db_url())
        .await?;
    let [first_handles, mut second_handles] = colliding_batches(&pool).await?;
    let keys: Vec<i64> = sqlx::query_scalar(
        "SELECT hashtextextended(
             '1:digest-mirror:' || abs(hashtextextended(encode(handle, 'hex'), 0) % 256)::text,
             0
         ) FROM unnest($1::bytea[]) AS batch(handle)",
    )
    .bind(second_handles.to_vec())
    .fetch_all(&pool)
    .await?;
    // Deliberately enqueue the larger advisory key first in batch 2. Its
    // pre-lock query must nevertheless wait for the smaller key first.
    if keys[0] < keys[1] {
        second_handles.swap(0, 1);
    }
    let smaller_key = *keys.iter().min().unwrap();
    let all_handles: Vec<_> = first_handles
        .iter()
        .chain(second_handles.iter())
        .cloned()
        .collect();
    let first_tasks: Vec<_> = first_handles.into_iter().map(upload_task).collect();
    let second_tasks: Vec<_> = second_handles.into_iter().map(upload_task).collect();

    let mut first = pool.begin().await?;
    let first_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(first.as_mut())
        .await?;
    crate::executor::lock_digest_mirror_stripes(&mut first, &first_tasks).await?;

    let mut second = pool.begin().await?;
    let second_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(second.as_mut())
        .await?;
    sqlx::query("SET LOCAL statement_timeout = '10s'")
        .execute(second.as_mut())
        .await?;
    let second_batch = tokio::spawn(async move {
        crate::executor::enqueue_upload_tasks(&mut second, &second_tasks).await?;
        second.commit().await?;
        Ok::<_, ExecutionError>(())
    });

    // Observe blocking directly, rather than hoping the two futures overlap.
    // No retry wrapper is used: either transaction error fails this test.
    let waiting_key = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let key: Option<i64> = sqlx::query_scalar(
                "SELECT (classid::bigint << 32) | objid::bigint
                 FROM pg_locks
                 WHERE pid = $1 AND locktype = 'advisory' AND NOT granted",
            )
            .bind(second_pid)
            .fetch_optional(&pool)
            .await?;
            if let Some(key) = key {
                break Ok::<_, sqlx::Error>(key);
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("second batch must wait for a shared stripe")?;
    assert_eq!(
        waiting_key, smaller_key,
        "lock order must ignore input order"
    );
    let second_held: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_locks
         WHERE pid = $1 AND locktype = 'advisory' AND granted",
    )
    .bind(second_pid)
    .fetch_one(&pool)
    .await?;
    assert_eq!(second_held, 0, "wait before acquiring a larger stripe");

    crate::executor::enqueue_upload_tasks(&mut first, &first_tasks).await?;
    let first_held: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_locks
         WHERE pid = $1 AND locktype = 'advisory' AND granted",
    )
    .bind(first_pid)
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        first_held, 2,
        "pre-lock keys must match the real trigger keys"
    );
    first.commit().await?;
    tokio::time::timeout(Duration::from_secs(5), second_batch)
        .await
        .expect("second batch must finish after the first commits")??;

    let committed: i64 =
        sqlx::query_scalar("SELECT count(*) FROM ciphertext_digest WHERE handle = ANY($1)")
            .bind(all_handles)
            .fetch_one(&pool)
            .await?;
    assert_eq!(committed, 4, "both batches must commit without a retry");
    println!("Confirmed: reverse-order SNS batches acquire the same advisory keys in order, wait without a deadlock, and commit all four handles without retrying.");
    pool.close().await;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn batch_prelocks_cover_ct64_only_enqueues_and_release_on_rollback() -> TestResult {
    let instance = setup_test_db(ImportMode::None).await?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(instance.db_url())
        .await?;
    let [[handle, _], _] = colliding_batches(&pool).await?;
    let mut task = upload_task(handle.clone());
    // A failed/absent CT128 still takes a digest stripe when enqueueing CT64.
    task.ct128 = Arc::new(BigCiphertext::default());

    let mut transaction = pool.begin().await?;
    crate::executor::lock_digest_mirror_stripes(&mut transaction, &[]).await?;
    let empty_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_locks WHERE pid = pg_backend_pid() AND locktype = 'advisory'",
    )
    .fetch_one(transaction.as_mut())
    .await?;
    assert_eq!(empty_count, 0, "empty batches must not take stripes");

    crate::executor::lock_digest_mirror_stripes(&mut transaction, std::slice::from_ref(&task))
        .await?;
    let prelocked: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_locks WHERE pid = pg_backend_pid() AND locktype = 'advisory'",
    )
    .fetch_one(transaction.as_mut())
    .await?;
    assert_eq!(prelocked, 1, "CT64-only work must pre-acquire its stripe");
    assert!(task.enqueue_upload_task(&mut transaction).await?);
    transaction.rollback().await?;

    let remaining: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_locks WHERE pid = pg_backend_pid() AND locktype = 'advisory'",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        remaining, 0,
        "rollback must release transaction-scoped locks"
    );
    let persisted: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM ciphertext_digest WHERE handle = $1)")
            .bind(handle)
            .fetch_one(&pool)
            .await?;
    assert!(!persisted, "the enqueue must roll back with its locks");
    pool.close().await;
    Ok(())
}
