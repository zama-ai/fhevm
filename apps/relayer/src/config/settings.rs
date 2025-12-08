use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayConfig {
    pub blockchain_rpc: BlockchainRpcConfig,
    pub listener: ListenerConfig,
    pub tx_engine: TxEngineConfig,
    pub readiness_checker: ReadinessCheckConfig,
    pub contracts: ContractConfig,
}

impl GatewayConfig {
    pub fn validate(&self) -> Result<(), AppConfigError> {
        self.blockchain_rpc.validate()?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlockchainRpcConfig {
    pub ws_url: String,
    pub http_url: String,
    pub chain_id: u64,
    pub ws_health_check_timeout_secs: u64,
    pub http_health_check_timeout_secs: u64,
}

impl BlockchainRpcConfig {
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

#[derive(Debug, Deserialize, Clone)]
pub struct ListenerConfig {
    /// Optional starting block number for event subscriptions
    pub last_block_number: Option<u64>,
    /// WebSocket reconnection configuration
    pub ws_reconnect_config: RetrySettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TxEngineConfig {
    pub private_key: String,
    pub max_concurrency: u16,
    pub retry: RetrySettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReadinessCheckConfig {
    pub max_concurrency: u16,
    pub retry: RetrySettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpMetricsConfig {
    pub histogram_buckets: Vec<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    /// Requests per second allowed (token refill rate)
    pub requests_per_second: u32,
    /// Maximum burst size allowed (bucket capacity)
    pub burst_size: u32,
    /// Base retry-after time in seconds for rate limited responses
    pub retry_after_seconds: u64,
    /// Maximum additional jitter in milliseconds (0 = no jitter)
    pub jitter_max_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpConfig {
    /// HTTP endpoint address to bind to (e.g., "0.0.0.0:3000").
    /// Can be None to disable HTTP server (useful for tests or metrics-only mode).
    /// When Some, server will bind to this address and update the field with actual bound address.
    pub endpoint: Option<String>,
    /// Rate limiting configuration for HTTP endpoints
    pub rate_limit_post_endpoints: RateLimitConfig,
    /// HTTP metrics configuration
    pub metrics: HttpMetricsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsConfig {
    /// Endpoint for metrics server (e.g., "0.0.0.0:9898")
    pub endpoint: String,
}

#[derive(Deserialize, Clone)]
pub struct StorageConfig {
    /// PostgreSQL database URL for SQL storage
    pub sql_database_url: String,
    /// Maximum number of connections in the SQL connection pool
    pub sql_max_connections: u32,
    pub sql_health_check_timeout_secs: u64,
}

impl fmt::Debug for StorageConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StorageConfig")
            .field("sql_database_url", &"[REDACTED]")
            .field("sql_max_connections", &self.sql_max_connections)
            .field(
                "sql_health_check_timeout_secs",
                &self.sql_health_check_timeout_secs,
            )
            .finish()
    }
}

impl fmt::Display for StorageConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StorageConfig {{ sql_database_url: [REDACTED], sql_max_connections: {} }}",
            self.sql_max_connections
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RetrySettings {
    pub max_attempts: u32,
    pub retry_interval_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractConfig {
    pub decryption_address: String,
    pub input_verification_address: String,
    /// Number of shares required for user decryption threshold consensus
    pub user_decrypt_shares_threshold: u16,
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

#[derive(Debug, Deserialize, Clone)]
/// Top-level configuration structure.
///
/// Contains all configuration settings for the relayer service.
pub struct Settings {
    /// Network configurations
    pub gateway: GatewayConfig,
    /// Logging configuration
    pub log: LogConfig,
    /// Hard-coded data (from config for keyurl)
    pub keyurl: KeyUrl,
    /// HTTP server configuration
    pub http: HttpConfig,
    /// Metrics server configuration
    pub metrics: MetricsConfig,
    /// Storage configuration
    pub storage: StorageConfig,
}

// Error type for application-specific configuration errors
#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone)]
pub enum AppConfigError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid contract address: {0}")]
    InvalidAddress(String),

    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid network configuration: {0}")]
    InvalidNetworkConfig(String),
}

impl Settings {
    pub fn new(config_file: Option<String>) -> Result<Self, AppConfigError> {
        // First get base config from files
        let s = Config::builder().add_source(File::with_name("config/local").required(false));
        let s = match config_file {
            Some(config_file) => s.add_source(File::with_name(&config_file).required(true)),
            None => s,
        };
        // Change how we specify environment variables
        // Environment variables always override file-based configuration
        let s = s.add_source(
            Environment::with_prefix("APP")
                .separator("__") // Use double underscore
                .prefix_separator("_"), // Separator between APP and the rest
        );

        let settings: Settings = s
            .build()
            .map_err(|err| AppConfigError::Config(err.to_string()))?
            .try_deserialize()
            .map_err(|err| AppConfigError::Config(err.to_string()))?;

        // Validate network configurations
        settings.gateway.validate()?;

        // Ensure HTTP metrics configuration is provided
        if settings.http.metrics.histogram_buckets.is_empty() {
            panic!("HTTP metrics histogram buckets must be set in the configuration file.");
        }

        Ok(settings)
    }

    pub fn validate_addresses(&self) -> Result<(), AppConfigError> {
        // Create a vector of (name, address) pairs to validate
        let addresses = vec![
            ("decryption", &self.gateway.contracts.decryption_address),
            (
                "input_verification",
                &self.gateway.contracts.input_verification_address,
            ),
        ];

        // Iterate and validate each address
        for (name, address) in addresses {
            if !address.starts_with("0x") || address.len() != 42 {
                return Err(AppConfigError::InvalidAddress(format!(
                    "Invalid {name} address: {address}"
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

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    /// Log format: compact, pretty, or json
    pub format: String,
    /// Whether to show file and line information
    pub show_file_line: bool,
    /// Whether to show thread IDs
    pub show_thread_ids: bool,
    /// Whether to show timestamps (optional)
    pub show_timestamp: bool,
    /// Whether to show target module paths
    pub show_target: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, File, FileFormat};
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    /// Composable configuration builder for tests
    /// Starts with local.yaml.example and allows targeted modifications
    struct ConfigBuilder {
        config: serde_yaml::Value,
    }

    impl ConfigBuilder {
        /// Load configuration from local.yaml.example
        fn from_example() -> Result<Self, Box<dyn std::error::Error>> {
            let config_content = std::fs::read_to_string("config/local.yaml.example")?;
            let config = serde_yaml::from_str(&config_content)?;
            Ok(ConfigBuilder { config })
        }

        /// Remove a field using dot notation (e.g., "gateway.contracts.user_decrypt_shares_threshold")
        fn remove_field(mut self, path: &str) -> Self {
            let parts: Vec<&str> = path.split('.').collect();
            if let Some(parent) = self.get_parent_mut(&parts[..parts.len() - 1]) {
                if let Some(field_name) = parts.last() {
                    if let Some(mapping) = parent.as_mapping_mut() {
                        mapping.remove(field_name);
                    }
                }
            }
            self
        }

        /// Set a field value using dot notation
        #[allow(dead_code)]
        fn set_field(mut self, path: &str, value: serde_yaml::Value) -> Self {
            let parts: Vec<&str> = path.split('.').collect();
            if let Some(parent) = self.get_parent_mut(&parts[..parts.len() - 1]) {
                if let Some(field_name) = parts.last() {
                    if let Some(mapping) = parent.as_mapping_mut() {
                        mapping.insert(serde_yaml::Value::String(field_name.to_string()), value);
                    }
                }
            }
            self
        }

        /// Write configuration to a temporary file and return the path
        #[allow(clippy::wrong_self_convention)]
        fn to_temp_file(self) -> Result<PathBuf, Box<dyn std::error::Error>> {
            let content = serde_yaml::to_string(&self.config)?;
            let mut temp_file = NamedTempFile::new()?;
            temp_file.write_all(content.as_bytes())?;
            let path = temp_file.into_temp_path().keep()?;
            Ok(path)
        }

        /// Helper to navigate to parent object in the YAML tree
        fn get_parent_mut(&mut self, path: &[&str]) -> Option<&mut serde_yaml::Value> {
            let mut current = &mut self.config;
            for part in path {
                current = current.get_mut(part)?;
            }
            Some(current)
        }
    }

    #[test]
    fn test_user_decrypt_shares_threshold_is_required() {
        // Create config without user_decrypt_shares_threshold field
        let config_path = ConfigBuilder::from_example()
            .expect("Failed to load example config")
            .remove_field("gateway.contracts.user_decrypt_shares_threshold")
            .to_temp_file()
            .expect("Failed to create temp config file");

        // Try to build config - should fail because user_decrypt_shares_threshold is missing
        let config = Config::builder()
            .add_source(File::from(config_path.as_path()).format(FileFormat::Yaml))
            .build()
            .expect("Failed to build config");

        let result: Result<Settings, _> = config.try_deserialize();

        // This should fail with a deserialization error due to missing required field
        assert!(
            result.is_err(),
            "Configuration parsing should fail when user_decrypt_shares_threshold is missing"
        );

        // Check that the error mentions the missing field
        let error_msg = format!("{}", result.unwrap_err());
        assert!(
            error_msg.contains("user_decrypt_shares_threshold")
                || error_msg.contains("missing field"),
            "Error should mention the missing user_decrypt_shares_threshold field, got: {}",
            error_msg
        );
    }

    #[test]
    fn test_user_decrypt_shares_threshold_works_when_present() {
        // Create config with valid configuration (using example as-is)
        let config_path = ConfigBuilder::from_example()
            .expect("Failed to load example config")
            .to_temp_file()
            .expect("Failed to create temp config file");

        // Try to build config - should succeed
        let config = Config::builder()
            .add_source(File::from(config_path.as_path()).format(FileFormat::Yaml))
            .build()
            .expect("Failed to build config");

        let settings: Settings = config.try_deserialize().expect(
            "Configuration parsing should succeed when user_decrypt_shares_threshold is present",
        );

        // Verify the value was parsed correctly (value from local.yaml.example)
        assert_eq!(settings.gateway.contracts.user_decrypt_shares_threshold, 9);
    }
}
