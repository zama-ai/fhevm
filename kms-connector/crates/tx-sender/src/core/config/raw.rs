//! Module used to deserialize the tx-sender configuration using serde.
//!
//! The `RawConfig` can then be parsed into a `Config` in the `parsed` module.

use connector_utils::{
    config::{AwsKmsConfig, DeserializeRawConfig, RawContractConfig},
    monitoring::{health::default_healthcheck_timeout_secs, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::{Deserialize, Serialize};

/// Deserializable representation of the KMS connector configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawConfig {
    pub database_url: String,
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,
    #[serde(default = "default_database_polling_timeout_secs")]
    pub database_polling_timeout_secs: u64,
    pub gateway_url: String,
    pub chain_id: u64,
    pub decryption_contract: RawContractConfig,
    pub kms_management_contract: RawContractConfig,
    #[serde(default = "default_service_name")]
    pub service_name: String,
    pub private_key: Option<String>,
    pub aws_kms_config: Option<AwsKmsConfig>,
    #[serde(default = "default_tx_retries")]
    pub tx_retries: u8,
    #[serde(default = "default_tx_retry_interval_ms")]
    pub tx_retry_interval_ms: u64,
    #[serde(default = "default_responses_batch_size")]
    pub responses_batch_size: u8,
    #[serde(default = "default_gas_multiplier_percent")]
    pub gas_multiplier_percent: usize,
    #[serde(default = "default_task_limit")]
    pub task_limit: usize,
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: String,
    #[serde(default = "default_healthcheck_timeout_secs")]
    pub healthcheck_timeout_secs: u64,
}

fn default_service_name() -> String {
    "kms-connector-tx-sender".to_string()
}

fn default_database_pool_size() -> u32 {
    16
}

fn default_database_polling_timeout_secs() -> u64 {
    5
}

fn default_tx_retries() -> u8 {
    3
}

fn default_tx_retry_interval_ms() -> u64 {
    100
}

fn default_responses_batch_size() -> u8 {
    10
}

fn default_gas_multiplier_percent() -> usize {
    130 // 130% gas increase by default
}

impl DeserializeRawConfig for RawConfig {}

// Default implementation for testing purpose
impl Default for RawConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost".to_string(),
            database_pool_size: default_database_pool_size(),
            database_polling_timeout_secs: default_database_polling_timeout_secs(),
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
            tx_retries: default_tx_retries(),
            tx_retry_interval_ms: default_tx_retry_interval_ms(),
            responses_batch_size: default_responses_batch_size(),
            gas_multiplier_percent: default_gas_multiplier_percent(),
            task_limit: default_task_limit(),
            monitoring_endpoint: default_monitoring_endpoint(),
            healthcheck_timeout_secs: default_healthcheck_timeout_secs(),
        }
    }
}
