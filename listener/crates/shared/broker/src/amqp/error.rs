use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ConnectionError {
    #[error("connection failed: {0}")]
    Connection(#[from] lapin::Error),
    #[error("channel creation failed: {0}")]
    Channel(String),
}

impl From<ConnectionError> for ConsumerError {
    fn from(err: ConnectionError) -> Self {
        match err {
            ConnectionError::Connection(e) => ConsumerError::Connection(e),
            ConnectionError::Channel(s) => ConsumerError::Channel(s),
        }
    }
}

impl From<ConnectionError> for ExchangeError {
    fn from(err: ConnectionError) -> Self {
        match err {
            ConnectionError::Connection(e) => ExchangeError::Connection(e),
            ConnectionError::Channel(s) => ExchangeError::Channel(s),
        }
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ConsumerError {
    #[error("connection failed: {0}")]
    Connection(#[from] lapin::Error),

    #[error("channel creation failed: {0}")]
    Channel(String),

    #[error("queue declaration failed for '{queue}': {source}")]
    QueueDeclaration {
        queue: String,
        #[source]
        source: lapin::Error,
    },

    #[error("queue binding failed for '{queue}' to '{exchange}': {source}")]
    QueueBinding {
        queue: String,
        exchange: String,
        #[source]
        source: lapin::Error,
    },

    #[error("exchange declaration failed for '{exchange}': {source}")]
    ExchangeDeclaration {
        exchange: String,
        #[source]
        source: lapin::Error,
    },

    #[error("consumer registration failed for '{consumer_tag}': {source}")]
    ConsumerRegistration {
        consumer_tag: String,
        #[source]
        source: lapin::Error,
    },

    #[error("message acknowledgement failed: {0}")]
    Ack(#[source] lapin::Error),

    #[error("message negative acknowledgement failed: {0}")]
    Nack(#[source] lapin::Error),

    #[error("publish to DLX failed: {0}")]
    DeadLetter(#[source] lapin::Error),

    #[error("publish to retry exchange failed: {0}")]
    Retry(lapin::Error),

    #[error("consumer stream ended unexpectedly")]
    StreamEnded,

    #[error("configuration error: {0}")]
    Configuration(String),
}

impl ConsumerError {
    /// Returns `true` if the error is a transient connection/channel failure
    /// that can be recovered by creating a new channel and consumer.
    ///
    /// Only `Configuration` errors are non-reconnectable (user error that
    /// won't fix itself on retry).
    pub fn is_reconnectable(&self) -> bool {
        !matches!(self, Self::Configuration(_))
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ExchangeError {
    #[error("exchange declaration failed for '{name}': {source}")]
    Declaration {
        name: String,
        #[source]
        source: lapin::Error,
    },

    #[error("channel creation failed: {0}")]
    Channel(String),

    #[error("connection failed: {0}")]
    Connection(#[from] lapin::Error),
}
