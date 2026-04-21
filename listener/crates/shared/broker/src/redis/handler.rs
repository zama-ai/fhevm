pub use crate::traits::handler::{
    AckDecision, AsyncHandlerPayloadOnly, AsyncHandlerWithContext, Handler, HandlerError,
};
pub use crate::traits::message::{Message, MessageMetadata};

// Backward-compatible aliases for existing redis_broker callers
#[allow(dead_code)]
pub type RedisHandler = dyn Handler;
#[allow(dead_code)]
pub type RedisHandlerError = HandlerError;

/// Backward-compatible alias — use `AsyncHandlerPayloadOnly` directly for new code.
pub type AsyncRedisHandlerPayloadOnly<F, T, E> = AsyncHandlerPayloadOnly<F, T, E>;

/// Backward-compatible alias — use `AsyncHandlerWithContext` directly for new code.
/// The handler receives `MessageMetadata` instead of the old `RedisMessageContext`.
pub type AsyncRedisHandlerWithArgs<F, T, E> = AsyncHandlerWithContext<F, T, E>;

/// Backward-compatible wrapper that ignores both payload and receives context.
///
/// For new code, prefer `AsyncHandlerPayloadOnly` with an empty-payload handler.
#[derive(Clone)]
pub struct AsyncRedisHandlerNoArgs<F, E> {
    f: F,
    _phantom: std::marker::PhantomData<E>,
}

impl<F, E> AsyncRedisHandlerNoArgs<F, E> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<F, Fut, E> Handler for AsyncRedisHandlerNoArgs<F, E>
where
    F: Fn(MessageMetadata) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<(), E>> + Send,
    E: std::error::Error + Send + Sync + 'static,
{
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        (self.f)(msg.metadata.clone())
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
            metadata: MessageMetadata::new("1234567890-0", "test.stream", 1),
        }
    }

    #[tokio::test]
    async fn async_handler_with_args_success() {
        let handler = AsyncRedisHandlerWithArgs::new(
            |payload: TestPayload, _ctx: MessageMetadata| async move {
                assert_eq!(payload.value, 42);
                Ok::<(), MockError>(())
            },
        );

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn async_handler_with_args_receives_context() {
        let handler = AsyncRedisHandlerWithArgs::new(
            |_payload: TestPayload, ctx: MessageMetadata| async move {
                assert_eq!(ctx.id, "1234567890-0");
                assert_eq!(ctx.topic, "test.stream");
                assert_eq!(ctx.delivery_count, 1);
                Ok::<(), MockError>(())
            },
        );

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn async_handler_with_args_deserialization_error() {
        let handler = AsyncRedisHandlerWithArgs::new(
            |_payload: TestPayload, _ctx: MessageMetadata| async move { Ok::<(), MockError>(()) },
        );

        let msg = make_message(br#"{"invalid": "json"}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HandlerError::Deserialization(_)));
        assert!(err.to_string().contains("deserialization failed"));
    }

    #[tokio::test]
    async fn async_handler_with_args_execution_error() {
        let handler = AsyncRedisHandlerWithArgs::new(
            |_payload: TestPayload, _ctx: MessageMetadata| async move {
                Err::<(), MockError>(MockError("handler failed".to_string()))
            },
        );

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

    #[tokio::test]
    async fn async_handler_no_args_success() {
        let handler =
            AsyncRedisHandlerNoArgs::new(
                |_ctx: MessageMetadata| async move { Ok::<(), MockError>(()) },
            );

        let msg = make_message(&[]);
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn async_handler_no_args_receives_context() {
        let handler = AsyncRedisHandlerNoArgs::new(|ctx: MessageMetadata| async move {
            assert_eq!(ctx.id, "1234567890-0");
            assert_eq!(ctx.topic, "test.stream");
            Ok::<(), MockError>(())
        });

        let msg = make_message(&[]);
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn async_handler_no_args_execution_error() {
        let handler = AsyncRedisHandlerNoArgs::new(|_ctx: MessageMetadata| async move {
            Err::<(), MockError>(MockError("no args handler failed".to_string()))
        });

        let msg = make_message(&[]);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HandlerError::Execution(_)));

        let source = err.source();
        assert!(source.is_some());
        assert!(
            source
                .unwrap()
                .to_string()
                .contains("no args handler failed")
        );
    }

    #[tokio::test]
    async fn payload_only_handler_success() {
        let handler = AsyncRedisHandlerPayloadOnly::new(|payload: TestPayload| async move {
            assert_eq!(payload.value, 42);
            Ok::<(), MockError>(())
        });

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn payload_only_handler_deserialization_error() {
        let handler = AsyncRedisHandlerPayloadOnly::new(|_payload: TestPayload| async move {
            Ok::<(), MockError>(())
        });

        let msg = make_message(br#"{"invalid": "json"}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HandlerError::Deserialization(_)));
    }

    #[tokio::test]
    async fn payload_only_handler_execution_error() {
        let handler = AsyncRedisHandlerPayloadOnly::new(|_payload: TestPayload| async move {
            Err::<(), MockError>(MockError("payload only failed".to_string()))
        });

        let msg = make_message(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HandlerError::Execution(_)));

        let source = err.source();
        assert!(source.is_some());
        assert!(source.unwrap().to_string().contains("payload only failed"));
    }

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
}
