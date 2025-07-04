//! Module used to parse kms-worker configuration.
//!
//! The `raw` module is first used to deserialize the configuration.

use crate::core::config::raw::{RawConfig, S3Config};
use connector_utils::config::{ContractConfig, DeserializeRawConfig, Error, Result};
use std::{
    fmt::{self, Display},
    path::Path,
    time::Duration,
};
use tracing::info;

/// Configuration of the `KmsWorker`.
#[derive(Clone, Debug)]
pub struct Config {
    /// The URL of the Postgres database.
    pub database_url: String,
    /// The size of the database connection pool.
    pub database_pool_size: u32,
    /// The Gateway RPC endpoint.
    pub gateway_url: String,
    /// The KMS Core endpoint.
    pub kms_core_endpoint: String,
    /// The Chain ID of the Gateway.
    pub chain_id: u64,
    /// The `Decryption` contract configuration.
    pub decryption_contract: ContractConfig,
    /// The `GatewayConfig` contract configuration.
    pub gateway_config_contract: ContractConfig,
    /// The service name used for tracing.
    pub service_name: String,

    /// The limit number of events to fetch from the database (default: 10).
    pub events_batch_size: u8,
    /// Number of retries for GRPC requests sent to the KMS Core (default: 3).
    pub grpc_request_retries: u8,
    /// Timeout to get public decryption responses from KMS Core (default: 300s / 5min).
    pub public_decryption_timeout: Duration,
    /// Timeout to get user decryption responses from KMS Core (default: 300s / 5min).
    pub user_decryption_timeout: Duration,
    /// Retry interval to poll GRPC responses from KMS Core (default: 5s).
    pub grpc_poll_interval: Duration,

    /// S3 configuration for ciphertext storage (optional).
    pub s3_config: Option<S3Config>,
    /// Number of retries for S3 ciphertext retrieval (default: 3).
    pub s3_ciphertext_retrieval_retries: u8,
    /// Timeout to connect to a S3 bucket (default: 2s).
    pub s3_connect_timeout: Duration,

    // TODO: implement to increase security
    /// Whether to verify coprocessors against the `GatewayConfig` contract (defaults to false).
    pub verify_coprocessors: bool,
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Service Name: {}", self.service_name)?;
        writeln!(f, "Database URL: {}", self.database_url)?;
        writeln!(
            f,
            "  Database connection pool size: {}",
            self.database_pool_size
        )?;
        writeln!(f, "KMS Core Endpoint: {}", self.kms_core_endpoint)?;
        writeln!(f, "Gateway URL: {}", self.gateway_url)?;
        writeln!(f, "Chain ID: {}", self.chain_id)?;
        writeln!(f, "{}", self.decryption_contract)?;
        writeln!(f, "{}", self.gateway_config_contract)?;
        write!(f, "Events batch size: {}", self.events_batch_size)?;
        write!(f, "GRPC Requests Retries: {}", self.grpc_request_retries)?;
        writeln!(
            f,
            "Public Decryption Timeout: {}s",
            self.public_decryption_timeout.as_secs()
        )?;
        writeln!(
            f,
            "User Decryption Timeout: {}s",
            self.user_decryption_timeout.as_secs()
        )?;
        write!(
            f,
            "GRPC Poll Interval: {}s",
            self.grpc_poll_interval.as_secs()
        )?;
        writeln!(
            f,
            "Number of retries for S3 ciphertext retrieval: {}",
            self.s3_ciphertext_retrieval_retries
        )?;
        writeln!(
            f,
            "S3 ciphertext retrieval timeout: {}s",
            self.s3_connect_timeout.as_secs()
        )?;

        Ok(())
    }
}

impl Config {
    /// Loads the configuration from environment variables and optionally from a TOML file.
    ///
    /// Environment variables take precedence over file configuration.
    /// Environment variables are prefixed with KMS_CONNECTOR_.
    pub fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> Result<Self> {
        if let Some(config_path) = &path {
            info!("Loading config from: {}", config_path.as_ref().display());
        } else {
            info!("Loading config using environment variables only");
        }

        let raw_config = RawConfig::from_env_and_file(path)?;
        Self::parse(raw_config)
    }

    fn parse(raw_config: RawConfig) -> Result<Self> {
        let decryption_contract =
            ContractConfig::parse("Decryption", raw_config.decryption_contract)?;
        let gateway_config_contract =
            ContractConfig::parse("GatewayConfig", raw_config.gateway_config_contract)?;

        // Validate critical configuration parts
        if raw_config.gateway_url.is_empty() {
            return Err(Error::EmptyField("Gateway URL".to_string()));
        }

        if raw_config.kms_core_endpoint.is_empty() {
            return Err(Error::EmptyField("KMS Core endpoint".to_string()));
        }

        let public_decryption_timeout =
            Duration::from_secs(raw_config.public_decryption_timeout_secs);
        let user_decryption_timeout = Duration::from_secs(raw_config.user_decryption_timeout_secs);
        let grpc_retry_interval = Duration::from_secs(raw_config.grpc_poll_interval_secs);
        let s3_ciphertext_retrieval_timeout = Duration::from_secs(raw_config.s3_connect_timeout);

        Ok(Self {
            database_url: raw_config.database_url,
            database_pool_size: raw_config.database_pool_size,
            gateway_url: raw_config.gateway_url,
            kms_core_endpoint: raw_config.kms_core_endpoint,
            chain_id: raw_config.chain_id,
            decryption_contract,
            gateway_config_contract,
            service_name: raw_config.service_name,
            events_batch_size: raw_config.events_batch_size,
            grpc_request_retries: raw_config.grpc_request_retries,
            public_decryption_timeout,
            user_decryption_timeout,
            grpc_poll_interval: grpc_retry_interval,
            s3_config: raw_config.s3_config,
            s3_ciphertext_retrieval_retries: raw_config.s3_ciphertext_retrieval_retries,
            s3_connect_timeout: s3_ciphertext_retrieval_timeout,
            verify_coprocessors: raw_config.verify_coprocessors,
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
    use std::{env, fs, str::FromStr};
    use tempfile::NamedTempFile;

    fn cleanup_env_vars() {
        unsafe {
            env::remove_var("KMS_CONNECTOR_DATABASE_URL");
            env::remove_var("KMS_CONNECTOR_GATEWAY_URL");
            env::remove_var("KMS_CONNECTOR_KMS_CORE_ENDPOINT");
            env::remove_var("KMS_CONNECTOR_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
            env::remove_var("KMS_CONNECTOR_S3_CONFIG__REGION");
            env::remove_var("KMS_CONNECTOR_S3_CONFIG__BUCKET");
            env::remove_var("KMS_CONNECTOR_EVENTS_BATCH_SIZE");
            env::remove_var("KMS_CONNECTOR_GRPC_REQUEST_RETRIES");
            env::remove_var("KMS_CONNECTOR_PUBLIC_DECRYPTION_TIMEOUT_SECS");
            env::remove_var("KMS_CONNECTOR_USER_DECRYPTION_TIMEOUT_SECS");
            env::remove_var("KMS_CONNECTOR_GRPC_POLL_INTERVAL_SECS");
            env::remove_var("KMS_CONNECTOR_S3_CIPHERTEXT_RETRIEVAL_RETRIES");
            env::remove_var("KMS_CONNECTOR_S3_CONNECT_TIMEOUT");
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
        assert_eq!(raw_config.kms_core_endpoint, config.kms_core_endpoint);
        assert_eq!(raw_config.chain_id, config.chain_id);
        assert_eq!(
            Address::from_str(&raw_config.decryption_contract.address).unwrap(),
            config.decryption_contract.address,
        );
        assert_eq!(
            Address::from_str(&raw_config.gateway_config_contract.address).unwrap(),
            config.gateway_config_contract.address,
        );
        assert_eq!(raw_config.kms_core_endpoint, config.kms_core_endpoint);
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
        assert_eq!(raw_config.s3_config, config.s3_config);
        assert_eq!(raw_config.verify_coprocessors, config.verify_coprocessors);
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
            env::set_var("KMS_CONNECTOR_KMS_CORE_ENDPOINT", "http://localhost:50053");
            env::set_var("KMS_CONNECTOR_CHAIN_ID", "31888");
            env::set_var(
                "KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS",
                "0x0000000000000000000000000000000000000001",
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
        assert_eq!(config.kms_core_endpoint, "http://localhost:50053");
        assert_eq!(config.chain_id, 31888);
        assert_eq!(
            config.decryption_contract.address,
            Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap()
        );
        assert_eq!(
            config.gateway_config_contract.address,
            Address::from_str("0x0000000000000000000000000000000000000001").unwrap()
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
            env::set_var("KMS_CONNECTOR_S3_CONFIG__REGION", "test-region");
            env::set_var("KMS_CONNECTOR_S3_CONFIG__BUCKET", "test-bucket");
        }

        // Load config from both sources
        let config = Config::from_env_and_file(Some(temp_file.path())).unwrap();

        // Verify that environment variables take precedence
        assert_eq!(config.chain_id, 77737);
        assert_eq!(config.service_name, "kms-connector-override");
        assert_eq!(config.s3_config.as_ref().unwrap().region, "test-region");
        assert_eq!(config.s3_config.as_ref().unwrap().bucket, "test-bucket");

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
