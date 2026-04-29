use async_trait::async_trait;
use std::time::Duration;

use super::handler::Handler;

/// Exposes the common retry fields that all backend-specific retry configs share.
///
/// Useful for logging, monitoring, or generic code that inspects retry policy
/// without caring which backend is in use.
pub trait RetryPolicy {
    fn max_retries(&self) -> u32;
    fn retry_delay(&self) -> Duration;
}

/// Queue-agnostic consumer trait.
///
/// Both `RmqConsumer` and `RedisConsumer` implement this trait, allowing
/// application code to be generic over the backend.
///
/// Each backend brings its own prefetch-safe config type via the associated
/// type — retry semantics (DLX vs XCLAIM) remain backend-specific, but the
/// run method shares the same signature.
#[async_trait]
pub trait Consumer: Send + Sync + Sized {
    /// Config for `run` — retry + throughput tuning.
    type PrefetchConfig: Send;
    /// Backend-specific error type.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Connect to the backend and return a ready-to-use consumer.
    async fn connect(url: &str) -> Result<Self, Self::Error>;

    /// Run a high-throughput consumer with strong message loss guarantees.
    async fn run(
        &self,
        config: Self::PrefetchConfig,
        handler: impl Handler + 'static,
    ) -> Result<(), Self::Error>;
}
