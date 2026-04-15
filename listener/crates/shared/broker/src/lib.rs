//! broker - A unified message broker abstraction for Redis Streams and RabbitMQ.
//!
//! This crate provides a unified interface for message brokers with support for:
//! - **Direct routing**: Messages to specific consumers
//! - **Fanout**: Broadcast to multiple consumer groups
//! - **Competing consumers**: Load balancing within a group
//! - **Circuit breaker**: Pause consumption during downstream outages (see [`CircuitBreakerConfig`])
//! - **Transient vs permanent errors**: [`HandlerError::transient()`] / [`HandlerError::permanent()`] drive retry + circuit breaker behavior
//!
//! # Routing Model
//!
//! ```text
//! Namespace = "ethereum"
//! Routing   = "blocks"
//!
//! Redis:
//!   stream = "ethereum.blocks"
//!
//! RabbitMQ (shared exchange topology):
//!   exchange = "main"               (configured once at broker level)
//!   routing  = "ethereum.blocks"    (namespace-qualified)
//! ```
//!
//! # Quick Start
//!
//! ```ignore
//! use broker::{Broker, Topic, routing, AsyncHandlerPayloadOnly};
//!
//! // Connect to broker
//! let broker = Broker::redis("redis://localhost:6379").await?;
//! // Or: Broker::amqp("amqp://localhost:5672").build().await?;
//!
//! // Publisher
//! let publisher = broker.publisher("ethereum").await?;
//! publisher.publish("blocks", &block_event).await?;
//!
//! // Consumer
//! let topic = Topic::new(routing::BLOCKS).with_namespace("ethereum");
//! broker.consumer(&topic)
//!     .group("indexer")
//!     .consumer_name("pod-1")
//!     .prefetch(100)
//!     .run(handler).await?;
//! ```

#![deny(clippy::correctness)]
#![warn(clippy::suspicious, clippy::style, clippy::complexity, clippy::perf)]

// Core trait definitions (absorbed from the `mq` crate)
pub mod traits;

// Backend implementations (feature-gated)
#[cfg(feature = "amqp")]
pub mod amqp;
#[cfg(feature = "redis")]
pub mod redis;

// Metrics
pub mod metrics;

// Facade modules
mod config;
mod consumer;
mod error;
mod publisher;
mod topic;

use std::sync::Arc;
#[cfg(feature = "redis")]
use std::time::Duration;

#[cfg(not(any(feature = "redis", feature = "amqp")))]
compile_error!("At least one broker backend must be enabled: \"redis\" or \"amqp\"");

#[cfg(feature = "amqp")]
use amqp::{
    AmqpQueueInspector, ConnectionManager as AmqpConnectionManager, ExchangeManager, RmqPublisher,
};
#[cfg(feature = "redis")]
use redis::{RedisConnectionManager, RedisPublisher, RedisQueueInspector};
use traits::depth::QueueInspector;
use traits::publisher::DynPublisher;

/// Default shared AMQP main exchange for broker-level global topology.
#[cfg(feature = "amqp")]
const AMQP_DEFAULT_MAIN_EXCHANGE: &str = "main";
/// Default shared AMQP retry exchange for broker-level global topology.
#[cfg(feature = "amqp")]
const AMQP_DEFAULT_RETRY_EXCHANGE: &str = "retry";
/// Default shared AMQP dead-letter exchange for broker-level global topology.
#[cfg(feature = "amqp")]
const AMQP_DEFAULT_DLX_EXCHANGE: &str = "dlx";

#[cfg(feature = "amqp")]
type AmqpInfraTopology = amqp::ExchangeTopology;

// Re-export main types
#[cfg(feature = "amqp")]
pub use amqp::ExchangeTopology;
#[cfg(feature = "amqp")]
pub use config::AmqpOptions;
pub use config::ConsumerConfig;
#[cfg(feature = "redis")]
pub use config::RedisOptions;
pub use consumer::{Consumer, ConsumerBuilder};
pub use error::BrokerError;
pub use publisher::Publisher;
pub use tokio_util::sync::CancellationToken;
pub use topic::Topic;

// Re-export commonly used types from traits module
pub use traits::{
    AckDecision, AsyncHandlerNoArgs, AsyncHandlerPayloadClassified, AsyncHandlerPayloadOnly,
    AsyncHandlerWithContext, CircuitBreakerConfig, Handler, HandlerError, Message, MessageMetadata,
    QueueDepths,
};

#[cfg(feature = "amqp")]
pub(crate) fn default_amqp_infra_topology() -> AmqpInfraTopology {
    AmqpInfraTopology::new(
        AMQP_DEFAULT_MAIN_EXCHANGE,
        AMQP_DEFAULT_RETRY_EXCHANGE,
        AMQP_DEFAULT_DLX_EXCHANGE,
    )
}

/// Message broker supporting Redis Streams and RabbitMQ.
///
/// # Examples
///
/// ```ignore
/// use broker::Broker;
///
/// // Create broker from environment
/// let broker = match std::env::var("BROKER_TYPE").as_deref() {
///     Ok("redis") => Broker::redis("redis://localhost:6379").await?,
///     Ok("amqp") => Broker::amqp("amqp://localhost:5672").build().await?,
///     _ => panic!("Unknown broker type"),
/// };
///
/// // Get publisher
/// let publisher = broker.publisher("ethereum").await?;
///
/// // Create consumer
/// let topic = Topic::new("blocks").with_namespace("ethereum");
/// broker.consumer(&topic).group("indexer").run(handler).await?;
/// ```
#[derive(Clone)]
pub enum Broker {
    /// Redis Streams backend.
    #[cfg(feature = "redis")]
    Redis {
        /// Shared connection manager.
        conn: Arc<RedisConnectionManager>,
        /// When true, publisher issues `WAIT` after each `XADD` for replication durability.
        ensure_publish: bool,
    },
    /// RabbitMQ backend.
    #[cfg(feature = "amqp")]
    Amqp {
        /// Shared connection manager.
        conn: Arc<AmqpConnectionManager>,
        /// Active AMQP topology used by publisher/consumer.
        /// Defaults to shared global topology (`main`, `retry`, `dlx`).
        amqp_infra_topology: Arc<AmqpInfraTopology>,
        /// When true, publisher enables `confirm_select` for publisher confirms.
        ensure_publish: bool,
    },
}

impl Broker {
    // ═══════════════════════════════════════════════════════════════════════════
    // Constructors
    // ═══════════════════════════════════════════════════════════════════════════

    /// Create a Redis broker from URL.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let broker = Broker::redis("redis://localhost:6379").await?;
    /// ```
    #[cfg(feature = "redis")]
    pub async fn redis(url: &str) -> Result<Self, BrokerError> {
        let conn = RedisConnectionManager::new_with_retry(url).await;
        Ok(Self::Redis {
            conn: Arc::new(conn),
            ensure_publish: false,
        })
    }

    /// Create a Redis broker from URL with `ensure_publish` replication durability.
    ///
    /// When `ensure_publish` is true, the publisher issues `WAIT 1 500` after
    /// each `XADD` to confirm replication to at least one replica.
    #[cfg(feature = "redis")]
    pub async fn redis_with_ensure_publish(
        url: &str,
        ensure_publish: bool,
    ) -> Result<Self, BrokerError> {
        let conn = RedisConnectionManager::new_with_retry(url).await;
        Ok(Self::Redis {
            conn: Arc::new(conn),
            ensure_publish,
        })
    }

    /// Create an AMQP broker builder.
    ///
    /// Returns an [`AmqpBrokerBuilder`] for fluent configuration.
    /// Defaults to exchange topology `main`, `retry`, `dlx`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Default exchanges: "main", "retry", "dlx"
    /// let broker = Broker::amqp("amqp://localhost:5672").build().await?;
    ///
    /// // Prefixed: "listener", "listener.retry", "listener.dlx"
    /// let broker = Broker::amqp("amqp://localhost:5672")
    ///     .with_exchange_prefix("listener")
    ///     .build()
    ///     .await?;
    ///
    /// // Full control over exchange names
    /// let broker = Broker::amqp("amqp://localhost:5672")
    ///     .with_topology(ExchangeTopology::new("a", "b", "c"))
    ///     .build()
    ///     .await?;
    /// ```
    #[cfg(feature = "amqp")]
    pub fn amqp(url: &str) -> AmqpBrokerBuilder<'_> {
        AmqpBrokerBuilder {
            url,
            topology: None,
            ensure_publish: false,
        }
    }

    #[cfg(feature = "amqp")]
    fn amqp_infra_topology_snapshot(topology: &Arc<AmqpInfraTopology>) -> AmqpInfraTopology {
        topology.as_ref().clone()
    }

    pub async fn from_url(url: &str) -> Result<Self, BrokerError> {
        let url =  url.trim();
        if url.starts_with("redis://") {
            Broker::redis(url)
            .await
        } else if url.starts_with("amqp://") {
            Broker::amqp(url)
            .build()
            .await
        } else {
            Err(BrokerError::UnknownUrlSchema(url.to_string()))
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Publishing
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get a publisher scoped to a namespace.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let publisher = broker.publisher("ethereum").await?;
    /// // Redis stream: "ethereum.blocks"
    /// // AMQP routing key: "ethereum.blocks" on configured main exchange
    /// publisher.publish("blocks", &block_event).await?;
    /// publisher.publish("forks", &fork_event).await?;
    /// ```
    pub async fn publisher(&self, namespace: &str) -> Result<Publisher, BrokerError> {
        self.publisher_with_namespace(Some(namespace)).await
    }

    /// Get an unscoped publisher (routing keys are not namespace-qualified).
    pub async fn publisher_unscoped(&self) -> Result<Publisher, BrokerError> {
        self.publisher_with_namespace(None).await
    }

    async fn publisher_with_namespace(
        &self,
        namespace: Option<&str>,
    ) -> Result<Publisher, BrokerError> {
        let namespace = namespace.and_then(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        });

        match self {
            #[cfg(feature = "redis")]
            Broker::Redis {
                conn,
                ensure_publish,
            } => {
                let mut builder = RedisPublisher::builder((**conn).clone())
                    .max_retries(3)
                    .auto_trim(Duration::from_secs(60))
                    .fallback_maxlen(100_000);

                if *ensure_publish {
                    builder = builder.replication_wait(1, Duration::from_millis(500));
                }

                let inner = builder.build();
                let dyn_pub = DynPublisher::new(inner);
                Ok(Publisher::new(dyn_pub, namespace))
            }
            #[cfg(feature = "amqp")]
            Broker::Amqp {
                conn,
                amqp_infra_topology,
                ensure_publish,
            } => {
                let manager = ExchangeManager::new(Arc::clone(conn));
                let topology = Self::amqp_infra_topology_snapshot(amqp_infra_topology);
                manager.declare_topology(&topology).await?;

                let inner = if *ensure_publish {
                    RmqPublisher::for_exchange_with_confirms(Arc::clone(conn), &topology.main)
                        .await?
                } else {
                    RmqPublisher::for_exchange(Arc::clone(conn), &topology.main).await
                };

                let dyn_pub = DynPublisher::new(inner);
                Ok(Publisher::new(dyn_pub, namespace))
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Consuming
    // ═══════════════════════════════════════════════════════════════════════════

    /// Create a consumer builder for a topic.
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
    pub fn consumer(&self, topic: &Topic) -> ConsumerBuilder<'_> {
        match self {
            #[cfg(feature = "redis")]
            Broker::Redis { conn, .. } => {
                ConsumerBuilder::new(consumer::ConsumerBackend::Redis(conn), topic.clone())
            }
            #[cfg(feature = "amqp")]
            Broker::Amqp {
                conn,
                amqp_infra_topology,
                ..
            } => {
                let topology = Self::amqp_infra_topology_snapshot(amqp_infra_topology);
                ConsumerBuilder::new(
                    consumer::ConsumerBackend::Amqp(conn, topology),
                    topic.clone(),
                )
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Introspection
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get the backend name as a string.
    pub fn backend_name(&self) -> &'static str {
        match self {
            #[cfg(feature = "redis")]
            Broker::Redis { .. } => "redis",
            #[cfg(feature = "amqp")]
            Broker::Amqp { .. } => "amqp",
        }
    }

    /// Returns whether `ensure_publish` (replication durability) is enabled.
    pub fn ensure_publish(&self) -> bool {
        match self {
            #[cfg(feature = "redis")]
            Broker::Redis { ensure_publish, .. } => *ensure_publish,
            #[cfg(feature = "amqp")]
            Broker::Amqp { ensure_publish, .. } => *ensure_publish,
        }
    }

    /// Query queue/stream depth for a given topic and optional consumer group.
    ///
    /// Returns message counts for the principal, retry, and dead-letter queues.
    /// Uses [`Topic::key()`] as the backend queue/stream name.
    ///
    /// When `group` is provided, Redis also queries `XINFO GROUPS` to populate
    /// `pending` (PEL count) and `lag` (undelivered entries). Use
    /// [`QueueDepths::has_pending_work()`] to check if a consumer will receive
    /// messages — this is the primary mechanism for seed-message decisions.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use broker::{Broker, Topic, routing};
    ///
    /// // Stream-level depth only (no group)
    /// let topic = Topic::new(routing::BLOCKS).with_namespace("ethereum");
    /// let depths = broker.queue_depths(&topic, None).await?;
    ///
    /// // Consumer-group-aware depth (Redis: includes pending + lag)
    /// let topic = Topic::new(routing::FETCH_NEW_BLOCKS).with_namespace("ethereum");
    /// let depths = broker.queue_depths(&topic, Some("fetch-new-blocks")).await?;
    /// if !depths.has_pending_work() {
    ///     // Publish a seed message to bootstrap the cursor loop
    /// }
    /// ```
    pub async fn queue_depths(
        &self,
        topic: &Topic,
        group: Option<&str>,
    ) -> Result<QueueDepths, BrokerError> {
        let name = topic.key();
        match self {
            #[cfg(feature = "redis")]
            Broker::Redis { conn, .. } => {
                let inspector = RedisQueueInspector::new((**conn).clone());
                Ok(inspector.queue_depths(&name, group).await?)
            }
            #[cfg(feature = "amqp")]
            Broker::Amqp { conn, .. } => {
                let inspector = AmqpQueueInspector::new((**conn).clone());
                Ok(inspector.queue_depths(&name, group).await?)
            }
        }
    }

    /// Fast check: is the queue/stream empty for this consumer group?
    ///
    /// Returns `true` when the consumer will receive **no** messages — the
    /// caller should publish a seed to bootstrap the processing loop.
    ///
    /// Single backend round-trip (no dead-letter or retry queries).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use broker::{Broker, Topic, routing};
    ///
    /// let topic = Topic::new(routing::FETCH_NEW_BLOCKS).with_namespace("ethereum");
    /// if broker.is_empty(&topic, routing::FETCH_NEW_BLOCKS).await? {
    ///     publisher.publish(routing::FETCH_NEW_BLOCKS, &Value::Null).await?;
    /// }
    /// ```
    pub async fn is_empty(&self, topic: &Topic, group: &str) -> Result<bool, BrokerError> {
        let name = topic.key();
        match self {
            #[cfg(feature = "redis")]
            Broker::Redis { conn, .. } => {
                let inspector = RedisQueueInspector::new((**conn).clone());
                Ok(inspector.is_empty(&name, group).await?)
            }
            #[cfg(feature = "amqp")]
            Broker::Amqp { conn, .. } => {
                let inspector = AmqpQueueInspector::new((**conn).clone());
                Ok(inspector.is_empty(&name, group).await?)
            }
        }
    }

    /// Returns `true` if the consumer group is caught up — either fully idle
    /// or has at most one message currently being consumed (pending <= 1, lag == 0).
    ///
    /// Designed for deduplication guards: when the prefetch count is 1, a single
    /// pending entry means a consumer is already processing the message. Callers
    /// can use this to skip duplicate work rather than enqueueing an overlapping task.
    ///
    /// Single backend round-trip (no dead-letter or retry queries).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use broker::{Broker, Topic, routing};
    ///
    /// let topic = Topic::new(routing::FETCH_NEW_BLOCKS).with_namespace("ethereum");
    /// if broker.is_empty_or_pending(&topic, routing::FETCH_NEW_BLOCKS).await? {
    ///     // Consumer is idle or processing its last message — safe to skip
    /// }
    /// ```
    pub async fn is_empty_or_pending(
        &self,
        topic: &Topic,
        group: &str,
    ) -> Result<bool, BrokerError> {
        let name = topic.key();
        match self {
            #[cfg(feature = "redis")]
            Broker::Redis { conn, .. } => {
                let inspector = RedisQueueInspector::new((**conn).clone());
                Ok(inspector.is_empty_or_pending(&name, group).await?)
            }
            #[cfg(feature = "amqp")]
            Broker::Amqp { conn, .. } => {
                let inspector = AmqpQueueInspector::new((**conn).clone());
                Ok(inspector.is_empty_or_pending(&name, group).await?)
            }
        }
    }

    /// Returns `true` if the queue or stream for this topic exists.
    ///
    /// - **Redis**: checks key type via `TYPE` — `true` only for stream keys.
    /// - **AMQP**: passive `queue_declare` — `true` if the broker knows the queue.
    ///
    /// Does **not** check consumer group existence — only the underlying
    /// queue/stream.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use broker::{Broker, Topic, routing};
    ///
    /// let topic = Topic::new(routing::FETCH_NEW_BLOCKS).with_namespace("ethereum");
    /// if !broker.exists(&topic).await? {
    ///     // Queue/stream hasn't been created yet — skip depth check
    /// }
    /// ```
    pub async fn exists(&self, topic: &Topic) -> Result<bool, BrokerError> {
        let name = topic.key();
        match self {
            #[cfg(feature = "redis")]
            Broker::Redis { conn, .. } => {
                let inspector = RedisQueueInspector::new((**conn).clone());
                Ok(inspector.exists(&name).await?)
            }
            #[cfg(feature = "amqp")]
            Broker::Amqp { conn, .. } => {
                let inspector = AmqpQueueInspector::new((**conn).clone());
                Ok(inspector.exists(&name).await?)
            }
        }
    }
}

/// Builder for configuring an AMQP broker.
///
/// Created via [`Broker::amqp`]. Call [`.build()`](AmqpBrokerBuilder::build)
/// to finalize.
#[cfg(feature = "amqp")]
#[must_use]
pub struct AmqpBrokerBuilder<'a> {
    url: &'a str,
    topology: Option<AmqpInfraTopology>,
    ensure_publish: bool,
}

#[cfg(feature = "amqp")]
impl<'a> AmqpBrokerBuilder<'a> {
    /// Set exchange prefix — derives `"{prefix}"`, `"{prefix}.retry"`, `"{prefix}.dlx"`.
    ///
    /// ```ignore
    /// Broker::amqp("amqp://localhost:5672")
    ///     .with_exchange_prefix("listener")
    ///     .build()
    ///     .await?;
    /// ```
    pub fn with_exchange_prefix(mut self, prefix: impl AsRef<str>) -> Self {
        self.topology = Some(AmqpInfraTopology::from_prefix(prefix));
        self
    }

    /// Set a fully custom exchange topology.
    ///
    /// ```ignore
    /// use broker::ExchangeTopology;
    ///
    /// Broker::amqp("amqp://localhost:5672")
    ///     .with_topology(ExchangeTopology::new("my-main", "my-retry", "my-dlx"))
    ///     .build()
    ///     .await?;
    /// ```
    pub fn with_topology(mut self, topology: AmqpInfraTopology) -> Self {
        self.topology = Some(topology);
        self
    }

    /// Enable publisher confirms for replication-aware publish durability.
    ///
    /// When enabled, the publisher calls `confirm_select()` on its channel
    /// and awaits `Confirmation::Ack` from the broker for every publish.
    pub fn with_ensure_publish(mut self, ensure_publish: bool) -> Self {
        self.ensure_publish = ensure_publish;
        self
    }

    /// Build the AMQP broker.
    pub async fn build(self) -> Result<Broker, BrokerError> {
        let topology = self.topology.unwrap_or_else(default_amqp_infra_topology);
        let conn = AmqpConnectionManager::new(self.url);
        Ok(Broker::Amqp {
            conn: Arc::new(conn),
            amqp_infra_topology: Arc::new(topology),
            ensure_publish: self.ensure_publish,
        })
    }
}

impl std::fmt::Debug for Broker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "redis")]
            Broker::Redis { ensure_publish, .. } => f
                .debug_struct("Broker::Redis")
                .field("ensure_publish", ensure_publish)
                .finish_non_exhaustive(),
            #[cfg(feature = "amqp")]
            Broker::Amqp { ensure_publish, .. } => f
                .debug_struct("Broker::Amqp")
                .field("ensure_publish", ensure_publish)
                .finish_non_exhaustive(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_creation() {
        let topic = Topic::new("blocks").with_namespace("ethereum");
        assert_eq!(topic.namespace(), Some("ethereum"));
        assert_eq!(topic.routing_segment(), "blocks");
        assert_eq!(topic.key(), "ethereum.blocks");
    }

    #[test]
    fn test_consumer_config() {
        let config = ConsumerConfig::new("my-worker")
            .with_prefetch(100)
            .with_retries(5);

        assert_eq!(config.group, "my-worker");
        assert_eq!(config.prefetch, 100);
        assert_eq!(config.max_retries, 5);
    }

    #[cfg(feature = "amqp")]
    #[tokio::test]
    async fn test_amqp_default_topology() {
        let broker = Broker::amqp("amqp://guest:guest@localhost:5672")
            .build()
            .await
            .unwrap();
        match &broker {
            Broker::Amqp {
                amqp_infra_topology,
                ..
            } => {
                assert_eq!(amqp_infra_topology.main, "main");
                assert_eq!(amqp_infra_topology.retry, "retry");
                assert_eq!(amqp_infra_topology.dlx, "dlx");
            }
            #[allow(unreachable_patterns)]
            _ => panic!("expected AMQP variant"),
        }
    }

    #[cfg(feature = "amqp")]
    #[tokio::test]
    async fn test_amqp_with_exchange_prefix() {
        let broker = Broker::amqp("amqp://guest:guest@localhost:5672")
            .with_exchange_prefix("listener")
            .build()
            .await
            .unwrap();
        match &broker {
            Broker::Amqp {
                amqp_infra_topology,
                ..
            } => {
                assert_eq!(amqp_infra_topology.main, "listener");
                assert_eq!(amqp_infra_topology.retry, "listener.retry");
                assert_eq!(amqp_infra_topology.dlx, "listener.dlx");
            }
            #[allow(unreachable_patterns)]
            _ => panic!("expected AMQP variant"),
        }
    }

    #[cfg(feature = "amqp")]
    #[tokio::test]
    async fn test_amqp_with_custom_topology() {
        let topology = ExchangeTopology::new("my-app", "my-app-retry", "my-app-dead");
        let broker = Broker::amqp("amqp://guest:guest@localhost:5672")
            .with_topology(topology)
            .build()
            .await
            .unwrap();
        match &broker {
            Broker::Amqp {
                amqp_infra_topology,
                ..
            } => {
                assert_eq!(amqp_infra_topology.main, "my-app");
                assert_eq!(amqp_infra_topology.retry, "my-app-retry");
                assert_eq!(amqp_infra_topology.dlx, "my-app-dead");
            }
            #[allow(unreachable_patterns)]
            _ => panic!("expected AMQP variant"),
        }
    }
}
