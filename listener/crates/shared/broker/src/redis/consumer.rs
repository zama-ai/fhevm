use redis::Value;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;
use tokio::sync::{Mutex, mpsc};
use tokio::time::{MissedTickBehavior, interval, sleep};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

/// Backoff duration after a connection error before retrying.
/// Gives `ConnectionManager` time to auto-reconnect internally.
const CONN_RETRY_BACKOFF: Duration = Duration::from_secs(5);
const CLASS_TRANSIENT: &str = "transient";
const CLASS_PERMANENT: &str = "permanent";
/// Maximum retries for classification persistence before propagating the error.
const MAX_CLASSIFICATION_RETRIES: u32 = 10;

use crate::traits::handler::{AckDecision, Handler, HandlerOutcome};
use crate::traits::message::{Message, MessageMetadata};

use super::{
    circuit_breaker::CircuitBreaker, claim_task::ClaimSweeper, config::RedisPrefetchConfig,
    connection::RedisConnectionManager, error::RedisConsumerError,
};

/// Result of processing a message in a worker task.
/// Used by `run()` to communicate results back to the main loop.
struct ProcessingResult {
    /// The stream entry ID, needed for XACK
    stream_id: String,
    /// Outcome classification — preserves the Transient/Permanent distinction
    /// so the main loop can call the correct circuit breaker method.
    outcome: HandlerOutcome,
    /// Original message payload — only populated when `outcome == Dead` so the
    /// main loop can write the full message content to the dead stream.
    /// `None` for all other outcomes to avoid unnecessary cloning.
    payload: Option<Vec<u8>>,
    /// Delivery count from the message metadata — needed for dead-letter routing
    /// so the DLQ entry records how many times the message was delivered.
    delivery_count: u64,
}

/// Cancels a token when dropped.
///
/// Used to guarantee background task shutdown even when a function returns
/// early due to an error.
struct CancelOnDrop {
    token: CancellationToken,
}

impl CancelOnDrop {
    fn new(token: CancellationToken) -> Self {
        Self { token }
    }
}

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.token.cancel();
    }
}

/// Sends a stream ID to the abnormal-exit channel when dropped unless disarmed.
///
/// Used by prefetch-safe worker tasks to guarantee in-flight cleanup even when
/// a worker panics before sending `ProcessingResult` to the main loop.
struct WorkerAbnormalExitGuard {
    stream_id: Option<String>,
    tx: mpsc::UnboundedSender<String>,
}

impl WorkerAbnormalExitGuard {
    fn new(stream_id: String, tx: mpsc::UnboundedSender<String>) -> Self {
        Self {
            stream_id: Some(stream_id),
            tx,
        }
    }

    /// Disable drop-side notification after successful result handoff.
    fn disarm(&mut self) {
        self.stream_id = None;
    }
}

impl Drop for WorkerAbnormalExitGuard {
    fn drop(&mut self) {
        if let Some(stream_id) = self.stream_id.take() {
            let _ = self.tx.send(stream_id);
        }
    }
}

/// Redis Streams consumer service.
///
/// Provides three consumption strategies mirroring the RMQ consumer:
/// - `run_simple` — basic XREADGROUP with XACK on every message
/// - `run_with_retry` — XREADGROUP + ClaimSweeper for idle message recovery
/// - `run` — bounded mpsc + worker pool + ACK in main loop
pub struct RedisConsumer {
    connection: Arc<RedisConnectionManager>,
    cancel_token: CancellationToken,
}

impl RedisConsumer {
    /// Create a new consumer with the given connection manager.
    pub fn new(connection: Arc<RedisConnectionManager>) -> Self {
        Self {
            connection,
            cancel_token: CancellationToken::new(),
        }
    }

    /// Set a cancellation token for graceful shutdown.
    ///
    /// When cancelled, the consumer finishes its current batch, drains
    /// in-flight results, and exits cleanly.
    pub fn with_cancellation(mut self, token: CancellationToken) -> Self {
        self.cancel_token = token;
        self
    }

    /// Create a new consumer from a Redis URL (one-liner convenience).
    ///
    /// Mirrors `RmqConsumer::with_addr()` — handles connection setup internally
    /// so the developer doesn't need to manually create a `RedisConnectionManager`
    /// and wrap it in `Arc`.
    pub async fn with_url(url: &str) -> Result<Self, RedisConsumerError> {
        let conn = RedisConnectionManager::new_with_retry(url).await;
        Ok(Self {
            connection: Arc::new(conn),
            cancel_token: CancellationToken::new(),
        })
    }

    /// Get the underlying connection manager.
    pub fn connection(&self) -> &Arc<RedisConnectionManager> {
        &self.connection
    }

    // ─────────────────────────────────────────────────────────────
    // Strategy 1: Simple consumer — no retry, XACK on every message
    // ─────────────────────────────────────────────────────────────

    // ─────────────────────────────────────────────────────────────
    // Strategy 3: Prefetch safe — bounded mpsc, worker pool, ACK in main loop
    // ─────────────────────────────────────────────────────────────

    /// Run a high-throughput consumer with STRONG message loss guarantees.
    ///
    /// Same architecture as RMQ's `run`:
    /// - Bounded `mpsc::channel` matching `prefetch_count`
    /// - Workers receive `Message`, call handler, send `ProcessingResult` back
    /// - Main loop: XACK on success, leave in PEL on failure
    /// - ClaimSweeper runs in parallel
    /// - Startup drain of own pending via `XREADGROUP ... 0`
    pub async fn run(
        &self,
        config: RedisPrefetchConfig,
        handler: impl Handler + 'static,
    ) -> Result<(), RedisConsumerError> {
        let handler: Arc<dyn Handler> = Arc::new(handler);
        // Ensure consumer group exists
        self.ensure_group(&config.retry.base.stream, &config.retry.base.group_name)
            .await?;
        let classification_marker_key = config.retry.classification_marker_key();

        // Spawn ClaimSweeper
        let cancel = CancellationToken::new();
        let classification_paused = Arc::new(AtomicBool::new(false));
        let sweeper = ClaimSweeper::new_with_classification_pause(
            (*self.connection).clone(),
            config.retry.clone(),
            Arc::clone(&classification_paused),
        );
        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            sweeper.run(cancel_clone).await;
        });
        let _sweeper_cancel_guard = CancelOnDrop::new(cancel.clone());

        info!(
            stream = %config.retry.base.stream,
            group = %config.retry.base.group_name,
            consumer = %config.retry.base.consumer_name,
            prefetch_count = config.prefetch_count,
            "Prefetch safe Redis consumer started (ACK in main loop)"
        );

        // Signal "connected" — flipped to 0 on connection errors below.
        metrics::gauge!(
            "broker_consumer_connected",
            "backend" => "redis",
            "topic" => config.retry.base.stream.clone(),
        )
        .set(1.0);

        // Bounded channel — size matches prefetch_count for backpressure
        let (result_tx, mut result_rx) = mpsc::channel::<ProcessingResult>(config.prefetch_count);
        // Worker abnormal-exit side channel (panic/send-failure before result handoff).
        let (abnormal_exit_tx, mut abnormal_exit_rx) = mpsc::unbounded_channel::<String>();
        // Guards against duplicate local dispatch of the same stream entry while
        // the first worker is still in-flight (new-read path racing pending-drain path).
        let in_flight = Arc::new(Mutex::new(HashSet::<String>::new()));

        // Phase 1: drain own pending messages (single-threaded for ordering)
        info!("Phase 1: draining own pending messages");
        loop {
            if self.cancel_token.is_cancelled() {
                info!("Cancellation requested during Phase 1, stopping");
                break;
            }
            let entries = match self
                .xreadgroup(
                    &config.retry.base.stream,
                    &config.retry.base.group_name,
                    &config.retry.base.consumer_name,
                    config.prefetch_count,
                    "0",
                )
                .await
            {
                Ok(entries) => entries,
                Err(e) if e.is_connection_error() => {
                    warn!(
                        error = %e,
                        stream = %config.retry.base.stream,
                        "Connection error during Phase 1 pending drain, forcing reconnect..."
                    );
                    self.connection.force_reconnect().await;
                    sleep(CONN_RETRY_BACKOFF).await;
                    continue;
                }
                Err(e) => return Err(e),
            };

            if entries.is_empty() {
                info!("Phase 1 complete: no more pending messages");
                break;
            }

            let delivery_counts = self
                .get_pending_delivery_counts(
                    &config.retry.base.stream,
                    &config.retry.base.group_name,
                    &entries,
                )
                .await;

            for (stream_id, data) in entries {
                let count = delivery_counts.get(&stream_id).copied().unwrap_or(2);
                let msg = build_message(
                    stream_id.clone(),
                    config.retry.base.stream.clone(),
                    count,
                    data,
                );

                let outcome = HandlerOutcome::from(handler.call(&msg).await);
                match outcome {
                    HandlerOutcome::Ack => {
                        if let Err(e) = self
                            .xack(
                                &config.retry.base.stream,
                                &config.retry.base.group_name,
                                &stream_id,
                            )
                            .await
                        {
                            if e.is_connection_error() {
                                warn!(error = %e, stream_id = %stream_id, "Connection error during XACK, message stays in PEL");
                            } else {
                                return Err(e);
                            }
                        } else {
                            self.clear_failure_marker_safe(
                                &classification_marker_key,
                                &stream_id,
                                &classification_paused,
                            )
                            .await?;
                        }
                    }
                    HandlerOutcome::Nack | HandlerOutcome::Delay(_) => {
                        // Voluntary yield — leave in PEL so ClaimSweeper retries.
                        debug!(stream_id = %stream_id, "Pending message voluntarily yielded, leaving in PEL");
                        self.clear_failure_marker_safe(
                            &classification_marker_key,
                            &stream_id,
                            &classification_paused,
                        )
                        .await?;
                    }
                    HandlerOutcome::Dead => {
                        warn!(stream_id = %stream_id, "Pending message requested dead-letter, routing immediately");
                        self.xadd_dead_letter(
                            &config.retry,
                            &stream_id,
                            &msg.payload,
                            msg.metadata.delivery_count,
                            "AckDecision::Dead",
                        )
                        .await?;
                        self.clear_failure_marker_safe(
                            &classification_marker_key,
                            &stream_id,
                            &classification_paused,
                        )
                        .await?;
                    }
                    HandlerOutcome::Transient => {
                        warn!(stream_id = %stream_id, "Pending message transient failure, preserving infinite retry classification");
                        self.mark_transient_safe(
                            &classification_marker_key,
                            &stream_id,
                            &classification_paused,
                        )
                        .await?;
                    }
                    HandlerOutcome::Permanent => {
                        if count >= config.retry.max_retries as u64 {
                            warn!(
                                stream_id = %stream_id,
                                delivery_count = count,
                                max_retries = config.retry.max_retries,
                                "Permanent failure exhausted retry budget, routing to DLQ"
                            );
                            self.xadd_dead_letter(
                                &config.retry,
                                &stream_id,
                                &msg.payload,
                                count,
                                "max_retries_exhausted",
                            )
                            .await?;
                            self.clear_failure_marker_safe(
                                &classification_marker_key,
                                &stream_id,
                                &classification_paused,
                            )
                            .await?;
                        } else {
                            warn!(stream_id = %stream_id, "Pending message permanent failure, leaving in PEL for ClaimSweeper");
                            self.mark_permanent_safe(
                                &classification_marker_key,
                                &stream_id,
                                &classification_paused,
                            )
                            .await?;
                        }
                    }
                }
            }
        }

        // Phase 2: main loop with worker pool
        // Circuit breaker state is preserved across connection errors (handled in-loop with backoff).
        info!("Phase 2: consuming new messages with worker pool");
        let mut cb = config.retry.circuit_breaker.as_ref().map(|cfg| {
            CircuitBreaker::new(cfg.clone()).with_labels("redis", config.retry.base.stream.clone())
        });

        let mut consecutive_conn_errors: u32 = 0;

        // Periodic pending drain — ClaimSweeper XCLAIMs failed messages back to our PEL;
        // XREADGROUP ">" never sees them. We poll "0" periodically so reclaimed entries
        // are re-processed during steady-state (same pattern as run_with_retry).
        let mut pending_check_interval = interval(Duration::from_secs(1));
        pending_check_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        // New-message poll interval. Must use `interval` (not `sleep`) inside
        // select! so the timer state persists when other branches fire first.
        // With `sleep`, the timer resets on every select iteration, causing
        // starvation when block_ms >= pending_check_interval period.
        let mut new_msg_interval = interval(Duration::from_millis(config.block_ms as u64));
        new_msg_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        let loop_result: Result<(), RedisConsumerError> = loop {
            // Circuit breaker check: if open, pause consumption
            if let Some(ref mut breaker) = cb
                && !breaker.should_allow_request()
            {
                let cooldown = breaker.remaining_cooldown();
                info!(
                    cooldown = ?cooldown,
                    stream = %config.retry.base.stream,
                    "Circuit breaker OPEN — pausing consumption"
                );
                sleep(cooldown).await;
                continue;
            }

            tokio::select! {
                // Periodic drain of own PEL — reclaimed messages get handler retries
                _ = pending_check_interval.tick() => {
                    if let Some(ref breaker) = cb
                        && breaker.is_open()
                    {
                        continue;
                    }
                    let pending = match self.xreadgroup(
                        &config.retry.base.stream,
                        &config.retry.base.group_name,
                        &config.retry.base.consumer_name,
                        config.prefetch_count,
                        "0",
                    ).await {
                        Ok(entries) => entries,
                        Err(e) if e.is_connection_error() => {
                            warn!(
                                error = %e,
                                stream = %config.retry.base.stream,
                                "Connection error during pending drain, forcing reconnect..."
                            );
                            self.connection.force_reconnect().await;
                            continue;
                        }
                        Err(e) => break Err(e),
                    };
                    if !pending.is_empty() {
                        let delivery_counts = self
                            .get_pending_delivery_counts(
                                &config.retry.base.stream,
                                &config.retry.base.group_name,
                                &pending,
                            )
                            .await;
                        for (stream_id, data) in pending {
                            if let Some(ref breaker) = cb
                                && breaker.is_open()
                            {
                                break;
                            }
                            let should_dispatch = {
                                let mut set = in_flight.lock().await;
                                set.insert(stream_id.clone())
                            };
                            if !should_dispatch {
                                debug!(stream_id = %stream_id, "Skipping pending entry already in-flight");
                                continue;
                            }
                            let count = delivery_counts.get(&stream_id).copied().unwrap_or(2);
                            let msg = build_message(
                                stream_id.clone(),
                                config.retry.base.stream.clone(),
                                count,
                                data,
                            );
                            // Create guard before spawn so if the main loop panics between
                            // in-flight insert and spawn, cleanup is still signaled on drop.
                            let dispatch_guard = WorkerAbnormalExitGuard::new(
                                stream_id.clone(),
                                abnormal_exit_tx.clone(),
                            );
                            let handler = Arc::clone(&handler);
                            let tx = result_tx.clone();
                            tokio::spawn(async move {
                                let mut exit_guard = dispatch_guard;
                                let delivery_count = msg.metadata.delivery_count;
                                let handler_start = std::time::Instant::now();
                                let call_result = handler.call(&msg).await;
                                metrics::histogram!("broker_handler_duration_seconds",
                                    "backend" => "redis",
                                    "topic" => msg.metadata.topic.clone(),
                                ).record(handler_start.elapsed().as_secs_f64());
                                // Include payload for Dead and all error outcomes so the
                                // main loop can route to DLQ without re-reading the stream.
                                let needs_payload = matches!(&call_result, Ok(AckDecision::Dead) | Err(_));
                                let payload = needs_payload.then(|| msg.payload.clone());
                                let outcome = HandlerOutcome::from(call_result);
                                match tx
                                    .send(ProcessingResult {
                                        stream_id: stream_id.clone(),
                                        outcome,
                                        payload,
                                        delivery_count,
                                    })
                                    .await
                                {
                                    Ok(()) => {
                                        // Main loop now owns cleanup via result channel.
                                        exit_guard.disarm();
                                    }
                                    Err(e) => {
                                        error!(
                                            ?e,
                                            stream_id = %stream_id,
                                            "Failed to send pending processing result - message stays in PEL"
                                        );
                                        // Keep guard armed: drop notifies abnormal-exit channel.
                                    }
                                }
                            });
                        }
                    }
                }

                // Poll for new messages on a periodic interval.
                // We do NOT use Redis BLOCK — it's incompatible with
                // MultiplexedConnection (redis-rs #1236) and hangs
                // indefinitely on dead sockets.
                _ = new_msg_interval.tick() => {
                    match self.xreadgroup(
                        &config.retry.base.stream,
                        &config.retry.base.group_name,
                        &config.retry.base.consumer_name,
                        config.prefetch_count,
                        ">",
                    ).await {
                        Ok(entries) => {
                            consecutive_conn_errors = 0;
                            for (stream_id, data) in entries {
                                let should_dispatch = {
                                    let mut set = in_flight.lock().await;
                                    set.insert(stream_id.clone())
                                };
                                if !should_dispatch {
                                    debug!(stream_id = %stream_id, "Skipping new entry already in-flight");
                                    continue;
                                }
                                let msg = build_message(
                                    stream_id.clone(),
                                    config.retry.base.stream.clone(),
                                    1,
                                    data,
                                );
                                let dispatch_guard = WorkerAbnormalExitGuard::new(
                                    stream_id.clone(),
                                    abnormal_exit_tx.clone(),
                                );

                                let handler = Arc::clone(&handler);
                                let tx = result_tx.clone();

                                tokio::spawn(async move {
                                    let mut exit_guard = dispatch_guard;
                                    let delivery_count = msg.metadata.delivery_count;
                                    let handler_start = std::time::Instant::now();
                                    let call_result = handler.call(&msg).await;
                                    metrics::histogram!("broker_handler_duration_seconds",
                                        "backend" => "redis",
                                        "topic" => msg.metadata.topic.clone(),
                                    ).record(handler_start.elapsed().as_secs_f64());
                                    // Include payload for Dead and all error outcomes so the
                                    // main loop can route to DLQ without re-reading the stream.
                                    let needs_payload = matches!(&call_result, Ok(AckDecision::Dead) | Err(_));
                                    let payload = needs_payload.then(|| msg.payload.clone());
                                    let outcome = HandlerOutcome::from(call_result);
                                    match tx
                                        .send(ProcessingResult {
                                            stream_id: stream_id.clone(),
                                            outcome,
                                            payload,
                                            delivery_count,
                                        })
                                        .await
                                    {
                                        Ok(()) => {
                                            exit_guard.disarm();
                                        }
                                        Err(e) => {
                                            error!(
                                                ?e,
                                                stream_id = %stream_id,
                                                "Failed to send processing result - message stays in PEL"
                                            );
                                        }
                                    }
                                });
                            }
                        }
                        Err(e) if e.is_connection_error() => {
                            consecutive_conn_errors += 1;
                            warn!(
                                error = %e,
                                consecutive = consecutive_conn_errors,
                                stream = %config.retry.base.stream,
                                "Connection error in prefetch consumer, forcing reconnect..."
                            );
                            metrics::counter!("broker_consumer_reconnections_total",
                                "backend" => "redis",
                                "topic" => config.retry.base.stream.clone(),
                            ).increment(1);
                            metrics::gauge!(
                                "broker_consumer_connected",
                                "backend" => "redis",
                                "topic" => config.retry.base.stream.clone(),
                            )
                            .set(0.0);
                            self.connection.force_reconnect().await;
                            sleep(CONN_RETRY_BACKOFF).await;
                            metrics::gauge!(
                                "broker_consumer_connected",
                                "backend" => "redis",
                                "topic" => config.retry.base.stream.clone(),
                            )
                            .set(1.0);
                        }
                        Err(e) => break Err(e),
                    }
                }

                // Process completed results — ACK happens here (not in spawned tasks)
                Some(result) = result_rx.recv() => {
                    {
                        let mut set = in_flight.lock().await;
                        set.remove(&result.stream_id);
                    }

                    // ── metrics: outcome counter + delivery count ──
                    metrics::counter!("broker_messages_consumed_total",
                        "backend" => "redis",
                        "topic" => config.retry.base.stream.clone(),
                        "outcome" => crate::metrics::outcome_label(&result.outcome),
                    ).increment(1);
                    metrics::histogram!("broker_message_delivery_count",
                        "backend" => "redis",
                        "topic" => config.retry.base.stream.clone(),
                    ).record(result.delivery_count as f64);

                    match result.outcome {
                        HandlerOutcome::Ack => {
                            if let Some(ref mut breaker) = cb { breaker.record_success(); }
                            debug!(stream_id = %result.stream_id, "Handler succeeded, acknowledging");
                            if let Err(e) = self.xack(
                                &config.retry.base.stream,
                                &config.retry.base.group_name,
                                &result.stream_id,
                            ).await {
                                if e.is_connection_error() {
                                    warn!(error = %e, stream_id = %result.stream_id, "Connection error during XACK, message stays in PEL");
                                } else {
                                    break Err(e);
                                }
                            } else if let Err(e) = self
                                .clear_failure_marker_safe(
                                    &classification_marker_key,
                                    &result.stream_id,
                                    &classification_paused,
                                )
                                .await
                            {
                                break Err(e);
                            }
                        }
                        HandlerOutcome::Nack | HandlerOutcome::Delay(_) => {
                            // Voluntary yield — leave in PEL so ClaimSweeper retries after claim_min_idle.
                            // Circuit breaker is unaffected (not an infra failure).
                            if let Some(ref mut breaker) = cb { breaker.record_success(); }
                            debug!(stream_id = %result.stream_id, "Handler voluntarily yielded, leaving in PEL for ClaimSweeper");
                            if let Err(e) = self
                                .clear_failure_marker_safe(
                                    &classification_marker_key,
                                    &result.stream_id,
                                    &classification_paused,
                                )
                                .await
                            {
                                break Err(e);
                            }
                        }
                        HandlerOutcome::Dead => {
                            if let Some(ref mut breaker) = cb { breaker.record_permanent_failure(); }
                            metrics::counter!("broker_messages_dead_lettered_total",
                                "backend" => "redis",
                                "topic" => config.retry.base.stream.clone(),
                                "reason" => "handler_requested",
                            ).increment(1);
                            warn!(stream_id = %result.stream_id, "Handler requested dead-letter, routing immediately");
                            let payload = result.payload.as_deref().unwrap_or(&[]);
                            if let Err(e) = self
                                .xadd_dead_letter(
                                    &config.retry,
                                    &result.stream_id,
                                    payload,
                                    result.delivery_count,
                                    "AckDecision::Dead",
                                )
                                .await
                            {
                                break Err(e);
                            }
                            if let Err(e) = self
                                .clear_failure_marker_safe(
                                    &classification_marker_key,
                                    &result.stream_id,
                                    &classification_paused,
                                )
                                .await
                            {
                                break Err(e);
                            }
                        }
                        HandlerOutcome::Transient => {
                            if let Some(ref mut breaker) = cb { breaker.record_transient_failure(); }
                            warn!(stream_id = %result.stream_id, "Transient failure, leaving in PEL");
                            if let Err(e) = self
                                .mark_transient_safe(
                                    &classification_marker_key,
                                    &result.stream_id,
                                    &classification_paused,
                                )
                                .await
                            {
                                break Err(e);
                            }
                        }
                        HandlerOutcome::Permanent => {
                            if let Some(ref mut breaker) = cb { breaker.record_permanent_failure(); }
                            if result.delivery_count >= config.retry.max_retries as u64 {
                                // Retry budget exhausted — DLQ immediately instead of
                                // waiting for ClaimSweeper to win the idle-time race
                                // against the periodic pending drain.
                                metrics::counter!("broker_messages_dead_lettered_total",
                                    "backend" => "redis",
                                    "topic" => config.retry.base.stream.clone(),
                                    "reason" => "max_retries_exhausted",
                                ).increment(1);
                                warn!(
                                    stream_id = %result.stream_id,
                                    delivery_count = result.delivery_count,
                                    max_retries = config.retry.max_retries,
                                    "Permanent failure exhausted retry budget, routing to DLQ"
                                );
                                let payload = result.payload.as_deref().unwrap_or(&[]);
                                if let Err(e) = self
                                    .xadd_dead_letter(
                                        &config.retry,
                                        &result.stream_id,
                                        payload,
                                        result.delivery_count,
                                        "max_retries_exhausted",
                                    )
                                    .await
                                {
                                    break Err(e);
                                }
                                if let Err(e) = self
                                    .clear_failure_marker_safe(
                                        &classification_marker_key,
                                        &result.stream_id,
                                        &classification_paused,
                                    )
                                    .await
                                {
                                    break Err(e);
                                }
                            } else {
                                warn!(stream_id = %result.stream_id, "Handler failed, leaving in PEL for ClaimSweeper");
                                if let Err(e) = self
                                    .mark_permanent_safe(
                                        &classification_marker_key,
                                        &result.stream_id,
                                        &classification_paused,
                                    )
                                    .await
                                {
                                    break Err(e);
                                }
                            }
                        }
                    }
                }

                // Worker ended before delivering `ProcessingResult` (panic or send failure).
                Some(stream_id) = abnormal_exit_rx.recv() => {
                    let removed = {
                        let mut set = in_flight.lock().await;
                        set.remove(&stream_id)
                    };
                    if removed {
                        warn!(
                            stream_id = %stream_id,
                            "Worker exited before reporting result, cleared in-flight marker"
                        );
                    }
                }

                // Graceful shutdown — finish current batch, then drain.
                _ = self.cancel_token.cancelled() => {
                    info!(
                        stream = %config.retry.base.stream,
                        "Cancellation requested, stopping consumer gracefully"
                    );
                    break Ok(());
                }
            }
        };

        // Shutdown drain path for fatal loop errors:
        // - stop the claim sweeper
        // - flush already-computed worker outcomes from the channel
        // Classification persistence uses the same safe retry logic as steady-state
        // so stale markers are not left behind on connection outages.
        cancel.cancel();
        info!("Draining remaining results before shutdown");
        while let Ok(result) = result_rx.try_recv() {
            {
                let mut set = in_flight.lock().await;
                set.remove(&result.stream_id);
            }
            match result.outcome {
                HandlerOutcome::Ack => {
                    match self
                        .xack(
                            &config.retry.base.stream,
                            &config.retry.base.group_name,
                            &result.stream_id,
                        )
                        .await
                    {
                        Err(e) => {
                            warn!(error = %e, stream_id = %result.stream_id, "Failed to XACK during shutdown drain, message stays in PEL");
                        }
                        Ok(()) => {
                            if let Err(e) = self
                                .clear_failure_marker(&classification_marker_key, &result.stream_id)
                                .await
                            {
                                warn!(error = %e, stream_id = %result.stream_id, "Failed to clear failure markers during shutdown drain");
                            }
                        }
                    }
                }
                HandlerOutcome::Nack | HandlerOutcome::Delay(_) => {
                    // leave in PEL — ClaimSweeper will retry
                    if let Err(e) = self
                        .clear_failure_marker(&classification_marker_key, &result.stream_id)
                        .await
                    {
                        warn!(error = %e, stream_id = %result.stream_id, "Failed to clear failure markers during shutdown drain");
                    }
                }
                HandlerOutcome::Dead => {
                    let payload = result.payload.as_deref().unwrap_or(&[]);
                    match self
                        .xadd_dead_letter(
                            &config.retry,
                            &result.stream_id,
                            payload,
                            result.delivery_count,
                            "AckDecision::Dead",
                        )
                        .await
                    {
                        Err(e) => {
                            warn!(error = %e, stream_id = %result.stream_id, "Failed to add to dead-letter during shutdown drain");
                        }
                        Ok(()) => {
                            if let Err(e) = self
                                .clear_failure_marker(&classification_marker_key, &result.stream_id)
                                .await
                            {
                                warn!(error = %e, stream_id = %result.stream_id, "Failed to clear failure markers during shutdown drain");
                            }
                        }
                    }
                }
                HandlerOutcome::Transient => {
                    // leave in PEL
                    if let Err(e) = self
                        .mark_transient(&classification_marker_key, &result.stream_id)
                        .await
                    {
                        warn!(error = %e, stream_id = %result.stream_id, "Failed to mark transient classification during shutdown drain");
                    }
                }
                HandlerOutcome::Permanent => {
                    if result.delivery_count >= config.retry.max_retries as u64 {
                        let payload = result.payload.as_deref().unwrap_or(&[]);
                        match self
                            .xadd_dead_letter(
                                &config.retry,
                                &result.stream_id,
                                payload,
                                result.delivery_count,
                                "max_retries_exhausted",
                            )
                            .await
                        {
                            Err(e) => {
                                warn!(error = %e, stream_id = %result.stream_id, "Failed to DLQ exhausted message during shutdown drain");
                            }
                            Ok(()) => {
                                if let Err(e) = self
                                    .clear_failure_marker(
                                        &classification_marker_key,
                                        &result.stream_id,
                                    )
                                    .await
                                {
                                    warn!(error = %e, stream_id = %result.stream_id, "Failed to clear failure markers during shutdown drain");
                                }
                            }
                        }
                    } else if let Err(e) = self
                        .mark_permanent(&classification_marker_key, &result.stream_id)
                        .await
                    {
                        warn!(error = %e, stream_id = %result.stream_id, "Failed to mark permanent classification during shutdown drain");
                    }
                }
            }
        }
        while let Ok(stream_id) = abnormal_exit_rx.try_recv() {
            let mut set = in_flight.lock().await;
            set.remove(&stream_id);
        }
        info!("Safe consumer shutdown complete");
        loop_result
    }

    // ─────────────────────────────────────────────────────────────
    // Private helpers
    // ─────────────────────────────────────────────────────────────

    async fn classification_set(
        &self,
        marker_key: &str,
        stream_id: &str,
        class: &str,
    ) -> Result<(), RedisConsumerError> {
        let mut conn = self.connection.get_connection();
        let _: i64 = redis::cmd("HSET")
            .arg(marker_key)
            .arg(stream_id)
            .arg(class)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::TransientMarker {
                key: marker_key.to_string(),
                source: e,
            })?;
        Ok(())
    }

    async fn clear_failure_marker(
        &self,
        marker_key: &str,
        stream_id: &str,
    ) -> Result<(), RedisConsumerError> {
        let mut conn = self.connection.get_connection();
        let _: i64 = redis::cmd("HDEL")
            .arg(marker_key)
            .arg(stream_id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::TransientMarker {
                key: marker_key.to_string(),
                source: e,
            })?;
        Ok(())
    }

    async fn clear_failure_marker_safe(
        &self,
        marker_key: &str,
        stream_id: &str,
        classification_paused: &AtomicBool,
    ) -> Result<(), RedisConsumerError> {
        self.classification_write_safe(stream_id, classification_paused, "clear", || {
            self.clear_failure_marker(marker_key, stream_id)
        })
        .await
    }

    async fn mark_transient(
        &self,
        marker_key: &str,
        stream_id: &str,
    ) -> Result<(), RedisConsumerError> {
        self.classification_set(marker_key, stream_id, CLASS_TRANSIENT)
            .await
    }

    async fn mark_transient_safe(
        &self,
        marker_key: &str,
        stream_id: &str,
        classification_paused: &AtomicBool,
    ) -> Result<(), RedisConsumerError> {
        self.classification_write_safe(stream_id, classification_paused, "mark transient", || {
            self.mark_transient(marker_key, stream_id)
        })
        .await
    }

    async fn mark_permanent(
        &self,
        marker_key: &str,
        stream_id: &str,
    ) -> Result<(), RedisConsumerError> {
        self.classification_set(marker_key, stream_id, CLASS_PERMANENT)
            .await
    }

    async fn mark_permanent_safe(
        &self,
        marker_key: &str,
        stream_id: &str,
        classification_paused: &AtomicBool,
    ) -> Result<(), RedisConsumerError> {
        self.classification_write_safe(stream_id, classification_paused, "mark permanent", || {
            self.mark_permanent(marker_key, stream_id)
        })
        .await
    }

    async fn classification_write_safe<F, Fut>(
        &self,
        stream_id: &str,
        classification_paused: &AtomicBool,
        action: &'static str,
        mut op: F,
    ) -> Result<(), RedisConsumerError>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<(), RedisConsumerError>>,
    {
        let mut attempts: u32 = 0;
        loop {
            match op().await {
                Ok(()) => {
                    if classification_paused.swap(false, Ordering::Relaxed) {
                        info!("Classification persistence recovered, resuming ClaimSweeper");
                    }
                    return Ok(());
                }
                Err(e) if e.is_connection_error() => {
                    attempts += 1;
                    if attempts >= MAX_CLASSIFICATION_RETRIES {
                        warn!(
                            error = %e,
                            stream_id = %stream_id,
                            action = action,
                            attempts = attempts,
                            "Classification write exhausted retries, skipping — \
                             ClaimSweeper defaults to transient (safe) when marker is missing"
                        );
                        return Ok(());
                    }
                    if !classification_paused.swap(true, Ordering::Relaxed) {
                        warn!(
                            error = %e,
                            stream_id = %stream_id,
                            action = action,
                            "Connection error while persisting message classification, pausing ClaimSweeper until Redis recovers"
                        );
                    } else {
                        warn!(
                            error = %e,
                            stream_id = %stream_id,
                            action = action,
                            "Connection error while persisting message classification, retrying..."
                        );
                    }
                    sleep(CONN_RETRY_BACKOFF).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Create consumer group (idempotent).
    async fn ensure_group(&self, stream: &str, group: &str) -> Result<(), RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let result: Result<String, redis::RedisError> = redis::cmd("XGROUP")
            .arg("CREATE")
            .arg(stream)
            .arg(group)
            .arg("0")
            .arg("MKSTREAM")
            .query_async(&mut conn)
            .await;

        match result {
            Ok(_) => {
                info!(stream = %stream, group = %group, "Consumer group created");
            }
            Err(e) if e.to_string().contains("BUSYGROUP") => {
                debug!(stream = %stream, group = %group, "Consumer group already exists");
            }
            Err(e) => {
                return Err(RedisConsumerError::GroupCreation {
                    stream: stream.to_string(),
                    group: group.to_string(),
                    source: e,
                });
            }
        }

        Ok(())
    }

    /// XREADGROUP wrapper that returns parsed (stream_id, data) pairs.
    ///
    /// Non-blocking: does NOT use Redis BLOCK argument. BLOCK commands on
    /// `ConnectionManager` (which wraps `MultiplexedConnection`) are
    /// architecturally broken — a blocking call monopolizes the shared TCP
    /// connection and hangs indefinitely on dead sockets (redis-rs #1236).
    ///
    /// Callers use client-side `sleep()` as the polling interval instead.
    /// `response_timeout` (5s) on the `ConnectionManager` guarantees this
    /// method returns within bounded time even on dead sockets.
    ///
    /// Connection recovery is the **caller's** responsibility:
    /// callers must call `force_reconnect()` when they receive a connection
    /// error, then retry. This mirrors the RMQ consumer's outer reconnection
    /// loop pattern.
    ///
    /// `start_id` is `">"` for new messages or `"0"` for pending drain.
    async fn xreadgroup(
        &self,
        stream: &str,
        group: &str,
        consumer: &str,
        count: usize,
        start_id: &str,
    ) -> Result<Vec<(String, Vec<u8>)>, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let result: Value = redis::cmd("XREADGROUP")
            .arg("GROUP")
            .arg(group)
            .arg(consumer)
            .arg("COUNT")
            .arg(count)
            .arg("STREAMS")
            .arg(stream)
            .arg(start_id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: stream.to_string(),
                source: e,
            })?;

        Ok(parse_xreadgroup_response(result))
    }

    /// XACK wrapper.
    async fn xack(&self, stream: &str, group: &str, id: &str) -> Result<(), RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let _: i64 = redis::cmd("XACK")
            .arg(stream)
            .arg(group)
            .arg(id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::Acknowledge {
                stream: stream.to_string(),
                source: e,
            })?;

        Ok(())
    }

    /// Query the real delivery count for a batch of pending entry IDs via `XPENDING`.
    ///
    /// Returns a map from stream ID to delivery count. On connection error,
    /// returns an empty map (callers fall back to a default).
    async fn get_pending_delivery_counts(
        &self,
        stream: &str,
        group: &str,
        ids: &[(String, Vec<u8>)],
    ) -> HashMap<String, u64> {
        if ids.is_empty() {
            return HashMap::new();
        }

        let mut conn = self.connection.get_connection();

        let first_id = &ids[0].0;
        let last_id = &ids[ids.len() - 1].0;

        let result: Result<Value, redis::RedisError> = redis::cmd("XPENDING")
            .arg(stream)
            .arg(group)
            .arg(first_id)
            .arg(last_id)
            .arg(ids.len())
            .query_async(&mut conn)
            .await;

        match result {
            Ok(Value::Array(items)) => {
                let mut map = HashMap::with_capacity(items.len());
                for item in items {
                    if let Value::Array(fields) = item
                        && fields.len() >= 4
                    {
                        let id = match &fields[0] {
                            Value::BulkString(b) => String::from_utf8_lossy(b).to_string(),
                            _ => continue,
                        };
                        let count = match &fields[3] {
                            Value::Int(n) => *n as u64,
                            _ => continue,
                        };
                        map.insert(id, count);
                    }
                }
                map
            }
            Ok(_) => HashMap::new(),
            Err(e) => {
                warn!(
                    error = %e,
                    stream = %stream,
                    "Failed to query delivery counts via XPENDING, falling back to defaults"
                );
                HashMap::new()
            }
        }
    }

    /// Route a message directly to the dead stream and XACK it from the main stream.
    ///
    /// Used when a handler returns [`AckDecision::Dead`] or when a permanent failure
    /// exhausts its retry budget (`delivery_count >= max_retries`). The message is
    /// published to `config.dead_stream` via `XADD` with the original payload, then
    /// XACK'd so it is removed from the main stream's PEL immediately.
    async fn xadd_dead_letter(
        &self,
        config: &super::config::RedisRetryConfig,
        stream_id: &str,
        payload: &[u8],
        delivery_count: u64,
        reason: &str,
    ) -> Result<(), RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let _: String = redis::cmd("XADD")
            .arg(&config.dead_stream)
            .arg("*")
            .arg("original_id")
            .arg(stream_id)
            .arg("original_stream")
            .arg(&config.base.stream)
            .arg("delivery_count")
            .arg(delivery_count)
            .arg("reason")
            .arg(reason)
            .arg("data")
            .arg(payload)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::DeadLetter {
                stream: config.dead_stream.clone(),
                source: e,
            })?;

        // XACK from the main stream to remove from PEL
        self.xack(&config.base.stream, &config.base.group_name, stream_id)
            .await
    }
}

/// Build a `Message` from stream entry components.
fn build_message(
    stream_id: String,
    stream_name: String,
    delivery_count: u64,
    data: Vec<u8>,
) -> Message {
    Message {
        payload: data,
        metadata: MessageMetadata {
            id: stream_id,
            topic: stream_name,
            delivery_count,
            headers: std::collections::HashMap::new(),
        },
    }
}

/// Parse the XREADGROUP response into (stream_id, data) pairs.
///
/// Response format:
/// ```text
/// [[stream_name, [[id, [field, value, ...]], ...]], ...]
/// ```
///
/// We extract the "data" field from each entry.
fn parse_xreadgroup_response(value: Value) -> Vec<(String, Vec<u8>)> {
    let mut results = Vec::new();

    // Nil response means no messages (BLOCK timeout)
    let streams = match value {
        Value::Array(s) => s,
        Value::Nil => return results,
        _ => return results,
    };

    for stream in streams {
        let parts = match stream {
            Value::Array(p) if p.len() >= 2 => p,
            _ => continue,
        };

        // parts[0] = stream name, parts[1] = entries array
        let entries = match &parts[1] {
            Value::Array(e) => e,
            _ => continue,
        };

        for entry in entries {
            let entry_parts = match entry {
                Value::Array(ep) if ep.len() >= 2 => ep,
                _ => continue,
            };

            // entry_parts[0] = stream ID
            let stream_id = match &entry_parts[0] {
                Value::BulkString(b) => String::from_utf8_lossy(b).to_string(),
                _ => continue,
            };

            // entry_parts[1] = fields array [key, value, key, value, ...]
            let fields = match &entry_parts[1] {
                Value::Array(f) => f,
                _ => continue,
            };

            // Find the "data" field
            let mut data = Vec::new();
            let mut iter = fields.iter();
            while let Some(key) = iter.next() {
                if let Value::BulkString(k) = key
                    && k == b"data"
                    && let Some(Value::BulkString(v)) = iter.next()
                {
                    data = v.clone();
                    break;
                }
                // Skip value
                let _ = iter.next();
            }

            results.push((stream_id, data));
        }
    }

    results
}

#[async_trait::async_trait]
impl crate::traits::consumer::Consumer for RedisConsumer {
    type PrefetchConfig = RedisPrefetchConfig;
    type Error = RedisConsumerError;

    async fn connect(url: &str) -> Result<Self, Self::Error> {
        Self::with_url(url).await
    }

    async fn run(
        &self,
        config: Self::PrefetchConfig,
        handler: impl Handler + 'static,
    ) -> Result<(), Self::Error> {
        self.run(config, handler).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_message() {
        let msg = build_message(
            "1234-0".to_string(),
            "test.stream".to_string(),
            1,
            b"hello".to_vec(),
        );

        assert_eq!(msg.payload, b"hello");
        assert_eq!(msg.metadata.id, "1234-0");
        assert_eq!(msg.metadata.topic, "test.stream");
        assert_eq!(msg.metadata.delivery_count, 1);
    }

    #[test]
    fn test_parse_xreadgroup_response_nil() {
        let result = parse_xreadgroup_response(Value::Nil);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_xreadgroup_response_empty() {
        let result = parse_xreadgroup_response(Value::Array(vec![]));
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_xreadgroup_response_with_data() {
        let value = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"test.stream".to_vec()),
            Value::Array(vec![Value::Array(vec![
                Value::BulkString(b"1234-0".to_vec()),
                Value::Array(vec![
                    Value::BulkString(b"data".to_vec()),
                    Value::BulkString(b"{\"block\":42}".to_vec()),
                ]),
            ])]),
        ])]);

        let result = parse_xreadgroup_response(value);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "1234-0");
        assert_eq!(result[0].1, b"{\"block\":42}");
    }

    #[test]
    fn test_parse_xreadgroup_response_multiple_entries() {
        let value = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"test.stream".to_vec()),
            Value::Array(vec![
                Value::Array(vec![
                    Value::BulkString(b"1-0".to_vec()),
                    Value::Array(vec![
                        Value::BulkString(b"data".to_vec()),
                        Value::BulkString(b"msg1".to_vec()),
                    ]),
                ]),
                Value::Array(vec![
                    Value::BulkString(b"2-0".to_vec()),
                    Value::Array(vec![
                        Value::BulkString(b"data".to_vec()),
                        Value::BulkString(b"msg2".to_vec()),
                    ]),
                ]),
            ]),
        ])]);

        let result = parse_xreadgroup_response(value);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "1-0");
        assert_eq!(result[1].0, "2-0");
    }

    #[test]
    fn test_parse_xreadgroup_response_extra_fields() {
        // Entry with extra fields besides "data"
        let value = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"test.stream".to_vec()),
            Value::Array(vec![Value::Array(vec![
                Value::BulkString(b"1-0".to_vec()),
                Value::Array(vec![
                    Value::BulkString(b"type".to_vec()),
                    Value::BulkString(b"block".to_vec()),
                    Value::BulkString(b"data".to_vec()),
                    Value::BulkString(b"payload".to_vec()),
                ]),
            ])]),
        ])]);

        let result = parse_xreadgroup_response(value);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, b"payload");
    }
}
