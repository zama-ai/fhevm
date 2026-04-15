use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::{future::Future, marker::PhantomData, time::Duration};
use thiserror::Error;

use super::message::{Message, MessageMetadata};

/// Explicit routing decision returned by a handler on the success path.
///
/// Combined with [`HandlerError`] variants on the `Err` arm, this gives full
/// control over the message lifecycle without side effects on the [`Message`]
/// itself — the handler declares intent via the return type.
///
/// # Per-backend semantics
///
/// | Variant | AMQP (RabbitMQ) | Redis Streams |
/// |---|---|---|
/// | `Ack` | `basic_ack` | `XACK` |
/// | `Nack` | `basic_nack(requeue: true)` → back to main queue | Leave in PEL (ClaimSweeper handles retry) |
/// | `Dead` | Publish to DLQ directly, then `basic_ack` | `XADD` to dead stream + `XACK` |
/// | `Delay(d)` | Publish to retry exchange with per-message `expiration` TTL, then `basic_ack` | Leave in PEL (no per-message TTL in Redis Streams; ClaimSweeper `claim_min_idle` governs timing) |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AckDecision {
    /// Remove from queue — processing succeeded.
    Ack,
    /// Requeue to main queue (AMQP) / leave in PEL (Redis) — voluntary yield.
    ///
    /// Use when you want to requeue deliberately (e.g. idempotency guard saw
    /// another consumer already handling this event) without indicating an error.
    /// Does NOT trip the circuit breaker. Does NOT increment the retry counter.
    Nack,
    /// Skip all retries — route directly to the dead-letter queue / dead stream.
    ///
    /// Use for permanently unprocessable messages: unknown schema version, invalid
    /// ABI encoding, malformed payload that can never succeed on retry.
    Dead,
    /// Requeue with a custom delay before the next attempt.
    ///
    /// **AMQP**: publishes to the retry exchange with a per-message `expiration`
    /// property, overriding the queue-level TTL.
    ///
    /// **Redis**: no per-message TTL support in Redis Streams — treated as `Nack`
    /// (message stays in PEL; ClaimSweeper `claim_min_idle` governs when it is reclaimed).
    Delay(Duration),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum HandlerError {
    #[error("deserialization failed: {0}")]
    Deserialization(#[from] serde_json::Error),

    #[error("handler execution failed: {0}")]
    Execution(#[source] Box<dyn std::error::Error + Send + Sync>),

    /// Transient (infrastructure) failure — not the message's fault.
    /// Triggers the circuit breaker when configured on the consumer.
    #[error("transient failure (infrastructure): {0}")]
    Transient(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl HandlerError {
    /// Wrap an infrastructure error as `Transient`.
    ///
    /// Use this for failures that are not the message's fault — database
    /// connection lost, external API timeout, network error, etc.
    /// These trip the circuit breaker when configured on the consumer.
    ///
    /// Works with `?` via `.map_err`:
    ///
    /// ```rust,ignore
    /// db.save(&event).await.map_err(HandlerError::transient)?;
    /// api.call().await.map_err(HandlerError::transient)?;
    /// ```
    pub fn transient(e: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Transient(Box::new(e))
    }

    /// Wrap a logic error as permanent (`Execution`).
    ///
    /// Use this for failures that are the message's fault — invalid data,
    /// business rule violation, ABI decoding error, etc.
    /// These reset the circuit breaker's transient counter.
    ///
    /// Works with `?` via `.map_err`:
    ///
    /// ```rust,ignore
    /// validate(&event).map_err(HandlerError::permanent)?;
    /// abi_decode(&log).map_err(HandlerError::permanent)?;
    /// ```
    pub fn permanent(e: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Execution(Box::new(e))
    }
}

/// Backend-agnostic classification of a handler result.
///
/// Used by prefetch-safe consumers to carry the outcome through the mpsc
/// channel without losing the routing decision that the circuit breaker
/// and ACK/NACK logic depend on.
///
/// Constructed via `From<Result<AckDecision, HandlerError>>` — consumers
/// never build this directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerOutcome {
    /// Handler succeeded and wants to ACK.
    Ack,
    /// Handler succeeded but voluntarily requeues (no circuit breaker impact).
    Nack,
    /// Handler wants immediate dead-letter routing, bypassing retries.
    Dead,
    /// Handler wants a custom delay before the next attempt.
    Delay(Duration),
    /// Infrastructure failure — increments the circuit breaker's transient counter.
    Transient,
    /// Logic or deserialization failure — resets the transient counter.
    Permanent,
}

impl From<Result<AckDecision, HandlerError>> for HandlerOutcome {
    fn from(result: Result<AckDecision, HandlerError>) -> Self {
        match result {
            Ok(AckDecision::Ack) => Self::Ack,
            Ok(AckDecision::Nack) => Self::Nack,
            Ok(AckDecision::Dead) => Self::Dead,
            Ok(AckDecision::Delay(d)) => Self::Delay(d),
            Err(HandlerError::Transient(_)) => Self::Transient,
            Err(_) => Self::Permanent,
        }
    }
}

/// Queue-agnostic message handler trait.
///
/// Both RMQ and Redis consumers call `handler.call(&msg)` where `msg` is a
/// backend-constructed [`Message`]. The handler never knows which backend
/// produced the message.
///
/// Returns `Ok(AckDecision)` to explicitly control the message routing on the
/// success path, or `Err(HandlerError)` to signal failure semantics.
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError>;
}

/// Handler wrapper that ignores the message payload and calls `F()`.
///
/// Used for trigger/signal consumers where the message is just a wake-up
/// signal and the handler does not need any data from the payload.
pub struct AsyncHandlerNoArgs<F, E> {
    f: F,
    _phantom: PhantomData<E>,
}

impl<F: Clone, E> Clone for AsyncHandlerNoArgs<F, E> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<F, E> AsyncHandlerNoArgs<F, E> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<F, Fut, E> Handler for AsyncHandlerNoArgs<F, E>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send,
    E: std::error::Error + Send + Sync + 'static,
{
    async fn call(&self, _msg: &Message) -> Result<AckDecision, HandlerError> {
        (self.f)()
            .await
            .map(|_| AckDecision::Ack)
            .map_err(|e| HandlerError::Execution(Box::new(e)))
    }
}

/// Handler wrapper that deserializes `msg.payload` to `T` and calls `F(T)`.
///
/// This is the most common handler style — the handler only cares about the
/// deserialized payload and does not need delivery metadata.
///
/// The user-supplied closure returns `Result<(), E>` for ergonomics. The
/// wrapper maps `Ok(())` to `Ok(AckDecision::Ack)` automatically. To return
/// a different `AckDecision`, use a closure that returns `Result<AckDecision, E>`
/// and implement [`Handler`] directly, or use [`AsyncHandlerWithContext`].
///
/// Replaces both `broker::AsyncHandlerWithArgs` and
/// `redis_broker::AsyncRedisHandlerPayloadOnly`.
pub struct AsyncHandlerPayloadOnly<F, T, E> {
    f: F,
    _phantom: PhantomData<(T, E)>,
}

impl<F: Clone, T, E> Clone for AsyncHandlerPayloadOnly<F, T, E> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<F, T, E> AsyncHandlerPayloadOnly<F, T, E> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<F, Fut, T, E> Handler for AsyncHandlerPayloadOnly<F, T, E>
where
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send,
    T: DeserializeOwned + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let payload: T = serde_json::from_slice(&msg.payload)?;
        (self.f)(payload)
            .await
            .map(|_| AckDecision::Ack)
            .map_err(|e| HandlerError::Execution(Box::new(e)))
    }
}

/// Handler wrapper that deserializes `msg.payload` to `T` and calls `F(T)`,
/// where the closure returns `Result<(), HandlerError>` directly.
///
/// Unlike [`AsyncHandlerPayloadOnly`] (which wraps all closure errors as
/// `HandlerError::Execution`), this handler **preserves** the error classification
/// returned by the closure. Use this when your handler needs to distinguish
/// transient (infrastructure) from permanent (logic) failures:
///
/// ```rust,ignore
/// use broker::{AsyncHandlerPayloadClassified, HandlerError};
///
/// let handler = AsyncHandlerPayloadClassified::new(|block: BlockEvent| async move {
///     // Transient: infrastructure is broken, not the message.
///     db.save(&block).await.map_err(HandlerError::transient)?;
///
///     // Permanent: the message itself is invalid.
///     verify_block(&block).map_err(HandlerError::permanent)?;
///
///     Ok(())
/// });
/// ```
pub struct AsyncHandlerPayloadClassified<F, T> {
    f: F,
    _phantom: PhantomData<T>,
}

impl<F: Clone, T> Clone for AsyncHandlerPayloadClassified<F, T> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<F, T> AsyncHandlerPayloadClassified<F, T> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<F, Fut, T> Handler for AsyncHandlerPayloadClassified<F, T>
where
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), HandlerError>> + Send,
    T: DeserializeOwned + Send + Sync + 'static,
{
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let payload: T = serde_json::from_slice(&msg.payload)?;
        (self.f)(payload).await.map(|_| AckDecision::Ack)
    }
}

/// Handler wrapper that deserializes `msg.payload` to `T` and calls
/// `F(T, MessageMetadata)`.
///
/// Use this when the handler needs delivery metadata such as the message ID,
/// topic, or delivery count. The metadata is backend-agnostic.
///
/// Replaces `redis_broker::AsyncRedisHandlerWithArgs` (which took
/// `RedisMessageContext` — now [`MessageMetadata`]).
pub struct AsyncHandlerWithContext<F, T, E> {
    f: F,
    _phantom: PhantomData<(T, E)>,
}

impl<F: Clone, T, E> Clone for AsyncHandlerWithContext<F, T, E> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<F, T, E> AsyncHandlerWithContext<F, T, E> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<F, Fut, T, E> Handler for AsyncHandlerWithContext<F, T, E>
where
    F: Fn(T, MessageMetadata) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send,
    T: DeserializeOwned + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let payload: T = serde_json::from_slice(&msg.payload)?;
        (self.f)(payload, msg.metadata.clone())
            .await
            .map(|_| AckDecision::Ack)
            .map_err(|e| HandlerError::Execution(Box::new(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use thiserror::Error as ThisError;

    #[derive(ThisError, Debug)]
    #[error("Mock error: {0}")]
    struct MockError(String);

    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct TestPayload {
        value: i32,
    }

    fn make_message(data: &[u8]) -> Message {
        Message {
            payload: data.to_vec(),
            metadata: MessageMetadata::new("test-id-123", "test.topic", 1),
        }
    }

    // ── AsyncHandlerPayloadOnly tests ──────────────────────────

    #[tokio::test]
    async fn payload_only_success() {
        let handler = AsyncHandlerPayloadOnly::new(|payload: TestPayload| async move {
            assert_eq!(payload.value, 42);
            Ok::<(), MockError>(())
        });

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn payload_only_deserialization_error() {
        let handler =
            AsyncHandlerPayloadOnly::new(|_p: TestPayload| async move { Ok::<(), MockError>(()) });

        let msg = make_message(br#"{"invalid": "json"}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HandlerError::Deserialization(_)));
        assert!(err.to_string().contains("deserialization failed"));
    }

    #[tokio::test]
    async fn payload_only_execution_error() {
        let handler = AsyncHandlerPayloadOnly::new(|_p: TestPayload| async move {
            Err::<(), MockError>(MockError("handler failed".to_string()))
        });

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HandlerError::Execution(_)));
        assert!(err.to_string().contains("handler execution failed"));

        let source = err.source();
        assert!(source.is_some());
        assert!(source.unwrap().to_string().contains("handler failed"));
    }

    // ── AsyncHandlerWithContext tests ──────────────────────────

    #[tokio::test]
    async fn with_context_success() {
        let handler =
            AsyncHandlerWithContext::new(|payload: TestPayload, ctx: MessageMetadata| async move {
                assert_eq!(payload.value, 42);
                assert_eq!(ctx.id, "test-id-123");
                assert_eq!(ctx.topic, "test.topic");
                assert_eq!(ctx.delivery_count, 1);
                Ok::<(), MockError>(())
            });

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn with_context_deserialization_error() {
        let handler =
            AsyncHandlerWithContext::new(|_p: TestPayload, _ctx: MessageMetadata| async move {
                Ok::<(), MockError>(())
            });

        let msg = make_message(br#"{"bad": "data"}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HandlerError::Deserialization(_)
        ));
    }

    #[tokio::test]
    async fn with_context_execution_error() {
        let handler =
            AsyncHandlerWithContext::new(|_p: TestPayload, _ctx: MessageMetadata| async move {
                Err::<(), MockError>(MockError("ctx handler failed".to_string()))
            });

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HandlerError::Execution(_)));

        let source = err.source();
        assert!(source.is_some());
        assert!(source.unwrap().to_string().contains("ctx handler failed"));
    }

    // ── AsyncHandlerNoArgs tests ──────────────────────────

    #[tokio::test]
    async fn no_args_success() {
        let handler = AsyncHandlerNoArgs::new(|| async move { Ok::<(), MockError>(()) });

        let msg = make_message(b"anything");
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn no_args_ignores_payload() {
        let handler = AsyncHandlerNoArgs::new(|| async move { Ok::<(), MockError>(()) });

        // Even invalid JSON should succeed — payload is ignored
        let msg = make_message(b"not json at all!!!");
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn no_args_execution_error() {
        let handler = AsyncHandlerNoArgs::new(|| async move {
            Err::<(), MockError>(MockError("handler failed".to_string()))
        });

        let msg = make_message(b"{}");
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HandlerError::Execution(_)));
        assert!(err.to_string().contains("handler execution failed"));

        let source = err.source();
        assert!(source.is_some());
        assert!(source.unwrap().to_string().contains("handler failed"));
    }

    // ── HandlerError::Transient tests ──────────────────────────

    #[tokio::test]
    async fn transient_error_variant() {
        let err = HandlerError::Transient(Box::new(MockError("db down".to_string())));
        assert!(matches!(err, HandlerError::Transient(_)));
        assert!(err.to_string().contains("transient failure"));

        let source = err.source();
        assert!(source.is_some());
        assert!(source.unwrap().to_string().contains("db down"));
    }

    #[tokio::test]
    async fn transient_vs_execution_are_distinct() {
        let transient = HandlerError::Transient(Box::new(MockError("infra".to_string())));
        let execution = HandlerError::Execution(Box::new(MockError("logic".to_string())));

        assert!(matches!(transient, HandlerError::Transient(_)));
        assert!(!matches!(transient, HandlerError::Execution(_)));

        assert!(matches!(execution, HandlerError::Execution(_)));
        assert!(!matches!(execution, HandlerError::Transient(_)));
    }

    // ── Convenience constructor tests ──────────────────────────

    #[test]
    fn transient_constructor_wraps_error() {
        let err = HandlerError::transient(MockError("db down".to_string()));
        assert!(matches!(err, HandlerError::Transient(_)));
        assert!(err.to_string().contains("transient failure"));
    }

    #[test]
    fn permanent_constructor_wraps_error() {
        let err = HandlerError::permanent(MockError("bad data".to_string()));
        assert!(matches!(err, HandlerError::Execution(_)));
        assert!(err.to_string().contains("handler execution failed"));
    }

    // ── AckDecision / HandlerOutcome tests ─────────────────────

    #[test]
    fn handler_outcome_from_ack() {
        let outcome = HandlerOutcome::from(Ok::<AckDecision, HandlerError>(AckDecision::Ack));
        assert_eq!(outcome, HandlerOutcome::Ack);
    }

    #[test]
    fn handler_outcome_from_nack() {
        let outcome = HandlerOutcome::from(Ok::<AckDecision, HandlerError>(AckDecision::Nack));
        assert_eq!(outcome, HandlerOutcome::Nack);
    }

    #[test]
    fn handler_outcome_from_dead() {
        let outcome = HandlerOutcome::from(Ok::<AckDecision, HandlerError>(AckDecision::Dead));
        assert_eq!(outcome, HandlerOutcome::Dead);
    }

    #[test]
    fn handler_outcome_from_delay() {
        let d = Duration::from_secs(10);
        let outcome = HandlerOutcome::from(Ok::<AckDecision, HandlerError>(AckDecision::Delay(d)));
        assert_eq!(outcome, HandlerOutcome::Delay(d));
    }

    #[test]
    fn handler_outcome_from_transient_err() {
        let outcome = HandlerOutcome::from(Err::<AckDecision, HandlerError>(
            HandlerError::Transient(Box::new(MockError("infra".to_string()))),
        ));
        assert_eq!(outcome, HandlerOutcome::Transient);
    }

    #[test]
    fn handler_outcome_from_permanent_err() {
        let outcome = HandlerOutcome::from(Err::<AckDecision, HandlerError>(
            HandlerError::Execution(Box::new(MockError("logic".to_string()))),
        ));
        assert_eq!(outcome, HandlerOutcome::Permanent);
    }

    // ── AsyncHandlerPayloadClassified tests ───────────────────

    #[tokio::test]
    async fn classified_success() {
        let handler = AsyncHandlerPayloadClassified::new(|payload: TestPayload| async move {
            assert_eq!(payload.value, 42);
            Ok(())
        });

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn classified_deserialization_error() {
        let handler = AsyncHandlerPayloadClassified::new(|_p: TestPayload| async move { Ok(()) });

        let msg = make_message(br#"{"invalid": "json"}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HandlerError::Deserialization(_)
        ));
    }

    #[tokio::test]
    async fn classified_transient_error_preserved() {
        let handler = AsyncHandlerPayloadClassified::new(|_p: TestPayload| async move {
            Err(HandlerError::transient(MockError("db down".to_string())))
        });

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, HandlerError::Transient(_)),
            "expected Transient, got {err:?}"
        );
    }

    #[tokio::test]
    async fn classified_permanent_error_preserved() {
        let handler = AsyncHandlerPayloadClassified::new(|_p: TestPayload| async move {
            Err(HandlerError::permanent(MockError("bad block".to_string())))
        });

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, HandlerError::Execution(_)),
            "expected Execution, got {err:?}"
        );
    }
}
