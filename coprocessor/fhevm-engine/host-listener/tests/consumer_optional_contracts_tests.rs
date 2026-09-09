//! Broker-to-database coverage for the same optional contracts as the legacy listener.
use alloy::{
    primitives::{Address, B256, U256},
    sol_types::SolEvent,
};
use broker::{AsyncHandlerPayloadClassified, Topic};
use consumer::{BlockPayload, Broker};
use fhevm_engine_common::{
    chain_id::ChainId,
    types::{COMPUTED_HANDLE_INDEX_MARKER, HANDLE_VERSION},
};
use host_listener::{
    consumer::{catchup::ManualCatchupArgs, run_consumer, ConsumerConfig},
    contracts::{BridgeContract, ProtocolConfig},
};
use primitives::{
    event::{BlockFlow, IndexedLog, TransactionPayload, WatchCommand},
    routing,
    utils::chain_id_to_namespace,
};
use test_harness::instance::{setup_test_db, ImportMode};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[tokio::test]
#[ignore = "requires Docker and CATCHUP_BROKER_URL (Redis or RabbitMQ)"]
async fn optional_contracts_on_canonical_chain() {
    exercise(true, true).await;
}
#[tokio::test]
#[ignore = "requires Docker and CATCHUP_BROKER_URL (Redis or RabbitMQ)"]
async fn protocol_config_ignored_on_noncanonical_chain() {
    exercise(true, false).await;
}
#[tokio::test]
#[ignore = "requires Docker and CATCHUP_BROKER_URL (Redis or RabbitMQ)"]
async fn optional_contracts_can_be_omitted() {
    exercise(false, true).await;
}

async fn exercise(enabled: bool, canonical: bool) {
    tokio::time::timeout(
        std::time::Duration::from_secs(90),
        run(enabled, canonical),
    )
    .await
    .unwrap();
}
async fn run(enabled: bool, canonical: bool) {
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
        manual_catchup: ManualCatchupArgs::default(),
        url: broker_url,
        acl_address: Address::ZERO,
        tfhe_address: Address::repeat_byte(1),
        kms_generation_address: Address::repeat_byte(2),
        protocol_config_address: enabled.then_some(Address::repeat_byte(3)),
        confidential_bridge_address: enabled.then_some(Address::repeat_byte(4)),
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
        canonical_protocol_config_chain_id: Some(if canonical {
            chain_id.as_u64()
        } else {
            1
        }),
    };

    let pool = sqlx::PgPool::connect(instance.db_url.as_str())
        .await
        .unwrap();
    let stop = CancellationToken::new();
    let (tx, mut rx) = mpsc::unbounded_channel();
    let watch = broker
        .consumer(&Topic::namespaced(
            chain_id_to_namespace(chain_id.as_u64()),
            routing::WATCH,
        ))
        .group("optional-contract-tests")
        .with_cancellation(stop.clone())
        .build()
        .unwrap();
    watch.ensure_topology().await.unwrap();
    let control = tokio::spawn(watch.run(AsyncHandlerPayloadClassified::new(
        move |command: WatchCommand| {
            let tx = tx.clone();
            async move {
                let filters = command.into_filters();
                assert_eq!(filters.len(), if enabled { 10 } else { 6 });
                for address in
                    [Address::repeat_byte(3), Address::repeat_byte(4)]
                {
                    assert_eq!(
                        filters
                            .iter()
                            .filter(|f| f.log_address == Some(address))
                            .count(),
                        if enabled { 2 } else { 0 }
                    );
                }
                tx.send(filters[0].consumer_id.clone()).unwrap();
                Ok(())
            }
        },
    )));
    let consumer = tokio::spawn(run_consumer(config));
    let id = rx.recv().await.unwrap();
    let publisher = broker.publisher("").await.unwrap();
    // Send wrong-address events first: neither optional contract may accept them.
    let mut wrong = payload(chain_id.as_u64(), 99, BlockFlow::Live);
    for log in &mut wrong.transactions[0].logs {
        log.address = Address::repeat_byte(9);
    }
    publisher
        .publish(&routing::consumer_new_event_routing(id.clone()), &wrong)
        .await
        .unwrap();
    wait_block(&pool, chain_id.as_i64(), 99, "pending").await;
    assert_counts(&pool, 0, 0, 0).await;
    let live = payload(chain_id.as_u64(), 100, BlockFlow::Live);
    publisher
        .publish(&routing::consumer_new_event_routing(id.clone()), &live)
        .await
        .unwrap();
    wait_block(&pool, chain_id.as_i64(), 100, "pending").await;
    assert_counts(
        &pool,
        i64::from(enabled && canonical),
        i64::from(enabled),
        0,
    )
    .await;
    let observations: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM fallback_granted_events")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(observations, i64::from(enabled));
    // Finalized catchup must materialize the deferred fallback, including when
    // the protocol proposal was already ingested through the live queue.
    let finalized = payload(chain_id.as_u64(), 100, BlockFlow::FinalCatchup);
    publisher
        .publish(
            &routing::consumer_final_catchup_event_routing(id),
            &finalized,
        )
        .await
        .unwrap();
    wait_block(&pool, chain_id.as_i64(), 100, "finalized").await;
    assert_counts(
        &pool,
        i64::from(enabled && canonical),
        i64::from(enabled),
        i64::from(enabled),
    )
    .await;
    if enabled && canonical {
        let (version, proposal): (String, Vec<u8>) =
            sqlx::query_as("SELECT version, proposal_id FROM upgrade_state")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(version, "v2");
        assert_eq!(proposal, U256::from(2).to_be_bytes::<32>());
    }
    consumer.abort();
    let _ = consumer.await;
    stop.cancel();
    control.await.unwrap().unwrap();
    instance.parent_token.cancel();
}
async fn wait_block(
    pool: &sqlx::PgPool,
    chain: i64,
    number: i64,
    status: &str,
) {
    loop {
        let found: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM host_chain_blocks_valid WHERE chain_id=$1 AND block_number=$2 AND block_status=$3)").bind(chain).bind(number).bind(status).fetch_one(pool).await.unwrap();
        if found {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
}
async fn assert_counts(
    pool: &sqlx::PgPool,
    upgrades: i64,
    bridges: i64,
    computations: i64,
) {
    let actual: (i64, i64, i64, i64) = sqlx::query_as("SELECT (SELECT COUNT(*) FROM upgrade_state), (SELECT COUNT(*) FROM bridge_handle_events), (SELECT COUNT(*) FROM computations), (SELECT COUNT(*) FROM handle_bridged_events)").fetch_one(pool).await.unwrap();
    assert_eq!(actual, (upgrades, bridges, computations, bridges));
}
fn payload(chain: u64, number: u64, flow: BlockFlow) -> BlockPayload {
    let mut handle = [0x11; 32];
    handle[21] = COMPUTED_HANDLE_INDEX_MARKER;
    handle[22..30].copy_from_slice(&chain.to_be_bytes());
    handle[30] = 4;
    handle[31] = HANDLE_VERSION;
    let upgrade = ProtocolConfig::CoprocessorUpgradeProposed {
        proposalId: U256::from(2),
        softwareVersion: "v2".into(),
        chainUpgradeWindows: vec![ProtocolConfig::ChainUpgradeWindow {
            chainId: chain,
            startBlock: 10000,
            endBlock: 20000,
        }],
        gwStartBlock: 10000,
    }
    .encode_log_data();
    let bridge = BridgeContract::BridgeHandle {
        senderDapp: Address::repeat_byte(5),
        srcHandle: B256::from(handle),
        dstChainId: 2000,
        guid: B256::repeat_byte(6),
    }
    .encode_log_data();
    let fallback = BridgeContract::FallbackGrantedPlaintext {
        dstHandle: B256::from(handle),
        plaintext: U256::from(42),
    }
    .encode_log_data();
    let mut source = handle;
    source[22..30].copy_from_slice(&2000_u64.to_be_bytes());
    let destination = fhevm_engine_common::bridge::derive_dst_handle(
        &source,
        &Address::ZERO.0,
        chain,
        &B256::ZERO.0,
        1700000000,
    );
    let received = BridgeContract::HandleBridged {
        receiverDapp: Address::repeat_byte(8),
        srcHandle: B256::from(source),
        dstHandle: B256::from(destination),
        guid: B256::repeat_byte(8),
    }
    .encode_log_data();
    // Matching chain alone is insufficient: a forged destination must be rejected.
    let invalid_received = BridgeContract::HandleBridged {
        receiverDapp: Address::repeat_byte(8),
        srcHandle: B256::from(source),
        dstHandle: B256::from(handle),
        guid: B256::repeat_byte(9),
    }
    .encode_log_data();
    let foreign_source = BridgeContract::BridgeHandle {
        senderDapp: Address::repeat_byte(5),
        srcHandle: B256::from(source),
        dstChainId: 2000,
        guid: B256::repeat_byte(9),
    }
    .encode_log_data();
    BlockPayload {
        flow,
        chain_id: chain,
        block_number: number,
        block_hash: B256::from(U256::from(number).to_be_bytes::<32>()),
        parent_hash: B256::ZERO,
        timestamp: 1700000000,
        transactions: vec![TransactionPayload {
            from: Address::repeat_byte(5),
            to: None,
            hash: B256::repeat_byte(7),
            transaction_index: 0,
            value: U256::ZERO,
            data: Default::default(),
            logs: [
                (Address::repeat_byte(3), upgrade),
                (Address::repeat_byte(4), bridge),
                (Address::repeat_byte(4), fallback),
                (Address::repeat_byte(4), received),
                (Address::repeat_byte(4), invalid_received),
                (Address::repeat_byte(4), foreign_source),
            ]
            .into_iter()
            .enumerate()
            .map(|(i, (address, data))| IndexedLog {
                address,
                log_index: i as u64,
                topics: data.topics().to_vec(),
                data: data.data,
            })
            .collect(),
        }],
    }
}
