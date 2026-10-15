//! Exercise production chain scheduling and release against PostgreSQL without
//! starting the FHE worker or generating keys.

use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, PgPool};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tfhe_worker::dependence_chain::{LockMngr, LockingReason};
use tokio::time::{timeout, Duration};
use uuid::Uuid;

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::rpc::types::Log;
use alloy::sol_types::SolEvent;
use fhevm_engine_common::chain_id::ChainId;
use host_listener::cmd::block_history::BlockSummary;
use host_listener::contracts::{AclContract, TfheContract};
use host_listener::database::ingest::{ingest_block_logs, BlockLogs, IngestOptions};
use host_listener::database::tfhe_event_propagate::Database;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn setup() -> Result<(DBInstance, PgPool)> {
    let instance = setup_test_db(ImportMode::None).await?;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(instance.db_url())
        .await?;
    Ok((instance, pool))
}

async fn chain(pool: &PgPool, id: u8, age: i32, count: i32, priority: i16) -> Result {
    sqlx::query(
        "INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at, dependency_count, schedule_priority)
         VALUES ($1, 'updated', NOW() - make_interval(secs => $2), $3, $4)",
    )
    .bind(vec![id]).bind(age).bind(count).bind(priority).execute(pool).await?;
    Ok(())
}

fn manager(pool: &PgPool) -> LockMngr {
    LockMngr::new(Uuid::new_v4(), pool.clone())
}

async fn dependent_count(pool: &PgPool, id: u8) -> Result<i32> {
    Ok(sqlx::query_scalar(
        "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(vec![id])
    .fetch_one(pool)
    .await?)
}

async fn wait_for_lock(pool: &PgPool, query_pattern: &str, pid: Option<i32>) -> Result {
    timeout(Duration::from_secs(5), async {
        loop {
            let waiting: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM pg_stat_activity WHERE datname = current_database()
                 AND wait_event_type = 'Lock' AND query LIKE $1
                 AND ($2::int IS NULL OR pid = $2))",
            )
            .bind(query_pattern)
            .bind(pid)
            .fetch_one(pool)
            .await?;
            if waiting {
                return Ok::<_, sqlx::Error>(());
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await??;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn retry_cooldown_is_shared_and_preserves_queue_position_in_both_schedulers() -> Result {
    let (_instance, pool) = setup().await?;
    for fallback in [false, true] {
        sqlx::query("TRUNCATE dependence_chain")
            .execute(&pool)
            .await?;
        chain(&pool, 1, 20, 0, 0).await?; // consumer retained the earlier schedule
        chain(&pool, 2, 10, 0, 0).await?; // producer repaired by replay
        let mut worker = manager(&pool);
        let first = if fallback {
            worker.acquire_early_lock().await?
        } else {
            worker.acquire_next_lock().await?
        };
        assert_eq!(first.0, Some(vec![1]));
        let original_queue_position = worker.get_current_lock().unwrap().last_updated_at;
        worker.release_current_lock(false, true).await?;
        let state: (String, bool, bool, bool, bool) = sqlx::query_as(
            "SELECT status, worker_id IS NULL, lock_acquired_at IS NULL,
                    lock_expires_at > clock_timestamp(),
                    lock_expires_at <= clock_timestamp() + INTERVAL '1 second'
             FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(vec![1_u8])
        .fetch_one(&pool)
        .await?;
        assert_eq!(state, ("updated".into(), true, true, true, true));
        // Another pod observes the same cooldown, regardless of which
        // acquisition path it uses. The repaired producer can run meanwhile.
        let mut other_worker = manager(&pool);
        let second = if fallback {
            other_worker.acquire_early_lock().await?
        } else {
            other_worker.acquire_next_lock().await?
        };
        assert_eq!(second.0, Some(vec![2]), "fallback={fallback}");
        assert_eq!(worker.acquire_next_lock().await?.0, None);
        assert_eq!(worker.acquire_early_lock().await?.0, None);
        other_worker.release_current_lock(true, false).await?;
        // Add unrelated, newer backlog. Expire the deadline deterministically;
        // real idle expiry is tested separately. The blocked consumer retains
        // precedence over this backlog in normal scheduling.
        chain(&pool, 3, 5, 0, 0).await?;
        sqlx::query("UPDATE dependence_chain SET lock_expires_at = NOW() - INTERVAL '1 second' WHERE dependence_chain_id = $1")
            .bind(vec![1_u8]).execute(&pool).await?;
        // Normal scheduling restores the old queue position. Rescue scheduling
        // separately rotates by retry deadlines so stale dependency counters
        // cannot leave a newer producer behind repeatedly retried consumers.
        let retried = worker.acquire_next_lock().await?;
        assert_eq!(retried, (Some(vec![1]), LockingReason::UpdatedUnowned));
        assert_eq!(
            worker.get_current_lock().unwrap().last_updated_at,
            original_queue_position
        );
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn fallback_rotates_expired_retries_until_a_newer_stale_count_producer_runs() -> Result {
    let (_instance, pool) = setup().await?;
    chain(&pool, 1, 50, 0, 0).await?; // oldest blocked consumer
    chain(&pool, 2, 40, 0, 0).await?; // another blocked consumer
    chain(&pool, 3, 30, 1, 1).await?; // newer repaired producer with a stale count
    chain(&pool, 4, 20, 0, 0).await?; // unrelated productive backlog
    let mut worker = manager(&pool);
    for expected in 1..=3 {
        assert_eq!(worker.acquire_early_lock().await?.0, Some(vec![expected]));
        worker.release_current_lock(false, true).await?;
        // Model productive work between rescues lasting longer than the
        // cooldown. Eligibility alone must not keep selecting chain 1 again.
        sqlx::query("UPDATE dependence_chain SET lock_expires_at = NOW() - INTERVAL '1 second' WHERE dependence_chain_id = $1")
            .bind(vec![expected]).execute(&pool).await?;
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn periodic_rescue_does_not_repeat_an_expired_old_consumer() -> Result {
    let (_instance, pool) = setup().await?;
    chain(&pool, 1, 30, 0, 0).await?;
    chain(&pool, 2, 20, 1, 1).await?;
    chain(&pool, 3, 10, 0, 0).await?;
    let mut worker = manager(&pool);
    worker.set_dependency_count_fallback_interval(1);
    assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![1]));
    worker.release_current_lock(false, true).await?;
    sqlx::query("UPDATE dependence_chain SET lock_expires_at = NOW() - INTERVAL '1 second' WHERE dependence_chain_id = $1")
        .bind(vec![1_u8]).execute(&pool).await?;
    // The periodic fallback must choose the producer despite the consumer
    // already being eligible again and ordinary count-zero backlog remaining.
    assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![2]));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn idle_retry_becomes_eligible_when_shared_cooldown_expires() -> Result {
    let (_instance, pool) = setup().await?;
    chain(&pool, 1, 20, 0, 0).await?;
    let mut first_pod = manager(&pool);
    assert_eq!(first_pod.acquire_next_lock().await?.0, Some(vec![1]));
    first_pod.release_current_lock(false, true).await?;
    let mut second_pod = manager(&pool);
    assert_eq!(second_pod.acquire_next_lock().await?.0, None);
    assert_eq!(second_pod.acquire_early_lock().await?.0, None);
    // No ingestion or notification clears the deadline. Ordinary polling
    // recovers the only chain after the fixed one-second database cooldown.
    let retried = timeout(Duration::from_secs(5), async {
        loop {
            let candidate = second_pod.acquire_next_lock().await?;
            if candidate.0.is_some() {
                return Ok::<_, sqlx::Error>(candidate);
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await??;
    assert_eq!(retried, (Some(vec![1]), LockingReason::UpdatedUnowned));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn completed_chain_has_no_cooldown_and_cannot_be_stolen_without_an_owner() -> Result {
    let (_instance, pool) = setup().await?;
    chain(&pool, 1, 20, 0, 0).await?;
    let mut worker = manager(&pool);
    assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![1]));
    worker.release_current_lock(true, true).await?;
    let state: (String, bool) = sqlx::query_as(
        "SELECT status, lock_expires_at IS NULL FROM dependence_chain WHERE dependence_chain_id = $1",
    ).bind(vec![1_u8]).fetch_one(&pool).await?;
    assert_eq!(state, ("processed".into(), true));
    // An old ownerless deadline is not an expired worker lease. In particular,
    // it must not make a processed chain eligible through the stealing branch.
    sqlx::query("UPDATE dependence_chain SET lock_expires_at = NOW() - INTERVAL '1 second' WHERE dependence_chain_id = $1")
        .bind(vec![1_u8]).execute(&pool).await?;
    assert_eq!(worker.acquire_next_lock().await?.0, None);
    assert_eq!(worker.acquire_early_lock().await?.0, None);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn idle_scheduler_recovers_stale_counts_after_ready_work() -> Result {
    let (_instance, pool) = setup().await?;
    chain(&pool, 1, 20, 1, 0).await?;
    chain(&pool, 2, 10, 0, 1).await?;
    let mut worker = manager(&pool);
    // Ready slow-lane work still precedes stale fast-lane counters.
    assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![2]));
    worker.release_current_lock(true, false).await?;
    assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![1]));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn productive_ready_work_cannot_postpone_counter_recovery_forever() -> Result {
    let (_instance, pool) = setup().await?;
    chain(&pool, 1, 100, 1, 1).await?;
    for id in 2..=5 {
        chain(&pool, id, 10, 0, 0).await?;
    }
    let mut worker = manager(&pool);
    worker.set_dependency_count_fallback_interval(2);
    for _ in 0..2 {
        let next = worker.acquire_next_lock().await?.0.expect("ready work");
        assert_ne!(next, vec![1]);
        worker.release_current_lock(true, false).await?;
    }
    // There is still productive count-zero work, but its successful completion
    // must not reset the independent rescue cadence.
    assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![1]));
    worker.release_current_lock(false, true).await?;
    assert_ne!(worker.acquire_next_lock().await?.0, Some(vec![1]));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn expired_positive_count_owner_is_recoverable_but_live_owner_is_preserved() -> Result {
    let (_instance, pool) = setup().await?;
    chain(&pool, 1, 20, 1, 0).await?;
    chain(&pool, 2, 10, 1, 0).await?;
    let old_owner = Uuid::new_v4();
    sqlx::query("UPDATE dependence_chain SET status = 'processing', worker_id = $1, lock_expires_at = NOW() + INTERVAL '1 hour'")
        .bind(old_owner).execute(&pool).await?;
    let mut worker = manager(&pool);
    assert_eq!(worker.acquire_next_lock().await?.0, None);
    sqlx::query("UPDATE dependence_chain SET lock_expires_at = NOW() - INTERVAL '1 second' WHERE dependence_chain_id = $1")
        .bind(vec![2_u8]).execute(&pool).await?;
    let (acquired, reason) = worker.acquire_next_lock().await?;
    assert_eq!(acquired, Some(vec![2]));
    assert_eq!(reason, LockingReason::ExpiredLock);
    let owner: Uuid =
        sqlx::query_scalar("SELECT worker_id FROM dependence_chain WHERE dependence_chain_id = $1")
            .bind(vec![1_u8])
            .fetch_one(&pool)
            .await?;
    assert_eq!(owner, old_owner);
    Ok(())
}

async fn parent_and_child(pool: &PgPool) -> Result<LockMngr> {
    chain(pool, 1, 20, 0, 0).await?;
    chain(pool, 2, 10, 1, 0).await?;
    sqlx::query(
        "UPDATE dependence_chain SET dependents = ARRAY[$1::bytea] WHERE dependence_chain_id = $2",
    )
    .bind(vec![2_u8])
    .bind(vec![1_u8])
    .execute(pool)
    .await?;
    let mut worker = manager(pool);
    assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![1]));
    Ok(worker)
}

#[tokio::test]
#[serial(db)]
async fn dependent_update_failure_rolls_back_release_and_retry_decrements_once() -> Result {
    let (_instance, pool) = setup().await?;
    let mut worker = parent_and_child(&pool).await?;
    // Force a real database error in the second half of release. The parent
    // must remain owned/processing rather than commit without notifying child.
    sqlx::query("ALTER TABLE dependence_chain ADD CONSTRAINT reject_test_decrement CHECK (dependence_chain_id <> '\\x02'::bytea OR dependency_count > 0)")
        .execute(&pool).await?;
    assert!(worker.release_current_lock(true, false).await.is_err());
    let row: (String, Option<Uuid>) = sqlx::query_as(
        "SELECT status, worker_id FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(vec![1_u8])
    .fetch_one(&pool)
    .await?;
    assert_eq!(row, ("processing".into(), Some(worker.worker_id())));
    assert_eq!(dependent_count(&pool, 2).await?, 1);
    sqlx::query("ALTER TABLE dependence_chain DROP CONSTRAINT reject_test_decrement")
        .execute(&pool)
        .await?;
    assert_eq!(worker.release_current_lock(true, false).await?, 1);
    assert_eq!(dependent_count(&pool, 2).await?, 0);
    assert_eq!(worker.release_current_lock(true, false).await?, 0);
    assert_eq!(dependent_count(&pool, 2).await?, 0);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn release_retries_without_deadlocking_with_listener_chain_locks() -> Result {
    let (_instance, pool) = setup().await?;
    // Cover both hash orders: the listener can first upsert an inferred chain
    // and then repair a different stored chain. Also check that a listener
    // wakeup or a new owner during the retry is not overwritten.
    for (parent, child, wakeup, steal) in [
        (2_u8, 1_u8, false, false),
        (1, 2, false, false),
        (2, 1, true, false),
        (2, 1, false, true),
    ] {
        sqlx::query("TRUNCATE dependence_chain")
            .execute(&pool)
            .await?;
        chain(&pool, parent, 20, 0, 0).await?;
        // An unlocked dependent sorts before the busy one. Starting counts at
        // two makes an accidental double decrement observable despite GREATEST.
        chain(&pool, 0, 10, 2, 0).await?;
        chain(&pool, child, 10, 2, 0).await?;
        sqlx::query("UPDATE dependence_chain SET dependents = ARRAY['\\x00'::bytea, $1::bytea] WHERE dependence_chain_id = $2")
            .bind(vec![child]).bind(vec![parent]).execute(&pool).await?;
        let mut worker = manager(&pool);
        assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![parent]));

        let mut gate = pool.begin().await?;
        sqlx::query("SELECT 1 FROM dependence_chain WHERE dependence_chain_id = $1 FOR UPDATE")
            .bind(vec![parent])
            .execute(&mut *gate)
            .await?;
        let mut ingestion = pool.begin().await?;
        let listener_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&mut *ingestion)
            .await?;
        sqlx::query("SELECT 1 FROM dependence_chain WHERE dependence_chain_id = $1 FOR UPDATE")
            .bind(vec![child])
            .execute(&mut *ingestion)
            .await?;

        let release = tokio::spawn(async move {
            let result = worker.release_current_lock(true, false).await;
            (result, worker)
        });
        // Queue the worker on the parent first, followed by the listener that
        // already holds the child. Releasing the gate forces parent -> child
        // versus child -> parent without timing sleeps or production hooks.
        wait_for_lock(&pool, "%SELECT status, dependents%", None).await?;
        let new_owner = Uuid::new_v4();
        let listener = tokio::spawn(async move {
            sqlx::query("SELECT 1 FROM dependence_chain WHERE dependence_chain_id = $1 FOR UPDATE")
                .bind(vec![parent])
                .execute(&mut *ingestion)
                .await?;
            if wakeup {
                sqlx::query(
                    "UPDATE dependence_chain SET status = 'updated' WHERE dependence_chain_id = $1",
                )
                .bind(vec![parent])
                .execute(&mut *ingestion)
                .await?;
            }
            if steal {
                sqlx::query(
                    "UPDATE dependence_chain SET worker_id = $1 WHERE dependence_chain_id = $2",
                )
                .bind(new_owner)
                .bind(vec![parent])
                .execute(&mut *ingestion)
                .await?;
            }
            ingestion.commit().await
        });
        wait_for_lock(&pool, "%FOR UPDATE%", Some(listener_pid)).await?;
        gate.rollback().await?;

        timeout(Duration::from_secs(5), listener).await???;
        let (released, mut worker) = timeout(Duration::from_secs(5), release).await??;
        assert_eq!(released?, u64::from(!steal));
        let row: (String, Option<Uuid>) = sqlx::query_as(
            "SELECT status, worker_id FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(vec![parent])
        .fetch_one(&pool)
        .await?;
        let expected_status = if steal {
            "processing"
        } else if wakeup {
            "updated"
        } else {
            "processed"
        };
        assert_eq!(row, (expected_status.into(), steal.then_some(new_owner)));
        assert_eq!(
            dependent_count(&pool, child).await?,
            1 + i32::from(wakeup || steal)
        );
        assert_eq!(
            dependent_count(&pool, 0).await?,
            1 + i32::from(wakeup || steal)
        );
        assert_eq!(worker.release_current_lock(true, false).await?, 0);
        assert_eq!(
            dependent_count(&pool, child).await?,
            1 + i32::from(wakeup || steal)
        );
        assert_eq!(
            dependent_count(&pool, 0).await?,
            1 + i32::from(wakeup || steal)
        );
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn cleanup_cannot_remove_parent_before_dependent_notification_commits() -> Result {
    let (_instance, pool) = setup().await?;
    let mut worker = parent_and_child(&pool).await?;
    // Pause inside the dependent UPDATE, after the parent has been changed but
    // before either change commits. A row-lock blocker is unsuitable here now
    // that release deliberately retries instead of waiting on dependent rows.
    sqlx::raw_sql(
        "CREATE FUNCTION pause_test_dependent_update() RETURNS trigger LANGUAGE plpgsql AS $$
         BEGIN
             PERFORM pg_advisory_xact_lock(4278, 1);
             RETURN NEW;
         END;
         $$;
         CREATE TRIGGER pause_test_dependent_update BEFORE UPDATE OF dependency_count ON dependence_chain
         FOR EACH ROW EXECUTE FUNCTION pause_test_dependent_update();",
    ).execute(&pool).await?;
    let mut blocker = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(4278, 1)")
        .execute(&mut *blocker)
        .await?;
    let release = tokio::spawn(async move { worker.release_current_lock(true, false).await });
    wait_for_lock(
        &pool,
        "%WHERE dependence_chain_id = ANY ($1::bytea[])%",
        None,
    )
    .await?;
    let mut cleaner = LockMngr::new_with_conf(
        Uuid::new_v4(),
        pool.clone(),
        30,
        false,
        None,
        Some(0),
        Some(0),
    );
    assert_eq!(cleaner.do_cleanup().await?, 0);
    blocker.rollback().await?;
    assert_eq!(timeout(Duration::from_secs(5), release).await???, 1);
    assert_eq!(dependent_count(&pool, 2).await?, 0);
    assert_eq!(cleaner.do_cleanup().await?, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn listener_wakeup_and_stolen_ownership_do_not_decrement_dependents() -> Result {
    let (_instance, pool) = setup().await?;
    let mut worker = parent_and_child(&pool).await?;
    sqlx::query("UPDATE dependence_chain SET status = 'updated' WHERE dependence_chain_id = $1")
        .bind(vec![1_u8])
        .execute(&pool)
        .await?;
    worker.release_current_lock(true, false).await?;
    assert_eq!(dependent_count(&pool, 2).await?, 1);
    assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![1]));
    sqlx::query("UPDATE dependence_chain SET worker_id = $1 WHERE dependence_chain_id = $2")
        .bind(Uuid::new_v4())
        .bind(vec![1_u8])
        .execute(&pool)
        .await?;
    assert_eq!(worker.release_current_lock(true, false).await?, 0);
    assert_eq!(dependent_count(&pool, 2).await?, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn processing_error_uses_and_rolls_back_with_the_computation_transaction() -> Result {
    let (instance, pool) = setup().await?;
    chain(&pool, 1, 20, 0, 0).await?;
    // A single connection catches any attempt to acquire another pooled
    // connection while the computation transaction owns the only connection.
    let one_connection = PgPoolOptions::new()
        .max_connections(1)
        .connect(instance.db_url())
        .await?;
    let mut worker = manager(&one_connection);
    assert_eq!(worker.acquire_next_lock().await?.0, Some(vec![1]));
    worker.record_processing_error("operation failed".into());
    let mut tx = one_connection.begin().await?;
    timeout(
        Duration::from_secs(2),
        worker.flush_processing_error(&mut tx),
    )
    .await??;
    tx.rollback().await?;
    let error: Option<String> = sqlx::query_scalar(
        "SELECT error_message FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(vec![1_u8])
    .fetch_one(&pool)
    .await?;
    assert_eq!(error, None);
    // The pending annotation survives a rollback and is only cleared once the
    // computation transaction has committed successfully.
    let mut tx = one_connection.begin().await?;
    worker.flush_processing_error(&mut tx).await?;
    tx.commit().await?;
    worker.clear_processing_error();
    let error: Option<String> = sqlx::query_scalar(
        "SELECT error_message FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(vec![1_u8])
    .fetch_one(&pool)
    .await?;
    assert_eq!(error.as_deref(), Some("operation failed"));
    Ok(())
}

fn log<E: SolEvent>(event: &E, block: &BlockSummary, tx: FixedBytes<32>, index: u64) -> Log {
    Log {
        inner: alloy::primitives::Log {
            address: Address::ZERO,
            data: event.encode_log_data(),
        },
        block_hash: Some(block.hash),
        block_number: Some(block.number),
        block_timestamp: None,
        transaction_hash: Some(tx),
        transaction_index: Some(0),
        log_index: Some(index),
        removed: false,
    }
}

fn allow(handle: FixedBytes<32>) -> AclContract::Allowed {
    AclContract::Allowed {
        caller: Address::ZERO,
        account: Address::with_last_byte(1),
        handle,
    }
}

fn add(input: FixedBytes<32>, result: FixedBytes<32>) -> TfheContract::FheAdd {
    TfheContract::FheAdd {
        caller: Address::ZERO,
        lhs: input,
        rhs: input,
        scalarByte: FixedBytes::ZERO,
        result,
    }
}

async fn ingest(db: &mut Database, block: BlockSummary, logs: Vec<Log>) -> Result {
    ingest_block_logs(
        ChainId::try_from(42_u64)?,
        db,
        &BlockLogs {
            logs,
            summary: block,
            catchup: true,
            finalized: false,
        },
        &None,
        &None,
        &None,
        IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: true,
            dependent_ops_max_per_chain: 0,
        },
    )
    .await?;
    Ok(())
}

async fn finish_computations(pool: &PgPool, chain: &[u8]) -> Result {
    // Model only FHE results; scheduling, ownership and dependent notification
    // below always use the production LockMngr.
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type) SELECT output_handle, '\\x00'::bytea, 0, 5 FROM computations WHERE dependence_chain_id = $1 AND is_allowed ON CONFLICT DO NOTHING")
        .bind(chain).execute(&mut *tx).await?;
    sqlx::query(
        "UPDATE computations SET is_completed = TRUE WHERE dependence_chain_id = $1 AND is_allowed",
    )
    .bind(chain)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

async fn replayed_fork_makes_progress(missing_acl_only: bool, cold_replay: bool) -> Result {
    let (instance, pool) = setup().await?;
    let mut db = Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128).await?;
    let block = BlockSummary {
        number: 100,
        hash: FixedBytes::with_last_byte(0xb1),
        parent_hash: FixedBytes::with_last_byte(0xb0),
        timestamp: 1_790_000_000,
    };
    let [a, b, c, d] = [0xa1, 0xb1, 0xc1, 0xd1].map(FixedBytes::<32>::with_last_byte);
    let [ha, hb, hc, hd] = [0x11, 0x12, 0x13, 0x14].map(FixedBytes::<32>::with_last_byte);
    let logs = vec![
        log(
            &TfheContract::TrivialEncrypt {
                caller: Address::ZERO,
                pt: U256::from(7),
                toType: 5,
                result: ha,
            },
            &block,
            a,
            0,
        ),
        log(&allow(ha), &block, a, 1),
        log(&add(ha, hb), &block, b, 2),
        log(&allow(hb), &block, b, 3),
        log(&add(hb, hc), &block, c, 4),
        log(&allow(hc), &block, c, 5),
        log(&add(ha, hd), &block, d, 6),
        log(&allow(hd), &block, d, 7),
    ];
    // A -> B -> C, A -> D. Without B or its allow event the first pass
    // schedules C as an independent depth-zero chain, ahead of repaired B.
    let partial = logs
        .iter()
        .filter(|event| {
            event.log_index != Some(3) && (missing_acl_only || event.log_index != Some(2))
        })
        .cloned()
        .collect();
    ingest(&mut db, block, partial).await?;
    // Break the tie between the initially independent A and C deterministically.
    sqlx::query("UPDATE dependence_chain SET last_updated_at = last_updated_at - INTERVAL '1 second' WHERE dependence_chain_id = $1")
        .bind(a.to_vec()).execute(&pool).await?;
    let mut worker = manager(&pool);
    assert_eq!(worker.acquire_next_lock().await?.0, Some(a.to_vec()));
    finish_computations(&pool, a.as_slice()).await?;
    worker.release_current_lock(true, false).await?;
    if cold_replay {
        db = Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128).await?;
    }
    ingest(&mut db, block, logs).await?;
    let mut reached_b = false;
    for _ in 0..10 {
        let next = worker
            .acquire_next_lock()
            .await?
            .0
            .expect("pending graph remains reachable");
        if next == b.to_vec() {
            reached_b = true;
            finish_computations(&pool, &next).await?;
            worker.release_current_lock(true, false).await?;
            break;
        }
        if next == c.to_vec() {
            worker.release_current_lock(false, true).await?;
        } else {
            finish_computations(&pool, &next).await?;
            worker.release_current_lock(true, false).await?;
        }
    }
    assert!(
        reached_b,
        "a replayed producer must not starve behind its consumer"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn replay_missing_acl_recovers_producer_before_consumer() -> Result {
    replayed_fork_makes_progress(true, false).await
}

#[tokio::test]
#[serial(db)]
async fn cold_replay_missing_acl_recovers_producer_before_consumer() -> Result {
    replayed_fork_makes_progress(true, true).await
}

#[tokio::test]
#[serial(db)]
async fn replay_missing_operation_recovers_producer_before_consumer() -> Result {
    replayed_fork_makes_progress(false, false).await
}

#[tokio::test]
#[serial(db)]
async fn cold_replay_missing_operation_recovers_producer_before_consumer() -> Result {
    replayed_fork_makes_progress(false, true).await
}
