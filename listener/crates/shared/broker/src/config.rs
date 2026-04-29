//! Consumer configuration types.

use std::time::Duration;

/// Consumer configuration with sensible defaults.
///
/// # Examples
///
/// ```
/// use broker::ConsumerConfig;
/// use std::time::Duration;
///
/// let config = ConsumerConfig::new("fetch-blocks-worker")
///     .with_prefetch(100)
///     .with_retries(5)
///     .with_circuit_breaker(3, Duration::from_secs(30));
/// ```
#[derive(Debug, Clone)]
pub struct ConsumerConfig {
    /// Logical queue/consumer-group name (e.g., "fetch-blocks-worker", "fork-handler").
    /// - RMQ: queue defaults to `{namespace}.{group}` (or uses already-qualified input)
    /// - Redis: this becomes the consumer group name
    pub group: String,

    /// Instance name within the group (auto-generated if None).
    /// - RMQ: this becomes the consumer tag
    /// - Redis: this becomes the consumer name within the group
    pub consumer_name: Option<String>,

    /// How many messages to process in parallel (default: 10).
    pub prefetch: usize,

    /// Max retries before dead-letter (default: 3).
    pub max_retries: u32,

    /// Circuit breaker: (failure_threshold, cooldown_duration).
    pub circuit_breaker: Option<(u32, Duration)>,
}

impl Default for ConsumerConfig {
    fn default() -> Self {
        Self {
            group: String::new(),
            consumer_name: None,
            prefetch: 10,
            max_retries: 3,
            circuit_breaker: None,
        }
    }
}

impl ConsumerConfig {
    /// Create a new consumer config with the given group name.
    pub fn new(group: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            ..Default::default()
        }
    }

    /// Set the prefetch count.
    pub fn with_prefetch(mut self, n: usize) -> Self {
        self.prefetch = n;
        self
    }

    /// Set the max retries before dead-letter.
    pub fn with_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    /// Set the circuit breaker configuration.
    pub fn with_circuit_breaker(mut self, threshold: u32, cooldown: Duration) -> Self {
        self.circuit_breaker = Some((threshold, cooldown));
        self
    }

    /// Set the consumer name within the group.
    pub fn with_consumer_name(mut self, name: impl Into<String>) -> Self {
        self.consumer_name = Some(name.into());
        self
    }
}

/// Redis-specific options (optional).
#[cfg(feature = "redis")]
#[derive(Debug, Clone, Default)]
pub struct RedisOptions {
    /// Block time in milliseconds for XREADGROUP (default: 5000).
    pub block_ms: Option<usize>,
    /// Minimum idle time in seconds before claiming a pending message (default: 30).
    pub claim_min_idle_secs: Option<u64>,
    /// Interval in seconds between claim sweep cycles (default: 10).
    pub claim_interval_secs: Option<u64>,
}

/// AMQP-specific options (optional).
#[cfg(feature = "amqp")]
#[derive(Debug, Clone, Default)]
pub struct AmqpOptions {
    /// Retry delay in seconds (default: 5).
    pub retry_delay_secs: Option<u64>,
    /// Routing key pattern for wildcards like "ethereum.blocks.#".
    ///
    /// When not set, broker uses the namespace-qualified default:
    /// `{namespace}.{routing}`.
    pub routing_key_pattern: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_config_defaults() {
        let config = ConsumerConfig::new("my-worker");
        assert_eq!(config.group, "my-worker");
        assert_eq!(config.prefetch, 10);
        assert_eq!(config.max_retries, 3);
        assert!(config.consumer_name.is_none());
        assert!(config.circuit_breaker.is_none());
    }

    #[test]
    fn test_consumer_config_builder() {
        let config = ConsumerConfig::new("my-worker")
            .with_prefetch(100)
            .with_retries(5)
            .with_consumer_name("pod-1")
            .with_circuit_breaker(3, Duration::from_secs(30));

        assert_eq!(config.group, "my-worker");
        assert_eq!(config.prefetch, 100);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.consumer_name, Some("pod-1".to_string()));
        assert_eq!(config.circuit_breaker, Some((3, Duration::from_secs(30))));
    }
}
