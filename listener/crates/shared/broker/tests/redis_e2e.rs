#![cfg(feature = "redis")]
//! End-to-end integration tests for `broker::redis`.
//!
//! Each test follows the three-step pattern from the `broker` README:
//!   1. Define a payload type.
//!   2. Write a handler with `broker::AsyncHandlerPayloadOnly`.
//!   3. Pick the `broker::redis` backend, build config, and call `consumer.run_*()`.
//!
//! All tests share a single Redis container (`e2e-redis`) using
//! `ReuseDirective::CurrentSession` so the container survives the handle drop
//! inside the `OnceCell` init closure. Each test uses unique stream names to
//! avoid cross-contamination.
//!
//! ⚠️  Always run via Make — the Makefile removes the named container before
//! and after the suite so it is not left running between runs:
//!
//! ```bash
//! make test-e2e-redis
//! ```
//!
//! Running `cargo test -p broker --features redis --test redis_e2e -- --include-ignored` directly
//! will leave `e2e-redis` running after the suite finishes.

use std::{
    collections::{HashMap, HashSet},
    future::Future,
    process::Command,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU32, Ordering},
    },
    time::Duration,
};

// Traits and shared types come from `broker` — the backend-agnostic layer.
use broker::traits::{Consumer, DynPublisher};
#[allow(unused_imports)] // Publisher brings .publish() into scope.
use broker::{AckDecision, AsyncHandlerPayloadOnly};
// Backend-specific construction (connection, config builder, concrete types) comes from `broker::redis`.
use broker::redis::{
    RedisConnectionManager, RedisConsumer, RedisConsumerConfigBuilder, RedisPublisher,
    StreamManager, StreamTopology,
};
use test_support::shared_redis_url;

// ── Step 1: shared test payload ───────────────────────────────────────────────

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

/// Publish a synthetic block stream at a fixed rate.
///
/// This simulates the listener's polling loop producing canonical blocks and
/// publishing them to a chain-specific Redis stream through `DynPublisher`.
async fn publish_blocks_at_rps(
    publisher: &DynPublisher,
    stream: &str,
    chain_id: u64,
    start_block: u64,
    count: u64,
    rps: u64,
) -> Result<(), broker::traits::DynPublishError> {
    let per_message_delay = Duration::from_millis((1000 / rps.max(1)).max(1));
    for i in 0..count {
        publisher
            .publish(
                stream,
                &ListenerEvent::Block {
                    chain_id,
                    block_number: start_block + i,
                },
            )
            .await?;
        tokio::time::sleep(per_message_delay).await;
    }
    Ok(())
}

// ── Generic Consumer trait helper ─────────────────────────────────────────────

/// Assert that a single message published via `publish_fn` is received exactly once.
///
/// Bound on `broker::traits::Consumer` — exercises the trait directly, independent of which
/// backend (`RedisConsumer`, `RmqConsumer`, …) is passed in.
async fn assert_simple_roundtrip<C, F>(consumer: C, config: C::PrefetchConfig, publish_fn: F)
where
    C: Consumer + 'static,
    F: Future<Output = ()>,
{
    let count = Arc::new(AtomicU32::new(0));
    let count_clone = count.clone();

    // Step 2: handler — from broker
    let handler = AsyncHandlerPayloadOnly::new(move |_: serde_json::Value| {
        let c = count_clone.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            Ok::<(), std::io::Error>(())
        }
    });

    // Step 3: run consumer in background
    let handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    // Give the consumer time to start and create the Redis consumer group.
    tokio::time::sleep(Duration::from_millis(400)).await;

    publish_fn.await;

    // Poll until the handler is called or 5 s elapse.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while count.load(Ordering::SeqCst) == 0 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    handle.abort();
    assert_eq!(
        count.load(Ordering::SeqCst),
        1,
        "expected handler to be called exactly once"
    );
}

fn docker_container_control(action: &str, container_id: &str) {
    let status = Command::new("docker")
        .arg(action)
        .arg(container_id)
        .status()
        .unwrap_or_else(|e| panic!("failed to run `docker {action}` for {container_id}: {e}"));
    assert!(
        status.success(),
        "`docker {action}` failed for container {container_id}"
    );
}

async fn read_classification_marker(url: &str, marker_key: &str, msg_id: &str) -> Option<String> {
    let client = redis::Client::open(url).unwrap();
    let mut raw_conn = match client.get_multiplexed_async_connection().await {
        Ok(conn) => conn,
        Err(_) => return None,
    };

    redis::cmd("HGET")
        .arg(marker_key)
        .arg(msg_id)
        .query_async::<Option<String>>(&mut raw_conn)
        .await
        .ok()
        .flatten()
}

async fn wait_for_classification_marker(
    url: &str,
    marker_key: &str,
    msg_id: &str,
    expected: &str,
    deadline_timeout: Duration,
) {
    let deadline = tokio::time::Instant::now() + deadline_timeout;

    loop {
        let marker = read_classification_marker(url, marker_key, msg_id).await;
        if marker.as_deref() == Some(expected) {
            return;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "classification marker was not persisted after Redis recovery (msg_id={msg_id}, marker={marker:?})"
        );
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

// ── Test 1: run_simple roundtrip ──────────────────────────────────────────────

/// Publish one message → `run_simple` consumer receives and ACKs it.
/// Exercises the generic `broker::traits::Consumer` trait via `assert_simple_roundtrip`.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_run_simple_roundtrip() {
    let url = shared_redis_url().await;

    // Step 3 (backend): publisher + consumer from broker_redis
    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::builder(conn)
        .auto_trim(Duration::from_secs(1))
        .build();

    let config = RedisConsumerConfigBuilder::new()
        .stream("simple.events")
        .group_name("simple-group")
        .consumer_name("consumer-1")
        .dead_stream("simple.events:dead")
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    let consumer = RedisConsumer::connect(url).await.unwrap();

    assert_simple_roundtrip(consumer, config, async move {
        // broker::traits::Publisher trait — topic = stream name for Redis
        publisher
            .publish("simple.events", &BlockEvent { block_number: 42 })
            .await
            .unwrap();
    })
    .await;
}

// ── Test 2: run_with_retry — handler fails then succeeds ──────────────────────

/// Handler returns `Err` on the first two deliveries and `Ok` on the third.
/// The ClaimSweeper reclaims the message after each failure and re-delivers it.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_run_with_retry_eventually_succeeds() {
    let url = shared_redis_url().await;

    // Step 3 (backend): config from broker_redis
    let config = RedisConsumerConfigBuilder::new()
        .stream("retry.events")
        .group_name("retry-group")
        .consumer_name("consumer-1")
        .dead_stream("retry.events:dead")
        .max_retries(5)
        // Short idle/interval so the test finishes quickly.
        .claim_min_idle(Duration::from_millis(300))
        .claim_interval(Duration::from_millis(400))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    // Step 2: handler — from broker; fails on the first two calls, succeeds on the third
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

    // Step 3 (backend): consumer from broker_redis, run in background
    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    // Give the consumer time to start and create the group.
    tokio::time::sleep(Duration::from_millis(400)).await;

    // Publish via mq::Publisher — topic = stream name
    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);
    publisher
        .publish("retry.events", &BlockEvent { block_number: 1 })
        .await
        .unwrap();

    // Each retry cycle: consumer blocks on XREADGROUP > for up to 5 s, then drains PEL.
    // 3 cycles × ~5 s + claim overhead ≈ 17 s — use 30 s for safety.
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

// ── Test 3: multi-publish roundtrip ───────────────────────────────────────────

/// Publish 3 messages sequentially; consumer receives all 3.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_multi_publish_roundtrip() {
    let url = shared_redis_url().await;

    // Step 3 (backend): config from broker_redis
    let config = RedisConsumerConfigBuilder::new()
        .stream("batch.events")
        .group_name("batch-group")
        .consumer_name("consumer-1")
        .dead_stream("batch.events:dead")
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    // Step 2: handler — from broker
    let count = Arc::new(AtomicU32::new(0));
    let count_clone = count.clone();
    let handler = AsyncHandlerPayloadOnly::new(move |_: BlockEvent| {
        let c = count_clone.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            Ok::<(), std::io::Error>(())
        }
    });

    // Step 3 (backend): consumer from broker_redis, run in background
    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    // Publish 3 messages via mq::Publisher — topic = stream name
    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);
    let events = vec![
        BlockEvent { block_number: 1 },
        BlockEvent { block_number: 2 },
        BlockEvent { block_number: 3 },
    ];
    for event in &events {
        publisher.publish("batch.events", event).await.unwrap();
    }

    // Poll until all 3 are received.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    while count.load(Ordering::SeqCst) < 3 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    consumer_handle.abort();
    assert_eq!(
        count.load(Ordering::SeqCst),
        3,
        "all 3 batch messages should be received"
    );
}

// ── Test 4: run_with_retry → DLQ after max_retries ───────────────────────────

/// Handler always returns `Err`. After `max_retries` failed deliveries the
/// ClaimSweeper routes the message to the dead stream.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_run_with_retry_dlq_after_max_retries() {
    let url = shared_redis_url().await;

    let dead_stream = "dlq.events:dead";

    // Step 3 (backend): config from broker_redis
    let config = RedisConsumerConfigBuilder::new()
        .stream("dlq.events")
        .group_name("dlq-group")
        .consumer_name("consumer-1")
        .dead_stream(dead_stream)
        .max_retries(2)
        // Short idle/interval for a fast test.
        .claim_min_idle(Duration::from_millis(300))
        .claim_interval(Duration::from_millis(400))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    // Step 2: handler — from broker; always fails so the message exhausts max_retries
    let handler = AsyncHandlerPayloadOnly::new(|_: BlockEvent| async move {
        Err::<(), _>(std::io::Error::other("always fails"))
    });

    // Step 3 (backend): consumer from broker_redis, run in background
    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    // Give the consumer time to start and create the group.
    tokio::time::sleep(Duration::from_millis(400)).await;

    // Publish via mq::Publisher — topic = stream name
    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);
    publisher
        .publish("dlq.events", &BlockEvent { block_number: 99 })
        .await
        .unwrap();

    // Each retry cycle: consumer blocks on XREADGROUP > for up to 5 s before draining PEL.
    // With max_retries=2: 2 delivery failures × ~5 s block + claim overhead ≈ 12 s — use 20 s.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Check dead stream length via a raw Redis command.
        let client = redis::Client::open(url).unwrap();
        let mut raw_conn = client.get_multiplexed_async_connection().await.unwrap();
        let len: u64 = redis::cmd("XLEN")
            .arg(dead_stream)
            .query_async(&mut raw_conn)
            .await
            .unwrap_or(0);

        if len >= 1 {
            consumer_handle.abort();
            assert_eq!(len, 1, "exactly one message should be in the dead stream");
            return;
        }

        if tokio::time::Instant::now() >= deadline {
            consumer_handle.abort();
            panic!("message was not moved to dead stream within timeout");
        }
    }
}

// ── Test 5: AckDecision::Dead — immediate dead-stream without retry ───────────

/// Verifies that a handler returning `AckDecision::Dead` routes the message
/// directly to the dead stream on the **first delivery**, without waiting for
/// `claim_min_idle` to expire or exhausting `max_retries`.
///
/// Key difference from `test_run_with_retry_dlq_after_max_retries`:
/// - Handler never returns `Err` — it explicitly returns `Ok(AckDecision::Dead)`
/// - Handler is called **exactly once** (no retry cycles)
/// - Message appears in dead stream immediately, not after claim_min_idle
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_ack_decision_dead_immediate() {
    let url = shared_redis_url().await;

    let dead_stream = "immediate-dead.events:dead";

    let config = RedisConsumerConfigBuilder::new()
        .stream("immediate-dead.events")
        .group_name("dead-group")
        .consumer_name("consumer-1")
        .dead_stream(dead_stream)
        // high max_retries — the handler should never reach them
        .max_retries(10)
        // Long claim_min_idle — the message must NOT need the sweeper to route to DLQ
        .claim_min_idle(Duration::from_secs(60))
        .claim_interval(Duration::from_secs(15))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = call_count.clone();

    // Custom handler that always returns AckDecision::Dead
    struct DeadHandler(Arc<AtomicU32>);

    #[async_trait::async_trait]
    impl broker::Handler for DeadHandler {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(AckDecision::Dead)
        }
    }

    let dead_handler = DeadHandler(call_count.clone());
    let _ = call_count_clone; // suppress unused warning

    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, dead_handler).await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);
    publisher
        .publish("immediate-dead.events", &BlockEvent { block_number: 42 })
        .await
        .unwrap();

    // Poll for dead stream entry — should appear within a few seconds (no claim delay)
    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    loop {
        tokio::time::sleep(Duration::from_millis(200)).await;

        let client = redis::Client::open(url).unwrap();
        let mut raw_conn = client.get_multiplexed_async_connection().await.unwrap();
        let len: u64 = redis::cmd("XLEN")
            .arg(dead_stream)
            .query_async(&mut raw_conn)
            .await
            .unwrap_or(0);

        if len >= 1 {
            // Give a little extra time to confirm no retry deliveries happen
            tokio::time::sleep(Duration::from_millis(800)).await;
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

// ── Test 6: Circuit Breaker — halts consumption during the cooldown window ────
//
// Handler returns `HandlerError::Transient` for the first 3 deliveries, tripping
// the circuit (threshold=3), then `Ok(Ack)` for all subsequent deliveries.
//
// After the circuit opens we publish a "signal" message (block_number=99) and
// assert it is NOT consumed during the cooldown window, then IS consumed once
// the circuit transitions through Half-Open back to Closed.
//
// Implementation note for Redis: the 3 trip messages remain in the PEL (not
// XACKed). After the cooldown the consumer drains its own PEL first (all return
// Ok at that point), then picks up the signal from the stream. The signal_received
// flag is therefore the cleanest assertion target and works regardless of how many
// PEL re-deliveries occur.
//
// A long claim_min_idle (60 s) prevents the ClaimSweeper from interfering during
// the test window.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_circuit_breaker_halts_consumption() {
    let url = shared_redis_url().await;

    let config = RedisConsumerConfigBuilder::new()
        .stream("cb-halt.events")
        .group_name("cb-halt-group")
        .consumer_name("consumer-1")
        .dead_stream("cb-halt.events:dead")
        .max_retries(10)
        // Long claim_min_idle keeps the ClaimSweeper from touching the PEL
        // during the test window (< 10 s total).
        .claim_min_idle(Duration::from_secs(60))
        .claim_interval(Duration::from_secs(10))
        .circuit_breaker_threshold(3)
        .circuit_breaker_cooldown(Duration::from_millis(2000))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    let transient_count = Arc::new(AtomicU32::new(0));
    let signal_received = Arc::new(AtomicBool::new(false));

    // Handler:
    //   block_number == 99  → signal message: record and ACK immediately.
    //   otherwise           → increment transient_count; return Transient for the
    //                         first 3 calls (trips CB), Ok for subsequent calls
    //                         (Half-Open probe and PEL re-deliveries after cooldown).
    struct TrippingHandler {
        transient_count: Arc<AtomicU32>,
        signal_received: Arc<AtomicBool>,
    }

    #[async_trait::async_trait]
    impl broker::Handler for TrippingHandler {
        async fn call(&self, msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            if let Ok(event) = serde_json::from_slice::<BlockEvent>(&msg.payload)
                && event.block_number == 99
            {
                self.signal_received.store(true, Ordering::SeqCst);
                return Ok(AckDecision::Ack);
            }
            let prev = self.transient_count.fetch_add(1, Ordering::SeqCst);
            if prev < 3 {
                Err(broker::HandlerError::Transient(Box::new(
                    std::io::Error::other("simulated infrastructure failure"),
                )))
            } else {
                Ok(AckDecision::Ack)
            }
        }
    }

    let consumer = RedisConsumer::connect(url).await.unwrap();
    // Clone Arcs before the move closure so the outer test retains handles.
    let transient_count_for_handler = transient_count.clone();
    let signal_received_for_handler = signal_received.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer
            .run(
                config,
                TrippingHandler {
                    transient_count: transient_count_for_handler,
                    signal_received: signal_received_for_handler,
                },
            )
            .await;
    });

    // Wait for the consumer to start and create the consumer group.
    tokio::time::sleep(Duration::from_millis(400)).await;

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);

    // Publish 3 trip messages before the consumer's first XREADGROUP blocking call
    // so all three arrive in a single batch and are processed sequentially,
    // guaranteeing the CB trips on exactly the 3rd failure.
    for i in 0..3u64 {
        publisher
            .publish("cb-halt.events", &BlockEvent { block_number: i })
            .await
            .unwrap();
    }

    // Wait until at least 3 transient failures are recorded (circuit is now Open).
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    while transient_count.load(Ordering::SeqCst) < 3 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(
        transient_count.load(Ordering::SeqCst) >= 3,
        "expected at least 3 transient failures to trip the circuit"
    );

    // ── Circuit is Open ───────────────────────────────────────────────────────
    // Publish the signal message — it must NOT be consumed during the cooldown.
    publisher
        .publish("cb-halt.events", &BlockEvent { block_number: 99 })
        .await
        .unwrap();

    // Observe: mid-cooldown (1 s into the 2 s window), signal must still be unprocessed.
    tokio::time::sleep(Duration::from_millis(1000)).await;
    assert!(
        !signal_received.load(Ordering::SeqCst),
        "signal message must not be consumed while the circuit is Open"
    );

    // ── After cooldown: PEL drained (all Ok) → Half-Open → signal consumed ───
    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    while !signal_received.load(Ordering::SeqCst) && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    consumer_handle.abort();
    assert!(
        signal_received.load(Ordering::SeqCst),
        "signal message must be consumed after the CB cooldown expires"
    );
}

// ── Test 7: Circuit Breaker — prevents DLQ pollution during a sustained outage ─
//
// A handler that always returns `HandlerError::Transient` simulates a downstream
// outage (e.g., database unavailable).
//
// Transient failures now retry indefinitely and do not consume `max_retries`.
// Without a circuit breaker, the message remains in retry circulation.
//
// With CB (threshold=2, cooldown=5 s) the consumer pauses after 2 failures,
// preventing any further XREADGROUP reads. The ClaimSweeper is neutralised by a
// very long claim_min_idle (60 s) so it cannot accumulate delivery counts within
// the 3 s observation window — keeping the dead stream empty.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_circuit_breaker_prevents_dlq_pollution() {
    let url = shared_redis_url().await;

    let dead_stream = "cb-dlq.events:dead";

    let config = RedisConsumerConfigBuilder::new()
        .stream("cb-dlq.events")
        .group_name("cb-dlq-group")
        .consumer_name("consumer-1")
        .dead_stream(dead_stream)
        .max_retries(3)
        // claim_min_idle=60 s prevents the ClaimSweeper from incrementing
        // delivery_count during the 3 s observation window.
        .claim_min_idle(Duration::from_secs(60))
        .claim_interval(Duration::from_secs(10))
        .circuit_breaker_threshold(2)
        .circuit_breaker_cooldown(Duration::from_secs(5))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    // Handler: always returns Transient — simulates a sustained infrastructure outage.
    struct AlwaysTransient;

    #[async_trait::async_trait]
    impl broker::Handler for AlwaysTransient {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            Err(broker::HandlerError::Transient(Box::new(
                std::io::Error::other("downstream is unavailable"),
            )))
        }
    }

    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, AlwaysTransient).await;
    });

    // Wait for the consumer to start and create the consumer group.
    tokio::time::sleep(Duration::from_millis(400)).await;

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);

    // Publish 2 messages — enough to trip the circuit (threshold=2).
    publisher
        .publish("cb-dlq.events", &BlockEvent { block_number: 1 })
        .await
        .unwrap();
    publisher
        .publish("cb-dlq.events", &BlockEvent { block_number: 2 })
        .await
        .unwrap();

    // Wait for the CB to open and for the observation window to elapse.
    // cooldown = 5 s; we check at 3 s — still within the Open window.
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Verify the dead stream is empty using a raw Redis command.
    let client = redis::Client::open(url).unwrap();
    let mut raw_conn = client.get_multiplexed_async_connection().await.unwrap();
    let dead_len: u64 = redis::cmd("XLEN")
        .arg(dead_stream)
        .query_async(&mut raw_conn)
        .await
        .unwrap_or(0);

    consumer_handle.abort();

    assert_eq!(
        dead_len, 0,
        "circuit breaker must prevent messages from reaching the dead stream during a transient outage"
    );
}

// ── Test 8: Transient failures retry indefinitely (bounded retry ignored) ──────
//
// With max_retries=1, transient failures should keep retrying and must never
// be moved to the dead stream by retry-budget exhaustion.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_transient_failures_retry_indefinitely() {
    let url = shared_redis_url().await;

    let dead_stream = "transient-infinite.events:dead";
    let config = RedisConsumerConfigBuilder::new()
        .stream("transient-infinite.events")
        .group_name("transient-infinite-group")
        .consumer_name("consumer-1")
        .dead_stream(dead_stream)
        .max_retries(1)
        .claim_min_idle(Duration::from_millis(300))
        .claim_interval(Duration::from_millis(300))
        .prefetch_count(10)
        .block_ms(1000)
        .build_prefetch()
        .unwrap();

    struct AlwaysTransient(Arc<AtomicU32>);

    #[async_trait::async_trait]
    impl broker::Handler for AlwaysTransient {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Err(broker::HandlerError::Transient(Box::new(
                std::io::Error::other("downstream is unavailable"),
            )))
        }
    }

    let attempts = Arc::new(AtomicU32::new(0));
    let attempts_for_handler = attempts.clone();
    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer
            .run(config, AlwaysTransient(attempts_for_handler))
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);
    publisher
        .publish("transient-infinite.events", &BlockEvent { block_number: 1 })
        .await
        .unwrap();

    let attempts_deadline = tokio::time::Instant::now() + Duration::from_secs(12);
    while tokio::time::Instant::now() < attempts_deadline {
        if attempts.load(Ordering::SeqCst) >= 4 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let client = redis::Client::open(url).unwrap();
    let mut raw_conn = client.get_multiplexed_async_connection().await.unwrap();
    let dead_len: u64 = redis::cmd("XLEN")
        .arg(dead_stream)
        .query_async(&mut raw_conn)
        .await
        .unwrap_or(0);

    consumer_handle.abort();

    assert!(
        attempts.load(Ordering::SeqCst) >= 4,
        "transient message should keep retrying even with max_retries=1; attempts={}",
        attempts.load(Ordering::SeqCst)
    );
    assert_eq!(
        dead_len, 0,
        "transient failures must not be moved to dead stream by max_retries"
    );
}

// ── Test 9: mark_transient retries across Redis outage and recovers marker ────
//
// Simulates Redis becoming unavailable exactly between handler completion and
// classification persistence. `mark_transient_safe` must block/retry and write
// the transient marker once Redis is back.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_mark_transient_retries_until_redis_recovers() {
    use testcontainers::core::ImageExt;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::redis::Redis;

    let container = Redis::default().with_tag("6.2.0").start().await.unwrap();
    let port = container.get_host_port_ipv4(6379).await.unwrap();
    let url = format!("redis://127.0.0.1:{port}");

    let stream = "classification-outage.events";
    let group = "classification-outage-group";
    let config = RedisConsumerConfigBuilder::new()
        .stream(stream)
        .group_name(group)
        .consumer_name("consumer-1")
        .dead_stream("classification-outage.events:dead")
        .max_retries(2)
        .claim_min_idle(Duration::from_secs(60))
        .claim_interval(Duration::from_secs(10))
        .prefetch_count(10)
        .block_ms(500)
        .build_prefetch()
        .unwrap();
    let classification_key = config.retry.classification_marker_key();

    let entered_handler = Arc::new(tokio::sync::Notify::new());
    let release_handler = Arc::new(tokio::sync::Notify::new());

    struct BlockThenTransient {
        entered: Arc<tokio::sync::Notify>,
        release: Arc<tokio::sync::Notify>,
    }

    #[async_trait::async_trait]
    impl broker::Handler for BlockThenTransient {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            self.entered.notify_one();
            self.release.notified().await;
            Err(broker::HandlerError::Transient(Box::new(
                std::io::Error::other("simulated transient failure"),
            )))
        }
    }

    let consumer = RedisConsumer::connect(&url).await.unwrap();
    let entered_for_handler = Arc::clone(&entered_handler);
    let release_for_handler = Arc::clone(&release_handler);
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer
            .run(
                config,
                BlockThenTransient {
                    entered: entered_for_handler,
                    release: release_for_handler,
                },
            )
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    let conn = RedisConnectionManager::new(&url).await.unwrap();
    let publisher = RedisPublisher::new(conn);
    let msg_id = publisher
        .publish(stream, &BlockEvent { block_number: 7 })
        .await
        .unwrap();

    tokio::time::timeout(Duration::from_secs(5), entered_handler.notified())
        .await
        .expect("handler should receive the first delivery");

    docker_container_control("pause", container.id());
    release_handler.notify_waiters();

    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(
        !consumer_handle.is_finished(),
        "consumer should stay alive and retry marker persistence while Redis is down"
    );

    docker_container_control("unpause", container.id());

    wait_for_classification_marker(
        &url,
        &classification_key,
        &msg_id,
        "transient",
        Duration::from_secs(30),
    )
    .await;

    let final_marker = read_classification_marker(&url, &classification_key, &msg_id).await;
    assert_eq!(
        final_marker.as_deref(),
        Some("transient"),
        "classification marker key {} should contain transient for msg_id={}",
        classification_key,
        msg_id
    );

    consumer_handle.abort();
}

// ── Test 9: Transient -> Permanent transition re-enables bounded DLQ path ──────
//
// First delivery is transient (infinite path), subsequent deliveries are
// permanent; once permanent is observed, the message should be dead-lettered
// according to max_retries.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_transient_then_permanent_eventually_dead_letters() {
    let url = shared_redis_url().await;

    let dead_stream = "transient-to-permanent.events:dead";
    let config = RedisConsumerConfigBuilder::new()
        .stream("transient-to-permanent.events")
        .group_name("transient-to-permanent-group")
        .consumer_name("consumer-1")
        .dead_stream(dead_stream)
        .max_retries(1)
        .claim_min_idle(Duration::from_millis(300))
        .claim_interval(Duration::from_millis(300))
        .prefetch_count(10)
        .block_ms(1000)
        .build_prefetch()
        .unwrap();

    struct TransientThenPermanent(Arc<AtomicU32>);

    #[async_trait::async_trait]
    impl broker::Handler for TransientThenPermanent {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            let call_index = self.0.fetch_add(1, Ordering::SeqCst);
            if call_index == 0 {
                Err(broker::HandlerError::Transient(Box::new(
                    std::io::Error::other("downstream is unavailable"),
                )))
            } else {
                Err(broker::HandlerError::permanent(std::io::Error::other(
                    "invalid payload",
                )))
            }
        }
    }

    let calls = Arc::new(AtomicU32::new(0));
    let calls_for_handler = calls.clone();
    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer
            .run(config, TransientThenPermanent(calls_for_handler))
            .await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);
    publisher
        .publish(
            "transient-to-permanent.events",
            &BlockEvent { block_number: 1 },
        )
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    let dead_len = loop {
        tokio::time::sleep(Duration::from_millis(250)).await;
        let client = redis::Client::open(url).unwrap();
        let mut raw_conn = client.get_multiplexed_async_connection().await.unwrap();
        let len: u64 = redis::cmd("XLEN")
            .arg(dead_stream)
            .query_async(&mut raw_conn)
            .await
            .unwrap_or(0);
        if len >= 1 || tokio::time::Instant::now() >= deadline {
            break len;
        }
    };

    consumer_handle.abort();

    assert!(
        calls.load(Ordering::SeqCst) >= 2,
        "handler should see transient then permanent calls; calls={}",
        calls.load(Ordering::SeqCst)
    );
    assert_eq!(
        dead_len, 1,
        "message should eventually dead-letter after becoming permanent"
    );
}

// ── Test 10: run — steady-state pending retry ────────────────

/// Handler fails on first delivery; message stays in PEL. The periodic
/// XREADGROUP "0" drain picks it up during steady-state and the handler
/// succeeds on retry. Proves failed messages get handler retries before DLQ.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_run_retries_pending_in_steady_state() {
    let url = shared_redis_url().await;

    let dead_stream = "prefetch-retry.events:dead";

    let config = RedisConsumerConfigBuilder::new()
        .stream("prefetch-retry.events")
        .group_name("prefetch-retry-group")
        .consumer_name("consumer-1")
        .dead_stream(dead_stream)
        .max_retries(3)
        .claim_min_idle(Duration::from_millis(300))
        .claim_interval(Duration::from_millis(400))
        .prefetch_count(10)
        .block_ms(2000)
        .build_prefetch()
        .unwrap();

    let call_count = Arc::new(AtomicU32::new(0));

    struct PrefetchRetryHandler(Arc<AtomicU32>);

    #[async_trait::async_trait]
    impl broker::Handler for PrefetchRetryHandler {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            let prev = self.0.fetch_add(1, Ordering::SeqCst);
            if prev < 1 {
                Err(broker::HandlerError::permanent(std::io::Error::other(
                    "simulated failure",
                )))
            } else {
                Ok(AckDecision::Ack)
            }
        }
    }

    impl Clone for PrefetchRetryHandler {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    let handler = PrefetchRetryHandler(call_count.clone());

    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);
    publisher
        .publish("prefetch-retry.events", &BlockEvent { block_number: 1 })
        .await
        .unwrap();

    // Pending drain runs every 1 s. First delivery fails → PEL. Second delivery
    // (from XREADGROUP 0) succeeds. Allow up to 15 s.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    while call_count.load(Ordering::SeqCst) < 2 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    consumer_handle.abort();

    assert!(
        call_count.load(Ordering::SeqCst) >= 2,
        "handler should be called at least 2 times (1 failure + 1 success from steady-state pending drain), got {}",
        call_count.load(Ordering::SeqCst)
    );

    let client = redis::Client::open(url).unwrap();
    let mut raw_conn = client.get_multiplexed_async_connection().await.unwrap();
    let dead_len: u64 = redis::cmd("XLEN")
        .arg(dead_stream)
        .query_async(&mut raw_conn)
        .await
        .unwrap_or(0);

    assert_eq!(
        dead_len, 0,
        "message must not reach dead stream when handler succeeds on retry"
    );
}

// ── Test 9: DynPublisher multichain fanout + control routing ───────────────────

/// Mirrors the AMQP dynpublisher multichain e2e with Redis semantics:
/// - fanout via separate consumer groups on the same stream
/// - routing via separate stream names (`*.blocks` / `*.control`)
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_dynpublisher_multichain_routing_for_multiple_apps() {
    let url = shared_redis_url().await;

    let eth_blocks_stream = "dynpub.ethereum.events.blocks";
    let polygon_blocks_stream = "dynpub.polygon.events.blocks";
    let eth_control_stream = "dynpub.ethereum.events.control";

    let eth_blocks_topology = StreamTopology {
        main: eth_blocks_stream.to_string(),
        dead: format!("{eth_blocks_stream}:dead"),
    };
    let polygon_blocks_topology = StreamTopology {
        main: polygon_blocks_stream.to_string(),
        dead: format!("{polygon_blocks_stream}:dead"),
    };
    let eth_control_topology = StreamTopology {
        main: eth_control_stream.to_string(),
        dead: format!("{eth_control_stream}:dead"),
    };

    let manager_conn = RedisConnectionManager::new(url).await.unwrap();
    let stream_manager = StreamManager::new(manager_conn);
    stream_manager
        .ensure_topology(&eth_blocks_topology)
        .await
        .unwrap();
    stream_manager
        .ensure_topology(&polygon_blocks_topology)
        .await
        .unwrap();
    stream_manager
        .ensure_topology(&eth_control_topology)
        .await
        .unwrap();

    let app_a_eth_blocks = Arc::new(AtomicU32::new(0));
    let app_a_eth_block_numbers = Arc::new(Mutex::new(HashSet::<u64>::new()));
    let app_a_eth_unexpected = Arc::new(AtomicU32::new(0));
    let app_b_eth_blocks = Arc::new(AtomicU32::new(0));
    let app_b_eth_block_numbers = Arc::new(Mutex::new(HashSet::<u64>::new()));
    let app_b_eth_unexpected = Arc::new(AtomicU32::new(0));
    let app_a_polygon_blocks = Arc::new(AtomicU32::new(0));
    let app_a_polygon_block_numbers = Arc::new(Mutex::new(HashSet::<u64>::new()));
    let app_a_polygon_unexpected = Arc::new(AtomicU32::new(0));
    let watch_count = Arc::new(AtomicU32::new(0));
    let watch_received = Arc::new(Mutex::new(None::<ListenerEvent>));
    let watch_unexpected = Arc::new(AtomicU32::new(0));

    let app_a_eth_handler = {
        let blocks = app_a_eth_blocks.clone();
        let block_numbers = app_a_eth_block_numbers.clone();
        let unexpected = app_a_eth_unexpected.clone();
        AsyncHandlerPayloadOnly::new(move |event: ListenerEvent| {
            let blocks = blocks.clone();
            let block_numbers = block_numbers.clone();
            let unexpected = unexpected.clone();
            async move {
                match event {
                    ListenerEvent::Block {
                        chain_id: 1,
                        block_number,
                    } => {
                        blocks.fetch_add(1, Ordering::SeqCst);
                        block_numbers.lock().unwrap().insert(block_number);
                    }
                    _ => {
                        unexpected.fetch_add(1, Ordering::SeqCst);
                    }
                }
                Ok::<(), std::convert::Infallible>(())
            }
        })
    };

    let app_b_eth_handler = {
        let blocks = app_b_eth_blocks.clone();
        let block_numbers = app_b_eth_block_numbers.clone();
        let unexpected = app_b_eth_unexpected.clone();
        AsyncHandlerPayloadOnly::new(move |event: ListenerEvent| {
            let blocks = blocks.clone();
            let block_numbers = block_numbers.clone();
            let unexpected = unexpected.clone();
            async move {
                match event {
                    ListenerEvent::Block {
                        chain_id: 1,
                        block_number,
                    } => {
                        blocks.fetch_add(1, Ordering::SeqCst);
                        block_numbers.lock().unwrap().insert(block_number);
                    }
                    _ => {
                        unexpected.fetch_add(1, Ordering::SeqCst);
                    }
                }
                Ok::<(), std::convert::Infallible>(())
            }
        })
    };

    let app_a_polygon_handler = {
        let blocks = app_a_polygon_blocks.clone();
        let block_numbers = app_a_polygon_block_numbers.clone();
        let unexpected = app_a_polygon_unexpected.clone();
        AsyncHandlerPayloadOnly::new(move |event: ListenerEvent| {
            let blocks = blocks.clone();
            let block_numbers = block_numbers.clone();
            let unexpected = unexpected.clone();
            async move {
                match event {
                    ListenerEvent::Block {
                        chain_id: 137,
                        block_number,
                    } => {
                        blocks.fetch_add(1, Ordering::SeqCst);
                        block_numbers.lock().unwrap().insert(block_number);
                    }
                    _ => {
                        unexpected.fetch_add(1, Ordering::SeqCst);
                    }
                }
                Ok::<(), std::convert::Infallible>(())
            }
        })
    };

    let watch_handler = {
        let count = watch_count.clone();
        let received = watch_received.clone();
        let unexpected = watch_unexpected.clone();
        AsyncHandlerPayloadOnly::new(move |event: ListenerEvent| {
            let count = count.clone();
            let received = received.clone();
            let unexpected = unexpected.clone();
            async move {
                match &event {
                    ListenerEvent::WatchRegister { chain_id: 1, .. } => {
                        count.fetch_add(1, Ordering::SeqCst);
                        let mut r = received.lock().unwrap();
                        if r.is_none() {
                            *r = Some(event.clone());
                        }
                    }
                    _ => {
                        unexpected.fetch_add(1, Ordering::SeqCst);
                    }
                }
                Ok::<(), std::convert::Infallible>(())
            }
        })
    };

    let app_a_eth_config = RedisConsumerConfigBuilder::new()
        .with_topology(&eth_blocks_topology)
        .group_name("dynpub-app-a-eth-group")
        .consumer_name("dynpub-app-a-eth-consumer")
        .max_retries(3)
        .claim_min_idle(Duration::from_secs(2))
        .claim_interval(Duration::from_secs(1))
        .prefetch_count(32)
        .block_ms(1000)
        .build_prefetch()
        .unwrap();

    let app_b_eth_config = RedisConsumerConfigBuilder::new()
        .with_topology(&eth_blocks_topology)
        .group_name("dynpub-app-b-eth-group")
        .consumer_name("dynpub-app-b-eth-consumer")
        .max_retries(3)
        .claim_min_idle(Duration::from_secs(2))
        .claim_interval(Duration::from_secs(1))
        .prefetch_count(32)
        .block_ms(1000)
        .build_prefetch()
        .unwrap();

    let app_a_polygon_config = RedisConsumerConfigBuilder::new()
        .with_topology(&polygon_blocks_topology)
        .group_name("dynpub-app-a-polygon-group")
        .consumer_name("dynpub-app-a-polygon-consumer")
        .max_retries(3)
        .claim_min_idle(Duration::from_secs(2))
        .claim_interval(Duration::from_secs(1))
        .prefetch_count(32)
        .block_ms(1000)
        .build_prefetch()
        .unwrap();

    let watch_config = RedisConsumerConfigBuilder::new()
        .with_topology(&eth_control_topology)
        .group_name("dynpub-listener-watch-group")
        .consumer_name("dynpub-listener-watch-consumer")
        .max_retries(3)
        .claim_min_idle(Duration::from_secs(2))
        .claim_interval(Duration::from_secs(1))
        .prefetch_count(16)
        .block_ms(1000)
        .build_prefetch()
        .unwrap();

    let app_a_eth_consumer = RedisConsumer::connect(url).await.unwrap();
    let app_b_eth_consumer = RedisConsumer::connect(url).await.unwrap();
    let app_a_polygon_consumer = RedisConsumer::connect(url).await.unwrap();
    let watch_consumer = RedisConsumer::connect(url).await.unwrap();

    let app_a_eth_handle = tokio::spawn(async move {
        let _ = app_a_eth_consumer
            .run(app_a_eth_config, app_a_eth_handler)
            .await;
    });
    let app_b_eth_handle = tokio::spawn(async move {
        let _ = app_b_eth_consumer
            .run(app_b_eth_config, app_b_eth_handler)
            .await;
    });
    let app_a_polygon_handle = tokio::spawn(async move {
        let _ = app_a_polygon_consumer
            .run(app_a_polygon_config, app_a_polygon_handler)
            .await;
    });
    let watch_handle = tokio::spawn(async move {
        let _ = watch_consumer.run(watch_config, watch_handler).await;
    });

    tokio::time::sleep(Duration::from_millis(1200)).await;

    let mut publishers: HashMap<String, DynPublisher> = HashMap::new();
    publishers.insert(
        "ethereum".to_string(),
        DynPublisher::new(RedisPublisher::new(
            RedisConnectionManager::new(url).await.unwrap(),
        )),
    );
    publishers.insert(
        "polygon".to_string(),
        DynPublisher::new(RedisPublisher::new(
            RedisConnectionManager::new(url).await.unwrap(),
        )),
    );

    publishers["ethereum"]
        .publish(
            eth_control_stream,
            &ListenerEvent::WatchRegister {
                chain_id: 1,
                consumer_id: "app-a".to_string(),
                contract_addresses: vec!["0xabc0000000000000000000000000000000000001".to_string()],
            },
        )
        .await
        .unwrap();

    let eth_expected: u32 = 24;
    let polygon_expected: u32 = 11;

    let eth_pub = publishers
        .get("ethereum")
        .expect("ethereum publisher should exist")
        .clone();
    let polygon_pub = publishers
        .get("polygon")
        .expect("polygon publisher should exist")
        .clone();

    let eth_publish_handle = tokio::spawn(async move {
        publish_blocks_at_rps(
            &eth_pub,
            eth_blocks_stream,
            1,
            1_000_000,
            u64::from(eth_expected),
            20,
        )
        .await
        .unwrap();
    });

    let polygon_publish_handle = tokio::spawn(async move {
        publish_blocks_at_rps(
            &polygon_pub,
            polygon_blocks_stream,
            137,
            2_000_000,
            u64::from(polygon_expected),
            12,
        )
        .await
        .unwrap();
    });

    eth_publish_handle.await.unwrap();
    polygon_publish_handle.await.unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    while tokio::time::Instant::now() < deadline {
        let done = app_a_eth_blocks.load(Ordering::SeqCst) == eth_expected
            && app_b_eth_blocks.load(Ordering::SeqCst) == eth_expected
            && app_a_polygon_blocks.load(Ordering::SeqCst) == polygon_expected
            && watch_count.load(Ordering::SeqCst) == 1;
        if done {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    for publisher in publishers.values() {
        publisher.shutdown().await;
    }

    app_a_eth_handle.abort();
    app_b_eth_handle.abort();
    app_a_polygon_handle.abort();
    watch_handle.abort();

    assert_eq!(
        app_a_eth_blocks.load(Ordering::SeqCst),
        eth_expected,
        "app-a should receive every ETH block"
    );
    assert_eq!(
        app_b_eth_blocks.load(Ordering::SeqCst),
        eth_expected,
        "app-b should receive every ETH block"
    );
    assert_eq!(
        app_a_polygon_blocks.load(Ordering::SeqCst),
        polygon_expected,
        "polygon stream should receive only polygon blocks"
    );
    assert_eq!(
        watch_count.load(Ordering::SeqCst),
        1,
        "watch stream should receive exactly one control message"
    );

    assert_eq!(
        app_a_eth_unexpected.load(Ordering::SeqCst),
        0,
        "app-a ETH queue should not receive unexpected payloads"
    );
    assert_eq!(
        app_b_eth_unexpected.load(Ordering::SeqCst),
        0,
        "app-b ETH queue should not receive unexpected payloads"
    );
    assert_eq!(
        app_a_polygon_unexpected.load(Ordering::SeqCst),
        0,
        "polygon queue should not receive unexpected payloads"
    );
    assert_eq!(
        watch_unexpected.load(Ordering::SeqCst),
        0,
        "watch queue should not receive unexpected payloads"
    );

    // Content validation: verify received payloads match what was published.
    let eth_expected_blocks: HashSet<u64> =
        (1_000_000..1_000_000 + u64::from(eth_expected)).collect();
    let polygon_expected_blocks: HashSet<u64> =
        (2_000_000..2_000_000 + u64::from(polygon_expected)).collect();

    let app_a_eth_received = app_a_eth_block_numbers.lock().unwrap().clone();
    let app_b_eth_received = app_b_eth_block_numbers.lock().unwrap().clone();
    let app_a_polygon_received = app_a_polygon_block_numbers.lock().unwrap().clone();

    assert_eq!(
        app_a_eth_received, eth_expected_blocks,
        "app-a ETH should receive exactly the published block numbers"
    );
    assert_eq!(
        app_b_eth_received, eth_expected_blocks,
        "app-b ETH should receive exactly the published block numbers"
    );
    assert_eq!(
        app_a_polygon_received, polygon_expected_blocks,
        "polygon consumer should receive exactly the published block numbers"
    );

    let watch_payload = watch_received.lock().unwrap().clone();
    let expected_watch = ListenerEvent::WatchRegister {
        chain_id: 1,
        consumer_id: "app-a".to_string(),
        contract_addresses: vec!["0xabc0000000000000000000000000000000000001".to_string()],
    };
    assert_eq!(
        watch_payload.as_ref(),
        Some(&expected_watch),
        "watch queue should receive the exact WatchRegister payload"
    );
}

// ── Test 10: prefetch-safe does not double-dispatch slow handlers ───────────────

/// Regression test for pending-drain overlap:
/// when handler latency exceeds the 1s pending tick, each stream ID must still
/// be dispatched exactly once while it is in-flight.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_run_no_duplicate_dispatch_when_handler_slow() {
    let url = shared_redis_url().await;

    let stream = "prefetch.no-dup.slow.events";
    let dead_stream = "prefetch.no-dup.slow.events:dead";
    let expected_messages: u32 = 5;

    let config = RedisConsumerConfigBuilder::new()
        .stream(stream)
        .group_name("prefetch-no-dup-group")
        .consumer_name("consumer-1")
        .dead_stream(dead_stream)
        .max_retries(3)
        // Keep ClaimSweeper out of the test window; we only validate local dispatch behavior.
        .claim_min_idle(Duration::from_secs(30))
        .claim_interval(Duration::from_secs(10))
        .prefetch_count(8)
        .block_ms(1000)
        .build_prefetch()
        .unwrap();

    let total_calls = Arc::new(AtomicU32::new(0));
    let duplicate_calls = Arc::new(AtomicU32::new(0));
    let seen_blocks = Arc::new(Mutex::new(HashSet::<u64>::new()));

    let handler = {
        let total_calls = total_calls.clone();
        let duplicate_calls = duplicate_calls.clone();
        let seen_blocks = seen_blocks.clone();
        AsyncHandlerPayloadOnly::new(move |event: BlockEvent| {
            let total_calls = total_calls.clone();
            let duplicate_calls = duplicate_calls.clone();
            let seen_blocks = seen_blocks.clone();
            async move {
                // Intentionally slower than the 1s pending tick in run.
                tokio::time::sleep(Duration::from_millis(1500)).await;
                total_calls.fetch_add(1, Ordering::SeqCst);
                let inserted = seen_blocks.lock().unwrap().insert(event.block_number);
                if !inserted {
                    duplicate_calls.fetch_add(1, Ordering::SeqCst);
                }
                Ok::<(), std::convert::Infallible>(())
            }
        })
    };

    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);
    for i in 0..u64::from(expected_messages) {
        publisher
            .publish(stream, &BlockEvent { block_number: i })
            .await
            .unwrap();
    }

    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    while tokio::time::Instant::now() < deadline {
        let unique = seen_blocks.lock().unwrap().len();
        if unique == expected_messages as usize {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    consumer_handle.abort();

    let unique = seen_blocks.lock().unwrap().len();
    assert_eq!(
        unique, expected_messages as usize,
        "every published message should be observed"
    );
    assert_eq!(
        duplicate_calls.load(Ordering::SeqCst),
        0,
        "no message should be dispatched more than once while in-flight"
    );
    assert_eq!(
        total_calls.load(Ordering::SeqCst),
        expected_messages,
        "handler should be called exactly once per message"
    );
}

// ── Test 11: prefetch-safe recovers from worker panic ───────────────────────────

/// Regression test for panic-safe in-flight cleanup:
/// one message panics once in a worker, then is retried and successfully
/// processed without blocking subsequent messages.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_run_recovers_from_worker_panic() {
    let url = shared_redis_url().await;

    let stream = "prefetch.panic-recovery.events";
    let dead_stream = "prefetch.panic-recovery.events:dead";
    let expected_messages: u32 = 3;

    let config = RedisConsumerConfigBuilder::new()
        .stream(stream)
        .group_name("prefetch-panic-recovery-group")
        .consumer_name("consumer-1")
        .dead_stream(dead_stream)
        .max_retries(5)
        .claim_min_idle(Duration::from_millis(300))
        .claim_interval(Duration::from_millis(400))
        .prefetch_count(8)
        .block_ms(1000)
        .build_prefetch()
        .unwrap();

    let panic_once = Arc::new(AtomicBool::new(false));
    let panic_count = Arc::new(AtomicU32::new(0));
    let success_calls = Arc::new(AtomicU32::new(0));
    let processed_blocks = Arc::new(Mutex::new(HashSet::<u64>::new()));

    let handler = {
        let panic_once = panic_once.clone();
        let panic_count = panic_count.clone();
        let success_calls = success_calls.clone();
        let processed_blocks = processed_blocks.clone();
        AsyncHandlerPayloadOnly::new(move |event: BlockEvent| {
            let panic_once = panic_once.clone();
            let panic_count = panic_count.clone();
            let success_calls = success_calls.clone();
            let processed_blocks = processed_blocks.clone();
            async move {
                if event.block_number == 1 && !panic_once.swap(true, Ordering::SeqCst) {
                    panic_count.fetch_add(1, Ordering::SeqCst);
                    panic!("intentional worker panic for recovery regression test");
                }

                success_calls.fetch_add(1, Ordering::SeqCst);
                processed_blocks.lock().unwrap().insert(event.block_number);
                Ok::<(), std::convert::Infallible>(())
            }
        })
    };

    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    tokio::time::sleep(Duration::from_millis(400)).await;

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::new(conn);
    for block in 1..=u64::from(expected_messages) {
        publisher
            .publish(
                stream,
                &BlockEvent {
                    block_number: block,
                },
            )
            .await
            .unwrap();
    }

    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    while tokio::time::Instant::now() < deadline {
        let unique = processed_blocks.lock().unwrap().len();
        if unique == expected_messages as usize {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    consumer_handle.abort();

    let processed = processed_blocks.lock().unwrap().clone();
    assert_eq!(
        panic_count.load(Ordering::SeqCst),
        1,
        "handler should panic exactly once for block 1"
    );
    assert_eq!(
        processed.len(),
        expected_messages as usize,
        "all published blocks should eventually be processed despite one worker panic"
    );
    assert!(
        processed.contains(&1),
        "the block that panicked initially should be retried and processed"
    );
    assert_eq!(
        success_calls.load(Ordering::SeqCst),
        expected_messages,
        "successful handler calls should match published message count"
    );
}

// Helpers for auto-trim assertions.
async fn redis_xlen(url: &str, stream: &str) -> u64 {
    let client = redis::Client::open(url).unwrap();
    let mut raw_conn = client.get_multiplexed_async_connection().await.unwrap();
    redis::cmd("XLEN")
        .arg(stream)
        .query_async(&mut raw_conn)
        .await
        .unwrap()
}

async fn redis_group_count(url: &str, stream: &str) -> usize {
    let client = redis::Client::open(url).unwrap();
    let mut raw_conn = client.get_multiplexed_async_connection().await.unwrap();
    let groups: redis::Value = redis::cmd("XINFO")
        .arg("GROUPS")
        .arg(stream)
        .query_async(&mut raw_conn)
        .await
        .unwrap();

    match groups {
        redis::Value::Array(items) => items.len(),
        _ => 0,
    }
}

// ── Test: auto_trim + fallback_maxlen works without groups ─────────────────────

#[tokio::test]
#[ignore = "requires Docker"]
async fn test_auto_trim_with_fallback_trims_without_groups() {
    let url = shared_redis_url().await;
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let stream = format!("autotrim.fallback.no-groups.{suffix}");
    let fallback_maxlen: u64 = 100;
    let publish_count: u64 = 5000;

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::builder(conn)
        .auto_trim(Duration::from_millis(200))
        .fallback_maxlen(fallback_maxlen as usize)
        .build();

    for i in 0..publish_count {
        publisher
            .publish(&stream, &BlockEvent { block_number: i })
            .await
            .unwrap();
    }

    assert_eq!(
        redis_group_count(url, &stream).await,
        0,
        "this test must run with no consumer groups to exercise fallback_maxlen"
    );

    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    let mut last_len = redis_xlen(url, &stream).await;

    while tokio::time::Instant::now() < deadline {
        last_len = redis_xlen(url, &stream).await;

        // MAXLEN ~ is approximate, so use tolerant threshold.
        if last_len <= fallback_maxlen * 10 {
            assert!(
                last_len < publish_count,
                "fallback trimming should reduce stream length (len={last_len}, published={publish_count})"
            );
            publisher.shutdown().await;
            return;
        }

        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    publisher.shutdown().await;
    panic!(
        "fallback auto-trim did not converge near target: last_len={last_len}, fallback_maxlen={fallback_maxlen}, published={publish_count}"
    );
}

// ── Test: auto_trim works without fallback via group progress (MINID path) ────

#[tokio::test]
#[ignore = "requires Docker"]
async fn test_auto_trim_without_fallback_trims_with_consumer_group_progress() {
    let url = shared_redis_url().await;
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let stream = format!("autotrim.no-fallback.group.{suffix}");
    let group = format!("autotrim-no-fallback-group-{suffix}");
    let publish_count: u32 = 1500;

    let consumed = Arc::new(AtomicU32::new(0));
    let consumed_clone = consumed.clone();

    let handler = AsyncHandlerPayloadOnly::new(move |_: BlockEvent| {
        let c = consumed_clone.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            Ok::<(), std::io::Error>(())
        }
    });

    let config = RedisConsumerConfigBuilder::new()
        .stream(&stream)
        .group_name(&group)
        .consumer_name("consumer-1")
        .dead_stream(format!("{stream}:dead"))
        .prefetch_count(256)
        .block_ms(200)
        .build_prefetch()
        .unwrap();

    let consumer = RedisConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    tokio::time::sleep(Duration::from_millis(500)).await;

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::builder(conn)
        .auto_trim(Duration::from_millis(200)) // no fallback_maxlen on purpose
        .build();

    for i in 0..publish_count {
        publisher
            .publish(
                &stream,
                &BlockEvent {
                    block_number: i as u64,
                },
            )
            .await
            .unwrap();
    }

    let consume_deadline = tokio::time::Instant::now() + Duration::from_secs(25);
    while consumed.load(Ordering::SeqCst) < publish_count
        && tokio::time::Instant::now() < consume_deadline
    {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    assert_eq!(
        consumed.load(Ordering::SeqCst),
        publish_count,
        "consumer should process+ack all messages so trimmer can advance safe MINID"
    );

    assert!(
        redis_group_count(url, &stream).await >= 1,
        "this test requires a consumer group to exercise MINID trimming path"
    );

    // Give ACKs a brief moment to settle before asserting trim.
    tokio::time::sleep(Duration::from_millis(500)).await;

    let trim_deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    let mut last_len = redis_xlen(url, &stream).await;

    while tokio::time::Instant::now() < trim_deadline {
        last_len = redis_xlen(url, &stream).await;

        // MINID ~ is approximate; tolerate some tail.
        if last_len <= 500 {
            consumer_handle.abort();
            publisher.shutdown().await;
            return;
        }

        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    consumer_handle.abort();
    publisher.shutdown().await;
    panic!(
        "no-fallback auto-trim did not trim by group progress: last_len={last_len}, published={publish_count}"
    );
}

// ── Stream depth introspection tests ────────────────────────────────────────

/// Verify `RedisQueueInspector::queue_depths` returns correct counts for
/// principal and dead-letter streams after publishing messages.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_queue_depth_introspection() {
    use broker::redis::RedisQueueInspector;
    use broker::traits::depth::QueueInspector;

    let url = shared_redis_url().await;
    let stream = "depth.events";
    let dead_stream = "depth.events:dead";

    // Publish 5 messages to the principal stream.
    let conn = RedisConnectionManager::new(url).await.unwrap();
    let publisher = RedisPublisher::builder(conn.clone()).build();
    for i in 0..5u64 {
        publisher
            .publish(stream, &BlockEvent { block_number: i })
            .await
            .unwrap();
    }

    // Also push 2 messages directly to the dead-letter stream to simulate DLQ entries.
    let publisher_dead = RedisPublisher::builder(conn.clone()).build();
    for i in 100..102u64 {
        publisher_dead
            .publish(dead_stream, &BlockEvent { block_number: i })
            .await
            .unwrap();
    }

    // Query depth without group (stream-level only).
    let inspector = RedisQueueInspector::new(conn);
    let depths = inspector.queue_depths(stream, None).await.unwrap();

    assert_eq!(
        depths.principal, 5,
        "5 messages published to principal stream"
    );
    assert_eq!(
        depths.retry, None,
        "Redis retry is PEL-based, no separate stream"
    );
    assert_eq!(depths.dead_letter, 2, "2 messages in dead-letter stream");
    assert_eq!(depths.total(), 7);
    assert!(!depths.is_empty());
    assert_eq!(depths.pending, None, "no group queried");
    assert_eq!(depths.lag, None, "no group queried");
    // Without group-level metrics, has_pending_work falls back to principal > 0.
    assert!(depths.has_pending_work());
}

/// Verify `RedisQueueInspector::queue_depths` returns zeros for a stream
/// that does not exist (XLEN on a non-existent key returns 0).
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_queue_depth_nonexistent_stream_returns_zeros() {
    use broker::redis::RedisQueueInspector;
    use broker::traits::depth::QueueInspector;

    let url = shared_redis_url().await;
    let conn = RedisConnectionManager::new(url).await.unwrap();
    let inspector = RedisQueueInspector::new(conn);

    let depths = inspector
        .queue_depths("nonexistent.stream.depth.test", None)
        .await
        .unwrap();

    assert_eq!(depths.principal, 0);
    assert_eq!(depths.retry, None);
    assert_eq!(depths.dead_letter, 0);
    assert!(depths.is_empty());
}

/// Verify `is_empty` returns `true` when the stream does not exist.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_is_empty_nonexistent_stream() {
    use broker::redis::RedisQueueInspector;
    use broker::traits::depth::QueueInspector;

    let url = shared_redis_url().await;
    let conn = RedisConnectionManager::new(url).await.unwrap();
    let inspector = RedisQueueInspector::new(conn);

    let empty = inspector
        .is_empty("nonexistent.is_empty.test", "some-group")
        .await
        .unwrap();

    assert!(empty, "non-existent stream should be empty");
}

/// Verify `is_empty` returns `false` when a consumer group has pending
/// messages (published, read by group, but not ACKed).
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_is_empty_with_pending_messages() {
    use broker::redis::RedisQueueInspector;
    use broker::traits::depth::QueueInspector;
    use redis::AsyncCommands;

    let url = shared_redis_url().await;
    let stream = "is_empty.pending.test";
    let group = "test-group";
    let consumer = "test-consumer";

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let mut raw_conn = conn.get_connection();

    // Publish a message.
    let _: String = raw_conn
        .xadd(stream, "*", &[("data", "hello")])
        .await
        .unwrap();

    // Create group and read (delivers to PEL but don't ACK).
    let _: () = redis::cmd("XGROUP")
        .arg("CREATE")
        .arg(stream)
        .arg(group)
        .arg("0")
        .arg("MKSTREAM")
        .query_async(&mut raw_conn)
        .await
        .unwrap();

    let _: redis::Value = redis::cmd("XREADGROUP")
        .arg("GROUP")
        .arg(group)
        .arg(consumer)
        .arg("COUNT")
        .arg(1)
        .arg("STREAMS")
        .arg(stream)
        .arg(">")
        .query_async(&mut raw_conn)
        .await
        .unwrap();

    // Now pending=1, should NOT be empty.
    let inspector = RedisQueueInspector::new(conn);
    let empty = inspector.is_empty(stream, group).await.unwrap();
    assert!(!empty, "stream with pending messages should not be empty");
}

/// Verify `is_empty` returns `true` when all messages are consumed and ACKed.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_is_empty_all_acked() {
    use broker::redis::RedisQueueInspector;
    use broker::traits::depth::QueueInspector;
    use redis::AsyncCommands;

    let url = shared_redis_url().await;
    let stream = "is_empty.acked.test";
    let group = "test-group";
    let consumer = "test-consumer";

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let mut raw_conn = conn.get_connection();

    // Publish, create group, read, then ACK.
    let id: String = raw_conn
        .xadd(stream, "*", &[("data", "hello")])
        .await
        .unwrap();

    let _: () = redis::cmd("XGROUP")
        .arg("CREATE")
        .arg(stream)
        .arg(group)
        .arg("0")
        .arg("MKSTREAM")
        .query_async(&mut raw_conn)
        .await
        .unwrap();

    let _: redis::Value = redis::cmd("XREADGROUP")
        .arg("GROUP")
        .arg(group)
        .arg(consumer)
        .arg("COUNT")
        .arg(1)
        .arg("STREAMS")
        .arg(stream)
        .arg(">")
        .query_async(&mut raw_conn)
        .await
        .unwrap();

    let _: u64 = raw_conn.xack(stream, group, &[&id]).await.unwrap();

    // pending=0, lag=0 (all delivered and ACKed). XLEN=1 but is_empty=true.
    let inspector = RedisQueueInspector::new(conn);
    let empty = inspector.is_empty(stream, group).await.unwrap();
    assert!(
        empty,
        "stream with all messages ACKed should be empty (even though XLEN > 0)"
    );
}

/// Verify `exists` returns `false` for a stream that does not exist.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_exists_nonexistent_stream() {
    use broker::redis::RedisQueueInspector;
    use broker::traits::depth::QueueInspector;

    let url = shared_redis_url().await;
    let conn = RedisConnectionManager::new(url).await.unwrap();
    let inspector = RedisQueueInspector::new(conn);

    let found = inspector.exists("nonexistent.exists.test").await.unwrap();

    assert!(!found, "non-existent stream should return false");
}

/// Verify `exists` returns `true` for a stream that has been created.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_exists_after_xadd() {
    use broker::redis::RedisQueueInspector;
    use broker::traits::depth::QueueInspector;
    use redis::AsyncCommands;

    let url = shared_redis_url().await;
    let stream = "exists.after_xadd.test";

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let mut raw_conn = conn.get_connection();

    let _: String = raw_conn
        .xadd(stream, "*", &[("data", "hello")])
        .await
        .unwrap();

    let inspector = RedisQueueInspector::new(conn);
    let found = inspector.exists(stream).await.unwrap();

    assert!(found, "stream created via XADD should exist");
}

/// Verify `exists` returns `false` for a non-stream key (e.g. a plain string).
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_exists_returns_false_for_non_stream_key() {
    use broker::redis::RedisQueueInspector;
    use broker::traits::depth::QueueInspector;
    use redis::AsyncCommands;

    let url = shared_redis_url().await;
    let key = "exists.string_key.test";

    let conn = RedisConnectionManager::new(url).await.unwrap();
    let mut raw_conn = conn.get_connection();

    let _: () = raw_conn.set(key, "not-a-stream").await.unwrap();

    let inspector = RedisQueueInspector::new(conn);
    let found = inspector.exists(key).await.unwrap();

    assert!(!found, "TYPE check should return false for non-stream keys");
}
