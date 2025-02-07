use alloy::primitives::map::HashMap;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub ws_url: String,
    pub http_url: String,
    pub chain_id: u64,
    pub retry_delay: u64,
    pub max_reconnection_attempts: u32,
}

#[derive(Debug, Deserialize)]
pub struct TransactionConfig {
    pub private_key_env: String,
    pub gas_limit: Option<u64>,
    pub max_priority_fee: Option<String>,
    pub timeout_secs: Option<u64>,
    pub confirmations: Option<u64>,
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

#[derive(Debug, Deserialize)]
pub struct ContractConfig {
    pub decryption_oracle_address: String,
    pub tfhe_executor_address: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub environment: String,
    pub network: NetworkConfig,
    pub transaction: TransactionConfig,
    pub contracts: ContractConfig,
    pub log: LogConfig,
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
        println!("Final WS URL: {}", settings.network.ws_url);

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
