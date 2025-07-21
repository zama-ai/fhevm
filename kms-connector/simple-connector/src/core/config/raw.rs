//! Module used to deserialize the kms-connector configuration using serde.
//!
//! The `RawConfig` can then be parsed into a `Config` in the `parsed` module.

use crate::error::{Error, Result};
use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

/// Configuration for S3 ciphertext storage.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct S3Config {
    /// AWS S3 region for ciphertext storage.
    pub region: String,
    /// AWS S3 bucket for ciphertext storage.
    pub bucket: String,
    /// AWS S3 endpoint URL for ciphertext storage.
    pub endpoint: Option<String>,
}

/// Configuration for AWS KMS signer.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AwsKmsConfig {
    /// AWS KMS key ID for signing.
    pub key_id: String,
    /// AWS region for KMS.
    pub region: Option<String>,
    /// AWS endpoint URL for KMS.
    pub endpoint: Option<String>,
}

/// Deserializable representation of the KMS connector configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawConfig {
    pub gateway_url: String,
    pub kms_core_endpoint: String,
    pub chain_id: u64,
    pub decryption_address: String,
    pub gateway_config_address: String,
    #[serde(default = "default_channel_size")]
    pub channel_size: usize,
    #[serde(default = "default_service_name")]
    pub service_name: String,
    #[serde(default = "default_public_decryption_timeout")]
    pub public_decryption_timeout_secs: u64,
    #[serde(default = "default_user_decryption_timeout")]
    pub user_decryption_timeout_secs: u64,
    #[serde(default = "default_retry_interval")]
    pub retry_interval_secs: u64,
    pub decryption_domain_name: Option<String>,
    pub decryption_domain_version: Option<String>,
    pub gateway_config_domain_name: Option<String>,
    pub gateway_config_domain_version: Option<String>,
    #[serde(default)]
    pub private_key: Option<String>,
    #[serde(default)]
    pub s3_config: Option<S3Config>,
    #[serde(default)]
    pub aws_kms_config: Option<AwsKmsConfig>,
    #[serde(default = "default_verify_coprocessors")]
    pub verify_coprocessors: Option<bool>,
    // Coordination parameters
    #[serde(default = "default_enable_coordinated_sending")]
    pub enable_coordinated_sending: bool,
    #[serde(default = "default_message_send_delta_ms")]
    pub message_send_delta_ms: u64,
    #[serde(default = "default_message_spacing_ms")]
    pub message_spacing_ms: u64,
    #[serde(default = "default_pending_events_max")]
    pub pending_events_max: usize,
    #[serde(default = "default_pending_events_queue_slowdown_threshold")]
    pub pending_events_queue_slowdown_threshold: f32,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_starting_block_number")]
    pub starting_block_number: Option<u64>,
    #[serde(default = "default_max_concurrent_tasks")]
    pub max_concurrent_tasks: usize,
    // Polling parameters
    #[serde(default = "default_use_polling_mode")]
    pub use_polling_mode: bool,
    #[serde(default = "default_base_poll_interval_ms")]
    pub base_poll_interval_ms: u64,
    #[serde(default = "default_max_blocks_per_batch")]
    pub max_blocks_per_batch: u64,
}

fn default_service_name() -> String {
    "kms-connector".to_string()
}

fn default_channel_size() -> usize {
    5000
}

fn default_public_decryption_timeout() -> u64 {
    300 // 5 minutes
}

fn default_user_decryption_timeout() -> u64 {
    300 // 5 minutes
}

fn default_retry_interval() -> u64 {
    1 // 1 second
}

fn default_verify_coprocessors() -> Option<bool> {
    Some(false)
}

fn default_enable_coordinated_sending() -> bool {
    true
}

fn default_message_send_delta_ms() -> u64 {
    1000 // 1000ms delay after block time
}

fn default_message_spacing_ms() -> u64 {
    5 // 5ms between messages
}

fn default_pending_events_max() -> usize {
    10000 // Maximum 10k pending messages
}

fn default_pending_events_queue_slowdown_threshold() -> f32 {
    0.8 // Slow down at 80% capacity
}

fn default_max_retries() -> u32 {
    3 // 3 retries per message
}

fn default_starting_block_number() -> Option<u64> {
    None // Start from latest block by default
}

fn default_max_concurrent_tasks() -> usize {
    500 // Maximum 500 concurrent tasks
}

fn default_use_polling_mode() -> bool {
    true // Use WebSocket by default
}

fn default_base_poll_interval_ms() -> u64 {
    100 // Poll every 100 ms
}

fn default_max_blocks_per_batch() -> u64 {
    10 // Process 10 blocks per batch
}

impl RawConfig {
    pub fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> Result<Self> {
        let mut builder = ConfigBuilder::builder();

        // If path is provided, add it as a config source
        if let Some(path) = path {
            info!(
                "Loading configuration from file: {}",
                path.as_ref().display()
            );
            builder = builder.add_source(
                File::with_name(path.as_ref().to_str().unwrap()).format(FileFormat::Toml),
            );
        }

        // Add environment variables last so they take precedence
        info!("Adding environment variables with prefix KMS_CONNECTOR_");
        builder = builder.add_source(Environment::with_prefix("KMS_CONNECTOR"));

        let settings = builder
            .build()
            .map_err(|e| Error::Config(format!("Failed to build config: {e}")))?;

        settings
            .try_deserialize()
            .map_err(|e| Error::Config(format!("Failed to deserialize config: {e}")))
    }
}
