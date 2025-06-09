use alloy::transports::http::reqwest::Url;
use std::time::Duration;

pub mod gw_listener;
pub mod http_server;

#[derive(Clone, Debug)]
pub struct ConfigSettings {
    pub database_url: String,
    pub database_pool_size: u32,
    pub verify_proof_req_db_channel: String,

    pub gw_url: Url,

    pub error_sleep_initial_secs: u16,
    pub error_sleep_max_secs: u16,
    pub health_check_port: u16,
    pub health_check_timeout: Duration,
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost/coprocessor".to_owned()),
            database_pool_size: 16,
            verify_proof_req_db_channel: "verify_proof_requests".to_owned(),
            gw_url: "ws://127.0.0.1:8546".try_into().expect("Invalid URL"),
            error_sleep_initial_secs: 1,
            error_sleep_max_secs: 10,
            health_check_port: 8080,
            health_check_timeout: Duration::from_secs(4),
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
