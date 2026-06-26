//! Layered configuration: `config/default.toml` overlaid by `APP_<SECTION>__<FIELD>`
//! environment variables (double-underscore nesting), mirroring relayer's layering.

use config::{Config, Environment, File};
use serde::Deserialize;
use solana_commitment_config::CommitmentConfig;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub solana: SolanaConfig,
    pub http: HttpConfig,
    pub metrics: MetricsConfig,
    pub database: DatabaseConfig,
    #[serde(default)]
    pub log: LogConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SolanaConfig {
    /// JSON-RPC endpoint the Carbon crawler polls (`getSignaturesForAddress`).
    pub rpc_url: String,
    /// `zama-host` program id. Default is mainnet; override per-network via
    /// `APP_SOLANA__PROGRAM_ID` (the local validator keypair pubkey differs).
    pub program_id: String,
    /// `finalized` | `confirmed` | `processed`. Default `finalized`.
    #[serde(default = "default_commitment")]
    pub commitment: String,
    /// Forward-poll interval for new signatures.
    #[serde(with = "humantime_serde_secs", default = "default_poll_interval")]
    pub poll_interval: Duration,
    /// `getSignaturesForAddress` backfill page size.
    #[serde(default = "default_backfill_batch")]
    pub backfill_batch: usize,
}

impl SolanaConfig {
    /// Resolves the configured commitment string to a [`CommitmentConfig`],
    /// defaulting to `finalized` for any unrecognized value. The single mapping
    /// shared by the pipeline crawler and the on-chain cross-check RPC.
    pub(crate) fn commitment_config(&self) -> CommitmentConfig {
        match self.commitment.as_str() {
            "processed" => CommitmentConfig::processed(),
            "confirmed" => CommitmentConfig::confirmed(),
            _ => CommitmentConfig::finalized(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpConfig {
    /// `host:port` for the JSON API. `:0` binds a free port (tests).
    pub endpoint: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsConfig {
    /// `host:port` for the Prometheus `/metrics` server (separate port).
    pub endpoint: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    #[serde(default = "default_log_format")]
    pub format: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            format: default_log_format(),
        }
    }
}

fn default_commitment() -> String {
    "finalized".to_string()
}
fn default_poll_interval() -> Duration {
    Duration::from_secs(5)
}
fn default_backfill_batch() -> usize {
    100
}
fn default_max_connections() -> u32 {
    10
}
fn default_log_format() -> String {
    "json".to_string()
}

/// `humantime`-style duration deserialization for seconds-granularity fields.
mod humantime_serde_secs {
    use serde::{Deserialize, Deserializer};
    use std::time::Duration;

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
        let raw = String::deserialize(d)?;
        humantime::parse_duration(&raw).map_err(serde::de::Error::custom)
    }
}

impl Settings {
    /// Loads `config/<dir>/default.toml` then overlays `APP_` env vars.
    pub fn load(config_dir: &str) -> anyhow::Result<Self> {
        let settings = Config::builder()
            .add_source(File::with_name(&format!("{config_dir}/default")).required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?;
        Ok(settings.try_deserialize()?)
    }
}
