use alloy::primitives::{Address, FixedBytes};
use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use serde::{Deserialize, Deserializer, Serialize};
use std::{path::Path, str::FromStr, time::Duration};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub gateway_url: String,
    pub gateway_chain_id: u64,
    pub decryption_address: Address,
    pub private_key: Option<String>,
    pub aws_kms_config: Option<AwsKmsConfig>,
    #[serde(deserialize_with = "parse_ct_handles")]
    pub ct_handles: Vec<FixedBytes<32>>,
    #[serde(default = "default_parallel_requests")]
    pub parallel_requests: u64,
    #[serde(deserialize_with = "parse_tests_duration")]
    pub tests_duration: Duration,
    #[serde(deserialize_with = "parse_tests_interval")]
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

        builder = builder.add_source(Environment::default());

        let settings = builder.build()?;
        let config = settings.try_deserialize()?;
        Ok(config)
    }
}

fn default_parallel_requests() -> u64 {
    100
}

const DEFAULT_TESTS_DURATION: Duration = Duration::from_secs(3600);
const DEFAULT_TESTS_INTERVAL_DURATION: Duration = Duration::from_secs(1);

fn parse_tests_duration<'de, D>(d: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let tests_duration = Option::<u64>::deserialize(d)?
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_TESTS_DURATION);

    Ok(tests_duration)
}

fn parse_tests_interval<'de, D>(d: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let tests_interval_duration = Option::<u64>::deserialize(d)?
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_TESTS_INTERVAL_DURATION);

    Ok(tests_interval_duration)
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
