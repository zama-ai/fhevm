use lapin::{
    BasicProperties, Channel, Confirmation,
    options::{BasicPublishOptions, ConfirmSelectOptions},
};
use serde::Serialize;
use std::{fmt::Debug, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::{sync::RwLock, time::sleep};
use tracing::{info, warn};

use super::connection::ConnectionManager;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PublisherError {
    #[error("serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("channel error: {0}")]
    Channel(#[from] lapin::Error),

    #[error("publish was nacked by broker")]
    Nacked,

    #[error(
        "exchange not configured: use for_exchange() or connect() before publishing via the Publisher trait"
    )]
    ExchangeNotConfigured,

    #[error(
        "publish failed after {retries} retries (addr: {addr}, exchange: {exchange}, routing_key: {routing_key})"
    )]
    RetriesExhausted {
        retries: u32,
        addr: String,
        exchange: String,
        routing_key: String,
    },
}

pub struct RmqPublisher {
    connection: Arc<ConnectionManager>,
    channel: RwLock<Channel>,
    max_retries: u32,
    base_retry_delay_ms: u64,
    /// Exchange fixed at construction.
    /// Required for the `mq::Publisher` trait: `topic` maps to the AMQP routing key
    /// and the exchange is determined here. Set via `for_exchange()`.
    exchange: Option<String>,
    /// When true, publisher confirms are enabled on the channel (`confirm_select`).
    /// The broker sends `basic.ack`/`basic.nack` for each publish.
    confirms_enabled: bool,
}

impl RmqPublisher {
    /// Create a publisher using a shared connection manager.
    pub async fn new(connection: Arc<ConnectionManager>) -> Self {
        Self::with_config(connection, 10, 1000).await
    }

    /// Create a publisher with custom retry settings using a shared connection manager.
    pub async fn with_config(
        connection: Arc<ConnectionManager>,
        max_retries: u32,
        base_retry_delay_ms: u64,
    ) -> Self {
        let channel = connection.create_channel_with_retry().await;
        info!("RmqPublisher connected successfully");
        Self {
            connection,
            channel: RwLock::new(channel),
            max_retries,
            base_retry_delay_ms,
            exchange: None,
            confirms_enabled: false,
        }
    }

    /// Create a publisher bound to a specific exchange using a shared connection manager.
    ///
    /// This is required for using the queue-agnostic `Publisher` trait where
    /// `topic` maps to the AMQP routing key and the exchange is fixed.
    pub async fn for_exchange(connection: Arc<ConnectionManager>, exchange: &str) -> Self {
        let mut publisher = Self::new(connection).await;
        publisher.exchange = Some(exchange.to_string());
        publisher
    }

    /// Create a publisher bound to an exchange with publisher confirms enabled.
    ///
    /// Calls `confirm_select()` on the channel so that every `basic_publish`
    /// returns a meaningful `Confirmation::Ack` or `Confirmation::Nack` from
    /// the broker, rather than `Confirmation::NotRequested`.
    pub async fn for_exchange_with_confirms(
        connection: Arc<ConnectionManager>,
        exchange: &str,
    ) -> Result<Self, PublisherError> {
        let channel = connection.create_channel_with_retry().await;
        channel
            .confirm_select(ConfirmSelectOptions::default())
            .await
            .map_err(PublisherError::Channel)?;
        info!("RmqPublisher connected with publisher confirms enabled");
        Ok(Self {
            connection,
            channel: RwLock::new(channel),
            max_retries: 10,
            base_retry_delay_ms: 1000,
            exchange: Some(exchange.to_string()),
            confirms_enabled: true,
        })
    }

    /// Convenience: create a publisher with its own connection, bound to an exchange.
    ///
    /// Each call creates a new TCP connection. Prefer `for_exchange()` with a shared
    /// `Arc<ConnectionManager>` in production to multiplex channels over one connection.
    pub async fn connect(addr: &str, exchange: &str) -> Self {
        let connection = Arc::new(ConnectionManager::new(addr));
        Self::for_exchange(connection, exchange).await
    }

    async fn reconnect(&self) {
        // Re-enable confirms on the new channel — confirm mode is per-channel, not per-connection.
        // Retry with fresh channels if confirm_select fails (bounded: 3 attempts).
        // If all attempts fail, install the channel anyway — the publish path's
        // NotRequested handler will detect the missing confirms and trigger another retry.
        let mut new_channel = self.connection.create_channel_with_retry().await;
        if self.confirms_enabled {
            for attempt in 0..3u32 {
                match new_channel
                    .confirm_select(ConfirmSelectOptions::default())
                    .await
                {
                    Ok(_) => break,
                    Err(e) => {
                        warn!(
                            error = %e,
                            attempt = attempt + 1,
                            "Failed to enable publisher confirms on new channel"
                        );
                        if attempt < 2 {
                            new_channel = self.connection.create_channel_with_retry().await;
                        }
                    }
                }
            }
        }
        let mut channel = self.channel.write().await;
        *channel = new_channel;
    }

    fn is_channel_healthy(channel: &Channel) -> bool {
        channel.status().connected()
    }

    async fn publish_amqp<T: Serialize + Debug>(
        &self,
        exchange: &str,
        routing_key: &str,
        payload: &T,
    ) -> Result<(), PublisherError> {
        let body = serde_json::to_vec(payload)?;
        let publish_start = std::time::Instant::now();

        for attempt in 0..self.max_retries {
            {
                let channel = self.channel.read().await;
                if !Self::is_channel_healthy(&channel) {
                    drop(channel);
                    warn!("Channel unhealthy, attempting reconnect...");
                    self.reconnect().await;
                }
            }

            let result = {
                let channel = self.channel.read().await;
                channel
                    .basic_publish(
                        exchange.into(),
                        routing_key.into(),
                        BasicPublishOptions::default(),
                        &body,
                        BasicProperties::default().with_delivery_mode(2),
                    )
                    .await
            };

            match result {
                Ok(confirm) => match confirm.await {
                    Ok(Confirmation::Ack(_)) => {
                        metrics::counter!("broker_messages_published_total",
                            "backend" => "amqp", "topic" => routing_key.to_owned()
                        )
                        .increment(1);
                        metrics::histogram!("broker_publish_duration_seconds",
                            "backend" => "amqp", "topic" => routing_key.to_owned()
                        )
                        .record(publish_start.elapsed().as_secs_f64());
                        return Ok(());
                    }
                    Ok(Confirmation::Nack(_)) => {
                        metrics::counter!("broker_publish_errors_total",
                            "backend" => "amqp", "topic" => routing_key.to_owned(), "error_kind" => "nacked"
                        ).increment(1);
                        warn!("Publish nacked by broker (attempt {})", attempt + 1);
                    }
                    Ok(Confirmation::NotRequested) => {
                        if self.confirms_enabled {
                            // confirm_select was lost (likely after reconnect failure).
                            // Do NOT return Ok — fall through to retry with reconnect,
                            // which will attempt to re-enable confirms.
                            warn!(
                                "Confirmation::NotRequested despite confirms enabled \
                                 (attempt {}) — will retry with reconnect",
                                attempt + 1
                            );
                        } else {
                            metrics::counter!("broker_messages_published_total",
                                "backend" => "amqp", "topic" => routing_key.to_owned()
                            )
                            .increment(1);
                            metrics::histogram!("broker_publish_duration_seconds",
                                "backend" => "amqp", "topic" => routing_key.to_owned()
                            )
                            .record(publish_start.elapsed().as_secs_f64());
                            return Ok(());
                        }
                    }
                    Err(e) => {
                        metrics::counter!("broker_publish_errors_total",
                            "backend" => "amqp", "topic" => routing_key.to_owned(), "error_kind" => "connection"
                        ).increment(1);
                        warn!("Confirm error (attempt {}): {}", attempt + 1, e);
                    }
                },
                Err(e) => {
                    metrics::counter!("broker_publish_errors_total",
                        "backend" => "amqp", "topic" => routing_key.to_owned(), "error_kind" => "connection"
                    ).increment(1);
                    warn!("Publish failed (attempt {}): {}", attempt + 1, e);
                }
            }

            let delay = self
                .base_retry_delay_ms
                .saturating_mul(2u64.pow(attempt))
                .min(30_000);
            sleep(Duration::from_millis(delay)).await;

            self.reconnect().await;
        }

        Err(PublisherError::RetriesExhausted {
            retries: self.max_retries,
            addr: self.connection.addr().to_string(),
            exchange: exchange.to_string(),
            routing_key: routing_key.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl crate::traits::publisher::Publisher for RmqPublisher {
    type Error = PublisherError;

    /// Publish to the fixed exchange — `topic` is the AMQP routing key.
    ///
    /// Requires the publisher to be created via `for_exchange()`.
    /// The exchange is fixed at construction; the topic string becomes the routing key,
    /// matching the RFC topology (`{chain}.events.blocks`, `request.{watch_id}`, etc.).
    async fn publish<T: Serialize + Debug + Send + Sync>(
        &self,
        topic: &str,
        payload: &T,
    ) -> Result<(), Self::Error> {
        let exchange = self
            .exchange
            .as_deref()
            .ok_or(PublisherError::ExchangeNotConfigured)?;
        self.publish_amqp(exchange, topic, payload).await
    }
}
