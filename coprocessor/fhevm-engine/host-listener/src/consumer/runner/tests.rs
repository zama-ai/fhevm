use super::*;
use primitives::event::BlockFlow;

/// Exercises the real host runner and PostgreSQL ingestion. The control peer
/// stands in for core RPC fetching; gw-listener cleanup is represented by SQL.
#[tokio::test]
#[ignore = "requires Docker and DRIFT_RECOVERY_BROKER_URL (Redis or RabbitMQ)"]
async fn drift_restart_is_autonomous_and_isolates_old_messages() {
    tokio::time::timeout(
        std::time::Duration::from_secs(90),
        exercise_restart(true, false),
    )
    .await
    .unwrap();
}

#[tokio::test]
#[ignore = "requires Docker and DRIFT_RECOVERY_BROKER_URL (Redis or RabbitMQ)"]
async fn drift_restart_works_without_manual_catchup() {
    tokio::time::timeout(
        std::time::Duration::from_secs(90),
        exercise_restart(false, false),
    )
    .await
    .unwrap();
}

#[tokio::test]
#[ignore = "requires Docker and DRIFT_RECOVERY_BROKER_URL (Redis or RabbitMQ)"]
async fn consumer_recovers_events_after_real_revert() {
    tokio::time::timeout(
        std::time::Duration::from_secs(90),
        exercise_restart(true, true),
    )
    .await
    .unwrap();
}

async fn exercise_restart(manual: bool, real_cleanup: bool) {
    use alloy::primitives::Address;
    use broker::{AsyncHandlerPayloadClassified, Topic};
    use primitives::{
        event::{CatchupPayload, FilterCommand, WatchCommand},
        routing,
        utils::chain_id_to_namespace,
    };
    use test_harness::instance::{setup_test_db, ImportMode};
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let broker_url = std::env::var("DRIFT_RECOVERY_BROKER_URL").unwrap();
    let broker = Broker::from_url(&broker_url).await.unwrap();
    let chain_id = ChainId::try_from(
        (uuid::Uuid::new_v4().as_u128() % 900_000_000) as u64 + 100_000_000,
    )
    .unwrap();
    let config = ConsumerConfig {
        manual_catchup: super::super::catchup::ManualCatchupArgs {
            catchup_from_block: manual.then_some(900),
            catchup_up_to_block: None,
        },
        url: broker_url,
        acl_address: Address::ZERO,
        tfhe_address: Address::repeat_byte(1),
        kms_generation_address: Address::repeat_byte(2),
        protocol_config_address: None,
        confidential_bridge_address: None,
        database_url: instance.db_url.clone(),
        database_retry_interval: std::time::Duration::from_millis(10),
        service_name: "drift-test".into(),
        health_port: 0,
        dependence_cache_size: 16,
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain: 1,
        chain_id: chain_id.as_u64().to_string(),
        gcs_mode: false,
        disable_synthetic_ops: true,
        canonical_protocol_config_chain_id: None,
    };
    let db = new_database(&config, chain_id).await.unwrap();
    let pool = db.pool().await;
    let stop = CancellationToken::new();
    let namespace = chain_id_to_namespace(chain_id.as_u64());
    let (watch_tx, mut watch_rx) = mpsc::unbounded_channel();
    let (unwatch_tx, mut unwatch_rx) = mpsc::unbounded_channel();
    let (catchup_tx, mut catchup_rx) = mpsc::unbounded_channel();
    let control_stop = CancellationToken::new();
    let mut controls = JoinSet::new();
    let watch = broker
        .consumer(&Topic::namespaced(&namespace, routing::WATCH))
        .group("drift-test-watch")
        .with_cancellation(control_stop.clone())
        .build()
        .unwrap();
    watch.ensure_topology().await.unwrap();
    controls.spawn(watch.run(AsyncHandlerPayloadClassified::new(
        move |command: WatchCommand| {
            let tx = watch_tx.clone();
            async move {
                tx.send(command.into_filters()[0].consumer_id.clone())
                    .unwrap();
                Ok(())
            }
        },
    )));
    let unwatch = broker
        .consumer(&Topic::namespaced(&namespace, routing::UNWATCH))
        .group("drift-test-unwatch")
        .with_cancellation(control_stop.clone())
        .build()
        .unwrap();
    unwatch.ensure_topology().await.unwrap();
    controls.spawn(unwatch.run(AsyncHandlerPayloadClassified::new(
        move |command: FilterCommand| {
            let tx = unwatch_tx.clone();
            async move {
                let _ = tx.send(command.consumer_id);
                Ok(())
            }
        },
    )));
    let catchup = broker
        .consumer(&Topic::namespaced(&namespace, routing::FINAL_CATCHUP))
        .group("drift-test-catchup")
        .with_cancellation(control_stop.clone())
        .build()
        .unwrap();
    catchup.ensure_topology().await.unwrap();
    controls.spawn(catchup.run(AsyncHandlerPayloadClassified::new(
        move |command: CatchupPayload| {
            let tx = catchup_tx.clone();
            async move {
                tx.send(command).unwrap();
                Ok(())
            }
        },
    )));
    let runner = Runner {
        contracts: vec![config.acl_address, config.tfhe_address],
        options: IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: false,
            dependent_ops_max_per_chain: 1,
            is_protocol_config_listener: false,
            disable_synthetic_ops: true,
        },
        config,
        db,
        broker: broker.clone(),
        stack_mode: StackMode::new(false),
        blockchain_tick: HeartBeat::new(),
        stop: stop.clone(),
        identity: format!("drift-test.{}", uuid::Uuid::new_v4()),
    };
    let run = tokio::spawn(async move { runner.run().await });
    let publisher = broker.publisher("").await.unwrap();
    let block = |number, flow| event_block(chain_id.as_u64(), number, flow);
    let first = watch_rx.recv().await.unwrap();
    assert!(first.ends_with(".s0"));
    publisher
        .publish(
            &routing::consumer_new_event_routing(first.clone()),
            &block(1000, BlockFlow::Live),
        )
        .await
        .unwrap();
    if manual {
        let request = catchup_rx.recv().await.unwrap();
        assert_eq!((request.block_start, request.block_end), (900, 999));
        publisher
            .publish(
                &routing::consumer_final_catchup_event_routing(first.clone()),
                &block(950, BlockFlow::FinalCatchup),
            )
            .await
            .unwrap();
        wait_for_block(&pool, 950).await;
    } else {
        wait_for_block(&pool, 1000).await;
        assert!(catchup_rx.try_recv().is_err());
    }
    if real_cleanup {
        wait_for_events(&pool, &[950, 1000]).await;
    }
    sqlx::query("INSERT INTO drift_revert_signal (host_chain_id, offending_host_block_number, status) VALUES ($1, 900, 'pending')")
    .bind(chain_id.as_u64() as i64).execute(&pool).await.unwrap();
    // An empty broker cannot hide the signal: the periodic DB watcher stops it.
    assert_eq!(unwatch_rx.recv().await.unwrap(), first);
    assert!(tokio::time::timeout(
        std::time::Duration::from_millis(100),
        watch_rx.recv()
    )
    .await
    .is_err());
    sqlx::query("UPDATE drift_revert_signal SET offending_host_block_number = 800, status = 'reverting'")
    .execute(&pool).await.unwrap();
    assert!(tokio::time::timeout(
        std::time::Duration::from_millis(100),
        watch_rx.recv()
    )
    .await
    .is_err());
    if real_cleanup {
        sqlx::query("INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES ($1, 'test', '0x0') ON CONFLICT DO NOTHING")
        .bind(chain_id.as_u64() as i64).execute(&pool).await.unwrap();
        let sql = test_harness::db_utils::revert_coprocessor_db_state_sql(
            chain_id.as_u64() as i64,
            799,
        );
        sqlx::raw_sql(&sql).execute(&pool).await.unwrap();
        assert_event_sets(&pool, &[]).await;
    } else {
        sqlx::query(
            "DELETE FROM host_chain_blocks_valid WHERE block_number >= 800",
        )
        .execute(&pool)
        .await
        .unwrap();
    }
    sqlx::query("UPDATE drift_revert_signal SET status = 'done'")
        .execute(&pool)
        .await
        .unwrap();
    let second = watch_rx.recv().await.unwrap();
    assert!(second.ends_with(".s1"));
    assert_ne!(first, second);
    // Old producers may still publish, but their subscription cannot ingest.
    publisher
        .publish(
            &routing::consumer_new_event_routing(first.clone()),
            &block(7777, BlockFlow::Live),
        )
        .await
        .unwrap();
    publisher
        .publish(
            &routing::consumer_final_catchup_event_routing(first.clone()),
            &block(7778, BlockFlow::FinalCatchup),
        )
        .await
        .unwrap();
    publisher
        .publish(
            &routing::consumer_final_event_routing(first.clone()),
            &block(7779, BlockFlow::Final),
        )
        .await
        .unwrap();
    publisher
        .publish(
            &routing::consumer_final_catchup_event_routing(first),
            &block(7780, BlockFlow::FinalCatchup),
        )
        .await
        .unwrap();
    publisher
        .publish(
            &routing::consumer_new_event_routing(second.clone()),
            &block(1100, BlockFlow::Live),
        )
        .await
        .unwrap();
    let request = catchup_rx.recv().await.unwrap();
    assert_eq!(request.consumer_id, second);
    assert_eq!((request.block_start, request.block_end), (800, 1099));
    publisher
        .publish(
            &routing::consumer_final_catchup_event_routing(second),
            &block(850, BlockFlow::FinalCatchup),
        )
        .await
        .unwrap();
    wait_for_block(&pool, 850).await;
    wait_for_block(&pool, 1100).await;
    if real_cleanup {
        for number in [1000, 950, 950] {
            publisher
                .publish(
                    &routing::consumer_final_catchup_event_routing(
                        request.consumer_id.clone(),
                    ),
                    &block(number, BlockFlow::FinalCatchup),
                )
                .await
                .unwrap();
        }
        wait_for_events(&pool, &[850, 950, 1000, 1100]).await;
    }
    let old: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM host_chain_blocks_valid WHERE block_number IN (7777, 7778, 7779, 7780)").fetch_one(&pool).await.unwrap();
    assert_eq!(old, 0);
    stop.cancel();
    run.await.unwrap().unwrap();
    control_stop.cancel();
    while let Some(result) = controls.join_next().await {
        result.unwrap().unwrap();
    }
    instance.parent_token.cancel();
}

async fn wait_for_block(pool: &sqlx::PgPool, block: i64) {
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let exists: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM host_chain_blocks_valid WHERE block_number = $1)").bind(block).fetch_one(pool).await.unwrap();
            if exists { return; }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    }).await.unwrap();
}

// Real ABI-encoded TFHE and ACL events exercise the same ingestion as RPC payloads.
fn event_block(chain_id: u64, number: u64, flow: BlockFlow) -> BlockPayload {
    use alloy::{
        primitives::{Address, B256, U256},
        sol_types::SolEvent,
    };
    use primitives::event::{IndexedLog, TransactionPayload};
    let handle = B256::from(U256::from(number).to_be_bytes::<32>());
    let caller = Address::repeat_byte(2);
    let tfhe = crate::contracts::TfheContract::TrivialEncrypt {
        caller,
        pt: U256::from(number),
        toType: 4,
        result: handle,
    }
    .encode_log_data();
    let acl = crate::contracts::AclContract::Allowed {
        caller,
        account: caller,
        handle,
    }
    .encode_log_data();
    BlockPayload {
        flow,
        chain_id,
        block_number: number,
        block_hash: handle,
        parent_hash: B256::ZERO,
        timestamp: 1_700_000_000,
        transactions: vec![TransactionPayload {
            from: caller,
            to: Some(Address::repeat_byte(1)),
            hash: handle,
            transaction_index: 0,
            value: U256::ZERO,
            data: Default::default(),
            logs: [(Address::repeat_byte(1), tfhe), (Address::ZERO, acl)]
                .into_iter()
                .enumerate()
                .map(|(index, (address, data))| IndexedLog {
                    log_index: index as u64,
                    address,
                    topics: data.topics().to_vec(),
                    data: data.data,
                })
                .collect(),
        }],
    }
}

async fn event_sets(pool: &sqlx::PgPool) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let computations = sqlx::query_scalar(
        "SELECT output_handle FROM computations ORDER BY output_handle",
    )
    .fetch_all(pool)
    .await
    .unwrap();
    let allowed = sqlx::query_scalar(
        "SELECT handle FROM allowed_handles ORDER BY handle",
    )
    .fetch_all(pool)
    .await
    .unwrap();
    (computations, allowed)
}

fn expected_handles(blocks: &[u64]) -> Vec<Vec<u8>> {
    let mut expected: Vec<_> = blocks
        .iter()
        .map(|n| {
            alloy::primitives::U256::from(*n)
                .to_be_bytes::<32>()
                .to_vec()
        })
        .collect();
    expected.sort();
    expected
}

async fn assert_event_sets(pool: &sqlx::PgPool, blocks: &[u64]) {
    let expected = expected_handles(blocks);
    assert_eq!(event_sets(pool).await, (expected.clone(), expected));
}

async fn wait_for_events(pool: &sqlx::PgPool, blocks: &[u64]) {
    let expected = expected_handles(blocks);
    let result =
        tokio::time::timeout(std::time::Duration::from_secs(15), async {
            loop {
                let (computations, allowed) = event_sets(pool).await;
                if computations == expected && allowed == expected {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            }
        })
        .await;
    assert_event_sets(pool, blocks).await;
    result.unwrap();
}
