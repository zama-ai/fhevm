//! Broker metrics registration and helpers.
//!
//! This module provides:
//! - [`describe_metrics()`]: registers Prometheus HELP strings for all broker metrics
//! - [`record_queue_depths()`]: converts a [`QueueDepths`] into gauge observations
//! - [`spawn_queue_depth_poller()`]: background task that polls queue depths periodically

use std::time::Duration;

use tokio::task::JoinHandle;
use tokio::time::{MissedTickBehavior, interval};
use tokio_util::sync::CancellationToken;
use tracing::debug;

use crate::traits::depth::QueueDepths;
use crate::{Broker, Topic};

/// Register metric descriptions with the global recorder.
///
/// Call once at application startup, after installing the metrics exporter.
/// Safe to call multiple times (describe is idempotent).
pub fn describe_metrics() {
    use metrics::{Unit, describe_counter, describe_gauge, describe_histogram};

    // ── Publishing ────────────────────────────────────────────────────────
    describe_counter!(
        "broker_messages_published_total",
        Unit::Count,
        "Total messages successfully published"
    );
    describe_counter!(
        "broker_publish_errors_total",
        Unit::Count,
        "Publish failures per attempt"
    );
    describe_histogram!(
        "broker_publish_duration_seconds",
        Unit::Seconds,
        "End-to-end publish latency including retries"
    );

    // ── Consuming ─────────────────────────────────────────────────────────
    describe_counter!(
        "broker_messages_consumed_total",
        Unit::Count,
        "Messages processed by handler, partitioned by outcome"
    );
    describe_histogram!(
        "broker_handler_duration_seconds",
        Unit::Seconds,
        "Handler execution wall-clock time per message"
    );
    describe_counter!(
        "broker_messages_dead_lettered_total",
        Unit::Count,
        "Messages routed to dead-letter queue"
    );
    describe_histogram!(
        "broker_message_delivery_count",
        Unit::Count,
        "Distribution of delivery counts at processing time"
    );

    // ── Circuit breaker ───────────────────────────────────────────────────
    describe_gauge!(
        "broker_circuit_breaker_state",
        Unit::Count,
        "Circuit breaker state: 0=closed, 1=open, 2=half_open"
    );
    describe_counter!(
        "broker_circuit_breaker_trips_total",
        Unit::Count,
        "Times the circuit breaker tripped to open"
    );
    describe_gauge!(
        "broker_circuit_breaker_consecutive_failures",
        Unit::Count,
        "Current consecutive transient failure count"
    );

    // ── Queue depth ───────────────────────────────────────────────────────
    describe_gauge!(
        "broker_queue_depth_principal",
        Unit::Count,
        "Messages in the principal queue/stream"
    );
    describe_gauge!(
        "broker_queue_depth_retry",
        Unit::Count,
        "Messages in the retry queue (AMQP only)"
    );
    describe_gauge!(
        "broker_queue_depth_dead_letter",
        Unit::Count,
        "Messages in the dead-letter queue/stream"
    );
    describe_gauge!(
        "broker_queue_depth_pending",
        Unit::Count,
        "Pending entry list count (Redis only)"
    );
    describe_gauge!(
        "broker_queue_depth_lag",
        Unit::Count,
        "Consumer group lag (Redis 7.0+ only)"
    );

    // ── Connection health ─────────────────────────────────────────────────
    describe_counter!(
        "broker_consumer_reconnections_total",
        Unit::Count,
        "Consumer reconnection cycles"
    );
    describe_gauge!(
        "broker_consumer_connected",
        Unit::Count,
        "Consumer connectivity: 1=connected, 0=reconnecting"
    );
    describe_counter!(
        "broker_claim_sweeper_messages_claimed_total",
        Unit::Count,
        "Messages reclaimed by ClaimSweeper"
    );
    describe_counter!(
        "broker_claim_sweeper_messages_dead_lettered_total",
        Unit::Count,
        "Messages moved to DLQ by ClaimSweeper"
    );
}

/// Record [`QueueDepths`] as Prometheus gauges.
///
/// Intended to be called periodically (e.g., every 15s) by the application
/// to keep queue depth gauges fresh for Prometheus scraping.
///
/// `None` fields (e.g., `retry` for Redis, `pending`/`lag` for AMQP) are
/// silently skipped — no gauge is set.
pub fn record_queue_depths(depths: &QueueDepths, backend: &str, topic: &str) {
    metrics::gauge!("broker_queue_depth_principal", "backend" => backend.to_owned(), "topic" => topic.to_owned())
        .set(depths.principal as f64);
    if let Some(retry) = depths.retry {
        metrics::gauge!("broker_queue_depth_retry", "backend" => backend.to_owned(), "topic" => topic.to_owned())
            .set(retry as f64);
    }
    metrics::gauge!("broker_queue_depth_dead_letter", "backend" => backend.to_owned(), "topic" => topic.to_owned())
        .set(depths.dead_letter as f64);
    if let Some(pending) = depths.pending {
        metrics::gauge!("broker_queue_depth_pending", "backend" => backend.to_owned(), "topic" => topic.to_owned())
            .set(pending as f64);
    }
    if let Some(lag) = depths.lag {
        metrics::gauge!("broker_queue_depth_lag", "backend" => backend.to_owned(), "topic" => topic.to_owned())
            .set(lag as f64);
    }
}

/// Map a [`HandlerOutcome`] to a static label string for the `outcome` label.
pub(crate) fn outcome_label(outcome: &crate::traits::handler::HandlerOutcome) -> &'static str {
    use crate::traits::handler::HandlerOutcome;
    match outcome {
        HandlerOutcome::Ack => "ack",
        HandlerOutcome::Nack => "nack",
        HandlerOutcome::Dead => "dead",
        HandlerOutcome::Delay(_) => "delay",
        HandlerOutcome::Transient => "transient",
        HandlerOutcome::Permanent => "permanent",
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Queue depth periodic poller
// ═══════════════════════════════════════════════════════════════════════════

/// A (topic, consumer-group) pair to poll for queue depth metrics.
///
/// The `topic` label emitted to Prometheus is derived from [`Topic::key`]
/// (e.g. `"chain-id-1.fetch-new-blocks"`).
#[derive(Debug, Clone)]
pub struct QueueDepthPollTarget {
    pub topic: Topic,
    pub group: String,
}

impl QueueDepthPollTarget {
    pub fn new(topic: Topic, group: impl Into<String>) -> Self {
        Self {
            topic,
            group: group.into(),
        }
    }
}

/// Spawn a background task that polls queue depths at `poll_interval` and
/// records them as gauges via [`record_queue_depths`].
///
/// The `backend` label is read from [`Broker::backend_name`]. The `topic`
/// label is [`Topic::key`].
///
/// Errors (e.g. a queue/stream that doesn't exist yet) are logged at DEBUG and
/// swallowed — the next tick retries. The task stops when `cancel` fires.
///
/// Call this once at startup, after the broker and consumer topologies are
/// ready. Dropping the returned `JoinHandle` does **not** stop the task —
/// use `cancel.cancel()` for that.
///
/// # Example
///
/// ```ignore
/// use broker::metrics::{spawn_queue_depth_poller, QueueDepthPollTarget};
/// use broker::Topic;
/// use std::time::Duration;
/// use tokio_util::sync::CancellationToken;
///
/// let cancel = CancellationToken::new();
/// let targets = vec![
///     QueueDepthPollTarget::new(
///         Topic::new("fetch-new-blocks").with_namespace("chain-id-1"),
///         "fetch-new-blocks",
///     ),
/// ];
/// spawn_queue_depth_poller(broker.clone(), targets, Duration::from_secs(15), cancel);
/// ```
pub fn spawn_queue_depth_poller(
    broker: Broker,
    targets: Vec<QueueDepthPollTarget>,
    poll_interval: Duration,
    cancel: CancellationToken,
) -> JoinHandle<()> {
    let backend = broker.backend_name();
    tokio::spawn(async move {
        let mut tick = interval(poll_interval);
        tick.set_missed_tick_behavior(MissedTickBehavior::Delay);
        loop {
            tokio::select! {
                _ = cancel.cancelled() => return,
                _ = tick.tick() => {
                    for target in &targets {
                        match broker.queue_depths(&target.topic, Some(&target.group)).await {
                            Ok(depths) => {
                                let label = target.topic.key();
                                record_queue_depths(&depths, backend, &label);
                            }
                            Err(e) => {
                                debug!(
                                    topic = %target.topic,
                                    error = %e,
                                    "queue_depths poll failed (topology may not exist yet)"
                                );
                            }
                        }
                    }
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // AC-2.1
    #[test]
    fn describe_metrics_does_not_panic() {
        describe_metrics();
    }

    // AC-2.2 (partial — gauges are set, but asserting requires a test recorder)
    #[test]
    fn record_queue_depths_with_all_fields() {
        let depths = QueueDepths {
            principal: 42,
            retry: Some(5),
            dead_letter: 3,
            pending: Some(10),
            lag: Some(7),
        };
        // No recorder installed → calls are no-ops, must not panic
        record_queue_depths(&depths, "redis", "test.stream");
    }

    #[test]
    fn record_queue_depths_skips_none_fields() {
        let depths = QueueDepths {
            principal: 10,
            retry: None,
            dead_letter: 0,
            pending: None,
            lag: None,
        };
        record_queue_depths(&depths, "redis", "test.stream");
    }
}
