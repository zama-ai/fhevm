mod config;
mod connection;
mod consumer;
mod depth;
mod error;
mod exchange;
mod handler;
pub mod rmq_publisher;

pub use config::{ConsumerConfigBuilder, CronConfig, ExchangeTopology, PrefetchConfig};
pub use connection::ConnectionManager;
pub use consumer::RmqConsumer;
pub use error::{ConnectionError, ConsumerError, ExchangeError};
pub use exchange::ExchangeManager;
pub use handler::{AsyncHandlerNoArgs, AsyncHandlerWithArgs, Handler, HandlerError};

// Re-export from traits for convenience
pub use crate::traits::circuit_breaker::CircuitBreakerConfig;
pub use crate::traits::{
    AckDecision, AsyncHandlerPayloadOnly, AsyncHandlerWithContext, Consumer as ConsumerTrait,
    Message, MessageMetadata, Publisher as PublisherTrait, RetryPolicy,
};

pub use rmq_publisher::{PublisherError, RmqPublisher};

// Re-export depth inspector
pub use depth::AmqpQueueInspector;
