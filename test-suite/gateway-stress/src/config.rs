use alloy::primitives::{Address, FixedBytes};
use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use serde::{Deserialize, Deserializer, Serialize};
use std::{path::Path, str::FromStr, time::Duration};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub gateway_url: String,
    pub host_chain_id: u64,
    pub gateway_chain_id: u64,
    pub decryption_address: Address,
    pub private_key: Option<String>,
    pub mnemonic: Option<String>,
    #[serde(default = "default_mnemonic_index")]
    pub mnemonic_index: usize,
    pub aws_kms_config: Option<AwsKmsConfig>,
    #[serde(deserialize_with = "parse_ct_handles")]
    pub user_ct_handles: Vec<FixedBytes<32>>,
    #[serde(deserialize_with = "parse_ct_handles")]
    pub public_ct_handles: Vec<FixedBytes<32>>,
    pub allowed_contract: Address,
    pub parallel_requests: u32,
    #[serde(with = "humantime_serde")]
    pub tests_duration: Duration,
    #[serde(with = "humantime_serde")]
    pub tests_interval: Duration,
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

fn parse_ct_handles<'de, D>(d: D) -> Result<Vec<FixedBytes<32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let ct_handles = Vec::<String>::deserialize(d)?
        .iter()
        .map(|h| FixedBytes::from_str(h.as_str()).expect("Invalid handle: {h}"))
        .collect();

    Ok(ct_handles)
}
