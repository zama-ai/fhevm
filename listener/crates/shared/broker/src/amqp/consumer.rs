use futures::stream::StreamExt;
use lapin::{
    Channel, ExchangeKind,
    message::Delivery,
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicPublishOptions,
        BasicQosOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
    },
    types::{AMQPValue, FieldTable, ShortString},
};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use tokio_util::sync::CancellationToken;

use crate::traits::circuit_breaker::CircuitBreaker;
use crate::traits::handler::{Handler, HandlerOutcome};
use crate::traits::message::{Message, MessageMetadata};

use super::{
    config::{CronConfig, PrefetchConfig, RetryConfig},
    connection::ConnectionManager,
    error::ConsumerError,
};

/// Result of processing a message in a worker task.
/// Used by `run()` to communicate results back to the main loop.
struct ProcessingResult {
    /// The original delivery, needed for ACK/NACK
    delivery: Delivery,
    /// Outcome classification — preserves the Transient/Permanent distinction
    /// so the main loop can call the correct circuit breaker method.
    outcome: HandlerOutcome,
}

/// RabbitMQ consumer service.
pub struct RmqConsumer {
    connection: Arc<ConnectionManager>,
    cancel_token: CancellationToken,
}

impl RmqConsumer {
    const PERMANENT_RETRY_COUNT_HEADER: &'static str = "x-mq-permanent-retry-count";

    /// Create a new consumer with the given connection manager.
    pub fn new(connection: Arc<ConnectionManager>) -> Self {
        Self {
            connection,
            cancel_token: CancellationToken::new(),
        }
    }

    /// Create a new consumer with a fresh connection manager.
    pub fn with_addr(addr: impl Into<String>) -> Self {
        Self {
            connection: Arc::new(ConnectionManager::new(addr)),
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

    /// Get the underlying connection manager.
    pub fn connection(&self) -> &Arc<ConnectionManager> {
        &self.connection
    }

    /// Ensure the AMQP topology (exchanges, queues, bindings) exists without
    /// starting to consume.
    ///
    /// Call this before checking queue depth or publishing seed messages to
    /// guarantee that queues are bound to exchanges and messages won't be
    /// silently dropped.
    pub async fn ensure_topology(&self, config: &PrefetchConfig) -> Result<(), ConsumerError> {
        let channel = self.connection.create_channel_with_retry().await;
        self.setup_retry_queues(&channel, &config.retry).await
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Public consumer methods — each wraps a reconnection loop around an inner
    // consume loop. On reconnectable errors (connection drop, channel death,
    // stream end), the outer loop creates a fresh channel and re-subscribes.
    // Only `Configuration` errors propagate to the caller.
    // ═══════════════════════════════════════════════════════════════════════════

    /// Run a high-throughput consumer with STRONG message loss guarantees.
    ///
    /// This consumer:
    /// - ACKs/NACKs only from the main loop (not in spawned tasks)
    /// - Guarantees no message loss even on worker crash
    /// - Uses bounded channel to prevent memory exhaustion
    ///
    /// Automatically reconnects on connection loss. Circuit breaker state
    /// persists across reconnections. On reconnection the mpsc channel and
    /// spawned tasks are dropped; unacked messages are redelivered by RabbitMQ.
    pub async fn run(
        &self,
        config: PrefetchConfig,
        handler: impl Handler + 'static,
    ) -> Result<(), ConsumerError> {
        let handler: Arc<dyn Handler> = Arc::new(handler);
        let mut cb = config.retry.circuit_breaker.as_ref().map(|cfg| {
            CircuitBreaker::new(cfg.clone()).with_labels("amqp", config.retry.base.queue.clone())
        });

        loop {
            let channel = self.connection.create_channel_with_retry().await;

            // Signal "connected" as soon as the channel is established.
            metrics::gauge!(
                "broker_consumer_connected",
                "backend" => "amqp",
                "topic" => config.retry.base.queue.clone(),
            )
            .set(1.0);

            let result = self
                .consume_loop_prefetch_safe(&channel, &config, Arc::clone(&handler), &mut cb)
                .await;

            match result {
                Err(e) if e.is_reconnectable() => {
                    if self.cancel_token.is_cancelled() {
                        info!(queue = %config.retry.base.queue, "Cancellation requested, skipping reconnect");
                        return Ok(());
                    }
                    metrics::counter!("broker_consumer_reconnections_total",
                        "backend" => "amqp",
                        "topic" => config.retry.base.queue.clone(),
                    )
                    .increment(1);
                    metrics::gauge!(
                        "broker_consumer_connected",
                        "backend" => "amqp",
                        "topic" => config.retry.base.queue.clone(),
                    )
                    .set(0.0);
                    warn!(
                        error = %e,
                        delay = ?self.connection.reconnect_delay,
                        queue = %config.retry.base.queue,
                        "Prefetch-safe consumer disconnected, reconnecting..."
                    );
                    sleep(self.connection.reconnect_delay).await;
                }
                other => return other,
            }
        }
    }

    /// Run a cron-style scheduled job consumer.
    ///
    /// Automatically reconnects on connection loss.
    pub async fn run_cron(
        &self,
        config: CronConfig,
        handler: impl Handler,
    ) -> Result<(), ConsumerError> {
        loop {
            let channel = self.connection.create_channel_with_retry().await;

            let result = self.consume_loop_cron(&channel, &config, &handler).await;

            match result {
                Err(e) if e.is_reconnectable() => {
                    warn!(
                        error = %e,
                        delay = ?self.connection.reconnect_delay,
                        queue = %config.base.queue,
                        "Cron consumer disconnected, reconnecting..."
                    );
                    sleep(self.connection.reconnect_delay).await;
                }
                other => return other,
            }
        }
    }

    async fn consume_loop_prefetch_safe(
        &self,
        channel: &Channel,
        config: &PrefetchConfig,
        handler: Arc<dyn Handler>,
        cb: &mut Option<CircuitBreaker>,
    ) -> Result<(), ConsumerError> {
        channel
            .basic_qos(config.prefetch_count, BasicQosOptions::default())
            .await
            .map_err(ConsumerError::Connection)?;

        self.setup_retry_queues(channel, &config.retry).await?;

        let mut consumer = channel
            .basic_consume(
                ShortString::from(config.retry.base.queue.as_str()),
                ShortString::from(config.retry.base.consumer_tag.as_str()),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::ConsumerRegistration {
                consumer_tag: config.retry.base.consumer_tag.clone(),
                source: e,
            })?;

        info!(
            exchange = %config.retry.base.exchange,
            queue = %config.retry.base.queue,
            prefetch_count = %config.prefetch_count,
            circuit_breaker = %config.retry.circuit_breaker.is_some(),
            "Safe high-throughput consumer started (ACK in main loop)"
        );

        let channel = Arc::new(channel.clone());

        let (result_tx, mut result_rx) =
            mpsc::channel::<ProcessingResult>(config.prefetch_count as usize);

        loop {
            if let Some(breaker) = cb
                && !breaker.should_allow_request()
            {
                let cooldown = breaker.remaining_cooldown();
                info!(
                    cooldown = ?cooldown,
                    queue = %config.retry.base.queue,
                    "Circuit breaker OPEN — pausing consumption"
                );
                sleep(cooldown).await;
                continue;
            }

            tokio::select! {
                maybe_delivery = consumer.next() => {
                    let delivery = match maybe_delivery {
                        Some(Ok(d)) => d,
                        Some(Err(_)) => {
                            info!("Channel error detected, breaking for reconnection");
                            break;
                        }
                        None => {
                            info!("Consumer stream ended");
                            break;
                        }
                    };

                    let handler = Arc::clone(&handler);
                    let tx = result_tx.clone();
                    let msg = Self::delivery_to_message(&delivery, &config.retry.base.queue);

                    tokio::spawn(async move {
                        let handler_start = std::time::Instant::now();
                        let outcome = HandlerOutcome::from(handler.call(&msg).await);
                        metrics::histogram!("broker_handler_duration_seconds",
                            "backend" => "amqp",
                            "topic" => msg.metadata.topic.clone(),
                        ).record(handler_start.elapsed().as_secs_f64());
                        if let Err(e) = tx.send(ProcessingResult { delivery, outcome }).await {
                            error!(?e, "Failed to send processing result - message will be requeued");
                        }
                    });
                }

                Some(result) = result_rx.recv() => {
                    // ── metrics: outcome counter ──
                    metrics::counter!("broker_messages_consumed_total",
                        "backend" => "amqp",
                        "topic" => config.retry.base.queue.clone(),
                        "outcome" => crate::metrics::outcome_label(&result.outcome),
                    ).increment(1);

                    match result.outcome {
                        HandlerOutcome::Ack => {
                            if let Some(b) = cb { b.record_success(); }
                            debug!("Handler succeeded, acknowledging message");
                            result.delivery
                                .ack(BasicAckOptions::default())
                                .await
                                .map_err(ConsumerError::Ack)?;
                        }
                        HandlerOutcome::Nack => {
                            if let Some(b) = cb { b.record_success(); }
                            debug!("Handler voluntarily yielded, requeueing at tail of main queue");
                            result.delivery
                                .nack(BasicNackOptions { requeue: true, ..Default::default() })
                                .await
                                .map_err(ConsumerError::Nack)?;
                        }
                        HandlerOutcome::Dead => {
                            if let Some(b) = cb { b.record_permanent_failure(); }
                            metrics::counter!("broker_messages_dead_lettered_total",
                                "backend" => "amqp",
                                "topic" => config.retry.base.queue.clone(),
                                "reason" => "handler_requested",
                            ).increment(1);
                            warn!("Handler requested immediate dead-letter, bypassing retry");
                            Self::handle_dead_letter_static(&channel, &result.delivery, &config.retry).await?;
                        }
                        HandlerOutcome::Delay(duration) => {
                            if let Some(b) = cb { b.record_success(); }
                            debug!(delay_ms = %duration.as_millis(), "Handler requested delay requeue");
                            Self::handle_delay_static(&channel, &result.delivery, &config.retry, duration).await?;
                        }
                        HandlerOutcome::Transient => {
                            if let Some(b) = cb { b.record_transient_failure(); }
                            warn!("Transient failure, scheduling infinite retry");
                            Self::handle_transient_retry_static(&result.delivery).await?;
                        }
                        HandlerOutcome::Permanent => {
                            if let Some(b) = cb { b.record_permanent_failure(); }
                            error!("Handler failed, applying bounded retry policy");
                            Self::handle_retry_static(&channel, &result.delivery, &config.retry).await?;
                        }
                    }
                }

                // Graceful shutdown — finish current batch, then drain.
                _ = self.cancel_token.cancelled() => {
                    info!(
                        queue = %config.retry.base.queue,
                        "Cancellation requested, stopping AMQP consumer gracefully"
                    );
                    break;
                }
            }
        }

        // Drain remaining results before returning — ACK/NACK failures are
        // ignored since the channel is dead; RabbitMQ will redeliver unacked messages.
        info!("Draining remaining results before reconnection");
        while let Ok(result) = result_rx.try_recv() {
            match result.outcome {
                HandlerOutcome::Ack => {
                    let _ = result.delivery.ack(BasicAckOptions::default()).await;
                }
                HandlerOutcome::Nack => {
                    let _ = result
                        .delivery
                        .nack(BasicNackOptions {
                            requeue: true,
                            ..Default::default()
                        })
                        .await;
                }
                HandlerOutcome::Transient => {
                    let _ = Self::handle_transient_retry_static(&result.delivery).await;
                }
                HandlerOutcome::Permanent => {
                    let _ =
                        Self::handle_retry_static(&channel, &result.delivery, &config.retry).await;
                }
                HandlerOutcome::Dead => {
                    let _ =
                        Self::handle_dead_letter_static(&channel, &result.delivery, &config.retry)
                            .await;
                }
                HandlerOutcome::Delay(duration) => {
                    let _ = Self::handle_delay_static(
                        &channel,
                        &result.delivery,
                        &config.retry,
                        duration,
                    )
                    .await;
                }
            }
        }
        info!("Safe consumer drain complete");

        if self.cancel_token.is_cancelled() {
            Ok(())
        } else {
            Err(ConsumerError::StreamEnded)
        }
    }

    async fn consume_loop_cron(
        &self,
        channel: &Channel,
        config: &CronConfig,
        handler: &impl Handler,
    ) -> Result<(), ConsumerError> {
        self.setup_cron_queues(channel, config).await?;

        let mut consumer = channel
            .basic_consume(
                ShortString::from(config.base.queue.as_str()),
                ShortString::from(config.base.consumer_tag.as_str()),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::ConsumerRegistration {
                consumer_tag: config.base.consumer_tag.clone(),
                source: e,
            })?;

        info!(
            exchange = %config.base.exchange,
            queue = %config.base.queue,
            interval_ms = %config.interval.as_millis(),
            "Cron consumer started (publish a message to start the cron job)"
        );

        while let Some(delivery_result) = consumer.next().await {
            let delivery = delivery_result.map_err(ConsumerError::Connection)?;

            let msg = Self::delivery_to_message(&delivery, &config.base.queue);
            match handler.call(&msg).await {
                Ok(_) => {
                    debug!("Cron handler executed successfully");
                }
                Err(err) => {
                    error!(?err, "Cron handler execution failed");
                }
            }

            delivery
                .nack(BasicNackOptions {
                    requeue: false,
                    ..Default::default()
                })
                .await
                .map_err(ConsumerError::Nack)?;
        }

        Err(ConsumerError::StreamEnded)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Private helpers — topology setup, retry/DLQ routing, message conversion
    // ═══════════════════════════════════════════════════════════════════════════

    async fn declare_exchange_if_needed(
        &self,
        channel: &Channel,
        exchange: &str,
    ) -> Result<(), ConsumerError> {
        channel
            .exchange_declare(
                ShortString::from(exchange),
                ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::ExchangeDeclaration {
                exchange: exchange.to_string(),
                source: e,
            })?;
        Ok(())
    }

    async fn ensure_retry_exchanges(
        &self,
        channel: &Channel,
        config: &RetryConfig,
    ) -> Result<(), ConsumerError> {
        self.declare_exchange_if_needed(channel, &config.base.exchange)
            .await?;
        self.declare_exchange_if_needed(channel, &config.retry_exchange)
            .await?;
        self.declare_exchange_if_needed(channel, &config.dead_exchange)
            .await?;
        Ok(())
    }

    async fn setup_retry_queues(
        &self,
        channel: &Channel,
        config: &RetryConfig,
    ) -> Result<(), ConsumerError> {
        self.ensure_retry_exchanges(channel, config).await?;
        let retry_routing_key = config.retry_routing_key();
        let dead_routing_key = config.dead_routing_key();

        let mut queue_args = FieldTable::default();
        queue_args.insert(
            "x-dead-letter-exchange".into(),
            AMQPValue::LongString(config.retry_exchange.clone().into()),
        );
        queue_args.insert(
            "x-dead-letter-routing-key".into(),
            AMQPValue::LongString(retry_routing_key.clone().into()),
        );

        channel
            .queue_declare(
                ShortString::from(config.base.queue.as_str()),
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                queue_args,
            )
            .await
            .map_err(|e| ConsumerError::QueueDeclaration {
                queue: config.base.queue.clone(),
                source: e,
            })?;

        channel
            .queue_bind(
                ShortString::from(config.base.queue.as_str()),
                ShortString::from(config.base.exchange.as_str()),
                ShortString::from(config.base.routing_key.as_str()),
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::QueueBinding {
                queue: config.base.queue.clone(),
                exchange: config.base.exchange.clone(),
                source: e,
            })?;

        let mut retry_args = FieldTable::default();
        retry_args.insert(
            "x-message-ttl".into(),
            AMQPValue::LongUInt(config.retry_delay.as_millis() as u32),
        );
        // Retry queue TTL should return strictly to the originating queue.
        // Using the default exchange avoids fanout through the shared main exchange.
        retry_args.insert(
            "x-dead-letter-exchange".into(),
            AMQPValue::LongString("".into()),
        );
        retry_args.insert(
            "x-dead-letter-routing-key".into(),
            AMQPValue::LongString(config.base.queue.clone().into()),
        );

        let retry_queue = format!("{}.retry", &config.base.queue);
        channel
            .queue_declare(
                ShortString::from(retry_queue.as_str()),
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                retry_args,
            )
            .await
            .map_err(|e| ConsumerError::QueueDeclaration {
                queue: retry_queue.clone(),
                source: e,
            })?;

        channel
            .queue_bind(
                ShortString::from(retry_queue.as_str()),
                ShortString::from(config.retry_exchange.as_str()),
                ShortString::from(retry_routing_key.as_str()),
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::QueueBinding {
                queue: retry_queue.clone(),
                exchange: config.retry_exchange.clone(),
                source: e,
            })?;

        let error_queue = format!("{}.error", &config.base.queue);
        channel
            .queue_declare(
                ShortString::from(error_queue.as_str()),
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::QueueDeclaration {
                queue: error_queue.clone(),
                source: e,
            })?;

        channel
            .queue_bind(
                ShortString::from(error_queue.as_str()),
                ShortString::from(config.dead_exchange.as_str()),
                ShortString::from(dead_routing_key.as_str()),
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::QueueBinding {
                queue: error_queue.clone(),
                exchange: config.dead_exchange.clone(),
                source: e,
            })?;

        Ok(())
    }

    async fn setup_cron_queues(
        &self,
        channel: &Channel,
        config: &CronConfig,
    ) -> Result<(), ConsumerError> {
        self.declare_exchange_if_needed(channel, &config.base.exchange)
            .await?;
        self.declare_exchange_if_needed(channel, &config.retry_exchange)
            .await?;
        let retry_routing_key = RetryConfig::retry_routing_key_for_queue(&config.base.queue);

        let mut queue_args = FieldTable::default();
        queue_args.insert(
            "x-dead-letter-exchange".into(),
            AMQPValue::LongString(config.retry_exchange.clone().into()),
        );
        queue_args.insert(
            "x-dead-letter-routing-key".into(),
            AMQPValue::LongString(retry_routing_key.clone().into()),
        );

        channel
            .queue_declare(
                ShortString::from(config.base.queue.as_str()),
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                queue_args,
            )
            .await
            .map_err(|e| ConsumerError::QueueDeclaration {
                queue: config.base.queue.clone(),
                source: e,
            })?;

        channel
            .queue_bind(
                ShortString::from(config.base.queue.as_str()),
                ShortString::from(config.base.exchange.as_str()),
                ShortString::from(config.base.routing_key.as_str()),
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::QueueBinding {
                queue: config.base.queue.clone(),
                exchange: config.base.exchange.clone(),
                source: e,
            })?;

        let mut cron_args = FieldTable::default();
        cron_args.insert(
            "x-message-ttl".into(),
            AMQPValue::LongUInt(config.interval.as_millis() as u32),
        );
        // Cron delay queue should route back only to the originating queue.
        cron_args.insert(
            "x-dead-letter-exchange".into(),
            AMQPValue::LongString("".into()),
        );
        cron_args.insert(
            "x-dead-letter-routing-key".into(),
            AMQPValue::LongString(config.base.queue.clone().into()),
        );

        let cron_queue = format!("{}.cron-job", &config.base.queue);
        channel
            .queue_declare(
                ShortString::from(cron_queue.as_str()),
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                cron_args,
            )
            .await
            .map_err(|e| ConsumerError::QueueDeclaration {
                queue: cron_queue.clone(),
                source: e,
            })?;

        channel
            .queue_bind(
                ShortString::from(cron_queue.as_str()),
                ShortString::from(config.retry_exchange.as_str()),
                ShortString::from(retry_routing_key.as_str()),
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::QueueBinding {
                queue: cron_queue.clone(),
                exchange: config.retry_exchange.clone(),
                source: e,
            })?;

        Ok(())
    }

    async fn handle_retry_static(
        channel: &Channel,
        delivery: &lapin::message::Delivery,
        config: &RetryConfig,
    ) -> Result<(), ConsumerError> {
        // Permanent retry budget is tracked via a dedicated header so transient
        // dead-letter cycles do not consume max_retries.
        let retry_count =
            Self::extract_permanent_retry_count(delivery.properties.headers().as_ref());

        if retry_count >= config.max_retries {
            error!(
                retry_count = %retry_count,
                max_retries = %config.max_retries,
                "Moving permanent failure to DLX after max retries"
            );

            let dead_routing_key = config.dead_routing_key();
            channel
                .basic_publish(
                    ShortString::from(config.dead_exchange.as_str()),
                    ShortString::from(dead_routing_key.as_str()),
                    BasicPublishOptions::default(),
                    &delivery.data,
                    delivery.properties.clone(),
                )
                .await
                .map_err(ConsumerError::DeadLetter)?;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .map_err(ConsumerError::Ack)?;
        } else {
            let next_retry_count = retry_count.saturating_add(1);
            warn!(
                retry_count = %retry_count,
                next_retry_count = %next_retry_count,
                "Sending permanent failure to retry queue"
            );

            let retry_routing_key = config.retry_routing_key();
            let mut headers = delivery
                .properties
                .headers()
                .as_ref()
                .cloned()
                .unwrap_or_default();
            headers.insert(
                ShortString::from(Self::PERMANENT_RETRY_COUNT_HEADER),
                AMQPValue::LongUInt(next_retry_count),
            );

            channel
                .basic_publish(
                    ShortString::from(config.retry_exchange.as_str()),
                    ShortString::from(retry_routing_key.as_str()),
                    BasicPublishOptions::default(),
                    &delivery.data,
                    delivery.properties.clone().with_headers(headers),
                )
                .await
                .map_err(ConsumerError::Retry)?;

            delivery
                .ack(BasicAckOptions::default())
                .await
                .map_err(ConsumerError::Ack)?;
        }

        Ok(())
    }

    async fn handle_transient_retry_static(
        delivery: &lapin::message::Delivery,
    ) -> Result<(), ConsumerError> {
        delivery
            .nack(BasicNackOptions {
                requeue: false,
                ..Default::default()
            })
            .await
            .map_err(ConsumerError::Nack)?;
        Ok(())
    }

    async fn handle_dead_letter_static(
        channel: &Channel,
        delivery: &lapin::message::Delivery,
        config: &RetryConfig,
    ) -> Result<(), ConsumerError> {
        let dead_routing_key = config.dead_routing_key();
        channel
            .basic_publish(
                ShortString::from(config.dead_exchange.as_str()),
                ShortString::from(dead_routing_key.as_str()),
                BasicPublishOptions::default(),
                &delivery.data,
                delivery.properties.clone(),
            )
            .await
            .map_err(ConsumerError::DeadLetter)?;

        delivery
            .ack(BasicAckOptions::default())
            .await
            .map_err(ConsumerError::Ack)?;
        Ok(())
    }

    async fn handle_delay_static(
        channel: &Channel,
        delivery: &lapin::message::Delivery,
        config: &RetryConfig,
        duration: Duration,
    ) -> Result<(), ConsumerError> {
        let retry_routing_key = config.retry_routing_key();
        let expiration_ms = duration.as_millis().to_string();
        channel
            .basic_publish(
                ShortString::from(config.retry_exchange.as_str()),
                ShortString::from(retry_routing_key.as_str()),
                BasicPublishOptions::default(),
                &delivery.data,
                delivery
                    .properties
                    .clone()
                    .with_expiration(expiration_ms.into()),
            )
            .await
            .map_err(ConsumerError::Retry)?;

        delivery
            .ack(BasicAckOptions::default())
            .await
            .map_err(ConsumerError::Ack)?;
        Ok(())
    }

    fn delivery_to_message(delivery: &Delivery, queue: &str) -> Message {
        let retry_count = Self::extract_retry_count(delivery.properties.headers().as_ref());
        Message {
            payload: delivery.data.clone(),
            metadata: MessageMetadata::new(
                delivery.delivery_tag.to_string(),
                queue,
                retry_count as u64 + 1,
            ),
        }
    }

    fn extract_retry_count(headers: Option<&FieldTable>) -> u32 {
        headers
            .and_then(|hdrs| hdrs.inner().get("x-death"))
            .and_then(|x_death| match x_death {
                AMQPValue::FieldArray(array) => array.as_slice().first(),
                _ => None,
            })
            .and_then(|first_entry| match first_entry {
                AMQPValue::FieldTable(table) => table
                    .inner()
                    .get(&ShortString::from("count"))
                    .and_then(Self::parse_u32_header_value),
                _ => None,
            })
            .unwrap_or(0)
    }

    fn extract_permanent_retry_count(headers: Option<&FieldTable>) -> u32 {
        headers
            .and_then(|hdrs| hdrs.inner().get(Self::PERMANENT_RETRY_COUNT_HEADER))
            .and_then(Self::parse_u32_header_value)
            .unwrap_or(0)
    }

    fn parse_u32_header_value(value: &AMQPValue) -> Option<u32> {
        match value {
            AMQPValue::LongUInt(n) => Some(*n),
            AMQPValue::LongLongInt(n) => Some(*n as u32),
            AMQPValue::LongInt(n) => Some(*n as u32),
            _ => None,
        }
    }
}

#[async_trait::async_trait]
impl crate::traits::consumer::Consumer for RmqConsumer {
    type PrefetchConfig = PrefetchConfig;
    type Error = ConsumerError;

    async fn connect(url: &str) -> Result<Self, Self::Error> {
        Ok(Self::with_addr(url))
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
    use lapin::types::FieldArray;

    #[test]
    fn test_extract_retry_count_empty() {
        let count = RmqConsumer::extract_retry_count(None);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_extract_retry_count_empty_headers() {
        let headers = FieldTable::default();
        let count = RmqConsumer::extract_retry_count(Some(&headers));
        assert_eq!(count, 0);
    }

    #[test]
    fn test_extract_retry_count_with_value() {
        let mut inner_table = FieldTable::default();
        inner_table.insert(ShortString::from("count"), AMQPValue::LongUInt(3));

        let array = FieldArray::from(vec![AMQPValue::FieldTable(inner_table)]);

        let mut headers = FieldTable::default();
        headers.insert(ShortString::from("x-death"), AMQPValue::FieldArray(array));

        let count = RmqConsumer::extract_retry_count(Some(&headers));
        assert_eq!(count, 3);
    }

    #[test]
    fn test_extract_retry_count_with_long_long_int() {
        let mut inner_table = FieldTable::default();
        inner_table.insert(ShortString::from("count"), AMQPValue::LongLongInt(5));

        let array = FieldArray::from(vec![AMQPValue::FieldTable(inner_table)]);

        let mut headers = FieldTable::default();
        headers.insert(ShortString::from("x-death"), AMQPValue::FieldArray(array));

        let count = RmqConsumer::extract_retry_count(Some(&headers));
        assert_eq!(count, 5);
    }

    #[test]
    fn test_extract_permanent_retry_count_empty() {
        let count = RmqConsumer::extract_permanent_retry_count(None);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_extract_permanent_retry_count_with_value() {
        let mut headers = FieldTable::default();
        headers.insert(
            ShortString::from(RmqConsumer::PERMANENT_RETRY_COUNT_HEADER),
            AMQPValue::LongUInt(4),
        );

        let count = RmqConsumer::extract_permanent_retry_count(Some(&headers));
        assert_eq!(count, 4);
    }
}
