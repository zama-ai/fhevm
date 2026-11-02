use connector_utils::{
    config::{
        ContractConfig, DatabaseUrl, DeserializeConfig, RpcUrl,
        contract::{
            default_decryption_contract_config, default_kms_generation_contract_config,
            default_protocol_config_contract_config, deserialize_decryption_contract_config,
            deserialize_kms_generation_contract_config,
            deserialize_protocol_config_contract_config,
        },
        default_database_pool_size, deserialize_non_zero_duration,
    },
    monitoring::{health::default_healthcheck_timeout, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::Deserialize;
use std::{net::SocketAddr, time::Duration};

/// Configuration of the `GatewayListener`.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Config {
    /// The URL of the Postgres database.
    pub database_url: DatabaseUrl,
    /// The size of the database connection pool.
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,

    /// The Gateway RPC endpoint.
    pub gateway_url: RpcUrl,
    /// The Chain ID of the Gateway.
    pub gateway_chain_id: u64,
    /// The `Decryption` contract configuration.
    #[serde(deserialize_with = "deserialize_decryption_contract_config")]
    pub decryption_contract: ContractConfig,
    /// The `KMSGeneration` contract configuration.
    #[serde(deserialize_with = "deserialize_kms_generation_contract_config")]
    pub kms_generation_contract: ContractConfig,
    /// The `ProtocolConfig` contract configuration.
    #[serde(deserialize_with = "deserialize_protocol_config_contract_config")]
    pub protocol_config_contract: ContractConfig,

    /// The Ethereum RPC node endpoint.
    pub ethereum_url: RpcUrl,
    /// The Chain ID of the Ethereum chain.
    pub ethereum_chain_id: u64,

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
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_healthcheck_timeout"
    )]
    pub healthcheck_timeout: Duration,

    /// The polling interval for decryption requests.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_decryption_polling"
    )]
    pub decryption_polling: Duration,
    /// The polling interval for key management requests.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_key_management_polling"
    )]
    pub key_management_polling: Duration,

    /// The maximum number of blocks to fetch per `eth_getLogs` request.
    #[serde(default = "default_get_logs_batch_size")]
    pub get_logs_batch_size: u64,

    /// Maximum number of consecutive polling errors before stopping the loop.
    #[serde(default = "default_max_consecutive_polling_errors")]
    pub max_consecutive_polling_errors: usize,

    /// Timeout of a single Gateway or Ethereum RPC call.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_rpc_call_timeout"
    )]
    pub rpc_call_timeout: Duration,

    /// How long the Gateway head reported by the RPC node may stay the same before a warning.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_gateway_stale_head_threshold"
    )]
    pub gateway_stale_head_threshold: Duration,
    /// How long the Ethereum finalized block reported by the RPC node may stay the same before a
    /// warning.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_ethereum_stale_head_threshold"
    )]
    pub ethereum_stale_head_threshold: Duration,

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
    Duration::from_millis(500)
}

fn default_key_management_polling() -> Duration {
    Duration::from_secs(30)
}

fn default_get_logs_batch_size() -> u64 {
    100
}

fn default_max_consecutive_polling_errors() -> usize {
    20
}

fn default_rpc_call_timeout() -> Duration {
    Duration::from_secs(10)
}

fn default_gateway_stale_head_threshold() -> Duration {
    // The Gateway only produces blocks when it receives transactions, so this must exceed the
    // longest quiet period expected on the chain.
    Duration::from_mins(10)
}

fn default_ethereum_stale_head_threshold() -> Duration {
    // Ethereum finalizes a new block about every 6.4 minutes (one epoch).
    Duration::from_mins(20)
}

// Default implementation for testing purpose
impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost/kms-connector"
                .parse()
                .unwrap(),
            database_pool_size: default_database_pool_size(),
            gateway_url: "http://localhost:8545".parse().unwrap(),
            gateway_chain_id: 54321,
            ethereum_url: "http://localhost:8545".parse().unwrap(),
            ethereum_chain_id: 11155111,
            decryption_contract: default_decryption_contract_config(),
            kms_generation_contract: default_kms_generation_contract_config(),
            protocol_config_contract: default_protocol_config_contract_config(),
            service_name: default_service_name(),
            task_limit: default_task_limit(),
            monitoring_endpoint: default_monitoring_endpoint(),
            healthcheck_timeout: default_healthcheck_timeout(),
            decryption_polling: default_decryption_polling(),
            key_management_polling: default_key_management_polling(),
            get_logs_batch_size: default_get_logs_batch_size(),
            max_consecutive_polling_errors: default_max_consecutive_polling_errors(),
            rpc_call_timeout: default_rpc_call_timeout(),
            gateway_stale_head_threshold: default_gateway_stale_head_threshold(),
            ethereum_stale_head_threshold: default_ethereum_stale_head_threshold(),
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
            env::remove_var("KMS_CONNECTOR_ETHEREUM_URL");
            env::remove_var("KMS_CONNECTOR_ETHEREUM_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
            env::remove_var("KMS_CONNECTOR_GET_LOGS_BATCH_SIZE");
            env::remove_var("KMS_CONNECTOR_MAX_CONSECUTIVE_POLLING_ERRORS");
            env::remove_var("KMS_CONNECTOR_RPC_CALL_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_GATEWAY_STALE_HEAD_THRESHOLD");
            env::remove_var("KMS_CONNECTOR_ETHEREUM_STALE_HEAD_THRESHOLD");
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
            env::set_var("KMS_CONNECTOR_ETHEREUM_URL", "http://localhost:9545");
            env::set_var("KMS_CONNECTOR_ETHEREUM_CHAIN_ID", "31444");
            env::set_var(
                "KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS",
                "0x0000000000000000000000000000000000000002",
            );
            env::set_var(
                "KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS",
                "0x0000000000000000000000000000000000000003",
            );
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
        }

        // Load config from environment
        let config = Config::from_env_and_file::<&str>(None).unwrap();

        // Verify values
        assert_eq!(config.gateway_url, "http://localhost:9545".parse().unwrap());
        assert_eq!(config.gateway_chain_id, 31888);
        assert_eq!(
            config.decryption_contract.address,
            address!("0x5fbdb2315678afecb367f032d93f642f64180aa3")
        );
        assert_eq!(
            config.kms_generation_contract.address,
            address!("0x0000000000000000000000000000000000000002")
        );
        assert_eq!(
            config.protocol_config_contract.address,
            address!("0x0000000000000000000000000000000000000003")
        );
        assert_eq!(
            config.ethereum_url,
            "http://localhost:9545".parse().unwrap()
        );
        assert_eq!(config.ethereum_chain_id, 31444);
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
        let get_logs_batch_size: u64 = 500;
        let max_consecutive_polling_errors = 5;
        let rpc_call_timeout = Duration::from_secs(3);
        let gateway_stale_head_threshold = Duration::from_secs(120);
        let ethereum_stale_head_threshold = Duration::from_secs(900);
        let mut expected_config = example_config.clone();
        expected_config.gateway_chain_id = gateway_chain_id;
        expected_config.service_name = service_name.to_string();
        expected_config.get_logs_batch_size = get_logs_batch_size;
        expected_config.max_consecutive_polling_errors = max_consecutive_polling_errors;
        expected_config.rpc_call_timeout = rpc_call_timeout;
        expected_config.gateway_stale_head_threshold = gateway_stale_head_threshold;
        expected_config.ethereum_stale_head_threshold = ethereum_stale_head_threshold;
        unsafe {
            env::set_var(
                "KMS_CONNECTOR_GATEWAY_CHAIN_ID",
                gateway_chain_id.to_string(),
            );
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", service_name);
            env::set_var(
                "KMS_CONNECTOR_GET_LOGS_BATCH_SIZE",
                get_logs_batch_size.to_string(),
            );
            env::set_var(
                "KMS_CONNECTOR_MAX_CONSECUTIVE_POLLING_ERRORS",
                max_consecutive_polling_errors.to_string(),
            );
            env::set_var("KMS_CONNECTOR_RPC_CALL_TIMEOUT", "3s");
            env::set_var("KMS_CONNECTOR_GATEWAY_STALE_HEAD_THRESHOLD", "2m");
            env::set_var("KMS_CONNECTOR_ETHEREUM_STALE_HEAD_THRESHOLD", "15m");
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
