use consumer::{BlockPayload, Broker};
use fhevm_engine_common::chain_id::ChainId;
use host_listener::consumer::{
    catchup::ManualCatchupArgs, run_consumer, ConsumerConfig,
};
use primitives::event::BlockFlow;
use tokio::{sync::mpsc, task::JoinSet};
use tokio_util::sync::CancellationToken;

/// Exercises public startup with real PostgreSQL ingestion and a broker control peer.
#[tokio::test]
#[ignore = "requires Docker and CATCHUP_BROKER_URL (Redis or RabbitMQ)"]
async fn consumer_catchup_and_listen() {
    tokio::time::timeout(
        std::time::Duration::from_secs(90),
        exercise_catchup(false),
    )
    .await
    .unwrap();
}

#[tokio::test]
#[ignore = "requires Docker and CATCHUP_BROKER_URL (Redis or RabbitMQ)"]
async fn consumer_bounded_catchup_keeps_listening() {
    tokio::time::timeout(
        std::time::Duration::from_secs(90),
        exercise_catchup(true),
    )
    .await
    .unwrap();
}

async fn exercise_catchup(bounded: bool) {
    use alloy::primitives::Address;
    use broker::{AsyncHandlerPayloadClassified, Topic};
    use primitives::{
        event::{CatchupPayload, WatchCommand},
        routing,
        utils::chain_id_to_namespace,
    };
    use test_harness::instance::{setup_test_db, ImportMode};
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let broker_url = std::env::var("CATCHUP_BROKER_URL").unwrap();
    let broker = Broker::from_url(&broker_url).await.unwrap();
    let chain_id = ChainId::try_from(
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            % 900_000_000) as u64
            + 100_000_000,
    )
    .unwrap();
    let config = ConsumerConfig {
        manual_catchup: ManualCatchupArgs {
            catchup_from_block: Some(900),
            catchup_up_to_block: bounded.then_some(950),
        },
        url: broker_url,
        acl_address: Address::ZERO,
        tfhe_address: Address::repeat_byte(1),
        kms_generation_address: None,
        protocol_config_address: None,
        confidential_bridge_address: None,
        database_url: instance.db_url.clone(),
        database_retry_interval: std::time::Duration::from_millis(10),
        service_name: "catchup-test".into(),
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
    let pool = sqlx::PgPool::connect(instance.db_url.as_str())
        .await
        .unwrap();
    let namespace = chain_id_to_namespace(chain_id.as_u64());
    let (watch_tx, mut watch_rx) = mpsc::unbounded_channel();
    let (catchup_tx, mut catchup_rx) = mpsc::unbounded_channel();
    let control_stop = CancellationToken::new();
    let mut controls = JoinSet::new();
    let watch = broker
        .consumer(&Topic::namespaced(&namespace, routing::WATCH))
        .group("catchup-test-watch")
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
    let catchup = broker
        .consumer(&Topic::namespaced(&namespace, routing::CATCHUP))
        .group("catchup-test-catchup")
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
    let run = tokio::spawn(run_consumer(config));
    let publisher = broker.publisher("").await.unwrap();
    let block = |number, flow| event_block(chain_id.as_u64(), number, flow);
    let first = watch_rx.recv().await.unwrap();
    publisher
        .publish(
            &routing::consumer_new_event_routing(first.clone()),
            &block(1000, BlockFlow::Live),
        )
        .await
        .unwrap();
    let request = catchup_rx.recv().await.unwrap();
    assert_eq!(
        (request.block_start, request.block_end),
        (900, if bounded { 950 } else { 999 })
    );
    // Live advances before historical delivery finishes.
    publisher
        .publish(
            &routing::consumer_catchup_event_routing(first.clone()),
            &block(900, BlockFlow::Catchup),
        )
        .await
        .unwrap();
    publisher
        .publish(
            &routing::consumer_new_event_routing(first.clone()),
            &block(1001, BlockFlow::Live),
        )
        .await
        .unwrap();
    wait_for_events(&pool, &[900, 1000, 1001]).await;
    let end = request.block_end;
    // Out-of-order chunks and duplicate deliveries must preserve both event sets.
    for number in [end, 925, 900] {
        publisher
            .publish(
                &routing::consumer_catchup_event_routing(first.clone()),
                &block(number, BlockFlow::Catchup),
            )
            .await
            .unwrap();
    }
    wait_for_events(&pool, &[900, 925, end, 1000, 1001]).await;
    assert!(
        !run.is_finished(),
        "bounded replay must not stop live consumption"
    );
    publisher
        .publish(
            &routing::consumer_new_event_routing(first.clone()),
            &block(1002, BlockFlow::Live),
        )
        .await
        .unwrap();
    wait_for_events(&pool, &[900, 925, end, 1000, 1001, 1002]).await;
    // Only live delivery contributes timing rows. Historical replay (including
    // finalized replay when subscribed) must not pollute live timing statistics.
    let timed_blocks: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM host_chain_consumer_blocks WHERE chain_id = $1 ORDER BY block_number",
    ).bind(chain_id.as_i64()).fetch_all(&pool).await.unwrap();
    assert_eq!(timed_blocks, vec![1000, 1001, 1002]);
    // Public startup has no external stop handle. Each test owns its Tokio runtime,
    // which also drops the process services spawned by the consumer.
    run.abort();
    let _ = run.await;
    control_stop.cancel();
    while let Some(result) = controls.join_next().await {
        result.unwrap().unwrap();
    }
    instance.parent_token.cancel();
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
    let tfhe = host_listener::contracts::TfheContract::TrivialEncrypt {
        caller,
        pt: U256::from(number),
        toType: 4,
        result: handle,
    }
    .encode_log_data();
    let acl = host_listener::contracts::AclContract::Allowed {
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
