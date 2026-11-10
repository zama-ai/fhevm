//! A warm ingestion replay must not deadlock with batched worker diagnostics.
use alloy::primitives::{Address, FixedBytes, LogData};
use alloy::rpc::types::Log;
use alloy::sol_types::SolEvent;
use fhevm_engine_common::chain_id::ChainId;
use host_listener::cmd::block_history::BlockSummary;
use host_listener::contracts::{AclContract::Allowed, TfheContract};
use host_listener::database::ingest::{ingest_block_logs, BlockLogs, IngestOptions};
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;
use test_harness::instance::{setup_test_db, ImportMode};
use tfhe_worker::dependence_chain::LockMngr;
use tokio::time::{sleep, timeout, Duration};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn handle(id: u8) -> FixedBytes<32> {
    let mut bytes = [0; 32];
    bytes[0] = id;
    bytes[21] = 255;
    bytes[22..30].copy_from_slice(&42_u64.to_be_bytes());
    bytes[30] = 5;
    bytes.into()
}

fn block(logs: Vec<LogData>) -> BlockLogs<Log> {
    let summary = BlockSummary {
        number: 100,
        hash: FixedBytes::repeat_byte(100),
        parent_hash: FixedBytes::repeat_byte(99),
        timestamp: 100,
    };
    BlockLogs {
        logs: logs
            .into_iter()
            .enumerate()
            .map(|(index, data)| Log {
                inner: alloy::primitives::Log {
                    address: Address::ZERO,
                    data,
                },
                block_hash: Some(summary.hash),
                block_number: Some(summary.number),
                transaction_hash: Some(FixedBytes::repeat_byte(10)),
                transaction_index: Some(0),
                log_index: Some(index as u64),
                ..Default::default()
            })
            .collect(),
        summary,
        catchup: true,
        finalized: false,
    }
}

async fn ingest(db: &mut Database, logs: Vec<LogData>) -> Result<()> {
    ingest_block_logs(
        ChainId::try_from(42_u64)?,
        db,
        &block(logs),
        &Some(Address::ZERO),
        &Some(Address::ZERO),
        &None,
        &None,
        &None,
        IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: true,
            dependent_ops_max_per_chain: 0,
            is_protocol_config_listener: false,
            disable_synthetic_ops: true,
        },
    )
    .await?
    .require_complete()?;
    Ok(())
}

fn add(result: FixedBytes<32>) -> LogData {
    TfheContract::FheAdd {
        caller: Address::ZERO,
        lhs: handle(1),
        rhs: handle(2),
        scalarByte: FixedBytes([0]),
        result,
    }
    .encode_log_data()
}

async fn setup() -> Result<(test_harness::instance::DBInstance, Database)> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    let db = Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 64).await?;
    Ok((instance, db))
}

async fn wait_for_query(pool: &sqlx::PgPool, wait: &str, query_pattern: &str) -> Result<()> {
    timeout(Duration::from_secs(10), async {
        loop {
            let found: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE datname=current_database() AND pid<>pg_backend_pid() AND wait_event=$1 AND query LIKE $2)")
                .bind(wait).bind(query_pattern).fetch_one(pool).await?;
            if found { return Ok::<_, sqlx::Error>(()); }
            sleep(Duration::from_millis(20)).await;
        }
    }).await??;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn diagnostics_skip_locked_parents_during_warm_ingestion_replay() -> Result<()> {
    let (_instance, mut db) = setup().await?;
    let pool = db.pool().await;
    let child = FixedBytes::<32>::repeat_byte(10);
    let parent_b = FixedBytes::<32>::repeat_byte(32);
    let parent_c = FixedBytes::<32>::repeat_byte(48);
    let result_handle = handle(10);
    let op = add(result_handle);
    let acl = Allowed {
        caller: Address::ZERO,
        account: Address::ZERO,
        handle: result_handle,
    }
    .encode_log_data();
    // A cold first delivery stores this join under its own transaction hash.
    ingest(&mut db, vec![op.clone(), acl]).await?;
    for parent in [parent_b, parent_c] {
        sqlx::query("INSERT INTO dependence_chain(dependence_chain_id,status,dependency_count,dependents,error_message) VALUES ($1,'updated',0,'{}',$2)")
            .bind(parent.as_slice())
            .bind((parent == parent_b).then_some("previous parent diagnostic"))
            .execute(&pool).await?;
    }
    // A later warm delivery knows the two cross-block producers. This creates
    // a join with outer parents while retaining the same persisted child ID.
    db.dependence_chain.write().await.put(handle(1), parent_b);
    db.dependence_chain.write().await.put(handle(2), parent_c);
    let mut worker = LockMngr::new(Uuid::new_v4(), pool.clone());
    let acquired = worker.acquire_next_locks(3).await?;
    assert_eq!(acquired.len(), 3, "{acquired:?}");
    worker
        .set_processing_error(Some("failed child".into()), Some(child.as_slice()))
        .await?;
    worker
        .set_processing_error(Some("failed parent".into()), Some(parent_b.as_slice()))
        .await?;
    let mut barrier = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(9494403)")
        .execute(barrier.as_mut())
        .await?;
    // Pause only at the child upsert, after the real parent-gating SQL ran.
    sqlx::raw_sql(&format!(r#"
        CREATE FUNCTION pause_child_upsert() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN
          IF NEW.dependence_chain_id = decode('{}','hex') THEN
            PERFORM pg_advisory_xact_lock(9494403);
          END IF;
          RETURN NEW;
        END $$;
        CREATE TRIGGER pause_child_upsert BEFORE INSERT ON dependence_chain FOR EACH ROW EXECUTE FUNCTION pause_child_upsert();
    "#, hex::encode(child))).execute(&pool).await?;
    let listener = tokio::spawn(async move {
        let extra_acl = Allowed {
            caller: Address::ZERO,
            account: Address::repeat_byte(1),
            handle: result_handle,
        }
        .encode_log_data();
        ingest(&mut db, vec![op, extra_acl]).await
    });
    wait_for_query(&pool, "advisory", "%INSERT INTO dependence_chain%").await?;
    // Ingestion owns B and C, and is paused immediately before its A upsert.
    // The worker must be able to write A's diagnostic and commit its verdict
    // while skipping B. A blocking A -> B flush would deadlock against B -> A
    // when the listener resumes.
    let flushed = timeout(Duration::from_secs(5), async {
        let mut tx = pool.begin().await?;
        sqlx::query("UPDATE computations SET is_error=TRUE, error_message='failed child' WHERE output_handle=$1")
            .bind(result_handle.as_slice()).execute(tx.as_mut()).await?;
        worker.flush_processing_errors(&mut tx).await?;
        tx.commit().await?;
        worker.clear_processing_errors();
        Ok::<_, sqlx::Error>(())
    })
    .await;
    // Always release the test barrier, including when the old blocking flush
    // times out, so a regression cannot leave the listener task stuck.
    barrier.rollback().await?;
    timeout(Duration::from_secs(10), listener).await???;
    flushed.expect("diagnostic flush waited for the listener's parent locks")?;

    let messages: Vec<(Vec<u8>, Option<String>)> = sqlx::query_as(
        "SELECT dependence_chain_id, error_message FROM dependence_chain ORDER BY dependence_chain_id",
    )
    .fetch_all(&pool)
    .await?;
    assert_eq!(
        messages,
        vec![
            (child.to_vec(), Some("failed child".into())),
            (parent_b.to_vec(), Some("previous parent diagnostic".into())),
            (parent_c.to_vec(), None),
        ]
    );
    let verdict: (bool, Option<String>) =
        sqlx::query_as("SELECT is_error, error_message FROM computations WHERE output_handle=$1")
            .bind(result_handle.as_slice())
            .fetch_one(&pool)
            .await?;
    assert_eq!(verdict, (true, Some("failed child".into())));
    // A skipped informational message is discarded after a successful commit;
    // it must not leak into a later cycle when that row becomes available.
    let mut next = pool.begin().await?;
    worker.flush_processing_errors(&mut next).await?;
    next.commit().await?;
    let parent_message: Option<String> = sqlx::query_scalar(
        "SELECT error_message FROM dependence_chain WHERE dependence_chain_id=$1",
    )
    .bind(parent_b.as_slice())
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        parent_message.as_deref(),
        Some("previous parent diagnostic")
    );
    Ok(())
}
