//! Error types for broker.

use thiserror::Error;

/// Error type for broker operations.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum BrokerError {
    /// Redis consumer error.
    #[cfg(feature = "redis")]
    #[error("redis error: {0}")]
    Redis(#[from] crate::redis::RedisConsumerError),

    /// Redis publisher error.
    #[cfg(feature = "redis")]
    #[error("redis publisher error: {0}")]
    RedisPublisher(#[from] crate::redis::RedisPublisherError),

    /// AMQP connection error.
    #[cfg(feature = "amqp")]
    #[error("amqp connection error: {0}")]
    AmqpConnection(#[from] crate::amqp::ConnectionError),

    /// AMQP consumer error.
    #[cfg(feature = "amqp")]
    #[error("amqp consumer error: {0}")]
    AmqpConsumer(#[from] crate::amqp::ConsumerError),

    /// AMQP exchange error.
    #[cfg(feature = "amqp")]
    #[error("amqp exchange error: {0}")]
    AmqpExchange(#[from] crate::amqp::ExchangeError),

    /// AMQP publisher error.
    #[cfg(feature = "amqp")]
    #[error("amqp publisher error: {0}")]
    AmqpPublisher(#[from] crate::amqp::PublisherError),

    /// Consumer group name was not set.
    #[error("consumer group name is required")]
    MissingGroup,

    /// Publisher namespace does not match topic namespace.
    #[error("publisher namespace {publisher:?} does not match topic namespace {topic:?}")]
    NamespaceMismatch {
        publisher: Option<String>,
        topic: Option<String>,
    },

    /// Type-erased publish error (from DynPublisher).
    #[error("publish error: {0}")]
    Publish(#[from] crate::traits::publisher::DynPublishError),
}
