use alloy::transports::http::reqwest::Url;
use connector_utils::{
    config::{
        ContractConfig, DeserializeConfig,
        contract::{
            default_decryption_contract_config, default_kms_generation_contract_config,
            deserialize_decryption_contract_config, deserialize_kms_generation_contract_config,
        },
        default_database_pool_size,
    },
    monitoring::{health::default_healthcheck_timeout, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, str::FromStr, time::Duration};

/// Configuration of the `GatewayListener`.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct Config {
    /// The URL of the Postgres database.
    pub database_url: String,
    /// The size of the database connection pool.
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,

    /// The Gateway RPC endpoint.
    pub gateway_url: Url,
    /// The Chain ID of the Gateway.
    pub gateway_chain_id: u64,
    /// The `Decryption` contract configuration.
    #[serde(deserialize_with = "deserialize_decryption_contract_config")]
    pub decryption_contract: ContractConfig,
    /// The `KMSGeneration` contract configuration.
    #[serde(deserialize_with = "deserialize_kms_generation_contract_config")]
    pub kms_generation_contract: ContractConfig,

    /// The service name used for tracing.
    #[serde(default = "default_service_name")]
    pub service_name: String,
    /// The maximum number of tasks that can be executed concurrently.
    #[serde(default = "default_task_limit")]
    pub task_limit: usize,
    /// The monitoring server endpoint of the `GatewayListener`.
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: SocketAddr,
    /// The timeout to perform each external service connection healthcheck.
    #[serde(with = "humantime_serde", default = "default_healthcheck_timeout")]
    pub healthcheck_timeout: Duration,

    /// The polling interval for decryption requests.
    #[serde(with = "humantime_serde", default = "default_decryption_polling")]
    pub decryption_polling: Duration,
    /// The polling interval for key management requests.
    #[serde(with = "humantime_serde", default = "default_key_management_polling")]
    pub key_management_polling: Duration,

    /// Optional block number to start processing decryption events from.
    pub decryption_from_block_number: Option<u64>,
    /// Optional block number to start processing KMS operation events from.
    pub kms_operation_from_block_number: Option<u64>,
}

impl DeserializeConfig for Config {}

fn default_service_name() -> String {
    "kms-connector-gw-listener".to_string()
}

fn default_decryption_polling() -> Duration {
    Duration::from_secs(1)
}

fn default_key_management_polling() -> Duration {
    Duration::from_secs(30)
}

// Default implementation for testing purpose
impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost/kms-connector".to_string(),
            database_pool_size: default_database_pool_size(),
            gateway_url: Url::from_str("http://localhost:8545").unwrap(),
            gateway_chain_id: 54321,
            decryption_contract: default_decryption_contract_config(),
            kms_generation_contract: default_kms_generation_contract_config(),
            service_name: default_service_name(),
            task_limit: default_task_limit(),
            monitoring_endpoint: default_monitoring_endpoint(),
            healthcheck_timeout: default_healthcheck_timeout(),
            decryption_polling: default_decryption_polling(),
            key_management_polling: default_key_management_polling(),
            decryption_from_block_number: None,
            kms_operation_from_block_number: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::address;
    use serial_test::serial;
    use std::env;

    fn cleanup_env_vars() {
        unsafe {
            env::remove_var("KMS_CONNECTOR_DATABASE_URL");
            env::remove_var("KMS_CONNECTOR_GATEWAY_URL");
            env::remove_var("KMS_CONNECTOR_GATEWAY_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
        }
    }

    #[tokio::test]
    #[serial(config_tests)]
    async fn test_load_valid_config_from_file() {
        cleanup_env_vars();
        let default_config = Config::default();
        let example_config = Config::from_env_and_file(Some(example_config_path())).unwrap();
        assert_eq!(default_config, example_config);
    }

    #[tokio::test]
    #[serial(config_tests)]
    async fn test_load_from_env() {
        cleanup_env_vars();

        // Set environment variables
        unsafe {
            env::set_var(
                "KMS_CONNECTOR_DATABASE_URL",
                "postgres://postgres:postgres@localhost",
            );
            env::set_var("KMS_CONNECTOR_GATEWAY_URL", "http://localhost:9545");
            env::set_var("KMS_CONNECTOR_GATEWAY_CHAIN_ID", "31888");
            env::set_var(
                "KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS",
                "0x0000000000000000000000000000000000000002",
            );
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
        }

        // Load config from environment
        let config = Config::from_env_and_file::<&str>(None).unwrap();

        // Verify values
        assert_eq!(
            config.gateway_url,
            Url::from_str("http://localhost:9545").unwrap()
        );
        assert_eq!(config.gateway_chain_id, 31888);
        assert_eq!(
            config.decryption_contract.address,
            address!("0x5fbdb2315678afecb367f032d93f642f64180aa3")
        );
        assert_eq!(
            config.kms_generation_contract.address,
            address!("0x0000000000000000000000000000000000000002")
        );
        assert_eq!(config.service_name, "kms-connector-test");

        cleanup_env_vars();
    }

    #[tokio::test]
    #[serial(config_tests)]
    async fn test_env_overrides_file() {
        cleanup_env_vars();
        let example_config = Config::from_env_and_file(Some(example_config_path())).unwrap();

        // Set an environment variable to override the file
        let gateway_chain_id = 77737;
        let service_name = "kms-connector-override";
        let mut expected_config = example_config.clone();
        expected_config.gateway_chain_id = gateway_chain_id;
        expected_config.service_name = service_name.to_string();
        unsafe {
            env::set_var(
                "KMS_CONNECTOR_GATEWAY_CHAIN_ID",
                gateway_chain_id.to_string(),
            );
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", service_name);
        }

        // Load config from both sources
        let config = Config::from_env_and_file(Some(example_config_path())).unwrap();

        // Verify that environment variables take precedence
        assert_ne!(config.gateway_chain_id, example_config.gateway_chain_id);
        assert_ne!(config.service_name, example_config.service_name);
        assert_eq!(config, expected_config);

        cleanup_env_vars();
    }

    fn example_config_path() -> String {
        format!(
            "{}/../../config/gw-listener.toml",
            env!("CARGO_MANIFEST_DIR")
        )
    }
}
