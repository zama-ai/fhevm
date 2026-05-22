use std::time::Duration;

use crate::traits::circuit_breaker::CircuitBreakerConfig;
use crate::traits::consumer::RetryPolicy;

use super::error::ConsumerError;

/// Exchange topology for a specific chain.
/// Provides consistent naming for main, retry, and dead-letter exchanges.
#[derive(Debug, Clone)]
pub struct ExchangeTopology {
    /// Main exchange name (e.g., "ethereum.events")
    pub main: String,
    /// Retry exchange name (e.g., "ethereum.events.retry")
    pub retry: String,
    /// Dead-letter exchange name (e.g., "ethereum.events.dlx")
    pub dlx: String,
}

impl ExchangeTopology {
    /// Explicit constructor for full control over exchange names.
    pub fn new(main: impl Into<String>, retry: impl Into<String>, dlx: impl Into<String>) -> Self {
        Self {
            main: main.into(),
            retry: retry.into(),
            dlx: dlx.into(),
        }
    }

    /// Derive topology from a base prefix.
    /// Example: `"orders.events"` -> retry=`"orders.events.retry"`, dlx=`"orders.events.dlx"`
    ///
    /// # Example
    /// ```
    /// use broker::amqp::ExchangeTopology;
    /// let topology = ExchangeTopology::from_prefix("ethereum.events");
    /// assert_eq!(topology.main, "ethereum.events");
    /// assert_eq!(topology.retry, "ethereum.events.retry");
    /// assert_eq!(topology.dlx, "ethereum.events.dlx");
    /// ```
    pub fn from_prefix(prefix: impl AsRef<str>) -> Self {
        let prefix = prefix.as_ref();
        Self::new(prefix, format!("{prefix}.retry"), format!("{prefix}.dlx"))
    }
}

/// Base consumer configuration.
#[derive(Debug, Clone)]
pub struct ConsumerConfig {
    /// Exchange to consume from
    pub exchange: String,
    /// Queue name
    pub queue: String,
    /// Routing key for binding
    pub routing_key: String,
    /// Unique consumer tag
    pub consumer_tag: String,
}

/// Consumer configuration with retry support.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Base consumer configuration
    pub base: ConsumerConfig,
    /// Retry exchange name
    pub retry_exchange: String,
    /// Dead-letter exchange name
    pub dead_exchange: String,
    /// Maximum retry attempts for permanent failures before DLQ routing.
    ///
    /// `HandlerError::Transient` retries are infinite and do not consume this budget.
    pub max_retries: u32,
    /// Delay between retries
    pub retry_delay: Duration,
    /// Optional circuit breaker — pauses consumption on consecutive `Transient`
    /// handler errors, preventing DLQ pollution during downstream outages
    /// (DB down, API timeout, etc.). When `None`, all errors go through the
    /// normal retry/DLQ path.
    pub circuit_breaker: Option<CircuitBreakerConfig>,
}

impl RetryConfig {
    const RETRY_ROUTING_PREFIX: &'static str = "__mq.retry";
    const DEAD_ROUTING_PREFIX: &'static str = "__mq.dead";

    /// Build a queue-scoped retry routing key for internal retry plumbing.
    pub(crate) fn retry_routing_key_for_queue(queue: &str) -> String {
        format!("{}.{}", Self::RETRY_ROUTING_PREFIX, queue)
    }

    /// Build a queue-scoped dead routing key for internal dead-letter plumbing.
    pub(crate) fn dead_routing_key_for_queue(queue: &str) -> String {
        format!("{}.{}", Self::DEAD_ROUTING_PREFIX, queue)
    }

    /// Queue-scoped retry routing key used for internal retry plumbing.
    ///
    /// This isolates retries per queue even when multiple queues are bound to
    /// the same retry exchange.
    pub(crate) fn retry_routing_key(&self) -> String {
        Self::retry_routing_key_for_queue(&self.base.queue)
    }

    /// Queue-scoped dead-letter routing key used for internal DLQ plumbing.
    ///
    /// This isolates DLQ routing per queue even when multiple queues share
    /// the same dead-letter exchange.
    pub(crate) fn dead_routing_key(&self) -> String {
        Self::dead_routing_key_for_queue(&self.base.queue)
    }
}

/// Consumer configuration with retry and prefetch support for high-throughput.
#[derive(Debug, Clone)]
pub struct PrefetchConfig {
    /// Retry configuration
    pub retry: RetryConfig,
    /// Number of messages to prefetch (QoS)
    pub prefetch_count: u16,
}

impl RetryPolicy for RetryConfig {
    fn max_retries(&self) -> u32 {
        self.max_retries
    }

    fn retry_delay(&self) -> Duration {
        self.retry_delay
    }
}

/// Consumer configuration for cron-style scheduled jobs.
#[derive(Debug, Clone)]
pub struct CronConfig {
    /// Base consumer configuration
    pub base: ConsumerConfig,
    /// Retry/delay exchange name
    pub retry_exchange: String,
    /// Interval between job executions
    pub interval: Duration,
}

/// Builder for constructing consumer configurations with validation.
#[must_use]
#[derive(Debug, Default)]
pub struct ConsumerConfigBuilder {
    exchange: Option<String>,
    queue: Option<String>,
    routing_key: Option<String>,
    consumer_tag: Option<String>,
    retry_exchange: Option<String>,
    dead_exchange: Option<String>,
    max_retries: Option<u32>,
    retry_delay: Option<Duration>,
    prefetch_count: Option<u16>,
    cron_interval: Option<Duration>,
    cb_failure_threshold: Option<u32>,
    cb_cooldown_duration: Option<Duration>,
    cb_half_open_timeout: Option<Duration>,
}

impl ConsumerConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }

    pub fn queue(mut self, queue: impl Into<String>) -> Self {
        self.queue = Some(queue.into());
        self
    }

    pub fn routing_key(mut self, routing_key: impl Into<String>) -> Self {
        self.routing_key = Some(routing_key.into());
        self
    }

    pub fn consumer_tag(mut self, consumer_tag: impl Into<String>) -> Self {
        self.consumer_tag = Some(consumer_tag.into());
        self
    }

    pub fn retry_exchange(mut self, retry_exchange: impl Into<String>) -> Self {
        self.retry_exchange = Some(retry_exchange.into());
        self
    }

    pub fn dead_exchange(mut self, dead_exchange: impl Into<String>) -> Self {
        self.dead_exchange = Some(dead_exchange.into());
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

    pub fn retry_delay(mut self, retry_delay: Duration) -> Self {
        self.retry_delay = Some(retry_delay);
        self
    }

    pub fn prefetch_count(mut self, prefetch_count: u16) -> Self {
        self.prefetch_count = Some(prefetch_count);
        self
    }

    pub fn cron_interval(mut self, interval: Duration) -> Self {
        self.cron_interval = Some(interval);
        self
    }

    /// Set the circuit breaker failure threshold (consecutive `Transient` errors to trip).
    ///
    /// When both `circuit_breaker_threshold` and `circuit_breaker_cooldown` are set,
    /// the consumer will pause consumption after this many consecutive transient failures.
    /// When not set, no circuit breaker is used.
    pub fn circuit_breaker_threshold(mut self, threshold: u32) -> Self {
        self.cb_failure_threshold = Some(threshold);
        self
    }

    /// Set the circuit breaker cooldown duration (how long to pause before probing).
    ///
    /// After the circuit trips, the consumer pauses for this duration, then allows
    /// one test message through (half-open). If it succeeds, consumption resumes.
    pub fn circuit_breaker_cooldown(mut self, cooldown: Duration) -> Self {
        self.cb_cooldown_duration = Some(cooldown);
        self
    }

    /// Set the maximum time the breaker may remain in Half-Open without dispatching a probe.
    ///
    /// Provided for API symmetry with the Redis builder; **AMQP consumers ignore this value**
    /// at runtime because Half-Open ordering on AMQP is not preserved (the probe consumes
    /// whatever is at the head of the main queue, not necessarily the failed message). See
    /// `amqp/consumer.rs` for details. Defaults to `5 × cooldown_duration`.
    pub fn circuit_breaker_half_open_timeout(mut self, timeout: Duration) -> Self {
        self.cb_half_open_timeout = Some(timeout);
        self
    }

    /// Apply exchange topology to configure exchanges automatically.
    pub fn with_topology(mut self, topology: &ExchangeTopology) -> Self {
        self.exchange = Some(topology.main.clone());
        self.retry_exchange = Some(topology.retry.clone());
        self.dead_exchange = Some(topology.dlx.clone());
        self
    }

    fn build_base(&self) -> Result<ConsumerConfig, ConsumerError> {
        Ok(ConsumerConfig {
            exchange: self
                .exchange
                .clone()
                .ok_or_else(|| ConsumerError::Configuration("exchange is required".into()))?,
            queue: self
                .queue
                .clone()
                .ok_or_else(|| ConsumerError::Configuration("queue is required".into()))?,
            routing_key: self
                .routing_key
                .clone()
                .ok_or_else(|| ConsumerError::Configuration("routing_key is required".into()))?,
            consumer_tag: self
                .consumer_tag
                .clone()
                .ok_or_else(|| ConsumerError::Configuration("consumer_tag is required".into()))?,
        })
    }

    fn build_retry(&self) -> Result<RetryConfig, ConsumerError> {
        let base = self.build_base()?;

        // AMQP does not consult `half_open_timeout` at runtime (its breaker probes whatever is
        // at the queue head, with no PEL-equivalent). The field is honored from the builder
        // for API symmetry only; falling back to `5 × cooldown_duration` when unset.
        let circuit_breaker = match (self.cb_failure_threshold, self.cb_cooldown_duration) {
            (Some(threshold), Some(cooldown)) => Some(CircuitBreakerConfig {
                failure_threshold: threshold,
                cooldown_duration: cooldown,
                half_open_timeout: self
                    .cb_half_open_timeout
                    .unwrap_or_else(|| cooldown.saturating_mul(5)),
            }),
            (Some(threshold), None) => Some(CircuitBreakerConfig {
                failure_threshold: threshold,
                half_open_timeout: self
                    .cb_half_open_timeout
                    .unwrap_or(CircuitBreakerConfig::default().half_open_timeout),
                ..CircuitBreakerConfig::default()
            }),
            (None, Some(cooldown)) => Some(CircuitBreakerConfig {
                cooldown_duration: cooldown,
                half_open_timeout: self
                    .cb_half_open_timeout
                    .unwrap_or_else(|| cooldown.saturating_mul(5)),
                ..CircuitBreakerConfig::default()
            }),
            (None, None) => None,
        };

        Ok(RetryConfig {
            base,
            retry_exchange: self
                .retry_exchange
                .clone()
                .ok_or_else(|| ConsumerError::Configuration("retry_exchange is required".into()))?,
            dead_exchange: self
                .dead_exchange
                .clone()
                .ok_or_else(|| ConsumerError::Configuration("dead_exchange is required".into()))?,
            max_retries: self.max_retries.unwrap_or(3),
            retry_delay: self.retry_delay.unwrap_or(Duration::from_secs(5)),
            circuit_breaker,
        })
    }

    /// Build a prefetch consumer configuration.
    pub fn build_prefetch(self) -> Result<PrefetchConfig, ConsumerError> {
        let prefetch_count = self.prefetch_count.unwrap_or(10);
        let retry = self.build_retry()?;

        Ok(PrefetchConfig {
            retry,
            prefetch_count,
        })
    }

    /// Build a cron consumer configuration.
    pub fn build_cron(self) -> Result<CronConfig, ConsumerError> {
        let base = self.build_base()?;

        Ok(CronConfig {
            base,
            retry_exchange: self
                .retry_exchange
                .ok_or_else(|| ConsumerError::Configuration("retry_exchange is required".into()))?,
            interval: self
                .cron_interval
                .ok_or_else(|| ConsumerError::Configuration("cron_interval is required".into()))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_topology_from_prefix() {
        let topology = ExchangeTopology::from_prefix("ethereum.events");

        assert_eq!(topology.main, "ethereum.events");
        assert_eq!(topology.retry, "ethereum.events.retry");
        assert_eq!(topology.dlx, "ethereum.events.dlx");
    }

    #[test]
    fn test_consumer_config_builder_validation_fails() {
        let result = ConsumerConfigBuilder::new()
            .exchange("test.exchange")
            .retry_exchange("test.exchange.retry")
            .dead_exchange("test.exchange.dlx")
            // Missing queue, routing_key, consumer_tag
            .build_prefetch();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ConsumerError::Configuration(_)));
    }

    #[test]
    fn test_prefetch_config_builder() {
        let config = ConsumerConfigBuilder::new()
            .exchange("test.exchange")
            .queue("test.queue")
            .routing_key("test.key")
            .consumer_tag("test-consumer")
            .retry_exchange("test.exchange.retry")
            .dead_exchange("test.exchange.dlx")
            .prefetch_count(50)
            .build_prefetch()
            .unwrap();

        assert_eq!(config.prefetch_count, 50);
        assert_eq!(config.retry.base.exchange, "test.exchange");
        assert_eq!(config.retry.max_retries, 3);
        assert_eq!(config.retry.retry_delay, Duration::from_secs(5));
    }

    #[test]
    fn test_cron_config_builder() {
        let config = ConsumerConfigBuilder::new()
            .exchange("test.exchange")
            .queue("test.queue")
            .routing_key("test.key")
            .consumer_tag("test-consumer")
            .retry_exchange("test.exchange.retry")
            .cron_interval(Duration::from_secs(60))
            .build_cron()
            .unwrap();

        assert_eq!(config.base.exchange, "test.exchange");
        assert_eq!(config.retry_exchange, "test.exchange.retry");
        assert_eq!(config.interval, Duration::from_secs(60));
    }

    #[test]
    fn test_builder_with_topology() {
        let topology = ExchangeTopology::from_prefix("polygon.events");

        let config = ConsumerConfigBuilder::new()
            .with_topology(&topology)
            .queue("my.queue")
            .routing_key("my.key")
            .consumer_tag("my-consumer")
            .build_prefetch()
            .unwrap();

        assert_eq!(config.retry.base.exchange, "polygon.events");
        assert_eq!(config.retry.retry_exchange, "polygon.events.retry");
        assert_eq!(config.retry.dead_exchange, "polygon.events.dlx");
        assert_eq!(config.retry.retry_routing_key(), "__mq.retry.my.queue");
        assert_eq!(config.retry.dead_routing_key(), "__mq.dead.my.queue");
    }
}
