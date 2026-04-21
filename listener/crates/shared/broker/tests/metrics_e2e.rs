#![cfg(feature = "redis")]
//! End-to-end metrics acceptance tests for the `broker` crate.
//!
//! These tests verify that publishing and consuming messages causes the
//! expected `metrics` counters, gauges, and histograms to be recorded.
//!
//! Uses `metrics_util::debugging::DebuggingRecorder` to capture metrics
//! in-process without a Prometheus exporter. Because the global recorder
//! can only be set once per process, all tests in this file share a single
//! recorder installed in a `std::sync::OnceLock`.
//!
//! Run via:
//!
//! ```bash
//! make test-e2e-redis  # or:
//! TMPDIR=/tmp/claude cargo test -p broker --features redis --test metrics_e2e -- --ignored --test-threads=1
//! ```

use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicU32, Ordering},
};
use std::time::Duration;

use broker::{
    AckDecision, AsyncHandlerPayloadClassified, AsyncHandlerPayloadOnly, Broker, CancellationToken,
    Handler, HandlerError, Message, Topic,
};
use metrics_util::debugging::{DebugValue, DebuggingRecorder, Snapshotter};
use metrics_util::{CompositeKey, MetricKind};
use ordered_float::OrderedFloat;
use test_support::shared_redis_url;

// ── Global test recorder ────────────────────────────────────────────────────

static SNAPSHOTTER: OnceLock<Snapshotter> = OnceLock::new();

fn snapshotter() -> &'static Snapshotter {
    SNAPSHOTTER.get_or_init(|| {
        let recorder = DebuggingRecorder::new();
        let snapshotter = recorder.snapshotter();
        // This will fail if another test binary already set the global recorder,
        // but within this test binary it runs exactly once.
        let _ = recorder.install();
        snapshotter
    })
}

// ── Assertion helpers ───────────────────────────────────────────────────────

/// Find a metric by name + kind + labels in the snapshot.
fn find_metric(
    snap: &[(
        CompositeKey,
        Option<metrics::Unit>,
        Option<metrics::SharedString>,
        DebugValue,
    )],
    kind: MetricKind,
    name: &str,
    labels: &[(&str, &str)],
) -> Option<DebugValue> {
    snap.iter()
        .find(|(ck, _, _, _)| {
            ck.kind() == kind
                && ck.key().name() == name
                && labels
                    .iter()
                    .all(|(k, v)| ck.key().labels().any(|l| l.key() == *k && l.value() == *v))
        })
        .map(|(_, _, _, v)| match v {
            DebugValue::Counter(c) => DebugValue::Counter(*c),
            DebugValue::Gauge(g) => DebugValue::Gauge(*g),
            DebugValue::Histogram(h) => DebugValue::Histogram(h.clone()),
        })
}

fn assert_counter_gte(
    snap: &[(
        CompositeKey,
        Option<metrics::Unit>,
        Option<metrics::SharedString>,
        DebugValue,
    )],
    name: &str,
    min: u64,
    labels: &[(&str, &str)],
) {
    let value = find_metric(snap, MetricKind::Counter, name, labels);
    match value {
        Some(DebugValue::Counter(v)) => assert!(
            v >= min,
            "{name} = {v}, expected >= {min} (labels: {labels:?})"
        ),
        other => panic!("{name}: expected Counter >= {min}, got {other:?} (labels: {labels:?})"),
    }
}

#[allow(dead_code)]
fn assert_counter_eq(
    snap: &[(
        CompositeKey,
        Option<metrics::Unit>,
        Option<metrics::SharedString>,
        DebugValue,
    )],
    name: &str,
    expected: u64,
    labels: &[(&str, &str)],
) {
    let value = find_metric(snap, MetricKind::Counter, name, labels);
    match value {
        Some(DebugValue::Counter(v)) => assert_eq!(
            v, expected,
            "{name} = {v}, expected {expected} (labels: {labels:?})"
        ),
        other => panic!("{name}: expected Counter({expected}), got {other:?} (labels: {labels:?})"),
    }
}

fn assert_histogram_has_observations(
    snap: &[(
        CompositeKey,
        Option<metrics::Unit>,
        Option<metrics::SharedString>,
        DebugValue,
    )],
    name: &str,
    min_count: usize,
    labels: &[(&str, &str)],
) {
    let value = find_metric(snap, MetricKind::Histogram, name, labels);
    match value {
        Some(DebugValue::Histogram(values)) => assert!(
            values.len() >= min_count,
            "{name}: expected >= {min_count} observations, got {} (labels: {labels:?})",
            values.len()
        ),
        other => panic!(
            "{name}: expected Histogram with >= {min_count} obs, got {other:?} (labels: {labels:?})"
        ),
    }
}

fn assert_gauge_eq(
    snap: &[(
        CompositeKey,
        Option<metrics::Unit>,
        Option<metrics::SharedString>,
        DebugValue,
    )],
    name: &str,
    expected: f64,
    labels: &[(&str, &str)],
) {
    let value = find_metric(snap, MetricKind::Gauge, name, labels);
    match value {
        Some(DebugValue::Gauge(v)) => assert_eq!(
            v,
            OrderedFloat(expected),
            "{name} = {v}, expected {expected} (labels: {labels:?})"
        ),
        other => panic!("{name}: expected Gauge({expected}), got {other:?} (labels: {labels:?})"),
    }
}

// ── Shared payload ──────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct TestEvent {
    value: u64,
}

/// Wait for an atomic counter to reach `target` within `timeout`.
async fn wait_for(counter: &AtomicU32, target: u32, timeout: Duration) {
    let deadline = tokio::time::Instant::now() + timeout;
    while counter.load(Ordering::SeqCst) < target && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// AC-2.3: Publish N messages -> broker_messages_published_total == N
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore = "requires Docker"]
async fn ac_2_3_redis_publish_increments_published_total() {
    let snap = snapshotter();
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();
    let publisher = broker.publisher("m-pub").await.unwrap();

    publisher
        .publish("test-pub", &TestEvent { value: 1 })
        .await
        .unwrap();
    publisher
        .publish("test-pub", &TestEvent { value: 2 })
        .await
        .unwrap();

    let snapshot = snap.snapshot().into_vec();
    assert_counter_gte(
        &snapshot,
        "broker_messages_published_total",
        2,
        &[("backend", "redis"), ("topic", "m-pub.test-pub")],
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// AC-2.5: Publish -> broker_publish_duration_seconds has observations
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore = "requires Docker"]
async fn ac_2_5_redis_publish_records_duration_histogram() {
    let snap = snapshotter();
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();
    let publisher = broker.publisher("m-dur").await.unwrap();

    publisher
        .publish("test-dur", &TestEvent { value: 1 })
        .await
        .unwrap();

    let snapshot = snap.snapshot().into_vec();
    assert_histogram_has_observations(
        &snapshot,
        "broker_publish_duration_seconds",
        1,
        &[("backend", "redis"), ("topic", "m-dur.test-dur")],
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// AC-2.6: Consume ack -> broker_messages_consumed_total{outcome=ack}
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore = "requires Docker"]
async fn ac_2_6_redis_consume_ack_increments_outcome_counter() {
    let snap = snapshotter();
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();
    let topic = Topic::namespaced("m-ack", "events");

    let publisher = broker.publisher("m-ack").await.unwrap();
    publisher
        .publish("events", &TestEvent { value: 42 })
        .await
        .unwrap();

    let processed = Arc::new(AtomicU32::new(0));
    let processed_clone = processed.clone();
    let handler = AsyncHandlerPayloadOnly::new(move |_: TestEvent| {
        let c = processed_clone.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            Ok::<(), std::io::Error>(())
        }
    });

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("m-ack-group")
            .prefetch(1)
            .with_cancellation(cancel_clone)
            .run(handler)
            .await;
    });

    wait_for(&processed, 1, Duration::from_secs(8)).await;
    cancel.cancel();
    let _ = consumer_handle.await;

    assert!(
        processed.load(Ordering::SeqCst) >= 1,
        "handler should have processed at least 1 message"
    );

    let snapshot = snap.snapshot().into_vec();
    assert_counter_gte(
        &snapshot,
        "broker_messages_consumed_total",
        1,
        &[
            ("backend", "redis"),
            ("topic", "m-ack.events"),
            ("outcome", "ack"),
        ],
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// AC-2.7: Consume -> broker_handler_duration_seconds has observations
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore = "requires Docker"]
async fn ac_2_7_redis_consume_records_handler_duration() {
    let snap = snapshotter();
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();
    let topic = Topic::namespaced("m-hdur", "events");

    let publisher = broker.publisher("m-hdur").await.unwrap();
    publisher
        .publish("events", &TestEvent { value: 99 })
        .await
        .unwrap();

    let processed = Arc::new(AtomicU32::new(0));
    let processed_clone = processed.clone();
    let handler = AsyncHandlerPayloadOnly::new(move |_: TestEvent| {
        let c = processed_clone.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            Ok::<(), std::io::Error>(())
        }
    });

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("m-hdur-group")
            .prefetch(1)
            .with_cancellation(cancel_clone)
            .run(handler)
            .await;
    });

    wait_for(&processed, 1, Duration::from_secs(8)).await;
    // The histogram is recorded in the worker task after handler.call() returns,
    // but processed is incremented inside handler.call(). Give the worker task
    // time to record the histogram and send the result through the mpsc channel.
    tokio::time::sleep(Duration::from_millis(300)).await;
    // Take snapshot BEFORE cancelling — histogram values are cleared on snapshot
    let snapshot = snap.snapshot().into_vec();
    cancel.cancel();
    let _ = consumer_handle.await;

    assert_histogram_has_observations(
        &snapshot,
        "broker_handler_duration_seconds",
        1,
        &[("backend", "redis"), ("topic", "m-hdur.events")],
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// AC-2.8: Permanent error -> consumed_total{outcome=permanent}
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore = "requires Docker"]
async fn ac_2_8_redis_permanent_error_records_permanent_outcome() {
    let snap = snapshotter();
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();
    let topic = Topic::namespaced("m-perm", "events");

    let publisher = broker.publisher("m-perm").await.unwrap();
    publisher
        .publish("events", &TestEvent { value: 1 })
        .await
        .unwrap();

    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = call_count.clone();
    let handler = AsyncHandlerPayloadClassified::new(move |_: TestEvent| {
        let c = call_count_clone.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            Err(HandlerError::permanent(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "bad payload",
            )))
        }
    });

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("m-perm-group")
            .prefetch(1)
            .max_retries(2)
            .with_cancellation(cancel_clone)
            .run(handler)
            .await;
    });

    wait_for(&call_count, 1, Duration::from_secs(8)).await;
    cancel.cancel();
    let _ = consumer_handle.await;

    let snapshot = snap.snapshot().into_vec();
    assert_counter_gte(
        &snapshot,
        "broker_messages_consumed_total",
        1,
        &[
            ("backend", "redis"),
            ("topic", "m-perm.events"),
            ("outcome", "permanent"),
        ],
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// AC-2.9: Transient error -> consumed_total{outcome=transient}
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore = "requires Docker"]
async fn ac_2_9_redis_transient_error_records_transient_outcome() {
    let snap = snapshotter();
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();
    let topic = Topic::namespaced("m-trans", "events");

    let publisher = broker.publisher("m-trans").await.unwrap();
    publisher
        .publish("events", &TestEvent { value: 1 })
        .await
        .unwrap();

    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = call_count.clone();
    let handler = AsyncHandlerPayloadClassified::new(move |_: TestEvent| {
        let c = call_count_clone.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            Err(HandlerError::transient(std::io::Error::new(
                std::io::ErrorKind::ConnectionReset,
                "db down",
            )))
        }
    });

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("m-trans-group")
            .prefetch(1)
            .with_cancellation(cancel_clone)
            .run(handler)
            .await;
    });

    wait_for(&call_count, 1, Duration::from_secs(8)).await;
    // Let it process briefly so the outcome is recorded
    tokio::time::sleep(Duration::from_millis(200)).await;
    cancel.cancel();
    let _ = consumer_handle.await;

    let snapshot = snap.snapshot().into_vec();
    assert_counter_gte(
        &snapshot,
        "broker_messages_consumed_total",
        1,
        &[
            ("backend", "redis"),
            ("topic", "m-trans.events"),
            ("outcome", "transient"),
        ],
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// AC-2.11: AckDecision::Dead -> dead_lettered{reason=handler_requested}
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore = "requires Docker"]
async fn ac_2_11_redis_dead_decision_increments_dead_lettered() {
    let snap = snapshotter();
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();
    let topic = Topic::namespaced("m-dead", "events");

    let publisher = broker.publisher("m-dead").await.unwrap();
    publisher
        .publish("events", &TestEvent { value: 1 })
        .await
        .unwrap();

    /// Handler that always returns `AckDecision::Dead`.
    #[derive(Clone)]
    struct DeadHandler {
        call_count: Arc<AtomicU32>,
    }
    #[async_trait::async_trait]
    impl Handler for DeadHandler {
        async fn call(&self, _msg: &Message) -> Result<AckDecision, HandlerError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            Ok(AckDecision::Dead)
        }
    }

    let call_count = Arc::new(AtomicU32::new(0));
    let handler = DeadHandler {
        call_count: call_count.clone(),
    };

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("m-dead-group")
            .prefetch(1)
            .with_cancellation(cancel_clone)
            .run(handler)
            .await;
    });

    wait_for(&call_count, 1, Duration::from_secs(8)).await;
    tokio::time::sleep(Duration::from_millis(200)).await;
    cancel.cancel();
    let _ = consumer_handle.await;

    let snapshot = snap.snapshot().into_vec();
    assert_counter_gte(
        &snapshot,
        "broker_messages_dead_lettered_total",
        1,
        &[
            ("backend", "redis"),
            ("topic", "m-dead.events"),
            ("reason", "handler_requested"),
        ],
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// AC-2.13: First delivery -> delivery_count histogram observation
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore = "requires Docker"]
async fn ac_2_13_redis_delivery_count_histogram_recorded() {
    let snap = snapshotter();
    let url = shared_redis_url().await;
    let broker = Broker::redis(url).await.unwrap();
    let topic = Topic::namespaced("m-delcnt", "events");

    let publisher = broker.publisher("m-delcnt").await.unwrap();
    publisher
        .publish("events", &TestEvent { value: 1 })
        .await
        .unwrap();

    let processed = Arc::new(AtomicU32::new(0));
    let processed_clone = processed.clone();
    let handler = AsyncHandlerPayloadOnly::new(move |_: TestEvent| {
        let c = processed_clone.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            Ok::<(), std::io::Error>(())
        }
    });

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    let consumer_broker = broker.clone();
    let consumer_handle = tokio::spawn(async move {
        let _ = consumer_broker
            .consumer(&topic)
            .group("m-delcnt-group")
            .prefetch(1)
            .with_cancellation(cancel_clone)
            .run(handler)
            .await;
    });

    wait_for(&processed, 1, Duration::from_secs(8)).await;
    let snapshot = snap.snapshot().into_vec();
    cancel.cancel();
    let _ = consumer_handle.await;

    assert_histogram_has_observations(
        &snapshot,
        "broker_message_delivery_count",
        1,
        &[("backend", "redis"), ("topic", "m-delcnt.events")],
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// AC-2.2: record_queue_depths sets correct gauges
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore = "requires Docker"]
async fn ac_2_2_record_queue_depths_sets_gauges() {
    let snap = snapshotter();

    let depths = broker::QueueDepths {
        principal: 42,
        retry: Some(5),
        dead_letter: 3,
        pending: Some(10),
        lag: Some(7),
    };
    broker::metrics::record_queue_depths(&depths, "redis", "m-depth.test");

    let snapshot = snap.snapshot().into_vec();
    assert_gauge_eq(
        &snapshot,
        "broker_queue_depth_principal",
        42.0,
        &[("backend", "redis"), ("topic", "m-depth.test")],
    );
    assert_gauge_eq(
        &snapshot,
        "broker_queue_depth_retry",
        5.0,
        &[("backend", "redis"), ("topic", "m-depth.test")],
    );
    assert_gauge_eq(
        &snapshot,
        "broker_queue_depth_dead_letter",
        3.0,
        &[("backend", "redis"), ("topic", "m-depth.test")],
    );
    assert_gauge_eq(
        &snapshot,
        "broker_queue_depth_pending",
        10.0,
        &[("backend", "redis"), ("topic", "m-depth.test")],
    );
    assert_gauge_eq(
        &snapshot,
        "broker_queue_depth_lag",
        7.0,
        &[("backend", "redis"), ("topic", "m-depth.test")],
    );
}
