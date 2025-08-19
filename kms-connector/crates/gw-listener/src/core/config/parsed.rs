//! Module used to parse the gw-listener configuration.
//!
//! The `raw` module is first used to deserialize the configuration.

use super::raw::RawConfig;
use connector_utils::{
    config::{ContractConfig, DeserializeRawConfig, Error, Result},
    monitoring::otlp::default_dispatcher,
};
use std::{net::SocketAddr, path::Path, time::Duration};
use tracing::{error, info};

/// Configuration of the `GatewayListener`.
#[derive(Clone, Debug)]
pub struct Config {
    /// The URL of the Postgres database.
    pub database_url: String,
    /// The size of the database connection pool.
    pub database_pool_size: u32,
    /// The Gateway RPC endpoint.
    pub gateway_url: String,
    /// The Chain ID of the Gateway.
    pub chain_id: u64,
    /// The `Decryption` contract configuration.
    pub decryption_contract: ContractConfig,
    /// The `KmsManagement` contract configuration.
    pub kms_management_contract: ContractConfig,
    /// The service name used for tracing.
    pub service_name: String,
    /// The maximum number of tasks that can be executed concurrently.
    pub task_limit: usize,
    /// The monitoring server endpoint of the `GatewayListener`.
    pub monitoring_endpoint: SocketAddr,
    /// The timeout to perform each external service connection healthcheck.
    pub healthcheck_timeout: Duration,
    /// The polling interval for decryption requests.
    pub decryption_polling: Duration,
    /// The polling interval for key management requests.
    pub key_management_polling: Duration,
    /// Optional block number to start processing from.
    pub from_block_number: Option<u64>,
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
        let kms_management_contract =
            ContractConfig::parse("KmsManagement", raw_config.kms_management_contract)?;

        // Validate critical configuration parts
        if raw_config.gateway_url.is_empty() {
            return Err(Error::EmptyField("Gateway URL".to_string()));
        }

        let healthcheck_timeout = Duration::from_secs(raw_config.healthcheck_timeout_secs);
        let decryption_polling = Duration::from_millis(raw_config.decryption_polling_ms);
        let key_management_polling = Duration::from_millis(raw_config.key_management_polling_ms);

        Ok(Self {
            database_url: raw_config.database_url,
            database_pool_size: raw_config.database_pool_size,
            gateway_url: raw_config.gateway_url,
            chain_id: raw_config.chain_id,
            decryption_contract,
            kms_management_contract,
            service_name: raw_config.service_name,
            task_limit: raw_config.task_limit,
            monitoring_endpoint,
            healthcheck_timeout,
            decryption_polling,
            key_management_polling,
            from_block_number: raw_config.from_block_number,
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
            env::remove_var("KMS_CONNECTOR_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_KMS_MANAGEMENT_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
        }
    }

    #[tokio::test]
    #[serial(config_tests)]
    async fn test_load_valid_config_from_file() {
        cleanup_env_vars();
        let raw_config = RawConfig::default();

        let temp_file = NamedTempFile::new().unwrap();
        raw_config.to_file(temp_file.path()).unwrap();
        let config = Config::from_env_and_file(Some(temp_file.path())).unwrap();

        // Compare fields
        assert_eq!(raw_config.gateway_url, config.gateway_url);
        assert_eq!(raw_config.chain_id, config.chain_id);
        assert_eq!(
            Address::from_str(&raw_config.decryption_contract.address).unwrap(),
            config.decryption_contract.address,
        );
        assert_eq!(
            Address::from_str(&raw_config.kms_management_contract.address).unwrap(),
            config.kms_management_contract.address,
        );
        assert_eq!(raw_config.service_name, config.service_name);
        assert_eq!(
            raw_config.decryption_contract.domain_name.unwrap(),
            config.decryption_contract.domain_name,
        );
        assert_eq!(
            raw_config.decryption_contract.domain_version.unwrap(),
            config.decryption_contract.domain_version,
        );
        assert_eq!(
            raw_config.kms_management_contract.domain_name.unwrap(),
            config.kms_management_contract.domain_name,
        );
        assert_eq!(
            raw_config.kms_management_contract.domain_version.unwrap(),
            config.kms_management_contract.domain_version,
        );
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
            env::set_var("KMS_CONNECTOR_GATEWAY_URL", "ws://localhost:9545");
            env::set_var("KMS_CONNECTOR_CHAIN_ID", "31888");
            env::set_var(
                "KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_KMS_MANAGEMENT_CONTRACT__ADDRESS",
                "0x0000000000000000000000000000000000000002",
            );
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
        }

        // Load config from environment
        let config = Config::from_env_and_file::<&str>(None).unwrap();

        // Verify values
        assert_eq!(config.gateway_url, "ws://localhost:9545");
        assert_eq!(config.chain_id, 31888);
        assert_eq!(
            config.decryption_contract.address,
            Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap()
        );
        assert_eq!(
            config.kms_management_contract.address,
            Address::from_str("0x0000000000000000000000000000000000000002").unwrap()
        );
        assert_eq!(config.service_name, "kms-connector-test");

        cleanup_env_vars();
    }

    #[tokio::test]
    #[serial(config_tests)]
    async fn test_env_overrides_file() {
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

    #[tokio::test]
    #[serial(config_tests)]
    async fn test_invalid_address() {
        let raw_config = RawConfig {
            decryption_contract: RawContractConfig {
                address: "0x0000".to_string(),
                ..Default::default()
            },
            kms_management_contract: RawContractConfig {
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
