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
    #[serde(default = "default_gateway_config_address")]
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
    /// Optional scheduled start time in RFC3339 format (e.g., "2024-01-15T10:30:00Z")
    /// If not provided or time is in the past, connector starts immediately
    #[serde(default)]
    pub scheduled_start_time: Option<String>,
    /// Delta in milliseconds to add to block time before sending messages (default: 1000ms)
    /// This ensures coordinated sending across multiple connectors
    #[serde(default = "default_message_send_delta_ms")]
    pub message_send_delta_ms: u64,
    /// Optional starting block number for parsing (if not provided, starts from latest)
    /// Used for historical parsing or catch-up scenarios
    #[serde(default)]
    pub starting_block_number: Option<u64>,
    /// Enable coordinated message sending based on block time + delta (default: false)
    #[serde(default = "default_enable_coordinated_sending")]
    pub enable_coordinated_sending: bool,
    /// Fixed interval (in milliseconds) for sending messages to Core (alternative to block-time-based scheduling)
    /// When set to 0, uses block-time + delta scheduling. When > 0, sends messages at fixed intervals.
    #[serde(default = "default_fixed_send_interval_ms")]
    pub fixed_send_interval_ms: u64,
    /// Spacing in milliseconds between individual messages from the same block (default: 100ms)
    /// Used when fixed_send_interval_ms = 0 to prevent overwhelming Core with rapid messages
    #[serde(default = "default_message_spacing_ms")]
    pub message_spacing_ms: u64,
    /// Maximum number of pending events before pausing event intake (default: 10000)
    /// Used for backpressure control to prevent memory overflow during catch-up
    #[serde(default = "default_max_pending_events")]
    pub max_pending_events: usize,
    /// Use polling mode instead of WebSocket for event intake (default: false)
    /// When true, connector polls for new blocks instead of listening to WebSocket events
    #[serde(default = "default_use_polling_mode")]
    pub use_polling_mode: bool,
    /// Base polling interval in seconds when caught up to latest block (default: 2)
    #[serde(default = "default_base_poll_interval_secs")]
    pub base_poll_interval_secs: u64,
    /// Fast polling interval in milliseconds when catching up on historical blocks (default: 100)
    #[serde(default = "default_catch_up_poll_interval_ms")]
    pub catch_up_poll_interval_ms: u64,
    /// Maximum number of blocks to process in a single batch (default: 10)
    #[serde(default = "default_max_blocks_per_batch")]
    pub max_blocks_per_batch: u64,
    /// How far behind latest block to consider "caught up" (default: 5)
    #[serde(default = "default_catch_up_threshold")]
    pub catch_up_threshold: u64,
}

fn default_service_name() -> String {
    "kms-connector".to_string()
}

fn default_channel_size() -> usize {
    2000 // Increased for high-frequency chains (up to 100 events/block)
}

fn default_public_decryption_timeout() -> u64 {
    300 // 5 minutes
}

fn default_user_decryption_timeout() -> u64 {
    300 // 5 minutes
}

fn default_retry_interval() -> u64 {
    3 // 3 seconds (faster recovery for high-frequency chains)
}

fn default_verify_coprocessors() -> Option<bool> {
    Some(false)
}

fn default_gateway_config_address() -> String {
    // Default to local testing GatewayConfig contract address
    "0xeAC2EfFA07844aB326D92d1De29E136a6793DFFA".to_string()
}

fn default_message_send_delta_ms() -> u64 {
    200 // 2 seconds (2-3 block buffer for high-frequency chains)
}

fn default_enable_coordinated_sending() -> bool {
    true // Enabled by default for multi-pod coordination
}

fn default_fixed_send_interval_ms() -> u64 {
    0 // 0 means use block-time-based scheduling
}

fn default_message_spacing_ms() -> u64 {
    10 // 10ms spacing for high-frequency chains (user requested)
}

fn default_max_pending_events() -> usize {
    10000 // Allow up to 10k pending events before backpressure
}

fn default_use_polling_mode() -> bool {
    true // Use polling by default for high-frequency chains (more reliable)
}

fn default_base_poll_interval_secs() -> u64 {
    1 // Poll every 1 second for high-frequency chains (600-1000ms blocks)
}

fn default_catch_up_poll_interval_ms() -> u64 {
    200 // Poll every 200ms during catch-up (balanced for high-frequency chains)
}

fn default_max_blocks_per_batch() -> u64 {
    5 // Smaller batches for high event density (up to 100 events/block)
}

fn default_catch_up_threshold() -> u64 {
    3 // Consider caught up when within 3 blocks
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
