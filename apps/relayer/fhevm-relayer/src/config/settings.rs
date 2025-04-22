use alloy::primitives::map::HashMap;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;

/// Network configuration for blockchain connections.
///
/// This struct holds connection details for both L1 and L2 networks.
#[derive(Debug, Deserialize, Clone)]
pub struct NetworkConfig {
    /// WebSocket endpoint URL
    pub ws_url: String,
    /// HTTP endpoint URL
    pub http_url: String,
    /// Network chain ID
    pub chain_id: u64,
    /// Delay between retry attempts
    pub retry_delay: u64,
    /// Maximum number of reconnection attempts
    pub max_reconnection_attempts: u32,
}

impl NetworkConfig {
    pub fn validate(&self) -> Result<(), AppConfigError> {
        // Validate URLs
        if !self.ws_url.starts_with("ws://") && !self.ws_url.starts_with("wss://") {
            return Err(AppConfigError::InvalidNetworkConfig(format!(
                "Invalid WebSocket URL: {}",
                self.ws_url
            )));
        }
        if !self.http_url.starts_with("http://") && !self.http_url.starts_with("https://") {
            return Err(AppConfigError::InvalidNetworkConfig(format!(
                "Invalid HTTP URL: {}",
                self.http_url
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct NetworksConfig {
    pub fhevm: NetworkConfig,
    pub rollup: Option<NetworkConfig>,
}

impl NetworksConfig {
    pub fn validate(&self) -> Result<(), AppConfigError> {
        self.fhevm.validate()?;
        if let Some(rollup) = &self.rollup {
            rollup.validate()?;
        }
        Ok(())
    }

    pub fn get_network(&self, network_name: &str) -> Result<&NetworkConfig, AppConfigError> {
        match network_name {
            "fhevm" => Ok(&self.fhevm),
            "rollup" => self.rollup.as_ref().ok_or_else(|| {
                AppConfigError::InvalidNetworkConfig("Rollup network not configured".into())
            }),
            _ => Err(AppConfigError::InvalidNetworkConfig(format!(
                "Unknown network: {}",
                network_name
            ))),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TransactionConfig {
    /// Environment variable name containing the private key for httpz
    pub private_key_httpz_env: String,
    /// Environment variable name containing the private key for rollup
    pub private_key_gateway_env: String,
    /// Optional gas limit for transactions
    pub gas_limit: Option<u64>,
    /// Maximum priority fee for transactions
    pub max_priority_fee: Option<String>,
    /// Transaction timeout in seconds
    pub timeout_secs: Option<u64>,
    /// Required number of confirmations
    pub confirmations: Option<u64>,
    /// Retry configuration
    #[serde(default)]
    pub retry: RetrySettings,
    pub ciphertext_check_retry: RetrySettings,
}

impl TransactionConfig {
    pub fn get_max_priority_fee(&self) -> Result<Option<u128>, AppConfigError> {
        match &self.max_priority_fee {
            Some(fee_str) => fee_str
                .parse::<u128>()
                .map(Some)
                .map_err(|e| AppConfigError::Config(config::ConfigError::Foreign(Box::new(e)))),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RetrySettings {
    #[serde(default = "default_max_attempts")]
    pub max_attempts: u32,
    #[serde(default = "default_base_delay")]
    pub base_delay_secs: u64,
    #[serde(default = "default_max_delay")]
    pub max_delay_secs: u64,
}

fn default_max_attempts() -> u32 {
    3
}
fn default_base_delay() -> u64 {
    2
}
fn default_max_delay() -> u64 {
    60
}

impl Default for RetrySettings {
    fn default() -> Self {
        Self {
            max_attempts: default_max_attempts(),
            base_delay_secs: default_base_delay(),
            max_delay_secs: default_max_delay(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractConfig {
    pub decryption_oracle_address: String,
    pub tfhe_executor_address: String,
    pub decryption_manager_address: String,
    pub zkpok_manager_address: String,
    pub ciphertext_manager_address: String,
}

#[derive(Debug, Deserialize)]
pub struct InputProof {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyUrl {
    pub fhe_public_key: KeyData,
    pub crs: KeyData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyData {
    pub data_id: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
/// Top-level configuration structure.
///
/// Contains all configuration settings for the relayer service.
pub struct Settings {
    /// Current environment (development, production, etc.)
    pub environment: String,
    /// Network configurations
    pub networks: NetworksConfig,
    /// Transaction-related settings
    pub transaction: TransactionConfig,
    /// Contract addresses
    pub contracts: ContractConfig,
    /// Logging configuration
    pub log: LogConfig,
    /// Input proof endpoint address
    pub inputproof: InputProof,

    /// Hard-coded data (from config for keyurl)
    pub keyurl: KeyUrl,
}

// Error type for application-specific configuration errors
#[derive(thiserror::Error, Debug)]
pub enum AppConfigError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Invalid contract address: {0}")]
    InvalidAddress(String),

    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid network configuration: {0}")]
    InvalidNetworkConfig(String),
}

impl Settings {
    pub fn new() -> Result<Self, AppConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        // First get base config from files
        let s = Config::builder()
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            // Change how we specify environment variables
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__") // Use double underscore
                    .prefix_separator("_"), // Separator between APP and the rest
            )
            .build()?;

        let settings: Settings = s.try_deserialize()?;

        // Validate network configurations
        settings.networks.validate()?;

        // Log the network configurations for debugging
        tracing::info!(
            fhevm_ws = %settings.networks.fhevm.ws_url,
            fhevm_chain_id = %settings.networks.fhevm.chain_id,
            rollup_configured = settings.networks.rollup.is_some(),
            "Loaded network configurations"
        );

        Ok(settings)
    }

    pub fn validate_addresses(&self) -> Result<(), AppConfigError> {
        // Create a vector of (name, address) pairs to validate
        let addresses = vec![
            (
                "decryption_oracle",
                &self.contracts.decryption_oracle_address,
            ),
            ("tfhe_executor", &self.contracts.tfhe_executor_address),
        ];

        // Iterate and validate each address
        for (name, address) in addresses {
            if !address.starts_with("0x") || address.len() != 42 {
                return Err(AppConfigError::InvalidAddress(format!(
                    "Invalid {} address: {}",
                    name, address
                )));
            }
        }

        Ok(())
    }

    pub fn get_network(&self, network_name: &str) -> Result<&NetworkConfig, AppConfigError> {
        self.networks.get_network(network_name)
    }
}

// Helper function to get a required environment variable
pub fn get_required_env(key: &str) -> Result<String, AppConfigError> {
    env::var(key).map_err(|_| AppConfigError::MissingEnvVar(key.to_string()))
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    /// Log level: trace, debug, info, warn, or error
    pub level: String,
    /// Log format: compact, pretty, or json
    pub format: String,
    /// Whether to show file and line information
    pub show_file_line: bool,
    /// Whether to show thread IDs
    pub show_thread_ids: bool,
    /// Whether to show timestamps (optional)
    #[serde(default)]
    pub show_timestamp: bool,
    /// Custom filters for specific modules (optional)
    #[serde(default)]
    pub module_filters: Option<HashMap<String, String>>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "compact".to_string(),
            show_file_line: false,
            show_thread_ids: false,
            show_timestamp: true,
            module_filters: None,
        }
    }
}
