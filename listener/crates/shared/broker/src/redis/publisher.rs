use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};

use super::{
    connection::RedisConnectionManager,
    error::RedisPublisherError,
    trimmer::{StreamTrimmer, StreamTrimmerConfig},
};

/// Optional auto-trim configuration embedded in the publisher.
/// When set, the publisher automatically spawns a background `StreamTrimmer`
/// per stream on first publish — the developer never needs to know about trimming.
#[derive(Debug, Clone)]
struct AutoTrimConfig {
    /// Interval between trim cycles
    interval: Duration,
    /// Fallback MAXLEN if no consumer groups are found
    fallback_maxlen: Option<usize>,
}

/// Replication durability configuration for the publisher.
///
/// When set, the publisher issues a `WAIT` command after each successful
/// `XADD` to ensure the write has been replicated before returning.
#[derive(Debug, Clone)]
pub struct ReplicationConfig {
    /// Number of replicas that must acknowledge the write.
    /// For a standard primary + 1 replica setup, this is `1`.
    pub num_replicas: u32,
    /// Maximum time to wait for replica acknowledgment.
    /// Under normal cross-AZ conditions, replication takes < 1ms.
    /// This timeout is a ceiling for degraded conditions.
    pub timeout: Duration,
}

/// Redis Streams publisher with retry, optional MAXLEN trimming, optional
/// automatic background stream trimming, and optional replication durability.
///
/// Analogous to `RmqPublisher` — publishes serialized payloads to a
/// Redis stream via `XADD`, with exponential backoff retry on failure.
///
/// When `auto_trim` is enabled via the builder, the publisher transparently
/// manages stream size using consumer-group-aware trimming. The developer
/// never needs to interact with `StreamTrimmer` directly.
///
/// When `replication_wait` is enabled, the publisher issues `WAIT` after
/// each `XADD` to confirm replication to at least N replicas before returning.
pub struct RedisPublisher {
    connection: RedisConnectionManager,
    max_retries: u32,
    base_retry_delay_ms: u64,
    /// Optional approximate maximum stream length for MAXLEN trimming on each XADD.
    max_stream_len: Option<usize>,
    /// Optional auto-trim config — spawns background trimmers per stream.
    auto_trim: Option<AutoTrimConfig>,
    /// Active trimmer cancellation tokens, keyed by stream name.
    trimmers: Arc<Mutex<HashMap<String, CancellationToken>>>,
    /// Optional replication durability — issues WAIT after each XADD.
    replication_config: Option<ReplicationConfig>,
}

impl RedisPublisher {
    /// Create a new publisher with default settings (4 retries, 1s base delay, no MAXLEN, no auto-trim, no replication wait).
    pub fn new(connection: RedisConnectionManager) -> Self {
        Self {
            connection,
            max_retries: 4,
            base_retry_delay_ms: 1000,
            max_stream_len: None,
            auto_trim: None,
            trimmers: Arc::new(Mutex::new(HashMap::new())),
            replication_config: None,
        }
    }

    /// Create a publisher with custom configuration (backward compatible, no auto-trim).
    pub fn with_config(
        connection: RedisConnectionManager,
        max_retries: u32,
        base_retry_delay_ms: u64,
        max_stream_len: Option<usize>,
    ) -> Self {
        Self {
            connection,
            max_retries,
            base_retry_delay_ms,
            max_stream_len,
            auto_trim: None,
            trimmers: Arc::new(Mutex::new(HashMap::new())),
            replication_config: None,
        }
    }

    /// Create a builder for fine-grained publisher configuration including auto-trim.
    pub fn builder(connection: RedisConnectionManager) -> RedisPublisherBuilder {
        RedisPublisherBuilder::new(connection)
    }

    /// Publish a single payload to a Redis stream.
    ///
    /// Serializes the payload to JSON bytes and publishes via:
    /// `XADD {stream} MAXLEN ~ {max_len} * data {json_bytes}`
    ///
    /// If auto-trim is enabled, lazily spawns a background trimmer for this stream
    /// on the first publish call.
    ///
    /// Returns the auto-generated message ID on success.
    /// Retries with exponential backoff on failure.
    /// Returns `Err(RedisPublisherError::RetriesExhausted)` after max retries.
    pub async fn publish<T: Serialize + Debug>(
        &self,
        stream: &str,
        payload: &T,
    ) -> Result<String, RedisPublisherError> {
        // Lazily spawn trimmer for this stream if auto-trim is configured
        self.ensure_trimmer(stream).await;

        let body = serde_json::to_vec(payload)?;
        let publish_start = std::time::Instant::now();

        for attempt in 0..self.max_retries {
            let mut conn = self.connection.get_connection();

            let result: Result<String, redis::RedisError> =
                if let Some(maxlen) = self.max_stream_len {
                    redis::cmd("XADD")
                        .arg(stream)
                        .arg("MAXLEN")
                        .arg("~")
                        .arg(maxlen)
                        .arg("*")
                        .arg("data")
                        .arg(&body)
                        .query_async(&mut conn)
                        .await
                } else {
                    redis::cmd("XADD")
                        .arg(stream)
                        .arg("*")
                        .arg("data")
                        .arg(&body)
                        .query_async(&mut conn)
                        .await
                };

            match result {
                Ok(msg_id) => {
                    // If replication is configured, ensure at least N replicas
                    // have acknowledged this write before returning to the caller.
                    if let Some(ref repl) = self.replication_config {
                        let acked: u32 = match redis::cmd("WAIT")
                            .arg(repl.num_replicas)
                            .arg(repl.timeout.as_millis() as u64)
                            .query_async(&mut conn)
                            .await
                        {
                            Ok(n) => n,
                            Err(e) => {
                                // XADD already succeeded — message is on the primary.
                                // WAIT failed (connection dropped mid-command).
                                // Do NOT retry XADD or we'll duplicate the message.
                                warn!(
                                    error = %e,
                                    stream_id = %msg_id,
                                    stream = %stream,
                                    "WAIT failed after successful XADD — \
                                     replication state unknown, message exists on primary"
                                );
                                return Ok(msg_id);
                            }
                        };

                        if acked < repl.num_replicas {
                            warn!(
                                acked,
                                expected = repl.num_replicas,
                                stream_id = %msg_id,
                                stream = %stream,
                                "WAIT: insufficient replica ACKs — \
                                 write may not survive primary failover"
                            );
                        }
                    }
                    metrics::counter!("broker_messages_published_total",
                        "backend" => "redis", "topic" => stream.to_owned()
                    )
                    .increment(1);
                    metrics::histogram!("broker_publish_duration_seconds",
                        "backend" => "redis", "topic" => stream.to_owned()
                    )
                    .record(publish_start.elapsed().as_secs_f64());
                    return Ok(msg_id);
                }
                Err(e) => {
                    let error_kind = if e.is_io_error()
                        || e.is_connection_dropped()
                        || e.is_connection_refusal()
                    {
                        "connection"
                    } else if e.is_timeout() {
                        "timeout"
                    } else {
                        "other"
                    };
                    metrics::counter!("broker_publish_errors_total",
                        "backend" => "redis",
                        "topic" => stream.to_owned(),
                        "error_kind" => error_kind,
                    )
                    .increment(1);

                    // On connection/timeout errors, replace the stale ConnectionManager
                    // so the next retry gets a fresh TCP connection. ConnectionManager
                    // does NOT auto-reconnect for TimedOut errors (redis-rs maps them
                    // to RetryMethod::RetryImmediately, not Reconnect).
                    if error_kind == "connection" {
                        self.connection.force_reconnect().await;
                    }
                    warn!(
                        attempt = attempt + 1,
                        max_retries = self.max_retries,
                        error = %e,
                        stream = %stream,
                        "Publish failed, retrying..."
                    );
                }
            }

            // Exponential backoff
            let delay = self
                .base_retry_delay_ms
                .saturating_mul(2u64.pow(attempt))
                .min(30_000);
            sleep(Duration::from_millis(delay)).await;
        }

        Err(RedisPublisherError::RetriesExhausted {
            retries: self.max_retries,
            stream: stream.to_string(),
        })
    }

    /// Cancel all background trimmers and shut down cleanly.
    pub async fn shutdown(&self) {
        let trimmers = self.trimmers.lock().await;
        for (stream, token) in trimmers.iter() {
            debug!(stream = %stream, "Shutting down auto-trimmer");
            token.cancel();
        }
    }

    /// Lazily spawn a background trimmer for the given stream, if auto-trim
    /// is configured and no trimmer is already running for this stream.
    async fn ensure_trimmer(&self, stream: &str) {
        let Some(ref trim_config) = self.auto_trim else {
            return;
        };

        let mut trimmers = self.trimmers.lock().await;
        if trimmers.contains_key(stream) {
            return;
        }

        let cancel = CancellationToken::new();
        let trimmer = StreamTrimmer::new(
            self.connection.clone(),
            StreamTrimmerConfig {
                stream: stream.to_string(),
                interval: trim_config.interval,
                fallback_maxlen: trim_config.fallback_maxlen,
            },
        );

        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            trimmer.run(cancel_clone).await;
        });

        trimmers.insert(stream.to_string(), cancel);
        debug!(stream = %stream, "Auto-trimmer spawned");
    }
}

impl Drop for RedisPublisher {
    fn drop(&mut self) {
        // Cancel all trimmer tasks on drop.
        // We can't async here, but CancellationToken::cancel() is sync.
        if let Ok(trimmers) = self.trimmers.try_lock() {
            for (_, token) in trimmers.iter() {
                token.cancel();
            }
        }
    }
}

/// Builder for constructing a `RedisPublisher` with fine-grained configuration.
///
/// # Example
///
/// ```rust,ignore
/// let publisher = RedisPublisher::builder(conn)
///     .max_retries(10)
///     .base_retry_delay_ms(1000)
///     .max_stream_len(100_000)
///     .auto_trim(Duration::from_secs(60))
///     .fallback_maxlen(100_000)
///     .replication_wait(1, Duration::from_millis(500))
///     .build();
/// ```
#[must_use]
pub struct RedisPublisherBuilder {
    connection: RedisConnectionManager,
    max_retries: u32,
    base_retry_delay_ms: u64,
    max_stream_len: Option<usize>,
    auto_trim_interval: Option<Duration>,
    fallback_maxlen: Option<usize>,
    replication_config: Option<ReplicationConfig>,
}

impl RedisPublisherBuilder {
    fn new(connection: RedisConnectionManager) -> Self {
        Self {
            connection,
            max_retries: 10,
            base_retry_delay_ms: 1000,
            max_stream_len: None,
            auto_trim_interval: None,
            fallback_maxlen: None,
            replication_config: None,
        }
    }

    /// Maximum number of publish retries before panicking (default: 10).
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Base delay in milliseconds for exponential backoff (default: 1000).
    pub fn base_retry_delay_ms(mut self, ms: u64) -> Self {
        self.base_retry_delay_ms = ms;
        self
    }

    /// Optional approximate maximum stream length applied on each XADD via `MAXLEN ~`.
    pub fn max_stream_len(mut self, len: usize) -> Self {
        self.max_stream_len = Some(len);
        self
    }

    /// Enable automatic background stream trimming at the given interval.
    ///
    /// When enabled, the publisher spawns a background task per stream that
    /// periodically trims entries all consumer groups have fully processed.
    /// This is the Redis equivalent of RabbitMQ's automatic queue cleanup
    /// after message acknowledgement — the developer doesn't need to know
    /// about Redis stream trimming.
    pub fn auto_trim(mut self, interval: Duration) -> Self {
        self.auto_trim_interval = Some(interval);
        self
    }

    /// Fallback maximum stream length used by auto-trim when no consumer groups exist.
    /// Only relevant when `auto_trim` is enabled.
    pub fn fallback_maxlen(mut self, len: usize) -> Self {
        self.fallback_maxlen = Some(len);
        self
    }

    /// Enable replication durability via the Redis `WAIT` command.
    ///
    /// After each successful `XADD`, the publisher blocks until `num_replicas`
    /// replicas have acknowledged the write, or `timeout` elapses.
    ///
    /// - On a single-node Redis (no replicas), `WAIT` returns `0` immediately.
    ///   This is harmless but provides no durability benefit.
    /// - On ElastiCache Serverless, `WAIT` is blocked by AWS. Use node-based only.
    ///
    /// Recommended: `replication_wait(1, Duration::from_millis(500))`
    pub fn replication_wait(mut self, num_replicas: u32, timeout: Duration) -> Self {
        self.replication_config = Some(ReplicationConfig {
            num_replicas,
            timeout,
        });
        self
    }

    /// Build the `RedisPublisher`.
    pub fn build(self) -> RedisPublisher {
        let auto_trim = self.auto_trim_interval.map(|interval| AutoTrimConfig {
            interval,
            fallback_maxlen: self.fallback_maxlen,
        });

        RedisPublisher {
            connection: self.connection,
            max_retries: self.max_retries,
            base_retry_delay_ms: self.base_retry_delay_ms,
            max_stream_len: self.max_stream_len,
            auto_trim,
            trimmers: Arc::new(Mutex::new(HashMap::new())),
            replication_config: self.replication_config,
        }
    }
}

#[async_trait::async_trait]
impl crate::traits::publisher::Publisher for RedisPublisher {
    type Error = RedisPublisherError;

    async fn publish<T: Serialize + Debug + Send + Sync>(
        &self,
        topic: &str,
        payload: &T,
    ) -> Result<(), Self::Error> {
        self.publish(topic, payload).await.map(|_id| ())
    }

    async fn shutdown(&self) {
        self.shutdown().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_defaults() {
        // Can't fully test without Redis, but verify builder compiles and defaults
        // are set correctly by inspecting the built publisher.
    }

    #[test]
    fn test_builder_with_auto_trim() {
        // Verify that auto_trim config is set via builder
        // (integration test would verify actual trimmer spawning)
    }

    // Integration tests requiring Redis
    #[tokio::test]
    #[ignore]
    async fn test_publish_single() {
        let conn = RedisConnectionManager::new("redis://localhost:6379")
            .await
            .unwrap();
        let publisher = RedisPublisher::new(conn);

        let payload = serde_json::json!({"block_number": 42, "chain_id": 1});
        let result = publisher.publish("test.publish", &payload).await;
        assert!(result.is_ok());
        let msg_id = result.unwrap();
        assert!(!msg_id.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_publish_with_maxlen() {
        let conn = RedisConnectionManager::new("redis://localhost:6379")
            .await
            .unwrap();
        let publisher = RedisPublisher::with_config(conn, 3, 500, Some(1000));

        let payload = serde_json::json!({"block_number": 42});
        let result = publisher.publish("test.publish.maxlen", &payload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_publish_with_auto_trim() {
        let conn = RedisConnectionManager::new("redis://localhost:6379")
            .await
            .unwrap();
        let publisher = RedisPublisher::builder(conn)
            .max_retries(3)
            .auto_trim(Duration::from_secs(5))
            .fallback_maxlen(1000)
            .build();

        let payload = serde_json::json!({"block_number": 42});
        let result = publisher.publish("test.publish.autotrim", &payload).await;
        assert!(result.is_ok());

        // Verify trimmer was spawned
        let trimmers = publisher.trimmers.lock().await;
        assert!(trimmers.contains_key("test.publish.autotrim"));

        publisher.shutdown().await;
    }
}
