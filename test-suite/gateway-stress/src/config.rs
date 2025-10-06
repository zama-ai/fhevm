use alloy::primitives::{Address, FixedBytes, U256};
use config::{Config as ConfigBuilder, File, FileFormat};
use serde::Deserialize;
use std::{path::Path, time::Duration};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub user_ct: Vec<CiphertextConfig>,
    pub public_ct: Vec<CiphertextConfig>,
    pub allowed_contract: Address,
    pub parallel_requests: u32,
    #[serde(with = "humantime_serde")]
    pub tests_duration: Duration,
    #[serde(with = "humantime_serde")]
    pub tests_interval: Duration,
    #[serde(default)]
    pub sequential: bool,
    #[serde(default)]
    pub blockchain: Option<BlockchainConfig>,
    #[serde(default)]
    pub database: Option<DatabaseConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CiphertextConfig {
    pub handle: FixedBytes<32>,
    pub digest: FixedBytes<32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlockchainConfig {
    pub gateway_url: String,
    pub host_chain_id: u64,
    pub gateway_chain_id: u64,
    pub decryption_address: Address,
    pub private_key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub urls: Vec<String>,
    #[serde(default = "default_pool_size")]
    pub pool_size: u32,
    #[serde(with = "humantime_serde", default = "default_db_connection_timeout")]
    pub connection_timeout: Duration,
    pub key_id: U256,
    pub copro_tx_sender_addr: Address,
    pub insertion_chunk_size: usize,
}

impl Config {
    pub fn from_env_and_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let builder = ConfigBuilder::builder()
            .add_source(File::with_name(path.as_ref().to_str().unwrap()).format(FileFormat::Toml));
        let settings = builder.build()?;
        let config = settings.try_deserialize()?;
        Ok(config)
    }
}

fn default_pool_size() -> u32 {
    10
}

fn default_db_connection_timeout() -> Duration {
    Duration::from_secs(30)
}
