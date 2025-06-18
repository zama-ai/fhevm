//! Module used to deserialize the gw-listener configuration using serde.
//!
//! The `RawConfig` can then be parsed into a `Config` in the `parsed` module.

use connector_utils::config::{DeserializeRawConfig, RawContractConfig};
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
}

fn default_service_name() -> String {
    "kms-connector-gw-listener".to_string()
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
        }
    }
}
