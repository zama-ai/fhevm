use super::{tests::new_test_iter, *};
use alloy::node_bindings::Anvil;
use alloy::providers::ext::AnvilApi;
use alloy::transports::mock::Asserter;
use fhevm_engine_common::versioning::CUTOVER_LOCK_ID;
use serial_test::serial;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

async fn setup(from: u64) -> Result<(DBInstance, Database, InfiniteLogIter)> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let db = Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
        .await?;
    let mut iter = new_test_iter(50);
    let recovery = RecoveryCheckpoint::new(db.clone(), [None; 5], false, false);
    recovery.start(from, 0).await?;
    iter.recovery = Some(recovery);
    iter.recovery_started = true;
    iter.catchup_blocks = Some((from, None));
    Ok((instance, db, iter))
}

fn block(number: u64) -> BlockLogs<Log> {
    BlockLogs {
        logs: vec![],
        summary: BlockSummary {
            number,
            hash: BlockHash::with_last_byte(number as u8),
            parent_hash: BlockHash::with_last_byte(
                number.saturating_sub(1) as u8
            ),
            timestamp: number,
        },
        catchup: false,
        finalized: false,
    }
}

async fn ingest(db: &mut Database, block: &BlockLogs<Log>) -> Result<()> {
    ingest_block_logs(
        db.chain_id,
        db,
        block,
        &None,
        &None,
        &None,
        &None,
        &None,
        IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: true,
            dependent_ops_max_per_chain: 0,
            is_protocol_config_listener: false,
        },
    )
    .await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn restart_keeps_failed_gap_after_realtime_advances_far_beyond_it(
) -> Result<()> {
    let (_instance, mut db, mut iter) = setup(90).await?;
    let anvil = Anvil::new().spawn();
    let provider = ProviderBuilder::new()
        .connect_ws(WsConnect::new(anvil.ws_endpoint()))
        .await?;
    provider.anvil_mine(Some(113), None).await?;
    iter.url = anvil.ws_endpoint();
    iter.start_at_block = Some(90);
    iter.new_log_stream_no_retry().await?;
    iter.catchup_paging = 4;
    iter.consume_catchup_blocks().await?; // empty finalized page 90..=93
    assert_eq!(iter.recovery.as_ref().unwrap().read().await?.next_block, 94);

    let mut failed = block(100);
    failed.logs.push(Log::default()); // rejected identity, no DB commit
    assert!(ingest(&mut db, &failed).await.is_err());
    iter.retry_from_block(100).await?;
    ingest(&mut db, &block(113)).await?;
    iter.last_valid_block = Some(113);
    iter.consume_catchup_blocks().await?;
    assert_eq!(
        iter.catchup_blocks.unwrap().0,
        94,
        "100 is still inside the finality window"
    );

    // The gap must survive arbitrarily far beyond any fixed startup lookback.
    ingest(&mut db, &block(1_000)).await?;
    db.poller_set_last_caught_up_block(db.chain_id, 1_000)
        .await?;
    drop(iter);
    let mut restarted = new_test_iter(50);
    restarted.url = anvil.ws_endpoint();
    restarted.start_at_block =
        db.read_last_valid_block_checked().await?.map(|n| {
            n.saturating_sub_unsigned(
                restarted.catchup_finalization_in_blocks
                    + restarted.catchup_margin,
            )
        });
    restarted.recovery =
        Some(RecoveryCheckpoint::new(db.clone(), [None; 5], false, false));
    restarted.new_log_stream_no_retry().await?;
    assert_eq!(restarted.catchup_blocks.unwrap().0, 89, "restart includes overlap before the durable frontier, not MAX(block) - margin");
    assert_eq!(
        db.poller_get_last_caught_up_block(db.chain_id).await?,
        Some(1_000)
    );
    Ok(())
}

#[tokio::test]
async fn first_start_covers_finality_and_overlap() -> Result<()> {
    let asserter = Asserter::new();
    asserter.push_success(&alloy::primitives::U64::from(113));
    let provider = ProviderBuilder::new().connect_mocked_client(asserter);
    let iter = new_test_iter(50);
    assert_eq!(iter.catchup_block_from(&provider).await?, 88);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn failure_preserves_queued_ancestors_even_when_their_hashes_are_known(
) -> Result<()> {
    let (_instance, db, mut iter) = setup(100).await?;
    for number in [95, 96] {
        let ancestor = block(number);
        iter.block_history.add_block(ancestor.summary);
        iter.next_blocklogs.push_back(ancestor);
    }
    iter.last_valid_block = Some(200);
    iter.retry_from_block(99).await?;
    assert!(iter.next_blocklogs.is_empty());
    assert_eq!(iter.block_history.size(), 0);
    assert_eq!(iter.catchup_blocks.unwrap().0, 95);
    drop(iter);
    let restarted = RecoveryCheckpoint::new(db, [None; 5], false, false);
    assert_eq!(restarted.start(1_000, 5).await?.next_block, 90);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn restart_replays_a_partially_committed_page() -> Result<()> {
    let (_instance, db, mut iter) = setup(40).await?;
    let revision = iter.recovery.as_ref().unwrap().read().await?.revision;
    iter.pending_catchup_page = Some(CatchupPage {
        from_block: 40,
        next_page: 60,
        remaining_blocks: 2,
        revision,
    });
    iter.acknowledge_catchup_block().await?;
    assert_eq!(iter.recovery.as_ref().unwrap().read().await?.next_block, 40);
    drop(iter);
    let restarted = RecoveryCheckpoint::new(db, [None; 5], false, false);
    assert_eq!(restarted.start(200, 5).await?.next_block, 35);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn stale_page_cannot_clear_another_listeners_recovery_request(
) -> Result<()> {
    let (_instance, db, iter) = setup(40).await?;
    let first = iter.recovery.as_ref().unwrap();
    let second = RecoveryCheckpoint::new(db, [None; 5], false, false);
    let old = first.read().await?;
    second.require(50).await?; // a new gap inside an already fetched page
    assert_eq!(first.complete(40, 60, old.revision).await?.next_block, 40);
    let old = first.read().await?;
    second.require(35).await?;
    assert_eq!(first.complete(40, 60, old.revision).await?.next_block, 35);
    let old = first.read().await?;
    second.require(35).await?; // same height, but a new recovery request
    assert_eq!(first.complete(35, 60, old.revision).await?.next_block, 35);
    let fresh = first.read().await?;
    assert_eq!(first.complete(35, 60, fresh.revision).await?.next_block, 60);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn empty_finalized_page_advances_durable_progress() -> Result<()> {
    let (_instance, _db, mut iter) = setup(40).await?;
    let asserter = Asserter::new();
    asserter.push_success(&Vec::<Log>::new());
    iter.catchup_provider =
        Some(ProviderBuilder::new().connect_mocked_client(asserter));
    iter.block_history.add_block(block(60).summary);
    iter.consume_catchup_blocks().await?;
    assert!(iter.next_blocklogs.is_empty());
    assert_eq!(iter.recovery.as_ref().unwrap().read().await?.next_block, 41);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn checkpoint_write_failure_leaves_the_page_retryable() -> Result<()> {
    let (_instance, db, mut iter) = setup(40).await?;
    let revision = iter.recovery.as_ref().unwrap().read().await?.revision;
    iter.pending_catchup_page = Some(CatchupPage {
        from_block: 40,
        next_page: 60,
        remaining_blocks: 1,
        revision,
    });
    let pool = db.pool().await;
    sqlx::raw_sql("CREATE FUNCTION fail_checkpoint() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'checkpoint write rejected'; END $$;
        CREATE TRIGGER fail_checkpoint BEFORE UPDATE ON host_listener_catchup_state FOR EACH ROW EXECUTE FUNCTION fail_checkpoint();")
        .execute(&pool).await?;
    assert!(iter.acknowledge_catchup_block().await.is_err());
    assert_eq!(iter.catchup_blocks.unwrap().0, 40);
    assert!(iter.pending_catchup_page.is_some());
    assert_eq!(iter.recovery.as_ref().unwrap().read().await?.next_block, 40);
    sqlx::query("DROP TRIGGER fail_checkpoint ON host_listener_catchup_state")
        .execute(&pool)
        .await?;
    iter.finish_pending_catchup_page().await?;
    assert_eq!(iter.recovery.as_ref().unwrap().read().await?.next_block, 60);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn checkpoint_is_scoped_to_contract_roles_listener_role_and_stack(
) -> Result<()> {
    let (_instance, db, _iter) = setup(40).await?;
    let a = Address::with_last_byte(1);
    let b = Address::with_last_byte(2);
    let filtered = RecoveryCheckpoint::new(
        db.clone(),
        [Some(a), Some(b), None, None, None],
        false,
        false,
    );
    assert_eq!(filtered.start(80, 0).await?.next_block, 80);
    let swapped_roles = RecoveryCheckpoint::new(
        db.clone(),
        [Some(b), Some(a), None, None, None],
        false,
        false,
    );
    assert_eq!(swapped_roles.start(100, 0).await?.next_block, 100);
    let protocol = RecoveryCheckpoint::new(
        db.clone(),
        [Some(a), Some(b), None, None, None],
        true,
        false,
    );
    assert_eq!(protocol.start(100, 0).await?.next_block, 100);
    let gcs = RecoveryCheckpoint::new(
        db,
        [Some(a), Some(b), None, None, None],
        false,
        true,
    );
    assert_eq!(gcs.start(120, 0).await?.next_block, 120);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn bootstrap_database_failure_is_not_treated_as_empty_history(
) -> Result<()> {
    let (_instance, db, _iter) = setup(40).await?;
    db.pool().await.close().await;
    assert!(db.read_last_valid_block_checked().await.is_err());
    Ok(())
}

async fn retire(pool: &sqlx::PgPool) -> Result<()> {
    sqlx::query("UPDATE versioning SET stack_version = 'v0.15'")
        .execute(pool)
        .await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
#[tracing_test::traced_test]
async fn retired_listener_startup_without_checkpoint_stays_alive() -> Result<()>
{
    let (instance, db, _iter) = setup(40).await?;
    let pool = db.pool().await;
    sqlx::query("DELETE FROM host_listener_catchup_state")
        .execute(&pool)
        .await?;
    retire(&pool).await?;
    let anvil = Anvil::new().spawn();
    let args = Args::parse_from([
        "host-listener",
        "--url",
        &anvil.ws_endpoint(),
        "--database-url",
        instance.db_url(),
        "--acl-contract-address",
        "0x0000000000000000000000000000000000000001",
        "--tfhe-contract-address",
        "0x0000000000000000000000000000000000000002",
        "--health-port",
        "0",
        "--start-at-block",
        "0",
    ]);
    let listener = main(args);
    tokio::pin!(listener);
    tokio::select! {
        result = &mut listener => panic!("retired listener exited: {result:?}"),
        result = tokio::time::timeout(Duration::from_secs(10), async {
            while !logs_contain("Listener stack retired; pausing ingestion") {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }) => result?,
    }
    // Keep polling the actual entrypoint after it has reached the paused state.
    assert!(
        tokio::time::timeout(Duration::from_millis(100), &mut listener)
            .await
            .is_err()
    );
    let count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM host_listener_catchup_state")
            .fetch_one(&pool)
            .await?;
    assert_eq!(count, 0, "retirement must not create a checkpoint");
    assert!(!logs_contain("Error creating new log stream, retrying"));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn retired_checkpoint_writes_preserve_existing_frontier() -> Result<()> {
    let (_instance, db, iter) = setup(40).await?;
    let recovery = iter.recovery.as_ref().unwrap();
    let before = recovery.read().await?;
    retire(&db.pool().await).await?;
    for result in [
        recovery.start(100, 5).await,
        recovery.require(10).await,
        recovery.complete(40, 60, before.revision).await,
    ] {
        assert!(result.err().unwrap().is::<ListenerRetired>());
    }
    let after = recovery.read().await?;
    assert_eq!(
        (after.next_block, after.revision),
        (before.next_block, before.revision)
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
#[tracing_test::traced_test]
async fn retired_listener_reconnect_exits_retry_loop_without_advancing(
) -> Result<()> {
    let (_instance, db, mut iter) = setup(40).await?;
    let anvil = Anvil::new().spawn();
    iter.url = anvil.ws_endpoint();
    iter.start_at_block = Some(40);
    iter.new_log_stream_no_retry().await?;
    let before = iter.recovery.as_ref().unwrap().read().await?;
    retire(&db.pool().await).await?;
    iter.stream = None;
    let result =
        tokio::time::timeout(Duration::from_secs(2), iter.next()).await?;
    assert!(result.err().unwrap().is::<ListenerRetired>());
    assert_eq!(iter.catchup_blocks.unwrap().0, 40);
    let after = iter.recovery.as_ref().unwrap().read().await?;
    assert_eq!(
        (after.next_block, after.revision),
        (before.next_block, before.revision)
    );
    assert!(!logs_contain("Error creating new log stream, retrying"));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn cutover_while_checkpoint_waits_for_guard_preserves_pending_page(
) -> Result<()> {
    let (_instance, db, mut iter) = setup(40).await?;
    let before = iter.recovery.as_ref().unwrap().read().await?;
    iter.pending_catchup_page = Some(CatchupPage {
        from_block: 40,
        next_page: 60,
        remaining_blocks: 1,
        revision: before.revision,
    });
    let pool = db.pool().await;
    let mut cutover = pool.begin().await?;
    let cutover_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *cutover)
        .await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *cutover)
        .await?;
    let completion = async {
        let result = iter.acknowledge_catchup_block().await;
        (iter, result)
    };
    let commit_cutover = async {
        // Wait for the checkpoint's BEGIN check to pass and its shared lock to
        // block. This exercises Ok(None), rather than the already-retired error.
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let waiting: bool = sqlx::query_scalar(
                    "SELECT EXISTS(SELECT 1 FROM pg_stat_activity
                     WHERE datname = current_database()
                       AND $1 = ANY(pg_blocking_pids(pid)))",
                )
                .bind(cutover_pid)
                .fetch_one(&pool)
                .await?;
                if waiting {
                    return Ok::<_, sqlx::Error>(());
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await??;
        sqlx::query("UPDATE versioning SET stack_version = 'v0.15'")
            .execute(&mut *cutover)
            .await?;
        cutover.commit().await?;
        Ok::<_, anyhow::Error>(())
    };
    let ((iter, result), committed) = tokio::join!(completion, commit_cutover);
    committed?;
    assert!(result.unwrap_err().is::<ListenerRetired>());
    assert_eq!(iter.catchup_blocks.unwrap().0, 40);
    assert!(iter.pending_catchup_page.is_some());
    let after = iter.recovery.as_ref().unwrap().read().await?;
    assert_eq!(
        (after.next_block, after.revision),
        (before.next_block, before.revision)
    );
    Ok(())
}

#[tokio::test]
async fn retired_pause_keeps_heartbeat_alive_and_honors_cancellation(
) -> Result<()> {
    let heartbeat = HeartBeat::with_elapsed_secs(30);
    let cancel = CancellationToken::new();
    let paused = pause_retired_listener(
        Err(ListenerRetired.into()),
        &heartbeat,
        &cancel,
    );
    tokio::pin!(paused);
    assert!(tokio::time::timeout(Duration::from_millis(20), &mut paused)
        .await
        .is_err());
    assert!(heartbeat.is_recent(&Duration::from_secs(1)));
    cancel.cancel();
    tokio::time::timeout(Duration::from_secs(1), paused).await??;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn checkpoint_database_errors_are_not_treated_as_retirement() -> Result<()>
{
    let (_instance, db, iter) = setup(40).await?;
    db.pool().await.close().await;
    let error = iter
        .recovery
        .as_ref()
        .unwrap()
        .start(40, 0)
        .await
        .err()
        .unwrap();
    assert!(matches!(
        error.downcast_ref::<sqlx::Error>(),
        Some(sqlx::Error::PoolClosed)
    ));
    let result = tokio::time::timeout(
        Duration::from_secs(1),
        pause_retired_listener(
            Err(error),
            &HeartBeat::default(),
            &CancellationToken::new(),
        ),
    )
    .await?;
    assert!(matches!(
        result.unwrap_err().downcast_ref::<sqlx::Error>(),
        Some(sqlx::Error::PoolClosed)
    ));
    Ok(())
}
