use alloy::transports::http::reqwest::Url;
use connector_utils::{
    config::{
        AwsKmsConfig, ContractConfig, DeserializeConfig, Error, KmsWallet,
        contract::{
            default_decryption_contract_config, default_kms_generation_contract_config,
            deserialize_decryption_contract_config, deserialize_kms_generation_contract_config,
        },
        default_database_pool_size, deserialize_pg_interval, serialize_pg_interval,
    },
    monitoring::{health::default_healthcheck_timeout, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::postgres::types::PgInterval;
use std::{net::SocketAddr, str::FromStr, time::Duration};

/// Configuration of the `TransactionSender`.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct Config {
    /// The URL of the Postgres database.
    pub database_url: String,
    /// The size of the database connection pool.
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,
    /// The timeout for polling the database for responses.
    #[serde(with = "humantime_serde", default = "default_database_polling_timeout")]
    pub database_polling_timeout: Duration,

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
    /// The private key of the `TransactionSender`'s wallet.
    pub private_key: Option<String>,
    /// The AWS KMS configuration of the `TransactionSender`'s wallet.
    pub aws_kms_config: Option<AwsKmsConfig>,

    /// The number of retries for transaction sending.
    #[serde(default = "default_tx_retries")]
    pub tx_retries: u8,
    /// The interval between transaction retries.
    #[serde(with = "humantime_serde", default = "default_tx_retry_interval")]
    pub tx_retry_interval: Duration,
    /// Enable tracing of reverted transactions.
    #[serde(default = "default_trace_reverted_tx")]
    pub trace_reverted_tx: bool,
    /// The batch size for KMS Core response processing.
    #[serde(default = "default_responses_batch_size")]
    pub responses_batch_size: u8,
    /// The gas multiplier percentage after each transaction attempt.
    #[serde(
        deserialize_with = "parse_gas_multiplier_percent",
        default = "default_gas_multiplier_percent"
    )]
    pub gas_multiplier_percent: usize,

    /// The service name used for tracing.
    #[serde(default = "default_service_name")]
    pub service_name: String,
    /// The maximum number of tasks that can be executed concurrently.
    #[serde(default = "default_task_limit")]
    pub task_limit: usize,

    /// The interval between garbage collection runs.
    #[serde(with = "humantime_serde", default = "default_gc_run_interval")]
    pub gc_run_interval: Duration,
    /// The expiration time for completed/failed decryptions, after which they will be deleted.
    #[serde(
        deserialize_with = "deserialize_pg_interval",
        serialize_with = "serialize_pg_interval",
        default = "default_gc_decryption_expiry"
    )]
    pub gc_decryption_expiry: PgInterval,
    /// The time limit for decryption to be under process, after which they will be considered as
    /// pending again.
    #[serde(
        deserialize_with = "deserialize_pg_interval",
        serialize_with = "serialize_pg_interval",
        default = "default_gc_decryption_under_process_limit"
    )]
    pub gc_decryption_under_process_limit: PgInterval,

    /// The monitoring server endpoint of the `TransactionSender`.
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: SocketAddr,
    /// The interval between gauge updates.
    #[serde(with = "humantime_serde", default = "default_gauge_update_interval")]
    pub gauge_update_interval: Duration,
    /// The timeout to perform each external service connection healthcheck.
    #[serde(with = "humantime_serde", default = "default_healthcheck_timeout")]
    pub healthcheck_timeout: Duration,
}

fn parse_gas_multiplier_percent<'de, D>(d: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let gas_multiplier_percent = <usize>::deserialize(d)?;
    if gas_multiplier_percent < 100 {
        Err(serde::de::Error::custom(
            "`gas_multiplier_percent` must be greater than or equal to 100%",
        ))
    } else {
        Ok(gas_multiplier_percent)
    }
}

impl DeserializeConfig for Config {}

impl Config {
    pub async fn build_wallet(&self) -> Result<KmsWallet, Error> {
        let chain_id = Some(self.gateway_chain_id);
        if let Some(private_key) = &self.private_key {
            KmsWallet::from_private_key_str(private_key, chain_id)
        } else if let Some(aws_kms_config) = self.aws_kms_config.clone() {
            KmsWallet::from_aws_kms(aws_kms_config, chain_id).await
        } else {
            Err(Error::InvalidConfig(
                "Either AWS KMS or private key must be configured".into(),
            ))
        }
    }
}

fn default_service_name() -> String {
    "kms-connector-tx-sender".to_string()
}

fn default_database_polling_timeout() -> Duration {
    Duration::from_secs(5)
}

fn default_tx_retries() -> u8 {
    4
}

fn default_tx_retry_interval() -> Duration {
    Duration::from_millis(10)
}

fn default_trace_reverted_tx() -> bool {
    true
}

fn default_responses_batch_size() -> u8 {
    50
}

fn default_gas_multiplier_percent() -> usize {
    115 // 115% gas increase by default
}

fn default_gauge_update_interval() -> Duration {
    Duration::from_secs(10)
}

fn default_gc_run_interval() -> Duration {
    Duration::from_mins(5)
}

fn default_gc_decryption_expiry() -> PgInterval {
    PgInterval::try_from(Duration::from_hours(24)).unwrap()
}

fn default_gc_decryption_under_process_limit() -> PgInterval {
    PgInterval::try_from(Duration::from_mins(6)).unwrap()
}

// Default implementation for testing purpose
impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost/kms-connector".to_string(),
            database_pool_size: default_database_pool_size(),
            database_polling_timeout: default_database_polling_timeout(),
            gateway_url: Url::from_str("http://localhost:8545").unwrap(),
            gateway_chain_id: 54321,
            decryption_contract: default_decryption_contract_config(),
            kms_generation_contract: default_kms_generation_contract_config(),
            service_name: default_service_name(),
            private_key: Some(
                "0x3f45b129a7fd099146e9fe63851a71646231f7743c712695f3b2d2bf0e41c774".to_string(),
            ),
            aws_kms_config: None,
            tx_retries: default_tx_retries(),
            tx_retry_interval: default_tx_retry_interval(),
            trace_reverted_tx: default_trace_reverted_tx(),
            responses_batch_size: default_responses_batch_size(),
            gas_multiplier_percent: default_gas_multiplier_percent(),
            task_limit: default_task_limit(),
            gc_run_interval: default_gc_run_interval(),
            gc_decryption_expiry: default_gc_decryption_expiry(),
            gc_decryption_under_process_limit: default_gc_decryption_under_process_limit(),
            monitoring_endpoint: default_monitoring_endpoint(),
            gauge_update_interval: default_gauge_update_interval(),
            healthcheck_timeout: default_healthcheck_timeout(),
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
            env::remove_var("KMS_CONNECTOR_PRIVATE_KEY");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
            env::remove_var("KMS_CONNECTOR_RESPONSES_BATCH_SIZE");
            env::remove_var("KMS_CONNECTOR_TX_RETRIES");
            env::remove_var("KMS_CONNECTOR_TX_RETRY_INTERVAL");
            env::remove_var("KMS_CONNECTOR_TRACE_REVERTED_TX");
            env::remove_var("KMS_CONNECTOR_GAS_MULTIPLIER_PERCENT");
            env::remove_var("KMS_CONNECTOR_GAUGE_UPDATE_INTERVAL");
            env::remove_var("KMS_CONNECTOR_GC_RUN_INTERVAL");
            env::remove_var("KMS_CONNECTOR_GC_DECRYPTION_EXPIRY");
            env::remove_var("KMS_CONNECTOR_GC_DECRYPTION_UNDER_PROCESS_LIMIT");
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
                "KMS_CONNECTOR_PRIVATE_KEY",
                "8355bb293b8714a06b972bfe692d1bd9f24235c1f4007ae0be285d398b0bba2f",
            );
            env::set_var(
                "KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS",
                "0x0000000000000000000000000000000000000002",
            );
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
            env::set_var("KMS_CONNECTOR_RESPONSES_BATCH_SIZE", "20");
            env::set_var("KMS_CONNECTOR_TX_RETRIES", "5");
            env::set_var("KMS_CONNECTOR_TX_RETRY_INTERVAL", "200ms");
            env::set_var("KMS_CONNECTOR_TRACE_REVERTED_TX", "false");
            env::set_var("KMS_CONNECTOR_GAS_MULTIPLIER_PERCENT", "180");
            env::set_var("KMS_CONNECTOR_GAUGE_UPDATE_INTERVAL", "20s");
            env::set_var("KMS_CONNECTOR_GC_RUN_INTERVAL", "2m");
            env::set_var("KMS_CONNECTOR_GC_DECRYPTION_EXPIRY", "50m");
            env::set_var("KMS_CONNECTOR_GC_DECRYPTION_UNDER_PROCESS_LIMIT", "1m");
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
        assert_eq!(config.responses_batch_size, 20);
        assert_eq!(config.tx_retries, 5);
        assert_eq!(config.tx_retry_interval, Duration::from_millis(200));
        assert!(!config.trace_reverted_tx);
        assert_eq!(config.gas_multiplier_percent, 180);
        assert_eq!(config.gauge_update_interval, Duration::from_secs(20));
        assert_eq!(config.gc_run_interval, Duration::from_mins(2));
        assert_eq!(
            config.gc_decryption_expiry,
            PgInterval::try_from(Duration::from_mins(50)).unwrap()
        );
        assert_eq!(
            config.gc_decryption_under_process_limit,
            PgInterval::try_from(Duration::from_mins(1)).unwrap()
        );

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
        format!("{}/../../config/tx-sender.toml", env!("CARGO_MANIFEST_DIR"))
    }
}
