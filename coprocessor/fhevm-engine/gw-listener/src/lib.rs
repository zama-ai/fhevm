use alloy::primitives::{Address, Uint};
use alloy::transports::http::reqwest::Url;
use fhevm_engine_common::utils::DatabaseURL;
use std::time::Duration;

pub mod aws_s3;
pub(crate) mod database;
pub(crate) mod digest;
pub(crate) mod drift_detector;
pub mod gw_listener;
pub mod http_server;
pub(crate) mod metrics;
pub(crate) mod sks_key;

pub(crate) type KeyId = Uint<256, 4>;

#[derive(Clone, Copy, Debug)]
pub enum KeyType {
    ServerKey = 0,
    PublicKey = 1,
}

impl TryFrom<u8> for KeyType {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> anyhow::Result<KeyType> {
        match value {
            0 => Ok(KeyType::ServerKey),
            1 => Ok(KeyType::PublicKey),
            _ => Err(anyhow::anyhow!("Invalid KeyType")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConfigSettings {
    pub database_url: DatabaseURL,
    pub database_pool_size: u32,
    pub verify_proof_req_db_channel: String,

    pub gw_url: Url,

    pub error_sleep_initial_secs: u16,
    pub error_sleep_max_secs: u16,

    pub health_check_port: u16,

    pub health_check_timeout: Duration,

    pub get_logs_poll_interval: Duration,
    pub get_logs_block_batch_size: u64,
    pub replay_from_block: Option<i64>,
    pub replay_skip_verify_proof: bool,

    pub log_last_processed_every_number_of_updates: u64,

    pub ciphertext_commits_address: Option<Address>,
    pub gateway_config_address: Option<Address>,
    pub drift_no_consensus_timeout: Duration,
    pub drift_post_consensus_grace: Duration,
    /// How long to wait after detecting a pending drift-revert signal before
    /// running the revert SQL. Gives other services time to see the signal and
    /// drop their in-flight work.
    pub drift_auto_revert_grace_period: Duration,
    /// If true, the drift detector creates drift-revert signals when it sees
    /// a consensus mismatch. If false, drift is still detected and logged,
    /// but no signal is created.
    pub drift_auto_revert_enabled: bool,
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            database_url: DatabaseURL::default(),
            database_pool_size: 16,
            verify_proof_req_db_channel: "event_zkpok_new_work".to_owned(),
            gw_url: "ws://127.0.0.1:8546".try_into().expect("Invalid URL"),
            error_sleep_initial_secs: 1,
            error_sleep_max_secs: 10,
            health_check_port: 8080,
            health_check_timeout: Duration::from_secs(4),
            get_logs_poll_interval: Duration::from_millis(500),
            get_logs_block_batch_size: 100,
            replay_from_block: None,
            replay_skip_verify_proof: false,
            log_last_processed_every_number_of_updates: 50,
            ciphertext_commits_address: None,
            gateway_config_address: None,
            drift_no_consensus_timeout: Duration::from_secs(5),
            drift_post_consensus_grace: Duration::from_secs(2),
            drift_auto_revert_grace_period: Duration::from_secs(60),
            drift_auto_revert_enabled: false,
        }
    }
}

/// Represents the health status of the gateway listener service
#[derive(Debug)]
pub struct HealthStatus {
    /// Overall health of the service
    pub healthy: bool,
    /// Database connection status
    pub database_connected: bool,
    /// Blockchain provider connection status
    pub blockchain_connected: bool,
    /// Details about any issues encountered during health check
    pub details: Option<String>,
}

impl HealthStatus {
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            database_connected: true,
            blockchain_connected: true,
            details: None,
        }
    }

    pub fn unhealthy(
        database_connected: bool,
        blockchain_connected: bool,
        details: String,
    ) -> Self {
        Self {
            healthy: false,
            database_connected,
            blockchain_connected,
            details: Some(details),
        }
    }
}
