use crate::http::endpoints::v2::types::keyurl::KeyData;
use crate::http::utils::redact::redact;
use config::{Config, Environment, File};
use derivative::Derivative;
use serde::Deserializer;
use serde::{de::Error, Deserialize, Serialize};
use std::collections::HashSet;
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

/// Generic deserializer to handle both standard YAML arrays and
/// Env Variable indexed maps (e.g., field__0__key, field__1__key).
/// Works for any Vec<T> where T: Deserialize — used for listeners,
/// host_chains, backoff_intervals, histogram buckets, etc.
pub(crate) fn deserialize_vec_from_map_or_seq<'de, D, T>(
    deserializer: D,
) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    // A manual Visitor (visit_seq / visit_map via deserialize_any) rather than a
    // `#[serde(untagged)]` enum. Untagged enums buffer the value into serde's `Content`
    // before trying each variant, and that buffering cannot round-trip a `u64` above
    // `i64::MAX` (e.g. an RFC-021 Solana host `chain_id`) through the `config` crate — the
    // whole list then fails with "data did not match any variant of untagged enum". The
    // Visitor lets each element deserialize directly from the config value, so the
    // element's own `deserialize_u64_from_str_or_num` handles large ids correctly.
    struct VecVisitor<T>(std::marker::PhantomData<T>);

    impl<'de, T> serde::de::Visitor<'de> for VecVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a sequence, or a map keyed by integer index")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(item) = seq.next_element::<T>()? {
                out.push(item);
            }
            Ok(out)
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            // Keys are stringified indices ("0", "1", …); order by index, ignore non-numeric keys.
            let mut items: Vec<(usize, T)> = Vec::new();
            while let Some((key, value)) = map.next_entry::<String, T>()? {
                if let Ok(idx) = key.parse::<usize>() {
                    items.push((idx, value));
                }
            }
            items.sort_by_key(|(idx, _)| *idx);
            Ok(items.into_iter().map(|(_, v)| v).collect())
        }
    }

    deserializer.deserialize_any(VecVisitor(std::marker::PhantomData))
}

/// Unified listener pool configuration
/// Supports multiple listener types (WebSocket subscriptions and HTTP polling)
/// with shared deduplication and staggered connection recycling
#[derive(Debug, Deserialize, Clone)]
pub struct ListenerPoolConfig {
    /// Optional starting block number, overriding the stored cursor. Polling listeners only:
    /// a WebSocket subscription cannot start from a past block.
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
    /// Widest block span a single `eth_getLogs` may ask for (polling listeners only).
    /// Catching up from far behind the head - after downtime, or after `last_block_number`
    /// is rewound to force a replay - is chunked to this many blocks per query, so it never
    /// asks for a range the provider rejects nor gets back one outsized response.
    pub max_blocks_per_query: u64,
    /// How long the event registry remembers an event, in seconds (1-10).
    ///
    /// It bounds two things: how long the same log observed by two listener instances is
    /// recognized as one event, and how long an entry survives while its handlers run. The
    /// window restarts when the handlers finish.
    pub dedup_ttl_seconds: u64,
    /// Maximum number of events the registry tracks at once.
    ///
    /// **Formula:** `events_per_second * num_listeners * dedup_ttl_seconds * safety_buffer`
    ///
    /// **Recommended values (with 3 listeners, 5s TTL, 1.2x buffer):**
    /// - 100 events/sec -> 1,800
    /// - 1000 events/sec -> 18,000
    /// - 5000 events/sec -> 90,000
    pub dedup_max_capacity: usize,
    /// List of listeners in the pool
    /// Each listener has a type and URL; instance_id is assigned by position (0-indexed)
    #[serde(deserialize_with = "deserialize_vec_from_map_or_seq")]
    pub listeners: Vec<ListenerInstanceConfig>,
}

/// Signer configuration — explicitly tagged by type.
/// Invalid combinations are unrepresentable.
#[derive(Clone, Deserialize, Derivative)]
#[derivative(Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SignerConfig {
    /// Local private key signer (for local dev and CI)
    PrivateKey {
        #[derivative(Debug(format_with = "redact"))]
        private_key: String,
    },
    /// AWS KMS signer (for deployment)
    AwsKms {
        #[derivative(Debug(format_with = "redact"))]
        key_id: String,
        region: String,
        #[derivative(Debug(format_with = "redact"))]
        endpoint: Option<String>,
    },
}

#[derive(Deserialize, Clone, Derivative)]
#[derivative(Debug)]
pub struct TxEngineConfig {
    pub signer: SignerConfig,
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
    pub host_acl_check: HostAclCheckConfig,
    pub gw_ciphertext_check: GwCiphertextCheckConfig,
    pub public_decrypt: PublicDecryptQueueSettings,
    pub user_decrypt: UserDecryptQueueSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HostAclCheckConfig {
    pub retry: RetrySettings,
    /// Deadline for one Solana host-chain read attempt. The EVM path is dialed through alloy
    /// and is not covered by this. Defaulted so an existing deployment's config keeps parsing:
    /// a read that hangs forever holds its readiness permit forever, so no deployment should be
    /// left without a deadline just because its file predates this field.
    #[serde(default = "default_host_acl_request_timeout_ms")]
    pub request_timeout_ms: u64,
}

/// Ten seconds: a batched `getMultipleAccounts` at `confirmed` answers in well under a second,
/// so this bounds a stalled read without cutting a merely slow one short.
fn default_host_acl_request_timeout_ms() -> u64 {
    10_000
}

#[derive(Debug, Deserialize, Clone)]
pub struct GwCiphertextCheckConfig {
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
    #[serde(deserialize_with = "deserialize_vec_from_map_or_seq")]
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
    /// Default retry-after seconds for queued API responses
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
    #[serde(deserialize_with = "deserialize_vec_from_map_or_seq")]
    pub query_duration_histogram_bucket: Vec<f64>,
    #[serde(deserialize_with = "deserialize_vec_from_map_or_seq")]
    pub request_status_duration_histogram_bucket: Vec<f64>,
    #[serde(deserialize_with = "deserialize_vec_from_map_or_seq")]
    pub transaction_duration_secs_histogram_bucket: Vec<f64>,
    /// Histogram buckets for raw ETA (before clamping) in retry-after computation.
    /// Higher resolution at small values for typical requests, exponential for full queue.
    /// Example: [1, 2, 5, 10, 20, 30, 60, 120, 300, 600, 1200, 2400]
    #[serde(deserialize_with = "deserialize_vec_from_map_or_seq")]
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
    /// Whether the expiry (retention) worker is enabled. Defaults to false.
    #[serde(default)]
    pub expiry_enabled: bool,
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

        if self.expiry_enabled {
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
    pub user_decrypt_shares_threshold: u32,
}

/// User-decryption signature check configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct UserDecryptSignatureCheckConfig {
    pub erc1271_gas_limit: u64,
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

#[derive(Debug, Deserialize, Clone)]
pub struct HostChainConfig {
    /// Host chain id. RFC-021 Solana host ids carry the chain-type high bit and exceed
    /// `i64::MAX`, which serde_yaml's untagged buffering (used by `deserialize_vec_from_map_or_seq`
    /// for `host_chains`) cannot represent as a bare number; accept a quoted string too so the
    /// Solana host id can be configured while EVM ids stay plain numbers.
    #[serde(deserialize_with = "deserialize_u64_from_str_or_num")]
    pub chain_id: u64,
    pub url: String,
    pub acl_address: String,
}

/// Deserializes a `u64` from either a YAML number or a string (see `HostChainConfig::chain_id`).
fn deserialize_u64_from_str_or_num<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct U64StrOrNum;
    impl serde::de::Visitor<'_> for U64StrOrNum {
        type Value = u64;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a u64 as a number or a decimal string")
        }
        fn visit_u64<E>(self, v: u64) -> Result<u64, E> {
            Ok(v)
        }
        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<u64, E> {
            u64::try_from(v).map_err(serde::de::Error::custom)
        }
        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<u64, E> {
            v.parse::<u64>().map_err(serde::de::Error::custom)
        }
    }
    deserializer.deserialize_any(U64StrOrNum)
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProtocolConfigSettings {
    pub ethereum_http_rpc_url: String,
    pub address: String,
    pub retry: RetrySettings,
}

/// `/v2/keyurl` settings, tagged by `source`. Required with no default, so a deployment
/// cannot silently pick the wrong source.
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "source", rename_all = "lowercase")]
pub enum KeyUrlConfig {
    /// Read the active key/CRS/KMS context from the Ethereum host chain (KMSGeneration +
    /// ProtocolConfig contracts) and keep `/v2/keyurl` in sync by polling.
    Chain {
        /// KMSGeneration contract address on the Ethereum host chain.
        kms_generation_address: String,
        /// How often the poller reads the active key/CRS/KMS context from the host chain.
        poll_interval_ms: u64,
    },
    /// Serve `/v2/keyurl` from static configuration; no host-chain poller runs. Required
    /// against protocol deployments that do not implement
    /// `ProtocolConfig.getCurrentKmsContextAndEpoch` (protocol v0.13 and earlier).
    Config {
        /// Served as `response.fheKeyInfo[0].fhePublicKey`.
        fhe_public_key: KeyData,
        /// Served as `response.crs["2048"]`.
        crs: KeyData,
    },
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
    /// HTTP server configuration
    pub http: HttpConfig,
    /// Metrics server configuration
    pub metrics: MetricsConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Host chain configurations (required, at least one entry)
    #[serde(deserialize_with = "deserialize_vec_from_map_or_seq")]
    pub host_chains: Vec<HostChainConfig>,
    /// ProtocolConfig contract settings for dynamic threshold resolution
    pub protocol_config: ProtocolConfigSettings,
    /// `/v2/keyurl` settings: host-chain poller or static config
    pub keyurl: KeyUrlConfig,
    /// User-decryption signature check configuration
    pub user_decrypt_signature_check: UserDecryptSignatureCheckConfig,
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
                .prefix_separator("_") // Separator between APP and the rest
                // Required for structs with numeric fields (e.g., host_chains.chain_id: u64)
                // inside Vec fields using deserialize_vec_from_map_or_seq. Without this,
                // env vars remain strings and fail to deserialize into numeric types.
                .try_parsing(true),
        );

        let settings: Settings = s
            .build()
            .map_err(|err| AppConfigError::Config(err.to_string()))?
            .try_deserialize()
            .map_err(|err| AppConfigError::Config(err.to_string()))?;

        // Validate network configurations
        settings.gateway.validate()?;

        // Validate host chains configuration
        settings.validate_host_chains()?;

        // Validate cron startup delay (10% rule)
        settings.storage.cron.validate()?;

        // Ensure HTTP metrics configuration is provided
        if settings.http.metrics.histogram_buckets.is_empty() {
            return Err(AppConfigError::Config(
                "HTTP metrics histogram buckets must be set in the configuration file.".into(),
            ));
        }

        // Validate contract addresses
        settings.validate_addresses()?;

        // Validate protocol_config settings
        settings.validate_protocol_config()?;

        // Validate keyurl poller settings
        settings.validate_keyurl()?;

        // Validate listener pool configuration
        settings.validate_listener_pool_config()?;

        Ok(settings)
    }

    fn validate_addresses(&self) -> Result<(), AppConfigError> {
        use alloy::primitives::Address;
        use std::str::FromStr;

        let mut addresses = vec![
            ("decryption", &self.gateway.contracts.decryption_address),
            (
                "input_verification",
                &self.gateway.contracts.input_verification_address,
            ),
            ("protocol_config", &self.protocol_config.address),
        ];
        if let KeyUrlConfig::Chain {
            kms_generation_address,
            ..
        } = &self.keyurl
        {
            addresses.push(("keyurl.kms_generation_address", kms_generation_address));
        }

        for (name, address) in addresses {
            if Address::from_str(address).is_err() {
                return Err(AppConfigError::InvalidAddress(format!(
                    "Invalid {name} address: {address}"
                )));
            }
        }

        Ok(())
    }

    pub fn validate_host_chains(&self) -> Result<(), AppConfigError> {
        use alloy::primitives::Address;
        use std::str::FromStr;

        if self.host_chains.is_empty() {
            return Err(AppConfigError::Config(
                "host_chains must have at least one entry".to_string(),
            ));
        }

        let mut seen_chain_ids = HashSet::new();
        for (i, hc) in self.host_chains.iter().enumerate() {
            if !seen_chain_ids.insert(hc.chain_id) {
                return Err(AppConfigError::Config(format!(
                    "host_chains contains duplicate chain_id: {}",
                    hc.chain_id
                )));
            }
            if !hc.url.starts_with("http://") && !hc.url.starts_with("https://") {
                return Err(AppConfigError::InvalidNetworkConfig(format!(
                    "host_chains[{}].url must start with http:// or https://: {}",
                    i, hc.url
                )));
            }
            // The chain-id discriminator and ACL address encoding must agree so
            // downstream components cannot classify the same host differently.
            let valid_acl_address = if crate::core::event::is_solana_host_chain_id(hc.chain_id) {
                crate::http::utils::solana_address::is_solana_address(&hc.acl_address)
            } else {
                Address::from_str(&hc.acl_address).is_ok()
            };
            if !valid_acl_address {
                return Err(AppConfigError::InvalidAddress(format!(
                    "host_chains[{}].acl_address does not match chain_id {}: {}",
                    i, hc.chain_id, hc.acl_address
                )));
            }
        }

        Ok(())
    }

    fn validate_protocol_config(&self) -> Result<(), AppConfigError> {
        let pc = &self.protocol_config;
        if !pc.ethereum_http_rpc_url.starts_with("http://")
            && !pc.ethereum_http_rpc_url.starts_with("https://")
        {
            return Err(AppConfigError::InvalidNetworkConfig(format!(
                "protocol_config.ethereum_http_rpc_url must start with http:// or https://: {}",
                pc.ethereum_http_rpc_url
            )));
        }
        if pc.retry.max_attempts < 1 {
            return Err(AppConfigError::Config(
                "protocol_config.retry.max_attempts must be at least 1".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_keyurl(&self) -> Result<(), AppConfigError> {
        match &self.keyurl {
            KeyUrlConfig::Chain {
                poll_interval_ms, ..
            } => {
                if *poll_interval_ms < 1 {
                    return Err(AppConfigError::Config(
                        "keyurl.poll_interval_ms must be at least 1".to_string(),
                    ));
                }
            }
            KeyUrlConfig::Config {
                fhe_public_key,
                crs,
            } => {
                // Statically configured values are served verbatim to every SDK client, so a
                // placeholder or typo has to fail here rather than at the client.
                validate_keyurl_key_data("keyurl.fhe_public_key", fhe_public_key)?;
                validate_keyurl_key_data("keyurl.crs", crs)?;
            }
        }
        Ok(())
    }

    fn validate_listener_pool_config(&self) -> Result<(), AppConfigError> {
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

        // A zero-wide chunk never reaches the head, so the catch-up loop would poll forever
        // without advancing the cursor.
        if pool_config.max_blocks_per_query == 0 {
            return Err(AppConfigError::Config(
                "listener_pool.max_blocks_per_query must be at least 1".to_string(),
            ));
        }

        // Validate dedup max capacity (should be reasonable)
        if pool_config.dedup_max_capacity < 1000 || pool_config.dedup_max_capacity > 10_000_000 {
            return Err(AppConfigError::Config(format!(
                "dedup_max_capacity must be between 1000 and 10,000,000, got: {}",
                pool_config.dedup_max_capacity
            )));
        }

        // Only the polling listener replays blocks missed while the relayer was not
        // listening; `eth_subscribe` takes no fromBlock, so a WebSocket listener that starts
        // late sees only new logs.
        if !pool_config
            .listeners
            .iter()
            .any(|listener| listener.listener_type == ListenerType::Polling)
        {
            tracing::warn!(
                "listener_pool has no polling listener: events emitted while the relayer is \
                 not listening will be missed, since a WebSocket subscription does not \
                 replay them"
            );
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

/// Hex characters in a `data_id` after `0x` — the 32-byte id form `chain` mode serves.
const KEYURL_DATA_ID_HEX_LEN: usize = 64;

/// Validate one statically configured `/v2/keyurl` entry.
///
/// `key` is the full config key (`keyurl.fhe_public_key` / `keyurl.crs`) so an operator reading
/// a failed startup can tell which of the two is wrong and fix it from the message alone.
fn validate_keyurl_key_data(key: &str, key_data: &KeyData) -> Result<(), AppConfigError> {
    // No `0x` prefix yields "", which then fails the length check.
    let digits = key_data.data_id.strip_prefix("0x").unwrap_or("");
    if digits.len() != KEYURL_DATA_ID_HEX_LEN || !digits.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(AppConfigError::Config(format!(
            "{key}.data_id must be a 0x-prefixed hex string of {KEYURL_DATA_ID_HEX_LEN} digits, got: {}",
            key_data.data_id
        )));
    }

    if key_data.urls.is_empty() {
        return Err(AppConfigError::Config(format!(
            "{key}.urls must contain at least one URL"
        )));
    }
    for (i, url) in key_data.urls.iter().enumerate() {
        if let Err(e) = reqwest::Url::parse(url) {
            return Err(AppConfigError::InvalidNetworkConfig(format!(
                "{key}.urls[{i}] is not a valid URL ({e}): {url}"
            )));
        }
    }

    Ok(())
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
    use serial_test::serial;
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
        let tx_engine_config = TxEngineConfig {
            signer: SignerConfig::PrivateKey {
                private_key: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                    .to_string(),
            },
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

        let debug_output = format!("{:?}", tx_engine_config);

        assert!(
            !debug_output.contains("1234567890abcdef"),
            "Private key should not appear in debug output. Got: {}",
            debug_output
        );

        assert!(
            debug_output.contains("private_key: [REDACTED]"),
            "Debug output should contain 'private_key: [REDACTED]' but got: {}",
            debug_output
        );
    }

    #[test]
    fn test_aws_kms_fields_are_redacted_in_debug_output() {
        let tx_engine_config = TxEngineConfig {
            signer: SignerConfig::AwsKms {
                key_id: "arn:aws:kms:us-east-1:123456789012:key/secret-key-id".to_string(),
                region: "us-east-1".to_string(),
                endpoint: Some("http://localhost:4566".to_string()),
            },
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

        let debug_output = format!("{:?}", tx_engine_config);

        assert!(
            !debug_output.contains("secret-key-id"),
            "AWS KMS key_id should not appear in debug output. Got: {}",
            debug_output
        );
        assert!(
            !debug_output.contains("localhost:4566"),
            "AWS KMS endpoint should not appear in debug output. Got: {}",
            debug_output
        );
        assert!(
            debug_output.contains("key_id: [REDACTED]"),
            "Debug output should contain 'key_id: [REDACTED]' but got: {}",
            debug_output
        );
        assert!(
            debug_output.contains("endpoint: [REDACTED]"),
            "Debug output should contain 'endpoint: [REDACTED]' but got: {}",
            debug_output
        );
    }

    #[test]
    fn test_signer_config_deserialize_private_key() {
        let yaml = r#"
            type: "private_key"
            private_key: "0xabc123"
        "#;
        let config: SignerConfig = serde_yaml::from_str(yaml).unwrap();
        match config {
            SignerConfig::PrivateKey { private_key } => {
                assert_eq!(private_key, "0xabc123");
            }
            _ => panic!("Expected PrivateKey variant"),
        }
    }

    #[test]
    fn test_signer_config_deserialize_aws_kms() {
        let yaml = r#"
            type: "aws_kms"
            key_id: "arn:aws:kms:us-east-1:123456789012:key/abc"
            region: "us-east-1"
            endpoint: "http://localhost:4566"
        "#;
        let config: SignerConfig = serde_yaml::from_str(yaml).unwrap();
        match config {
            SignerConfig::AwsKms {
                key_id,
                region,
                endpoint,
            } => {
                assert_eq!(key_id, "arn:aws:kms:us-east-1:123456789012:key/abc");
                assert_eq!(region, "us-east-1");
                assert_eq!(endpoint.unwrap(), "http://localhost:4566");
            }
            _ => panic!("Expected AwsKms variant"),
        }
    }

    #[test]
    fn test_signer_config_deserialize_aws_kms_without_endpoint() {
        let yaml = r#"
            type: "aws_kms"
            key_id: "arn:aws:kms:us-east-1:123456789012:key/abc"
            region: "us-east-1"
        "#;
        let config: SignerConfig = serde_yaml::from_str(yaml).unwrap();
        match config {
            SignerConfig::AwsKms { endpoint, .. } => {
                assert!(endpoint.is_none());
            }
            _ => panic!("Expected AwsKms variant"),
        }
    }

    #[test]
    fn test_signer_config_deserialize_invalid_type() {
        let yaml = r#"
            type: "invalid"
            key: "value"
        "#;
        let result: Result<SignerConfig, _> = serde_yaml::from_str(yaml);
        assert!(
            result.is_err(),
            "Invalid signer type should fail deserialization"
        );
    }

    #[test]
    fn test_signer_config_deserialize_missing_type() {
        let yaml = r#"
            private_key: "0xabc123"
        "#;
        let result: Result<SignerConfig, _> = serde_yaml::from_str(yaml);
        assert!(
            result.is_err(),
            "Missing type field should fail deserialization"
        );
    }

    /// Deserialize `Settings` from a builder that is expected to be invalid, returning the
    /// error message. Deserialization alone is enough: `keyurl.source` is enforced by serde.
    fn settings_load_error(builder: ConfigBuilder) -> String {
        let config_path = builder
            .to_temp_file()
            .expect("Failed to create temp config file");
        let config = Config::builder()
            .add_source(File::from(config_path.as_path()).format(FileFormat::Yaml))
            .build()
            .expect("Failed to build config");
        let result: Result<Settings, _> = config.try_deserialize();
        result
            .expect_err("Configuration parsing should have failed")
            .to_string()
    }

    /// A `keyurl` block for `source: config`, minus the top-level fields named in `omit`.
    fn static_keyurl(omit: &[&str]) -> serde_yaml::Value {
        let mut value: serde_yaml::Value = serde_yaml::from_str(
            r#"
            source: config
            fhe_public_key:
              data_id: "0x0400000000000000000000000000000000000000000000000000000000000003"
              urls: ["http://minio:9000/kms-public/PUB/PublicKey/03"]
            crs:
              data_id: "0x0400000000000000000000000000000000000000000000000000000000000004"
              urls: ["http://minio:9000/kms-public/PUB/CRS/04"]
            "#,
        )
        .expect("Failed to parse static keyurl block");
        let mapping = value.as_mapping_mut().expect("keyurl block is a mapping");
        for field in omit {
            mapping.remove(*field);
        }
        value
    }

    /// Run the whole `Settings::new` path — deserialize *and* the `validate_*` pass — on a
    /// modified example config. Uses a `.yaml` path: the format is inferred from the file name.
    fn settings_new(builder: ConfigBuilder) -> Result<Settings, AppConfigError> {
        let content = serde_yaml::to_string(&builder.config).expect("Failed to serialize");
        let dir = tempfile::tempdir().expect("Failed to create temp dir");
        let config_path = dir.path().join("validate-keyurl.yaml");
        std::fs::write(&config_path, content).expect("Failed to write config file");
        Settings::new(Some(config_path.to_string_lossy().to_string()))
    }

    /// A `keyurl` builder seeded with the valid static block, for tests that then break one field.
    fn config_keyurl_builder() -> ConfigBuilder {
        ConfigBuilder::from_example()
            .expect("Failed to load example config")
            .set_field("keyurl", static_keyurl(&[]))
    }

    #[test]
    fn test_keyurl_source_is_required() {
        let err = settings_load_error(
            ConfigBuilder::from_example()
                .expect("Failed to load example config")
                .remove_field("keyurl.source"),
        );
        assert!(
            err.contains("missing configuration field \"keyurl.source\""),
            "Error should name the missing `source` field, got: {err}"
        );
    }

    #[test]
    fn test_keyurl_source_rejects_unknown_value() {
        let err = settings_load_error(
            ConfigBuilder::from_example()
                .expect("Failed to load example config")
                .set_field("keyurl.source", serde_yaml::Value::String("bogus".into())),
        );
        assert!(
            err.contains("unknown variant `bogus`")
                && err.contains("`chain`")
                && err.contains("`config`"),
            "Error should reject `bogus` and name the valid variants, got: {err}"
        );
    }

    #[test]
    fn test_keyurl_chain_requires_kms_generation_address() {
        let err = settings_load_error(
            ConfigBuilder::from_example()
                .expect("Failed to load example config")
                .remove_field("keyurl.kms_generation_address"),
        );
        assert!(
            err.contains("missing configuration field \"keyurl.kms_generation_address\""),
            "Error should name the missing `kms_generation_address` field, got: {err}"
        );
    }

    #[test]
    fn test_keyurl_chain_requires_poll_interval_ms() {
        let err = settings_load_error(
            ConfigBuilder::from_example()
                .expect("Failed to load example config")
                .remove_field("keyurl.poll_interval_ms"),
        );
        assert!(
            err.contains("missing configuration field \"keyurl.poll_interval_ms\""),
            "Error should name the missing `poll_interval_ms` field, got: {err}"
        );
    }

    #[test]
    fn test_keyurl_config_requires_fhe_public_key() {
        let err = settings_load_error(
            ConfigBuilder::from_example()
                .expect("Failed to load example config")
                .set_field("keyurl", static_keyurl(&["fhe_public_key"])),
        );
        assert!(
            err.contains("missing configuration field \"keyurl.fhe_public_key\""),
            "Error should name the missing `fhe_public_key` field, got: {err}"
        );
    }

    #[test]
    fn test_keyurl_config_requires_crs() {
        let err = settings_load_error(
            ConfigBuilder::from_example()
                .expect("Failed to load example config")
                .set_field("keyurl", static_keyurl(&["crs"])),
        );
        assert!(
            err.contains("missing configuration field \"keyurl.crs\""),
            "Error should name the missing `crs` field, got: {err}"
        );
    }

    #[test]
    #[serial] // Settings::new reads the environment
    fn test_keyurl_config_accepts_valid_static_values() {
        let settings = settings_new(config_keyurl_builder())
            .expect("A valid `keyurl.source: config` block should deserialize and validate");

        match &settings.keyurl {
            KeyUrlConfig::Config {
                fhe_public_key,
                crs,
            } => {
                assert_eq!(
                    fhe_public_key.data_id,
                    "0x0400000000000000000000000000000000000000000000000000000000000003"
                );
                assert_eq!(
                    fhe_public_key.urls,
                    vec!["http://minio:9000/kms-public/PUB/PublicKey/03"]
                );
                assert_eq!(
                    crs.data_id,
                    "0x0400000000000000000000000000000000000000000000000000000000000004"
                );
                assert_eq!(crs.urls, vec!["http://minio:9000/kms-public/PUB/CRS/04"]);
            }
            other => panic!("expected keyurl.source: config, got {other:?}"),
        }
    }

    #[test]
    #[serial]
    fn test_keyurl_config_rejects_empty_urls() {
        let err = settings_new(config_keyurl_builder().set_field(
            "keyurl.fhe_public_key.urls",
            serde_yaml::to_value(Vec::<&str>::new()).unwrap(),
        ))
        .expect_err("Empty `urls` should fail validation")
        .to_string();
        assert!(
            err.contains("keyurl.fhe_public_key.urls") && err.contains("at least one URL"),
            "Error should name the offending key and the expected form, got: {err}"
        );
    }

    #[test]
    #[serial]
    fn test_keyurl_config_rejects_unparsable_url() {
        let err = settings_new(config_keyurl_builder().set_field(
            "keyurl.crs.urls",
            serde_yaml::to_value(["not a url"]).unwrap(),
        ))
        .expect_err("An unparsable `urls` entry should fail validation")
        .to_string();
        assert!(
            err.contains("keyurl.crs.urls[0]") && err.contains("not a valid URL"),
            "Error should name the offending key and index, got: {err}"
        );
    }

    /// The placeholder case: mainnet gitops values carry a literal `fhe-public-key-data-id`,
    /// which would otherwise boot green and be served to every SDK client.
    #[test]
    #[serial]
    fn test_keyurl_config_rejects_non_hex_data_id() {
        let err = settings_new(config_keyurl_builder().set_field(
            "keyurl.crs.data_id",
            serde_yaml::Value::String("fhe-public-key-data-id".into()),
        ))
        .expect_err("A non-0x `data_id` should fail validation")
        .to_string();
        assert!(
            err.contains("keyurl.crs.data_id") && err.contains("hex string of 64 digits"),
            "Error should name the offending key and the expected form, got: {err}"
        );
    }

    #[test]
    #[serial]
    fn test_keyurl_config_rejects_data_id_of_wrong_length() {
        let err = settings_new(config_keyurl_builder().set_field(
            "keyurl.fhe_public_key.data_id",
            serde_yaml::Value::String("0x0400000000000000000000000000000003".into()),
        ))
        .expect_err("A `0x` `data_id` of the wrong length should fail validation")
        .to_string();
        assert!(
            err.contains("keyurl.fhe_public_key.data_id")
                && err.contains("hex string of 64 digits"),
            "Error should name the offending key and the expected form, got: {err}"
        );
    }

    /// `Chain` mode must be untouched by the `Config`-mode content rules.
    #[test]
    #[serial]
    fn test_keyurl_chain_mode_unaffected_by_config_validation() {
        let settings = settings_new(ConfigBuilder::from_example().expect("example config"))
            .expect("The unmodified example (`source: chain`) should still pass validation");
        assert!(matches!(settings.keyurl, KeyUrlConfig::Chain { .. }));
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
                expiry_enabled: false,
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

    #[test]
    #[serial] // avoid env var leakage from parallel tests
    fn test_settings_new_rejects_invalid_address() {
        let result = settings_new(
            ConfigBuilder::from_example()
                .expect("Failed to load example config")
                .set_field(
                    "gateway.contracts.decryption_address",
                    serde_yaml::Value::String("not-a-valid-address".into()),
                ),
        );

        assert!(
            result.is_err(),
            "Settings::new() should fail with an invalid address"
        );
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("Invalid") && err.contains("address"),
            "Error should mention invalid address, got: {err}"
        );
    }

    #[test]
    fn test_deserialize_host_chains_from_indexed_env_vars() {
        let config_path = ConfigBuilder::from_example()
            .expect("Failed to load example config")
            .to_temp_file()
            .expect("Failed to create temp config file");

        // Simulate env vars: APP_HOST_CHAINS__0__*, APP_HOST_CHAINS__1__*
        let config = Config::builder()
            .add_source(File::from(config_path.as_path()).format(FileFormat::Yaml))
            .set_override("host_chains.0.chain_id", 8009_i64)
            .expect("Failed to set override")
            .set_override("host_chains.0.url", "http://localhost:8545")
            .expect("Failed to set override")
            .set_override(
                "host_chains.0.acl_address",
                "0x339EBB773A9bC1deCFfD5ef4BC7c907e26C1f836",
            )
            .expect("Failed to set override")
            .set_override("host_chains.1.chain_id", 10901_i64)
            .expect("Failed to set override")
            .set_override("host_chains.1.url", "http://localhost:9545")
            .expect("Failed to set override")
            .set_override(
                "host_chains.1.acl_address",
                "0xE61cff9C581c7c91AEF682c2C10e8632864339ab",
            )
            .expect("Failed to set override")
            .build()
            .expect("Failed to build config");

        let settings: Settings = config
            .try_deserialize()
            .expect("Failed to deserialize settings with indexed host_chains");
        let host_chains = &settings.host_chains;

        assert_eq!(
            host_chains.len(),
            2,
            "Should have 2 host chains from indexed overrides"
        );

        assert_eq!(host_chains[0].chain_id, 8009);
        assert_eq!(host_chains[0].url, "http://localhost:8545");
        assert_eq!(
            host_chains[0].acl_address,
            "0x339EBB773A9bC1deCFfD5ef4BC7c907e26C1f836"
        );

        assert_eq!(host_chains[1].chain_id, 10901);
        assert_eq!(host_chains[1].url, "http://localhost:9545");
        assert_eq!(
            host_chains[1].acl_address,
            "0xE61cff9C581c7c91AEF682c2C10e8632864339ab"
        );
    }

    /// RFC-021: a high-bit host chain carries a Solana base58 acl_address, while
    /// a clear-bit host chain carries an EVM address.
    #[test]
    fn test_host_chains_accepts_solana_address_base58_acl() {
        let config_path = ConfigBuilder::from_example()
            .expect("Failed to load example config")
            .to_temp_file()
            .expect("Failed to create temp config file");

        let config = Config::builder()
            .add_source(File::from(config_path.as_path()).format(FileFormat::Yaml))
            .build()
            .expect("Failed to build config");

        let mut settings: Settings = config.try_deserialize().expect("Failed to deserialize");
        // SPL Token program id — a canonical 32-byte Solana base58 pubkey.
        settings.host_chains[0].chain_id = (1u64 << 63) | 8009;
        settings.host_chains[0].acl_address =
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string();
        settings
            .validate_host_chains()
            .expect("Solana base58 acl_address must be accepted");

        settings.host_chains[0].acl_address =
            "0x339EBB773A9bC1deCFfD5ef4BC7c907e26C1f836".to_string();
        let err = settings
            .validate_host_chains()
            .expect_err("Solana chain id with EVM acl_address must be rejected");
        assert!(
            err.to_string().contains("does not match chain_id"),
            "Error should mention the chain/address mismatch, got: {err}"
        );

        settings.host_chains[0].chain_id = 8009;
        settings.host_chains[0].acl_address =
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string();
        let err = settings
            .validate_host_chains()
            .expect_err("EVM chain id with Solana acl_address must be rejected");
        assert!(err.to_string().contains("does not match chain_id"));
    }

    #[test]
    fn test_deserialize_host_chains_standard_yaml_still_works() {
        let config_path = ConfigBuilder::from_example()
            .expect("Failed to load example config")
            .to_temp_file()
            .expect("Failed to create temp config file");

        let config = Config::builder()
            .add_source(File::from(config_path.as_path()).format(FileFormat::Yaml))
            .build()
            .expect("Failed to build config");

        let settings: Settings = config.try_deserialize().expect("Failed to deserialize");
        assert_eq!(settings.host_chains.len(), 1);
        assert_eq!(settings.host_chains[0].chain_id, 8009);
        assert_eq!(settings.host_chains[0].url, "http://localhost:8545");
        assert_eq!(
            settings.host_chains[0].acl_address,
            "0x339EBB773A9bC1deCFfD5ef4BC7c907e26C1f836"
        );
    }

    /// Mirrors the real K8s deployment: YAML base config + env var overrides.
    /// Loads env vars from tests/relayer-test-env-only.env, then calls
    /// Settings::new (same code path as the real app).
    /// Smoke test: a QUOTED RFC-021 Solana host chain id (> `i64::MAX`) loads through the real
    /// `Settings::new` path to the exact u64. The `config` crate's numeric value is i64-backed, so
    /// the id must be a quoted string (as the e2e side-stack setup writes it) to avoid a lossy f64
    /// coercion. NOTE: this does NOT reproduce the env-specific `host_chains` "untagged MapOrSeq"
    /// crash observed in the live stack after a config reconcile (not isolable in a unit test — it
    /// needs the full live env). The manual `Visitor` in `deserialize_vec_from_map_or_seq` removes
    /// the untagged-enum buffering that error names, but its effect on the live crash is confirmed
    /// against the running relayer, not by this test (which passes with or without the Visitor).
    #[test]
    #[serial] // avoid env var leakage from parallel tests
    fn test_settings_loads_solana_host_chain_id_above_i64_max() {
        const SOLANA_CHAIN_ID: u64 = (1u64 << 63) | 12345; // 9223372036854788153 > i64::MAX
                                                           // Mirror the live config (the e2e setup deploy.ts writes the Solana host_chains entry with a
                                                           // QUOTED chain_id, because the `config` crate's numeric value is i64-backed and a bare
                                                           // number above i64::MAX coerces to a lossy f64). Loading must succeed through the real
                                                           // Settings::new path; this regresses the untagged-enum crash the Visitor fix removed.
        let base = std::fs::read_to_string("tests/relayer-test-config.yaml")
            .expect("read tests/relayer-test-config.yaml");
        let solana_cfg = base
            .replace(
                "  - chain_id: 8009",
                &format!("  - chain_id: \"{SOLANA_CHAIN_ID}\""),
            )
            .replace(
                "    acl_address: \"0x339EBB773A9bC1deCFfD5ef4BC7c907e26C1f836\"",
                "    acl_address: \"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA\"",
            );
        assert!(
            solana_cfg.contains(&format!("\"{SOLANA_CHAIN_ID}\"")),
            "test fixture must contain the quoted Solana chain id"
        );
        let path =
            std::env::temp_dir().join(format!("relayer-solana-host-{}.yaml", std::process::id()));
        std::fs::write(&path, solana_cfg).expect("write temp config");
        let result = Settings::new(Some(path.to_string_lossy().into_owned()));
        let _ = std::fs::remove_file(&path);
        let settings = result.expect(
            "Settings::new must load a quoted Solana host chain id above i64::MAX (RFC-021)",
        );
        assert_eq!(settings.host_chains[0].chain_id, SOLANA_CHAIN_ID);
    }

    /// Values in the env file intentionally differ from the YAML to prove
    /// env vars actually override. This catches map-vs-sequence and other
    /// type issues for any field set via env vars.
    #[test]
    #[serial] // avoid env var leakage from parallel tests
    fn test_settings_from_yaml_with_env_overrides() {
        let env_content = std::fs::read_to_string("tests/relayer-test-env-only.env")
            .expect("Failed to read tests/relayer-test-env-only.env");

        // Set env vars (same as K8s would inject them)
        let mut set_vars = Vec::new();
        for line in env_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                env::set_var(key, value);
                set_vars.push(key.to_string());
            }
        }

        // Use Settings::new — same code path as the real app
        let result = Settings::new(Some("tests/relayer-test-config.yaml".to_string()));

        // Clean up env vars before assertions to avoid leaking into other tests
        for key in &set_vars {
            env::remove_var(key);
        }

        let settings = result.expect(
            "Failed to deserialize Settings with env overrides — \
             a config field may not support env var configuration",
        );

        // Verify env var overrides took effect (values differ from YAML)
        // host_chains: YAML has chain_id=8009, env has 99999
        assert_eq!(settings.host_chains[0].chain_id, 99999);
        assert_eq!(settings.host_chains[0].url, "http://env-override:8545");

        // blockchain_rpc: YAML has chain_id=654321, env has 111111
        assert_eq!(settings.gateway.blockchain_rpc.chain_id, 111111);
        assert_eq!(
            settings.gateway.blockchain_rpc.http_url,
            "http://env-override:8757"
        );

        // listeners: YAML has 3, env overrides to 2
        assert_eq!(settings.gateway.listener_pool.listeners.len(), 2);

        // contracts: YAML has threshold=9, env has 5
        assert_eq!(settings.gateway.contracts.user_decrypt_shares_threshold, 5);

        // copro_kms_backoff_intervals: env overrides to different values
        assert_eq!(
            settings.http.retry_after.copro_kms_backoff_intervals.len(),
            2
        );
        assert_eq!(
            settings.http.retry_after.copro_kms_backoff_intervals[0].retry_interval_secs,
            2
        );

        // histogram buckets: env overrides via indexed env vars
        assert_eq!(
            settings.metrics.query_duration_histogram_bucket,
            vec![0.01, 0.05, 0.1]
        );
        assert_eq!(
            settings.http.metrics.histogram_buckets,
            vec![0.01, 0.05, 0.1]
        );

        // keyurl: env overrides, `source` included (both YAML and env say `chain`)
        match &settings.keyurl {
            KeyUrlConfig::Chain {
                kms_generation_address,
                poll_interval_ms,
            } => {
                assert_eq!(
                    kms_generation_address,
                    "0x00000000000000000000000000000000000000ff"
                );
                assert_eq!(*poll_interval_ms, 12000);
            }
            other => panic!("expected keyurl.source: chain, got {other:?}"),
        }

        // storage: env overrides
        assert!(settings.storage.sql_database_url.contains("env-override"));
    }

    /// `keyurl.source: config` from env vars alone, no `keyurl` block in any file — how the
    /// gitops deployments that need this mode are wired. `urls` is the load-bearing part: config-rs
    /// turns `URLS__0` / `URLS__1` into an index-keyed map, so without
    /// `deserialize_vec_from_map_or_seq` on `KeyData::urls` this fails with "invalid type: map,
    /// expected a sequence". Two URLs prove index order rather than collapse to one element.
    #[test]
    #[serial] // avoid env var leakage from parallel tests
    fn test_keyurl_source_config_from_env_only() {
        const KEY_DATA_ID: &str =
            "0x0400000000000000000000000000000000000000000000000000000000000003";
        const KEY_URL_0: &str = "http://minio-a:9000/kms-public/PUB-p1/PublicKey/03";
        const KEY_URL_1: &str = "http://minio-b:9000/kms-public/PUB-p2/PublicKey/03";
        const CRS_DATA_ID: &str =
            "0x0400000000000000000000000000000000000000000000000000000000000004";
        const CRS_URL_0: &str = "http://minio-a:9000/kms-public/PUB-p1/CRS/04";

        // Base config with the whole `keyurl` block removed: every field comes from env.
        let builder = ConfigBuilder::from_example()
            .expect("Failed to load example config")
            .remove_field("keyurl");

        let vars = [
            ("APP_KEYURL__SOURCE", "config"),
            ("APP_KEYURL__FHE_PUBLIC_KEY__DATA_ID", KEY_DATA_ID),
            ("APP_KEYURL__FHE_PUBLIC_KEY__URLS__0", KEY_URL_0),
            ("APP_KEYURL__FHE_PUBLIC_KEY__URLS__1", KEY_URL_1),
            ("APP_KEYURL__CRS__DATA_ID", CRS_DATA_ID),
            ("APP_KEYURL__CRS__URLS__0", CRS_URL_0),
        ];
        for (key, value) in vars {
            env::set_var(key, value);
        }

        // Settings::new — the same code path as the real app.
        let result = settings_new(builder);

        // Clean up before asserting so a failure cannot leak into other tests.
        for (key, _) in vars {
            env::remove_var(key);
        }

        let settings = result.expect(
            "`keyurl.source: config` should load from env vars alone — \
             `urls` may be missing its map-or-sequence deserializer",
        );

        match &settings.keyurl {
            KeyUrlConfig::Config {
                fhe_public_key,
                crs,
            } => {
                assert_eq!(fhe_public_key.data_id, KEY_DATA_ID);
                assert_eq!(fhe_public_key.urls, vec![KEY_URL_0, KEY_URL_1]);
                assert_eq!(crs.data_id, CRS_DATA_ID);
                assert_eq!(crs.urls, vec![CRS_URL_0]);
            }
            other => panic!("expected keyurl.source: config, got {other:?}"),
        }
    }
}
