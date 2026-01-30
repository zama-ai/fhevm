use alloy::{primitives::Address, transports::http::reqwest::Url};
use connector_utils::{
    config::{
        ContractConfig, DeserializeConfig,
        contract::{
            default_decryption_contract_config, default_gateway_config_contract_config,
            default_kms_generation_contract_config, deserialize_decryption_contract_config,
            deserialize_gateway_config_contract_config, deserialize_kms_generation_contract_config,
        },
        default_database_pool_size,
    },
    monitoring::{health::default_healthcheck_timeout, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::{net::SocketAddr, str::FromStr, time::Duration};

/// Configuration of the `KmsWorker`.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct Config {
    /// The URL of the Postgres database.
    pub database_url: String,
    /// The size of the database connection pool.
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,
    /// The timeout for polling the database for fast events (decryption for ex).
    #[serde(with = "humantime_serde", default = "default_db_fast_event_polling")]
    pub db_fast_event_polling: Duration,
    /// The timeout for polling the database for long events (prep keygen for ex).
    #[serde(with = "humantime_serde", default = "default_db_long_event_polling")]
    pub db_long_event_polling: Duration,
    /// The limit number of events to fetch from the database.
    #[serde(default = "default_events_batch_size")]
    pub events_batch_size: u8,

    /// The Gateway RPC endpoint.
    pub gateway_url: Url,
    /// The Chain ID of the Gateway.
    pub gateway_chain_id: u64,
    /// The `Decryption` contract configuration.
    #[serde(deserialize_with = "deserialize_decryption_contract_config")]
    pub decryption_contract: ContractConfig,
    /// The `GatewayConfig` contract configuration.
    #[serde(deserialize_with = "deserialize_gateway_config_contract_config")]
    pub gateway_config_contract: ContractConfig,
    /// The `KMSGeneration` contract configuration.
    #[serde(deserialize_with = "deserialize_kms_generation_contract_config")]
    pub kms_generation_contract: ContractConfig,

    /// The Host Chains configuration.
    #[serde(deserialize_with = "deserialize_host_chains")]
    pub host_chains: Vec<HostChainConfig>,

    /// The KMS Core endpoints.
    #[serde(deserialize_with = "deserialize_non_empty")]
    pub kms_core_endpoints: Vec<String>,
    /// Number of retries for GRPC requests sent to the KMS Core.
    #[serde(default = "default_grpc_request_retries")]
    pub grpc_request_retries: u8,
    /// The maximum number of decryption attempts.
    #[serde(default = "default_max_decryption_attempts")]
    pub max_decryption_attempts: u16,

    /// Number of retries for S3 ciphertext retrieval.
    #[serde(default = "default_s3_ciphertext_retrieval_retries")]
    pub s3_ciphertext_retrieval_retries: u8,
    /// Timeout to connect to a S3 bucket.
    #[serde(with = "humantime_serde", default = "default_s3_connect_timeout")]
    pub s3_connect_timeout: Duration,

    /// The service name used for tracing.
    #[serde(default = "default_service_name")]
    pub service_name: String,
    /// The maximum number of tasks that can be executed concurrently.
    #[serde(default = "default_task_limit")]
    pub task_limit: usize,
    /// The monitoring server endpoint of the `KmsWorker`.
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: SocketAddr,
    /// The timeout to perform each external service connection healthcheck.
    #[serde(with = "humantime_serde", default = "default_healthcheck_timeout")]
    pub healthcheck_timeout: Duration,
}

/// Configuration of a single Host Chain.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct HostChainConfig {
    /// The Host Chain RPC endpoint.
    pub url: Url,
    /// The Chain ID of the Host Chain.
    #[serde(alias = "chainId")]
    pub chain_id: u64,
    /// The `ACL` contract address on the Host Chain.
    #[serde(alias = "aclAddress")]
    pub acl_address: Address,
}

fn deserialize_host_chains<'de, D>(d: D) -> Result<Vec<HostChainConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    let host_chains = if let Ok(host_chains_json_str) = std::env::var("KMS_CONNECTOR_HOST_CHAINS") {
        serde_json::from_str(&host_chains_json_str).map_err(serde::de::Error::custom)?
    } else {
        <Vec<HostChainConfig>>::deserialize(d)?
    };
    if host_chains.is_empty() {
        Err(serde::de::Error::custom(
            "Field should not be an empty array",
        ))
    } else {
        Ok(host_chains)
    }
}

fn deserialize_non_empty<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let vec = <Vec<T>>::deserialize(d)?;
    if vec.is_empty() {
        Err(serde::de::Error::custom(
            "Field should not be an empty array",
        ))
    } else {
        Ok(vec)
    }
}

fn default_service_name() -> String {
    "kms-connector-kms-worker".to_string()
}

fn default_db_fast_event_polling() -> Duration {
    Duration::from_secs(3)
}

fn default_db_long_event_polling() -> Duration {
    Duration::from_secs(60)
}

fn default_events_batch_size() -> u8 {
    50
}

fn default_grpc_request_retries() -> u8 {
    3
}

fn default_max_decryption_attempts() -> u16 {
    20
}

fn default_s3_ciphertext_retrieval_retries() -> u8 {
    3
}

fn default_s3_connect_timeout() -> Duration {
    Duration::from_secs(3)
}

impl DeserializeConfig for Config {}

// Default implementation for testing purpose
impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost/kms-connector".to_string(),
            database_pool_size: default_database_pool_size(),
            db_fast_event_polling: default_db_fast_event_polling(),
            db_long_event_polling: default_db_long_event_polling(),
            events_batch_size: default_events_batch_size(),
            gateway_url: Url::from_str("http://localhost:8545").unwrap(),
            gateway_chain_id: 54321,
            decryption_contract: default_decryption_contract_config(),
            gateway_config_contract: default_gateway_config_contract_config(),
            kms_generation_contract: default_kms_generation_contract_config(),
            host_chains: vec![HostChainConfig {
                url: Url::from_str("http://localhost:8545").unwrap(),
                chain_id: 12345,
                acl_address: Address::default(),
            }],
            kms_core_endpoints: vec!["http://localhost:50051".to_string()],
            grpc_request_retries: default_grpc_request_retries(),
            max_decryption_attempts: default_max_decryption_attempts(),
            s3_ciphertext_retrieval_retries: default_s3_ciphertext_retrieval_retries(),
            s3_connect_timeout: default_s3_connect_timeout(),
            service_name: default_service_name(),
            task_limit: default_task_limit(),
            monitoring_endpoint: default_monitoring_endpoint(),
            healthcheck_timeout: default_healthcheck_timeout(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use serial_test::serial;
    use std::{env, str::FromStr};

    fn cleanup_env_vars() {
        unsafe {
            env::remove_var("KMS_CONNECTOR_DATABASE_URL");
            env::remove_var("KMS_CONNECTOR_EVENTS_BATCH_SIZE");
            env::remove_var("KMS_CONNECTOR_GATEWAY_URL");
            env::remove_var("KMS_CONNECTOR_GATEWAY_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_HOST_CHAINS");
            env::remove_var("KMS_CONNECTOR_KMS_CORE_ENDPOINTS");
            env::remove_var("KMS_CONNECTOR_GRPC_REQUEST_RETRIES");
            env::remove_var("KMS_CONNECTOR_MAX_DECRYPTION_ATTEMPTS");
            env::remove_var("KMS_CONNECTOR_S3_CIPHERTEXT_RETRIEVAL_RETRIES");
            env::remove_var("KMS_CONNECTOR_S3_CONNECT_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
        }
    }

    #[test]
    #[serial(config_tests)]
    fn test_load_valid_config_from_file() {
        cleanup_env_vars();
        let default_config = Config::default();
        let example_config = Config::from_env_and_file(Some(example_config_path())).unwrap();
        assert_eq!(default_config, example_config);
    }

    #[test]
    #[serial(config_tests)]
    fn test_load_from_env() {
        cleanup_env_vars();

        // Set environment variables
        unsafe {
            env::set_var(
                "KMS_CONNECTOR_DATABASE_URL",
                "postgres://postgres:postgres@localhost",
            );
            env::set_var("KMS_CONNECTOR_EVENTS_BATCH_SIZE", "15");
            env::set_var("KMS_CONNECTOR_GATEWAY_URL", "http://localhost:9545");
            env::set_var("KMS_CONNECTOR_GATEWAY_CHAIN_ID", "31888");
            env::set_var(
                "KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_HOST_CHAINS",
                r#"
                    [
                        {
                            "url": "http://localhost:9545",
                            "chain_id": 31888,
                            "acl_address": "0x5fbdb2315678afecb367f032d93f642f64180aa3"
                        }
                    ]
                "#,
            );
            env::set_var(
                "KMS_CONNECTOR_KMS_CORE_ENDPOINTS",
                "http://localhost:50053,http://localhost:50054",
            );
            env::set_var("KMS_CONNECTOR_GRPC_REQUEST_RETRIES", "5");
            env::set_var("KMS_CONNECTOR_MAX_DECRYPTION_ATTEMPTS", "300");
            env::set_var("KMS_CONNECTOR_S3_CIPHERTEXT_RETRIEVAL_RETRIES", "5");
            env::set_var("KMS_CONNECTOR_S3_CONNECT_TIMEOUT", "4s");
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
        }

        // Load config from environment
        let config = Config::from_env_and_file::<&str>(None).unwrap();

        // Verify values
        assert_eq!(config.events_batch_size, 15);
        assert_eq!(
            config.gateway_url,
            Url::from_str("http://localhost:9545").unwrap()
        );
        assert_eq!(config.gateway_chain_id, 31888);
        assert_eq!(
            config.decryption_contract.address,
            Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap()
        );
        assert_eq!(
            config.gateway_config_contract.address,
            Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap()
        );
        assert_eq!(
            config.kms_generation_contract.address,
            Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap()
        );
        assert_eq!(
            config.host_chains,
            vec![HostChainConfig {
                url: Url::from_str("http://localhost:9545").unwrap(),
                chain_id: 31888,
                acl_address: Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3")
                    .unwrap()
            }]
        );
        assert_eq!(
            config.kms_core_endpoints,
            vec!["http://localhost:50053", "http://localhost:50054"]
        );
        assert_eq!(config.grpc_request_retries, 5);
        assert_eq!(config.max_decryption_attempts, 300);
        assert_eq!(config.s3_ciphertext_retrieval_retries, 5);
        assert_eq!(config.s3_connect_timeout.as_secs(), 4);
        assert_eq!(config.service_name, "kms-connector-test");

        cleanup_env_vars();
    }

    #[test]
    #[serial(config_tests)]
    fn test_env_overrides_file() {
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

    #[test]
    #[serial(config_tests)]
    fn test_host_chains_from_env_camel_case() {
        cleanup_env_vars();

        // Set environment variables
        unsafe {
            env::set_var(
                "KMS_CONNECTOR_HOST_CHAINS",
                r#"
                    [
                        {
                            "url": "http://localhost:9545",
                            "chainId": 31888,
                            "aclAddress": "0x5fbdb2315678afecb367f032d93f642f64180aa3"
                        }
                    ]
                "#,
            );
        }

        // Load config from environment
        let config = Config::from_env_and_file(Some(example_config_path())).unwrap();

        // Verify values
        assert_eq!(
            config.host_chains,
            vec![HostChainConfig {
                url: Url::from_str("http://localhost:9545").unwrap(),
                chain_id: 31888,
                acl_address: Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3")
                    .unwrap()
            }]
        );
        cleanup_env_vars();
    }

    fn example_config_path() -> String {
        format!(
            "{}/../../config/kms-worker.toml",
            env!("CARGO_MANIFEST_DIR")
        )
    }
}
