//! Fan-out to the KMS connector endpoints and t-of-n aggregation of decryption responses. Start with `docs.md`.

mod aggregator;
mod call;
mod client;
pub mod config;
mod flows;
#[cfg(test)]
pub(crate) mod mock;
#[cfg(test)]
mod scenarios;

pub use aggregator::{AggregationError, Aggregator};
pub use call::{CallError, CallResult, Caller};
pub use client::{
    AttemptError, ConnectorClient, Endpoint, HttpClient, HttpReply, MAX_RESPONSE_BYTES,
};
pub use config::{
    ConfigError, KmsAggregatorConfig, PublicDecryptConfig, UserChecks, UserDecryptConfig,
};
pub use flows::public_decrypt::{PublicDecrypt, PublicDecryptOutput};
pub use flows::user_decrypt::{UserDecrypt, UserDecryptOutput, UserDecryptShare};
pub use flows::{Flow, RejectReason};
