//! Module used to deserialize the kms-worker configuration using serde.
//!
//! The `RawConfig` can then be parsed into a `Config` in the `parsed` module.

use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use connector_utils::{
    config::{DeserializeRawConfig, RawContractConfig, Result, default_database_pool_size},
    monitoring::{health::default_healthcheck_timeout_secs, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

/// Deserializable representation of the `KmsWorker` configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawConfig {
    pub database_url: String,
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,
    #[serde(default = "default_database_polling_timeout_secs")]
    pub database_polling_timeout_secs: u64,
    pub gateway_url: String,
    #[serde(default)]
    pub kms_core_endpoints: Vec<String>,
    pub kms_core_endpoint: Option<String>,
    pub chain_id: u64,
    pub decryption_contract: RawContractConfig,
    pub gateway_config_contract: RawContractConfig,
    pub kms_generation_contract: RawContractConfig,
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
    #[serde(default = "default_s3_ciphertext_retrieval_retries")]
    pub s3_ciphertext_retrieval_retries: u8,
    #[serde(default = "default_s3_connect_timeout")]
    pub s3_connect_timeout: u64,
    #[serde(default = "default_task_limit")]
    pub task_limit: usize,
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: String,
    #[serde(default = "default_healthcheck_timeout_secs")]
    pub healthcheck_timeout_secs: u64,
}

fn default_service_name() -> String {
    "kms-connector-kms-worker".to_string()
}

fn default_database_polling_timeout_secs() -> u64 {
    5
}

fn default_events_batch_size() -> u8 {
    50
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
    1 // 1 seconds
}

fn default_s3_ciphertext_retrieval_retries() -> u8 {
    3
}

fn default_s3_connect_timeout() -> u64 {
    2 // 2 seconds
}

impl DeserializeRawConfig for RawConfig {
    fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> Result<Self>
    where
        for<'a> Self: Sized + Deserialize<'a>,
    {
        let mut builder = ConfigBuilder::builder();

        // If path is provided, add it as a config source
        if let Some(path) = path {
            builder = builder.add_source(
                File::with_name(path.as_ref().to_str().unwrap()).format(FileFormat::Toml),
            );
        }

        // Add environment variables last so they take precedence
        info!("Adding environment variables with prefix KMS_CONNECTOR_");
        builder = builder.add_source(
            Environment::with_prefix("KMS_CONNECTOR")
                .prefix_separator("_")
                .separator("__")
                .list_separator(",")
                .with_list_parse_key("kms_core_endpoints")
                .try_parsing(true),
        );

        let settings = builder.build()?;
        let config = settings.try_deserialize()?;
        Ok(config)
    }
}

// Default implementation for testing purpose
impl Default for RawConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost".to_string(),
            database_pool_size: 16,
            database_polling_timeout_secs: default_database_polling_timeout_secs(),
            gateway_url: "ws://localhost:8545".to_string(),
            kms_core_endpoints: vec!["http://localhost:50052".to_string()],
            kms_core_endpoint: None,
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
            kms_generation_contract: RawContractConfig {
                address: "0x0000000000000000000000000000000000000000".to_string(),
                domain_name: Some("KMSGeneration".to_string()),
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
            task_limit: default_task_limit(),
            monitoring_endpoint: default_monitoring_endpoint(),
            healthcheck_timeout_secs: default_healthcheck_timeout_secs(),
        }
    }
}
