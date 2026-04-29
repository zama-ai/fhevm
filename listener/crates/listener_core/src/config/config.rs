//! Configuration module for listener_core.
//!
//! Loads settings from YAML files and/or environment variables.
//! Environment variables override file values using format: APP_SECTION__FIELD
//!
//! # Example
//! ```bash
//! APP_DATABASE__DB_URL="postgres://..." cargo run
//! ```
//!
//! # Broker Configuration
//!
//! The broker section supports both AMQP (RabbitMQ) and Redis Streams backends.
//! Set `broker_type` to switch between them:
//!
//! ```yaml
//! broker:
//!   broker_type: redis  # or 'amqp'
//!   broker_url: redis://localhost:6379
//! ```

use config::{Config, Environment, File};
use derivative::Derivative;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use thiserror::Error;

const MIN_RANGE_SIZE: usize = 1;
const MAX_RANGE_SIZE: usize = 10000;
const MIN_PARALLEL_REQUESTS: usize = 1;
const MAX_PARALLEL_REQUESTS: usize = 200;
const MIN_BATCH_RANGE: usize = 1;

/// 1 advisory-lock session + 1 concurrent query at least.
const MIN_POOL_MIN_CONNECTIONS: u32 = 2;
/// 1 advisory-lock + 4 concurrent handler queries (fetch/reorg, watch, unwatch, cleaner).
const MIN_POOL_MAX_CONNECTIONS: u32 = 5;
const MAX_BATCH_RANGE: usize = 100;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration error: {0}")]
    Parse(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

fn redact<T>(_: &T, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[REDACTED]")
}

/// Broker backend type.
///
/// Determines which message broker to use for publishing and consuming events.
#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BrokerType {
    /// RabbitMQ/AMQP backend
    Amqp,
    #[default]
    /// Redis Streams backend (default)
    Redis,
}

#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BlockFetcherStrategy {
    #[default]
    BlockReceipts,
    BatchReceiptsFull,
    BatchReceiptsRange,
    TransactionReceiptsParallel,
    TransactionReceiptsSequential,
}

/// Configuration for the starting block when the database is empty.
///
/// Accepts either `"current"` (resolves to chain tip - 1 at init) or a specific block number.
/// Defaults to `Current`.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum BlockStartConfig {
    /// Start from the current chain height minus 1 (for reorg safety at first block).
    #[default]
    Current,
    /// Start from a specific block number.
    Number(u64),
}

impl fmt::Display for BlockStartConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockStartConfig::Current => write!(f, "current"),
            BlockStartConfig::Number(n) => write!(f, "{}", n),
        }
    }
}

impl<'de> Deserialize<'de> for BlockStartConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BlockStartVisitor;

        impl<'de> Visitor<'de> for BlockStartVisitor {
            type Value = BlockStartConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("\"current\" or a block number (u64)")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BlockStartConfig::Number(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(de::Error::custom(format!(
                        "invalid block_start_on_first_start: block number cannot be negative, got {}",
                        v
                    )));
                }
                Ok(BlockStartConfig::Number(v as u64))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v.eq_ignore_ascii_case("current") {
                    return Ok(BlockStartConfig::Current);
                }
                match v.parse::<u64>() {
                    Ok(n) => Ok(BlockStartConfig::Number(n)),
                    Err(_) => Err(de::Error::custom(format!(
                        "invalid block_start_on_first_start: expected 'current' or a block number, got '{}'",
                        v
                    ))),
                }
            }
        }

        deserializer.deserialize_any(BlockStartVisitor)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct StrategyConfig {
    #[serde(default)]
    pub block_start_on_first_start: BlockStartConfig,

    #[serde(default = "default_range_size")]
    pub range_size: usize,

    #[serde(default = "default_max_parallel_requests")]
    pub max_parallel_requests: usize,

    #[serde(default)]
    pub block_fetcher: BlockFetcherStrategy,

    #[serde(default = "default_batch_receipts_size_range")]
    pub batch_receipts_size_range: usize,

    #[serde(default)]
    pub compute_block: Option<bool>,

    /// Active only if compute_block is active, other wise, computation is fully skipped.
    /// When true, block verification will skip computation.
    /// (e.g. Polygon type 0x7F for transaction root) with an ERROR log instead of failing.
    /// When false, unsupported types cause a hard verification failure.
    /// Defaults to true.
    #[serde(default = "default_compute_block_allow_skipping")]
    pub compute_block_allow_skipping: bool,

    #[serde(default)]
    pub automatic_startup: bool,

    #[serde(default = "default_loop_delay_ms")]
    pub loop_delay_ms: u64,

    #[serde(default = "default_max_exponential_backoff_ms")]
    pub max_exponential_backoff_ms: u64,

    #[serde(default)]
    pub publish: PublishConfig,
}

fn default_range_size() -> usize {
    100
}
fn default_max_parallel_requests() -> usize {
    50
}
fn default_batch_receipts_size_range() -> usize {
    10
}
fn default_loop_delay_ms() -> u64 {
    1000
}
fn default_max_exponential_backoff_ms() -> u64 {
    20_000
}
fn default_compute_block_allow_skipping() -> bool {
    true
}
fn default_publish_retry_secs() -> u64 {
    1
}
fn default_publish_stale() -> bool {
    true
}
fn default_publish_no_stale_retries() -> u32 {
    5
}

#[derive(Debug, Deserialize, Clone)]
pub struct PublishConfig {
    #[serde(default = "default_publish_retry_secs")]
    pub publish_retry_secs: u64,

    #[serde(default = "default_publish_stale")]
    pub publish_stale: bool,

    #[serde(default = "default_publish_no_stale_retries")]
    pub publish_no_stale_retries: u32,
}

impl Default for PublishConfig {
    fn default() -> Self {
        Self {
            publish_retry_secs: default_publish_retry_secs(),
            publish_stale: default_publish_stale(),
            publish_no_stale_retries: default_publish_no_stale_retries(),
        }
    }
}

impl StrategyConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.range_size < MIN_RANGE_SIZE || self.range_size > MAX_RANGE_SIZE {
            return Err(ConfigError::Validation(format!(
                "range size must be between {} and {}, got: {}",
                MIN_RANGE_SIZE, MAX_RANGE_SIZE, self.range_size
            )));
        }
        if self.max_parallel_requests < MIN_PARALLEL_REQUESTS
            || self.max_parallel_requests > MAX_PARALLEL_REQUESTS
        {
            return Err(ConfigError::Validation(format!(
                "max_parallel_requests must be between {} and {}, got: {}",
                MIN_PARALLEL_REQUESTS, MAX_PARALLEL_REQUESTS, self.max_parallel_requests
            )));
        }
        if self.batch_receipts_size_range < MIN_BATCH_RANGE
            || self.batch_receipts_size_range > MAX_BATCH_RANGE
        {
            return Err(ConfigError::Validation(format!(
                "batch_receipts_size_range must be between {} and {}, got: {}",
                MIN_BATCH_RANGE, MAX_BATCH_RANGE, self.batch_receipts_size_range
            )));
        }
        Ok(())
    }
}

impl Default for StrategyConfig {
    fn default() -> Self {
        Self {
            block_start_on_first_start: BlockStartConfig::default(),
            range_size: default_range_size(),
            max_parallel_requests: default_max_parallel_requests(),
            block_fetcher: BlockFetcherStrategy::default(),
            batch_receipts_size_range: default_batch_receipts_size_range(),
            compute_block: None,
            compute_block_allow_skipping: default_compute_block_allow_skipping(),
            automatic_startup: true,
            loop_delay_ms: default_loop_delay_ms(),
            max_exponential_backoff_ms: default_max_exponential_backoff_ms(),
            publish: PublishConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CleanerConfig {
    #[serde(default = "default_cleaner_active")]
    pub active: bool,

    #[serde(default = "default_blocks_to_keep")]
    pub blocks_to_keep: u64,

    #[serde(default = "default_cron_secs")]
    pub cron_secs: u64,
}

fn default_cleaner_active() -> bool {
    true
}
fn default_blocks_to_keep() -> u64 {
    1000
}
fn default_cron_secs() -> u64 {
    3600
}

impl Default for CleanerConfig {
    fn default() -> Self {
        Self {
            active: default_cleaner_active(),
            blocks_to_keep: default_blocks_to_keep(),
            cron_secs: default_cron_secs(),
        }
    }
}

#[derive(Deserialize, Clone, Derivative)]
#[derivative(Debug)]
pub struct BlockchainConfig {
    #[serde(default = "default_blockchain_type")]
    pub r#type: String,

    pub chain_id: u64, // REQUIRED

    #[derivative(Debug(format_with = "redact"))]
    pub rpc_url: String, // REQUIRED, REDACTED

    pub network: String, // REQUIRED

    #[serde(default = "default_finality_depth")]
    pub finality_depth: u64,

    #[serde(default)]
    pub cleaner: CleanerConfig,

    #[serde(default)]
    pub strategy: StrategyConfig,
}

fn default_finality_depth() -> u64 {
    64
}

fn default_blockchain_type() -> String {
    "evm".to_string()
}

impl BlockchainConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if !self.rpc_url.starts_with("http://") && !self.rpc_url.starts_with("https://") {
            return Err(ConfigError::Validation(
                "rpc_url must start with http:// or https://".to_string(),
            ));
        }
        if self.chain_id == 0 {
            return Err(ConfigError::Validation(
                "chain_id must be strictly positive".to_string(),
            ));
        }
        if self.cleaner.blocks_to_keep < 2 {
            return Err(ConfigError::Validation(format!(
                "cleaner.blocks_to_keep ({}) must be >= 2 for reorg checking",
                self.cleaner.blocks_to_keep
            )));
        }
        if self.cleaner.blocks_to_keep <= self.finality_depth {
            return Err(ConfigError::Validation(format!(
                "cleaner.blocks_to_keep ({}) must be greater than finality_depth ({})",
                self.cleaner.blocks_to_keep, self.finality_depth
            )));
        }
        self.strategy.validate()?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PoolConfig {
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_acquire_timeout_secs")]
    pub acquire_timeout_secs: u64,
    #[serde(default = "default_idle_timeout_secs")]
    pub idle_timeout_secs: u64,
    #[serde(default = "default_max_lifetime_secs")]
    pub max_lifetime_secs: u64,
}

fn default_max_connections() -> u32 {
    10
}
fn default_min_connections() -> u32 {
    2
}
fn default_acquire_timeout_secs() -> u64 {
    5
}
fn default_idle_timeout_secs() -> u64 {
    600
}
fn default_max_lifetime_secs() -> u64 {
    1800
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: default_max_connections(),
            min_connections: default_min_connections(),
            acquire_timeout_secs: default_acquire_timeout_secs(),
            idle_timeout_secs: default_idle_timeout_secs(),
            max_lifetime_secs: default_max_lifetime_secs(),
        }
    }
}

impl PoolConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.min_connections < MIN_POOL_MIN_CONNECTIONS {
            return Err(ConfigError::Validation(format!(
                "pool.min_connections ({}) must be >= {} (1 advisory-lock session + 1 query connection)",
                self.min_connections, MIN_POOL_MIN_CONNECTIONS
            )));
        }
        if self.max_connections < MIN_POOL_MAX_CONNECTIONS {
            return Err(ConfigError::Validation(format!(
                "pool.max_connections ({}) must be >= {} (1 advisory-lock + 4 concurrent handler queries)",
                self.max_connections, MIN_POOL_MAX_CONNECTIONS
            )));
        }
        if self.max_connections < self.min_connections {
            return Err(ConfigError::Validation(format!(
                "pool.max_connections ({}) must be >= pool.min_connections ({})",
                self.max_connections, self.min_connections
            )));
        }
        Ok(())
    }
}

#[derive(Deserialize, Clone, Derivative)]
#[derivative(Debug)]
pub struct DatabaseConfig {
    #[derivative(Debug(format_with = "redact"))]
    pub db_url: String, // REQUIRED, REDACTED — used by both password and IAM modes
    /// IAM auth configuration. When absent or enabled=false, password from db_url is used.
    /// When enabled=true, host/port/user/dbname are parsed from db_url and the password
    /// is replaced with a short-lived IAM token.
    /// Always deserialized (even without iam-auth feature) so config errors are visible.
    #[serde(default)]
    pub iam_auth: Option<IamAuthConfig>,
    #[serde(default = "default_migration_max_attempts")]
    pub migration_max_attempts: u32,
    #[serde(default)]
    pub pool: PoolConfig,
}

fn default_migration_max_attempts() -> u32 {
    5
}

/// IAM role-based RDS authentication configuration.
/// Connection details (host, port, user, dbname) are parsed from `db_url`.
/// This struct is always deserialized regardless of the `iam-auth` feature flag,
/// so that misconfiguration is detected at startup rather than silently ignored.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct IamAuthConfig {
    /// Opt-in flag. When false (or section absent), falls back to password in db_url.
    #[serde(default)]
    pub enabled: bool,
    /// Optional path to a custom RDS CA bundle PEM file.
    /// If not set, relies on the system/rustls trust store (supports modern RDS CAs).
    pub ssl_ca_path: Option<String>,
}

impl DatabaseConfig {
    /// Returns true if IAM auth is configured and enabled.
    pub fn use_iam_auth(&self) -> bool {
        self.iam_auth.as_ref().is_some_and(|c| c.enabled)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if !self.db_url.starts_with("postgres://") && !self.db_url.starts_with("postgresql://") {
            return Err(ConfigError::Validation(
                "db_url must start with postgres:// or postgresql://".to_string(),
            ));
        }

        // Catch misconfiguration: IAM auth requested but feature not compiled in
        #[cfg(not(feature = "iam-auth"))]
        if self.use_iam_auth() {
            return Err(ConfigError::Validation(
                "iam_auth.enabled=true but binary was built without the iam-auth feature"
                    .to_string(),
            ));
        }

        self.pool.validate()?;
        Ok(())
    }
}

/// Unified broker configuration supporting both AMQP and Redis backends.
///
/// # Backend Selection
///
/// Set `broker_type` to choose the backend:
/// - `amqp` (default): Uses RabbitMQ with exchanges and queues
/// - `redis`: Uses Redis Streams with consumer groups
///
/// # URL Requirements
///
/// The `broker_url` must match the `broker_type`:
/// - For AMQP: URL starting with `amqp://` or `amqps://`
/// - For Redis: URL starting with `redis://` or `rediss://`
///
/// # Durability
///
/// Set `ensure_publish` to enable replication-aware publish durability:
/// - Redis: issues `WAIT 1 500` after every `XADD`
/// - AMQP: enables publisher confirms (`confirm_select`)
///
/// # Example
///
/// ```yaml
/// broker:
///   broker_type: redis
///   broker_url: redis://localhost:6379
///   ensure_publish: true
/// ```
#[derive(Deserialize, Clone, Derivative)]
#[derivative(Debug)]
pub struct BrokerConfig {
    /// Backend type: `amqp` or `redis`
    #[serde(default)]
    pub broker_type: BrokerType,

    /// Broker connection URL
    #[derivative(Debug(format_with = "redact"))]
    pub broker_url: String,

    /// Enable replication-aware publish durability.
    ///
    /// - Redis: issues `WAIT` after every `XADD` to confirm replication
    /// - AMQP: enables publisher confirms (`confirm_select`)
    ///
    /// Default: `false` (backward compatible)
    #[serde(default)]
    pub ensure_publish: bool,

    #[serde(default = "default_circuit_breaker_cooldown_secs")]
    pub circuit_breaker_cooldown_secs: u64,

    #[serde(default = "default_circuit_breaker_threshold")]
    pub circuit_breaker_threshold: u32,

    #[serde(default = "default_claim_min_idle")]
    pub claim_min_idle: u64,
}

fn default_claim_min_idle() -> u64 {
    // Defaulted to 30,
    // to add buffer the value from semaphore evm rpc provider http client timeout.
    30
}

fn default_circuit_breaker_cooldown_secs() -> u64 {
    10
}

fn default_circuit_breaker_threshold() -> u32 {
    3
}

impl BrokerConfig {
    /// Get the broker URL.
    pub fn url(&self) -> &str {
        &self.broker_url
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        match self.broker_type {
            BrokerType::Amqp => {
                if !self.broker_url.starts_with("amqp://")
                    && !self.broker_url.starts_with("amqps://")
                {
                    return Err(ConfigError::Validation(
                        "broker_url must start with amqp:// or amqps:// when broker_type is 'amqp'"
                            .to_string(),
                    ));
                }
            }
            BrokerType::Redis => {
                if !self.broker_url.starts_with("redis://")
                    && !self.broker_url.starts_with("rediss://")
                {
                    return Err(ConfigError::Validation(
                        "broker_url must start with redis:// or rediss:// when broker_type is 'redis'"
                            .to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Telemetry / metrics configuration.
///
/// When `enabled` is `true` (default), the binary starts a Prometheus HTTP
/// server on `metrics_port` that serves `/metrics`.
#[derive(Debug, Deserialize, Clone)]
pub struct TelemetrySettings {
    /// Enable the Prometheus metrics endpoint. Default: `true`.
    #[serde(default = "default_telemetry_enabled")]
    pub enabled: bool,
    /// Port for the Prometheus `/metrics` endpoint. Default: `9090`.
    #[serde(default = "default_metrics_port")]
    pub metrics_port: u16,
}

fn default_telemetry_enabled() -> bool {
    true
}
fn default_metrics_port() -> u16 {
    9090
}

impl Default for TelemetrySettings {
    fn default() -> Self {
        Self {
            enabled: default_telemetry_enabled(),
            metrics_port: default_metrics_port(),
        }
    }
}

/// Log format selection.
#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    /// JSON output — Loki-ready for production ingestion.
    #[default]
    Json,
    /// Compact single-line human-readable format.
    Compact,
    /// Pretty multi-line format for local development.
    Pretty,
}

/// Logging configuration.
///
/// Controls tracing-subscriber initialization: output format, verbosity,
/// and which metadata fields are included in each log line.
///
/// All fields have sensible production defaults and can be overridden via
/// YAML config or environment variables (`APP_LOG__FORMAT`, `APP_LOG__LEVEL`, etc.).
#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    /// Output format: `json`, `compact`, or `pretty`.
    #[serde(default)]
    pub format: LogFormat,

    /// Show source file and line number in log output.
    #[serde(default)]
    pub show_file_line: bool,

    /// Show thread IDs in log output.
    #[serde(default = "default_true")]
    pub show_thread_ids: bool,

    /// Show RFC 3339 timestamps in log output.
    #[serde(default = "default_true")]
    pub show_timestamp: bool,

    /// Show the tracing target (module path) in log output.
    #[serde(default = "default_true")]
    pub show_target: bool,

    /// Inject `name`, `network`, and `chain_id` as constant fields in every log line.
    #[serde(default = "default_true")]
    pub show_constants: bool,

    /// Default log level. Overridden entirely by `RUST_LOG` env var when set.
    #[serde(default = "default_log_level")]
    pub level: String,
}

fn default_true() -> bool {
    true
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            format: LogFormat::default(),
            show_file_line: false,
            show_thread_ids: true,
            show_timestamp: true,
            show_target: true,
            show_constants: true,
            level: default_log_level(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[serde(default = "default_name")]
    pub name: String,

    /// Port of the shared application HTTP server.
    ///
    /// **Required — no default.** Hosts the Kubernetes health probes
    /// (`/livez`, `/readyz`) today; designed as the single mount point
    /// for future operational routes (metrics, admin endpoints, ...).
    pub http_port: u16,

    pub database: DatabaseConfig,
    pub broker: BrokerConfig,
    pub blockchain: BlockchainConfig,
    #[serde(default)]
    pub telemetry: TelemetrySettings,
    #[serde(default)]
    pub log: LogConfig,
}

fn default_name() -> String {
    "listener".to_string()
}

impl Settings {
    /// Load config from file and/or environment variables.
    ///
    /// Environment variables override file values.
    /// Format: `APP_SECTION__FIELD` (double underscore separator)
    ///
    /// # Example
    /// ```bash
    /// APP_DATABASE__DB_URL="postgres://..." cargo run
    /// ```
    pub fn new(config_file: Option<&str>) -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        // 1. Load from file if provided
        if let Some(file) = config_file {
            builder = builder.add_source(File::with_name(file).required(true));
        }

        // 2. Load from environment (APP_SECTION__FIELD format)
        builder = builder.add_source(
            Environment::with_prefix("APP")
                .separator("__")
                .prefix_separator("_"),
        );

        // 3. Build and deserialize
        let settings: Settings = builder
            .build()
            .map_err(|e| ConfigError::Parse(e.to_string()))?
            .try_deserialize()
            .map_err(|e| ConfigError::Parse(e.to_string()))?;

        // 4. Validate all sections
        settings.validate()?;

        Ok(settings)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.http_port == 0 {
            return Err(ConfigError::Validation(
                "http_port must be a non-zero port number (no default is provided; configure it explicitly)".to_string(),
            ));
        }
        self.database.validate()?;
        self.broker.validate()?;
        self.blockchain.validate()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_config_defaults() {
        let config = StrategyConfig::default();
        assert_eq!(config.block_start_on_first_start, BlockStartConfig::Current);
        assert_eq!(config.range_size, 100);
        assert_eq!(config.max_parallel_requests, 50);
        assert_eq!(config.block_fetcher, BlockFetcherStrategy::BlockReceipts);
        assert_eq!(config.batch_receipts_size_range, 10);
        assert!(config.automatic_startup);
    }

    #[test]
    fn test_block_start_config_from_u64() {
        let value: BlockStartConfig = serde_json::from_str("12345").unwrap();
        assert_eq!(value, BlockStartConfig::Number(12345));
    }

    #[test]
    fn test_block_start_config_from_string_current() {
        let value: BlockStartConfig = serde_json::from_str("\"current\"").unwrap();
        assert_eq!(value, BlockStartConfig::Current);
    }

    #[test]
    fn test_block_start_config_from_string_current_case_insensitive() {
        let value: BlockStartConfig = serde_json::from_str("\"Current\"").unwrap();
        assert_eq!(value, BlockStartConfig::Current);
        let value: BlockStartConfig = serde_json::from_str("\"CURRENT\"").unwrap();
        assert_eq!(value, BlockStartConfig::Current);
    }

    #[test]
    fn test_block_start_config_from_numeric_string() {
        // Env vars arrive as strings via the config crate
        let value: BlockStartConfig = serde_json::from_str("\"99999\"").unwrap();
        assert_eq!(value, BlockStartConfig::Number(99999));
    }

    #[test]
    fn test_block_start_config_invalid_string_rejected() {
        let result = serde_json::from_str::<BlockStartConfig>("\"foo\"");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("invalid block_start_on_first_start"));
        assert!(err.contains("foo"));
    }

    #[test]
    fn test_block_start_config_display() {
        assert_eq!(BlockStartConfig::Current.to_string(), "current");
        assert_eq!(BlockStartConfig::Number(42).to_string(), "42");
    }

    #[test]
    fn test_blocks_to_keep_minimum_validation() {
        let config = BlockchainConfig {
            r#type: "evm".to_string(),
            chain_id: 1,
            rpc_url: "https://rpc.example.com".to_string(),
            network: "test".to_string(),
            finality_depth: 0,
            cleaner: CleanerConfig {
                blocks_to_keep: 1,
                ..Default::default()
            },
            strategy: StrategyConfig::default(),
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be >= 2"));
    }

    #[test]
    fn test_publish_config_defaults() {
        let config = PublishConfig::default();
        assert_eq!(config.publish_retry_secs, 1);
        assert!(config.publish_stale);
        assert_eq!(config.publish_no_stale_retries, 5);
    }

    #[test]
    fn test_strategy_config_with_publish_defaults() {
        let config = StrategyConfig::default();
        assert_eq!(config.publish.publish_retry_secs, 1);
        assert!(config.publish.publish_stale);
        assert_eq!(config.publish.publish_no_stale_retries, 5);
    }

    #[test]
    fn test_strategy_validation_batch_size_too_large() {
        let config = StrategyConfig {
            range_size: 20000,
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("range size"));
    }

    #[test]
    fn test_strategy_validation_parallel_requests_too_large() {
        let config = StrategyConfig {
            max_parallel_requests: 500,
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("max_parallel_requests")
        );
    }

    #[test]
    fn zero_chain_id() {
        let config = BlockchainConfig {
            network: "fake-chain".to_string(),
            chain_id: 0,
            r#type: default_blockchain_type(),
            rpc_url: "https://ethereum-rpc.publicnode.com".to_string(), // REQUIRED, REDACTED
            finality_depth: default_finality_depth(),
            cleaner: CleanerConfig::default(),
            strategy: StrategyConfig::default(),
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("strictly positive")
        );
    }

    #[test]
    fn test_database_url_validation() {
        let config = DatabaseConfig {
            db_url: "mysql://invalid".to_string(),
            iam_auth: None,
            migration_max_attempts: 5,
            pool: PoolConfig::default(),
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("postgres://"));
    }

    #[test]
    fn test_broker_amqp_url_validation() {
        let config = BrokerConfig {
            broker_type: BrokerType::Amqp,
            broker_url: "http://invalid".to_string(),
            ensure_publish: false,
            circuit_breaker_cooldown_secs: 10,
            circuit_breaker_threshold: 3,
            claim_min_idle: 30,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("amqp://"));
    }

    #[test]
    fn test_broker_redis_url_validation() {
        let config = BrokerConfig {
            broker_type: BrokerType::Redis,
            broker_url: "http://invalid".to_string(),
            ensure_publish: false,
            circuit_breaker_cooldown_secs: 10,
            circuit_breaker_threshold: 3,
            claim_min_idle: 30,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("redis://"));
    }

    #[test]
    fn test_broker_type_default_is_redis() {
        assert_eq!(BrokerType::default(), BrokerType::Redis);
    }

    #[test]
    fn test_broker_url_method() {
        let amqp_config = BrokerConfig {
            broker_type: BrokerType::Amqp,
            broker_url: "amqp://localhost:5672".to_string(),
            ensure_publish: false,
            circuit_breaker_cooldown_secs: 10,
            circuit_breaker_threshold: 3,
            claim_min_idle: 30,
        };
        assert_eq!(amqp_config.url(), "amqp://localhost:5672");

        let redis_config = BrokerConfig {
            broker_type: BrokerType::Redis,
            broker_url: "redis://localhost:6379".to_string(),
            ensure_publish: false,
            circuit_breaker_cooldown_secs: 10,
            circuit_breaker_threshold: 3,
            claim_min_idle: 30,
        };
        assert_eq!(redis_config.url(), "redis://localhost:6379");
    }

    #[test]
    fn test_broker_valid_amqp_config() {
        let config = BrokerConfig {
            broker_type: BrokerType::Amqp,
            broker_url: "amqp://localhost:5672".to_string(),
            ensure_publish: false,
            circuit_breaker_cooldown_secs: 10,
            circuit_breaker_threshold: 3,
            claim_min_idle: 30,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_broker_valid_redis_config() {
        let config = BrokerConfig {
            broker_type: BrokerType::Redis,
            broker_url: "redis://localhost:6379".to_string(),
            ensure_publish: false,
            circuit_breaker_cooldown_secs: 10,
            circuit_breaker_threshold: 3,
            claim_min_idle: 30,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_blockchain_rpc_url_validation() {
        let config = BlockchainConfig {
            r#type: "evm".to_string(),
            chain_id: 1,
            rpc_url: "ws://invalid".to_string(),
            network: "ethereum-mainnet".to_string(),
            cleaner: CleanerConfig::default(),
            strategy: StrategyConfig::default(),
            finality_depth: 15,
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("http://"));
    }

    #[test]
    fn test_secrets_redacted_in_debug() {
        let db_config = DatabaseConfig {
            db_url: "postgres://user:secret_password@localhost/db".to_string(),
            iam_auth: None,
            migration_max_attempts: 5,
            pool: PoolConfig::default(),
        };
        let debug_output = format!("{:?}", db_config);
        assert!(!debug_output.contains("secret_password"));
        assert!(debug_output.contains("[REDACTED]"));

        let broker_config = BrokerConfig {
            broker_type: BrokerType::Amqp,
            broker_url: "amqp://user:secret_password@localhost/vhost".to_string(),
            ensure_publish: false,
            circuit_breaker_cooldown_secs: 10,
            circuit_breaker_threshold: 3,
            claim_min_idle: 30,
        };
        let debug_output = format!("{:?}", broker_config);
        assert!(!debug_output.contains("secret_password"));
        assert!(debug_output.contains("[REDACTED]"));

        let blockchain_config = BlockchainConfig {
            r#type: "evm".to_string(),
            chain_id: 1,
            rpc_url: "https://secret-api-key@rpc.example.com".to_string(),
            network: "ethereum-mainnet".to_string(),
            cleaner: CleanerConfig::default(),
            strategy: StrategyConfig::default(),
            finality_depth: 15,
        };
        let debug_output = format!("{:?}", blockchain_config);
        assert!(!debug_output.contains("secret-api-key"));
        assert!(debug_output.contains("[REDACTED]"));
    }

    #[test]
    fn test_pool_default_passes_validation() {
        assert!(PoolConfig::default().validate().is_ok());
    }

    #[test]
    fn test_pool_min_connections_too_low() {
        let config = PoolConfig {
            min_connections: 1,
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("min_connections"));
    }

    #[test]
    fn test_pool_max_connections_too_low() {
        let config = PoolConfig {
            max_connections: 3,
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("max_connections"));
    }

    #[test]
    fn test_pool_max_less_than_min() {
        let config = PoolConfig {
            min_connections: 6,
            max_connections: 5,
            ..Default::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("max_connections"));
    }
}
