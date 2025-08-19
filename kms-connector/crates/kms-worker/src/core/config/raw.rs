//! Module used to deserialize the kms-worker configuration using serde.
//!
//! The `RawConfig` can then be parsed into a `Config` in the `parsed` module.

use connector_utils::{
    config::{DeserializeRawConfig, RawContractConfig},
    monitoring::{health::default_healthcheck_timeout_secs, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::{Deserialize, Serialize};

/// Configuration for S3 ciphertext storage.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct S3Config {
    /// AWS S3 region for ciphertext storage.
    pub region: String,
    /// AWS S3 bucket for ciphertext storage.
    pub bucket: String,
    /// AWS S3 endpoint URL for ciphertext storage.
    pub endpoint: Option<String>,
}

/// Deserializable representation of the `KmsWorker` configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawConfig {
    pub database_url: String,
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,
    #[serde(default = "default_database_polling_timeout_secs")]
    pub database_polling_timeout_secs: u64,
    pub gateway_url: String,
    pub kms_core_endpoint: String,
    pub chain_id: u64,
    pub decryption_contract: RawContractConfig,
    pub gateway_config_contract: RawContractConfig,
    #[serde(default = "default_service_name")]
    pub service_name: String,
    #[serde(default = "default_events_batch_size")]
    pub events_batch_size: u8,
    #[serde(default = "default_grpc_request_retries")]
    pub grpc_request_retries: u8,
    #[serde(default = "default_public_decryption_timeout")]
    pub public_decryption_timeout_secs: u64,
    #[serde(default = "default_user_decryption_timeout")]
    pub user_decryption_timeout_secs: u64,
    #[serde(default = "default_grpc_poll_interval")]
    pub grpc_poll_interval_secs: u64,
    #[serde(default)]
    pub s3_config: Option<S3Config>,
    #[serde(default = "default_s3_ciphertext_retrieval_retries")]
    pub s3_ciphertext_retrieval_retries: u8,
    #[serde(default = "default_s3_connect_timeout")]
    pub s3_connect_timeout: u64,
    #[serde(default = "default_task_limit")]
    pub task_limit: usize,
    #[serde(default = "default_verify_coprocessors")]
    pub verify_coprocessors: bool,
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: String,
    #[serde(default = "default_healthcheck_timeout_secs")]
    pub healthcheck_timeout_secs: u64,
}

fn default_service_name() -> String {
    "kms-connector-kms-worker".to_string()
}

fn default_database_pool_size() -> u32 {
    16
}

fn default_database_polling_timeout_secs() -> u64 {
    5
}

fn default_events_batch_size() -> u8 {
    10
}

fn default_grpc_request_retries() -> u8 {
    3
}

fn default_public_decryption_timeout() -> u64 {
    300 // 5 minutes
}

fn default_user_decryption_timeout() -> u64 {
    300 // 5 minutes
}

fn default_grpc_poll_interval() -> u64 {
    5 // 5 seconds
}

fn default_s3_ciphertext_retrieval_retries() -> u8 {
    3
}

fn default_s3_connect_timeout() -> u64 {
    2 // 2 seconds
}

fn default_verify_coprocessors() -> bool {
    false
}

impl DeserializeRawConfig for RawConfig {}

// Default implementation for testing purpose
impl Default for RawConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost".to_string(),
            database_pool_size: 16,
            database_polling_timeout_secs: default_database_polling_timeout_secs(),
            gateway_url: "ws://localhost:8545".to_string(),
            kms_core_endpoint: "http://localhost:50052".to_string(),
            chain_id: 1,
            decryption_contract: RawContractConfig {
                address: "0x0000000000000000000000000000000000000000".to_string(),
                domain_name: Some("Decryption".to_string()),
                domain_version: Some("1".to_string()),
            },
            gateway_config_contract: RawContractConfig {
                address: "0x0000000000000000000000000000000000000000".to_string(),
                domain_name: Some("GatewayConfig".to_string()),
                domain_version: Some("1".to_string()),
            },
            service_name: "kms-connector".to_string(),
            events_batch_size: 10,
            grpc_request_retries: 3,
            public_decryption_timeout_secs: 300,
            user_decryption_timeout_secs: 300,
            grpc_poll_interval_secs: 5,
            s3_ciphertext_retrieval_retries: 3,
            s3_connect_timeout: 2,
            s3_config: None,
            task_limit: default_task_limit(),
            verify_coprocessors: false,
            monitoring_endpoint: default_monitoring_endpoint(),
            healthcheck_timeout_secs: default_healthcheck_timeout_secs(),
        }
    }
}
