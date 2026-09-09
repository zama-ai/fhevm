//! End-to-end tests for `ListenerConsumer::watch_contract` /
//! `ListenerConsumer::unwatch_contract`.
//!
//! Spins up a throwaway Redis via testcontainers, publishes through the
//! consumer-lib API, and verifies the messages arrive on the expected
//! routing keys with the correct payload.
//!
//! Run via:
//!
//! ```bash
//! make test-e2e-consumer
//! ```

use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use broker::{AsyncHandlerPayloadOnly, Broker, CancellationToken, Topic};
use consumer::{
    AckDecision, BlockFlow, BlockPayload, CatchupPayload, FilterCommand, FilterType,
    ListenerConsumer,
};
use primitives::routing;
use primitives::utils::chain_id_to_namespace;
use test_support::shared_redis_url;
use tokio::sync::Mutex as AsyncMutex;

// ── Shared container ────────────────────────────────────────────────────────

static REDIS_TEST_LOCK: AsyncMutex<()> = AsyncMutex::const_new(());
static TEST_ID: AtomicU64 = AtomicU64::new(0);

async fn consumer_redis_url() -> String {
    // Use a dedicated logical DB in the shared Redis container so this suite can
    // reset state with FLUSHDB without disturbing other tests.
    format!("{}/15", shared_redis_url().await.trim_end_matches('/'))
}

fn unique_name(prefix: &str) -> String {
    format!(
        "{prefix}-{}-{}",
        std::process::id(),
        TEST_ID.fetch_add(1, Ordering::Relaxed)
    )
}

async fn reset_redis(url: &str) {
    let client = redis::Client::open(url).expect("invalid Redis URL in reset_redis");
    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .expect("failed to connect to Redis in reset_redis");
    redis::cmd("FLUSHDB")
        .query_async::<()>(&mut conn)
        .await
        .expect("FLUSHDB failed in reset_redis");
}

async fn wait_for_consumer_ack(broker: &Broker, topic: &Topic, group: &str) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while tokio::time::Instant::now() < deadline {
        if broker
            .is_empty(topic, group)
            .await
            .expect("broker.is_empty failed during wait_for_consumer_ack")
        {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    panic!("consumer group {group} did not drain and ACK within 5 seconds");
}

/// Publish a FilterCommand through the given routing key, subscribe on the
/// other side, and return the deserialized message that arrived.
async fn assert_filter_command_roundtrip(
    routing_key: &str,
    group_prefix: &str,
    command: &FilterCommand,
) -> FilterCommand {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let chain_id = 1;
    let consumer = ListenerConsumer::new(&broker, chain_id, &command.consumer_id);
    let topic = Topic::new(routing_key).with_namespace(chain_id_to_namespace(chain_id));
    let group = unique_name(group_prefix);
    let consumer_name = unique_name("consumer");
    let cancel = CancellationToken::new();

    let received = Arc::new(Mutex::new(None::<FilterCommand>));
    let received_clone = received.clone();
    let handler = AsyncHandlerPayloadOnly::new(move |msg: FilterCommand| {
        let received = received_clone.clone();
        async move {
            *received.lock().unwrap() = Some(msg);
            Ok::<(), std::convert::Infallible>(())
        }
    });

    let consumer_broker = broker.clone();
    let consumer_topic = topic.clone();
    let consumer_group = group.clone();
    let consumer_cancel = cancel.clone();
    let consumer_handle = tokio::spawn(async move {
        consumer_broker
            .consumer(&consumer_topic)
            .group(&consumer_group)
            .consumer_name(&consumer_name)
            .prefetch(10)
            .redis_block_ms(100)
            .with_cancellation(consumer_cancel)
            .run(handler)
            .await
    });

    tokio::time::sleep(Duration::from_millis(400)).await;
    if consumer_handle.is_finished() {
        let result = consumer_handle
            .await
            .expect("consumer task should not panic");
        panic!("{routing_key} consumer exited before publish: {result:?}");
    }

    match routing_key {
        routing::WATCH => consumer.register_filter(command).await.unwrap(),
        routing::UNWATCH => consumer.unregister_filter(command).await.unwrap(),
        other => panic!("unexpected routing key: {other}"),
    };

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while received.lock().unwrap().is_none() && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    wait_for_consumer_ack(&broker, &topic, &group).await;

    cancel.cancel();
    let run_result = tokio::time::timeout(Duration::from_secs(5), consumer_handle)
        .await
        .expect("consumer should stop after cancellation")
        .expect("consumer task should not panic");
    run_result.expect("consumer should not return an error");

    received
        .lock()
        .unwrap()
        .clone()
        .unwrap_or_else(|| panic!("should receive {routing_key} message"))
}

#[tokio::test]
#[ignore = "requires Docker"]
async fn watch_contract_publishes_register_filter() {
    let command = FilterCommand {
        consumer_id: "gateway".into(),
        from: Some(
            "0x00000000000000000000000000000000deadbeef"
                .parse()
                .unwrap(),
        ),
        to: Some(
            "0x00000000000000000000000000000000cafebabe"
                .parse()
                .unwrap(),
        ),
        log_address: None,
        filter_type: None,
    };

    let msg = assert_filter_command_roundtrip(routing::WATCH, "watch-e2e-register", &command).await;
    assert_eq!(msg, command);
}

#[tokio::test]
#[ignore = "requires Docker"]
async fn register_full_block_publishes_wildcard_filter() {
    // A full-block subscription carries no address fields; it must validate and
    // round-trip over the broker just like an address-scoped filter.
    let command = FilterCommand {
        consumer_id: "gateway".into(),
        from: None,
        to: None,
        log_address: None,
        filter_type: None,
    };

    let msg =
        assert_filter_command_roundtrip(routing::WATCH, "watch-e2e-full-block", &command).await;
    assert_eq!(msg, command);
    assert!(msg.from.is_none() && msg.to.is_none() && msg.log_address.is_none());
}

#[tokio::test]
#[ignore = "requires Docker"]
async fn watch_final_contract_publishes_register_filter() {
    // A finalized-only filter must survive the broker wire with its
    // filter_type intact — the listener relies on it to create a FINAL watcher.
    let command = FilterCommand {
        consumer_id: "gateway".into(),
        from: None,
        to: None,
        log_address: Some(
            "0x00000000000000000000000000000000deadbeef"
                .parse()
                .unwrap(),
        ),
        filter_type: Some(FilterType::Final),
    };

    let msg = assert_filter_command_roundtrip(routing::WATCH, "watch-e2e-final", &command).await;
    assert_eq!(msg, command);
    assert_eq!(msg.filter_type, Some(FilterType::Final));
}

/// Build a minimal BlockPayload with the given flow for delivery-queue tests.
fn sample_block_payload(flow: BlockFlow, chain_id: u64, block_number: u64) -> BlockPayload {
    BlockPayload {
        flow,
        chain_id,
        block_number,
        block_hash: [1u8; 32].into(),
        parent_hash: [0u8; 32].into(),
        timestamp: 1_700_000_000,
        transactions: vec![],
    }
}

#[tokio::test]
#[ignore = "requires Docker"]
async fn request_final_catchup_publishes_payload() {
    // A final catchup request must land on the chain-namespaced
    // `final-catchup` control queue with the CatchupPayload intact — that is
    // the wire the listener's FinalCatchupHandler consumes.
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let chain_id = 1;
    let consumer = ListenerConsumer::new(&broker, chain_id, "gateway");
    let topic = Topic::new(routing::FINAL_CATCHUP).with_namespace(chain_id_to_namespace(chain_id));
    let group = unique_name("final-catchup-e2e");
    let consumer_name = unique_name("consumer");
    let cancel = CancellationToken::new();

    let received = Arc::new(Mutex::new(None::<CatchupPayload>));
    let received_clone = received.clone();
    let handler = AsyncHandlerPayloadOnly::new(move |msg: CatchupPayload| {
        let received = received_clone.clone();
        async move {
            *received.lock().unwrap() = Some(msg);
            Ok::<(), std::convert::Infallible>(())
        }
    });

    let consumer_broker = broker.clone();
    let consumer_topic = topic.clone();
    let consumer_group = group.clone();
    let consumer_cancel = cancel.clone();
    let consumer_handle = tokio::spawn(async move {
        consumer_broker
            .consumer(&consumer_topic)
            .group(&consumer_group)
            .consumer_name(&consumer_name)
            .prefetch(10)
            .redis_block_ms(100)
            .with_cancellation(consumer_cancel)
            .run(handler)
            .await
    });

    tokio::time::sleep(Duration::from_millis(400)).await;
    consumer.request_final_catchup(100, 200).await.unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while received.lock().unwrap().is_none() && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    wait_for_consumer_ack(&broker, &topic, &group).await;
    cancel.cancel();
    consumer_handle
        .await
        .expect("consumer task should not panic")
        .expect("consumer should not return an error");

    let payload = received
        .lock()
        .unwrap()
        .clone()
        .expect("should receive final catchup request");
    assert_eq!(payload.consumer_id, "gateway");
    assert_eq!(payload.block_start, 100);
    assert_eq!(payload.block_end, 200);
}

/// Shared slot the delivery-roundtrip handlers write into. Tests run under
/// REDIS_TEST_LOCK, so at most one roundtrip uses this at a time; each test
/// resets it before use.
static RECEIVED_PAYLOAD: Mutex<Option<BlockPayload>> = Mutex::new(None);

/// Publish a payload on a consumer delivery queue via the raw broker and
/// wait for the caller's consume handler (which writes RECEIVED_PAYLOAD)
/// to pick it up; returns the received payload.
async fn assert_delivery_roundtrip<Fut>(
    broker: &Broker,
    delivery_topic: Topic,
    consume_future: Fut,
    payload: &BlockPayload,
) -> BlockPayload
where
    Fut: std::future::Future<Output = Result<(), broker::BrokerError>> + Send + 'static,
{
    let consume_handle = tokio::spawn(consume_future);
    tokio::time::sleep(Duration::from_millis(400)).await;

    let publisher = broker.publisher_unscoped().await.unwrap();
    publisher
        .publish(&delivery_topic.to_string(), payload)
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while RECEIVED_PAYLOAD.lock().unwrap().is_none() && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let got = RECEIVED_PAYLOAD
        .lock()
        .unwrap()
        .clone()
        .expect("should receive delivered payload");
    consume_handle.abort();
    got
}

#[tokio::test]
#[ignore = "requires Docker"]
async fn consume_final_receives_published_final_event() {
    // A BlockPayload published on {consumer_id}.final-event must reach the
    // consume_final handler with its FINAL flow tag intact.
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    *RECEIVED_PAYLOAD.lock().unwrap() = None;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer = ListenerConsumer::new(&broker, 1, &unique_name("final-consumer"));
    consumer.ensure_final_consumer().await.unwrap();

    let payload = sample_block_payload(BlockFlow::Final, 1, 42);
    let consume_future = consumer.consume_final(move |p, _cancel| async move {
        *RECEIVED_PAYLOAD.lock().unwrap() = Some(p);
        Ok(AckDecision::Ack)
    });

    let got = assert_delivery_roundtrip(
        &broker,
        consumer.final_consumer_topic(),
        consume_future,
        &payload,
    )
    .await;
    consumer.cancel_final();

    assert_eq!(got, payload);
    assert_eq!(got.flow, BlockFlow::Final);
}

#[tokio::test]
#[ignore = "requires Docker"]
async fn consume_final_catchup_receives_published_event() {
    // A BlockPayload published on {consumer_id}.final-catchup-event must
    // reach the consume_final_catchup handler with its FINAL_CATCHUP flow tag.
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    *RECEIVED_PAYLOAD.lock().unwrap() = None;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer = ListenerConsumer::new(&broker, 1, &unique_name("final-catchup-consumer"));
    consumer.ensure_final_catchup_consumer().await.unwrap();

    let payload = sample_block_payload(BlockFlow::FinalCatchup, 1, 43);
    let consume_future = consumer.consume_final_catchup(move |p, _cancel| async move {
        *RECEIVED_PAYLOAD.lock().unwrap() = Some(p);
        Ok(AckDecision::Ack)
    });

    let got = assert_delivery_roundtrip(
        &broker,
        consumer.final_catchup_consumer_topic(),
        consume_future,
        &payload,
    )
    .await;
    consumer.cancel_final_catchup();

    assert_eq!(got, payload);
    assert_eq!(got.flow, BlockFlow::FinalCatchup);
}

#[tokio::test]
#[ignore = "requires Docker"]
async fn unwatch_contract_publishes_unregister_filter() {
    let command = FilterCommand {
        consumer_id: "gateway".into(),
        from: None,
        to: Some(
            "0x00000000000000000000000000000000deadbeef"
                .parse()
                .unwrap(),
        ),
        log_address: None,
        filter_type: None,
    };

    let msg =
        assert_filter_command_roundtrip(routing::UNWATCH, "watch-e2e-unregister", &command).await;
    assert_eq!(msg, command);
}
