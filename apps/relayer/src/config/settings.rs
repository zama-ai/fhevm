use config::{Config, Environment, File};
use serde::Deserializer;
use serde::{de::Error, Deserialize, Serialize};
use std::env;
use std::fmt;
use std::time::Duration;

// Listener configuration limits
const MAX_LISTENER_INSTANCES: usize = 3;
const MIN_DEDUP_TTL_SECONDS: u64 = 1;
const MAX_DEDUP_TTL_SECONDS: u64 = 10;

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
        self.readiness_checker.public_decrypt.validate()?;
        self.readiness_checker.user_decrypt.validate()?;
        self.tx_engine
            .tx_throttlers
            .input_proof
            .validate("input proof")?;
        self.tx_engine
            .tx_throttlers
            .user_decrypt
            .validate("user decrypt")?;
        self.tx_engine
            .tx_throttlers
            .public_decrypt
            .validate("public decrypt")?;
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

/// Per-listener RPC endpoint configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ListenerRpcConfig {
    /// WebSocket URL for this listener instance (e.g., "ws://localhost:8757")
    pub ws_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ListenerConfig {
    /// Optional starting block number for event subscriptions
    pub last_block_number: Option<u64>,
    /// WebSocket reconnection configuration
    pub ws_reconnect_config: RetrySettings,
    /// Number of parallel listener instances (1-3, required)
    pub listener_instances: usize,
    /// TTL for event deduplication cache in seconds (1-10, required)
    pub dedup_ttl_seconds: u64,
    /// WebSocket connection recycle interval in minutes
    /// Connections are recycled periodically to prevent staleness issues
    /// Staggered across listener instances to avoid simultaneous reconnections
    pub ws_recycle_interval_mins: u64,
    /// Maximum capacity for deduplication cache (required)
    ///
    /// **Sizing guidance:**
    /// The cache should accommodate all events received during the TTL window with a safety buffer.
    ///
    /// **Formula:** `events_per_second * listener_instances * dedup_ttl_seconds * safety_buffer`
    ///
    /// **Recommended values (with 3 listeners, 5s TTL, 1.2x buffer):**
    /// - 100 events/sec → 1,800
    /// - 300 events/sec → 5,400
    /// - 1000 events/sec → 18,000
    /// - 5000 events/sec → 90,000
    pub dedup_max_capacity: usize,
    /// Per-listener RPC URLs (optional)
    /// If provided, each listener instance uses its corresponding URL from this list.
    /// If not provided, all instances use the default blockchain_rpc URLs.
    /// The number of entries must equal listener_instances.
    #[serde(default)]
    pub listener_urls: Option<Vec<ListenerRpcConfig>>,
}

impl ListenerConfig {
    /// Get the WebSocket URL for a specific listener instance.
    /// If listener_urls is configured, returns the URL for the given instance_id.
    /// Otherwise, falls back to the default blockchain_rpc ws_url.
    pub fn get_ws_url_for_instance(
        &self,
        instance_id: usize,
        default: &BlockchainRpcConfig,
    ) -> String {
        if let Some(ref urls) = self.listener_urls {
            if let Some(cfg) = urls.get(instance_id) {
                return cfg.ws_url.clone();
            }
        }
        default.ws_url.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TxEngineConfig {
    pub private_key: String,
    pub max_concurrency: u16,
    pub retry: RetrySettings,
    pub tx_throttlers: TxThrottlersConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TxThrottlersConfig {
    pub input_proof: TxThrottlingConfig,
    pub public_decrypt: TxThrottlingConfig,
    pub user_decrypt: TxThrottlingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TxThrottlingConfig {
    pub per_seconds: u32,
    pub capacity: usize,
    pub safety_margin: usize,
}

impl TxThrottlingConfig {
    pub fn validate(&self, name: &str) -> Result<(), AppConfigError> {
        if self.capacity == 0 {
            return Err(AppConfigError::Config(format!(
                "Tx throttler {} capacity should be superior to 0: {}",
                name, self.capacity
            )));
        }
        if self.safety_margin >= self.capacity {
            return Err(AppConfigError::Config(format!(
                "Tx throttler {} safety margin should be inferior strictly to capacity: cap:{}, margin:{}",
                name,
                self.capacity,
                self.safety_margin,
            )));
        }
        if self.per_seconds == 0 {
            return Err(AppConfigError::Config(format!(
                "Tx throttler {} drain capacity should be superior to 0: {}",
                name, self.per_seconds
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReadinessCheckConfig {
    pub public_decrypt: PublicDecryptQueueSettings,
    pub user_decrypt: UserDecryptQueueSettings,
    pub retry: RetrySettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PublicDecryptQueueSettings {
    pub max_concurrency: usize,
    pub capacity: usize,
    pub safety_margin: usize,
}

impl PublicDecryptQueueSettings {
    pub fn validate(&self) -> Result<(), AppConfigError> {
        if self.capacity == 0 {
            return Err(AppConfigError::Config(format!(
                "Public decrypt queue capacity should be superior to 0: {}",
                self.capacity
            )));
        }
        if self.safety_margin >= self.capacity {
            return Err(AppConfigError::Config(format!(
                "Public decrypt queue safety margin should be inferior strictly to capacity: cap:{}, margin:{}",
                self.capacity,
                self.safety_margin,
            )));
        }
        if self.max_concurrency == 0 {
            return Err(AppConfigError::Config(format!(
                "Public decrypt queue max concurrency should be superior to 0: {}",
                self.max_concurrency
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserDecryptQueueSettings {
    pub max_concurrency: usize,
    pub capacity: usize,
    pub safety_margin: usize,
}

impl UserDecryptQueueSettings {
    pub fn validate(&self) -> Result<(), AppConfigError> {
        if self.capacity == 0 {
            return Err(AppConfigError::Config(format!(
                "Public decrypt queue capacity should be superior to 0: {}",
                self.capacity
            )));
        }
        if self.safety_margin >= self.capacity {
            return Err(AppConfigError::Config(format!(
                "Public decrypt queue safety margin should be inferior strictly to capacity: cap:{}, margin:{}",
                self.capacity,
                self.safety_margin,
            )));
        }
        if self.max_concurrency == 0 {
            return Err(AppConfigError::Config(format!(
                "Public decrypt queue max concurrency should be superior to 0: {}",
                self.max_concurrency
            )));
        }
        Ok(())
    }
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
    /// Default retry-after seconds for queued API responses
    pub api_retry_after_seconds: u32,
    /// Enable admin endpoints for dynamic configuration updates
    #[serde(default)]
    pub enable_admin_endpoint: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsConfig {
    /// Endpoint for metrics server (e.g., "0.0.0.0:9898")
    pub endpoint: String,

    // metrics buckets.
    pub query_duration_histogram_bucket: Vec<f64>,
    pub pool_wait_duration_seconds_histogram_bucket: Vec<f64>,
    pub request_status_duration_histogram_bucket: Vec<f64>,
    pub transaction_duration_secs_histogram_bucket: Vec<f64>,
}

/// Deserializes strings like "30s", "5m", "1d" into std::time::Duration.
/// 'y' not supported
fn deserialize_human_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    // Use humantime for standard units (d, h, m, s, ms)
    humantime::parse_duration(&s).map_err(Error::custom)
}

#[derive(Debug, Deserialize, Clone)]
pub struct CronConfig {
    // We map the YAML key `timeout_cron_interval_secs` to this field,
    // but parse the string value into a Duration.
    #[serde(deserialize_with = "deserialize_human_duration")]
    pub timeout_cron_interval: Duration,
    #[serde(deserialize_with = "deserialize_human_duration")]
    pub public_decrypt_timeout: Duration,
    #[serde(deserialize_with = "deserialize_human_duration")]
    pub user_decrypt_timeout: Duration,
    #[serde(deserialize_with = "deserialize_human_duration")]
    pub input_proof_timeout: Duration,
    #[serde(deserialize_with = "deserialize_human_duration")]
    pub expiry_cron_interval: Duration,
    #[serde(deserialize_with = "deserialize_human_duration")]
    pub public_decrypt_expiry: Duration,
    #[serde(deserialize_with = "deserialize_human_duration")]
    pub user_decrypt_expiry: Duration,
    #[serde(deserialize_with = "deserialize_human_duration")]
    pub input_proof_expiry: Duration,
    /// Delay before starting cron workers after recovery completes.
    /// This gives recovered requests time to process before timeout checks begin.
    /// Must be less than 10% of both timeout_cron_interval and expiry_cron_interval.
    #[serde(deserialize_with = "deserialize_human_duration")]
    pub cron_startup_delay_after_recovery: Duration,
}

impl CronConfig {
    /// Validates that cron startup delay is less than 10% of timeout/expiry durations.
    ///
    /// The 10% rule ensures the startup delay is a small fraction of the actual timeout
    /// and expiry durations, preventing excessive delays while still providing adequate
    /// breathing room for recovered requests.
    pub fn validate(&self) -> Result<(), AppConfigError> {
        let delay_secs = self.cron_startup_delay_after_recovery.as_secs_f64();

        // Find minimum timeout duration (for timeout cron validation)
        let min_timeout_secs = self
            .public_decrypt_timeout
            .as_secs_f64()
            .min(self.user_decrypt_timeout.as_secs_f64())
            .min(self.input_proof_timeout.as_secs_f64());

        // Check: delay < 10% of minimum timeout duration
        let timeout_max_delay = min_timeout_secs * 0.1;
        if delay_secs >= timeout_max_delay {
            return Err(AppConfigError::InvalidCronConfig(
                format!(
                    "cron_startup_delay_after_recovery ({}s) must be less than 10% of minimum timeout duration ({}s). Max allowed: {}s",
                    delay_secs, min_timeout_secs, timeout_max_delay
                )
            ));
        }

        // Find minimum expiry duration (for expiry cron validation)
        let min_expiry_secs = self
            .public_decrypt_expiry
            .as_secs_f64()
            .min(self.user_decrypt_expiry.as_secs_f64())
            .min(self.input_proof_expiry.as_secs_f64());

        // Check: delay < 10% of minimum expiry duration
        let expiry_max_delay = min_expiry_secs * 0.1;
        if delay_secs >= expiry_max_delay {
            return Err(AppConfigError::InvalidCronConfig(
                format!(
                    "cron_startup_delay_after_recovery ({}s) must be less than 10% of minimum expiry duration ({}s). Max allowed: {}s",
                    delay_secs, min_expiry_secs, expiry_max_delay
                )
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SqlPoolConfig {
    /// Maximum number of connections in the SQL connection pool
    pub max_connections: u32,
    /// Minimum number of idle connections to maintain
    pub min_connections: u32,
    /// Connection acquire timeout in seconds
    pub acquire_timeout_secs: u64,
    /// Idle connection timeout in seconds
    pub idle_timeout_secs: u64,
    /// Maximum connection lifetime in seconds
    pub max_lifetime_secs: u64,
}

#[derive(Deserialize, Clone)]
pub struct StorageConfig {
    /// PostgreSQL database URL for SQL storage
    pub sql_database_url: String,
    /// Connection pool configuration for regular application queries
    pub app_pool: SqlPoolConfig,
    /// Connection pool configuration for cron job queries
    pub cron_pool: SqlPoolConfig,
    pub sql_health_check_timeout_secs: u64,
    pub cron: CronConfig,
}

impl fmt::Debug for StorageConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StorageConfig")
            .field("sql_database_url", &"[REDACTED]")
            .field("app_pool", &self.app_pool)
            .field("cron_pool", &self.cron_pool)
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
            "StorageConfig {{ sql_database_url: [REDACTED], app_pool: max_connections: {}, cron_pool: max_connections: {} }}",
            self.app_pool.max_connections, self.cron_pool.max_connections
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
pub struct GlobalSettings {
    /// Determine if we are in the test setup directly from the configuration.
    #[serde(default = "default_test_mock")]
    pub test_mock: bool,
}

fn default_test_mock() -> bool {
    false
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            test_mock: default_test_mock(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
/// Top-level configuration structure.
///
/// Contains all configuration settings for the relayer service.
pub struct Settings {
    #[serde(default)]
    /// General settings
    pub global: GlobalSettings,
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

    #[error("Invalid cron configuration: {0}")]
    InvalidCronConfig(String),
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

        // Validate cron startup delay (10% rule)
        settings.storage.cron.validate()?;

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

    pub fn validate_listener_config(&self) -> Result<(), AppConfigError> {
        let listener_config = &self.gateway.listener;

        // Validate listener instances count
        if listener_config.listener_instances < 1
            || listener_config.listener_instances > MAX_LISTENER_INSTANCES
        {
            return Err(AppConfigError::Config(format!(
                "listener_instances must be between 1 and {}, got: {}",
                MAX_LISTENER_INSTANCES, listener_config.listener_instances
            )));
        }

        // Validate dedup TTL seconds
        if listener_config.dedup_ttl_seconds < MIN_DEDUP_TTL_SECONDS
            || listener_config.dedup_ttl_seconds > MAX_DEDUP_TTL_SECONDS
        {
            return Err(AppConfigError::Config(format!(
                "dedup_ttl_seconds must be between {} and {}, got: {}",
                MIN_DEDUP_TTL_SECONDS, MAX_DEDUP_TTL_SECONDS, listener_config.dedup_ttl_seconds
            )));
        }

        // Validate dedup max capacity (should be reasonable)
        if listener_config.dedup_max_capacity < 1000
            || listener_config.dedup_max_capacity > 10_000_000
        {
            return Err(AppConfigError::Config(format!(
                "dedup_max_capacity must be between 1000 and 10,000,000, got: {}",
                listener_config.dedup_max_capacity
            )));
        }

        // Validate listener_urls if provided
        if let Some(ref urls) = listener_config.listener_urls {
            // URL count must match listener_instances
            if urls.len() != listener_config.listener_instances {
                return Err(AppConfigError::Config(format!(
                    "listener_urls count ({}) must equal listener_instances ({})",
                    urls.len(),
                    listener_config.listener_instances
                )));
            }

            // Validate each URL format
            for (i, url_cfg) in urls.iter().enumerate() {
                if !url_cfg.ws_url.starts_with("ws://") && !url_cfg.ws_url.starts_with("wss://") {
                    return Err(AppConfigError::Config(format!(
                        "listener_urls[{}].ws_url invalid (must start with ws:// or wss://): {}",
                        i, url_cfg.ws_url
                    )));
                }
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
