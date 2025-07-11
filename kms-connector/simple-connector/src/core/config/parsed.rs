//! Module used to parse kms-connector configuration.
//!
//! The `raw` module is first used to deserialize the configuration.

use super::raw::{RawConfig, S3Config};
use crate::{Error, Result, core::utils::wallet::KmsWallet};
use alloy::primitives::Address;
use chrono::{DateTime, Utc};
use std::{
    fmt::{self, Display},
    path::Path,
    str::FromStr,
    time::Duration,
};
use tracing::{info, warn};

/// Configuration for the KMS connector.
#[derive(Debug, Clone)]
pub struct Config {
    /// The Gateway RPC endpoint.
    pub gateway_url: String,
    /// The KMS Core endpoint.
    pub kms_core_endpoint: String,
    /// The Chain ID of the Gateway.
    pub chain_id: u64,
    /// The `Decryption` contract address.
    pub decryption_address: Address,
    /// The `GatewayConfig` contract address.
    pub gateway_config_address: Address,
    /// The event processing channel size.
    pub channel_size: usize,
    /// The service name used for tracing.
    pub service_name: String,
    /// Timeout for public decryption requests in seconds (default: 300s / 5min)
    pub public_decryption_timeout: Duration,
    /// Timeout for user decryption requests in seconds (default: 300s / 5min)
    pub user_decryption_timeout: Duration,
    /// Retry interval (default: 5s).
    pub retry_interval: Duration,
    /// EIP-712 domain name for `Decryption` contract.
    pub decryption_domain_name: String,
    /// EIP-712 domain version for `Decryption` contract.
    pub decryption_domain_version: String,
    /// EIP-712 domain name for `GatewayConfig` contract.
    pub gateway_config_domain_name: String,
    /// EIP-712 domain version for `GatewayConfig` contract.
    pub gateway_config_domain_version: String,
    /// The wallet used to sign the decryption responses from the kms-core.
    pub wallet: KmsWallet,
    /// S3 configuration for ciphertext storage (optional).
    pub s3_config: Option<S3Config>,
    // TODO: implement to increase security
    /// Whether to verify coprocessors against the `GatewayConfig` contract (optional, defaults to true)
    pub verify_coprocessors: Option<bool>,
    /// Optional scheduled start time - connector will wait until this time to start
    /// If None or time is in the past, connector starts immediately
    pub scheduled_start_time: Option<DateTime<Utc>>,
    /// Delta in milliseconds to add to block time before sending messages
    /// This ensures coordinated sending across multiple connectors
    pub message_send_delta: Duration,
    /// Optional starting block number for parsing (if not provided, starts from latest)
    pub starting_block_number: Option<u64>,
    /// Enable coordinated message sending based on block time + delta
    pub enable_coordinated_sending: bool,
    /// Fixed interval (in milliseconds) for sending messages to Core (0 = use block-time-based scheduling)
    pub fixed_send_interval_ms: u64,
    /// Spacing in milliseconds between individual messages from the same block
    pub message_spacing_ms: u64,
    /// Maximum number of pending events before pausing event intake
    pub max_pending_events: usize,
    /// Use polling mode instead of WebSocket for event intake
    pub use_polling_mode: bool,
    /// Base polling interval when caught up to latest block
    pub base_poll_interval_secs: u64,
    /// Fast polling interval when catching up on historical blocks
    pub catch_up_poll_interval_ms: u64,
    /// Maximum number of blocks to process in a single batch
    pub max_blocks_per_batch: u64,
    /// How far behind latest block to consider "caught up"
    pub catch_up_threshold: u64,
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Service Name: {}", self.service_name)?;
        writeln!(f, "KMS Core Endpoint: {}", self.kms_core_endpoint)?;
        writeln!(f, "Gateway URL: {}", self.gateway_url)?;
        writeln!(f, "Chain ID: {}", self.chain_id)?;
        writeln!(f, "Wallet address: {:#x}", self.wallet.address())?;
        writeln!(
            f,
            "Decryption contract address: {}",
            self.decryption_address
        )?;
        writeln!(f, "GatewayConfig Address: {}", self.gateway_config_address)?;
        writeln!(f, "Decryption Domain Name: {}", self.decryption_domain_name)?;
        writeln!(
            f,
            "Decryption Domain Version: {}",
            self.decryption_domain_version
        )?;
        writeln!(
            f,
            "GatewayConfig Domain Name: {}",
            self.gateway_config_domain_name
        )?;
        writeln!(
            f,
            "GatewayConfig Domain Version: {}",
            self.gateway_config_domain_version
        )?;
        writeln!(f, "Channel Size: {}", self.channel_size)?;
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
        write!(f, "Retry Interval: {}s", self.retry_interval.as_secs())?;
        if let Some(scheduled_time) = self.scheduled_start_time {
            write!(
                f,
                "Scheduled Start Time: {}",
                scheduled_time.format("%Y-%m-%d %H:%M:%S UTC")
            )?;
        } else {
            write!(
                f,
                "Scheduled Start Time: Not configured (start immediately)"
            )?;
        }

        Ok(())
    }
}

impl Config {
    /// Validate timing configuration parameters for consistency
    pub fn validate_timing_config(&self) -> Result<()> {
        // Validate message spacing is reasonable
        if self.message_spacing_ms > 60000 {
            // Max 1 minute spacing
            return Err(Error::Config(
                "message_spacing_ms cannot exceed 60000ms (1 minute)".to_string(),
            ));
        }

        // Validate delta is reasonable
        if self.message_send_delta > std::time::Duration::from_millis(300000) {
            // Max 5 minutes delta
            return Err(Error::Config(
                "message_send_delta cannot exceed 300000ms (5 minutes)".to_string(),
            ));
        }

        // Warn about conflicting configurations
        if self.enable_coordinated_sending && self.fixed_send_interval_ms > 0 {
            tracing::warn!(
                "Both coordinated_sending and fixed_send_interval are enabled. \
                 Coordinated sending will take precedence."
            );
        }

        // Validate backpressure settings
        if self.max_pending_events == 0 {
            return Err(Error::Config(
                "max_pending_events must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Loads the configuration from environment variables and optionally from a TOML file.
    ///
    /// Environment variables take precedence over file configuration.
    /// Environment variables are prefixed with KMS_CONNECTOR_.
    pub async fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> Result<Self> {
        let raw_config = RawConfig::from_env_and_file(path)?;
        let config = Self::parse(raw_config).await?;

        info!("Configuration loaded successfully:\n{}", config);
        Ok(config)
    }

    async fn parse(mut raw_config: RawConfig) -> Result<Self> {
        let wallet = Self::parse_kms_wallet(&mut raw_config).await?;

        let decryption_address = Self::parse_decryption_address(&raw_config.decryption_address)?;
        let decryption_domain_name =
            Self::parse_decryption_domain_name(raw_config.decryption_domain_name);
        let decryption_domain_version =
            Self::parse_decryption_domain_version(raw_config.decryption_domain_version);

        let gateway_config_address =
            Self::parse_gateway_config_address(&raw_config.gateway_config_address)?;
        let gateway_config_domain_name =
            Self::parse_gateway_config_domain_name(raw_config.gateway_config_domain_name);
        let gateway_config_domain_version =
            Self::parse_gateway_config_domain_version(raw_config.gateway_config_domain_version);

        // Validate critical configuration parts
        if raw_config.gateway_url.is_empty() {
            return Err(Error::Config("Gateway URL is not configured".to_string()));
        }

        if raw_config.kms_core_endpoint.is_empty() {
            return Err(Error::Config(
                "KMS Core endpoint is not configured".to_string(),
            ));
        }

        if raw_config.decryption_address.is_empty() {
            return Err(Error::Config(
                "Decryption address is not configured".to_string(),
            ));
        }

        if raw_config.gateway_config_address.is_empty() {
            return Err(Error::Config(
                "GatewayConfig address is not configured".to_string(),
            ));
        }

        // Check S3 configuration - warn but don't fail if missing
        if raw_config.s3_config.is_none() {
            warn!("Optional S3 configuration is not provided. Some functionality may be limited.");
        }

        let public_decryption_timeout =
            Duration::from_secs(raw_config.public_decryption_timeout_secs);
        let user_decryption_timeout = Duration::from_secs(raw_config.user_decryption_timeout_secs);
        let retry_interval = Duration::from_secs(raw_config.retry_interval_secs);

        // Parse scheduled start time if provided
        let scheduled_start_time = if let Some(time_str) = raw_config.scheduled_start_time {
            match DateTime::parse_from_rfc3339(&time_str) {
                Ok(dt) => Some(dt.with_timezone(&Utc)),
                Err(e) => {
                    return Err(Error::Config(format!(
                        "Invalid scheduled_start_time format '{time_str}': {e}. Expected RFC3339 format (e.g., '2024-01-15T10:30:00Z')"
                    )));
                }
            }
        } else {
            None
        };

        let config = Config {
            gateway_url: raw_config.gateway_url,
            kms_core_endpoint: raw_config.kms_core_endpoint,
            chain_id: raw_config.chain_id,
            decryption_address,
            gateway_config_address,
            channel_size: raw_config.channel_size,
            service_name: raw_config.service_name,
            public_decryption_timeout,
            user_decryption_timeout,
            retry_interval,
            decryption_domain_name,
            decryption_domain_version,
            gateway_config_domain_name,
            gateway_config_domain_version,
            wallet,
            s3_config: raw_config.s3_config,
            verify_coprocessors: raw_config.verify_coprocessors,
            scheduled_start_time,
            message_send_delta: Duration::from_millis(raw_config.message_send_delta_ms),
            starting_block_number: raw_config.starting_block_number,
            enable_coordinated_sending: raw_config.enable_coordinated_sending,
            fixed_send_interval_ms: raw_config.fixed_send_interval_ms,
            message_spacing_ms: raw_config.message_spacing_ms,
            max_pending_events: raw_config.max_pending_events,
            use_polling_mode: raw_config.use_polling_mode,
            base_poll_interval_secs: raw_config.base_poll_interval_secs,
            catch_up_poll_interval_ms: raw_config.catch_up_poll_interval_ms,
            max_blocks_per_batch: raw_config.max_blocks_per_batch,
            catch_up_threshold: raw_config.catch_up_threshold,
        };

        Ok(config)
    }

    async fn parse_kms_wallet(raw_config: &mut RawConfig) -> Result<KmsWallet> {
        let chain_id = Some(raw_config.chain_id);
        if let Some(private_key) = raw_config.private_key.take() {
            KmsWallet::from_private_key_str(&private_key, chain_id).map_err(Error::from)
        } else if let Some(aws_kms_config) = raw_config.aws_kms_config.take() {
            KmsWallet::from_aws_kms(
                aws_kms_config.key_id,
                aws_kms_config.region,
                aws_kms_config.endpoint,
                chain_id,
            )
            .await
            .map_err(Error::from)
        } else {
            Err(Error::Config(
                "Either AWS KMS or private key must be configured".to_string(),
            ))
        }
    }

    fn parse_decryption_address(address: &str) -> Result<Address> {
        if !address.starts_with("0x") {
            return Err(Error::Config(
                "Decryption address must start with 0x".into(),
            ));
        }
        Address::from_str(address)
            .map_err(|e| Error::Config(format!("Invalid Decryption address: {e}")))
    }

    fn parse_gateway_config_address(address: &str) -> Result<Address> {
        if !address.starts_with("0x") {
            return Err(Error::Config(
                "GatewayConfig address must start with 0x".into(),
            ));
        }
        Address::from_str(address)
            .map_err(|e| Error::Config(format!("Invalid GatewayConfig address: {e}")))
    }

    fn parse_gateway_config_domain_version(raw_domain_version: Option<String>) -> String {
        raw_domain_version.unwrap_or_else(|| {
            warn!(
                "GatewayConfig domain version is empty, will use default '{}'",
                default_gateway_config_domain_version()
            );
            default_gateway_config_domain_version()
        })
    }

    fn parse_gateway_config_domain_name(raw_domain_name: Option<String>) -> String {
        let gateway_config_domain_name = raw_domain_name.unwrap_or_else(|| {
            warn!(
                "GatewayConfig domain name is empty, will use default '{}'",
                default_gateway_config_domain_name()
            );
            default_gateway_config_domain_name()
        });

        // Check for characters that might cause issues in EIP-712 domain messages
        if gateway_config_domain_name.chars().any(|c| c.is_control()) {
            warn!(
                "  GatewayConfig Domain Name contains control characters, may cause EIP-712 encoding issues"
            );
        } else if !gateway_config_domain_name.is_ascii() {
            warn!(
                "  GatewayConfig Domain Name contains non-ASCII characters, may cause EIP-712 compatibility issues"
            );
        } else {
            info!("  GatewayConfig Domain Name EIP-712 compatibility check: OK");
        }

        gateway_config_domain_name
    }

    fn parse_decryption_domain_name(raw_domain_name: Option<String>) -> String {
        let decryption_domain_name = raw_domain_name.unwrap_or_else(|| {
            warn!(
                "Decryption domain name is empty, will use default '{}'",
                default_decryption_domain_name()
            );
            default_decryption_domain_name()
        });

        // Check for characters that might cause issues in EIP-712 domain messages
        if decryption_domain_name.chars().any(|c| c.is_control()) {
            warn!(
                "  Decryption Domain Name contains control characters, may cause EIP-712 encoding issues"
            );
        } else if !decryption_domain_name.is_ascii() {
            warn!(
                "  Decryption Domain Name contains non-ASCII characters, may cause EIP-712 compatibility issues"
            );
        } else {
            info!("  Decryption Domain Name EIP-712 compatibility check: OK");
        }

        decryption_domain_name
    }

    fn parse_decryption_domain_version(raw_domain_version: Option<String>) -> String {
        raw_domain_version.unwrap_or_else(|| {
            warn!(
                "Decryption domain version is empty, will use default '{}'",
                default_decryption_domain_version()
            );
            default_decryption_domain_version()
        })
    }
}

pub fn default_decryption_domain_name() -> String {
    "Decryption".to_string()
}

pub fn default_decryption_domain_version() -> String {
    "1".to_string()
}

pub fn default_gateway_config_domain_name() -> String {
    "GatewayConfig".to_string()
}

pub fn default_gateway_config_domain_version() -> String {
    "1".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::{env, fs};
    use tempfile::NamedTempFile;

    fn cleanup_env_vars() {
        unsafe {
            env::remove_var("KMS_CONNECTOR_GATEWAY_URL");
            env::remove_var("KMS_CONNECTOR_KMS_CORE_ENDPOINT");
            env::remove_var("KMS_CONNECTOR_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_PRIVATE_KEY");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_ADDRESS");
            env::remove_var("KMS_CONNECTOR_GATEWAY_CONFIG_ADDRESS");
            env::remove_var("KMS_CONNECTOR_CHANNEL_SIZE");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
            env::remove_var("KMS_CONNECTOR_PUBLIC_DECRYPTION_TIMEOUT_SECS");
            env::remove_var("KMS_CONNECTOR_USER_DECRYPTION_TIMEOUT_SECS");
            env::remove_var("KMS_CONNECTOR_RETRY_INTERVAL_SECS");
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
        assert_eq!(raw_config.kms_core_endpoint, config.kms_core_endpoint);
        assert_eq!(raw_config.chain_id, config.chain_id);
        assert_eq!(
            Address::from_str(&raw_config.decryption_address).unwrap(),
            config.decryption_address
        );
        assert_eq!(
            Address::from_str(&raw_config.gateway_config_address).unwrap(),
            config.gateway_config_address
        );
        assert_eq!(raw_config.channel_size, config.channel_size);
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
            raw_config.retry_interval_secs,
            config.retry_interval.as_secs()
        );
        assert_eq!(
            raw_config.decryption_domain_name.unwrap(),
            config.decryption_domain_name
        );
        assert_eq!(
            raw_config.decryption_domain_version.unwrap(),
            config.decryption_domain_version
        );
        assert_eq!(
            raw_config.gateway_config_domain_name.unwrap(),
            config.gateway_config_domain_name
        );
        assert_eq!(
            raw_config.gateway_config_domain_version.unwrap(),
            config.gateway_config_domain_version
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
            env::set_var("KMS_CONNECTOR_GATEWAY_URL", "ws://localhost:9545");
            env::set_var("KMS_CONNECTOR_KMS_CORE_ENDPOINT", "http://localhost:50053");
            env::set_var("KMS_CONNECTOR_CHAIN_ID", "31888");
            env::set_var(
                "KMS_CONNECTOR_PRIVATE_KEY",
                "8355bb293b8714a06b972bfe692d1bd9f24235c1f4007ae0be285d398b0bba2f",
            );
            env::set_var(
                "KMS_CONNECTOR_DECRYPTION_ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_GATEWAY_CONFIG_ADDRESS",
                "0x0000000000000000000000000000000000000001",
            );
            env::set_var("KMS_CONNECTOR_CHANNEL_SIZE", "2000");
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
            env::set_var("KMS_CONNECTOR_PUBLIC_DECRYPTION_TIMEOUT_SECS", "600");
            env::set_var("KMS_CONNECTOR_USER_DECRYPTION_TIMEOUT_SECS", "600");
            env::set_var("KMS_CONNECTOR_RETRY_INTERVAL_SECS", "10");
        }

        // Load config from environment
        let config = Config::from_env_and_file::<&str>(None).await.unwrap();

        // Verify values
        assert_eq!(config.gateway_url, "ws://localhost:9545");
        assert_eq!(config.kms_core_endpoint, "http://localhost:50053");
        assert_eq!(config.chain_id, 31888);
        assert_eq!(
            config.decryption_address,
            Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap()
        );
        assert_eq!(
            config.gateway_config_address,
            Address::from_str("0x0000000000000000000000000000000000000001").unwrap()
        );
        assert_eq!(config.channel_size, 2000);
        assert_eq!(config.service_name, "kms-connector-test");
        assert_eq!(config.public_decryption_timeout.as_secs(), 600);
        assert_eq!(config.user_decryption_timeout.as_secs(), 600);
        assert_eq!(config.retry_interval.as_secs(), 10);

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
            decryption_address: "0x0000".to_string(),
            gateway_config_address: "0x000010".to_string(),
            ..Default::default()
        };
        assert!(matches!(
            Config::parse(raw_config).await,
            Err(Error::Config(_))
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
            Err(Error::Config(_))
        ));
    }

    impl RawConfig {
        pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
            let content = toml::to_string_pretty(self)
                .map_err(|e| Error::Config(format!("Failed to serialize config: {e}")))?;

            fs::write(path, content)
                .map_err(|e| Error::Config(format!("Failed to write config file: {e}")))?;

            Ok(())
        }
    }

    impl Default for RawConfig {
        fn default() -> Self {
            Self {
                gateway_url: "ws://localhost:8545".to_string(),
                kms_core_endpoint: "http://localhost:50052".to_string(),
                chain_id: 1,
                decryption_address: "0x0000000000000000000000000000000000000000".to_string(),
                gateway_config_address: "0x0000000000000000000000000000000000000000".to_string(),
                channel_size: 100,
                service_name: "kms-connector".to_string(),
                public_decryption_timeout_secs: 300,
                user_decryption_timeout_secs: 300,
                retry_interval_secs: 5,
                decryption_domain_name: Some("Decryption".to_string()),
                decryption_domain_version: Some("1".to_string()),
                gateway_config_domain_name: Some("GatewayConfig".to_string()),
                gateway_config_domain_version: Some("1".to_string()),
                private_key: Some(
                    "8355bb293b8714a06b972bfe692d1bd9f24235c1f4007ae0be285d398b0bba2f".to_string(),
                ),
                s3_config: None,
                aws_kms_config: None,
                verify_coprocessors: Some(true),
                scheduled_start_time: None,
                message_send_delta_ms: 1000,
                starting_block_number: None,
                enable_coordinated_sending: true,
                fixed_send_interval_ms: 0,
                message_spacing_ms: 100,
                max_pending_events: 10000,
                use_polling_mode: true,
                base_poll_interval_secs: 2,
                catch_up_poll_interval_ms: 100,
                max_blocks_per_batch: 10,
                catch_up_threshold: 5,
            }
        }
    }
}
