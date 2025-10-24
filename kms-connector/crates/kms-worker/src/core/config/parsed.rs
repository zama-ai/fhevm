//! Module used to parse kms-worker configuration.
//!
//! The `raw` module is first used to deserialize the configuration.

use crate::core::config::raw::RawConfig;
use connector_utils::{
    config::{ContractConfig, DeserializeRawConfig, Error, Result},
    monitoring::otlp::default_dispatcher,
};
use std::{net::SocketAddr, path::Path, time::Duration};
use tracing::{error, info, warn};

/// Configuration of the `KmsWorker`.
#[derive(Clone, Debug)]
pub struct Config {
    /// The URL of the Postgres database.
    pub database_url: String,
    /// The size of the database connection pool.
    pub database_pool_size: u32,
    /// The timeout for polling the database for events.
    pub database_polling_timeout: Duration,
    /// The Gateway RPC endpoint.
    pub gateway_url: String,
    /// The KMS Core endpoints.
    pub kms_core_endpoints: Vec<String>,
    /// The Chain ID of the Gateway.
    pub chain_id: u64,
    /// The `Decryption` contract configuration.
    pub decryption_contract: ContractConfig,
    /// The `GatewayConfig` contract configuration.
    pub gateway_config_contract: ContractConfig,
    /// The `KMSGeneration` contract configuration.
    pub kms_generation_contract: ContractConfig,
    /// The service name used for tracing.
    pub service_name: String,

    /// The limit number of events to fetch from the database.
    pub events_batch_size: u8,
    /// Number of retries for GRPC requests sent to the KMS Core.
    pub grpc_request_retries: u8,
    /// Timeout to get public decryption responses from KMS Core.
    pub public_decryption_timeout: Duration,
    /// Timeout to get user decryption responses from KMS Core.
    pub user_decryption_timeout: Duration,
    /// Retry interval to poll GRPC responses from KMS Core.
    pub grpc_poll_interval: Duration,

    /// Number of retries for S3 ciphertext retrieval.
    pub s3_ciphertext_retrieval_retries: u8,
    /// Timeout to connect to a S3 bucket.
    pub s3_connect_timeout: Duration,

    /// The maximum number of tasks that can be executed concurrently.
    pub task_limit: usize,

    /// The monitoring server endpoint of the `KmsWorker`.
    pub monitoring_endpoint: SocketAddr,
    /// The timeout to perform each external service connection healthcheck.
    pub healthcheck_timeout: Duration,
}

impl Config {
    /// Loads the configuration from environment variables and optionally from a TOML file.
    ///
    /// Environment variables take precedence over file configuration.
    /// Environment variables are prefixed with KMS_CONNECTOR_.
    pub fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> Result<Self> {
        tracing::dispatcher::with_default(&default_dispatcher(), || {
            if let Some(config_path) = &path {
                info!("Loading config from: {}", config_path.as_ref().display());
            } else {
                info!("Loading config using environment variables only");
            }

            let raw_config = RawConfig::from_env_and_file(path).inspect_err(|e| error!("{e}"))?;
            Self::parse(raw_config).inspect_err(|e| error!("{e}"))
        })
    }

    fn parse(raw_config: RawConfig) -> Result<Self> {
        let monitoring_endpoint = raw_config
            .monitoring_endpoint
            .parse::<SocketAddr>()
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        let decryption_contract =
            ContractConfig::parse("Decryption", raw_config.decryption_contract)?;
        let gateway_config_contract =
            ContractConfig::parse("GatewayConfig", raw_config.gateway_config_contract)?;
        let kms_generation_contract =
            ContractConfig::parse("KMSGeneration", raw_config.kms_generation_contract)?;

        // Validate critical configuration parts
        if raw_config.gateway_url.is_empty() {
            return Err(Error::EmptyField("Gateway URL".to_string()));
        }

        let kms_core_endpoints;
        if raw_config.kms_core_endpoints.is_empty() {
            if let Some(kms_core_endpoint) = raw_config.kms_core_endpoint {
                warn!("Using deprecated `kms_core_endpoint` field instead of `kms_core_endpoints`");
                kms_core_endpoints = vec![kms_core_endpoint];
            } else {
                return Err(Error::EmptyField("KMS Core endpoints".to_string()));
            }
        } else {
            kms_core_endpoints = raw_config.kms_core_endpoints;
        }

        let database_polling_timeout =
            Duration::from_secs(raw_config.database_polling_timeout_secs);
        let public_decryption_timeout =
            Duration::from_secs(raw_config.public_decryption_timeout_secs);
        let user_decryption_timeout = Duration::from_secs(raw_config.user_decryption_timeout_secs);
        let grpc_poll_interval = Duration::from_secs(raw_config.grpc_poll_interval_secs);
        let s3_ciphertext_retrieval_timeout = Duration::from_secs(raw_config.s3_connect_timeout);
        let healthcheck_timeout = Duration::from_secs(raw_config.healthcheck_timeout_secs);

        Ok(Self {
            database_url: raw_config.database_url,
            database_pool_size: raw_config.database_pool_size,
            database_polling_timeout,
            gateway_url: raw_config.gateway_url,
            kms_core_endpoints,
            chain_id: raw_config.chain_id,
            decryption_contract,
            gateway_config_contract,
            kms_generation_contract,
            service_name: raw_config.service_name,
            events_batch_size: raw_config.events_batch_size,
            grpc_request_retries: raw_config.grpc_request_retries,
            public_decryption_timeout,
            user_decryption_timeout,
            grpc_poll_interval,
            s3_ciphertext_retrieval_retries: raw_config.s3_ciphertext_retrieval_retries,
            s3_connect_timeout: s3_ciphertext_retrieval_timeout,
            task_limit: raw_config.task_limit,
            monitoring_endpoint,
            healthcheck_timeout,
        })
    }
}

// For testing purpose
impl Default for Config {
    fn default() -> Self {
        Self::parse(RawConfig::default()).expect("Couldn't parse default raw config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use connector_utils::config::RawContractConfig;
    use serial_test::serial;
    use std::{env, fs, path::Path, str::FromStr};
    use tempfile::NamedTempFile;

    fn cleanup_env_vars() {
        unsafe {
            env::remove_var("KMS_CONNECTOR_DATABASE_URL");
            env::remove_var("KMS_CONNECTOR_GATEWAY_URL");
            env::remove_var("KMS_CONNECTOR_KMS_CORE_ENDPOINTS");
            env::remove_var("KMS_CONNECTOR_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
            env::remove_var("KMS_CONNECTOR_EVENTS_BATCH_SIZE");
            env::remove_var("KMS_CONNECTOR_GRPC_REQUEST_RETRIES");
            env::remove_var("KMS_CONNECTOR_PUBLIC_DECRYPTION_TIMEOUT_SECS");
            env::remove_var("KMS_CONNECTOR_USER_DECRYPTION_TIMEOUT_SECS");
            env::remove_var("KMS_CONNECTOR_GRPC_POLL_INTERVAL_SECS");
            env::remove_var("KMS_CONNECTOR_S3_CIPHERTEXT_RETRIEVAL_RETRIES");
            env::remove_var("KMS_CONNECTOR_S3_CONNECT_TIMEOUT");
        }
    }

    #[test]
    #[serial(config_tests)]
    fn test_load_valid_config_from_file() {
        cleanup_env_vars();
        let raw_config = RawConfig::default();

        let temp_file = NamedTempFile::new().unwrap();
        raw_config.to_file(temp_file.path()).unwrap();
        let config = Config::from_env_and_file(Some(temp_file.path())).unwrap();

        // Compare fields
        assert_eq!(raw_config.gateway_url, config.gateway_url);
        assert_eq!(raw_config.kms_core_endpoints, config.kms_core_endpoints);
        assert_eq!(raw_config.chain_id, config.chain_id);
        assert_eq!(
            Address::from_str(&raw_config.decryption_contract.address).unwrap(),
            config.decryption_contract.address,
        );
        assert_eq!(
            Address::from_str(&raw_config.gateway_config_contract.address).unwrap(),
            config.gateway_config_contract.address,
        );
        assert_eq!(raw_config.kms_core_endpoints, config.kms_core_endpoints);
        assert_eq!(raw_config.service_name, config.service_name);
        assert_eq!(
            raw_config.public_decryption_timeout_secs,
            config.public_decryption_timeout.as_secs()
        );
        assert_eq!(
            raw_config.user_decryption_timeout_secs,
            config.user_decryption_timeout.as_secs()
        );
        assert_eq!(
            raw_config.grpc_poll_interval_secs,
            config.grpc_poll_interval.as_secs()
        );
        assert_eq!(
            raw_config.decryption_contract.domain_name.unwrap(),
            config.decryption_contract.domain_name,
        );
        assert_eq!(
            raw_config.decryption_contract.domain_version.unwrap(),
            config.decryption_contract.domain_version,
        );
        assert_eq!(
            raw_config.gateway_config_contract.domain_name.unwrap(),
            config.gateway_config_contract.domain_name,
        );
        assert_eq!(
            raw_config.gateway_config_contract.domain_version.unwrap(),
            config.gateway_config_contract.domain_version,
        );
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
            env::set_var("KMS_CONNECTOR_GATEWAY_URL", "ws://localhost:9545");
            env::set_var(
                "KMS_CONNECTOR_KMS_CORE_ENDPOINTS",
                "http://localhost:50053,http://localhost:50054",
            );
            env::set_var("KMS_CONNECTOR_CHAIN_ID", "31888");
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
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
            env::set_var("KMS_CONNECTOR_EVENTS_BATCH_SIZE", "15");
            env::set_var("KMS_CONNECTOR_GRPC_REQUEST_RETRIES", "5");
            env::set_var("KMS_CONNECTOR_PUBLIC_DECRYPTION_TIMEOUT_SECS", "600");
            env::set_var("KMS_CONNECTOR_USER_DECRYPTION_TIMEOUT_SECS", "600");
            env::set_var("KMS_CONNECTOR_GRPC_POLL_INTERVAL_SECS", "10");
            env::set_var("KMS_CONNECTOR_S3_CIPHERTEXT_RETRIEVAL_RETRIES", "5");
            env::set_var("KMS_CONNECTOR_S3_CONNECT_TIMEOUT", "4");
        }

        // Load config from environment
        let config = Config::from_env_and_file::<&str>(None).unwrap();

        // Verify values
        assert_eq!(config.gateway_url, "ws://localhost:9545");
        assert_eq!(
            config.kms_core_endpoints,
            vec!["http://localhost:50053", "http://localhost:50054"]
        );
        assert_eq!(config.chain_id, 31888);
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
        assert_eq!(config.service_name, "kms-connector-test");
        assert_eq!(config.events_batch_size, 15);
        assert_eq!(config.grpc_request_retries, 5);
        assert_eq!(config.public_decryption_timeout.as_secs(), 600);
        assert_eq!(config.user_decryption_timeout.as_secs(), 600);
        assert_eq!(config.grpc_poll_interval.as_secs(), 10);
        assert_eq!(config.s3_ciphertext_retrieval_retries, 5);
        assert_eq!(config.s3_connect_timeout.as_secs(), 4);

        cleanup_env_vars();
    }

    #[test]
    #[serial(config_tests)]
    fn test_env_overrides_file() {
        cleanup_env_vars();

        // Create a temp config file
        let raw_config = RawConfig::default();

        let temp_file = NamedTempFile::new().unwrap();
        raw_config.to_file(temp_file.path()).unwrap();

        // Set an environment variable to override the file
        unsafe {
            env::set_var("KMS_CONNECTOR_CHAIN_ID", "77737");
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-override");
        }

        // Load config from both sources
        let config = Config::from_env_and_file(Some(temp_file.path())).unwrap();

        // Verify that environment variables take precedence
        assert_eq!(config.chain_id, 77737);
        assert_eq!(config.service_name, "kms-connector-override");

        // File values should be used for non-overridden fields
        assert_eq!(config.gateway_url, "ws://localhost:8545");

        cleanup_env_vars();
    }

    #[test]
    #[serial(config_tests)]
    fn test_invalid_address() {
        let raw_config = RawConfig {
            decryption_contract: RawContractConfig {
                address: "0x0000".to_string(),
                ..Default::default()
            },
            gateway_config_contract: RawContractConfig {
                address: "0x000010".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(matches!(
            Config::parse(raw_config),
            Err(Error::InvalidConfig(_))
        ));
    }

    #[test]
    #[serial(config_tests)]
    fn test_kms_core_endpoint_fallback() {
        let raw_config = RawConfig {
            kms_core_endpoints: vec![],
            kms_core_endpoint: Some("http://localhost:50053".to_string()),
            ..Default::default()
        };
        let config = Config::parse(raw_config.clone()).unwrap();
        assert_eq!(
            config.kms_core_endpoints,
            vec![raw_config.kms_core_endpoint.unwrap()]
        )
    }

    impl RawConfig {
        pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
            let content = toml::to_string_pretty(self)
                .map_err(|e| Error::InvalidConfig(format!("Failed to serialize config: {e}")))?;

            fs::write(path, content)
                .map_err(|e| Error::InvalidConfig(format!("Failed to write config file: {e}")))?;

            Ok(())
        }
    }
}
