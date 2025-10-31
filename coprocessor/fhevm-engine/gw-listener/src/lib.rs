use alloy::primitives::Uint;
use alloy::transports::http::reqwest::Url;
use std::time::Duration;

use tracing::error;

pub mod aws_s3;
pub(crate) mod database;
pub(crate) mod digest;
pub mod gw_listener;
pub mod http_server;
pub(crate) mod metrics;
pub(crate) mod sks_key;

pub(crate) type ChainId = u64;
pub(crate) type KeyId = Uint<256, 4>;
pub(crate) type TenantId = u64;

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
    pub host_chain_id: ChainId,
    pub database_url: String,
    pub database_pool_size: u32,
    pub verify_proof_req_db_channel: String,

    pub gw_url: Url,

    pub error_sleep_initial_secs: u16,
    pub error_sleep_max_secs: u16,

    pub health_check_port: u16,

    pub health_check_timeout: Duration,

    pub get_logs_poll_interval: Duration,
    pub get_logs_block_batch_size: u64,
    pub catchup_kms_generation_from_block: Option<i64>,
}

pub fn chain_id_from_env() -> Option<ChainId> {
    let chain_id_str = std::env::var("CHAIN_ID").ok()?;
    let Ok(chain_id) = chain_id_str.parse::<u64>() else {
        error!("CHAIN_ID environment variable is not a valid u64");
        return None;
    };
    Some(chain_id)
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            host_chain_id: chain_id_from_env().unwrap_or(12345),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost/coprocessor".to_owned()),
            database_pool_size: 16,
            verify_proof_req_db_channel: "event_zkpok_new_work".to_owned(),
            gw_url: "ws://127.0.0.1:8546".try_into().expect("Invalid URL"),
            error_sleep_initial_secs: 1,
            error_sleep_max_secs: 10,
            health_check_port: 8080,
            health_check_timeout: Duration::from_secs(4),
            get_logs_poll_interval: Duration::from_secs(1),
            get_logs_block_batch_size: 100,
            catchup_kms_generation_from_block: None,
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
