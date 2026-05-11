#![cfg(feature = "amqp")]
//! End-to-end integration tests for `broker::amqp`.
//!
//! Each test follows the three-step pattern from the `broker` README:
//!   1. Define a payload type.
//!   2. Write a handler with `broker::AsyncHandlerPayloadOnly`.
//!   3. Pick the `broker::amqp` backend, build config, and call `consumer.run_*()`.
//!
//! All tests share a single RabbitMQ container (`e2e-rabbitmq`) using
//! `ReuseDirective::CurrentSession` so the container survives the handle drop
//! inside the `OnceCell` init closure. Each test uses unique exchange/queue
//! names to avoid cross-contamination.
//!
//! ⚠️  Always run via Make — the Makefile removes the named container before
//! and after the suite so it is not left running between runs:
//!
//! ```bash
//! make test-e2e-amqp
//! ```
//!
//! Running `cargo test -p broker --features amqp --test amqp_e2e -- --include-ignored` directly
//! will leave `e2e-rabbitmq` running after the suite finishes.

use std::{
    collections::{HashMap, HashSet},
    future::Future,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU32, Ordering},
    },
    time::Duration,
};

// Traits and shared types come from `broker` — the backend-agnostic layer.
use broker::traits::{Consumer, DynPublisher, Publisher};
use broker::{AckDecision, AsyncHandlerPayloadOnly, AsyncHandlerWithContext, MessageMetadata};
// Backend-specific construction (config builder, topology, concrete types) comes from `broker::amqp`.
use broker::amqp::{
    ConnectionManager, ConsumerConfigBuilder, ExchangeManager, ExchangeTopology, PublisherError,
    RmqConsumer, RmqPublisher,
};

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
/// publishing them to a chain-specific RMQ exchange through `DynPublisher`.
async fn publish_blocks_at_rps(
    publisher: &DynPublisher,
    routing_key: &str,
    chain_id: u64,
    start_block: u64,
    count: u64,
    rps: u64,
) -> Result<(), broker::traits::DynPublishError> {
    let per_message_delay = Duration::from_millis((1000 / rps.max(1)).max(1));
    for i in 0..count {
        publisher
            .publish(
                routing_key,
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

// ── Shared container ─────────────────────────────────────────────────────────

use tokio::sync::OnceCell;

static RABBITMQ_URL: OnceCell<String> = OnceCell::const_new();

/// Lazily start a single RabbitMQ container shared across all tests.
///
/// `ReuseDirective::CurrentSession` prevents the `Drop` impl from stopping
/// the container when the `ContainerAsync` handle goes out of scope at the
/// end of the `OnceCell` init closure. The container therefore stays alive
/// for all tests in the suite. Cleanup is handled by `make test-e2e-amqp`
/// via `docker rm -f e2e-rabbitmq`.
async fn shared_rabbitmq_url() -> &'static str {
    RABBITMQ_URL
        .get_or_init(|| async {
            use testcontainers::core::{ImageExt, ReuseDirective};
            use testcontainers::runners::AsyncRunner;
            use testcontainers_modules::rabbitmq::RabbitMq;

            let container = RabbitMq::default()
                .with_container_name("e2e-rabbitmq")
                .with_reuse(ReuseDirective::CurrentSession)
                .start()
                .await
                .unwrap();
            let port = container.get_host_port_ipv4(5672).await.unwrap();
            format!("amqp://guest:guest@127.0.0.1:{port}/%2f")
        })
        .await
        .as_str()
}

// ── Generic Consumer trait helper ─────────────────────────────────────────────

/// Assert that a single message published via `publish_fn` is received exactly once.
///
/// Bound on `broker::traits::Consumer` — exercises the trait directly, independent of which
/// backend (`RmqConsumer`, `RedisConsumer`, …) is passed in.
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

    // Give the consumer time to start and register with RabbitMQ.
    tokio::time::sleep(Duration::from_millis(600)).await;

    publish_fn.await;

    // Poll until the handler is called or 8 s elapse.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
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

// ── Test 1: run_simple roundtrip ──────────────────────────────────────────────

/// Pre-declare topology, publish one message, consumer receives and ACKs it.
/// Exercises the generic `broker::traits::Consumer` trait via `assert_simple_roundtrip`.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_run_simple_roundtrip() {
    let url = shared_rabbitmq_url().await;

    let exchange = "test.exchange";
    let queue = "test.queue";
    let routing_key = "test.key";

    let topology = ExchangeTopology::from_prefix(exchange);
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    // Step 3 (backend): publisher + consumer from broker_amqp
    // Exchange is fixed at construction; topic = AMQP routing key (broker::traits::Publisher trait).
    let publisher = RmqPublisher::connect(url, exchange).await;

    let config = ConsumerConfigBuilder::new()
        .with_topology(&topology)
        .queue(queue)
        .routing_key(routing_key)
        .consumer_tag("e2e-consumer")
        .max_retries(3)
        .retry_delay(Duration::from_millis(200))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    let consumer = RmqConsumer::connect(url).await.unwrap();

    assert_simple_roundtrip(consumer, config, async move {
        // broker::traits::Publisher trait — topic = routing_key; exchange is fixed in the publisher.
        publisher
            .publish(routing_key, &BlockEvent { block_number: 42 })
            .await
            .unwrap();
    })
    .await;
}

// ── Test 2: run_with_retry — handler fails then succeeds ──────────────────────

/// `run_with_retry` declares its own retry/DLX topology. The handler fails on
/// the first two deliveries and succeeds on the third. RabbitMQ re-delivers via
/// DLX + TTL after each failure.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_run_with_retry_eventually_succeeds() {
    let url = shared_rabbitmq_url().await;

    // AMQP requires exchanges to be declared before queue_bind — done here before consumer starts.
    let topology = ExchangeTopology {
        main: "retry.exchange".to_string(),
        retry: "retry.exchange.retry".to_string(),
        dlx: "retry.exchange.dlx".to_string(),
    };
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    // Step 3 (backend): config from broker_amqp; short retry_delay so the test finishes quickly.
    let config = ConsumerConfigBuilder::new()
        .exchange("retry.exchange")
        .queue("retry.queue")
        .routing_key("retry.key")
        .consumer_tag("retry-consumer")
        .retry_exchange("retry.exchange.retry")
        .dead_exchange("retry.exchange.dlx")
        .max_retries(5)
        .retry_delay(Duration::from_millis(500))
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

    // Step 3 (backend): consumer from broker_amqp, run in background
    let consumer = RmqConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    // Give the consumer time to start and declare all queues + bindings.
    tokio::time::sleep(Duration::from_millis(800)).await;

    // Publish via broker::traits::Publisher — topic = routing_key; exchange fixed at construction.
    let publisher = RmqPublisher::connect(url, "retry.exchange").await;
    publisher
        .publish("retry.key", &BlockEvent { block_number: 1 })
        .await
        .unwrap();

    // Poll for up to 20 s — each retry takes retry_delay (500 ms) + broker round-trip.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    while call_count.load(Ordering::SeqCst) < 3 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

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
    let url = shared_rabbitmq_url().await;

    let exchange = "batch.exchange";
    let queue = "batch.queue";
    let routing_key = "batch.key";

    let topology = ExchangeTopology::from_prefix(exchange);
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    // Step 3 (backend): config from broker_amqp
    let config = ConsumerConfigBuilder::new()
        .with_topology(&topology)
        .queue(queue)
        .routing_key(routing_key)
        .consumer_tag("batch-consumer")
        .max_retries(3)
        .retry_delay(Duration::from_millis(200))
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

    // Step 3 (backend): consumer from broker_amqp, run in background
    let consumer = RmqConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    // Give consumer time to start.
    tokio::time::sleep(Duration::from_millis(600)).await;

    // Publish 3 messages via broker::traits::Publisher — topic = routing_key; exchange fixed at construction.
    let publisher = RmqPublisher::connect(url, exchange).await;
    let events = vec![
        BlockEvent { block_number: 1 },
        BlockEvent { block_number: 2 },
        BlockEvent { block_number: 3 },
    ];
    for event in &events {
        publisher.publish(routing_key, event).await.unwrap();
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

// ── Test 4: AckDecision::Dead — bypasses all retry cycles ────────────────────

/// Verifies that a handler returning `AckDecision::Dead` routes the message
/// directly to the `.error` queue on the **first delivery**, without burning
/// any retry cycles.
///
/// Without `AckDecision::Dead`, the only way to reach the error queue was to
/// exhaust `max_retries`. This test confirms the fast-path works.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_ack_decision_dead_skips_retry() {
    let url = shared_rabbitmq_url().await;

    let topology = ExchangeTopology {
        main: "dead.exchange".to_string(),
        retry: "dead.exchange.retry".to_string(),
        dlx: "dead.exchange.dlx".to_string(),
    };
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    let config = ConsumerConfigBuilder::new()
        .exchange("dead.exchange")
        .queue("dead.queue")
        .routing_key("dead.key")
        .consumer_tag("dead-consumer")
        .retry_exchange("dead.exchange.retry")
        .dead_exchange("dead.exchange.dlx")
        // max_retries=5 but handler should never retry at all
        .max_retries(5)
        .retry_delay(Duration::from_millis(200))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    let call_count = Arc::new(AtomicU32::new(0));

    // Custom Handler impl that always returns AckDecision::Dead.
    // AsyncHandlerPayloadOnly always maps Ok(()) → Ack, so we implement Handler directly
    // to return Dead without going through the error path.
    struct DeadHandler(Arc<AtomicU32>);

    #[async_trait::async_trait]
    impl broker::Handler for DeadHandler {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(AckDecision::Dead)
        }
    }

    let dead_handler = DeadHandler(call_count.clone());

    let consumer = RmqConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, dead_handler).await;
    });

    tokio::time::sleep(Duration::from_millis(800)).await;

    let publisher = RmqPublisher::connect(url, "dead.exchange").await;
    publisher
        .publish("dead.key", &BlockEvent { block_number: 99 })
        .await
        .unwrap();

    // Poll up to 8 s — the message should be handled once immediately (no retry delay)
    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    while call_count.load(Ordering::SeqCst) == 0 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Give a bit of extra time to confirm there are no retry deliveries
    tokio::time::sleep(Duration::from_millis(1500)).await;

    consumer_handle.abort();

    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "handler should be called exactly once — AckDecision::Dead must not retry"
    );
}

// ── Helper: query AMQP queue depth ───────────────────────────────────────────

/// Returns the number of ready messages in the named AMQP queue.
///
/// Uses a passive `queue_declare` which inspects the queue without modifying it.
/// Returns 0 if the queue does not yet exist.
async fn amqp_queue_depth(url: &str, queue: &str) -> u32 {
    use lapin::{
        Connection, ConnectionProperties,
        options::QueueDeclareOptions,
        types::{FieldTable, ShortString},
    };

    let Ok(conn) = Connection::connect(url, ConnectionProperties::default()).await else {
        return 0;
    };
    let Ok(channel) = conn.create_channel().await else {
        return 0;
    };

    match channel
        .queue_declare(
            ShortString::from(queue),
            QueueDeclareOptions {
                passive: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
    {
        Ok(q) => q.message_count(),
        Err(_) => 0,
    }
}

// ── Test 5: Circuit Breaker — halts consumption during the cooldown window ────
//
// Handler returns `HandlerError::Transient` for the first 3 deliveries, tripping
// the circuit (threshold=3), then `Ok(Ack)` for all subsequent deliveries.
//
// After the circuit opens we publish a "signal" message (block_number=99) and
// assert it is NOT consumed during the cooldown window, then IS consumed once
// the circuit transitions through Half-Open back to Closed.
//
// A long retry_delay (60 s) prevents NACKed messages from cycling back through
// the retry queue within the test window, keeping the handler call count clean.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_circuit_breaker_halts_consumption() {
    let url = shared_rabbitmq_url().await;

    let topology = ExchangeTopology {
        main: "cb-halt.exchange".to_string(),
        retry: "cb-halt.exchange.retry".to_string(),
        dlx: "cb-halt.exchange.dlx".to_string(),
    };
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    let config = ConsumerConfigBuilder::new()
        .exchange("cb-halt.exchange")
        .queue("cb-halt.queue")
        .routing_key("cb-halt.key")
        .consumer_tag("cb-halt-consumer")
        .retry_exchange("cb-halt.exchange.retry")
        .dead_exchange("cb-halt.exchange.dlx")
        .max_retries(10)
        // Long retry_delay keeps NACKed messages in the retry queue for the
        // entire test duration — they cannot cycle back and inflate the count.
        .retry_delay(Duration::from_secs(60))
        .circuit_breaker_threshold(3)
        .circuit_breaker_cooldown(Duration::from_millis(2000))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    let transient_count = Arc::new(AtomicU32::new(0));
    let signal_received = Arc::new(AtomicBool::new(false));

    // Handler:
    //   block_number == 99  → signal message: record and ACK immediately.
    //   otherwise           → increment transient_count; return Transient for
    //                         the first 3 calls (trips CB), Ok for the rest
    //                         (Half-Open probe and any PEL re-deliveries).
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

    let consumer = RmqConsumer::connect(url).await.unwrap();
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

    // Give the consumer time to start and declare all queues.
    tokio::time::sleep(Duration::from_millis(800)).await;

    let publisher = RmqPublisher::connect(url, "cb-halt.exchange").await;

    // Publish 3 trip messages — each fails with Transient, opening the circuit
    // after the third failure (threshold = 3).
    for i in 0..3u64 {
        publisher
            .publish("cb-halt.key", &BlockEvent { block_number: i })
            .await
            .unwrap();
    }

    // Wait until all 3 transient failures are recorded (circuit is now Open).
    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    while transient_count.load(Ordering::SeqCst) < 3 && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert_eq!(
        transient_count.load(Ordering::SeqCst),
        3,
        "expected exactly 3 transient failures to trip the circuit"
    );

    // ── Circuit is Open ───────────────────────────────────────────────────────
    // Publish the signal message — it must NOT be consumed during the cooldown.
    publisher
        .publish("cb-halt.key", &BlockEvent { block_number: 99 })
        .await
        .unwrap();

    // Observe: mid-cooldown (1 s into the 2 s window), signal must still be unprocessed.
    tokio::time::sleep(Duration::from_millis(1000)).await;
    assert!(
        !signal_received.load(Ordering::SeqCst),
        "signal message must not be consumed while the circuit is Open"
    );

    // ── After cooldown: Half-Open → probe succeeds → circuit Closed ───────────
    // The signal is the next new message; handler returns Ok → circuit closes.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    while !signal_received.load(Ordering::SeqCst) && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    consumer_handle.abort();
    assert!(
        signal_received.load(Ordering::SeqCst),
        "signal message must be consumed after the CB cooldown expires"
    );
}

// ── Test 6: Circuit Breaker — prevents DLQ pollution during a sustained outage ─
//
// A handler that always returns `HandlerError::Transient` simulates a downstream
// outage (e.g., database is unavailable).
//
// Transient failures now retry indefinitely and never consume `max_retries`.
// Without a circuit breaker, messages keep cycling through the retry queue.
// With CB (threshold=2, cooldown=5 s) the consumer pauses after 2 failures;
// messages accumulate in the retry queue and the error queue stays empty.
//
// The long retry_delay (30 s) ensures no messages return from the retry queue
// during the 3 s observation window.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_circuit_breaker_prevents_dlq_pollution() {
    let url = shared_rabbitmq_url().await;

    let topology = ExchangeTopology {
        main: "cb-dlq.exchange".to_string(),
        retry: "cb-dlq.exchange.retry".to_string(),
        dlx: "cb-dlq.exchange.dlx".to_string(),
    };
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    let config = ConsumerConfigBuilder::new()
        .exchange("cb-dlq.exchange")
        .queue("cb-dlq.queue")
        .routing_key("cb-dlq.key")
        .consumer_tag("cb-dlq-consumer")
        .retry_exchange("cb-dlq.exchange.retry")
        .dead_exchange("cb-dlq.exchange.dlx")
        .max_retries(3)
        .retry_delay(Duration::from_secs(30))
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

    let consumer = RmqConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, AlwaysTransient).await;
    });

    // Wait for the consumer to start and declare all queues, including the error queue.
    tokio::time::sleep(Duration::from_millis(800)).await;

    let publisher = RmqPublisher::connect(url, "cb-dlq.exchange").await;

    // Publish 2 messages — exactly at the failure threshold. Both fail with Transient,
    // opening the circuit after the second failure.
    publisher
        .publish("cb-dlq.key", &BlockEvent { block_number: 1 })
        .await
        .unwrap();
    publisher
        .publish("cb-dlq.key", &BlockEvent { block_number: 2 })
        .await
        .unwrap();

    // Wait for the CB to open and settle. cooldown = 5 s; we check at 3 s —
    // still within the Open window, so no further messages are consumed.
    tokio::time::sleep(Duration::from_secs(3)).await;

    // The error queue is named "{queue}.error" (declared by setup_retry_queues).
    let error_queue_depth = amqp_queue_depth(url, "cb-dlq.queue.error").await;
    consumer_handle.abort();

    assert_eq!(
        error_queue_depth, 0,
        "circuit breaker must prevent messages from reaching the DLQ during a transient outage"
    );
}

// ── Test 7: Transient failures retry indefinitely (bounded retry ignored) ───────
//
// With max_retries=1, a transient failure should still keep retrying forever.
// We assert:
// - multiple transient handler calls happen (more than max_retries+1 attempts)
// - error queue remains empty.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_transient_failures_retry_indefinitely() {
    let url = shared_rabbitmq_url().await;

    let topology = ExchangeTopology {
        main: "transient-infinite.exchange".to_string(),
        retry: "transient-infinite.exchange.retry".to_string(),
        dlx: "transient-infinite.exchange.dlx".to_string(),
    };
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    let config = ConsumerConfigBuilder::new()
        .exchange("transient-infinite.exchange")
        .queue("transient-infinite.queue")
        .routing_key("transient-infinite.key")
        .consumer_tag("transient-infinite-consumer")
        .retry_exchange("transient-infinite.exchange.retry")
        .dead_exchange("transient-infinite.exchange.dlx")
        .max_retries(1)
        .retry_delay(Duration::from_millis(200))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    struct AlwaysTransient(Arc<AtomicU32>);

    #[async_trait::async_trait]
    impl broker::Handler for AlwaysTransient {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Err(broker::HandlerError::Transient(Box::new(
                std::io::Error::other("downstream unavailable"),
            )))
        }
    }

    let attempts = Arc::new(AtomicU32::new(0));
    let attempts_for_handler = attempts.clone();
    let consumer = RmqConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer
            .run(config, AlwaysTransient(attempts_for_handler))
            .await;
    });

    tokio::time::sleep(Duration::from_millis(700)).await;

    let publisher = RmqPublisher::connect(url, "transient-infinite.exchange").await;
    publisher
        .publish("transient-infinite.key", &BlockEvent { block_number: 1 })
        .await
        .unwrap();

    let attempts_deadline = tokio::time::Instant::now() + Duration::from_secs(12);
    while tokio::time::Instant::now() < attempts_deadline {
        let current = attempts.load(Ordering::SeqCst);
        if current >= 4 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let error_depth = amqp_queue_depth(url, "transient-infinite.queue.error").await;
    consumer_handle.abort();

    assert!(
        attempts.load(Ordering::SeqCst) >= 4,
        "transient message should keep retrying even with max_retries=1; attempts={}",
        attempts.load(Ordering::SeqCst)
    );
    assert_eq!(
        error_depth, 0,
        "transient failures must not be moved to DLQ by max_retries"
    );
}

// ── Test 8: Permanent failures still honor max_retries ──────────────────────────
//
// Permanent failures keep bounded retry semantics and should still dead-letter.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_permanent_failures_still_dead_letter_after_max_retries() {
    let url = shared_rabbitmq_url().await;

    let topology = ExchangeTopology {
        main: "permanent-bounded.exchange".to_string(),
        retry: "permanent-bounded.exchange.retry".to_string(),
        dlx: "permanent-bounded.exchange.dlx".to_string(),
    };
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    let config = ConsumerConfigBuilder::new()
        .exchange("permanent-bounded.exchange")
        .queue("permanent-bounded.queue")
        .routing_key("permanent-bounded.key")
        .consumer_tag("permanent-bounded-consumer")
        .retry_exchange("permanent-bounded.exchange.retry")
        .dead_exchange("permanent-bounded.exchange.dlx")
        .max_retries(1)
        .retry_delay(Duration::from_millis(200))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    struct AlwaysPermanent(Arc<AtomicU32>);

    #[async_trait::async_trait]
    impl broker::Handler for AlwaysPermanent {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Err(broker::HandlerError::permanent(std::io::Error::other(
                "invalid payload",
            )))
        }
    }

    let attempts = Arc::new(AtomicU32::new(0));
    let attempts_for_handler = attempts.clone();
    let consumer = RmqConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer
            .run(config, AlwaysPermanent(attempts_for_handler))
            .await;
    });

    tokio::time::sleep(Duration::from_millis(700)).await;

    let publisher = RmqPublisher::connect(url, "permanent-bounded.exchange").await;
    publisher
        .publish("permanent-bounded.key", &BlockEvent { block_number: 1 })
        .await
        .unwrap();

    let dlq_deadline = tokio::time::Instant::now() + Duration::from_secs(12);
    let error_depth = loop {
        let depth = amqp_queue_depth(url, "permanent-bounded.queue.error").await;
        if depth >= 1 || tokio::time::Instant::now() >= dlq_deadline {
            break depth;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    };

    consumer_handle.abort();

    assert!(
        attempts.load(Ordering::SeqCst) >= 2,
        "permanent failure should be attempted then retried before DLQ; attempts={}",
        attempts.load(Ordering::SeqCst)
    );
    assert_eq!(
        error_depth, 1,
        "permanent failure should reach DLQ after max_retries"
    );
}

// ── Test 9: Transient -> Permanent transition preserves permanent retry budget ──
//
// First delivery is transient (infinite path), then failures become permanent.
// With max_retries=1, we expect one permanent retry before DLQ:
//   transient -> permanent(retry) -> permanent(DLQ)
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_transient_then_permanent_eventually_dead_letters() {
    let url = shared_rabbitmq_url().await;

    let topology = ExchangeTopology {
        main: "transient-to-permanent.exchange".to_string(),
        retry: "transient-to-permanent.exchange.retry".to_string(),
        dlx: "transient-to-permanent.exchange.dlx".to_string(),
    };
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    let config = ConsumerConfigBuilder::new()
        .exchange("transient-to-permanent.exchange")
        .queue("transient-to-permanent.queue")
        .routing_key("transient-to-permanent.key")
        .consumer_tag("transient-to-permanent-consumer")
        .retry_exchange("transient-to-permanent.exchange.retry")
        .dead_exchange("transient-to-permanent.exchange.dlx")
        .max_retries(1)
        .retry_delay(Duration::from_millis(200))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    struct TransientThenPermanent(Arc<AtomicU32>);

    #[async_trait::async_trait]
    impl broker::Handler for TransientThenPermanent {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            let call_index = self.0.fetch_add(1, Ordering::SeqCst);
            if call_index == 0 {
                Err(broker::HandlerError::Transient(Box::new(
                    std::io::Error::other("downstream unavailable"),
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
    let consumer = RmqConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer
            .run(config, TransientThenPermanent(calls_for_handler))
            .await;
    });

    tokio::time::sleep(Duration::from_millis(700)).await;

    let publisher = RmqPublisher::connect(url, "transient-to-permanent.exchange").await;
    publisher
        .publish(
            "transient-to-permanent.key",
            &BlockEvent { block_number: 1 },
        )
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    let error_depth = loop {
        let depth = amqp_queue_depth(url, "transient-to-permanent.queue.error").await;
        if (depth >= 1 && calls.load(Ordering::SeqCst) >= 3)
            || tokio::time::Instant::now() >= deadline
        {
            break depth;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    };

    consumer_handle.abort();

    assert!(
        calls.load(Ordering::SeqCst) >= 3,
        "transient history must not consume permanent retry budget; calls={}",
        calls.load(Ordering::SeqCst)
    );
    assert_eq!(
        error_depth, 1,
        "message should DLQ only after bounded permanent retries"
    );
}

// ── Test 10: AsyncHandlerWithContext receives correct MessageMetadata ─────────
//
// Verifies that `AsyncHandlerWithContext` receives a `MessageMetadata` with:
// - `delivery_count == 1` on first delivery
// - `topic` matching the queue name
// - `id` being a non-empty delivery tag
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_async_handler_with_context_metadata() {
    let url = shared_rabbitmq_url().await;

    let exchange = "ctx.exchange";
    let queue = "ctx.queue";
    let routing_key = "ctx.key";

    let topology = ExchangeTopology::from_prefix(exchange);
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    let config = ConsumerConfigBuilder::new()
        .with_topology(&topology)
        .queue(queue)
        .routing_key(routing_key)
        .consumer_tag("ctx-consumer")
        .max_retries(3)
        .retry_delay(Duration::from_millis(200))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    let (meta_tx, mut meta_rx) = tokio::sync::mpsc::channel::<(BlockEvent, MessageMetadata)>(1);

    let handler = AsyncHandlerWithContext::new(move |event: BlockEvent, meta: MessageMetadata| {
        let tx = meta_tx.clone();
        async move {
            let _ = tx.send((event, meta)).await;
            Ok::<(), std::io::Error>(())
        }
    });

    let consumer = RmqConsumer::connect(url).await.unwrap();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer.run(config, handler).await;
    });

    tokio::time::sleep(Duration::from_millis(600)).await;

    let publisher = RmqPublisher::connect(url, exchange).await;
    publisher
        .publish(routing_key, &BlockEvent { block_number: 42 })
        .await
        .unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    let received = tokio::select! {
        result = meta_rx.recv() => result,
        _ = tokio::time::sleep_until(deadline) => None,
    };

    consumer_handle.abort();

    let (event, meta) = received.expect("handler should have received the message");

    assert_eq!(
        event.block_number, 42,
        "payload should be deserialized correctly"
    );
    assert_eq!(
        meta.delivery_count, 1,
        "first delivery should have delivery_count=1"
    );
    assert_eq!(meta.topic, queue, "topic should match the queue name");
    assert!(!meta.id.is_empty(), "id (delivery tag) should be non-empty");
}

// ── Test: RmqPublisher missing exchange returns typed error (no panic) ─────────────

/// Publisher constructed via `new`/`with_config` has exchange `None`. Trait `publish`
/// must return `Err(ExchangeNotConfigured)` instead of panicking.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_rmq_publisher_publish_without_exchange_returns_error() {
    let url = shared_rabbitmq_url().await;

    let connection = Arc::new(ConnectionManager::new(url));
    let publisher = RmqPublisher::with_config(connection, 10, 1000).await;

    let result = publisher
        .publish("test.key", &BlockEvent { block_number: 1 })
        .await;

    assert!(
        result.is_err(),
        "publish without exchange must return Err, got {:?}",
        result
    );

    let err = result.unwrap_err();
    assert!(
        matches!(err, PublisherError::ExchangeNotConfigured),
        "expected ExchangeNotConfigured, got {:?}",
        err
    );
}

// ── Test: DynPublisher + RMQ routing topology for multi-chain + multi-app ────
//
// Simulates listener-like block publishing at fixed RPS and verifies:
// - fanout to multiple app queues on the same chain
// - exchange isolation across chains
// - routing-key isolation between data (`blocks.#`) and control (`control.watch.#`)
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_dynpublisher_multichain_routing_for_multiple_apps() {
    let url = shared_rabbitmq_url().await;

    let eth_exchange = "dynpub.ethereum.events";
    let polygon_exchange = "dynpub.polygon.events";

    let app_a_eth_blocks_q = "dynpub.app-a.eth.blocks";
    let app_b_eth_blocks_q = "dynpub.app-b.eth.blocks";
    let app_a_polygon_blocks_q = "dynpub.app-a.polygon.blocks";
    let listener_watch_q = "dynpub.listener.eth.watch";

    let eth_topology = ExchangeTopology {
        main: eth_exchange.to_string(),
        retry: format!("{eth_exchange}.retry"),
        dlx: format!("{eth_exchange}.dlx"),
    };
    let polygon_topology = ExchangeTopology {
        main: polygon_exchange.to_string(),
        retry: format!("{polygon_exchange}.retry"),
        dlx: format!("{polygon_exchange}.dlx"),
    };

    let exchange_manager = ExchangeManager::with_addr(url);
    exchange_manager
        .declare_topology(&eth_topology)
        .await
        .unwrap();
    exchange_manager
        .declare_topology(&polygon_topology)
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

    let app_a_eth_config = ConsumerConfigBuilder::new()
        .with_topology(&eth_topology)
        .queue(app_a_eth_blocks_q)
        .routing_key("blocks.#")
        .consumer_tag("dynpub-app-a-eth")
        .max_retries(3)
        .retry_delay(Duration::from_secs(2))
        .prefetch_count(32)
        .build_prefetch()
        .unwrap();

    let app_b_eth_config = ConsumerConfigBuilder::new()
        .with_topology(&eth_topology)
        .queue(app_b_eth_blocks_q)
        .routing_key("blocks.#")
        .consumer_tag("dynpub-app-b-eth")
        .max_retries(3)
        .retry_delay(Duration::from_secs(2))
        .prefetch_count(32)
        .build_prefetch()
        .unwrap();

    let app_a_polygon_config = ConsumerConfigBuilder::new()
        .with_topology(&polygon_topology)
        .queue(app_a_polygon_blocks_q)
        .routing_key("blocks.#")
        .consumer_tag("dynpub-app-a-polygon")
        .max_retries(3)
        .retry_delay(Duration::from_secs(2))
        .prefetch_count(32)
        .build_prefetch()
        .unwrap();

    let watch_config = ConsumerConfigBuilder::new()
        .with_topology(&eth_topology)
        .queue(listener_watch_q)
        .routing_key("control.watch.#")
        .consumer_tag("dynpub-listener-watch")
        .max_retries(3)
        .retry_delay(Duration::from_secs(2))
        .prefetch_count(16)
        .build_prefetch()
        .unwrap();

    let app_a_eth_consumer = RmqConsumer::connect(url).await.unwrap();
    let app_b_eth_consumer = RmqConsumer::connect(url).await.unwrap();
    let app_a_polygon_consumer = RmqConsumer::connect(url).await.unwrap();
    let watch_consumer = RmqConsumer::connect(url).await.unwrap();

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
        DynPublisher::new(RmqPublisher::connect(url, eth_exchange).await),
    );
    publishers.insert(
        "polygon".to_string(),
        DynPublisher::new(RmqPublisher::connect(url, polygon_exchange).await),
    );

    publishers["ethereum"]
        .publish(
            "control.watch.register",
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
            "blocks.canonical",
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
            "blocks.canonical",
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
        "polygon queue should receive only polygon blocks"
    );
    assert_eq!(
        watch_count.load(Ordering::SeqCst),
        1,
        "watch queue should receive exactly one control message"
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

    // ── Content validation: verify received payloads match what was published ──
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

// ── Test 10: Shared exchange retry path is isolated per queue ─────────────────
//
// Two queues bind to the same main exchange/routing pattern. Queue A always fails
// permanently; Queue B always ACKs.
//
// Queue A retries must not leak into Queue B.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_shared_exchange_retry_isolated_per_queue() {
    let url = shared_rabbitmq_url().await;

    let topology = ExchangeTopology {
        main: "isolation.retry.exchange".to_string(),
        retry: "isolation.retry.exchange.retry".to_string(),
        dlx: "isolation.retry.exchange.dlx".to_string(),
    };
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    let queue_a = "isolation.retry.queue.a";
    let queue_b = "isolation.retry.queue.b";
    let routing_key = "blocks.#";

    let cfg_a = ConsumerConfigBuilder::new()
        .with_topology(&topology)
        .queue(queue_a)
        .routing_key(routing_key)
        .consumer_tag("isolation-retry-a")
        .max_retries(1)
        .retry_delay(Duration::from_millis(200))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    let cfg_b = ConsumerConfigBuilder::new()
        .with_topology(&topology)
        .queue(queue_b)
        .routing_key(routing_key)
        .consumer_tag("isolation-retry-b")
        .max_retries(1)
        .retry_delay(Duration::from_millis(200))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    let b_count = Arc::new(AtomicU32::new(0));

    // Queue A: always permanent failure
    let handler_a = AsyncHandlerPayloadOnly::new(|_: BlockEvent| async move {
        Err::<(), std::io::Error>(std::io::Error::other("forced permanent failure"))
    });

    // Queue B: always ACK
    let b_count_for_handler = b_count.clone();
    let handler_b = AsyncHandlerPayloadOnly::new(move |_: BlockEvent| {
        let count = b_count_for_handler.clone();
        async move {
            count.fetch_add(1, Ordering::SeqCst);
            Ok::<(), std::io::Error>(())
        }
    });

    let consumer_a = RmqConsumer::connect(url).await.unwrap();
    let consumer_b = RmqConsumer::connect(url).await.unwrap();

    let handle_a = tokio::spawn(async move {
        let _ = consumer_a.run(cfg_a, handler_a).await;
    });
    let handle_b = tokio::spawn(async move {
        let _ = consumer_b.run(cfg_b, handler_b).await;
    });

    tokio::time::sleep(Duration::from_millis(900)).await;

    let publisher = RmqPublisher::connect(url, &topology.main).await;
    publisher
        .publish("blocks.canonical", &BlockEvent { block_number: 42 })
        .await
        .unwrap();

    let first_delivery_deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    while b_count.load(Ordering::SeqCst) < 1
        && tokio::time::Instant::now() < first_delivery_deadline
    {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Allow Queue A retry + DLQ flow to complete; Queue B should not see extra deliveries.
    tokio::time::sleep(Duration::from_secs(2)).await;

    let a_error_queue = format!("{queue_a}.error");
    let b_error_queue = format!("{queue_b}.error");

    let a_error_deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    let a_error_depth = loop {
        let depth = amqp_queue_depth(url, &a_error_queue).await;
        if depth >= 1 || tokio::time::Instant::now() >= a_error_deadline {
            break depth;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    };
    let b_error_depth = amqp_queue_depth(url, &b_error_queue).await;

    handle_a.abort();
    handle_b.abort();

    assert_eq!(
        b_count.load(Ordering::SeqCst),
        1,
        "Queue B must receive exactly one original delivery (no retry leakage from Queue A)"
    );
    assert_eq!(
        a_error_depth, 1,
        "Queue A should dead-letter after max retries"
    );
    assert_eq!(
        b_error_depth, 0,
        "Queue B must not receive dead-letter traffic from Queue A"
    );
}

// ── Test 11: Shared exchange delay path is isolated per queue ─────────────────
//
// Queue A requests delayed requeue twice (`AckDecision::Delay`), then ACKs.
// Queue B should still see only the original message once.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_shared_exchange_delay_isolated_per_queue() {
    let url = shared_rabbitmq_url().await;

    let topology = ExchangeTopology {
        main: "isolation.delay.exchange".to_string(),
        retry: "isolation.delay.exchange.retry".to_string(),
        dlx: "isolation.delay.exchange.dlx".to_string(),
    };
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    let queue_a = "isolation.delay.queue.a";
    let queue_b = "isolation.delay.queue.b";
    let routing_key = "blocks.#";

    let cfg_a = ConsumerConfigBuilder::new()
        .with_topology(&topology)
        .queue(queue_a)
        .routing_key(routing_key)
        .consumer_tag("isolation-delay-a")
        .max_retries(5)
        .retry_delay(Duration::from_millis(100))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    let cfg_b = ConsumerConfigBuilder::new()
        .with_topology(&topology)
        .queue(queue_b)
        .routing_key(routing_key)
        .consumer_tag("isolation-delay-b")
        .max_retries(5)
        .retry_delay(Duration::from_millis(100))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    struct DelayThenAck {
        calls: Arc<AtomicU32>,
    }

    #[async_trait::async_trait]
    impl broker::Handler for DelayThenAck {
        async fn call(&self, _msg: &broker::Message) -> Result<AckDecision, broker::HandlerError> {
            let prev = self.calls.fetch_add(1, Ordering::SeqCst);
            if prev < 2 {
                Ok(AckDecision::Delay(Duration::from_millis(150)))
            } else {
                Ok(AckDecision::Ack)
            }
        }
    }

    let a_calls = Arc::new(AtomicU32::new(0));
    let b_count = Arc::new(AtomicU32::new(0));

    let handler_a = DelayThenAck {
        calls: a_calls.clone(),
    };
    let b_count_for_handler = b_count.clone();
    let handler_b = AsyncHandlerPayloadOnly::new(move |_: BlockEvent| {
        let count = b_count_for_handler.clone();
        async move {
            count.fetch_add(1, Ordering::SeqCst);
            Ok::<(), std::io::Error>(())
        }
    });

    let consumer_a = RmqConsumer::connect(url).await.unwrap();
    let consumer_b = RmqConsumer::connect(url).await.unwrap();

    let handle_a = tokio::spawn(async move {
        let _ = consumer_a.run(cfg_a, handler_a).await;
    });
    let handle_b = tokio::spawn(async move {
        let _ = consumer_b.run(cfg_b, handler_b).await;
    });

    tokio::time::sleep(Duration::from_millis(900)).await;

    let publisher = RmqPublisher::connect(url, &topology.main).await;
    publisher
        .publish("blocks.canonical", &BlockEvent { block_number: 99 })
        .await
        .unwrap();

    // Wait until Queue A completes its 2 delay cycles and final ACK.
    let a_done_deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while a_calls.load(Ordering::SeqCst) < 3 && tokio::time::Instant::now() < a_done_deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Additional buffer to catch potential leakage into Queue B.
    tokio::time::sleep(Duration::from_secs(1)).await;

    let b_error_queue = format!("{queue_b}.error");
    let b_error_depth = amqp_queue_depth(url, &b_error_queue).await;

    handle_a.abort();
    handle_b.abort();

    assert!(
        a_calls.load(Ordering::SeqCst) >= 3,
        "Queue A should process delayed redeliveries before final ACK"
    );
    assert_eq!(
        b_count.load(Ordering::SeqCst),
        1,
        "Queue B must receive exactly one original delivery (no delay leakage from Queue A)"
    );
    assert_eq!(
        b_error_depth, 0,
        "Queue B must not receive dead-letter traffic from Queue A delay flow"
    );
}

// ── Queue depth introspection tests ─────────────────────────────────────────

/// Verify `AmqpQueueInspector::queue_depths` returns correct counts for
/// principal, retry, and dead-letter queues after publishing messages.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_queue_depth_introspection() {
    use broker::amqp::AmqpQueueInspector;
    use broker::traits::depth::QueueInspector;

    let url = shared_rabbitmq_url().await;

    let exchange = "depth.exchange";
    let queue = "depth.queue";
    let routing_key = "depth.key";

    let topology = ExchangeTopology::from_prefix(exchange);
    ExchangeManager::with_addr(url)
        .declare_topology(&topology)
        .await
        .unwrap();

    // Declare the principal queue bound to the exchange so messages land there.
    let config = ConsumerConfigBuilder::new()
        .with_topology(&topology)
        .queue(queue)
        .routing_key(routing_key)
        .consumer_tag("depth-consumer")
        .max_retries(3)
        .retry_delay(Duration::from_millis(200))
        .prefetch_count(10)
        .build_prefetch()
        .unwrap();

    // Start a consumer briefly to force queue declaration, then drop it.
    let consumer = RmqConsumer::connect(url).await.unwrap();
    let noop_handler = AsyncHandlerPayloadOnly::new(move |_: serde_json::Value| async move {
        Ok::<(), std::io::Error>(())
    });
    let handle = tokio::spawn({
        let config = config.clone();
        async move {
            let _ = consumer.run(config, noop_handler).await;
        }
    });
    // Give the consumer time to declare the queue and bind it.
    tokio::time::sleep(Duration::from_millis(500)).await;
    handle.abort();

    // Publish 5 messages (no consumer running → they pile up in the queue).
    let publisher = RmqPublisher::connect(url, exchange).await;
    for i in 0..5u64 {
        publisher
            .publish(routing_key, &BlockEvent { block_number: i })
            .await
            .unwrap();
    }

    // Query depth via our inspector.
    let conn = ConnectionManager::new(url);
    let inspector = AmqpQueueInspector::new(conn);
    let depths = inspector.queue_depths(queue, None).await.unwrap();

    assert_eq!(depths.principal, 5, "5 messages published, none consumed");
    assert_eq!(depths.retry, Some(0), "no messages in retry queue");
    assert_eq!(depths.dead_letter, 0, "no messages in dead-letter queue");
    assert_eq!(depths.total(), 5);
    assert!(!depths.is_empty());
    assert_eq!(depths.pending, None, "AMQP does not track pending");
    assert_eq!(depths.lag, None, "AMQP does not track lag");
}

/// Verify `AmqpQueueInspector::queue_depths` returns zeros for a queue that
/// does not exist (passive declare returns NOT_FOUND → treated as 0).
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_queue_depth_nonexistent_queue_returns_zeros() {
    use broker::amqp::AmqpQueueInspector;
    use broker::traits::depth::QueueInspector;

    let url = shared_rabbitmq_url().await;
    let conn = ConnectionManager::new(url);
    let inspector = AmqpQueueInspector::new(conn);

    let depths = inspector
        .queue_depths("nonexistent.queue.depth.test", None)
        .await
        .unwrap();

    assert_eq!(depths.principal, 0);
    assert_eq!(depths.retry, Some(0));
    assert_eq!(depths.dead_letter, 0);
    assert!(depths.is_empty());
}

/// Verify `is_empty` returns `true` for a non-existent queue.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_is_empty_nonexistent_queue() {
    use broker::amqp::AmqpQueueInspector;
    use broker::traits::depth::QueueInspector;

    let url = shared_rabbitmq_url().await;
    let conn = ConnectionManager::new(url);
    let inspector = AmqpQueueInspector::new(conn);

    let empty = inspector
        .is_empty("nonexistent.is_empty.test", "ignored")
        .await
        .unwrap();

    assert!(empty, "non-existent queue should be empty");
}

/// Verify `exists` returns `false` for a queue that does not exist.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_exists_nonexistent_queue() {
    use broker::amqp::AmqpQueueInspector;
    use broker::traits::depth::QueueInspector;

    let url = shared_rabbitmq_url().await;
    let conn = ConnectionManager::new(url);
    let inspector = AmqpQueueInspector::new(conn);

    let found = inspector.exists("nonexistent.exists.test").await.unwrap();

    assert!(!found, "non-existent queue should return false");
}

/// Verify `exists` returns `true` for a queue that has been declared.
#[tokio::test]
#[ignore = "requires Docker"]
async fn test_exists_after_declare() {
    use broker::amqp::AmqpQueueInspector;
    use broker::traits::depth::QueueInspector;
    use lapin::options::QueueDeclareOptions;
    use lapin::types::FieldTable;

    let url = shared_rabbitmq_url().await;
    let conn = ConnectionManager::new(url);

    // Declare a durable queue so it exists.
    let channel = conn.create_channel().await.unwrap();
    channel
        .queue_declare(
            "exists.after_declare.test".into(),
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .unwrap();

    let inspector = AmqpQueueInspector::new(conn);
    let found = inspector.exists("exists.after_declare.test").await.unwrap();

    assert!(found, "declared queue should exist");
}
