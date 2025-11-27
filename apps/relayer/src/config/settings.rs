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
    /// HTTP endpoint address
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
}

impl fmt::Debug for StorageConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StorageConfig")
            .field("sql_database_url", &"[REDACTED]")
            .field("sql_max_connections", &self.sql_max_connections)
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
    use tempfile::NamedTempFile;

    #[test]
    fn test_user_decrypt_shares_threshold_is_required() {
        let config_content = r#"
gateway:
  blockchain_rpc:
    ws_url: "wss://test-gateway.example.com"
    http_url: "https://test-gateway.example.com"
    chain_id: 8009
  listener: {}
  tx_engine:
    private_key: "0x1234567890123456789012345678901234567890123456789012345678901234"
    max_concurrency: 100
  readiness_checker:
    max_concurrency: 100
    retry:
      max_attempts: 3
      retry_interval_ms: 2000
  contracts:
    decryption_address: "0x1234567890123456789012345678901234567890"
    input_verification_address: "0x1234567890123456789012345678901234567890"
    # Note: user_decrypt_shares_threshold is missing here
log:
  format: "compact"
  show_file_line: false
  show_thread_ids: false
  show_timestamp: true
  show_target: true
keyurl:
  fhe_public_key:
    data_id: "test-key"
    url: "https://test.example.com/key"
  crs:
    data_id: "test-crs"
    url: "https://test.example.com/crs"
http:
  rate_limit_on_post_endpoints:
    requests_per_second: 30
    burst_size: 30
    retry_after_seconds: 3
    jitter_max_ms: 2000
  metrics:
    histogram_buckets: [0.001, 0.01, 0.1, 1.0, 10.0]
metrics:
  endpoint: "0.0.0.0:9898"
storage:
  sql_database_url: "postgresql://postgres:postgres@localhost:5432/relayer_db"
  sql_max_connections: 10
"#;

        // Create a temporary config file
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(config_content.as_bytes())
            .expect("Failed to write config");

        // Try to build config - should fail because user_decrypt_shares_threshold is missing
        let config = Config::builder()
            .add_source(File::from(temp_file.path()).format(FileFormat::Yaml))
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
        let config_content = r#"
gateway:
  blockchain_rpc:
    ws_url: "wss://test-gateway.example.com"
    http_url: "https://test-gateway.example.com"
    chain_id: 8009
  listener:
    ws_reconnect_config:
      max_attempts: 30
      retry_interval_ms: 1000
  tx_engine:
    retry:
      max_attempts: 100
      retry_interval_ms: 500
    private_key: "0x1234567890123456789012345678901234567890123456789012345678901234"
    max_concurrency: 100
  readiness_checker:
    max_concurrency: 100
    retry:
      max_attempts: 3
      retry_interval_ms: 2000
  contracts:
    decryption_address: "0x1234567890123456789012345678901234567890"
    input_verification_address: "0x1234567890123456789012345678901234567890"
    user_decrypt_shares_threshold: 9
log:
  format: "compact"
  show_file_line: false
  show_thread_ids: false
  show_timestamp: true
  show_target: true
keyurl:
  fhe_public_key:
    data_id: "test-key"
    url: "https://test.example.com/key"
  crs:
    data_id: "test-crs"
    url: "https://test.example.com/crs"
http:
  rate_limit_post_endpoints:
    requests_per_second: 30
    burst_size: 30
    retry_after_seconds: 3
    jitter_max_ms: 2000
  metrics:
    histogram_buckets: [0.001, 0.01, 0.1, 1.0, 10.0]
metrics:
  endpoint: "0.0.0.0:9898"
storage:
  sql_database_url: "postgresql://postgres:postgres@localhost:5432/relayer_db"
  sql_max_connections: 10
"#;

        // Create a temporary config file
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(config_content.as_bytes())
            .expect("Failed to write config");

        // Try to build config - should succeed
        let config = Config::builder()
            .add_source(File::from(temp_file.path()).format(FileFormat::Yaml))
            .build()
            .expect("Failed to build config");

        let settings: Settings = config
            .try_deserialize()
            .expect("Configuration parsing should succeed when expected_share_count is present");

        // Verify the value was parsed correctly
        assert_eq!(settings.gateway.contracts.user_decrypt_shares_threshold, 9);
    }
}
