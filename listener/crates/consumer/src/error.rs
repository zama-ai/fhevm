use broker::BrokerError;
use primitives::event::{CatchupPayloadValidationError, FilterCommandValidationError};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConsumerError {
    #[error(transparent)]
    Broker(#[from] BrokerError),
    #[error(transparent)]
    InvalidFilterCommand(#[from] FilterCommandValidationError),
    #[error(transparent)]
    InvalidCatchupPayload(#[from] CatchupPayloadValidationError),
    #[error("FilterCommand consumer_id '{}' does not match ListenerConsumer consumer_id '{}'", .0, .1)]
    InconsistentConsumerId(String, String),
    #[error("Invalid parameter when configuring the consumer {}", .0)]
    InvalidParameter(String),
}
