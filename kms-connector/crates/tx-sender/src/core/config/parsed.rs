//! Module used to parse tx-sender configuration.
//!
//! The `raw` module is first used to deserialize the configuration.

use super::raw::RawConfig;
use connector_utils::{
    config::{AwsKmsConfig, ContractConfig, DeserializeRawConfig, Error, KmsWallet, Result},
    monitoring::otlp::default_dispatcher,
};
use std::{net::SocketAddr, path::Path, time::Duration};
use tracing::{error, info};

/// Configuration of the `TransactionSender`.
#[derive(Clone, Debug)]
pub struct Config {
    /// The URL of the Postgres database.
    pub database_url: String,
    /// The size of the database connection pool.
    pub database_pool_size: u32,
    /// The timeout for polling the database for responses.
    pub database_polling_timeout: Duration,
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
    /// The wallet used to sign the decryption responses from the kms-core.
    pub wallet: KmsWallet,
    /// The number of retries for transaction sending.
    pub tx_retries: u8,
    /// The interval between transaction retries.
    pub tx_retry_interval: Duration,
    /// The batch size for KMS Core response processing.
    pub responses_batch_size: u8,
    /// The gas multiplier percentage after each transaction attempt.
    pub gas_multiplier_percent: usize,
    /// The maximum number of tasks that can be executed concurrently.
    pub task_limit: usize,
    /// The monitoring server endpoint of the `TransactionSender`.
    pub monitoring_endpoint: SocketAddr,
    /// The timeout to perform each external service connection healthcheck.
    pub healthcheck_timeout: Duration,
}

impl Config {
    /// Loads the configuration from environment variables and optionally from a TOML file.
    ///
    /// Environment variables take precedence over file configuration.
    /// Environment variables are prefixed with KMS_CONNECTOR_.
    pub async fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> Result<Self> {
        let _dispatcher_guard = tracing::dispatcher::set_default(&default_dispatcher());
        if let Some(config_path) = &path {
            info!("Loading config from: {}", config_path.as_ref().display());
        } else {
            info!("Loading config using environment variables only");
        }

        let raw_config = RawConfig::from_env_and_file(path).inspect_err(|e| error!("{e}"))?;
        Self::parse(raw_config).await.inspect_err(|e| error!("{e}"))
    }

    async fn parse(raw_config: RawConfig) -> Result<Self> {
        let monitoring_endpoint = raw_config
            .monitoring_endpoint
            .parse::<SocketAddr>()
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;

        let wallet = Self::parse_kms_wallet(
            raw_config.chain_id,
            raw_config.private_key,
            raw_config.aws_kms_config,
        )
        .await?;

        let decryption_contract =
            ContractConfig::parse("Decryption", raw_config.decryption_contract)?;
        let kms_management_contract =
            ContractConfig::parse("KmsManagement", raw_config.kms_management_contract)?;

        // Validate critical configuration parts
        if raw_config.gateway_url.is_empty() {
            return Err(Error::EmptyField("Gateway URL".to_string()));
        }

        if raw_config.gas_multiplier_percent < 100 {
            return Err(Error::InvalidConfig(
                "gas_multiplier_percent should be greater than or equal to 100%".to_string(),
            ));
        }

        let database_polling_timeout =
            Duration::from_secs(raw_config.database_polling_timeout_secs);
        let tx_retry_interval = Duration::from_millis(raw_config.tx_retry_interval_ms);
        let healthcheck_timeout = Duration::from_secs(raw_config.healthcheck_timeout_secs);

        Ok(Self {
            database_url: raw_config.database_url,
            database_pool_size: raw_config.database_pool_size,
            database_polling_timeout,
            gateway_url: raw_config.gateway_url,
            chain_id: raw_config.chain_id,
            decryption_contract,
            kms_management_contract,
            service_name: raw_config.service_name,
            wallet,
            tx_retries: raw_config.tx_retries,
            tx_retry_interval,
            responses_batch_size: raw_config.responses_batch_size,
            gas_multiplier_percent: raw_config.gas_multiplier_percent,
            task_limit: raw_config.task_limit,
            monitoring_endpoint,
            healthcheck_timeout,
        })
    }

    async fn parse_kms_wallet(
        chain_id: u64,
        private_key: Option<String>,
        aws_kms_config: Option<AwsKmsConfig>,
    ) -> Result<KmsWallet> {
        let chain_id = Some(chain_id);
        if let Some(private_key) = private_key {
            KmsWallet::from_private_key_str(&private_key, chain_id)
        } else if let Some(aws_kms_config) = aws_kms_config {
            KmsWallet::from_aws_kms(aws_kms_config, chain_id).await
        } else {
            Err(Error::InvalidConfig(
                "Either AWS KMS or private key must be configured".into(),
            ))
        }
    }

    // Default implementation for testing purpose
    pub async fn default() -> Self {
        Self::parse(RawConfig::default())
            .await
            .expect("Failed to parse default RawConfig")
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
            env::remove_var("KMS_CONNECTOR_PRIVATE_KEY");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_KMS_MANAGEMENT_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
            env::remove_var("KMS_CONNECTOR_RESPONSES_BATCH_SIZE");
            env::remove_var("KMS_CONNECTOR_TX_RETRIES");
            env::remove_var("KMS_CONNECTOR_TX_RETRY_INTERVAL_MS");
            env::remove_var("KMS_CONNECTOR_GAS_MULTIPLIER_PERCENT");
        }
    }

    #[tokio::test]
    #[serial(config_tests)]
    async fn test_load_valid_config_from_file() {
        cleanup_env_vars();
        let raw_config = RawConfig::default();

        let temp_file = NamedTempFile::new().unwrap();
        raw_config.to_file(temp_file.path()).unwrap();
        let config = Config::from_env_and_file(Some(temp_file.path()))
            .await
            .unwrap();

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
        assert_eq!(raw_config.responses_batch_size, config.responses_batch_size);
        assert_eq!(raw_config.tx_retries, config.tx_retries);
        assert_eq!(
            raw_config.tx_retry_interval_ms as u128,
            config.tx_retry_interval.as_millis()
        );
        assert_eq!(
            raw_config.gas_multiplier_percent,
            config.gas_multiplier_percent
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
                "KMS_CONNECTOR_PRIVATE_KEY",
                "8355bb293b8714a06b972bfe692d1bd9f24235c1f4007ae0be285d398b0bba2f",
            );
            env::set_var(
                "KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_KMS_MANAGEMENT_CONTRACT__ADDRESS",
                "0x0000000000000000000000000000000000000002",
            );
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
            env::set_var("KMS_CONNECTOR_RESPONSES_BATCH_SIZE", "20");
            env::set_var("KMS_CONNECTOR_TX_RETRIES", "5");
            env::set_var("KMS_CONNECTOR_TX_RETRY_INTERVAL_MS", "200");
            env::set_var("KMS_CONNECTOR_GAS_MULTIPLIER_PERCENT", "180");
        }

        // Load config from environment
        let config = Config::from_env_and_file::<&str>(None).await.unwrap();

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
        assert_eq!(config.responses_batch_size, 20);
        assert_eq!(config.tx_retries, 5);
        assert_eq!(config.tx_retry_interval, Duration::from_millis(200));
        assert_eq!(config.gas_multiplier_percent, 180);

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
        let config = Config::from_env_and_file(Some(temp_file.path()))
            .await
            .unwrap();

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
            Config::parse(raw_config).await,
            Err(Error::InvalidConfig(_))
        ));
    }

    #[tokio::test]
    #[serial(config_tests)]
    async fn test_invalid_wallet() {
        let raw_config = RawConfig {
            private_key: None,
            ..Default::default()
        };
        assert!(matches!(
            Config::parse(raw_config).await,
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
