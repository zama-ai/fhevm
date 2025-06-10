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
}

fn default_service_name() -> String {
    "kms-connector".to_string()
}

fn default_channel_size() -> usize {
    1000
}

fn default_public_decryption_timeout() -> u64 {
    300 // 5 minutes
}

fn default_user_decryption_timeout() -> u64 {
    300 // 5 minutes
}

fn default_retry_interval() -> u64 {
    5 // 5 seconds
}

fn default_verify_coprocessors() -> Option<bool> {
    Some(false)
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
            .map_err(|e| Error::Config(format!("Failed to build config: {}", e)))?;

        settings
            .try_deserialize()
            .map_err(|e| Error::Config(format!("Failed to deserialize config: {}", e)))
    }
}
