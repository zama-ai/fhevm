//! ConsumerBuilder - fluent API for configuring consumers.

use std::sync::Arc;
use std::time::Duration;

use tokio_util::sync::CancellationToken;

#[cfg(feature = "amqp")]
use crate::amqp::{ConnectionManager as AmqpConnectionManager, ConsumerConfigBuilder, RmqConsumer};
#[cfg(feature = "redis")]
use crate::redis::{
    RedisConnectionManager, RedisConsumer, RedisConsumerConfigBuilder, StreamTopology,
};
use crate::traits::Handler;

#[cfg(feature = "amqp")]
use crate::AmqpInfraTopology;
#[cfg(feature = "amqp")]
use crate::config::AmqpOptions;
use crate::config::ConsumerConfig;
#[cfg(feature = "redis")]
use crate::config::RedisOptions;
use crate::error::BrokerError;
use crate::topic::Topic;

/// Consumer builder with fluent API.
///
/// # Examples
///
/// ```ignore
/// // Simple case
/// broker.consumer(&topic)
///     .group("indexer")
///     .run(handler).await?;
///
/// // Full configuration
/// broker.consumer(&topic)
///     .group("indexer")
///     .consumer_name("pod-1")
///     .prefetch(100)
///     .max_retries(5)
///     .circuit_breaker(3, Duration::from_secs(30))
///     .redis_block_ms(5000)
///     .amqp_retry_delay(10)
///     .run(handler).await?;
/// ```
#[must_use]
pub struct ConsumerBuilder<'a> {
    backend: ConsumerBackend<'a>,
    topic: Topic,
    config: ConsumerConfig,
    cancellation_token: Option<CancellationToken>,
    #[cfg(feature = "redis")]
    redis_opts: RedisOptions,
    #[cfg(feature = "amqp")]
    amqp_opts: AmqpOptions,
}

pub(crate) enum ConsumerBackend<'a> {
    #[cfg(feature = "redis")]
    Redis(&'a Arc<RedisConnectionManager>),
    #[cfg(feature = "amqp")]
    Amqp(&'a Arc<AmqpConnectionManager>, AmqpInfraTopology),
}

impl<'a> ConsumerBuilder<'a> {
    /// Create a new consumer builder (called by `Broker::consumer()`).
    pub(crate) fn new(backend: ConsumerBackend<'a>, topic: Topic) -> Self {
        Self {
            backend,
            topic,
            config: ConsumerConfig::default(),
            cancellation_token: None,
            #[cfg(feature = "redis")]
            redis_opts: RedisOptions::default(),
            #[cfg(feature = "amqp")]
            amqp_opts: AmqpOptions::default(),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Required
    // ═══════════════════════════════════════════════════════════════════════════

    /// Set the consumer group name (required).
    ///
    /// - **Redis**: Consumer group name
    /// - **AMQP**: Logical group name (queue defaults to `{namespace}.{group}`
    ///   when topic is namespaced, otherwise `{group}`)
    pub fn group(mut self, name: impl Into<String>) -> Self {
        self.config.group = name.into();
        self
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Optional common settings
    // ═══════════════════════════════════════════════════════════════════════════

    /// Set the consumer name within the group.
    ///
    /// - **Redis**: Consumer name (XREADGROUP GROUP {group} {consumer})
    /// - **AMQP**: Consumer tag
    ///
    /// If not set, a unique name is auto-generated.
    pub fn consumer_name(mut self, name: impl Into<String>) -> Self {
        self.config.consumer_name = Some(name.into());
        self
    }

    /// Set the prefetch count (messages to buffer).
    ///
    /// - **Redis**: `read_count` for XREADGROUP
    /// - **AMQP**: QoS prefetch count
    ///
    /// Default: 10
    pub fn prefetch(mut self, n: usize) -> Self {
        self.config.prefetch = n;
        self
    }

    /// Set max retries before dead-letter.
    ///
    /// Default: 3
    pub fn max_retries(mut self, n: u32) -> Self {
        self.config.max_retries = n;
        self
    }

    /// Set circuit breaker configuration.
    ///
    /// The circuit breaker pauses consumption when `threshold` consecutive
    /// transient errors occur. After `cooldown`, one probe message is processed.
    pub fn circuit_breaker(mut self, threshold: u32, cooldown: Duration) -> Self {
        self.config.circuit_breaker = Some((threshold, cooldown));
        self
    }

    /// Set a cancellation token for graceful shutdown.
    ///
    /// When the token is cancelled, the consumer stops after finishing its
    /// current message batch.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use tokio_util::sync::CancellationToken;
    ///
    /// let token = CancellationToken::new();
    /// let consumer = broker.consumer(&topic)
    ///     .group("indexer")
    ///     .with_cancellation(token.clone())
    ///     .build()?;
    ///
    /// // Later, to stop the consumer:
    /// token.cancel();
    /// ```
    pub fn with_cancellation(mut self, token: CancellationToken) -> Self {
        self.cancellation_token = Some(token);
        self
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Backend-specific (silently ignored on wrong backend)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Redis: Block time in milliseconds for XREADGROUP.
    ///
    /// **Silently ignored on AMQP.**
    #[cfg(feature = "redis")]
    pub fn redis_block_ms(mut self, ms: usize) -> Self {
        self.redis_opts.block_ms = Some(ms);
        self
    }

    /// Redis: Minimum idle time (seconds) before claiming a pending message.
    ///
    /// **Silently ignored on AMQP.**
    #[cfg(feature = "redis")]
    pub fn redis_claim_min_idle(mut self, secs: u64) -> Self {
        self.redis_opts.claim_min_idle_secs = Some(secs);
        self
    }

    /// Redis: Interval (seconds) between claim sweep cycles.
    ///
    /// **Silently ignored on AMQP.**
    #[cfg(feature = "redis")]
    pub fn redis_claim_interval(mut self, secs: u64) -> Self {
        self.redis_opts.claim_interval_secs = Some(secs);
        self
    }

    /// AMQP: Retry delay in seconds.
    ///
    /// **Silently ignored on Redis.**
    #[cfg(feature = "amqp")]
    pub fn amqp_retry_delay(mut self, secs: u64) -> Self {
        self.amqp_opts.retry_delay_secs = Some(secs);
        self
    }

    /// AMQP: Override the routing key for queue binding (e.g., "ethereum.blocks.#").
    ///
    /// **Silently ignored on Redis.**
    #[cfg(feature = "amqp")]
    pub fn amqp_routing_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.amqp_opts.routing_key_pattern = Some(pattern.into());
        self
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Terminal methods
    // ═══════════════════════════════════════════════════════════════════════════

    /// Build the consumer without running (for later execution).
    pub fn build(mut self) -> Result<Consumer, BrokerError> {
        if self.config.group.is_empty() {
            return Err(BrokerError::MissingGroup);
        }

        let consumer_name = self
            .config
            .consumer_name
            .clone()
            .unwrap_or_else(|| format!("{}-{}", self.config.group, generate_id()));

        // Take the token before borrowing self in match arms.
        let cancellation_token = self.cancellation_token.take();

        match self.backend {
            #[cfg(feature = "redis")]
            ConsumerBackend::Redis(conn) => {
                let mut inner = RedisConsumer::new(Arc::clone(conn));
                if let Some(token) = cancellation_token {
                    inner = inner.with_cancellation(token);
                }
                let config = self.build_redis_config(&consumer_name)?;
                Ok(Consumer {
                    inner: ConsumerInner::Redis {
                        consumer: inner,
                        config,
                    },
                })
            }
            #[cfg(feature = "amqp")]
            ConsumerBackend::Amqp(conn, ref topology) => {
                let mut inner = RmqConsumer::new(Arc::clone(conn));
                if let Some(token) = cancellation_token {
                    inner = inner.with_cancellation(token);
                }
                let config = self.build_amqp_config(&consumer_name, topology.clone())?;
                Ok(Consumer {
                    inner: ConsumerInner::Amqp {
                        consumer: inner,
                        config,
                    },
                })
            }
        }
    }

    /// Build and run immediately (convenience method).
    pub async fn run<H: Handler + Clone + 'static>(self, handler: H) -> Result<(), BrokerError> {
        let consumer = self.build()?;
        consumer.run(handler).await
    }

    #[cfg(feature = "redis")]
    fn build_redis_config(
        &self,
        consumer_name: &str,
    ) -> Result<crate::redis::RedisPrefetchConfig, BrokerError> {
        let stream_topology = StreamTopology::new(self.topic.key(), self.topic.dead_key());
        let mut builder = RedisConsumerConfigBuilder::new()
            .with_topology(&stream_topology)
            .group_name(&self.config.group)
            .consumer_name(consumer_name)
            .max_retries(self.config.max_retries)
            .prefetch_count(self.config.prefetch);

        if let Some((threshold, cooldown)) = self.config.circuit_breaker {
            builder = builder
                .circuit_breaker_threshold(threshold)
                .circuit_breaker_cooldown(cooldown);
        }

        if let Some(block_ms) = self.redis_opts.block_ms {
            builder = builder.block_ms(block_ms);
        }
        if let Some(claim_min_idle) = self.redis_opts.claim_min_idle_secs {
            builder = builder.claim_min_idle(Duration::from_secs(claim_min_idle));
        }
        if let Some(claim_interval) = self.redis_opts.claim_interval_secs {
            builder = builder.claim_interval(Duration::from_secs(claim_interval));
        }

        Ok(builder.build_prefetch()?)
    }

    #[cfg(feature = "amqp")]
    fn build_amqp_config(
        &self,
        consumer_name: &str,
        topology: AmqpInfraTopology,
    ) -> Result<crate::amqp::PrefetchConfig, BrokerError> {
        let queue_name = self.resolve_amqp_queue_name();
        let routing_key: String = self
            .amqp_opts
            .routing_key_pattern
            .clone()
            .unwrap_or_else(|| self.topic.key());

        let mut builder = ConsumerConfigBuilder::new()
            .with_topology(&topology)
            .queue(&queue_name)
            .routing_key(&routing_key)
            .consumer_tag(consumer_name)
            .max_retries(self.config.max_retries)
            .prefetch_count(self.config.prefetch as u16);

        if let Some((threshold, cooldown)) = self.config.circuit_breaker {
            builder = builder
                .circuit_breaker_threshold(threshold)
                .circuit_breaker_cooldown(cooldown);
        }

        if let Some(retry_delay) = self.amqp_opts.retry_delay_secs {
            builder = builder.retry_delay(Duration::from_secs(retry_delay));
        }

        Ok(builder.build_prefetch()?)
    }

    #[cfg(feature = "amqp")]
    fn resolve_amqp_queue_name(&self) -> String {
        let Some(namespace) = self.topic.namespace() else {
            return self.config.group.clone();
        };

        if self.config.group.starts_with(namespace)
            && self.config.group.as_bytes().get(namespace.len()) == Some(&b'.')
        {
            self.config.group.clone()
        } else {
            format!("{namespace}.{}", self.config.group)
        }
    }
}

/// A configured consumer ready to run.
pub struct Consumer {
    inner: ConsumerInner,
}

enum ConsumerInner {
    #[cfg(feature = "redis")]
    Redis {
        consumer: RedisConsumer,
        config: crate::redis::RedisPrefetchConfig,
    },
    #[cfg(feature = "amqp")]
    Amqp {
        consumer: RmqConsumer,
        config: crate::amqp::PrefetchConfig,
    },
}

impl Consumer {
    /// Ensure the underlying topology (exchanges, queues, bindings) exists
    /// without starting to consume.
    ///
    /// For AMQP: declares exchanges, queues, and bindings so that messages
    /// published to the exchange are routed correctly. This **must** be called
    /// before checking queue depth (`is_empty`) or publishing seed messages,
    /// otherwise AMQP silently drops messages with no bound queue.
    ///
    /// For Redis: no-op (streams are auto-created on first XADD).
    pub async fn ensure_topology(&self) -> Result<(), BrokerError> {
        match &self.inner {
            #[cfg(feature = "redis")]
            ConsumerInner::Redis { .. } => Ok(()),
            #[cfg(feature = "amqp")]
            ConsumerInner::Amqp { consumer, config } => {
                consumer.ensure_topology(config).await?;
                Ok(())
            }
        }
    }

    /// Run the consumer with the given handler (blocking, long-running).
    ///
    /// If a cancellation token was provided via [`ConsumerBuilder::with_cancellation`],
    /// the consumer stops after finishing its current batch and draining in-flight results.
    pub async fn run<H: Handler + Clone + 'static>(self, handler: H) -> Result<(), BrokerError> {
        match self.inner {
            #[cfg(feature = "redis")]
            ConsumerInner::Redis { consumer, config } => {
                consumer.run(config, handler).await?;
            }
            #[cfg(feature = "amqp")]
            ConsumerInner::Amqp { consumer, config } => {
                consumer.run(config, handler).await?;
            }
        }
        Ok(())
    }
}

/// Generate a simple unique ID for consumer names.
fn generate_id() -> String {
    uuid::Uuid::now_v7().to_string()
}

#[cfg(test)]
#[cfg(feature = "amqp")]
mod tests {
    use super::*;
    use crate::{
        AMQP_DEFAULT_DLX_EXCHANGE, AMQP_DEFAULT_MAIN_EXCHANGE, AMQP_DEFAULT_RETRY_EXCHANGE,
    };

    fn amqp_builder<'a>(conn: &'a Arc<AmqpConnectionManager>, group: &str) -> ConsumerBuilder<'a> {
        ConsumerBuilder::new(
            ConsumerBackend::Amqp(conn, crate::default_amqp_infra_topology()),
            Topic::new("blocks").with_namespace("ethereum"),
        )
        .group(group)
    }

    #[test]
    fn amqp_queue_name_defaults_to_namespace_group() {
        let conn = Arc::new(AmqpConnectionManager::new(
            "amqp://guest:guest@localhost:5672",
        ));
        let builder = amqp_builder(&conn, "indexer");
        assert_eq!(builder.resolve_amqp_queue_name(), "ethereum.indexer");
    }

    #[test]
    fn amqp_queue_name_keeps_prequalified_group() {
        let conn = Arc::new(AmqpConnectionManager::new(
            "amqp://guest:guest@localhost:5672",
        ));
        let builder = amqp_builder(&conn, "ethereum.indexer");
        assert_eq!(builder.resolve_amqp_queue_name(), "ethereum.indexer");
    }

    #[test]
    fn amqp_queue_name_without_namespace_uses_group_as_is() {
        let conn = Arc::new(AmqpConnectionManager::new(
            "amqp://guest:guest@localhost:5672",
        ));
        let builder = ConsumerBuilder::new(
            ConsumerBackend::Amqp(&conn, crate::default_amqp_infra_topology()),
            Topic::new("blocks"),
        )
        .group("indexer");

        assert_eq!(builder.resolve_amqp_queue_name(), "indexer");
    }

    #[test]
    fn amqp_config_uses_global_default_topology_when_not_overridden() {
        let conn = Arc::new(AmqpConnectionManager::new(
            "amqp://guest:guest@localhost:5672",
        ));
        let builder = amqp_builder(&conn, "indexer");
        let topology = match &builder.backend {
            ConsumerBackend::Amqp(_, t) => t.clone(),
            _ => panic!("expected AMQP backend"),
        };
        let config = builder
            .build_amqp_config("pod-1", topology)
            .expect("amqp config should build");

        assert_eq!(config.retry.base.exchange, AMQP_DEFAULT_MAIN_EXCHANGE);
        assert_eq!(config.retry.retry_exchange, AMQP_DEFAULT_RETRY_EXCHANGE);
        assert_eq!(config.retry.dead_exchange, AMQP_DEFAULT_DLX_EXCHANGE);
        assert_eq!(config.retry.base.queue, "ethereum.indexer");
        assert_eq!(config.retry.base.routing_key, "ethereum.blocks");
    }
}
