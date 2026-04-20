#![cfg(feature = "redis")]
//! End-to-end integration tests for the `Broker` **facade** API.
//!
//! Unlike `redis_e2e.rs` which constructs backend-specific types directly
//! (`RedisPublisher`, `RedisConsumer`, `RedisConsumerConfigBuilder`), these
//! tests exercise the high-level `Broker → Publisher / ConsumerBuilder`
//! workflow — the API that application code should use.
//!
//! Run via:
//!
//! ```bash
//! make test-e2e-broker
//! ```

use std::{
    collections::HashSet,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU32, Ordering},
    },
    time::Duration,
};

use broker::{
    AckDecision, AsyncHandlerPayloadClassified, AsyncHandlerPayloadOnly, Broker, HandlerError,
    Message, Topic,
};
use test_support::shared_redis_url;

// ── Shared payload types ─────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
struct BlockEvent {
    block_number: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ListenerEvent {
    Block {
        chain_id: u64,
        block_number: u64,
    },
    WatchRegister {
        chain_id: u64,
        consumer_id: String,
        contract_addresses: Vec<String>,
    },
}

// ── Redis assertion helpers ──────────────────────────────────────────────────

async fn redis_xlen(url: &str, stream: &str) -> u64 {
    let client = redis::Client::open(url).unwrap();
    let mut conn = client.get_multiplexed_async_connection().await.unwrap();
    redis::cmd("XLEN")
        .arg(stream)
        .query_async(&mut conn)
        .await
        .unwrap_or(0)
}

// ── Test 1: Simple roundtrip via Broker facade ──────────────────────────────

#[tokio::test]
#[ignore = "requires Docker"]
async fn test_broker_simple_roundtrip() {
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();

    let publisher = broker.publisher("bf-simple").await.unwrap();
    let topic = Topic::namespaced("bf-simple", "events");

    let count = Arc::new(AtomicU32::new(0));
    let count_clone = count.clone();
    let handler = AsyncHandlerPayloadOnly::new(move |_: BlockEvent| {
        let c = count_clone.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            Ok::<(), std::io::Error>(())
        }
    });

    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("bf-simple-group")
            .consumer_name("consumer-1")
            .prefetch(10)
            .run(handler)
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    publisher
        .publish("events", &BlockEvent { block_number: 42 })
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while count.load(Ordering::SeqCst) == 0 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    publisher.shutdown().await;
    consumer_handle.abort();
    assert_eq!(
        count.load(Ordering::SeqCst),
        1,
        "handler should be called exactly once"
    );
}

// ── Test 2: Multi-publish roundtrip ─────────────────────────────────────────

#[tokio::test]
#[ignore = "requires Docker"]
async fn test_broker_multi_publish_roundtrip() {
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();

    let publisher = broker.publisher("bf-batch").await.unwrap();
    let topic = Topic::namespaced("bf-batch", "events");

    let count = Arc::new(AtomicU32::new(0));
    let count_clone = count.clone();
    let handler = AsyncHandlerPayloadOnly::new(move |_: BlockEvent| {
        let c = count_clone.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            Ok::<(), std::io::Error>(())
        }
    });

    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("bf-batch-group")
            .consumer_name("consumer-1")
            .prefetch(10)
            .run(handler)
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    let events = vec![
        BlockEvent { block_number: 1 },
        BlockEvent { block_number: 2 },
        BlockEvent { block_number: 3 },
    ];
    publisher.publish_batch("events", &events).await.unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    while count.load(Ordering::SeqCst) < 3 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    publisher.shutdown().await;
    consumer_handle.abort();
    assert_eq!(
        count.load(Ordering::SeqCst),
        3,
        "all 3 batch messages should be received"
    );
}

// ── Test 3: Retry then succeed ──────────────────────────────────────────────

/// Handler fails twice, succeeds on third delivery via claim sweeper retry.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_broker_retry_eventually_succeeds() {
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();

    let publisher = broker.publisher("bf-retry").await.unwrap();
    let topic = Topic::namespaced("bf-retry", "events");

    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = call_count.clone();
    let handler = AsyncHandlerPayloadOnly::new(move |_: BlockEvent| {
        let c = call_count_clone.clone();
        async move {
            let prev = c.fetch_add(1, Ordering::SeqCst);
            if prev < 2 {
                Err(std::io::Error::other("simulated failure"))
            } else {
                Ok(())
            }
        }
    });

    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("bf-retry-group")
            .consumer_name("consumer-1")
            .prefetch(10)
            .max_retries(5)
            .redis_claim_min_idle(1)
            .redis_claim_interval(1)
            .run(handler)
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    publisher
        .publish("events", &BlockEvent { block_number: 1 })
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    while call_count.load(Ordering::SeqCst) < 3 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    publisher.shutdown().await;
    consumer_handle.abort();
    assert!(
        call_count.load(Ordering::SeqCst) >= 3,
        "handler should be called at least 3 times (2 failures + 1 success), got {}",
        call_count.load(Ordering::SeqCst)
    );
}

// ── Test 4: DLQ after max_retries ───────────────────────────────────────────

/// Handler always fails → message moves to dead stream after max_retries.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_broker_dlq_after_max_retries() {
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();

    let publisher = broker.publisher("bf-dlq").await.unwrap();
    let topic = Topic::namespaced("bf-dlq", "events");
    let dead_stream = topic.dead_key().to_owned();

    let handler = AsyncHandlerPayloadOnly::new(|_: BlockEvent| async move {
        Err::<(), _>(std::io::Error::other("always fails"))
    });

    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("bf-dlq-group")
            .consumer_name("consumer-1")
            .prefetch(10)
            .max_retries(2)
            .redis_claim_min_idle(1)
            .redis_claim_interval(1)
            .run(handler)
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    publisher
        .publish("events", &BlockEvent { block_number: 99 })
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;

        let len = redis_xlen(url, &dead_stream).await;
        if len >= 1 {
            publisher.shutdown().await;
            consumer_handle.abort();
            assert_eq!(len, 1, "exactly one message should be in the dead stream");
            return;
        }

        if tokio::time::Instant::now() >= deadline {
            publisher.shutdown().await;
            consumer_handle.abort();
            panic!("message was not moved to dead stream within timeout");
        }
    }
}

// ── Test 5: AckDecision::Dead — immediate dead-letter ───────────────────────

/// Handler returns `AckDecision::Dead` on first delivery → message goes to
/// dead stream immediately without waiting for claim sweeper or max_retries.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_broker_ack_decision_dead_immediate() {
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();

    let publisher = broker.publisher("bf-dead").await.unwrap();
    let topic = Topic::namespaced("bf-dead", "events");
    let dead_stream = topic.dead_key().to_owned();

    let call_count = Arc::new(AtomicU32::new(0));

    #[derive(Clone)]
    struct DeadHandler(Arc<AtomicU32>);

    #[async_trait::async_trait]
    impl broker::Handler for DeadHandler {
        async fn call(&self, _msg: &Message) -> Result<AckDecision, HandlerError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(AckDecision::Dead)
        }
    }

    let handler = DeadHandler(call_count.clone());

    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("bf-dead-group")
            .consumer_name("consumer-1")
            .prefetch(10)
            .max_retries(10)
            // Long claim_min_idle — message must NOT need the sweeper to route to DLQ
            .redis_claim_min_idle(60)
            .redis_claim_interval(15)
            .run(handler)
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    publisher
        .publish("events", &BlockEvent { block_number: 42 })
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    loop {
        tokio::time::sleep(Duration::from_millis(200)).await;

        let len = redis_xlen(url, &dead_stream).await;
        if len >= 1 {
            // Extra time to confirm no spurious retries
            tokio::time::sleep(Duration::from_millis(800)).await;
            publisher.shutdown().await;
            consumer_handle.abort();

            assert_eq!(
                call_count.load(Ordering::SeqCst),
                1,
                "handler should be called exactly once — AckDecision::Dead must not retry"
            );
            assert_eq!(len, 1, "exactly one message should be in the dead stream");
            return;
        }

        if tokio::time::Instant::now() >= deadline {
            publisher.shutdown().await;
            consumer_handle.abort();
            panic!(
                "message was not moved to dead stream within timeout. \
                 handler_calls={}, dead_stream_len={}",
                call_count.load(Ordering::SeqCst),
                len
            );
        }
    }
}

// ── Test 6: Circuit breaker halts then recovers ─────────────────────────────

/// Transient failures trip the circuit breaker; after cooldown the consumer
/// resumes and processes a signal message.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_broker_circuit_breaker_halts_consumption() {
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();

    let publisher = broker.publisher("bf-cb").await.unwrap();
    let topic = Topic::namespaced("bf-cb", "events");

    let transient_count = Arc::new(AtomicU32::new(0));
    let signal_received = Arc::new(AtomicBool::new(false));

    #[derive(Clone)]
    struct TrippingHandler {
        transient_count: Arc<AtomicU32>,
        signal_received: Arc<AtomicBool>,
    }

    #[async_trait::async_trait]
    impl broker::Handler for TrippingHandler {
        async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
            if let Ok(event) = serde_json::from_slice::<BlockEvent>(&msg.payload)
                && event.block_number == 99
            {
                self.signal_received.store(true, Ordering::SeqCst);
                return Ok(AckDecision::Ack);
            }
            let prev = self.transient_count.fetch_add(1, Ordering::SeqCst);
            if prev < 3 {
                Err(HandlerError::Transient(Box::new(std::io::Error::other(
                    "simulated infrastructure failure",
                ))))
            } else {
                Ok(AckDecision::Ack)
            }
        }
    }

    let handler = TrippingHandler {
        transient_count: transient_count.clone(),
        signal_received: signal_received.clone(),
    };

    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("bf-cb-group")
            .consumer_name("consumer-1")
            .prefetch(10)
            .max_retries(10)
            .circuit_breaker(3, Duration::from_millis(2000))
            // Long claim_min_idle keeps sweeper out of test window
            .redis_claim_min_idle(60)
            .redis_claim_interval(10)
            .run(handler)
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    // Publish 3 trip messages to open the circuit
    for i in 0..3u64 {
        publisher
            .publish("events", &BlockEvent { block_number: i })
            .await
            .unwrap();
    }

    // Wait until at least 3 transient failures recorded
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    while transient_count.load(Ordering::SeqCst) < 3 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(
        transient_count.load(Ordering::SeqCst) >= 3,
        "expected at least 3 transient failures to trip the circuit"
    );

    // Circuit is Open — signal must NOT be consumed during cooldown
    publisher
        .publish("events", &BlockEvent { block_number: 99 })
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(1000)).await;
    assert!(
        !signal_received.load(Ordering::SeqCst),
        "signal message must not be consumed while the circuit is Open"
    );

    // After cooldown: PEL drained → Half-Open → signal consumed
    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    while !signal_received.load(Ordering::SeqCst) && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    publisher.shutdown().await;
    consumer_handle.abort();
    assert!(
        signal_received.load(Ordering::SeqCst),
        "signal message must be consumed after the CB cooldown expires"
    );
}

// ── Test 7: Multichain namespace isolation via Broker facade ────────────────

/// Two chains (ethereum, polygon) with separate namespaces. Multiple consumer
/// groups on the same stream (fanout) and separate streams (routing).
/// Validates that the Broker facade correctly isolates namespaces.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_broker_multichain_namespace_isolation() {
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();

    // Namespace-scoped publishers
    let eth_publisher = broker.publisher("bf-mc-eth").await.unwrap();
    let polygon_publisher = broker.publisher("bf-mc-polygon").await.unwrap();

    // Topics
    let eth_blocks_topic = Topic::namespaced("bf-mc-eth", "blocks");
    let polygon_blocks_topic = Topic::namespaced("bf-mc-polygon", "blocks");
    let eth_control_topic = Topic::namespaced("bf-mc-eth", "control");

    // Counters
    let app_a_eth = Arc::new(AtomicU32::new(0));
    let app_a_eth_numbers = Arc::new(Mutex::new(HashSet::<u64>::new()));
    let app_b_eth = Arc::new(AtomicU32::new(0));
    let app_b_eth_numbers = Arc::new(Mutex::new(HashSet::<u64>::new()));
    let app_a_polygon = Arc::new(AtomicU32::new(0));
    let app_a_polygon_numbers = Arc::new(Mutex::new(HashSet::<u64>::new()));
    let watch_count = Arc::new(AtomicU32::new(0));
    let watch_received = Arc::new(Mutex::new(None::<ListenerEvent>));

    // App A — ETH blocks
    let app_a_eth_handler = {
        let count = app_a_eth.clone();
        let numbers = app_a_eth_numbers.clone();
        AsyncHandlerPayloadOnly::new(move |event: ListenerEvent| {
            let count = count.clone();
            let numbers = numbers.clone();
            async move {
                if let ListenerEvent::Block { block_number, .. } = event {
                    count.fetch_add(1, Ordering::SeqCst);
                    numbers.lock().unwrap().insert(block_number);
                }
                Ok::<(), std::convert::Infallible>(())
            }
        })
    };

    // App B — ETH blocks (different group = fanout)
    let app_b_eth_handler = {
        let count = app_b_eth.clone();
        let numbers = app_b_eth_numbers.clone();
        AsyncHandlerPayloadOnly::new(move |event: ListenerEvent| {
            let count = count.clone();
            let numbers = numbers.clone();
            async move {
                if let ListenerEvent::Block { block_number, .. } = event {
                    count.fetch_add(1, Ordering::SeqCst);
                    numbers.lock().unwrap().insert(block_number);
                }
                Ok::<(), std::convert::Infallible>(())
            }
        })
    };

    // App A — Polygon blocks
    let app_a_polygon_handler = {
        let count = app_a_polygon.clone();
        let numbers = app_a_polygon_numbers.clone();
        AsyncHandlerPayloadOnly::new(move |event: ListenerEvent| {
            let count = count.clone();
            let numbers = numbers.clone();
            async move {
                if let ListenerEvent::Block { block_number, .. } = event {
                    count.fetch_add(1, Ordering::SeqCst);
                    numbers.lock().unwrap().insert(block_number);
                }
                Ok::<(), std::convert::Infallible>(())
            }
        })
    };

    // Watch handler — ETH control channel
    let watch_handler = {
        let count = watch_count.clone();
        let received = watch_received.clone();
        AsyncHandlerPayloadOnly::new(move |event: ListenerEvent| {
            let count = count.clone();
            let received = received.clone();
            async move {
                if let ListenerEvent::WatchRegister { .. } = &event {
                    count.fetch_add(1, Ordering::SeqCst);
                    let mut r = received.lock().unwrap();
                    if r.is_none() {
                        *r = Some(event);
                    }
                }
                Ok::<(), std::convert::Infallible>(())
            }
        })
    };

    // Spawn consumers
    let b = broker.clone();
    let t = eth_blocks_topic.clone();
    let h1 = tokio::spawn(async move {
        let _ = b
            .consumer(&t)
            .group("bf-mc-app-a")
            .consumer_name("pod-1")
            .prefetch(32)
            .redis_block_ms(1000)
            .run(app_a_eth_handler)
            .await;
    });

    let b = broker.clone();
    let t = eth_blocks_topic.clone();
    let h2 = tokio::spawn(async move {
        let _ = b
            .consumer(&t)
            .group("bf-mc-app-b")
            .consumer_name("pod-1")
            .prefetch(32)
            .redis_block_ms(1000)
            .run(app_b_eth_handler)
            .await;
    });

    let b = broker.clone();
    let t = polygon_blocks_topic.clone();
    let h3 = tokio::spawn(async move {
        let _ = b
            .consumer(&t)
            .group("bf-mc-app-a-polygon")
            .consumer_name("pod-1")
            .prefetch(32)
            .redis_block_ms(1000)
            .run(app_a_polygon_handler)
            .await;
    });

    let b = broker.clone();
    let t = eth_control_topic.clone();
    let h4 = tokio::spawn(async move {
        let _ = b
            .consumer(&t)
            .group("bf-mc-listener")
            .consumer_name("pod-1")
            .prefetch(16)
            .redis_block_ms(1000)
            .run(watch_handler)
            .await;
    });

    tokio::time::sleep(Duration::from_millis(1200)).await;

    // Publish control message
    eth_publisher
        .publish(
            "control",
            &ListenerEvent::WatchRegister {
                chain_id: 1,
                consumer_id: "app-a".to_string(),
                contract_addresses: vec!["0xabc".to_string()],
            },
        )
        .await
        .unwrap();

    let eth_count: u32 = 20;
    let polygon_count: u32 = 10;

    // Publish block events
    for i in 0..u64::from(eth_count) {
        eth_publisher
            .publish(
                "blocks",
                &ListenerEvent::Block {
                    chain_id: 1,
                    block_number: 1_000_000 + i,
                },
            )
            .await
            .unwrap();
    }
    for i in 0..u64::from(polygon_count) {
        polygon_publisher
            .publish(
                "blocks",
                &ListenerEvent::Block {
                    chain_id: 137,
                    block_number: 2_000_000 + i,
                },
            )
            .await
            .unwrap();
    }

    // Wait for all consumers to process
    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    while tokio::time::Instant::now() < deadline {
        let done = app_a_eth.load(Ordering::SeqCst) == eth_count
            && app_b_eth.load(Ordering::SeqCst) == eth_count
            && app_a_polygon.load(Ordering::SeqCst) == polygon_count
            && watch_count.load(Ordering::SeqCst) == 1;
        if done {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    eth_publisher.shutdown().await;
    polygon_publisher.shutdown().await;
    h1.abort();
    h2.abort();
    h3.abort();
    h4.abort();

    // Fanout: both app-a and app-b get all ETH blocks
    assert_eq!(
        app_a_eth.load(Ordering::SeqCst),
        eth_count,
        "app-a should receive every ETH block"
    );
    assert_eq!(
        app_b_eth.load(Ordering::SeqCst),
        eth_count,
        "app-b should receive every ETH block (fanout)"
    );

    // Routing: polygon stream gets only polygon blocks
    assert_eq!(
        app_a_polygon.load(Ordering::SeqCst),
        polygon_count,
        "polygon consumer should receive only polygon blocks"
    );

    // Control channel
    assert_eq!(
        watch_count.load(Ordering::SeqCst),
        1,
        "watch stream should receive exactly one control message"
    );

    // Content validation
    let eth_expected: HashSet<u64> = (1_000_000..1_000_000 + u64::from(eth_count)).collect();
    let polygon_expected: HashSet<u64> =
        (2_000_000..2_000_000 + u64::from(polygon_count)).collect();

    assert_eq!(
        *app_a_eth_numbers.lock().unwrap(),
        eth_expected,
        "app-a ETH should receive exactly the published block numbers"
    );
    assert_eq!(
        *app_b_eth_numbers.lock().unwrap(),
        eth_expected,
        "app-b ETH should receive exactly the published block numbers"
    );
    assert_eq!(
        *app_a_polygon_numbers.lock().unwrap(),
        polygon_expected,
        "polygon consumer should receive exactly the published block numbers"
    );

    let watch_payload = watch_received.lock().unwrap().clone();
    assert_eq!(
        watch_payload,
        Some(ListenerEvent::WatchRegister {
            chain_id: 1,
            consumer_id: "app-a".to_string(),
            contract_addresses: vec!["0xabc".to_string()],
        }),
        "watch queue should receive the exact WatchRegister payload"
    );
}

// ── Test 8: Classified handler — simple roundtrip ───────────────────────────

/// `AsyncHandlerPayloadClassified` processes a valid message and ACKs it.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_broker_classified_handler_roundtrip() {
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();

    let publisher = broker.publisher("bf-cls-rt").await.unwrap();
    let topic = Topic::namespaced("bf-cls-rt", "events");

    let count = Arc::new(AtomicU32::new(0));
    let count_clone = count.clone();
    let handler = AsyncHandlerPayloadClassified::new(move |event: BlockEvent| {
        let c = count_clone.clone();
        async move {
            assert_eq!(event.block_number, 42);
            c.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    });

    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("bf-cls-rt-group")
            .consumer_name("consumer-1")
            .prefetch(10)
            .run(handler)
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    publisher
        .publish("events", &BlockEvent { block_number: 42 })
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while count.load(Ordering::SeqCst) == 0 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    publisher.shutdown().await;
    consumer_handle.abort();
    assert_eq!(
        count.load(Ordering::SeqCst),
        1,
        "classified handler should be called exactly once"
    );
}

// ── Test 9: Classified handler — transient errors trip circuit breaker ───────

/// `AsyncHandlerPayloadClassified` preserves `HandlerError::Transient`, which
/// trips the circuit breaker. After cooldown the consumer recovers and
/// processes a signal message.
///
/// This is the key difference from `AsyncHandlerPayloadOnly` — that wrapper
/// maps all closure errors to `HandlerError::Execution`, which does NOT trip
/// the circuit breaker.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_broker_classified_transient_trips_circuit_breaker() {
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();

    let publisher = broker.publisher("bf-cls-cb").await.unwrap();
    let topic = Topic::namespaced("bf-cls-cb", "events");

    let transient_count = Arc::new(AtomicU32::new(0));
    let signal_received = Arc::new(AtomicBool::new(false));

    let tc = transient_count.clone();
    let sr = signal_received.clone();
    let handler = AsyncHandlerPayloadClassified::new(move |event: BlockEvent| {
        let tc = tc.clone();
        let sr = sr.clone();
        async move {
            // Signal message — proves the CB recovered
            if event.block_number == 99 {
                sr.store(true, Ordering::SeqCst);
                return Ok(());
            }
            // Trip messages — return Transient to open the circuit
            tc.fetch_add(1, Ordering::SeqCst);
            Err(HandlerError::transient(std::io::Error::other(
                "simulated infra failure",
            )))
        }
    });

    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("bf-cls-cb-group")
            .consumer_name("consumer-1")
            .prefetch(10)
            .max_retries(10)
            .circuit_breaker(3, Duration::from_millis(2000))
            .redis_claim_min_idle(60)
            .redis_claim_interval(10)
            .run(handler)
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    // Publish 3 trip messages to open the circuit
    for i in 0..3u64 {
        publisher
            .publish("events", &BlockEvent { block_number: i })
            .await
            .unwrap();
    }

    // Wait until at least 3 transient failures recorded
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    while transient_count.load(Ordering::SeqCst) < 3 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(
        transient_count.load(Ordering::SeqCst) >= 3,
        "expected at least 3 transient failures to trip the circuit"
    );

    // Circuit is Open — signal must NOT be consumed during cooldown
    publisher
        .publish("events", &BlockEvent { block_number: 99 })
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(1000)).await;
    assert!(
        !signal_received.load(Ordering::SeqCst),
        "signal message must not be consumed while the circuit is Open"
    );

    // After cooldown: Half-Open → signal consumed
    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    while !signal_received.load(Ordering::SeqCst) && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    publisher.shutdown().await;
    consumer_handle.abort();
    assert!(
        signal_received.load(Ordering::SeqCst),
        "signal message must be consumed after the CB cooldown expires — \
         proves Transient errors from AsyncHandlerPayloadClassified trip the CB"
    );
}

// ── Test 10: Classified handler — permanent error → DLQ, CB untouched ───────

/// `AsyncHandlerPayloadClassified` with `HandlerError::permanent` routes the
/// message to the dead stream after max_retries, without tripping the circuit
/// breaker. A subsequent valid message is processed normally, proving the CB
/// stayed closed.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_broker_classified_permanent_dlq_no_cb_trip() {
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();

    let publisher = broker.publisher("bf-cls-perm").await.unwrap();
    let topic = Topic::namespaced("bf-cls-perm", "events");
    let dead_stream = topic.dead_key().to_owned();

    let success_count = Arc::new(AtomicU32::new(0));

    let sc = success_count.clone();
    let handler = AsyncHandlerPayloadClassified::new(move |event: BlockEvent| {
        let sc = sc.clone();
        async move {
            if event.block_number == 0 {
                // Permanent: the message itself is bad
                return Err(HandlerError::permanent(std::io::Error::other(
                    "invalid block",
                )));
            }
            // Valid message — proves the CB is still closed
            sc.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    });

    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("bf-cls-perm-group")
            .consumer_name("consumer-1")
            .prefetch(10)
            .max_retries(2)
            .circuit_breaker(3, Duration::from_millis(30_000))
            .redis_claim_min_idle(1)
            .redis_claim_interval(1)
            .run(handler)
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    // Publish a permanently-bad message
    publisher
        .publish("events", &BlockEvent { block_number: 0 })
        .await
        .unwrap();

    // Wait for it to land in the dead stream
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;
        let len = redis_xlen(url, &dead_stream).await;
        if len >= 1 {
            break;
        }
        if tokio::time::Instant::now() >= deadline {
            publisher.shutdown().await;
            consumer_handle.abort();
            panic!("permanent message was not moved to dead stream within timeout");
        }
    }

    // Now publish a valid message — it must be processed immediately
    // (proves the circuit breaker did NOT trip from permanent errors)
    publisher
        .publish("events", &BlockEvent { block_number: 42 })
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while success_count.load(Ordering::SeqCst) == 0 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    publisher.shutdown().await;
    consumer_handle.abort();

    assert_eq!(
        redis_xlen(url, &dead_stream).await,
        1,
        "exactly one message should be in the dead stream"
    );
    assert_eq!(
        success_count.load(Ordering::SeqCst),
        1,
        "valid message must be processed — CB should NOT have tripped from permanent errors"
    );
}
