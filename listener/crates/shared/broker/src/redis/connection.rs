use arc_swap::ArcSwap;
use redis::aio::{ConnectionManager, ConnectionManagerConfig};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

use super::error::RedisConsumerError;

/// Per-command timeout applied to every `ConnectionManager`.
///
/// Prevents indefinite hangs on half-open TCP sockets where the OS
/// TCP retransmission timeout (~75s macOS, ~15min Linux) is the only
/// backstop.  Applied at the `ConnectionManager` level so it covers
/// **all** commands (XREADGROUP, XADD, XACK, HSET, …), not just the
/// consumer's read path.
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(5);

/// Timeout for the initial TCP handshake when creating a connection.
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);

/// Manages Redis connections with automatic reconnection.
///
/// Wraps `redis::aio::ConnectionManager` which provides built-in
/// reconnection semantics. Each clone shares the same underlying
/// connection, and the connection is automatically re-established
/// if it drops.
///
/// Additionally exposes `force_reconnect()` for callers that detect
/// dead sockets via external timeouts — `ConnectionManager` only
/// triggers internal reconnection for a subset of IO error kinds
/// (not `TimedOut`), so an external nudge is needed.
#[derive(Clone)]
pub struct RedisConnectionManager {
    url: String,
    inner: Arc<ArcSwap<ConnectionManager>>,
    reconnect_delay: Duration,
}

impl RedisConnectionManager {
    /// Create a new RedisConnectionManager with the given URL.
    pub async fn new(url: &str) -> Result<Self, RedisConsumerError> {
        let client = redis::Client::open(url).map_err(RedisConsumerError::Connection)?;
        let manager = ConnectionManager::new_with_config(client, Self::cm_config())
            .await
            .map_err(RedisConsumerError::Connection)?;

        info!("RedisConnectionManager: Connected to Redis at {}", url);

        Ok(Self {
            url: url.to_string(),
            inner: Arc::new(ArcSwap::from_pointee(manager)),
            reconnect_delay: Duration::from_secs(5),
        })
    }

    /// Create a new RedisConnectionManager with custom reconnect delay.
    pub async fn with_reconnect_delay(
        url: &str,
        reconnect_delay: Duration,
    ) -> Result<Self, RedisConsumerError> {
        let mut mgr = Self::new(url).await?;
        mgr.reconnect_delay = reconnect_delay;
        Ok(mgr)
    }

    /// Get the Redis URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get a cloneable connection handle.
    ///
    /// `redis::aio::ConnectionManager` is cheaply cloneable and
    /// automatically reconnects on failure, so this simply clones
    /// the internal handle.
    pub fn get_connection(&self) -> ConnectionManager {
        let guard = self.inner.load();
        (**guard).clone()
    }

    /// Replace the inner `ConnectionManager` with a fresh one.
    ///
    /// Called after detecting a dead socket via command timeout.
    /// `ConnectionManager` only triggers its own reconnection for
    /// specific IO error kinds (`BrokenPipe`, `ConnectionReset`, etc.)
    /// but NOT for `TimedOut` — so we must create a new one.
    pub async fn force_reconnect(&self) {
        let client = match redis::Client::open(self.url.as_str()) {
            Ok(c) => c,
            Err(e) => {
                warn!(error = %e, "force_reconnect: failed to open Redis client");
                return;
            }
        };

        match ConnectionManager::new_with_config(client, Self::cm_config()).await {
            Ok(new_manager) => {
                self.inner.store(Arc::new(new_manager));
                info!(url = %self.url, "force_reconnect: replaced ConnectionManager with fresh connection");
            }
            Err(e) => {
                warn!(error = %e, "force_reconnect: failed to create new ConnectionManager, will retry on next command");
            }
        }
    }

    /// Perform a health check via PING command.
    pub async fn health_check(&self) -> Result<(), RedisConsumerError> {
        let mut conn = self.get_connection();
        redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
            .map_err(RedisConsumerError::Connection)?;
        Ok(())
    }

    /// Shared `ConnectionManagerConfig` used by both `new()` and `force_reconnect()`.
    ///
    /// `number_of_retries(0)` disables ConnectionManager's internal reconnection
    /// retry loop. Without this, a broken connection triggers a shared future that
    /// retries with exponential backoff (default 6 attempts × connection_timeout),
    /// blocking ALL commands for 30+ seconds. We handle reconnection ourselves
    /// via `force_reconnect()` which atomically swaps in a fresh ConnectionManager.
    fn cm_config() -> ConnectionManagerConfig {
        ConnectionManagerConfig::new()
            .set_response_timeout(RESPONSE_TIMEOUT)
            .set_connection_timeout(CONNECTION_TIMEOUT)
            .set_number_of_retries(0)
    }

    /// Create a connection with infinite retry on failure.
    /// Use this when the consumer must eventually succeed.
    pub async fn new_with_retry(url: &str) -> Self {
        loop {
            match Self::new(url).await {
                Ok(mgr) => return mgr,
                Err(e) => {
                    error!(
                        "Failed to connect to Redis at {}: {}. Retrying in 5s...",
                        url, e
                    );
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_manager_url() {
        // We can't easily test the async constructor without a Redis server,
        // so we test the basic properties after construction in integration tests.
        // This test verifies the module compiles correctly.
    }

    #[tokio::test]
    #[ignore]
    async fn test_connection_manager_new() {
        let manager = RedisConnectionManager::new("redis://localhost:6379")
            .await
            .unwrap();
        assert_eq!(manager.url(), "redis://localhost:6379");
    }

    #[tokio::test]
    #[ignore]
    async fn test_connection_manager_health_check() {
        let manager = RedisConnectionManager::new("redis://localhost:6379")
            .await
            .unwrap();
        let result = manager.health_check().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_connection_manager_with_custom_delay() {
        let manager = RedisConnectionManager::with_reconnect_delay(
            "redis://localhost:6379",
            Duration::from_secs(10),
        )
        .await
        .unwrap();
        assert_eq!(manager.reconnect_delay, Duration::from_secs(10));
    }
}
