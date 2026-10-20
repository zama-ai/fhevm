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

fn pause_test_config(
    database_url: DatabaseURL,
    anvil: &alloy::node_bindings::AnvilInstance,
    dependent_ops_max_per_chain: u32,
) -> PollerConfig {
    PollerConfig {
        url: anvil.endpoint(),
        acl_address: Address::with_last_byte(1),
        tfhe_address: Address::with_last_byte(2),
        kms_generation_address: None,
        protocol_config_address: None,
        confidential_bridge_address: None,
        database_url,
        finality_lag: 20,
        batch_size: 100,
        poll_interval: Duration::from_millis(100),
        retry_interval: Duration::from_millis(10),
        service_name: "retired-poller-test".into(),
        max_http_retries: 0,
        rpc_compute_units_per_second: 1000,
        health_port: 0,
        seed_start_block: None,
        replay_window: 64,
        dependence_cache_size: 64,
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain,
        gcs_mode: false,
        disable_synthetic_ops: true,
        canonical_protocol_config_chain_id: None,
    }
}

async fn assert_retired_poller_pauses(
    dependent_ops_max_per_chain: u32,
    retire_during_startup: bool,
    has_paused: impl Fn() -> bool,
) -> Result<()> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(instance.db_url()).await?;
    sqlx::query("INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block) VALUES (42, 7)")
        .execute(&pool).await?;
    let mut cutover = if retire_during_startup {
        let mut tx = pool.begin().await?;
        sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(CUTOVER_LOCK_ID)
            .execute(&mut *tx)
            .await?;
        Some(tx)
    } else {
        sqlx::query("UPDATE versioning SET consensus_version = 9999")
            .execute(&pool)
            .await?;
        None
    };
    let anvil = alloy::node_bindings::Anvil::new().chain_id(42).spawn();
    let config = pause_test_config(
        instance.db_url.clone(),
        &anvil,
        dependent_ops_max_per_chain,
    );
    let poller = run_poller(config);
    tokio::pin!(poller);
    if let Some(mut tx) = cutover.take() {
        // Retire while startup's promotion or replay transaction waits behind
        // the fence, after its initial retirement check already passed.
        tokio::select! {
            result = &mut poller => panic!("poller exited before cutover: {result:?}"),
            result = tokio::time::timeout(Duration::from_secs(10), async {
                loop {
                    let waiting: bool = sqlx::query_scalar(
                        "SELECT EXISTS (SELECT 1 FROM pg_stat_activity
                         WHERE datname = current_database() AND wait_event = 'advisory')",
                    ).fetch_one(&pool).await?;
                    if waiting { break; }
                    sleep(Duration::from_millis(10)).await;
                }
                Ok::<_, sqlx::Error>(())
            }) => result??,
        }
        sqlx::query("UPDATE versioning SET consensus_version = 9999")
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
    }
    tokio::select! {
        result = &mut poller => panic!("retired poller exited: {result:?}"),
        result = tokio::time::timeout(Duration::from_secs(10), async {
            while !has_paused() {
                sleep(Duration::from_millis(10)).await;
            }
        }) => result?,
    }
    assert!(
        tokio::time::timeout(Duration::from_millis(100), &mut poller)
            .await
            .is_err()
    );
    let replays: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM public.listener_replay_state")
            .fetch_one(&pool)
            .await?;
    assert_eq!(replays, 0, "retired startup must not create replay state");
    Ok(())
}

#[tokio::test]
#[tracing_test::traced_test]
async fn retired_poller_pauses_before_fast_lane_promotion() -> Result<()> {
    assert_retired_poller_pauses(0, false, || {
        logs_contain("Listener stack retired; pausing ingestion")
    })
    .await
}

#[tokio::test]
#[tracing_test::traced_test]
async fn retired_poller_pauses_before_replay_startup() -> Result<()> {
    assert_retired_poller_pauses(100, false, || {
        logs_contain("Listener stack retired; pausing ingestion")
    })
    .await
}

#[tokio::test]
#[tracing_test::traced_test]
async fn retired_poller_pauses_when_cutover_races_promotion() -> Result<()> {
    assert_retired_poller_pauses(0, true, || {
        logs_contain("Listener stack retired; pausing ingestion")
    })
    .await
}

#[tokio::test]
#[tracing_test::traced_test]
async fn retired_poller_pauses_when_cutover_races_replay_startup() -> Result<()>
{
    assert_retired_poller_pauses(100, true, || {
        logs_contain("Listener stack retired; pausing ingestion")
    })
    .await
}

#[tokio::test]
async fn delayed_live_forks_do_not_halt_or_insert_stale_work() -> Result<()> {
    use crate::contracts::TfheContract;
    use crate::database::ingest::{ingest_block_logs, IngestOptions};
    use alloy::primitives::{Address, U256};
    use alloy::sol_types::SolEvent;

    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let options = IngestOptions {
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain: 1,
        is_protocol_config_listener: false,
        disable_synthetic_ops: true,
    };
    // Cover a retained orphan, a previously unseen stale parent, and a child
    // of an orphan. Live/final queues and pollers can finish in any order.
    for (chain_number, stale_height) in [(42_u64, 100), (43, 99), (44, 101)] {
        let chain = ChainId::try_from(chain_number)?;
        let mut db = Database::new(&instance.db_url, chain, 64).await?;
        let pool = db.pool().await;
        let mut canonical = BlockLogs {
            logs: vec![],
            catchup: false,
            finalized: false,
            summary: BlockSummary {
                number: 100,
                hash: B256::repeat_byte(1),
                parent_hash: B256::repeat_byte(99),
                timestamp: 100,
            },
        };
        if stale_height == 100 {
            ingest_block_logs(
                chain,
                &mut db,
                &canonical,
                &None,
                &None,
                &None,
                &None,
                &None,
                options.clone(),
            )
            .await?;
        }
        canonical.summary.hash = B256::repeat_byte(2);
        canonical.finalized = true;
        ingest_block_logs(
            chain,
            &mut db,
            &canonical,
            &None,
            &None,
            &None,
            &None,
            &None,
            options.clone(),
        )
        .await?;
        let before: Vec<(Vec<u8>, String)> = sqlx::query_as(
            "SELECT block_hash, block_status FROM host_chain_blocks_valid
             WHERE chain_id = $1 ORDER BY block_hash",
        )
        .bind(chain.as_i64())
        .fetch_all(&pool)
        .await?;

        let mut handle = B256::repeat_byte(chain_number as u8);
        handle.0[21] = 0xff;
        handle.0[22..30].copy_from_slice(&chain_number.to_be_bytes());
        handle.0[30] = 4;
        handle.0[31] = 0;
        let event = TfheContract::TrivialEncrypt {
            caller: Address::ZERO,
            pt: U256::from(7),
            toType: 4,
            result: handle,
        };
        let stale_hash =
            B256::repeat_byte(if stale_height == 101 { 3 } else { 1 });
        let mut stale = BlockLogs {
            logs: vec![Log {
                inner: alloy::primitives::Log {
                    address: Address::ZERO,
                    data: event.encode_log_data(),
                },
                block_hash: Some(stale_hash),
                block_number: Some(stale_height),
                transaction_hash: Some(B256::repeat_byte(7)),
                log_index: Some(0),
                ..Default::default()
            }],
            catchup: false,
            finalized: false,
            summary: BlockSummary {
                number: stale_height,
                hash: stale_hash,
                parent_hash: B256::repeat_byte(match stale_height {
                    99 => 98,
                    100 => 99,
                    _ => 1,
                }),
                timestamp: stale_height,
            },
        };
        ingest_block_logs(
            chain,
            &mut db,
            &stale,
            &None,
            &None,
            &None,
            &None,
            &None,
            options.clone(),
        )
        .await?;
        assert!(!is_halted(&pool).await?);
        let after: Vec<(Vec<u8>, String)> = sqlx::query_as(
            "SELECT block_hash, block_status FROM host_chain_blocks_valid
             WHERE chain_id = $1 ORDER BY block_hash",
        )
        .bind(chain.as_i64())
        .fetch_all(&pool)
        .await?;
        assert_eq!(after, before, "stale delivery must not change history");
        let work: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM computations WHERE output_handle = $1",
        )
        .bind(handle.as_slice())
        .fetch_one(&pool)
        .await?;
        assert_eq!(work, 0, "stale delivery must not insert computations");

        if stale_height == 101 {
            // A finalized/canonical claim is different: retain fail-closed
            // containment for the same contradiction once it is authoritative.
            stale.finalized = true;
            assert!(ingest_block_logs(
                chain,
                &mut db,
                &stale,
                &None,
                &None,
                &None,
                &None,
                &None,
                options.clone()
            )
            .await
            .is_err());
            assert!(is_halted(&pool).await?);
        }
        pool.close().await;
    }
    Ok(())
}

#[tokio::test]
async fn poller_recovers_missing_canonical_height_before_restart_overlap(
) -> Result<()> {
    use alloy::providers::{ext::AnvilApi, Provider};

    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(instance.db_url()).await?;
    let anvil = alloy::node_bindings::Anvil::new().chain_id(42).spawn();
    let provider = ProviderBuilder::new().connect_http(anvil.endpoint_url());
    provider.anvil_mine(Some(10), None).await?;
    for number in [1_u64, 10] {
        let header = provider
            .get_block_by_number(number.into())
            .await?
            .unwrap()
            .header;
        sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id, block_hash, parent_hash, block_number, block_status) VALUES (42, $1, $2, $3, 'finalized')")
            .bind(header.hash.as_slice()).bind(header.parent_hash.as_slice())
            .bind(number as i64).execute(&pool).await?;
    }
    sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status) VALUES (42, $1, 2, 'pending')")
        .bind(B256::repeat_byte(99).as_slice()).execute(&pool).await?;
    sqlx::query("INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block) VALUES (42, 10)")
        .execute(&pool).await?;
    let config = PollerConfig {
        url: anvil.endpoint(),
        acl_address: Address::with_last_byte(1),
        tfhe_address: Address::with_last_byte(2),
        kms_generation_address: None,
        protocol_config_address: None,
        confidential_bridge_address: None,
        database_url: instance.db_url.clone(),
        finality_lag: 0,
        batch_size: 100,
        poll_interval: Duration::from_millis(10),
        retry_interval: Duration::from_millis(10),
        service_name: "missing-canonical-poller-test".into(),
        max_http_retries: 0,
        rpc_compute_units_per_second: 1000,
        health_port: 0,
        seed_start_block: None,
        replay_window: 1,
        dependence_cache_size: 64,
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain: 0,
        gcs_mode: false,
        disable_synthetic_ops: true,
        canonical_protocol_config_chain_id: None,
    };
    let poller = run_poller(config);
    tokio::pin!(poller);
    tokio::select! {
        result = &mut poller => panic!("poller exited during repair: {result:?}"),
        result = tokio::time::timeout(Duration::from_secs(15), async {
            loop {
                let anchor: i64 = sqlx::query_scalar("SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=42")
                    .fetch_one(&pool).await?;
                assert!(anchor >= 10, "historical repair must not rewind the saved anchor");
                let finalized: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id=42 AND block_status='finalized'")
                    .fetch_one(&pool).await?;
                if finalized == 10 && !readiness_blocked(&pool).await? { break; }
                sleep(Duration::from_millis(20)).await;
            }
            Ok::<_, anyhow::Error>(())
        }) => result??,
    }
    let old_status: String = sqlx::query_scalar("SELECT block_status FROM host_chain_blocks_valid WHERE chain_id=42 AND block_hash=$1")
        .bind(B256::repeat_byte(99).as_slice()).fetch_one(&pool).await?;
    assert_eq!(old_status, "orphaned");
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
async fn idle_poller_adopts_replay_started_by_a_dead_replica() -> Result<()> {
    use alloy::providers::ext::AnvilApi;
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(instance.db_url()).await?;
    let anvil = alloy::node_bindings::Anvil::new().chain_id(42).spawn();
    let provider = ProviderBuilder::new().connect_http(anvil.endpoint_url());
    provider.anvil_mine(Some(20), None).await?;
    sqlx::query("INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block) VALUES (42, 10)")
        .execute(&pool).await?;
    let mut config = pause_test_config(instance.db_url.clone(), &anvil, 1);
    config.finality_lag = 0;
    config.replay_window = 1;
    let poller = run_poller(config);
    tokio::pin!(poller);
    tokio::select! {
        result = &mut poller => panic!("poller exited: {result:?}"),
        result = tokio::time::timeout(Duration::from_secs(15), async {
            loop {
                let cursor: i64 = sqlx::query_scalar(
                    "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=42"
                ).fetch_one(&pool).await?;
                if cursor == 20 && !readiness_blocked(&pool).await? { break; }
                sleep(Duration::from_millis(20)).await;
            }
            // Simulate a second startup that persists a generation and exits
            // without reading any blocks. No new host block will wake the survivor.
            let HostReplayStartup::Ready { replay: Some(_), .. } =
                prepare_host_replay(&pool, false, 42, 5, None).await?
            else { panic!("second startup must create replay"); };
            assert!(readiness_blocked(&pool).await?);
            while readiness_blocked(&pool).await? {
                sleep(Duration::from_millis(20)).await;
            }
            Ok::<_, anyhow::Error>(())
        }) => result??,
    }
    Ok(())
}

#[tokio::test]
async fn missing_canonical_blocks_recover_across_gaps_in_observed_forks(
) -> Result<()> {
    use crate::contracts::TfheContract::TrivialEncrypt;
    use alloy::primitives::{Log as PrimitiveLog, U256};
    use alloy::sol_types::SolEvent;

    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    for with_event in [false, true] {
        let chain = ChainId::try_from(42_u64 + u64::from(with_event))?;
        let mut db = Database::new(&instance.db_url, chain, 64).await?;
        let pool = db.pool().await;
        // The listener missed old height 102, so orphan marking at an earlier
        // fork root cannot walk into the disconnected old 103 -> 104 segment.
        sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id, block_hash, parent_hash, block_number, block_status) VALUES ($1, $2, NULL, 100, 'finalized'), ($1, $3, NULL, 102, 'pending'), ($1, $4, $5, 103, 'pending'), ($1, $6, $4, 104, 'pending'), ($1, $7, $8, 104, 'pending')")
            .bind(chain.as_i64())
            .bind(B256::repeat_byte(1).as_slice())
            .bind(B256::repeat_byte(2).as_slice())
            .bind(B256::repeat_byte(13).as_slice())
            .bind(B256::repeat_byte(12).as_slice())
            .bind(B256::repeat_byte(14).as_slice())
            .bind(B256::repeat_byte(4).as_slice())
            .bind(B256::repeat_byte(3).as_slice())
            .execute(&pool).await?;
        let canonical_hash = |number| async move {
            Ok(B256::repeat_byte(match number {
                100 => 1,
                102 => 2,
                103 => 3,
                104 => 4,
                _ => panic!("unexpected height {number}"),
            }))
        };
        let result =
            update_finalized_blocks_aux(&mut db, 120, 10, canonical_hash).await;
        assert_eq!(result.missing_canonical_block, Some(103));
        let finalized: Vec<i64> = sqlx::query_scalar("SELECT block_number FROM host_chain_blocks_valid WHERE chain_id=$1 AND block_status='finalized' ORDER BY block_number")
            .bind(chain.as_i64()).fetch_all(&pool).await?;
        assert_eq!(finalized, vec![100, 102], "commit the safe prefix only");
        let canonical_inserted: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM host_chain_blocks_valid WHERE chain_id=$1 AND block_hash=$2)")
            .bind(chain.as_i64()).bind(B256::repeat_byte(3).as_slice())
            .fetch_one(&pool).await?;
        assert!(!canonical_inserted, "a bare RPC hash must not create a row");

        let mut handle = [0_u8; 32];
        handle[0] = 7;
        handle[21] = 0xff;
        handle[22..30].copy_from_slice(&chain.as_u64().to_be_bytes());
        handle[30] = 5;
        let logs = if with_event {
            vec![Log {
                inner: PrimitiveLog {
                    address: Address::ZERO,
                    data: TrivialEncrypt {
                        caller: Address::ZERO,
                        pt: U256::from(7),
                        toType: 5,
                        result: handle.into(),
                    }
                    .encode_log_data(),
                },
                block_hash: Some(B256::repeat_byte(3)),
                block_number: Some(103),
                transaction_hash: Some(B256::repeat_byte(30)),
                log_index: Some(0),
                ..Default::default()
            }]
        } else {
            vec![]
        };
        ingest_block_logs(
            chain,
            &mut db,
            &BlockLogs {
                logs,
                summary: BlockSummary {
                    number: 103,
                    hash: B256::repeat_byte(3),
                    parent_hash: B256::repeat_byte(2),
                    timestamp: 103,
                },
                catchup: true,
                finalized: false,
            },
            &None,
            &Some(Address::ZERO),
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
        let result =
            update_finalized_blocks_aux(&mut db, 120, 10, canonical_hash).await;
        assert_eq!(result.missing_canonical_block, None);
        let statuses: Vec<(Vec<u8>, String)> = sqlx::query_as("SELECT block_hash, block_status FROM host_chain_blocks_valid WHERE chain_id=$1 AND block_number >= 103 ORDER BY block_hash")
            .bind(chain.as_i64()).fetch_all(&pool).await?;
        assert_eq!(
            statuses,
            vec![
                (B256::repeat_byte(3).to_vec(), "finalized".into()),
                (B256::repeat_byte(4).to_vec(), "finalized".into()),
                (B256::repeat_byte(13).to_vec(), "orphaned".into()),
                (B256::repeat_byte(14).to_vec(), "orphaned".into()),
            ]
        );
        let computations: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM computations WHERE host_chain_id=$1",
        )
        .bind(chain.as_i64())
        .fetch_one(&pool)
        .await?;
        assert_eq!(computations, i64::from(with_event));
        assert!(!is_halted(&pool).await?);
        pool.close().await;
    }
    Ok(())
}

#[tokio::test]
async fn deferred_finalization_checks_the_committed_tip_without_pending_work(
) -> Result<()> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let chain = ChainId::try_from(42_u64)?;
    let mut db = Database::new(&instance.db_url, chain, 64).await?;
    let pool = db.pool().await;
    let canonical = B256::repeat_byte(2);
    sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status) VALUES (42, $1, 100, 'finalized')")
        .bind(canonical.as_slice()).execute(&pool).await?;
    let stale = BlockLogs {
        logs: vec![],
        catchup: false,
        finalized: false,
        summary: BlockSummary {
            number: 100,
            hash: B256::repeat_byte(1),
            parent_hash: B256::repeat_byte(99),
            timestamp: 100,
        },
    };
    ingest_block_logs(
        chain,
        &mut db,
        &stale,
        &None,
        &None,
        &None,
        &None,
        &None,
        IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: false,
            dependent_ops_max_per_chain: 1,
            is_protocol_config_listener: false,
            disable_synthetic_ops: true,
        },
    )
    .await?;
    let mut requested = vec![];
    update_finalized_blocks_aux(&mut db, 120, 20, |number| {
        requested.push(number);
        async move { Ok(canonical) }
    })
    .await;
    assert_eq!(requested, vec![100]);
    assert!(
        !is_halted(&pool).await?,
        "late live data is not a canonical change"
    );

    // An independent canonical read now confirms a real history change. There
    // are no pending rows, including none from the discarded stale delivery.
    let pending: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM host_chain_blocks_valid WHERE block_status = 'pending'")
        .fetch_one(&pool).await?;
    assert_eq!(pending, 0);
    update_finalized_blocks_aux(&mut db, 120, 20, |_| async {
        Ok(B256::repeat_byte(3))
    })
    .await;
    assert!(is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
async fn deferred_finalization_waits_for_an_unavailable_committed_tip(
) -> Result<()> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let chain = ChainId::try_from(42_u64)?;
    let mut db = Database::new(&instance.db_url, chain, 64).await?;
    let pool = db.pool().await;
    // An older listener may have left a pending fork at the committed tip.
    // It still needs normal orphan cleanup when the provider becomes available.
    sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id, block_hash, parent_hash, block_number, block_status) VALUES (42, $1, NULL, 100, 'finalized'), (42, $2, $1, 101, 'pending'), (42, $3, NULL, 100, 'pending')")
        .bind(B256::repeat_byte(2).as_slice())
        .bind(B256::repeat_byte(3).as_slice())
        .bind(B256::repeat_byte(1).as_slice()).execute(&pool).await?;
    let mut requested = vec![];
    update_finalized_blocks_aux(&mut db, 121, 20, |number| {
        requested.push(number);
        async { anyhow::bail!("canonical header temporarily unavailable") }
    })
    .await;
    assert_eq!(
        requested,
        vec![100],
        "verify the tip before pending heights"
    );
    let status: String = sqlx::query_scalar("SELECT block_status FROM host_chain_blocks_valid WHERE chain_id = 42 AND block_number = 101")
        .fetch_one(&pool).await?;
    assert_eq!(status, "pending");
    assert!(!is_halted(&pool).await?);

    requested.clear();
    update_finalized_blocks_aux(&mut db, 121, 20, |number| {
        requested.push(number);
        async move { Ok(B256::repeat_byte(if number == 100 { 2 } else { 3 })) }
    })
    .await;
    assert_eq!(requested, vec![100, 101]);
    let status: String = sqlx::query_scalar("SELECT block_status FROM host_chain_blocks_valid WHERE chain_id = 42 AND block_number = 101")
        .fetch_one(&pool).await?;
    assert_eq!(status, "finalized");
    let stale_status: String = sqlx::query_scalar("SELECT block_status FROM host_chain_blocks_valid WHERE chain_id = 42 AND block_hash = $1")
        .bind(B256::repeat_byte(1).as_slice()).fetch_one(&pool).await?;
    assert_eq!(stale_status, "orphaned");
    Ok(())
}

#[tokio::test]
async fn deferred_finalization_checks_all_retained_neighbors() -> Result<()> {
    use crate::database::ingest::update_finalized_blocks_aux;
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow!("{e}"))?;
    let chain = ChainId::try_from(42_u64)?;
    let mut db = Database::new(&instance.db_url, chain, 64).await?;
    let pool = db.pool().await;
    for neighbor in [99_i64, 100, 101] {
        // Isolate the three contradictions within one fixture. Recreating the
        // localhost DB while asynchronous pool teardown is still running races
        // DROP DATABASE, even after closing the main pool.
        sqlx::query("DELETE FROM public.chain_recovery_halts")
            .execute(&pool)
            .await?;
        sqlx::query("DELETE FROM host_chain_blocks_valid")
            .execute(&pool)
            .await?;
        let candidate = B256::repeat_byte(10);
        sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id,block_hash,parent_hash,block_number,block_status) VALUES (42,$1,$2,100,'pending'), (42,$3,$3,$4,'finalized')")
            .bind(candidate.as_slice()).bind(B256::repeat_byte(9).as_slice())
            .bind(B256::repeat_byte(20).as_slice()).bind(neighbor).execute(&pool).await?;
        update_finalized_blocks_aux(&mut db, 100, 0, |number| async move {
            Ok(if number == 100 {
                candidate
            } else {
                B256::repeat_byte(20)
            })
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
    pool.close().await;
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
    update_finalized_blocks_aux(&mut db, 100, 0, |number| async move {
        Ok(B256::with_last_byte(number as u8))
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
    validate_committed_header(&pool, ChainId::try_from(42_u64)?, &header)
        .await?;
    assert!(!is_halted(&pool).await?);
    header.hash = B256::repeat_byte(2);
    assert!(validate_committed_header(
        &pool,
        ChainId::try_from(42_u64)?,
        &header,
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
    let extended =
        begin_replay(&restarted, ListenerSource::Host, false, 42, 1, 10001)
            .await?;
    assert_eq!((extended.from_block, extended.to_block), (1, 10001));
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
    validate_committed_header(&pool, chain, &block.summary).await?;
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
    assert!(validate_committed_header(&pool, chain, &changed)
        .await
        .is_err());
    assert!(is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[tracing_test::traced_test]
async fn contained_poller_startup_stays_alive_without_starting_replay(
) -> Result<()> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(instance.db_url()).await?;
    record_halt(
        &pool,
        HistoryMismatch {
            source: ListenerSource::Host,
            gcs_mode: false,
            chain_id: 42,
            block_number: 65,
            expected_hash: Some(&[1_u8; 32]),
            observed_hash: Some(&[2_u8; 32]),
            reason: "test committed history conflict",
        },
    )
    .await?;
    let anvil = alloy::node_bindings::Anvil::new().chain_id(42).spawn();
    let config = pause_test_config(instance.db_url.clone(), &anvil, 0);
    let poller = run_poller(config);
    tokio::pin!(poller);
    tokio::select! {
        result = &mut poller => panic!("contained poller exited: {result:?}"),
        result = tokio::time::timeout(Duration::from_secs(10), async {
            while !logs_contain("Host listener paused by chain history containment") {
                sleep(Duration::from_millis(10)).await;
            }
        }) => result?,
    }
    assert!(
        tokio::time::timeout(Duration::from_millis(100), &mut poller)
            .await
            .is_err()
    );
    let replays: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM public.listener_replay_state")
            .fetch_one(&pool)
            .await?;
    assert_eq!(replays, 0);
    assert!(readiness_blocked(&pool).await?);
    Ok(())
}

#[tokio::test]
async fn poller_header_check_queued_at_promotion_uses_live_role_and_lock(
) -> Result<()> {
    use fhevm_engine_common::chain_recovery::lock_history_tx;
    use fhevm_engine_common::database::GCS_SCHEMA_QUOTED;
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(instance.db_url()).await?;
    let green = green_host_pool(&pool, instance.db_url()).await?;
    sqlx::query("INSERT INTO public.host_chain_blocks_valid (chain_id, block_hash, block_number, block_status) VALUES (42, $1, 65, 'finalized')")
        .bind(vec![1_u8; 32]).execute(&pool).await?;
    let mut cutover = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *cutover)
        .await?;
    let cutover_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *cutover)
        .await?;
    let check = tokio::spawn(async move {
        let header = BlockSummary {
            number: 65,
            hash: B256::repeat_byte(2),
            parent_hash: B256::ZERO,
            timestamp: 100,
        };
        validate_committed_header(&green, ChainId::try_from(42_u64)?, &header)
            .await
    });
    async fn wait_for_waiter(pool: &sqlx::PgPool, blocker: i32) -> Result<()> {
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let waiting: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE $1 = ANY(pg_blocking_pids(pid)))")
                    .bind(blocker).fetch_one(pool).await?;
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
    let mut live = pool.begin().await?;
    lock_history_tx(&mut live, ListenerSource::Host, false, 42).await?;
    let live_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *live)
        .await?;
    cutover.commit().await?;
    wait_for_waiter(&pool, live_pid).await?;
    assert!(!check.is_finished());
    assert!(!is_halted(&pool).await?);
    live.rollback().await?;
    assert!(tokio::time::timeout(Duration::from_secs(5), check)
        .await??
        .is_err());
    let roles: Vec<String> = sqlx::query_scalar("SELECT stack_role FROM public.chain_recovery_halts WHERE resolved_at IS NULL").fetch_all(&pool).await?;
    assert_eq!(roles, vec!["BCS"]);
    Ok(())
}

#[tokio::test]
async fn retired_poller_header_check_does_not_record_a_halt() -> Result<()> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(instance.db_url()).await?;
    sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status) VALUES (42, $1, 65, 'finalized')")
        .bind(vec![1_u8; 32]).execute(&pool).await?;
    sqlx::query("UPDATE versioning SET consensus_version = 9999")
        .execute(&pool)
        .await?;
    let header = BlockSummary {
        number: 65,
        hash: B256::repeat_byte(2),
        parent_hash: B256::ZERO,
        timestamp: 100,
    };
    validate_committed_header(&pool, ChainId::try_from(42_u64)?, &header)
        .await?;
    assert!(!is_halted(&pool).await?);
    Ok(())
}
