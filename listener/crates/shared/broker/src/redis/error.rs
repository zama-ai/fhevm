use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RedisConsumerError {
    #[error("connection failed: {0}")]
    Connection(#[from] redis::RedisError),

    #[error("consumer group creation failed for stream '{stream}' group '{group}': {source}")]
    GroupCreation {
        stream: String,
        group: String,
        #[source]
        source: redis::RedisError,
    },

    #[error("stream read failed for '{stream}': {source}")]
    StreamRead {
        stream: String,
        #[source]
        source: redis::RedisError,
    },

    #[error("acknowledge failed for stream '{stream}': {source}")]
    Acknowledge {
        stream: String,
        #[source]
        source: redis::RedisError,
    },

    #[error("claim failed for stream '{stream}': {source}")]
    Claim {
        stream: String,
        #[source]
        source: redis::RedisError,
    },

    #[error("dead letter operation failed for stream '{stream}': {source}")]
    DeadLetter {
        stream: String,
        #[source]
        source: redis::RedisError,
    },

    #[error("transient marker operation failed for key '{key}': {source}")]
    TransientMarker {
        key: String,
        #[source]
        source: redis::RedisError,
    },

    #[error("consumer stream ended unexpectedly")]
    StreamEnded,

    #[error("configuration error: {0}")]
    Configuration(String),
}

impl RedisConsumerError {
    /// Returns `true` if the error is a transient connection issue that
    /// `ConnectionManager` can recover from on the next command.
    ///
    /// Consumer loops use this to decide between:
    /// - **connection error** → log, backoff, continue loop (ConnectionManager auto-reconnects)
    /// - **other error** → propagate up
    pub fn is_connection_error(&self) -> bool {
        let redis_err = match self {
            Self::Connection(e) => Some(e),
            Self::StreamRead { source, .. } => Some(source),
            Self::Acknowledge { source, .. } => Some(source),
            Self::GroupCreation { source, .. } => Some(source),
            Self::Claim { source, .. } => Some(source),
            Self::DeadLetter { source, .. } => Some(source),
            Self::TransientMarker { source, .. } => Some(source),
            Self::StreamEnded | Self::Configuration(_) => None,
        };

        redis_err.is_some_and(|e| {
            e.is_io_error() || e.is_connection_dropped() || e.is_connection_refusal()
        })
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RedisPublisherError {
    #[error("serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("connection failed: {0}")]
    Connection(#[from] redis::RedisError),

    #[error("publish failed for stream '{stream}': {source}")]
    Publish {
        stream: String,
        #[source]
        source: redis::RedisError,
    },

    #[error("publish failed after {retries} retries (stream: {stream})")]
    RetriesExhausted { retries: u32, stream: String },
}
