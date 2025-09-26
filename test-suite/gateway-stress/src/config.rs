use alloy::primitives::{Address, FixedBytes};
use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use serde::{Deserialize, Deserializer, Serialize};
use std::{path::Path, str::FromStr, time::Duration};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub gateway_url: Option<String>,
    pub host_chain_id: Option<u64>,
    pub gateway_chain_id: Option<u64>,
    pub decryption_address: Option<Address>,
    pub private_key: Option<String>,
    pub mnemonic: Option<String>,
    #[serde(default = "default_mnemonic_index")]
    pub mnemonic_index: usize,
    pub aws_kms_config: Option<AwsKmsConfig>,
    #[serde(default, deserialize_with = "parse_ct_handles_option")]
    pub user_ct_handles: Vec<FixedBytes<32>>,
    #[serde(default, deserialize_with = "parse_ct_handles_option")]
    pub public_ct_handles: Vec<FixedBytes<32>>,
    pub allowed_contract: Option<Address>,
    pub parallel_requests: Option<u32>,
    #[serde(with = "humantime_serde::option")]
    pub tests_duration: Option<Duration>,
    #[serde(with = "humantime_serde::option")]
    pub tests_interval: Option<Duration>,
    #[serde(default)]
    pub sequential: bool,
    /// DB connector configuration
    pub db_connector: Option<DbConnectorTestConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DbConnectorTestConfig {
    pub database_urls: Vec<String>,
    pub request_type: String,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    pub batch_size: usize,
    #[serde(with = "humantime_serde")]
    pub batch_interval: Duration,
    pub pool_size: Option<usize>,
    pub connection_timeout: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AwsKmsConfig {
    pub key_id: String,
    pub region: Option<String>,
    pub endpoint: Option<String>,
}

impl Config {
    pub fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> anyhow::Result<Self> {
        let mut builder = ConfigBuilder::builder();

        // If path is provided, add it as a config source
        if let Some(path) = path {
            builder = builder.add_source(
                File::with_name(path.as_ref().to_str().unwrap()).format(FileFormat::Toml),
            );
        }

        builder = builder.add_source(
            Environment::default()
                .list_separator(",")
                .with_list_parse_key("public_ct_handles")
                .with_list_parse_key("user_ct_handles")
                .try_parsing(true),
        );

        let settings = builder.build()?;
        let config = settings.try_deserialize()?;
        Ok(config)
    }
}

fn default_mnemonic_index() -> usize {
    0
}

fn parse_ct_handles_option<'de, D>(d: D) -> Result<Vec<FixedBytes<32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<Vec<String>>::deserialize(d)?;
    Ok(opt.unwrap_or_default()
        .iter()
        .filter_map(|h| FixedBytes::from_str(h.as_str()).ok())
        .collect())
}
