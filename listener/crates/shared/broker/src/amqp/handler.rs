pub use crate::traits::handler::{AckDecision, AsyncHandlerPayloadOnly, Handler, HandlerError};
use crate::traits::message::Message;

use async_trait::async_trait;
use std::{future::Future, marker::PhantomData};

/// Backward-compatible alias: `AsyncHandlerWithArgs` is now
/// `AsyncHandlerPayloadOnly` from the common module.
pub type AsyncHandlerWithArgs<F, T, E> = AsyncHandlerPayloadOnly<F, T, E>;

/// Handler wrapper that ignores the message payload and calls `F()`.
///
/// Kept for backward compatibility with existing broker code (e.g. cron handlers).
#[derive(Clone)]
pub struct AsyncHandlerNoArgs<F, E> {
    f: F,
    _phantom: PhantomData<E>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::message::MessageMetadata;
    use std::error::Error;
    use thiserror::Error as ThisError;

    #[derive(ThisError, Debug)]
    #[error("Mock error: {0}")]
    struct MockError(String);

    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct TestPayload {
        value: i32,
    }

    fn make_msg(data: &[u8]) -> Message {
        Message {
            payload: data.to_vec(),
            metadata: MessageMetadata::new("tag-1", "test.queue", 0),
        }
    }

    #[tokio::test]
    async fn async_handler_with_args_success() {
        let handler = AsyncHandlerWithArgs::new(|payload: TestPayload| async move {
            assert_eq!(payload.value, 42);
            Ok::<(), MockError>(())
        });

        let msg = make_msg(br#"{"value": 42}"#);
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn async_handler_with_args_deserialization_error() {
        let handler =
            AsyncHandlerWithArgs::new(
                |_payload: TestPayload| async move { Ok::<(), MockError>(()) },
            );

        let msg = make_msg(br#"{"invalid": "json"}"#);
        let result = handler.call(&msg).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HandlerError::Deserialization(_)));
        assert!(err.to_string().contains("deserialization failed"));
    }

    #[tokio::test]
    async fn async_handler_with_args_execution_error() {
        let handler = AsyncHandlerWithArgs::new(|_payload: TestPayload| async move {
            Err::<(), MockError>(MockError("handler failed".to_string()))
        });

        let msg = make_msg(br#"{"value": 42}"#);
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
        let handler = AsyncHandlerNoArgs::new(|| async move { Ok::<(), MockError>(()) });

        let msg = make_msg(&[]);
        let result = handler.call(&msg).await;
        assert!(matches!(result, Ok(AckDecision::Ack)));
    }

    #[tokio::test]
    async fn async_handler_no_args_execution_error() {
        let handler = AsyncHandlerNoArgs::new(|| async move {
            Err::<(), MockError>(MockError("no args handler failed".to_string()))
        });

        let msg = make_msg(&[]);
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
}
