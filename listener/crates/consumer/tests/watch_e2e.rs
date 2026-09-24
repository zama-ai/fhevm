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
    AckDecision, BlockFlow, BlockPayload, CatchupPayload, ConsumerError, FilterCommand, FilterType,
    GroupStatus, ListenerConsumer,
};
use primitives::routing;
use primitives::utils::chain_id_to_namespace;
use redis::FromRedisValue;
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
        catchup_id: None,
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
    let catchup_id = uuid::Uuid::now_v7();
    consumer
        .request_final_catchup(catchup_id, 100, 200)
        .await
        .unwrap();

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
    // The caller-supplied id must survive the wire — it is the only handle
    // that can later cancel this request.
    assert_eq!(payload.catchup_id, Some(catchup_id));
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

async fn redis_group_names(url: &str, stream: &str) -> Vec<String> {
    let client = redis::Client::open(url).unwrap();
    let mut conn = client.get_multiplexed_async_connection().await.unwrap();
    let groups: redis::Value = redis::cmd("XINFO")
        .arg("GROUPS")
        .arg(stream)
        .query_async(&mut conn)
        .await
        .unwrap_or(redis::Value::Array(vec![]));
    let redis::Value::Array(entries) = groups else {
        return vec![];
    };
    entries
        .iter()
        .filter_map(|entry| {
            let redis::Value::Array(fields) = entry else {
                return None;
            };
            // Flat [key, value, key, value, ...]; "name" is the first pair.
            fields.chunks(2).find_map(|pair| match pair {
                [key, value] => {
                    let key = String::from_owned_redis_value(key.clone()).ok()?;
                    (key == "name")
                        .then(|| String::from_owned_redis_value(value.clone()).ok())
                        .flatten()
                }
                _ => None,
            })
        })
        .collect()
}

/// Two builds sharing one identity each get their own copy of every event.
///
/// This is the whole reason the suffix exists. Both consumers use the same
/// `consumer_id`, so they resolve to the same stream and the same registered
/// filters — the listener publishes once, for one subscriber, exactly as it
/// does today. What the suffix buys is two *groups* on that one stream, and a
/// group is what Redis fans out to. Without it both builds share a cursor and
/// every event goes to whichever one happens to read first.
///
/// The structural assertion is the stronger half: one stream, two groups. If
/// a future change derives the group from something that also feeds the topic,
/// the two collapse back into one and the delivery assertion below starts
/// passing or failing on a race instead of on the design.
#[tokio::test]
#[ignore = "requires Docker"]
async fn group_suffix_gives_each_build_its_own_copy_of_every_event() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer_id = unique_name("suffix-consumer");
    let blue = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v2");
    let green = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v3");

    // Same identity on both sides: same stream, and the filter rows either
    // registers are indistinguishable.
    assert_eq!(
        blue.consumer_topic(),
        green.consumer_topic(),
        "the suffix must not move the stream — only the group"
    );
    assert_eq!(blue.consumer_id(), green.consumer_id());

    blue.ensure_consumer().await.unwrap();

    let blue_seen = Arc::new(Mutex::new(Vec::<u64>::new()));
    let green_seen = Arc::new(Mutex::new(Vec::<u64>::new()));

    let blue_slot = blue_seen.clone();
    let blue_handle = tokio::spawn(blue.clone().consume(move |p: BlockPayload, _cancel| {
        let slot = blue_slot.clone();
        async move {
            slot.lock().unwrap().push(p.block_number);
            Ok(AckDecision::Ack)
        }
    }));
    let green_slot = green_seen.clone();
    let green_handle = tokio::spawn(green.clone().consume(move |p: BlockPayload, _cancel| {
        let slot = green_slot.clone();
        async move {
            slot.lock().unwrap().push(p.block_number);
            Ok(AckDecision::Ack)
        }
    }));

    // Both groups must exist before the event is published, or a consumer that
    // was late would miss it for reasons unrelated to the suffix.
    let stream = blue.consumer_topic().to_string();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    loop {
        let groups = redis_group_names(&url, &stream).await;
        if groups.len() == 2 {
            let mut groups = groups;
            groups.sort();
            assert_eq!(
                groups,
                vec![format!("{stream}.v2"), format!("{stream}.v3")],
                "one stream must carry one group per suffix"
            );
            break;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "expected two groups on {stream}, found {groups:?}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let payload = sample_block_payload(BlockFlow::Live, 1, 77);
    broker
        .publisher_unscoped()
        .await
        .unwrap()
        .publish(&stream, &payload)
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while (blue_seen.lock().unwrap().is_empty() || green_seen.lock().unwrap().is_empty())
        && tokio::time::Instant::now() < deadline
    {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    blue.cancel();
    green.cancel();
    blue_handle.abort();
    green_handle.abort();

    assert_eq!(
        *blue_seen.lock().unwrap(),
        vec![77],
        "the v2 build must receive the event"
    );
    assert_eq!(
        *green_seen.lock().unwrap(),
        vec![77],
        "the v3 build must receive the same event, not have it stolen by v2"
    );
}

/// Spawn a live consumer on `client` and wait until its group exists.
///
/// Group creation happens when a consumer starts reading, not when the stream
/// is ensured, so tests that want to observe a group have to let one run.
async fn start_live_consumer(
    url: &str,
    client: &ListenerConsumer,
) -> tokio::task::JoinHandle<Result<(), broker::BrokerError>> {
    let handle = tokio::spawn(
        client
            .clone()
            .consume(move |_: BlockPayload, _cancel| async move { Ok(AckDecision::Ack) }),
    );

    let stream = client.consumer_topic().key();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while redis_group_names(url, &stream).await.is_empty() {
        assert!(
            tokio::time::Instant::now() < deadline,
            "no consumer group appeared on {stream} within 10s"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    handle
}

/// Retiring a *predecessor identity* removes its streams and leaves the
/// successor's alone.
///
/// This is the rename branch: the identity itself changed, so the old streams
/// are nobody's and go away whole. The assertion that matters is the second
/// one — deleting by identity must be scoped to that identity. The two sets of
/// stream keys differ only by the consumer ID embedded in them, so a wildcard
/// or a prefix match that was slightly too generous would take the survivor
/// with it and stop the data plane.
#[tokio::test]
#[ignore = "requires Docker"]
async fn retiring_an_identity_deletes_only_its_own_streams() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let old = ListenerConsumer::new(&broker, 1, &unique_name("old-identity"));
    let new =
        ListenerConsumer::new(&broker, 1, &unique_name("new-identity")).with_group_suffix("v2");

    old.ensure_consumer().await.unwrap();
    new.ensure_consumer().await.unwrap();

    // Let the old identity's group exist, then take its reader away: that is
    // the state the migration task is built to recognize.
    let old_handle = start_live_consumer(&url, &old).await;
    old.cancel();
    old_handle.abort();

    let before = old.group_status().await.unwrap();
    assert_eq!(before.len(), 4, "an identity owns four streams");
    assert_eq!(
        before
            .iter()
            .filter(|(_, groups)| !groups.is_empty())
            .count(),
        1,
        "only the live stream was consumed, so only it has a group: {before:?}"
    );

    let deleted = old.delete_streams().await.unwrap();
    assert_eq!(
        deleted, 2,
        "the live stream and its dead-letter companion existed and should both go"
    );

    assert!(
        !broker.exists(&old.consumer_topic()).await.unwrap(),
        "the retired identity's stream must be gone"
    );
    assert!(
        broker.exists(&new.consumer_topic()).await.unwrap(),
        "retiring one identity must not touch another's streams"
    );

    // Deleting again is a no-op rather than an error, so a task that retries
    // after a partial failure converges instead of wedging.
    assert_eq!(old.delete_streams().await.unwrap(), 0);
}

/// Retiring a *predecessor group* under a shared identity leaves the stream and
/// the survivor's group intact.
///
/// This is the same-identity branch: only the group name moved, so the two
/// builds share one stream and one set of filter rows. Deleting the stream here
/// would delete the live build's stream — the only thing that may be removed is
/// the group the suffix-less build created.
#[tokio::test]
#[ignore = "requires Docker"]
async fn retiring_an_unsuffixed_group_leaves_the_shared_stream_intact() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer_id = unique_name("shared-identity");
    let legacy = ListenerConsumer::new(&broker, 1, &consumer_id);
    let current = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v2");

    legacy.ensure_consumer().await.unwrap();

    // The legacy group has to be created by a suffix-less build, not by hand:
    // the name this removes must be the name that build actually produces.
    let legacy_handle = start_live_consumer(&url, &legacy).await;
    let current_handle = start_live_consumer(&url, &current).await;

    let stream = current.consumer_topic().key();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while redis_group_names(&url, &stream).await.len() < 2 {
        assert!(
            tokio::time::Instant::now() < deadline,
            "expected both groups on {stream}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    legacy.cancel();
    legacy_handle.abort();

    // A build with no suffix would be destroying its own group.
    let refused = legacy.destroy_unsuffixed_groups().await.unwrap_err();
    assert!(
        matches!(refused, ConsumerError::InvalidParameter(_)),
        "a suffix-less client must refuse, got {refused:?}"
    );
    assert_eq!(redis_group_names(&url, &stream).await.len(), 2);

    assert_eq!(
        current.destroy_unsuffixed_groups().await.unwrap(),
        1,
        "only the live stream carried a legacy group"
    );

    current.cancel();
    current_handle.abort();

    assert_eq!(
        redis_group_names(&url, &stream).await,
        vec![format!("{stream}.v2")],
        "the surviving build's group must be the only one left"
    );
    assert!(
        broker.exists(&current.consumer_topic()).await.unwrap(),
        "retiring a group must not take the shared stream with it"
    );
}

/// Drop every entry from a stream without removing the stream itself.
async fn trim_stream_empty(url: &str, stream: &str) {
    let client = redis::Client::open(url).unwrap();
    let mut conn = client.get_multiplexed_async_connection().await.unwrap();
    let _: redis::Value = redis::cmd("XTRIM")
        .arg(stream)
        .arg("MAXLEN")
        .arg(0)
        .query_async(&mut conn)
        .await
        .unwrap();
}

/// Write positions must answer "has anybody appended since I last looked",
/// which is what lets the migration task wait for a publisher to stop before
/// it deletes the streams out from under it.
///
/// The trimming assertion is the one that matters: it is the reason this reads
/// the last generated ID rather than the stream length. A stream trimmed as
/// fast as it is written holds its length steady while entries keep arriving,
/// so a length-based check would read "quiet" on the busiest possible stream
/// and delete it mid-publish.
#[tokio::test]
#[ignore = "requires Docker"]
async fn write_positions_track_appends_and_survive_trimming() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let client = ListenerConsumer::new(&broker, 1, &unique_name("write-positions"));
    client.ensure_consumer().await.unwrap();

    let baseline = client.write_positions().await.unwrap();
    assert_eq!(baseline.len(), 4, "an identity owns four streams");

    // Nothing published: reading twice must give the same answer, or the
    // migration task could never conclude that a stream had gone quiet.
    assert_eq!(
        client.write_positions().await.unwrap(),
        baseline,
        "an idle stream must report a stable position"
    );

    let stream = client.consumer_topic().key();
    broker
        .publisher_unscoped()
        .await
        .unwrap()
        .publish(&stream, &sample_block_payload(BlockFlow::Live, 1, 42))
        .await
        .unwrap();

    let after_publish = client.write_positions().await.unwrap();
    assert_ne!(
        after_publish, baseline,
        "publishing must move the position, or a live stream would look quiet"
    );

    trim_stream_empty(&url, &stream).await;

    assert_eq!(
        client.write_positions().await.unwrap(),
        after_publish,
        "trimming must not move the position back: length shrinks, write position does not"
    );
}

/// Publish `count` blocks to a stream, numbered from 1.
async fn publish_blocks(broker: &Broker, stream: &str, count: u64) {
    let publisher = broker.publisher_unscoped().await.unwrap();
    for block_number in 1..=count {
        let payload = sample_block_payload(BlockFlow::Live, 1, block_number);
        publisher.publish(stream, &payload).await.unwrap();
    }
}

/// One named group's reported status, or `None` if it does not exist.
async fn group_named(client: &ListenerConsumer, stream: &str, group: &str) -> Option<GroupStatus> {
    client
        .group_status()
        .await
        .unwrap()
        .into_iter()
        .find(|(key, _)| key == stream)
        .and_then(|(_, groups)| groups.into_iter().find(|candidate| candidate.name == group))
}

/// A group arriving on a stream a predecessor is already reading takes over
/// from where that predecessor is, instead of replaying the whole backlog.
///
/// This is the case a version cutover creates: the suffixed group is new, but
/// the stream under it is not, and it may be holding a great deal of history —
/// a stream whose trimming is pinned by an abandoned group retains everything
/// up to its ceiling. Starting at the beginning would re-deliver all of it.
/// The predecessor's cursor is the honest answer: behind it is work already
/// done by a reader writing to the same database, ahead of it is work nobody
/// has taken.
#[tokio::test]
#[ignore = "requires Docker"]
async fn a_new_group_takes_over_from_its_predecessor_instead_of_replaying() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer_id = unique_name("ladder-takeover");
    let legacy = ListenerConsumer::new(&broker, 1, &consumer_id);
    let current = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v2");
    let stream = legacy.consumer_topic().key();

    // The predecessor reads three blocks and gets its cursor to the end.
    legacy.ensure_consumer().await.unwrap();
    let legacy_handle = start_live_consumer(&url, &legacy).await;
    publish_blocks(&broker, &stream, 3).await;

    // Wait for it to drain. Nothing else is published, so once its lag is zero
    // its cursor is at the end of the stream and cannot move again — otherwise
    // the cursor read here and the one the successor seeds from are two
    // different moments and the comparison below means nothing. Waiting on the
    // pending list too is what pins this test to the caught-up case: a
    // predecessor still holding entries seeds its successor somewhere else
    // entirely, which is the test below this one.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let legacy_cursor = loop {
        match group_named(&current, &stream, &stream).await {
            Some(group)
                if group.lag == Some(0)
                    && group.pending == 0
                    && group.last_delivered_id != "0-0" =>
            {
                break group.last_delivered_id;
            }
            other => assert!(
                tokio::time::Instant::now() < deadline,
                "predecessor never drained the stream: {other:?}"
            ),
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    };

    legacy.cancel();
    legacy_handle.abort();

    // The successor arrives on the populated stream.
    current.ensure_consumer().await.unwrap();

    let successor = group_named(&current, &stream, &format!("{stream}.v2"))
        .await
        .expect("the suffixed group must have been created");

    assert_eq!(
        successor.last_delivered_id, legacy_cursor,
        "the new group must start at its predecessor's cursor, not at the beginning"
    );
    assert_eq!(
        successor.lag,
        Some(0),
        "starting at the predecessor's cursor means nothing is waiting to be replayed"
    );

    // And it still receives what arrives after it.
    let seen = Arc::new(Mutex::new(Vec::<u64>::new()));
    let slot = seen.clone();
    let handle = tokio::spawn(current.clone().consume(move |p: BlockPayload, _cancel| {
        let slot = slot.clone();
        async move {
            slot.lock().unwrap().push(p.block_number);
            Ok(AckDecision::Ack)
        }
    }));

    let payload = sample_block_payload(BlockFlow::Live, 1, 99);
    broker
        .publisher_unscoped()
        .await
        .unwrap()
        .publish(&stream, &payload)
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while seen.lock().unwrap().is_empty() && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    current.cancel();
    handle.abort();

    assert_eq!(
        *seen.lock().unwrap(),
        vec![99],
        "the successor must receive new blocks, and only those"
    );
}

/// Deliver entries to a group and acknowledge none of them, which is the state
/// a consumer killed mid-batch leaves its group in.
async fn deliver_without_ack(url: &str, stream: &str, group: &str, count: usize) {
    let client = redis::Client::open(url).unwrap();
    let mut conn = client.get_multiplexed_async_connection().await.unwrap();
    let _: redis::Value = redis::cmd("XREADGROUP")
        .arg("GROUP")
        .arg(group)
        .arg("killed-consumer")
        .arg("COUNT")
        .arg(count)
        .arg("STREAMS")
        .arg(stream)
        .arg(">")
        .query_async(&mut conn)
        .await
        .unwrap();
}

/// The entry IDs in an `XREADGROUP` reply, which is nested four deep:
/// streams, then this stream's entries, then each entry's id and fields.
fn ids_in_read_reply(reply: &redis::Value) -> Vec<String> {
    let redis::Value::Array(streams) = reply else {
        return Vec::new();
    };
    let mut ids = Vec::new();
    for stream in streams {
        let redis::Value::Array(parts) = stream else {
            continue;
        };
        let Some(redis::Value::Array(entries)) = parts.get(1) else {
            continue;
        };
        for entry in entries {
            let redis::Value::Array(fields) = entry else {
                continue;
            };
            if let Some(redis::Value::BulkString(id)) = fields.first() {
                ids.push(String::from_utf8_lossy(id).into_owned());
            }
        }
    }
    ids
}

/// Deliver `count` entries to a group and acknowledge every one, which parks
/// its cursor `count` entries further on with nothing left outstanding.
///
/// Acknowledging exactly what was handed over — rather than the first `count`
/// entries on the stream — is what lets this be called on a group that was
/// itself seeded partway along.
async fn deliver_and_ack(url: &str, stream: &str, group: &str, count: usize) {
    let client = redis::Client::open(url).unwrap();
    let mut conn = client.get_multiplexed_async_connection().await.unwrap();

    let reply: redis::Value = redis::cmd("XREADGROUP")
        .arg("GROUP")
        .arg(group)
        .arg("acking-consumer")
        .arg("COUNT")
        .arg(count)
        .arg("STREAMS")
        .arg(stream)
        .arg(">")
        .query_async(&mut conn)
        .await
        .unwrap();

    let ids = ids_in_read_reply(&reply);
    assert_eq!(
        ids.len(),
        count,
        "the stream must hold {count} undelivered entries for this group"
    );

    let mut ack = redis::cmd("XACK");
    ack.arg(stream).arg(group);
    for id in &ids {
        ack.arg(id);
    }
    let acked: u64 = ack.query_async(&mut conn).await.unwrap();
    assert_eq!(acked as usize, count, "every delivered entry must be acked");
}

/// Consume `client`'s live stream until `count` blocks arrive or ten seconds
/// pass, then stop and return the block numbers seen, in order.
///
/// This is the assertion of choice for a seeded group. `XINFO GROUPS` reports
/// a null lag for a group created at an arbitrary mid-stream ID — Redis only
/// derives it exactly at the boundaries — and the delivered blocks are the
/// thing being tested in any case.
async fn drain_live(client: &ListenerConsumer, count: usize) -> Vec<u64> {
    let seen = Arc::new(Mutex::new(Vec::<u64>::new()));
    let slot = seen.clone();
    let handle = tokio::spawn(client.clone().consume(move |p: BlockPayload, _cancel| {
        let slot = slot.clone();
        async move {
            slot.lock().unwrap().push(p.block_number);
            Ok(AckDecision::Ack)
        }
    }));

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while seen.lock().unwrap().len() < count && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    // Keep reading briefly past the target so that delivering too much fails
    // here rather than passing unnoticed.
    tokio::time::sleep(Duration::from_millis(250)).await;

    client.cancel();
    handle.abort();
    seen.lock().unwrap().clone()
}

/// A group arriving on a stream whose predecessor left work outstanding starts
/// at the oldest entry that predecessor never finished, not at its cursor.
///
/// A cursor records what a group was handed, not what it completed. Nothing
/// reclaims a dead consumer's pending entries and the host listener takes no
/// part in its own shutdown, so whatever was in flight when a pod was replaced
/// stays outstanding for good. A successor inheriting the cursor steps over
/// that work and no reader ever comes back for it. Re-reading a few entries
/// into the same database is the cheaper mistake.
#[tokio::test]
#[ignore = "requires Docker"]
async fn a_new_group_picks_up_what_its_predecessor_left_unfinished() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer_id = unique_name("ladder-pending");
    let legacy = ListenerConsumer::new(&broker, 1, &consumer_id);
    let current = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v2");
    let stream = legacy.consumer_topic().key();

    legacy.ensure_consumer().await.unwrap();
    publish_blocks(&broker, &stream, 5).await;

    // No live consumer here on purpose: the five entries are handed over and
    // never acknowledged, so the cursor reaches the end of the stream while
    // none of the work behind it is done.
    deliver_without_ack(&url, &stream, &stream, 5).await;

    let predecessor = group_named(&current, &stream, &stream)
        .await
        .expect("the predecessor group must exist");
    assert_eq!(
        predecessor.pending, 5,
        "the predecessor must be holding all five entries"
    );
    assert_eq!(
        predecessor.lag,
        Some(0),
        "its cursor must be at the end of the stream, which is what makes \
         inheriting the cursor the wrong answer here"
    );

    current.ensure_consumer().await.unwrap();

    let successor = group_named(&current, &stream, &format!("{stream}.v2"))
        .await
        .expect("the suffixed group must have been created");

    assert_eq!(
        successor.lag,
        Some(5),
        "the successor must start behind all five unfinished entries, not at \
         the predecessor's cursor"
    );
}

/// With nothing older reading a populated stream, a new group starts at the
/// end.
///
/// There is no cursor to inherit and no way to tell how old the retained
/// entries are, so the tip is the floor: re-ingesting an unknown quantity of
/// history into a fresh database is the worse of the two mistakes. It is a
/// floor and not a recovery — this consumer wires no catchup flow — which is
/// why the case below, where there *is* something to inherit from, matters.
#[tokio::test]
#[ignore = "requires Docker"]
async fn a_first_group_on_an_unread_stream_starts_at_the_end() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let client =
        ListenerConsumer::new(&broker, 1, &unique_name("ladder-fresh")).with_group_suffix("v2");
    let stream = client.consumer_topic().key();

    // Blocks pile up on a stream nobody has ever read.
    publish_blocks(&broker, &stream, 5).await;
    assert!(
        redis_group_names(&url, &stream).await.is_empty(),
        "no group should exist yet"
    );

    client.ensure_consumer().await.unwrap();

    let groups = client
        .group_status()
        .await
        .unwrap()
        .into_iter()
        .find(|(key, _)| key == &stream)
        .map(|(_, groups)| groups)
        .unwrap();
    let group = groups
        .iter()
        .find(|group| group.name == format!("{stream}.v2"))
        .expect("the suffixed group must have been created");

    assert_eq!(
        group.lag,
        Some(0),
        "a group starting at the end has no backlog waiting for it, \
         but one starting at the beginning would report all five"
    );
}

/// A fresh identity starts at the end of a stream that does not exist yet, and
/// misses nothing by doing so.
///
/// This is the shape of an identity change: the new consumer id names streams
/// nobody has ever published to, so there is no predecessor to find and the
/// end of the stream is also its beginning. Continuity across that kind of
/// change comes from both identities being published to at once, not from a
/// cursor — which is why starting at the tip here is not a gap.
#[tokio::test]
#[ignore = "requires Docker"]
async fn a_fresh_identity_starts_at_the_tip_and_misses_nothing() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let client =
        ListenerConsumer::new(&broker, 1, &unique_name("ladder-newid")).with_group_suffix("v2");
    let stream = client.consumer_topic().key();

    // Nothing has ever been published under this identity.
    assert!(
        redis_group_names(&url, &stream).await.is_empty(),
        "a fresh identity must start with no groups"
    );

    client.ensure_consumer().await.unwrap();
    publish_blocks(&broker, &stream, 3).await;

    assert_eq!(
        drain_live(&client, 3).await,
        vec![1, 2, 3],
        "the group was created before anything was published, so the tip it \
         started at was the beginning and every block arrived"
    );
}

/// A suffixed predecessor is inherited from, not stepped over.
///
/// Both builds carry a group suffix whenever they disagree about the protocol
/// version, so at every cutover after the first the predecessor's group has a
/// suffix too. Matching only the unsuffixed name would find nothing here and
/// start at the tip, silently skipping everything the predecessor had not
/// reached.
#[tokio::test]
#[ignore = "requires Docker"]
async fn a_suffixed_predecessor_is_inherited_from() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer_id = unique_name("ladder-suffixed");
    let blue = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v1");
    let green = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v2");
    let stream = blue.consumer_topic().key();

    // The predecessor reads two of five and finishes both, leaving its cursor
    // short of the end with nothing outstanding. Being *behind* is what makes
    // this decisive: a predecessor that had caught up would be indistinguishable
    // from starting at the tip.
    blue.ensure_consumer().await.unwrap();
    publish_blocks(&broker, &stream, 5).await;
    deliver_and_ack(&url, &stream, &format!("{stream}.v1"), 2).await;

    let predecessor = group_named(&green, &stream, &format!("{stream}.v1"))
        .await
        .expect("the predecessor group must exist");
    assert_eq!(predecessor.pending, 0, "it finished what it was handed");
    assert_eq!(
        predecessor.lag,
        Some(3),
        "and it is three entries short of the end"
    );

    green.ensure_consumer().await.unwrap();

    let successor = group_named(&green, &stream, &format!("{stream}.v2"))
        .await
        .expect("the suffixed successor must have been created");
    assert_eq!(
        successor.last_delivered_id, predecessor.last_delivered_id,
        "the successor must start at its suffixed predecessor's cursor"
    );

    assert_eq!(
        drain_live(&green, 3).await,
        vec![3, 4, 5],
        "the successor must receive exactly the blocks its predecessor never \
         reached; starting at the tip would deliver none of them"
    );
}

/// Each rung inherits from the one below it, every time, not only the first.
///
/// The first cutover has an unsuffixed group to inherit from and the ones after
/// it do not. Walking three rungs is what separates a mechanism that works from
/// one that works once: `v2` must take over from `v1`, and `v3` from `v2` and
/// not from `v1`.
#[tokio::test]
#[ignore = "requires Docker"]
async fn every_rung_inherits_from_the_one_below_it() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer_id = unique_name("ladder-walk");
    let v1 = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v1");
    let v2 = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v2");
    let v3 = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v3");
    let stream = v1.consumer_topic().key();

    v1.ensure_consumer().await.unwrap();
    publish_blocks(&broker, &stream, 10).await;

    // Rung one gets through three of ten.
    deliver_and_ack(&url, &stream, &format!("{stream}.v1"), 3).await;

    // Rung two takes over there and gets through three more. That this call
    // finds three undelivered entries at all is the first half of the proof:
    // a group started at the tip would have none to hand over.
    v2.ensure_consumer().await.unwrap();
    deliver_and_ack(&url, &stream, &format!("{stream}.v2"), 3).await;

    // Rung three must take over from rung two, which is the nearest below it.
    v3.ensure_consumer().await.unwrap();

    assert_eq!(
        drain_live(&v3, 4).await,
        vec![7, 8, 9, 10],
        "v3 must pick up where v2 stopped. Starting from v1 would replay \
         blocks four through six, and starting at the tip would deliver none"
    );
}

/// A newer group on the same stream is never treated as a predecessor.
///
/// During a cutover the successor is reading the same stream as the stack being
/// replaced, so "another group is here" is not the same question as "something
/// older is here". Inheriting forwards would hand a restarting predecessor its
/// successor's position and skip whatever sits between them.
#[tokio::test]
#[ignore = "requires Docker"]
async fn a_newer_group_is_never_treated_as_a_predecessor() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer_id = unique_name("ladder-newer");
    let legacy = ListenerConsumer::new(&broker, 1, &consumer_id);
    let v2 = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v2");
    let v3 = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v3");
    let stream = legacy.consumer_topic().key();

    // Two groups are already here, one on either side of the one being created:
    // the unsuffixed one a single entry in, and a newer one much further along.
    legacy.ensure_consumer().await.unwrap();
    v3.ensure_consumer().await.unwrap();
    publish_blocks(&broker, &stream, 5).await;
    deliver_and_ack(&url, &stream, &stream, 1).await;
    deliver_and_ack(&url, &stream, &format!("{stream}.v3"), 4).await;

    v2.ensure_consumer().await.unwrap();

    assert_eq!(
        drain_live(&v2, 4).await,
        vec![2, 3, 4, 5],
        "v2 must inherit from the unsuffixed group below it. Taking v3's \
         position instead would deliver only block five"
    );
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

/// Ensure all four groups for `client`, so the whole identity is represented
/// rather than only the live flow.
async fn ensure_all_four(client: &ListenerConsumer) {
    client.ensure_consumer().await.unwrap();
    client.ensure_catchup_consumer().await.unwrap();
    client.ensure_final_consumer().await.unwrap();
    client.ensure_final_catchup_consumer().await.unwrap();
}

/// A retired build takes its own four groups with it and leaves everything else
/// standing.
///
/// After a cutover the outgoing build writes nothing, so its cursors are dead
/// weight: once its pods go they stop moving and the trimmer will not reclaim
/// past them. It can clean up after itself, but only its own side. The two
/// builds share one identity and therefore the same four streams, so the
/// entries and the incoming build's groups have to survive untouched.
///
/// The negative half is the real assertion. Destroying by stream key, or
/// reaching for `delete_streams`, would also pass a test that only checked that
/// the retired build's groups were gone.
#[tokio::test]
#[ignore = "requires Docker"]
async fn a_retired_build_destroys_its_own_groups_and_leaves_the_rest() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer_id = unique_name("retire-consumer");
    let blue = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v2");
    let green = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v3");

    ensure_all_four(&blue).await;
    ensure_all_four(&green).await;

    // A backlog the surviving build has not read yet. Taking the stream, or the
    // entries on it, would show up as the survivor's lag collapsing.
    let live = blue.consumer_topic().key();
    publish_blocks(&broker, &live, 3).await;

    let topics = [
        blue.consumer_topic(),
        blue.catchup_consumer_topic(),
        blue.final_consumer_topic(),
        blue.final_catchup_consumer_topic(),
    ];

    for topic in &topics {
        let key = topic.key();
        let mut names = redis_group_names(&url, &key).await;
        names.sort();
        assert_eq!(
            names,
            vec![format!("{key}.v2"), format!("{key}.v3")],
            "setup: both builds must hold a group on {key} before the retirement"
        );
    }

    let destroyed = blue.destroy_own_groups().await.unwrap();
    assert_eq!(destroyed, 4, "the retired build owns one group per stream");

    for topic in &topics {
        let key = topic.key();
        assert_eq!(
            redis_group_names(&url, &key).await,
            vec![format!("{key}.v3")],
            "the surviving build's group must be the only one left on {key}"
        );
        assert!(
            broker.exists(topic).await.unwrap(),
            "{key} is shared with the surviving build and must not be deleted"
        );
    }

    let survivor = group_named(&green, &live, &format!("{live}.v3"))
        .await
        .expect("the surviving build's group is still there");
    assert_eq!(
        survivor.lag,
        Some(3),
        "the backlog the survivor has not read must be exactly where it was"
    );

    // A restarted pod re-running this must converge rather than error.
    assert_eq!(
        blue.destroy_own_groups().await.unwrap(),
        0,
        "a second pass has nothing left to destroy"
    );

    // Without a suffix a client's "own" group is the bare legacy name, which a
    // predecessor sharing these streams may still be reading.
    let unsuffixed = ListenerConsumer::new(&broker, 1, &consumer_id);
    assert!(
        unsuffixed.destroy_own_groups().await.is_err(),
        "a client with no suffix must refuse: its own group is the legacy one"
    );
}

/// The live build sweeps the versioned groups below its own and nothing else.
///
/// This is the backstop for a retired build that never got to release its own
/// groups. Every other group on the stream is someone it must not touch: the
/// unsuffixed one belongs to a build that predates versioning and may still be
/// reading, its own is in use, and a newer one is a shadow stack on its way in.
#[tokio::test]
#[ignore = "requires Docker"]
async fn the_live_build_sweeps_only_the_versions_below_it() {
    let _guard = REDIS_TEST_LOCK.lock().await;
    let url = consumer_redis_url().await;
    reset_redis(&url).await;
    let broker = Broker::redis(&url).await.unwrap();

    let consumer_id = unique_name("sweep-consumer");
    let legacy = ListenerConsumer::new(&broker, 1, &consumer_id);
    let oldest = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v1");
    let retired = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v2");
    let live = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v3");
    let shadow = ListenerConsumer::new(&broker, 1, &consumer_id).with_group_suffix("v4");
    for client in [&legacy, &oldest, &retired, &live, &shadow] {
        ensure_all_four(client).await;
    }

    let live_stream = live.consumer_topic().key();
    publish_blocks(&broker, &live_stream, 3).await;

    let topics = [
        live.consumer_topic(),
        live.catchup_consumer_topic(),
        live.final_consumer_topic(),
        live.final_catchup_consumer_topic(),
    ];

    let destroyed = live.destroy_superseded_groups().await.unwrap();
    assert_eq!(destroyed, 8, "v1 and v2 hold one group per stream");

    for topic in &topics {
        let key = topic.key();
        let mut names = redis_group_names(&url, &key).await;
        names.sort();
        assert_eq!(
            names,
            vec![key.clone(), format!("{key}.v3"), format!("{key}.v4")],
            "only the versioned groups below v3 may go from {key}"
        );
        assert!(
            broker.exists(topic).await.unwrap(),
            "{key} is still read and must not be deleted"
        );
    }

    let survivor = group_named(&live, &live_stream, &format!("{live_stream}.v3"))
        .await
        .expect("the live build's group is still there");
    assert_eq!(
        survivor.lag,
        Some(3),
        "the live build's backlog must be exactly where it was"
    );

    assert_eq!(
        live.destroy_superseded_groups().await.unwrap(),
        0,
        "a second pass has nothing left to destroy"
    );
    assert_eq!(
        legacy.destroy_superseded_groups().await.unwrap(),
        0,
        "a client with no suffix cannot rank anything below itself"
    );
}
