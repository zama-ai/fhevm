pub mod circuit_breaker;
pub mod consumer;
pub mod depth;
pub mod handler;
pub mod message;
pub mod publisher;

// Re-exports matching the old `mq` crate's public API
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
pub use consumer::{Consumer, RetryPolicy};
pub use depth::{QueueDepths, QueueInspector};
pub use handler::{
    AckDecision, AsyncHandlerNoArgs, AsyncHandlerPayloadClassified, AsyncHandlerPayloadOnly,
    AsyncHandlerWithContext, Handler, HandlerError, HandlerOutcome,
};
pub use message::{Message, MessageMetadata};
pub use publisher::{DynPublishError, DynPublisher, Publisher};
