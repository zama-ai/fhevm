//! Module used to deserialize the gw-listener configuration using serde.
//!
//! The `RawConfig` can then be parsed into a `Config` in the `parsed` module.

use connector_utils::{
    config::{DeserializeRawConfig, RawContractConfig},
    monitoring::{health::default_healthcheck_timeout_secs, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::{Deserialize, Serialize};

/// Deserializable representation of the `GatewayListener` configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawConfig {
    pub database_url: String,
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,
    pub gateway_url: String,
    pub chain_id: u64,
    pub decryption_contract: RawContractConfig,
    pub kms_management_contract: RawContractConfig,
    #[serde(default = "default_service_name")]
    pub service_name: String,
    #[serde(default = "default_task_limit")]
    pub task_limit: usize,
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: String,
    #[serde(default = "default_healthcheck_timeout_secs")]
    pub healthcheck_timeout_secs: u64,
    #[serde(default = "default_decryption_polling_ms")]
    pub decryption_polling_ms: u64,
    #[serde(default = "default_key_management_polling_ms")]
    pub key_management_polling_ms: u64,
    pub from_block_number: Option<u64>,
}

fn default_service_name() -> String {
    "kms-connector-gw-listener".to_string()
}

fn default_database_pool_size() -> u32 {
    16
}

fn default_decryption_polling_ms() -> u64 {
    1000 // 1s
}

fn default_key_management_polling_ms() -> u64 {
    30000 // 30s
}

impl DeserializeRawConfig for RawConfig {}

// Default implementation for testing purpose
impl Default for RawConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost".to_string(),
            database_pool_size: default_database_pool_size(),
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
            service_name: default_service_name(),
            task_limit: default_task_limit(),
            monitoring_endpoint: default_monitoring_endpoint(),
            healthcheck_timeout_secs: default_healthcheck_timeout_secs(),
            decryption_polling_ms: default_decryption_polling_ms(),
            key_management_polling_ms: default_key_management_polling_ms(),
            from_block_number: None,
        }
    }
}
