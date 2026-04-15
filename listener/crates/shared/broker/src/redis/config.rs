use std::time::Duration;

use crate::traits::consumer::RetryPolicy;

use super::circuit_breaker::CircuitBreakerConfig;
use super::error::RedisConsumerError;

/// Stream topology for a specific chain.
/// Provides consistent naming for main and dead-letter streams.
#[derive(Debug, Clone)]
pub struct StreamTopology {
    /// Main stream name (e.g., "ethereum.events")
    pub main: String,
    /// Dead-letter stream name (e.g., "ethereum.events:dead")
    pub dead: String,
}

impl StreamTopology {
    /// Explicit constructor for full control over stream names.
    pub fn new(main: impl Into<String>, dead: impl Into<String>) -> Self {
        Self {
            main: main.into(),
            dead: dead.into(),
        }
    }

    /// Derive topology from a base prefix.
    /// Example: `"orders.events"` -> main=`"orders.events"`, dead=`"orders.events:dead"`
    ///
    /// # Example
    /// ```
    /// use broker::redis::StreamTopology;
    /// let topology = StreamTopology::from_prefix("ethereum.events");
    /// assert_eq!(topology.main, "ethereum.events");
    /// assert_eq!(topology.dead, "ethereum.events:dead");
    /// ```
    pub fn from_prefix(prefix: impl AsRef<str>) -> Self {
        let prefix = prefix.as_ref();
        Self::new(prefix, format!("{prefix}:dead"))
    }
}

/// Base Redis consumer configuration.
#[derive(Debug, Clone)]
pub struct RedisConsumerConfig {
    /// Stream to consume from
    pub stream: String,
    /// Consumer group name
    pub group_name: String,
    /// Unique consumer name within the group
    pub consumer_name: String,
}

/// Consumer configuration with retry support.
#[derive(Debug, Clone)]
pub struct RedisRetryConfig {
    /// Base consumer configuration
    pub base: RedisConsumerConfig,
    /// Dead-letter stream name
    pub dead_stream: String,
    /// Maximum retry attempts for permanent failures before dead-letter routing.
    ///
    /// `HandlerError::Transient` retries are infinite and do not consume this budget.
    pub max_retries: u32,
    /// Minimum idle time before claiming a pending message
    pub claim_min_idle: Duration,
    /// Interval between claim sweep cycles
    pub claim_interval: Duration,
    /// Optional circuit breaker configuration.
    /// When set, the consumer pauses consumption on consecutive `Transient` handler errors,
    /// preventing DLQ pollution during downstream outages (DB down, API timeout, etc.).
    /// When `None`, all handler errors go through the normal ClaimSweeper/DLQ path.
    pub circuit_breaker: Option<CircuitBreakerConfig>,
}

/// Consumer configuration with retry and prefetch support for high-throughput.
#[derive(Debug, Clone)]
pub struct RedisPrefetchConfig {
    /// Retry configuration
    pub retry: RedisRetryConfig,
    /// Number of messages to prefetch per XREADGROUP call.
    pub prefetch_count: usize,
    /// Milliseconds to sleep between non-blocking XREADGROUP polls for new messages.
    /// Lower values = lower latency but more Redis round-trips.
    pub block_ms: usize,
}

impl RetryPolicy for RedisRetryConfig {
    fn max_retries(&self) -> u32 {
        self.max_retries
    }

    fn retry_delay(&self) -> Duration {
        self.claim_min_idle
    }
}

impl RedisRetryConfig {
    /// Redis hash key used to persist failure classification.
    ///
    /// Hash field = stream entry ID, value = "transient" | "permanent".
    pub fn classification_marker_key(&self) -> String {
        format!(
            "mq:classification:{}:{}",
            self.base.stream, self.base.group_name
        )
    }

    /// Backward-compatible alias.
    /// Prefer [`Self::classification_marker_key`].
    #[allow(dead_code)]
    pub fn transient_marker_key(&self) -> String {
        self.classification_marker_key()
    }

    /// Backward-compatible alias.
    /// Prefer [`Self::classification_marker_key`].
    #[allow(dead_code)]
    pub fn permanent_marker_key(&self) -> String {
        self.classification_marker_key()
    }
}

/// Builder for constructing Redis consumer configurations with validation.
#[must_use]
#[derive(Debug, Default)]
pub struct RedisConsumerConfigBuilder {
    stream: Option<String>,
    group_name: Option<String>,
    consumer_name: Option<String>,
    dead_stream: Option<String>,
    max_retries: Option<u32>,
    claim_min_idle: Option<Duration>,
    claim_interval: Option<Duration>,
    prefetch_count: Option<usize>,
    block_ms: Option<usize>,
    cb_failure_threshold: Option<u32>,
    cb_cooldown_duration: Option<Duration>,
}

impl RedisConsumerConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stream(mut self, stream: impl Into<String>) -> Self {
        self.stream = Some(stream.into());
        self
    }

    pub fn group_name(mut self, group_name: impl Into<String>) -> Self {
        self.group_name = Some(group_name.into());
        self
    }

    pub fn consumer_name(mut self, consumer_name: impl Into<String>) -> Self {
        self.consumer_name = Some(consumer_name.into());
        self
    }

    pub fn dead_stream(mut self, dead_stream: impl Into<String>) -> Self {
        self.dead_stream = Some(dead_stream.into());
        self
    }

    /// Set the maximum retry attempts for permanent failures.
    ///
    /// This limit is only applied to `Execution`/`Deserialization` failures.
    /// `Transient` failures always retry indefinitely.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn claim_min_idle(mut self, claim_min_idle: Duration) -> Self {
        self.claim_min_idle = Some(claim_min_idle);
        self
    }

    pub fn claim_interval(mut self, claim_interval: Duration) -> Self {
        self.claim_interval = Some(claim_interval);
        self
    }

    pub fn prefetch_count(mut self, prefetch_count: usize) -> Self {
        self.prefetch_count = Some(prefetch_count);
        self
    }

    pub fn block_ms(mut self, block_ms: usize) -> Self {
        self.block_ms = Some(block_ms);
        self
    }

    /// Set the circuit breaker failure threshold (consecutive `Transient` errors to trip).
    ///
    /// When both `circuit_breaker_threshold` and `circuit_breaker_cooldown` are set,
    /// the consumer will pause consumption after this many consecutive transient failures,
    /// preventing DLQ pollution during downstream outages.
    /// When not set, no circuit breaker is used (backward compatible).
    pub fn circuit_breaker_threshold(mut self, threshold: u32) -> Self {
        self.cb_failure_threshold = Some(threshold);
        self
    }

    /// Set the circuit breaker cooldown duration (how long to pause before probing).
    ///
    /// After the circuit trips, the consumer pauses for this duration, then
    /// allows one test message through (half-open). If it succeeds, consumption
    /// resumes. If it fails, the circuit reopens for another cooldown period.
    pub fn circuit_breaker_cooldown(mut self, cooldown: Duration) -> Self {
        self.cb_cooldown_duration = Some(cooldown);
        self
    }

    /// Apply stream topology to configure streams automatically.
    pub fn with_topology(mut self, topology: &StreamTopology) -> Self {
        self.stream = Some(topology.main.clone());
        self.dead_stream = Some(topology.dead.clone());
        self
    }

    fn build_base(&self) -> Result<RedisConsumerConfig, RedisConsumerError> {
        Ok(RedisConsumerConfig {
            stream: self
                .stream
                .clone()
                .ok_or_else(|| RedisConsumerError::Configuration("stream is required".into()))?,
            group_name: self.group_name.clone().ok_or_else(|| {
                RedisConsumerError::Configuration("group_name is required".into())
            })?,
            consumer_name: self.consumer_name.clone().ok_or_else(|| {
                RedisConsumerError::Configuration("consumer_name is required".into())
            })?,
        })
    }

    fn build_retry(&self) -> Result<RedisRetryConfig, RedisConsumerError> {
        let base = self.build_base()?;

        // Build circuit breaker config if either threshold or cooldown is set
        let circuit_breaker = match (self.cb_failure_threshold, self.cb_cooldown_duration) {
            (Some(threshold), Some(cooldown)) => Some(CircuitBreakerConfig {
                failure_threshold: threshold,
                cooldown_duration: cooldown,
            }),
            (Some(threshold), None) => Some(CircuitBreakerConfig {
                failure_threshold: threshold,
                ..CircuitBreakerConfig::default()
            }),
            (None, Some(cooldown)) => Some(CircuitBreakerConfig {
                cooldown_duration: cooldown,
                ..CircuitBreakerConfig::default()
            }),
            (None, None) => None,
        };

        Ok(RedisRetryConfig {
            base,
            dead_stream: self.dead_stream.clone().ok_or_else(|| {
                RedisConsumerError::Configuration("dead_stream is required".into())
            })?,
            max_retries: self.max_retries.unwrap_or(3),
            claim_min_idle: self.claim_min_idle.unwrap_or(Duration::from_secs(30)),
            claim_interval: self.claim_interval.unwrap_or(Duration::from_secs(10)),
            circuit_breaker,
        })
    }

    /// Build a prefetch consumer configuration.
    pub fn build_prefetch(self) -> Result<RedisPrefetchConfig, RedisConsumerError> {
        let prefetch_count = self.prefetch_count.unwrap_or(10);
        let block_ms = self.block_ms.unwrap_or(200);
        let retry = self.build_retry()?;

        Ok(RedisPrefetchConfig {
            retry,
            prefetch_count,
            block_ms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_topology_from_prefix() {
        let topology = StreamTopology::from_prefix("ethereum.events");

        assert_eq!(topology.main, "ethereum.events");
        assert_eq!(topology.dead, "ethereum.events:dead");
    }

    #[test]
    fn test_consumer_config_builder_validation_fails() {
        let result = RedisConsumerConfigBuilder::new()
            .stream("ethereum.events")
            .dead_stream("ethereum.events:dead")
            // Missing group_name, consumer_name
            .build_prefetch();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, RedisConsumerError::Configuration(_)));
    }

    #[test]
    fn test_prefetch_config_builder() {
        let config = RedisConsumerConfigBuilder::new()
            .stream("ethereum.events")
            .group_name("notifier-group")
            .consumer_name("pod-1")
            .dead_stream("ethereum.events:dead")
            .prefetch_count(50)
            .block_ms(2000)
            .build_prefetch()
            .unwrap();

        assert_eq!(config.prefetch_count, 50);
        assert_eq!(config.block_ms, 2000);
        assert_eq!(config.retry.base.stream, "ethereum.events");
        assert_eq!(config.retry.max_retries, 3);
        assert_eq!(config.retry.claim_min_idle, Duration::from_secs(30));
    }

    #[test]
    fn test_prefetch_config_defaults() {
        let config = RedisConsumerConfigBuilder::new()
            .stream("ethereum.events")
            .group_name("notifier-group")
            .consumer_name("pod-1")
            .dead_stream("ethereum.events:dead")
            .build_prefetch()
            .unwrap();

        assert_eq!(config.prefetch_count, 10);
        assert_eq!(config.block_ms, 200);
    }

    #[test]
    fn test_builder_with_topology() {
        let topology = StreamTopology::from_prefix("polygon.events");

        let config = RedisConsumerConfigBuilder::new()
            .with_topology(&topology)
            .group_name("notifier-group")
            .consumer_name("pod-1")
            .build_prefetch()
            .unwrap();

        assert_eq!(config.retry.base.stream, "polygon.events");
        assert_eq!(config.retry.dead_stream, "polygon.events:dead");
    }
}
