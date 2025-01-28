use alloy::primitives::map::HashMap;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct ContractConfig {
    pub decryption_oracle_address: String,
    pub tfhe_executor_address: String,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub ws_url: String,
    pub retry_delay: u64,
    pub max_reconnection_attempts: u32,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub environment: String,
    pub network: NetworkConfig,
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

        let s = Config::builder()
            // Add environment-specific settings
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add local settings (ignored by git)
            .add_source(File::with_name("config/local").required(false))
            // Add environment variables with prefix "APP"
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        s.try_deserialize().map_err(AppConfigError::Config)
    }

    // Helper method to validate all contract addresses
    pub fn validate_addresses(&self) -> Result<(), AppConfigError> {
        validate_ethereum_address(&self.contracts.decryption_oracle_address)?;
        validate_ethereum_address(&self.contracts.tfhe_executor_address)?;
        Ok(())
    }
}

// Helper function to validate Ethereum addresses
pub fn validate_ethereum_address(address: &str) -> Result<(), AppConfigError> {
    if !address.starts_with("0x") || address.len() != 42 {
        return Err(AppConfigError::InvalidAddress(address.to_string()));
    }
    // Could add additional hex validation here
    Ok(())
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
