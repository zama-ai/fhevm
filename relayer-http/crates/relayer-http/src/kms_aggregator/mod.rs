//! Fan-out to the KMS connector endpoints and t-of-n aggregation of decryption responses. Start with `docs.md`.

pub mod config;

pub use config::{ConfigError, KmsAggregatorConfig};
