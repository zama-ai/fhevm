mod executor;
mod keyset;
mod switch_and_squash;

#[cfg(test)]
mod tests;

use fhevm_engine_common::types::FhevmError;
use serde::{Deserialize, Serialize};
use switch_and_squash::{SnsClientKey, SwitchAndSquashKey};
use thiserror::Error;
use tokio::sync::broadcast;
use tracing::info;

#[derive(Serialize, Deserialize, Clone)]
pub struct KeySet {
    pub sns_key: SwitchAndSquashKey,
    pub sns_secret_key: Option<SnsClientKey>,
    pub server_key: tfhe::ServerKey,
}

pub struct DBConfig {
    pub url: String,
    pub listen_channel: String,
    pub notify_channel: String,
    pub batch_limit: u32,
    pub polling_interval: u32,
    pub max_connections: u32,
}

pub struct Config {
    pub tenant_api_key: String,
    pub db: DBConfig,
}

/// Implement Display for Config
impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "db_url: {},  db_listen_channel: {}, db_notify_channel: {}, db_batch_limit: {}",
            self.db.url, self.db.listen_channel, self.db.notify_channel, self.db.batch_limit
        )
    }
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Conversion error: {0}")]
    ConversionError(#[from] anyhow::Error),

    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("CtType error: {0}")]
    CtType(#[from] FhevmError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
}

/// Starts the worker loop
///
/// # Arguments
///
/// * `keys` - The keys to use for the worker
/// * `limit` - The maximum number of tasks to process per iteration
pub async fn run(
    keys: Option<KeySet>,
    conf: &Config,
    cancel_chan: broadcast::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(target: "sns", "Worker started with {}", conf);

    executor::run_loop(keys, conf, cancel_chan).await?;

    info!(target: "sns", "Worker stopped");
    Ok(())
}
