//! Fan-out to the KMS connector endpoints and t-of-n aggregation of decryption responses. Start with `docs.md`.

mod call;
mod client;
pub mod config;
#[cfg(test)]
pub(crate) mod mock;

pub use call::{CallError, CallResult, Caller};
pub use client::{
    AttemptError, ConnectorClient, Endpoint, HttpClient, HttpReply, MAX_RESPONSE_BYTES,
};
pub use config::{ConfigError, KmsAggregatorConfig};
