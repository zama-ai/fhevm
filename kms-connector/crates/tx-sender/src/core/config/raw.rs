//! Module used to deserialize the tx-sender configuration using serde.
//!
//! The `RawConfig` can then be parsed into a `Config` in the `parsed` module.

use connector_utils::{
    config::{AwsKmsConfig, DeserializeRawConfig, RawContractConfig},
    otlp::default_metrics_endpoint,
};
use serde::{Deserialize, Serialize};

/// Deserializable representation of the KMS connector configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawConfig {
    pub database_url: String,
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,
    #[serde(default = "default_metrics_endpoint")]
    pub metrics_endpoint: String,
    pub gateway_url: String,
    pub chain_id: u64,
    pub decryption_contract: RawContractConfig,
    pub kms_management_contract: RawContractConfig,
    #[serde(default = "default_service_name")]
    pub service_name: String,
    #[serde(default)]
    pub private_key: Option<String>,
    #[serde(default)]
    pub aws_kms_config: Option<AwsKmsConfig>,
}

fn default_service_name() -> String {
    "kms-connector-tx-sender".to_string()
}

fn default_database_pool_size() -> u32 {
    16
}

impl DeserializeRawConfig for RawConfig {}

// Default implementation for testing purpose
impl Default for RawConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost".to_string(),
            database_pool_size: 16,
            metrics_endpoint: "0.0.0.0:9100".to_string(),
            gateway_url: "ws://localhost:8545".to_string(),
            chain_id: 1,
            decryption_contract: RawContractConfig {
                address: "0x0000000000000000000000000000000000000000".to_string(),
                domain_name: Some("Decryption".to_string()),
                domain_version: Some("1".to_string()),
            },
            kms_management_contract: RawContractConfig {
                address: "0x0000000000000000000000000000000000000000".to_string(),
                domain_name: Some("KmsManagement".to_string()),
                domain_version: Some("1".to_string()),
            },
            service_name: "kms-connector".to_string(),
            private_key: Some(
                "8355bb293b8714a06b972bfe692d1bd9f24235c1f4007ae0be285d398b0bba2f".to_string(),
            ),
            aws_kms_config: None,
        }
    }
}
