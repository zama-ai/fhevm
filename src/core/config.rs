use crate::error::{Error, Result};
use alloy::primitives::Address;
use bip39::Mnemonic;
use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, str::FromStr, time::Duration};
use tracing::{info, warn};

/// Configuration for S3 ciphertext storage
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct S3Config {
    /// AWS S3 region for ciphertext storage
    pub region: String,
    /// AWS S3 bucket for ciphertext storage
    pub bucket: String,
    /// AWS S3 endpoint URL for ciphertext storage
    pub endpoint: Option<String>,
}

/// Configuration for AWS KMS signer
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AwsKmsConfig {
    /// AWS KMS key ID for signing
    pub key_id: String,
    /// AWS region for KMS
    pub region: Option<String>,
    /// AWS endpoint URL for KMS
    pub endpoint: Option<String>,
}

/// Configuration for the KMS connector
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Gateway RPC endpoint
    pub gateway_url: String,
    /// KMS Core endpoint
    pub kms_core_endpoint: String,
    /// Mnemonic phrase for the wallet
    pub mnemonic: String,
    /// Chain ID
    pub chain_id: u64,
    /// Decryption contract address
    pub decryption_address: String,
    /// GatewayConfig contract address
    pub gateway_config_address: String,
    /// Channel size for event processing
    pub channel_size: Option<usize>,
    /// Service name for tracing
    #[serde(default = "default_service_name")]
    pub service_name: String,
    /// Timeout for public decryption requests in seconds (default: 300s / 5min)
    #[serde(default = "default_public_decryption_timeout")]
    pub public_decryption_timeout_secs: u64,
    /// Timeout for user decryption requests in seconds (default: 300s / 5min)
    #[serde(default = "default_user_decryption_timeout")]
    pub user_decryption_timeout_secs: u64,
    /// Retry interval in seconds (default: 5s)
    #[serde(default = "default_retry_interval")]
    pub retry_interval_secs: u64,
    /// Account index for wallet derivation (optional)
    pub account_index: Option<u32>,
    /// EIP-712 domain name for Decryption contract
    #[serde(default = "default_decryption_domain_name")]
    pub decryption_domain_name: String,
    /// EIP-712 domain version for Decryption contract
    #[serde(default = "default_decryption_domain_version")]
    pub decryption_domain_version: String,
    /// EIP-712 domain name for GatewayConfig contract
    #[serde(default = "default_gateway_config_domain_name")]
    pub gateway_config_domain_name: String,
    /// EIP-712 domain version for GatewayConfig contract
    #[serde(default = "default_gateway_config_domain_version")]
    pub gateway_config_domain_version: String,
    /// Path to the signing key file (optional)
    #[serde(default = "default_signing_key_path")]
    pub signing_key_path: Option<String>,
    /// Private key as a hex string (optional)
    #[serde(default)]
    pub private_key: Option<String>,
    /// S3 configuration for ciphertext storage (optional)
    #[serde(default)]
    pub s3_config: Option<S3Config>,
    /// AWS KMS configuration for signer (optional)
    #[serde(default)]
    pub aws_kms_config: Option<AwsKmsConfig>,
    // TODO: implement to increase security
    /// Whether to verify coprocessors against the GatewayConfig contract (optional, defaults to true)
    #[serde(default = "default_verify_coprocessors")]
    pub verify_coprocessors: Option<bool>,
}

fn default_service_name() -> String {
    "kms-connector".to_string()
}

fn default_public_decryption_timeout() -> u64 {
    300 // 5 minutes
}

fn default_user_decryption_timeout() -> u64 {
    300 // 5 minutes
}

fn default_retry_interval() -> u64 {
    5 // 5 seconds
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

fn default_signing_key_path() -> Option<String> {
    None
}

fn default_verify_coprocessors() -> Option<bool> {
    Some(false)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gateway_url: "ws://localhost:8545".to_string(),
            kms_core_endpoint: "http://[::1]:50052".to_string(),
            // Use a valid mnemonic with correct checksum
            mnemonic: "test test test test test test test test test test test junk".to_string(),
            chain_id: 31337,
            decryption_address: "0x5fbdb2315678afecb367f032d93f642f64180aa3".to_string(),
            gateway_config_address: "0x0000000000000000000000000000000000000001".to_string(),
            channel_size: Some(1000),
            service_name: default_service_name(),
            public_decryption_timeout_secs: default_public_decryption_timeout(),
            user_decryption_timeout_secs: default_user_decryption_timeout(),
            retry_interval_secs: default_retry_interval(),
            account_index: None,
            decryption_domain_name: default_decryption_domain_name(),
            decryption_domain_version: default_decryption_domain_version(),
            gateway_config_domain_name: default_gateway_config_domain_name(),
            gateway_config_domain_version: default_gateway_config_domain_version(),
            signing_key_path: default_signing_key_path(),
            private_key: None,
            s3_config: None,
            aws_kms_config: None,
            verify_coprocessors: default_verify_coprocessors(),
        }
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Loading configuration from: {}", path.as_ref().display());

        let content = fs::read_to_string(path)
            .map_err(|e| Error::Config(format!("Failed to read config file: {}", e)))?;

        let config: Self = toml::from_str(&content)
            .map_err(|e| Error::Config(format!("Failed to parse config file: {}", e)))?;

        // Validate and log configuration
        info!("Configuration loaded successfully:");
        info!("  Service Name: {}", config.service_name);
        info!("  KMS Core Endpoint: {}", config.kms_core_endpoint);
        info!("  Gateway URL: {}", config.gateway_url);
        info!("  Chain ID: {}", config.chain_id);
        info!(
            "  Decryption contract address: {}",
            config.decryption_address
        );
        info!("  GatewayConfig Address: {}", config.gateway_config_address);
        if let Some(size) = config.channel_size {
            info!("  Channel Size: {}", size);
        } else {
            warn!("  Channel Size: not specified, using default");
        }
        info!(
            "  Public Decryption Timeout: {}s",
            config.public_decryption_timeout_secs
        );
        info!(
            "  User Decryption Timeout: {}s",
            config.user_decryption_timeout_secs
        );
        info!("  Retry Interval: {}s", config.retry_interval_secs);

        // Log domain configuration
        info!(
            "  Decryption Domain Name: {}",
            config.decryption_domain_name
        );
        info!(
            "  Decryption Domain Version: {}",
            config.decryption_domain_version
        );
        info!(
            "  GatewayConfig Domain Name: {}",
            config.gateway_config_domain_name
        );
        info!(
            "  GatewayConfig Domain Version: {}",
            config.gateway_config_domain_version
        );

        // Validate and log UTF-8 status of domain names
        if config.decryption_domain_name.is_empty() {
            warn!("  Decryption Domain Name is empty, will use default at runtime");
        } else {
            // Check for characters that might cause issues in EIP-712 domain messages
            let has_control_chars = config
                .decryption_domain_name
                .chars()
                .any(|c| c.is_control());
            let has_non_ascii = !config.decryption_domain_name.is_ascii();

            if has_control_chars {
                warn!(
                    "  Decryption Domain Name contains control characters, may cause EIP-712 encoding issues"
                );
            } else if has_non_ascii {
                warn!(
                    "  Decryption Domain Name contains non-ASCII characters, may cause EIP-712 compatibility issues"
                );
            } else {
                info!("  Decryption Domain Name EIP-712 compatibility check: OK");
            }
        }

        if config.gateway_config_domain_name.is_empty() {
            warn!("  GatewayConfig Domain Name is empty, will use default at runtime");
        } else {
            // Check for characters that might cause issues in EIP-712 domain messages
            let has_control_chars = config
                .gateway_config_domain_name
                .chars()
                .any(|c| c.is_control());
            let has_non_ascii = !config.gateway_config_domain_name.is_ascii();

            if has_control_chars {
                warn!(
                    "  GatewayConfig Domain Name contains control characters, may cause EIP-712 encoding issues"
                );
            } else if has_non_ascii {
                warn!(
                    "  GatewayConfig Domain Name contains non-ASCII characters, may cause EIP-712 compatibility issues"
                );
            } else {
                info!("  GatewayConfig Domain Name EIP-712 compatibility check: OK");
            }
        }

        // Validate mnemonic
        Mnemonic::parse_normalized(&config.mnemonic)
            .map_err(|e| Error::Config(format!("Invalid mnemonic: {}", e)))?;
        info!("  Mnemonic: validated successfully");

        // Validate addresses
        if !config.decryption_address.starts_with("0x") {
            return Err(Error::Config(
                "Decryption address must start with 0x".into(),
            ));
        }
        Address::from_str(&config.decryption_address)
            .map_err(|e| Error::Config(format!("Invalid Decryption address: {}", e)))?;

        if !config.gateway_config_address.starts_with("0x") {
            return Err(Error::Config(
                "GatewayConfig address must start with 0x".into(),
            ));
        }
        Address::from_str(&config.gateway_config_address)
            .map_err(|e| Error::Config(format!("Invalid GatewayConfig address: {}", e)))?;

        Ok(config)
    }

    /// Load configuration from environment variables and optionally from a TOML file
    /// Environment variables take precedence over file configuration
    /// Environment variables are prefixed with KMS_CONNECTOR_
    pub fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> Result<Self> {
        let mut builder = ConfigBuilder::builder();

        // Start with default values
        let default_config = Self::default();
        builder = builder.add_source(config::File::from_str(
            &toml::to_string(&default_config)
                .map_err(|e| Error::Config(format!("Failed to serialize default config: {}", e)))?,
            config::FileFormat::Toml,
        ));

        // If path is provided, add it as a config source
        if let Some(path) = path {
            info!(
                "Loading configuration from file: {}",
                path.as_ref().display()
            );
            builder = builder.add_source(
                File::with_name(path.as_ref().to_str().unwrap()).format(FileFormat::Toml),
            );
        }

        // Add environment variables last so they take precedence
        info!("Adding environment variables with prefix KMS_CONNECTOR_");
        builder = builder.add_source(Environment::with_prefix("KMS_CONNECTOR"));

        let settings = builder
            .build()
            .map_err(|e| Error::Config(format!("Failed to build config: {}", e)))?;

        let config: Self = settings
            .try_deserialize()
            .map_err(|e| Error::Config(format!("Failed to deserialize config: {}", e)))?;

        // Log configuration
        info!("Configuration loaded successfully:");
        info!("  Service Name: {}", config.service_name);
        info!("  Gateway URL: {}", config.gateway_url);
        info!("  KMS Core Endpoint: {}", config.kms_core_endpoint);
        info!("  Chain ID: {}", config.chain_id);
        info!("  Decryption Address: {}", config.decryption_address);
        info!("  GatewayConfig Address: {}", config.gateway_config_address);
        if let Some(size) = config.channel_size {
            info!("  Channel Size: {}", size);
        } else {
            info!("  Channel Size: default");
        }
        info!(
            "  Public Decryption Timeout: {}s",
            config.public_decryption_timeout_secs
        );
        info!(
            "  User Decryption Timeout: {}s",
            config.user_decryption_timeout_secs
        );
        info!("  Retry Interval: {}s", config.retry_interval_secs);

        // Log domain configuration
        info!(
            "  Decryption Domain Name: {}",
            config.decryption_domain_name
        );
        info!(
            "  Decryption Domain Version: {}",
            config.decryption_domain_version
        );
        info!(
            "  GatewayConfig Domain Name: {}",
            config.gateway_config_domain_name
        );
        info!(
            "  GatewayConfig Domain Version: {}",
            config.gateway_config_domain_version
        );

        // Validate and log UTF-8 status of domain names
        if config.decryption_domain_name.is_empty() {
            warn!("  Decryption Domain Name is empty, will use default at runtime");
        } else {
            // Check for characters that might cause issues in EIP-712 domain messages
            let has_control_chars = config
                .decryption_domain_name
                .chars()
                .any(|c| c.is_control());
            let has_non_ascii = !config.decryption_domain_name.is_ascii();

            if has_control_chars {
                warn!(
                    "  Decryption Domain Name contains control characters, may cause EIP-712 encoding issues"
                );
            } else if has_non_ascii {
                warn!(
                    "  Decryption Domain Name contains non-ASCII characters, may cause EIP-712 compatibility issues"
                );
            } else {
                info!("  Decryption Domain Name EIP-712 compatibility check: OK");
            }
        }

        if config.gateway_config_domain_name.is_empty() {
            warn!("  GatewayConfig Domain Name is empty, will use default at runtime");
        } else {
            // Check for characters that might cause issues in EIP-712 domain messages
            let has_control_chars = config
                .gateway_config_domain_name
                .chars()
                .any(|c| c.is_control());
            let has_non_ascii = !config.gateway_config_domain_name.is_ascii();

            if has_control_chars {
                warn!(
                    "  GatewayConfig Domain Name contains control characters, may cause EIP-712 encoding issues"
                );
            } else if has_non_ascii {
                warn!(
                    "  GatewayConfig Domain Name contains non-ASCII characters, may cause EIP-712 compatibility issues"
                );
            } else {
                info!("  GatewayConfig Domain Name EIP-712 compatibility check: OK");
            }
        }

        // Validate the configuration using existing validation logic
        if !config.decryption_address.starts_with("0x") {
            return Err(Error::Config(
                "Decryption address must start with 0x".into(),
            ));
        }
        Address::from_str(&config.decryption_address)
            .map_err(|e| Error::Config(format!("Invalid Decryption address: {}", e)))?;

        if !config.gateway_config_address.starts_with("0x") {
            return Err(Error::Config(
                "GatewayConfig address must start with 0x".into(),
            ));
        }
        Address::from_str(&config.gateway_config_address)
            .map_err(|e| Error::Config(format!("Invalid GatewayConfig address: {}", e)))?;

        // Validate mnemonic
        info!("Validating mnemonic...");
        Mnemonic::parse_normalized(&config.mnemonic)
            .map_err(|e| Error::Config(format!("Invalid mnemonic: {}", e)))?;
        info!("Mnemonic validated successfully");

        // Log wallet configuration
        if config.signing_key_path.is_some() {
            info!("  Wallet: Using signing key file");
        } else if config.private_key.is_some() {
            info!("  Wallet: Using private key string");
        } else {
            info!("  Wallet: Using mnemonic");
        }

        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(path, content)
            .map_err(|e| Error::Config(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }

    /// Get Decryption address as Address type
    pub fn get_decryption_address(&self) -> Result<Address> {
        Address::from_str(&self.decryption_address)
            .map_err(|e| Error::Config(format!("Invalid Decryption address: {}", e)))
    }

    /// Get GatewayConfig address as Address type
    pub fn get_gateway_config_address(&self) -> Result<Address> {
        Address::from_str(&self.gateway_config_address)
            .map_err(|e| Error::Config(format!("Invalid GatewayConfig address: {}", e)))
    }

    /// Get public decryption timeout as Duration
    pub fn public_decryption_timeout(&self) -> Duration {
        Duration::from_secs(self.public_decryption_timeout_secs)
    }

    /// Get user decryption timeout as Duration
    pub fn user_decryption_timeout(&self) -> Duration {
        Duration::from_secs(self.user_decryption_timeout_secs)
    }

    /// Get retry interval as Duration
    pub fn retry_interval(&self) -> Duration {
        Duration::from_secs(self.retry_interval_secs)
    }

    /// Get the account index for wallet derivation
    /// If explicitly set in config, use that value
    /// Otherwise, extract it from the service name (e.g., "kms-connector-2" -> 2)
    pub fn get_account_index(&self) -> u32 {
        if let Some(index) = self.account_index {
            return index;
        }

        // Try to extract index from service name (e.g., "kms-connector-2" -> 2)
        let parts: Vec<&str> = self.service_name.split('-').collect();
        if parts.len() >= 2 {
            // Try to get the last part and parse it as a number
            if let Some(last) = parts.last() {
                if let Ok(index) = last.parse::<u32>() {
                    return index;
                }
            }
        }

        // Default to 0 if service name doesn't contain a number
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use tempfile::NamedTempFile;

    fn get_test_mnemonic() -> String {
        // Use a valid test mnemonic with correct checksum
        "test test test test test test test test test test test junk".to_string()
    }

    fn cleanup_env_vars() {
        unsafe {
            env::remove_var("KMS_CONNECTOR_GATEWAY_URL");
            env::remove_var("KMS_CONNECTOR_KMS_CORE_ENDPOINT");
            env::remove_var("KMS_CONNECTOR_MNEMONIC");
            env::remove_var("KMS_CONNECTOR_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_ADDRESS");
            env::remove_var("KMS_CONNECTOR_GATEWAY_CONFIG_ADDRESS");
            env::remove_var("KMS_CONNECTOR_CHANNEL_SIZE");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
            env::remove_var("KMS_CONNECTOR_PUBLIC_DECRYPTION_TIMEOUT_SECS");
            env::remove_var("KMS_CONNECTOR_USER_DECRYPTION_TIMEOUT_SECS");
            env::remove_var("KMS_CONNECTOR_RETRY_INTERVAL_SECS");
        }
    }

    #[test]
    #[serial]
    fn test_load_valid_config() {
        let config = Config {
            gateway_url: "ws://localhost:8545".to_string(),
            kms_core_endpoint: "http://localhost:50052".to_string(),
            mnemonic: get_test_mnemonic(),
            chain_id: 1,
            decryption_address: "0x0000000000000000000000000000000000000000".to_string(),
            gateway_config_address: "0x0000000000000000000000000000000000000000".to_string(),
            channel_size: Some(100),
            service_name: "kms-connector".to_string(),
            public_decryption_timeout_secs: 300,
            user_decryption_timeout_secs: 300,
            retry_interval_secs: 5,
            account_index: None,
            decryption_domain_name: "IDecryption".to_string(),
            decryption_domain_version: "1".to_string(),
            gateway_config_domain_name: "IGatewayConfig".to_string(),
            gateway_config_domain_version: "1".to_string(),
            signing_key_path: None,
            private_key: None,
            s3_config: None,
            aws_kms_config: None,
            verify_coprocessors: Some(true),
        };

        let temp_file = NamedTempFile::new().unwrap();
        config.to_file(temp_file.path()).unwrap();

        let loaded_config = Config::from_file(temp_file.path()).unwrap();

        // Compare fields
        assert_eq!(config.gateway_url, loaded_config.gateway_url);
        assert_eq!(config.kms_core_endpoint, loaded_config.kms_core_endpoint);
        assert_eq!(config.mnemonic, loaded_config.mnemonic);
        assert_eq!(config.chain_id, loaded_config.chain_id);
        assert_eq!(config.decryption_address, loaded_config.decryption_address);
        assert_eq!(
            config.gateway_config_address,
            loaded_config.gateway_config_address
        );
        assert_eq!(config.channel_size, loaded_config.channel_size);
        assert_eq!(config.kms_core_endpoint, loaded_config.kms_core_endpoint);
        assert_eq!(config.service_name, loaded_config.service_name);
        assert_eq!(
            config.public_decryption_timeout_secs,
            loaded_config.public_decryption_timeout_secs
        );
        assert_eq!(
            config.user_decryption_timeout_secs,
            loaded_config.user_decryption_timeout_secs
        );
        assert_eq!(
            config.retry_interval_secs,
            loaded_config.retry_interval_secs
        );
        assert_eq!(
            config.decryption_domain_name,
            loaded_config.decryption_domain_name
        );
        assert_eq!(
            config.decryption_domain_version,
            loaded_config.decryption_domain_version
        );
        assert_eq!(
            config.gateway_config_domain_name,
            loaded_config.gateway_config_domain_name
        );
        assert_eq!(
            config.gateway_config_domain_version,
            loaded_config.gateway_config_domain_version
        );
        assert_eq!(config.signing_key_path, loaded_config.signing_key_path);
        assert_eq!(config.s3_config, loaded_config.s3_config);
        assert_eq!(
            config.verify_coprocessors,
            loaded_config.verify_coprocessors
        );
    }

    #[test]
    #[serial]
    fn test_save_config() {
        let config = Config {
            gateway_url: "ws://localhost:8545".to_string(),
            kms_core_endpoint: "http://localhost:50052".to_string(),
            mnemonic: get_test_mnemonic(),
            chain_id: 1,
            decryption_address: "0x0000000000000000000000000000000000000000".to_string(),
            gateway_config_address: "0x0000000000000000000000000000000000000000".to_string(),
            channel_size: None,
            service_name: "kms-connector".to_string(),
            public_decryption_timeout_secs: 300,
            user_decryption_timeout_secs: 300,
            retry_interval_secs: 5,
            account_index: None,
            decryption_domain_name: "Decryption".to_string(),
            decryption_domain_version: "1".to_string(),
            gateway_config_domain_name: "GatewayConfig".to_string(),
            gateway_config_domain_version: "1".to_string(),
            signing_key_path: None,
            private_key: None,
            s3_config: None,
            aws_kms_config: None,
            verify_coprocessors: Some(true),
        };

        config.to_file("test_config.toml").unwrap();
        let loaded_config = Config::from_file("test_config.toml").unwrap();
        assert_eq!(config.gateway_url, loaded_config.gateway_url);

        fs::remove_file("test_config.toml").unwrap();
    }

    #[test]
    #[serial]
    fn test_invalid_address() {
        let config = Config {
            gateway_url: "ws://localhost:8545".to_string(),
            kms_core_endpoint: "http://localhost:50052".to_string(),
            mnemonic: get_test_mnemonic(),
            chain_id: 1,
            decryption_address: "0x0000".to_string(),
            gateway_config_address: "0x000010".to_string(),
            channel_size: None,
            service_name: "kms-connector".to_string(),
            public_decryption_timeout_secs: 300,
            user_decryption_timeout_secs: 300,
            retry_interval_secs: 5,
            account_index: None,
            decryption_domain_name: "Decryption".to_string(),
            decryption_domain_version: "1".to_string(),
            gateway_config_domain_name: "GatewayConfig".to_string(),
            gateway_config_domain_version: "1".to_string(),
            signing_key_path: None,
            private_key: None,
            s3_config: None,
            aws_kms_config: None,
            verify_coprocessors: Some(true),
        };

        let temp_file = NamedTempFile::new().unwrap();
        config.to_file(temp_file.path()).unwrap();

        assert!(Config::from_file(temp_file.path()).is_err());
    }

    #[test]
    #[serial]
    fn test_load_from_env() {
        cleanup_env_vars();

        // Set environment variables
        unsafe {
            env::set_var("KMS_CONNECTOR_GATEWAY_URL", "ws://localhost:9545");
            env::set_var("KMS_CONNECTOR_KMS_CORE_ENDPOINT", "http://localhost:50053");
            env::set_var("KMS_CONNECTOR_MNEMONIC", get_test_mnemonic());
            env::set_var("KMS_CONNECTOR_CHAIN_ID", "31888");
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
        let config = Config::from_env_and_file::<&str>(None).unwrap();

        // Verify values
        assert_eq!(config.gateway_url, "ws://localhost:9545");
        assert_eq!(config.kms_core_endpoint, "http://localhost:50053");
        assert_eq!(config.mnemonic, get_test_mnemonic());
        assert_eq!(config.chain_id, 31888);
        assert_eq!(
            config.decryption_address,
            "0x5fbdb2315678afecb367f032d93f642f64180aa3"
        );
        assert_eq!(
            config.gateway_config_address,
            "0x0000000000000000000000000000000000000001"
        );
        assert_eq!(config.channel_size, Some(2000));
        assert_eq!(config.service_name, "kms-connector-test");
        assert_eq!(config.public_decryption_timeout_secs, 600);
        assert_eq!(config.user_decryption_timeout_secs, 600);
        assert_eq!(config.retry_interval_secs, 10);

        cleanup_env_vars();
    }

    #[test]
    #[serial]
    fn test_env_overrides_file() {
        cleanup_env_vars();

        // Create a temp config file
        let config = Config {
            gateway_url: "ws://localhost:8545".to_string(),
            kms_core_endpoint: "http://localhost:50052".to_string(),
            mnemonic: get_test_mnemonic(),
            chain_id: 1,
            decryption_address: "0x0000000000000000000000000000000000000000".to_string(),
            gateway_config_address: "0x0000000000000000000000000000000000000000".to_string(),
            channel_size: Some(100),
            service_name: "kms-connector".to_string(),
            public_decryption_timeout_secs: 300,
            user_decryption_timeout_secs: 300,
            retry_interval_secs: 5,
            account_index: None,
            decryption_domain_name: "Decryption".to_string(),
            decryption_domain_version: "1".to_string(),
            gateway_config_domain_name: "GatewayConfig".to_string(),
            gateway_config_domain_version: "1".to_string(),
            signing_key_path: None,
            private_key: None,
            s3_config: None,
            aws_kms_config: None,
            verify_coprocessors: Some(true),
        };

        let temp_file = NamedTempFile::new().unwrap();
        config.to_file(temp_file.path()).unwrap();

        // Set an environment variable to override the file
        unsafe {
            env::set_var("KMS_CONNECTOR_CHAIN_ID", "77737");
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-override");
        }

        // Load config from both sources
        let loaded_config = Config::from_env_and_file(Some(temp_file.path())).unwrap();

        // Verify that environment variables take precedence
        assert_eq!(loaded_config.chain_id, 77737);
        assert_eq!(loaded_config.service_name, "kms-connector-override");
        // File values should be used for non-overridden fields
        assert_eq!(loaded_config.gateway_url, "ws://localhost:8545");

        cleanup_env_vars();
    }
}
