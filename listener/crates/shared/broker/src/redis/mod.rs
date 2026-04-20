mod circuit_breaker;
mod claim_task;
mod config;
mod connection;
mod consumer;
mod dead_letter;
mod depth;
mod error;
mod handler;
mod publisher;
mod stream_manager;
pub mod trimmer;

pub use claim_task::ClaimSweeper;
pub use config::{RedisConsumerConfigBuilder, RedisPrefetchConfig, StreamTopology};
pub use connection::RedisConnectionManager;
pub use consumer::RedisConsumer;
pub use dead_letter::{DeadLetterProcessor, DeadMessage};
pub use depth::RedisQueueInspector;
pub use error::{RedisConsumerError, RedisPublisherError};
pub use handler::{
    AckDecision, AsyncRedisHandlerNoArgs, AsyncRedisHandlerPayloadOnly, AsyncRedisHandlerWithArgs,
    Handler, HandlerError, Message, MessageMetadata,
};
pub use publisher::{RedisPublisher, RedisPublisherBuilder, ReplicationConfig};
pub use stream_manager::StreamManager;
pub use trimmer::{StreamTrimmer, StreamTrimmerConfig};

// Re-export from traits for convenience
pub use crate::traits::circuit_breaker::CircuitBreakerConfig;
pub use crate::traits::{
    AsyncHandlerPayloadOnly, AsyncHandlerWithContext, Consumer, Publisher as PublisherTrait,
    RetryPolicy,
};
