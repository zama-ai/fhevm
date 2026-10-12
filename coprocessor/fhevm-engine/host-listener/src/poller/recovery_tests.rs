use super::*;
use alloy::primitives::B256;
use fhevm_engine_common::chain_recovery::{
    begin_replay, complete_replay, is_halted, prepare_host_replay,
    readiness_blocked, record_halt, reset_green_replay_tx, HistoryMismatch,
    HostReplayStartup, ListenerSource,
};
use fhevm_engine_common::versioning::{
    begin_write_guarded, GcsRollbackPolicy, WriteGuard, CUTOVER_LOCK_ID,
};
use test_harness::instance::{setup_test_db, ImportMode};

#[tokio::test]
async fn deferred_finalization_checks_all_retained_neighbors() -> Result<()> {
    use crate::database::ingest::update_finalized_blocks_aux;
    for neighbor in [99_i64, 100, 101] {
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|e| anyhow!("{e}"))?;
        let chain = ChainId::try_from(42_u64)?;
        let mut db =
            Database::new(&instance.db_url().to_owned().into(), chain, 64)
                .await?;
        let pool = db.pool().await;
        let candidate = B256::repeat_byte(10);
        sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id,block_hash,parent_hash,block_number,block_status) VALUES (42,$1,$2,100,'pending'), (42,$3,$3,$4,'finalized')")
            .bind(candidate.as_slice()).bind(B256::repeat_byte(9).as_slice())
            .bind(B256::repeat_byte(20).as_slice()).bind(neighbor).execute(&pool).await?;
        update_finalized_blocks_aux(&mut db, 100, 0, |_| async {
            Ok(candidate)
        })
        .await;
        assert!(
            is_halted(&pool).await?,
            "neighbor {neighbor} must be contained"
        );
        let status: String = sqlx::query_scalar("SELECT block_status FROM host_chain_blocks_valid WHERE chain_id=42 AND block_hash=$1")
            .bind(candidate.as_slice()).fetch_one(&pool).await?;
        assert_eq!(
            status, "pending",
            "conflicting finalization must roll back"
        );
        let retained: String = sqlx::query_scalar("SELECT block_status FROM host_chain_blocks_valid WHERE chain_id=42 AND block_hash=$1")
            .bind(B256::repeat_byte(20).as_slice()).fetch_one(&pool).await?;
        assert_eq!(retained, "finalized");
    }
    Ok(())
}

#[tokio::test]
async fn deferred_finalization_accepts_unknown_legacy_parents() -> Result<()> {
    use crate::database::ingest::update_finalized_blocks_aux;
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow!("{e}"))?;
    let chain = ChainId::try_from(42_u64)?;
    let mut db =
        Database::new(&instance.db_url().to_owned().into(), chain, 64).await?;
    let pool = db.pool().await;
    for number in [99_i64, 100, 101] {
        sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id,block_hash,parent_hash,block_number,block_status) VALUES (42,$1,NULL,$2,$3)")
            .bind(B256::with_last_byte(number as u8).as_slice()).bind(number)
            .bind(if number == 100 { "pending" } else { "finalized" }).execute(&pool).await?;
    }
    update_finalized_blocks_aux(&mut db, 100, 0, |_| async {
        Ok(B256::with_last_byte(100))
    })
    .await;
    assert!(!is_halted(&pool).await?);
    let pending: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id=42 AND block_status='pending'")
        .fetch_one(&pool).await?;
    assert_eq!(pending, 0, "unknown parents must not wedge finalization");
    Ok(())
}

#[tokio::test]
async fn deferred_finalization_serializes_with_host_admission() -> Result<()> {
    use crate::database::ingest::update_finalized_blocks_aux;
    use fhevm_engine_common::chain_recovery::lock_history_tx;
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow!("{e}"))?;
    let chain = ChainId::try_from(42_u64)?;
    let mut db =
        Database::new(&instance.db_url().to_owned().into(), chain, 64).await?;
    let pool = db.pool().await;
    let candidate = B256::repeat_byte(10);
    sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id,block_hash,block_number,block_status) VALUES (42,$1,100,'pending')")
        .bind(candidate.as_slice()).execute(&pool).await?;
    let mut admission =
        begin_write_guarded(&pool, false, GcsRollbackPolicy::Continue)
            .await?
            .into_tx()
            .unwrap();
    lock_history_tx(&mut admission, ListenerSource::Host, false, 42).await?;
    let finalizer = tokio::spawn(async move {
        update_finalized_blocks_aux(&mut db, 100, 0, |_| async {
            Ok(candidate)
        })
        .await;
    });
    // Wait for actual lock contention, not an assumed task scheduling delay.
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let waiting: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM pg_locks WHERE locktype='advisory' AND NOT granted
                 AND database=(SELECT oid FROM pg_database WHERE datname=current_database())
                 AND classid=((hashtextextended('listener-history:host:BCS:42',0) >> 32) & 4294967295)::oid
                 AND objid=(hashtextextended('listener-history:host:BCS:42',0) & 4294967295)::oid)",
            ).fetch_one(&pool).await?;
            if waiting { break; }
            tokio::task::yield_now().await;
        }
        Ok::<_, sqlx::Error>(())
    }).await??;
    // Another admission establishes the successor while the finalizer waits.
    sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id,block_hash,parent_hash,block_number,block_status) VALUES (42,$1,$2,101,'finalized')")
        .bind(B256::repeat_byte(11).as_slice()).bind(B256::repeat_byte(20).as_slice())
        .execute(&mut *admission).await?;
    admission.commit().await?;
    tokio::time::timeout(Duration::from_secs(5), finalizer).await??;
    assert!(is_halted(&pool).await?);
    let status: String = sqlx::query_scalar("SELECT block_status FROM host_chain_blocks_valid WHERE chain_id=42 AND block_hash=$1")
        .bind(candidate.as_slice()).fetch_one(&pool).await?;
    assert_eq!(status, "pending");
    Ok(())
}

#[tokio::test]
async fn finalized_admission_preserves_parent_identity_across_batches(
) -> Result<()> {
    use crate::database::ingest::{ingest_block_logs, IngestOptions};
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let chain = ChainId::try_from(42_u64)?;
    let mut db = Database::new_with_stack_mode(
        &instance.db_url().to_owned().into(),
        chain,
        64,
        StackMode::new(false),
    )
    .await?;
    let options = IngestOptions {
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain: 0,
        is_protocol_config_listener: false,
        disable_synthetic_ops: true,
    };
    for number in [65, 66] {
        let block = BlockLogs {
            logs: vec![],
            catchup: true,
            finalized: true,
            summary: BlockSummary {
                number,
                hash: B256::with_last_byte(number as u8),
                parent_hash: B256::with_last_byte((number - 1) as u8),
                timestamp: number,
            },
        };
        ingest_block_logs(
            chain,
            &mut db,
            &block,
            &None,
            &None,
            &None,
            &None,
            &None,
            options.clone(),
        )
        .await?;
    }
    let inconsistent = BlockLogs {
        logs: vec![],
        catchup: true,
        finalized: true,
        summary: BlockSummary {
            number: 67,
            hash: B256::with_last_byte(67),
            parent_hash: B256::ZERO,
            timestamp: 67,
        },
    };
    assert!(ingest_block_logs(
        chain,
        &mut db,
        &inconsistent,
        &None,
        &None,
        &None,
        &None,
        &None,
        options
    )
    .await
    .is_err());
    let pool = db.pool().await;
    assert!(is_halted(&pool).await?);
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id = 42",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(count, 2, "invalid extension must not persist a block");
    Ok(())
}

#[tokio::test]
async fn unfinished_replay_is_adoptable_without_a_saved_cursor() -> Result<()> {
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    let old =
        begin_replay(&pool, ListenerSource::Host, false, 42, 3, 10).await?;
    let pending = fhevm_engine_common::chain_recovery::pending_replay(
        &pool,
        ListenerSource::Host,
        false,
        42,
    )
    .await?
    .unwrap();
    let resumed = begin_replay(
        &pool,
        ListenerSource::Host,
        false,
        42,
        pending.from_block,
        pending.to_block,
    )
    .await?;
    assert!(resumed.generation > old.generation);
    complete_replay(
        &pool,
        ListenerSource::Host,
        false,
        42,
        3,
        10,
        resumed.generation,
    )
    .await?;
    assert!(!readiness_blocked(&pool).await?);
    Ok(())
}

#[tokio::test]
async fn bounded_replay_preserves_backlog_drain_but_blocks_promotion(
) -> Result<()> {
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    let replay =
        begin_replay(&pool, ListenerSource::Host, false, 42, 2, 65).await?;
    assert!(readiness_blocked(&pool).await?);
    for gcs in [false, true] {
        begin_write_guarded(&pool, gcs, GcsRollbackPolicy::Continue)
            .await?
            .into_tx()
            .expect("background replay must not pause either stack")
            .rollback()
            .await?;
    }
    complete_replay(
        &pool,
        ListenerSource::Host,
        false,
        42,
        2,
        65,
        replay.generation,
    )
    .await?;
    assert!(!readiness_blocked(&pool).await?);
    begin_write_guarded(&pool, false, GcsRollbackPolicy::Continue)
        .await?
        .into_tx()
        .expect("derived writes stay available")
        .rollback()
        .await?;
    Ok(())
}

#[tokio::test]
async fn history_halt_drains_writers_and_survives_restarts_without_duplicates(
) -> Result<()> {
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    let writer = begin_write_guarded(&pool, false, GcsRollbackPolicy::Continue)
        .await?
        .into_tx()
        .expect("initial writer allowed");
    let halt_pool = pool.clone();
    let halt = tokio::spawn(async move {
        record_halt(
            &halt_pool,
            HistoryMismatch {
                source: ListenerSource::Host,
                gcs_mode: false,
                chain_id: 42,
                block_number: 65,
                expected_hash: Some(&[1; 32]),
                observed_hash: Some(&[2; 32]),
                reason: "test changed history",
            },
        )
        .await
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(
        !halt.is_finished(),
        "containment must drain previously admitted writes"
    );
    writer.commit().await?;
    tokio::time::timeout(Duration::from_secs(5), halt).await???;
    let restarted = sqlx::PgPool::connect(db.db_url()).await?;
    assert!(is_halted(&restarted).await?);
    assert!(readiness_blocked(&restarted).await?);
    for gcs in [false, true] {
        for policy in [GcsRollbackPolicy::Continue, GcsRollbackPolicy::Skip] {
            assert!(matches!(
                begin_write_guarded(&restarted, gcs, policy).await?,
                WriteGuard::Skip
            ));
        }
    }
    record_halt(
        &restarted,
        HistoryMismatch {
            source: ListenerSource::Host,
            gcs_mode: false,
            chain_id: 42,
            block_number: 65,
            expected_hash: Some(&[1; 32]),
            observed_hash: Some(&[2; 32]),
            reason: "repeated changed history",
        },
    )
    .await?;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM public.chain_recovery_halts WHERE resolved_at IS NULL")
        .fetch_one(&restarted).await?;
    assert_eq!(count, 1);
    assert!(complete_replay(
        &restarted,
        ListenerSource::Host,
        false,
        42,
        2,
        65,
        1
    )
    .await
    .is_err());
    Ok(())
}

#[tokio::test]
async fn changed_committed_host_header_is_contained_before_replay() -> Result<()>
{
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status)
                 VALUES (42, $1, 65, 'finalized')")
        .bind(vec![1_u8; 32]).execute(&pool).await?;
    sqlx::query("INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block) VALUES (42, 65)")
        .execute(&pool).await?;
    let mut header = BlockSummary {
        number: 65,
        hash: B256::repeat_byte(1),
        parent_hash: B256::ZERO,
        timestamp: 100,
    };
    validate_committed_header(
        &pool,
        ChainId::try_from(42_u64)?,
        &header,
        false,
    )
    .await?;
    assert!(!is_halted(&pool).await?);
    header.hash = B256::repeat_byte(2);
    assert!(validate_committed_header(
        &pool,
        ChainId::try_from(42_u64)?,
        &header,
        false
    )
    .await
    .is_err());
    assert!(is_halted(&pool).await?);
    let (hash, cursor): (Vec<u8>, i64) = sqlx::query_as(
        "SELECT block_hash, last_caught_up_block FROM host_chain_blocks_valid
         JOIN host_listener_poller_state USING (chain_id) WHERE chain_id = 42",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(hash, vec![1; 32]);
    assert_eq!(cursor, 65);
    Ok(())
}

#[tokio::test]
async fn overlapping_restart_adopts_pending_range_and_rejects_stale_completion(
) -> Result<()> {
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    let first =
        begin_replay(&pool, ListenerSource::Host, false, 42, 37, 100).await?;
    let second =
        begin_replay(&pool, ListenerSource::Host, false, 42, 38, 101).await?;
    assert_eq!((second.from_block, second.to_block), (37, 101));
    let stale = complete_replay(
        &pool,
        ListenerSource::Host,
        false,
        42,
        first.from_block,
        first.to_block,
        first.generation,
    )
    .await?;
    assert!(stale.pending);
    assert!(readiness_blocked(&pool).await?);
    let completed = complete_replay(
        &pool,
        ListenerSource::Host,
        false,
        42,
        second.from_block,
        second.to_block,
        second.generation,
    )
    .await?;
    assert!(!completed.pending);
    assert!(!readiness_blocked(&pool).await?);
    let fresh =
        begin_replay(&pool, ListenerSource::Host, false, 42, 102, 165).await?;
    assert_eq!((fresh.from_block, fresh.to_block), (102, 165));
    let restarted = sqlx::PgPool::connect(db.db_url()).await?;
    let resumed =
        begin_replay(&restarted, ListenerSource::Host, false, 42, 104, 167)
            .await?;
    assert_eq!((resumed.from_block, resumed.to_block), (102, 167));
    assert!(begin_replay(
        &restarted,
        ListenerSource::Host,
        false,
        42,
        1,
        10001
    )
    .await
    .is_err());
    assert!(readiness_blocked(&pool).await?);
    Ok(())
}

#[tokio::test]
async fn older_replica_cannot_release_a_new_generation_of_the_same_range(
) -> Result<()> {
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    let old =
        begin_replay(&pool, ListenerSource::Host, false, 42, 1, 64).await?;
    let new =
        begin_replay(&pool, ListenerSource::Host, false, 42, 1, 64).await?;
    assert!(new.generation > old.generation);
    let stale = complete_replay(
        &pool,
        ListenerSource::Host,
        false,
        42,
        1,
        64,
        old.generation,
    )
    .await?;
    assert!(stale.pending);
    assert_eq!(stale.generation, new.generation);
    assert!(readiness_blocked(&pool).await?);
    let done = complete_replay(
        &pool,
        ListenerSource::Host,
        false,
        42,
        1,
        64,
        new.generation,
    )
    .await?;
    assert!(!done.pending);
    assert!(!readiness_blocked(&pool).await?);
    Ok(())
}

async fn green_host_pool(
    pool: &sqlx::PgPool,
    url: &str,
) -> Result<sqlx::PgPool> {
    use fhevm_engine_common::database::{GCS_SCHEMA_QUOTED, GCS_SEARCH_PATH};
    sqlx::query(&format!("CREATE SCHEMA {GCS_SCHEMA_QUOTED}"))
        .execute(pool)
        .await?;
    sqlx::query(&format!(
        "CREATE TABLE {GCS_SCHEMA_QUOTED}.host_listener_poller_state
         (LIKE public.host_listener_poller_state INCLUDING ALL)"
    ))
    .execute(pool)
    .await?;
    let options = url
        .parse::<sqlx::postgres::PgConnectOptions>()?
        .options([("search_path", GCS_SEARCH_PATH)]);
    Ok(sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect_with(options)
        .await?)
}

#[tokio::test]
async fn host_startup_queued_behind_promotion_replays_the_live_cursor(
) -> Result<()> {
    use fhevm_engine_common::database::GCS_SCHEMA_QUOTED;
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    let green = green_host_pool(&pool, db.db_url()).await?;
    sqlx::query("INSERT INTO public.host_listener_poller_state (chain_id, last_caught_up_block) VALUES (42, 900)")
        .execute(&pool).await?;
    sqlx::query("INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block) VALUES (42, 200)")
        .execute(&green).await?;
    let mut cutover = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *cutover)
        .await?;
    let queued_pool = green.clone();
    let startup = tokio::spawn(async move {
        prepare_host_replay(&queued_pool, true, 42, 64, None).await
    });
    // Observe an actual queued shared fence before making Green the live schema.
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let waiting: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM pg_locks
                 WHERE locktype = 'advisory' AND NOT granted
                   AND database = (SELECT oid FROM pg_database WHERE datname = current_database())
                   AND classid = $1::bigint::oid AND objid = $2::bigint::oid)",
            )
            .bind(CUTOVER_LOCK_ID >> 32)
            .bind(CUTOVER_LOCK_ID & 0xffff_ffff)
            .fetch_one(&pool)
            .await?;
            if waiting {
                break;
            }
            tokio::task::yield_now().await;
        }
        Ok::<_, sqlx::Error>(())
    })
    .await??;
    // Controller tests cover the full cutover; this fixture exercises its
    // cursor merge and namespace transition with the startup hint still Green.
    sqlx::query(&format!(
        "INSERT INTO public.host_listener_poller_state
         SELECT * FROM {GCS_SCHEMA_QUOTED}.host_listener_poller_state
         ON CONFLICT (chain_id) DO UPDATE
         SET last_caught_up_block = EXCLUDED.last_caught_up_block"
    ))
    .execute(&mut *cutover)
    .await?;
    sqlx::query(&format!(
        "ALTER SCHEMA {GCS_SCHEMA_QUOTED} RENAME TO \"host-before-promotion\""
    ))
    .execute(&mut *cutover)
    .await?;
    cutover.commit().await?;
    let HostReplayStartup::Ready { cursor, replay } =
        tokio::time::timeout(Duration::from_secs(5), startup).await???
    else {
        panic!("promoted cursor must already exist");
    };
    assert_eq!(
        cursor, 200,
        "replay uses the promoted cursor, not Blue's old tip"
    );
    let first = replay.expect("a promoted positive cursor needs replay");
    assert_eq!(
        (first.progress.from_block, first.progress.to_block),
        (137, 200)
    );
    let roles: Vec<String> = sqlx::query_scalar(
        "SELECT stack_role FROM public.listener_replay_state WHERE source = 'host' AND chain_id = 42",
    )
    .fetch_all(&pool)
    .await?;
    assert_eq!(roles, vec!["BCS".to_owned()]);

    // A replacement now starts in Blue mode and must adopt the same obligation.
    let restarted = sqlx::PgPool::connect(db.db_url()).await?;
    let HostReplayStartup::Ready { replay, .. } =
        prepare_host_replay(&restarted, false, 42, 64, None).await?
    else {
        panic!("restart must retain the promoted cursor");
    };
    let replacement = replay.expect("unfinished replay must be adopted");
    assert!(replacement.progress.generation > first.progress.generation);
    assert!(!first.complete(&green, 200).await?);
    assert!(readiness_blocked(&pool).await?);
    assert!(!replacement.complete(&restarted, 200).await?);
    assert!(!readiness_blocked(&pool).await?);
    Ok(())
}

#[tokio::test]
async fn host_startup_seed_resolution_preserves_a_concurrent_cursor(
) -> Result<()> {
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    assert!(matches!(
        prepare_host_replay(&pool, false, 42, 64, None).await?,
        HostReplayStartup::NeedsSeed
    ));
    // While this caller resolves its seed outside the fence, another startup
    // initializes the cursor. A positive fresh seed is not a restart replay.
    let HostReplayStartup::Ready { cursor, replay } =
        prepare_host_replay(&pool, false, 42, 64, Some(7)).await?
    else {
        panic!("resolved seed must initialize the cursor");
    };
    assert_eq!(cursor, 7);
    assert!(replay.is_none());
    assert!(!readiness_blocked(&pool).await?);
    let HostReplayStartup::Ready { cursor, replay } =
        prepare_host_replay(&pool, false, 42, 64, Some(999)).await?
    else {
        panic!("the concurrent cursor must win over the delayed seed");
    };
    assert_eq!(cursor, 7);
    let replay = replay.expect("the existing positive cursor must replay");
    assert_eq!(
        (replay.progress.from_block, replay.progress.to_block),
        (1, 7)
    );
    let saved: i64 = sqlx::query_scalar(
        "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = 42",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(saved, 7);
    assert!(!replay.complete(&pool, 7).await?);

    for seed in [Some(0), None, Some(999)] {
        let HostReplayStartup::Ready { cursor, replay } =
            prepare_host_replay(&pool, false, 43, 64, seed).await?
        else {
            panic!("genesis cursor must remain initialized");
        };
        assert_eq!(cursor, 0);
        assert!(replay.is_none(), "genesis has no committed block to replay");
    }
    assert!(!readiness_blocked(&pool).await?);
    Ok(())
}

#[tokio::test]
async fn host_startup_adopts_missing_cursor_and_fences_reset_generations(
) -> Result<()> {
    use fhevm_engine_common::database::GCS_SCHEMA_QUOTED;
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    let green = green_host_pool(&pool, db.db_url()).await?;
    let old =
        begin_replay(&pool, ListenerSource::Host, true, 42, 3, 10).await?;
    assert!(matches!(
        prepare_host_replay(&green, true, 42, 64, None).await?,
        HostReplayStartup::NeedsSeed
    ));
    let HostReplayStartup::Ready { cursor, replay } =
        prepare_host_replay(&green, true, 42, 64, Some(0)).await?
    else {
        panic!("restored genesis cursor must adopt its pending obligation");
    };
    assert_eq!(cursor, 0);
    let adopted = replay.expect("missing cursor cannot discard pending replay");
    assert_eq!(
        (adopted.progress.from_block, adopted.progress.to_block),
        (3, 10)
    );
    assert!(adopted.progress.generation > old.generation);
    sqlx::query("INSERT INTO public.host_listener_poller_state (chain_id, last_caught_up_block) VALUES (99, 20)")
        .execute(&pool).await?;
    let HostReplayStartup::Ready { replay, .. } =
        prepare_host_replay(&pool, false, 99, 64, None).await?
    else {
        panic!("Blue cursor is present");
    };
    let blue = replay.expect("Blue obligation must remain independent");
    let mut reset = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *reset)
        .await?;
    sqlx::query(&format!("DROP SCHEMA {GCS_SCHEMA_QUOTED} CASCADE"))
        .execute(&mut *reset)
        .await?;
    sqlx::query(&format!("CREATE SCHEMA {GCS_SCHEMA_QUOTED}"))
        .execute(&mut *reset)
        .await?;
    sqlx::query(&format!(
        "CREATE TABLE {GCS_SCHEMA_QUOTED}.host_listener_poller_state
         (LIKE public.host_listener_poller_state INCLUDING ALL)"
    ))
    .execute(&mut *reset)
    .await?;
    reset_green_replay_tx(&mut reset).await?;
    reset.commit().await?;
    assert!(!adopted.complete(&green, 10).await?);
    let HostReplayStartup::Ready { replay, .. } =
        prepare_host_replay(&green, true, 42, 64, Some(10)).await?
    else {
        panic!("reset schema can be initialized again");
    };
    assert!(replay.is_none(), "reset invalidates the old pending range");
    let HostReplayStartup::Ready { replay, .. } =
        prepare_host_replay(&green, true, 42, 64, None).await?
    else {
        panic!("replacement Green cursor is present");
    };
    let fresh = replay.expect("restart creates a fresh replay generation");
    assert!(fresh.progress.generation > adopted.progress.generation);
    assert!(!adopted.complete(&green, 10).await?);
    let pending: Vec<(String, i64)> = sqlx::query_as(
        "SELECT stack_role, chain_id FROM public.listener_replay_state
         WHERE pending ORDER BY stack_role, chain_id",
    )
    .fetch_all(&pool)
    .await?;
    assert_eq!(
        pending,
        vec![("BCS".to_owned(), 99), ("GCS".to_owned(), 42)]
    );
    assert!(!fresh.complete(&green, 10).await?);
    assert!(
        readiness_blocked(&pool).await?,
        "Blue's gate must survive reset"
    );
    assert!(!blue.complete(&pool, 20).await?);
    assert!(!readiness_blocked(&pool).await?);
    Ok(())
}

#[tokio::test]
async fn host_admission_queued_at_promotion_uses_live_replica_lock(
) -> Result<()> {
    use crate::database::ingest::{ingest_block_logs, IngestOptions};
    use fhevm_engine_common::chain_recovery::lock_history_tx;
    use fhevm_engine_common::database::GCS_SCHEMA_QUOTED;
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(instance.db_url()).await?;
    let _green = green_host_pool(&pool, instance.db_url()).await?;
    let chain = ChainId::try_from(42_u64)?;
    let mut db = Database::new_with_stack_mode(
        &instance.db_url().to_owned().into(),
        chain,
        64,
        StackMode::new(true),
    )
    .await?;
    let options = IngestOptions {
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain: 0,
        is_protocol_config_listener: false,
        disable_synthetic_ops: true,
    };
    let mut cutover = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *cutover)
        .await?;
    let cutover_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *cutover)
        .await?;
    let queued_options = options.clone();
    let admission = tokio::spawn(async move {
        let block = BlockLogs {
            logs: vec![],
            catchup: true,
            finalized: true,
            summary: BlockSummary {
                number: 65,
                hash: B256::with_last_byte(65),
                parent_hash: B256::with_last_byte(64),
                timestamp: 65,
            },
        };
        ingest_block_logs(
            chain,
            &mut db,
            &block,
            &None,
            &None,
            &None,
            &None,
            &None,
            queued_options,
        )
        .await
    });
    async fn wait_for_waiter(pool: &sqlx::PgPool, blocker: i32) -> Result<()> {
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let waiting: bool = sqlx::query_scalar(
                    "SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE $1 = ANY(pg_blocking_pids(pid)))",
                ).bind(blocker).fetch_one(pool).await?;
                if waiting { return Ok::<_, sqlx::Error>(()); }
                tokio::task::yield_now().await;
            }
        }).await??;
        Ok(())
    }
    wait_for_waiter(&pool, cutover_pid).await?;
    sqlx::query(&format!(
        "ALTER SCHEMA {GCS_SCHEMA_QUOTED} RENAME TO host_previous_green"
    ))
    .execute(&mut *cutover)
    .await?;
    // Represent the admission lock used by an already-reconciled live replica.
    let mut live_replica = pool.begin().await?;
    lock_history_tx(&mut live_replica, ListenerSource::Host, false, 42).await?;
    let live_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *live_replica)
        .await?;
    cutover.commit().await?;
    wait_for_waiter(&pool, live_pid).await?;
    assert!(
        !admission.is_finished(),
        "stale Green mode must not bypass the live lock"
    );
    live_replica.rollback().await?;
    tokio::time::timeout(Duration::from_secs(5), admission).await???;

    let mut live_db =
        Database::new(&instance.db_url().to_owned().into(), chain, 64).await?;
    let conflicting = BlockLogs {
        logs: vec![],
        catchup: true,
        finalized: true,
        summary: BlockSummary {
            number: 65,
            hash: B256::with_last_byte(99),
            parent_hash: B256::with_last_byte(64),
            timestamp: 65,
        },
    };
    assert!(ingest_block_logs(
        chain,
        &mut live_db,
        &conflicting,
        &None,
        &None,
        &None,
        &None,
        &None,
        options
    )
    .await
    .is_err());
    let roles: Vec<String> = sqlx::query_scalar(
        "SELECT stack_role FROM public.chain_recovery_halts WHERE resolved_at IS NULL",
    ).fetch_all(&pool).await?;
    assert_eq!(roles, vec!["BCS"]);
    Ok(())
}

#[tokio::test]
async fn legacy_pending_fork_below_cursor_does_not_false_halt() -> Result<()> {
    use crate::database::ingest::{ingest_block_logs, IngestOptions};
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow!("{e}"))?;
    let chain = ChainId::try_from(42_u64)?;
    let mut db =
        Database::new(&instance.db_url().to_owned().into(), chain, 64).await?;
    let pool = db.pool().await;
    sqlx::query("INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block) VALUES (42,65)").execute(&pool).await?;
    sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status) VALUES (42,$1,65,'pending')")
        .bind(vec![99_u8;32]).execute(&pool).await?;
    let block = BlockLogs {
        logs: vec![],
        catchup: true,
        finalized: true,
        summary: BlockSummary {
            number: 65,
            hash: B256::repeat_byte(1),
            parent_hash: B256::ZERO,
            timestamp: 65,
        },
    };
    validate_committed_header(&pool, chain, &block.summary, false).await?;
    ingest_block_logs(
        chain,
        &mut db,
        &block,
        &None,
        &None,
        &None,
        &None,
        &None,
        IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: false,
            dependent_ops_max_per_chain: 0,
            is_protocol_config_listener: false,
            disable_synthetic_ops: true,
        },
    )
    .await?;
    assert!(!is_halted(&pool).await?);
    // The canonical row is now explicitly finalized; subsequent replacement
    // must still be contained even though the legacy pending sibling remains.
    let mut changed = block.summary;
    changed.hash = B256::repeat_byte(2);
    assert!(validate_committed_header(&pool, chain, &changed, false)
        .await
        .is_err());
    assert!(is_halted(&pool).await?);
    Ok(())
}
