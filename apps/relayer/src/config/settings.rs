use crate::http::utils::redact::redact;
use config::{Config, Environment, File};
use derivative::Derivative;
use serde::Deserializer;
use serde::{de::Error, Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::time::Duration;

// Listener pool configuration limits
const MIN_LISTENERS: usize = 2;
const MAX_LISTENERS: usize = 5;
const MIN_DEDUP_TTL_SECONDS: u64 = 1;
const MAX_DEDUP_TTL_SECONDS: u64 = 10;

/// Configuration for retrying when gateway event arrives before gw_reference_id is stored.
/// This is a workaround for the race condition where send_raw_transaction_sync has high latency.
/// TODO: Replace with proper event buffering solution.
#[derive(Debug, Deserialize, Clone)]
pub struct GwEventNotFoundRetryConfig {
    /// Maximum number of retry attempts (default: 3)
    #[serde(default = "default_gw_event_retry_max_retries")]
    pub max_retries: u32,
    /// Delay between retries in milliseconds (default: 1000)
    #[serde(default = "default_gw_event_retry_delay_ms")]
    pub retry_delay_ms: u64,
}

fn default_gw_event_retry_max_retries() -> u32 {
    3
}

fn default_gw_event_retry_delay_ms() -> u64 {
    1000
}

impl Default for GwEventNotFoundRetryConfig {
    fn default() -> Self {
        Self {
            max_retries: default_gw_event_retry_max_retries(),
            retry_delay_ms: default_gw_event_retry_delay_ms(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayConfig {
    pub blockchain_rpc: BlockchainRpcConfig,
    pub listener_pool: ListenerPoolConfig,
    pub tx_engine: TxEngineConfig,
    pub readiness_checker: ReadinessCheckConfig,
    pub contracts: ContractConfig,
    /// Retry config for gateway events arriving before gw_reference_id stored
    #[serde(default)]
    pub gw_event_not_found_retry: GwEventNotFoundRetryConfig,
}

impl GatewayConfig {
    pub fn validate(&self) -> Result<(), AppConfigError> {
        self.blockchain_rpc.validate()?;
        self.contracts.validate()?;
        self.readiness_checker.public_decrypt.validate()?;
        self.readiness_checker.user_decrypt.validate()?;
        self.readiness_checker.delegated_user_decrypt.validate()?;
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

#[derive(Deserialize, Clone, Derivative)]
#[derivative(Debug)]
pub struct BlockchainRpcConfig {
    #[derivative(Debug(format_with = "redact"))]
    pub http_url: String,
    #[derivative(Debug(format_with = "redact"))]
    pub read_http_url: String,
    pub chain_id: u64,
    pub ws_health_check_timeout_secs: u64,
    pub http_health_check_timeout_secs: u64,
}

impl BlockchainRpcConfig {
    pub fn validate(&self) -> Result<(), AppConfigError> {
        if !self.http_url.starts_with("http://") && !self.http_url.starts_with("https://") {
            return Err(AppConfigError::InvalidNetworkConfig(format!(
                "Invalid WRITE NODE HTTP URL: {}",
                self.http_url
            )));
        }
        if !self.read_http_url.starts_with("http://") && !self.read_http_url.starts_with("https://")
        {
            return Err(AppConfigError::InvalidNetworkConfig(format!(
                "Invalid READ NODE HTTP URL: {}",
                self.read_http_url
            )));
        }
        Ok(())
    }
}

/// Type of listener in the pool
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListenerType {
    /// WebSocket subscription listener (real-time events)
    Subscription,
    /// HTTP polling listener (eth_getLogs at intervals)
    Polling,
}

/// Configuration for a single listener instance
#[derive(Deserialize, Clone, Derivative)]
#[derivative(Debug)]
pub struct ListenerInstanceConfig {
    /// Type of listener: "subscription" (WebSocket) or "polling" (HTTP eth_getLogs)
    #[serde(rename = "type")]
    pub listener_type: ListenerType,
    /// URL for this listener
    /// - For subscription: ws:// or wss:// URL
    /// - For polling: http:// or https:// URL
    #[derivative(Debug(format_with = "redact"))]
    pub url: String,
}

/// Custom deserializer to handle both standard YAML arrays and
/// Env Variable indexed maps (e.g., listeners__0__url).
fn deserialize_listeners_from_map_or_seq<'de, D>(
    deserializer: D,
) -> Result<Vec<ListenerInstanceConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    // Helper enum to capture either a Sequence (YAML) or a Map (Env Var)
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MapOrSeq {
        List(Vec<ListenerInstanceConfig>),
        Map(HashMap<String, ListenerInstanceConfig>),
    }

    match MapOrSeq::deserialize(deserializer)? {
        MapOrSeq::List(list) => Ok(list),
        MapOrSeq::Map(map) => {
            // Filter keys that look like integers ("0", "1"), parse them, and sort
            let mut items: Vec<(usize, ListenerInstanceConfig)> = map
                .into_iter()
                .filter_map(|(k, v)| k.parse::<usize>().ok().map(|idx| (idx, v)))
                .collect();

            // Sort by index to ensure "0" comes before "1"
            items.sort_by_key(|(idx, _)| *idx);

            Ok(items.into_iter().map(|(_, v)| v).collect())
        }
    }
}

/// Unified listener pool configuration
/// Supports multiple listener types (WebSocket subscriptions and HTTP polling)
/// with shared deduplication and staggered connection recycling
#[derive(Debug, Deserialize, Clone)]
pub struct ListenerPoolConfig {
    /// Optional starting block number for event subscriptions
    pub last_block_number: Option<u64>,
    /// Reconnection configuration for WebSocket connection failures
    pub reconnect_config: RetrySettings,
    /// Max consecutive poll failures before giving up (polling listeners only)
    /// Should be higher than reconnect_config.max_attempts to tolerate transient errors (503, 429)
    /// Recommended: 40+ for polling vs 20 for WebSocket
    pub polling_max_attempts: u32,
    /// Connection recycle interval in minutes
    /// Staggered across all listeners to avoid simultaneous reconnections
    pub recycle_interval_mins: u64,
    /// Polling interval in milliseconds (for polling type listeners)
    pub poll_interval_ms: u64,
    /// TTL for event deduplication cache in seconds (1-10)
    pub dedup_ttl_seconds: u64,
    /// Maximum capacity for deduplication cache
    ///
    /// **Sizing guidance:**
    /// The cache should accommodate all events received during the TTL window with a safety buffer.
    ///
    /// **Formula:** `events_per_second * num_listeners * dedup_ttl_seconds * safety_buffer`
    ///
    /// **Recommended values (with 3 listeners, 5s TTL, 1.2x buffer):**
    /// - 100 events/sec → 1,800
    /// - 300 events/sec → 5,400
    /// - 1000 events/sec → 18,000
    /// - 5000 events/sec → 90,000
    pub dedup_max_capacity: usize,
    /// List of listeners in the pool
    /// Each listener has a type and URL; instance_id is assigned by position (0-indexed)
    #[serde(deserialize_with = "deserialize_listeners_from_map_or_seq")]
    pub listeners: Vec<ListenerInstanceConfig>,
}

#[derive(Deserialize, Clone, Derivative)]
#[derivative(Debug)]
pub struct TxEngineConfig {
    #[derivative(Debug(format_with = "redact"))]
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
    pub delegated_user_decrypt: UserDecryptQueueSettings,
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
pub struct HttpConfig {
    /// HTTP endpoint address to bind to (e.g., "0.0.0.0:3000").
    /// Can be None to disable HTTP server (useful for tests or metrics-only mode).
    /// When Some, server will bind to this address and update the field with actual bound address.
    pub endpoint: Option<String>,
    /// HTTP metrics configuration
    pub metrics: HttpMetricsConfig,
    /// Default retry-after seconds for queued API responses (V1 fallback)
    pub api_retry_after_seconds: u32,
    /// Enable admin endpoints for dynamic configuration updates
    #[serde(default)]
    pub enable_admin_endpoint: bool,
    /// Dynamic retry-after configuration for V2 handlers
    pub retry_after: super::retry_after::RetryAfterConfig,
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
    /// Histogram buckets for raw ETA (before clamping) in retry-after computation.
    /// Higher resolution at small values for typical requests, exponential for full queue.
    /// Example: [1, 2, 5, 10, 20, 30, 60, 120, 300, 600, 1200, 2400]
    pub retry_after_raw_eta_histogram_bucket: Vec<f64>,
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

#[derive(Deserialize, Clone, Derivative)]
#[derivative(Debug)]
pub struct StorageConfig {
    /// PostgreSQL database URL for SQL storage
    #[derivative(Debug(format_with = "redact"))]
    pub sql_database_url: String,
    /// Connection pool configuration for regular application queries
    pub app_pool: SqlPoolConfig,
    /// Connection pool configuration for cron job queries
    pub cron_pool: SqlPoolConfig,
    pub sql_health_check_timeout_secs: u64,
    pub cron: CronConfig,
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

impl ContractConfig {
    pub fn validate(&self) -> Result<(), AppConfigError> {
        if self.user_decrypt_shares_threshold < 1 {
            return Err(AppConfigError::Config(
                "user_decrypt_shares_threshold must be at least 1".to_string(),
            ));
        }
        Ok(())
    }
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

    pub fn validate_listener_pool_config(&self) -> Result<(), AppConfigError> {
        let pool_config = &self.gateway.listener_pool;

        // Validate listener count
        if pool_config.listeners.len() < MIN_LISTENERS
            || pool_config.listeners.len() > MAX_LISTENERS
        {
            return Err(AppConfigError::Config(format!(
                "listener_pool.listeners must have between {} and {} entries, got: {}",
                MIN_LISTENERS,
                MAX_LISTENERS,
                pool_config.listeners.len()
            )));
        }

        // Validate dedup TTL seconds
        if pool_config.dedup_ttl_seconds < MIN_DEDUP_TTL_SECONDS
            || pool_config.dedup_ttl_seconds > MAX_DEDUP_TTL_SECONDS
        {
            return Err(AppConfigError::Config(format!(
                "dedup_ttl_seconds must be between {} and {}, got: {}",
                MIN_DEDUP_TTL_SECONDS, MAX_DEDUP_TTL_SECONDS, pool_config.dedup_ttl_seconds
            )));
        }

        // Validate dedup max capacity (should be reasonable)
        if pool_config.dedup_max_capacity < 1000 || pool_config.dedup_max_capacity > 10_000_000 {
            return Err(AppConfigError::Config(format!(
                "dedup_max_capacity must be between 1000 and 10,000,000, got: {}",
                pool_config.dedup_max_capacity
            )));
        }

        // Validate each listener's URL format based on type
        for (i, listener) in pool_config.listeners.iter().enumerate() {
            match listener.listener_type {
                ListenerType::Subscription => {
                    if !listener.url.starts_with("ws://") && !listener.url.starts_with("wss://") {
                        return Err(AppConfigError::Config(format!(
                            "listeners[{}].url invalid for subscription type (must start with ws:// or wss://): {}",
                            i, listener.url
                        )));
                    }
                }
                ListenerType::Polling => {
                    if !listener.url.starts_with("http://") && !listener.url.starts_with("https://")
                    {
                        return Err(AppConfigError::Config(format!(
                            "listeners[{}].url invalid for polling type (must start with http:// or https://): {}",
                            i, listener.url
                        )));
                    }
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

    #[test]
    fn test_private_key_is_redacted_in_debug_output() {
        // Create a test TxEngineConfig with a dummy private key
        let tx_engine_config = TxEngineConfig {
            private_key: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            max_concurrency: 10,
            retry: RetrySettings {
                max_attempts: 3,
                retry_interval_ms: 1000,
            },
            tx_throttlers: TxThrottlersConfig {
                input_proof: TxThrottlingConfig {
                    per_seconds: 10,
                    capacity: 100,
                    safety_margin: 10,
                },
                public_decrypt: TxThrottlingConfig {
                    per_seconds: 10,
                    capacity: 100,
                    safety_margin: 10,
                },
                user_decrypt: TxThrottlingConfig {
                    per_seconds: 10,
                    capacity: 100,
                    safety_margin: 10,
                },
            },
        };

        // Get the debug output
        let debug_output = format!("{:?}", tx_engine_config);

        // Verify that the actual private key is NOT in the debug output
        assert!(
            !debug_output.contains("1234567890abcdef"),
            "Private key should not appear in debug output. Got: {}",
            debug_output
        );

        // Verify the exact format: private_key: [REDACTED]
        assert!(
            debug_output.contains("private_key: [REDACTED]"),
            "Debug output should contain 'private_key: [REDACTED]' but got: {}",
            debug_output
        );
    }

    #[test]
    fn test_sql_database_url_is_redacted_in_debug_output() {
        // Create a test StorageConfig with a dummy database URL
        let storage_config = StorageConfig {
            sql_database_url: "postgresql://user:password@localhost:5432/testdb".to_string(),
            app_pool: SqlPoolConfig {
                max_connections: 10,
                min_connections: 2,
                acquire_timeout_secs: 30,
                idle_timeout_secs: 600,
                max_lifetime_secs: 1800,
            },
            cron_pool: SqlPoolConfig {
                max_connections: 5,
                min_connections: 1,
                acquire_timeout_secs: 30,
                idle_timeout_secs: 600,
                max_lifetime_secs: 1800,
            },
            sql_health_check_timeout_secs: 5,
            cron: CronConfig {
                timeout_cron_interval: Duration::from_secs(60),
                public_decrypt_timeout: Duration::from_secs(300),
                user_decrypt_timeout: Duration::from_secs(300),
                input_proof_timeout: Duration::from_secs(300),
                expiry_cron_interval: Duration::from_secs(3600),
                public_decrypt_expiry: Duration::from_secs(7200),
                user_decrypt_expiry: Duration::from_secs(7200),
                input_proof_expiry: Duration::from_secs(7200),
                cron_startup_delay_after_recovery: Duration::from_secs(5),
            },
        };

        // Get the debug output
        let debug_output = format!("{:?}", storage_config);

        // Verify that the actual database URL (including password) is NOT in the debug output
        assert!(
            !debug_output.contains("password"),
            "Database password should not appear in debug output. Got: {}",
            debug_output
        );

        // Verify the exact format: sql_database_url: [REDACTED]
        assert!(
            debug_output.contains("sql_database_url: [REDACTED]"),
            "Debug output should contain 'sql_database_url: [REDACTED]' but got: {}",
            debug_output
        );
    }

    #[test]
    fn test_deserialize_listeners_from_indexed_env_vars() {
        // We reuse the ConfigBuilder logic from your existing tests
        let config_path = ConfigBuilder::from_example()
            .expect("Failed to load example config")
            .to_temp_file()
            .expect("Failed to create temp config file");

        // This simulates exactly what happens when you do:
        // export APP_GATEWAY__LISTENER_POOL__LISTENERS__0__TYPE=polling
        let config = Config::builder()
            .add_source(File::from(config_path.as_path()).format(FileFormat::Yaml))
            // Simulate Env Var: Index 0
            .set_override("gateway.listener_pool.listeners.0.type", "polling")
            .expect("Failed to set override")
            .set_override(
                "gateway.listener_pool.listeners.0.url",
                "http://localhost:1111",
            )
            .expect("Failed to set override")
            // Simulate Env Var: Index 1
            .set_override("gateway.listener_pool.listeners.1.type", "subscription")
            .expect("Failed to set override")
            .set_override(
                "gateway.listener_pool.listeners.1.url",
                "ws://localhost:2222",
            )
            .expect("Failed to set override")
            // Simulate Env Var: Index 2
            .set_override("gateway.listener_pool.listeners.2.type", "polling")
            .expect("Failed to set override")
            .set_override(
                "gateway.listener_pool.listeners.2.url",
                "http://localhost:3333",
            )
            .expect("Failed to set override")
            .build()
            .expect("Failed to build config");

        // 3. Deserialize
        let settings: Settings = config
            .try_deserialize()
            .expect("Failed to deserialize settings");
        let listeners = settings.gateway.listener_pool.listeners;

        assert_eq!(
            listeners.len(),
            3,
            "Should have 3 listeners from indexed overrides"
        );

        // Check Index 0
        assert_eq!(listeners[0].listener_type, ListenerType::Polling);
        assert_eq!(listeners[0].url, "http://localhost:1111");

        // Check Index 1 (Order must be preserved!)
        assert_eq!(listeners[1].listener_type, ListenerType::Subscription);
        assert_eq!(listeners[1].url, "ws://localhost:2222");

        // Check Index 2
        assert_eq!(listeners[2].listener_type, ListenerType::Polling);
        assert_eq!(listeners[2].url, "http://localhost:3333");
    }

    #[test]
    fn test_deserialize_listeners_standard_yaml_still_works() {
        let config_path = ConfigBuilder::from_example()
            .expect("Failed to load example config")
            // Reset listeners to a single item array for this test
            .to_temp_file()
            .expect("Failed to create temp config file");

        let config = Config::builder()
            .add_source(File::from(config_path.as_path()).format(FileFormat::Yaml))
            .build()
            .expect("Failed to build config");

        let settings: Settings = config.try_deserialize().expect("Failed to deserialize");
        assert_eq!(
            settings.gateway.listener_pool.listeners[0].url,
            "ws://localhost:8757"
        );
    }
}
