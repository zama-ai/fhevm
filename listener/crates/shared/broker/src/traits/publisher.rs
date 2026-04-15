use std::{fmt::Debug, future::Future, pin::Pin, sync::Arc};

use async_trait::async_trait;
use serde::Serialize;

/// Queue-agnostic publisher trait.
///
/// Both `RmqPublisher` and `RedisPublisher` implement this trait.
///
/// - `topic` maps to a routing key (RMQ, with the exchange fixed at construction)
///   or a stream name (Redis).
/// - `shutdown` has a default no-op; Redis overrides it to cancel background trimmers.
#[async_trait]
pub trait Publisher: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Publish a single payload to the given topic.
    async fn publish<T: Serialize + Debug + Send + Sync>(
        &self,
        topic: &str,
        payload: &T,
    ) -> Result<(), Self::Error>;

    /// Graceful shutdown — cancel background tasks if any.
    ///
    /// Default is a no-op. Redis overrides to cancel background trimmers.
    async fn shutdown(&self) {}
}

// ── Type erasure internals ────────────────────────────────────────────────────

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Erased error type used by `DynPublisher`.
pub type DynPublishError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Internal dyn-safe trait. Uses `&[u8]` instead of `&T: Serialize` so Rust
/// can build a vtable for it.
trait ErasedPublisher: Send + Sync + 'static {
    fn publish_bytes<'a>(
        &'a self,
        topic: &'a str,
        payload: &'a [u8],
    ) -> BoxFuture<'a, Result<(), DynPublishError>>;

    fn shutdown<'a>(&'a self) -> BoxFuture<'a, ()>;
}

/// Blanket impl: every `Publisher` is automatically an `ErasedPublisher`.
///
/// Serialization (T → JSON bytes) happens here, before calling the inner
/// backend, so the backend only ever sees `&[u8]` — no generics cross the
/// vtable boundary.
impl<P: Publisher> ErasedPublisher for P {
    fn publish_bytes<'a>(
        &'a self,
        topic: &'a str,
        payload: &'a [u8],
    ) -> BoxFuture<'a, Result<(), DynPublishError>> {
        Box::pin(async move {
            // Treat the bytes as an already-serialized JSON value so the
            // backend re-encodes them without double-serialization.
            let raw: &serde_json::value::RawValue =
                serde_json::from_slice(payload).map_err(|e| Box::new(e) as DynPublishError)?;
            self.publish(topic, &raw)
                .await
                .map_err(|e| Box::new(e) as DynPublishError)
        })
    }

    fn shutdown<'a>(&'a self) -> BoxFuture<'a, ()> {
        Box::pin(Publisher::shutdown(self))
    }
}

// ── Public type-erased handle ─────────────────────────────────────────────────

/// A type-erased, cheaply cloneable publisher that can be stored alongside
/// publishers of different backends in a `HashMap` or `Vec`.
///
/// Wraps any `impl Publisher` and serializes payloads to JSON bytes before
/// delegating to the underlying backend. The `Error` type is erased to
/// [`DynPublishError`] (`Box<dyn Error + Send + Sync>`).
///
/// # Multi-chain registry example
///
/// ```rust,ignore
/// use mq::DynPublisher;
/// use mq_amqp::RmqPublisher;
/// use mq_redis::RedisPublisher;
/// use std::collections::HashMap;
///
/// // Construction — backend-specific, done once at startup
/// let mut publishers: HashMap<String, DynPublisher> = HashMap::new();
///
/// publishers.insert(
///     "ethereum".into(),
///     DynPublisher::new(RmqPublisher::connect(url, "ethereum.events").await),
/// );
/// publishers.insert(
///     "polygon".into(),
///     DynPublisher::new(RedisPublisher::connect("redis://...").await?),
/// );
///
/// // Usage — fully agnostic, same call for any backend
/// publishers["ethereum"].publish("blocks.new", &block).await?;
/// publishers["polygon"].publish("blocks.new", &block).await?;
/// ```
#[derive(Clone)]
pub struct DynPublisher(Arc<dyn ErasedPublisher>);

impl DynPublisher {
    /// Wrap any `impl Publisher` in a type-erased handle.
    ///
    /// The resulting `DynPublisher` is `Clone` and `Send + Sync`.
    pub fn new(publisher: impl Publisher) -> Self {
        Self(Arc::new(publisher))
    }

    /// Publish a single payload — serializes to JSON then delegates to the backend.
    ///
    /// `topic` maps to a routing key (AMQP) or stream name (Redis), exactly as
    /// it does on the underlying `Publisher` trait.
    pub async fn publish<T: Serialize + Debug + Send + Sync>(
        &self,
        topic: &str,
        payload: &T,
    ) -> Result<(), DynPublishError> {
        let bytes = serde_json::to_vec(payload).map_err(|e| Box::new(e) as DynPublishError)?;
        self.0.publish_bytes(topic, &bytes).await
    }

    /// Graceful shutdown — delegates to the backend.
    ///
    /// For Redis, this cancels background stream trimmers. For AMQP, this is a no-op.
    pub async fn shutdown(&self) {
        self.0.shutdown().await;
    }
}
