mod executor;
mod switch_and_squash;

use serde::{Deserialize, Serialize};
use switch_and_squash::{SnsClientKey, SwitchAndSquashKey};
use tokio::sync::broadcast;
use tracing::info;

#[derive(Serialize, Deserialize, Clone)]
pub struct FhePubKeySet {
    pub public_key: tfhe::CompactPublicKey,
    pub server_key: tfhe::ServerKey,
    pub sns_key: Option<SwitchAndSquashKey>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct KeySet {
    pub client_key: tfhe::ClientKey,
    pub sns_secret_key: SnsClientKey,
    pub public_keys: FhePubKeySet,
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
) -> Result<(), Box<dyn std::error::Error>> {
    info!(target: "sns", "Worker started with {}", conf);

    executor::run_loop(keys, conf, cancel_chan).await?;

    info!(target: "sns", "Worker stopped");
    Ok(())
}
