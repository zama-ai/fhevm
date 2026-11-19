//! CFI-0024: a descendant in another transaction must not prevent slow-lane recovery.
use fhevm_engine_common::types::SchedulePriority;
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, PgPool};
use test_harness::instance::{setup_test_db, ImportMode};
use tfhe_worker::dependence_chain::LockMngr;
use uuid::Uuid;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
const THRESHOLD: i16 = 3;

fn handle(id: u8) -> Vec<u8> {
    vec![id; 32]
}

fn manager(pool: &PgPool) -> LockMngr {
    LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        3600,
        false,
        None,
        None,
        None,
        THRESHOLD,
    )
}

async fn chain(pool: &PgPool, id: u8, debt: i32, dependents: &[u8]) -> Result {
    sqlx::query(
        "INSERT INTO dependence_chain
         (dependence_chain_id, status, dependency_count, dependents)
         VALUES ($1, 'updated', $2, $3)",
    )
    .bind(handle(id))
    .bind(debt)
    .bind(dependents.iter().map(|id| handle(*id)).collect::<Vec<_>>())
    .execute(pool)
    .await?;
    Ok(())
}

async fn computation(
    pool: &PgPool,
    output: u8,
    chain: u8,
    transaction: u8,
    dependencies: &[u8],
    allowed: bool,
) -> Result {
    sqlx::query(
        "INSERT INTO computations
         (output_handle, dependence_chain_id, transaction_id, dependencies,
          fhe_operation, is_scalar, is_allowed, host_chain_id, operand_boundary_mask)
         VALUES ($1, $2, $3, $4, 0, false, $5, 1, $6)",
    )
    .bind(handle(output))
    .bind(handle(chain))
    .bind(handle(transaction))
    .bind(
        dependencies
            .iter()
            .map(|id| handle(*id))
            .collect::<Vec<_>>(),
    )
    .bind(allowed)
    .bind(vec![0_u8; 32])
    .execute(pool)
    .await?;
    Ok(())
}

async fn debt(pool: &PgPool, chain: u8) -> Result<i32> {
    Ok(sqlx::query_scalar(
        "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(handle(chain))
    .fetch_one(pool)
    .await?)
}

#[tokio::test]
#[serial(db)]
async fn cross_transaction_descendants_retire_and_rearm_with_their_producer() -> Result {
    for producer_allowed in [true, false] {
        let db = setup_test_db(ImportMode::None).await?;
        let pool = PgPoolOptions::new()
            .max_connections(3)
            .connect(db.db_url())
            .await?;
        // Two independent parents gate chain 3. Only chain 1 is demoted.
        chain(&pool, 1, 0, &[3]).await?;
        chain(&pool, 2, 0, &[3]).await?;
        chain(&pool, 3, 2, &[]).await?;
        computation(&pool, 11, 1, 11, &[], producer_allowed).await?;
        // A local intermediate and allowed consumer in a DIFFERENT transaction.
        computation(&pool, 12, 1, 12, &[11], false).await?;
        computation(&pool, 13, 1, 12, &[12], true).await?;
        // Healthy work sharing the scheduling unit must survive demotion too.
        computation(&pool, 14, 1, 14, &[], true).await?;
        computation(&pool, 15, 1, 15, &[], true).await?;
        computation(&pool, 21, 2, 21, &[], true).await?;
        computation(&pool, 31, 3, 31, &[13, 21], true).await?;
        sqlx::query(
            "UPDATE computations SET is_error = true, error_retry_count = $1,
             error_message = CASE WHEN output_handle = $2 THEN 'RETRYABLE execution panic'
                                  ELSE 'terminal invalid operation' END
             WHERE output_handle = ANY($3)",
        )
        .bind(THRESHOLD)
        .bind(handle(11))
        .bind(vec![handle(11), handle(15)])
        .execute(&pool)
        .await?;

        let mut worker = manager(&pool);
        worker.acquire_next_locks(8).await?;
        assert_eq!(worker.get_current_lock_ids(), vec![handle(1), handle(2)]);
        assert_eq!(
            worker.release_completed_locks().await?,
            1,
            "CFI-0024: a cross-transaction descendant must not block producer rearm"
        );
        let retired: (String, Option<Uuid>, Vec<Vec<u8>>) = sqlx::query_as(
            "SELECT status, worker_id, dependents FROM dependence_chain WHERE dependence_chain_id = $1",
        ).bind(handle(1)).fetch_one(&pool).await?;
        assert_eq!(retired, ("processed".into(), None, vec![]));
        assert_eq!(
            debt(&pool, 3).await?,
            1,
            "the other parent still owes a gate"
        );
        assert_eq!(worker.release_completed_locks().await?, 0);
        assert_eq!(debt(&pool, 3).await?, 1);
        let pending: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM computations WHERE dependence_chain_id = $1 AND NOT is_completed",
        )
        .bind(handle(1))
        .fetch_one(&pool)
        .await?;
        assert_eq!(
            pending, 5,
            "retirement must not discard or complete any computation"
        );

        // Exercise the public maintenance path, including the real rearm SQL.
        worker.do_cleanup().await?;
        let rearmed: (String, Option<Uuid>, i16) = sqlx::query_as(
            "SELECT status, worker_id, schedule_priority FROM dependence_chain WHERE dependence_chain_id = $1",
        ).bind(handle(1)).fetch_one(&pool).await?;
        assert_eq!(
            rearmed,
            ("updated".into(), None, i16::from(SchedulePriority::Slow))
        );
        let stamps: Vec<(i16, String)> = sqlx::query_as(
            "SELECT error_retry_count, error_message FROM computations
             WHERE output_handle = ANY($1) ORDER BY output_handle",
        )
        .bind(vec![handle(11), handle(15)])
        .fetch_all(&pool)
        .await?;
        assert_eq!(
            stamps,
            vec![
                (0, "RETRYABLE execution panic".into()),
                (THRESHOLD, "terminal invalid operation".into())
            ]
        );

        chain(&pool, 4, 0, &[]).await?;
        let mut replacement = manager(&pool);
        assert_eq!(
            replacement.acquire_next_lock().await?.0,
            Some(handle(4)),
            "fresh fast work must precede the rearmed slow chain"
        );
        replacement.release_completed_locks().await?;
        assert_eq!(replacement.acquire_next_lock().await?.0, Some(handle(1)));
        // Model the committed successful computation state. The worker-level
        // saved reproducer separately checks execution and actual outputs.
        sqlx::query(
            "UPDATE computations SET is_completed = true, is_error = false, error_message = NULL
             WHERE dependence_chain_id = $1 AND output_handle <> $2",
        )
        .bind(handle(1))
        .bind(handle(15))
        .execute(&pool)
        .await?;
        assert_eq!(replacement.release_completed_locks().await?, 1);
        assert_eq!(
            debt(&pool, 3).await?,
            1,
            "rearm must not discharge the old edge twice"
        );
        replacement.do_cleanup().await?;
        let status: String = sqlx::query_scalar(
            "SELECT status FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(handle(1))
        .fetch_one(&pool)
        .await?;
        assert_eq!(
            status, "processed",
            "a terminal row alone must not rearm the chain"
        );
        pool.close().await;
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn retirement_requires_own_pending_retryable_row_at_threshold() -> Result {
    let db = setup_test_db(ImportMode::None).await?;
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(db.db_url())
        .await?;
    for id in 1..=6 {
        chain(&pool, id, 0, &[]).await?;
        computation(&pool, id, id, 40, &[], true).await?;
        if id >= 3 {
            computation(&pool, id + 10, id, id + 10, &[], true).await?;
        }
    }
    // All six chains share a transaction with chain 2's demoted producer.
    // Only that chain may retire; the others have independently pending work.
    sqlx::query(
        "UPDATE computations SET error_retry_count = $1,
         is_error = (output_handle <> $2),
         is_completed = (output_handle = $3),
         error_message = CASE WHEN output_handle = $4 THEN 'terminal invalid operation'
                              ELSE 'RETRYABLE execution panic' END
         WHERE output_handle = ANY($5)",
    )
    .bind(THRESHOLD)
    .bind(handle(6))
    .bind(handle(5))
    .bind(handle(4))
    .bind((2..=6).map(handle).collect::<Vec<_>>())
    .execute(&pool)
    .await?;
    sqlx::query("UPDATE computations SET error_retry_count = $1 WHERE output_handle = $2")
        .bind(THRESHOLD - 1)
        .bind(handle(3))
        .execute(&pool)
        .await?;
    let mut worker = manager(&pool);
    assert_eq!(worker.acquire_next_locks(8).await?.len(), 6);
    assert_eq!(worker.release_completed_locks().await?, 1);
    assert_eq!(worker.get_current_lock_ids(), vec![handle(1), handle(3), handle(4), handle(5), handle(6)],
               "foreign, below-threshold, terminal, completed and non-error stamps cannot retire healthy work");
    pool.close().await;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn listener_refresh_preserves_debt_until_the_next_demotion_release() -> Result {
    let db = setup_test_db(ImportMode::None).await?;
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(db.db_url())
        .await?;
    chain(&pool, 1, 0, &[2]).await?;
    chain(&pool, 2, 1, &[]).await?;
    computation(&pool, 11, 1, 11, &[], true).await?;
    computation(&pool, 12, 1, 12, &[11], true).await?;
    sqlx::query("UPDATE computations SET is_error = true, error_message = 'RETRYABLE panic', error_retry_count = $1 WHERE output_handle = $2")
        .bind(THRESHOLD).bind(handle(11)).execute(&pool).await?;
    let mut worker = manager(&pool);
    assert_eq!(worker.acquire_next_lock().await?.0, Some(handle(1)));
    // The listener refreshes an already-owned chain; release must preserve it.
    sqlx::query("UPDATE dependence_chain SET status = 'updated' WHERE dependence_chain_id = $1")
        .bind(handle(1))
        .execute(&pool)
        .await?;
    assert_eq!(worker.release_completed_locks().await?, 1);
    worker.do_cleanup().await?;
    let refreshed: (String, Option<Uuid>, Vec<Vec<u8>>) = sqlx::query_as(
        "SELECT status, worker_id, dependents FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(handle(1))
    .fetch_one(&pool)
    .await?;
    assert_eq!(refreshed, ("updated".into(), None, vec![handle(2)]));
    assert_eq!(debt(&pool, 2).await?, 1);
    let retries: i16 =
        sqlx::query_scalar("SELECT error_retry_count FROM computations WHERE output_handle = $1")
            .bind(handle(11))
            .fetch_one(&pool)
            .await?;
    assert_eq!(
        retries, THRESHOLD,
        "a refresh alone must not reset the attempt budget"
    );

    // Once ingestion stops refreshing the chain, acquisition/release can retire
    // it and the maintenance sweep can make the producer selectable again.
    let mut replacement = manager(&pool);
    assert_eq!(replacement.acquire_next_lock().await?.0, Some(handle(1)));
    assert_eq!(replacement.release_completed_locks().await?, 1);
    assert_eq!(debt(&pool, 2).await?, 0);
    replacement.do_cleanup().await?;
    let state: (String, i16) = sqlx::query_as(
        "SELECT dc.status, c.error_retry_count FROM dependence_chain dc JOIN computations c
         ON c.dependence_chain_id = dc.dependence_chain_id WHERE c.output_handle = $1",
    )
    .bind(handle(11))
    .fetch_one(&pool)
    .await?;
    assert_eq!(state, ("updated".into(), 0));
    pool.close().await;
    Ok(())
}
