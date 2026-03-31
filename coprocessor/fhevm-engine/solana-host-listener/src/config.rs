use anyhow::{Context, Result};
use clap::Parser;
use std::{collections::HashMap, fs, path::PathBuf};

const DEFAULT_HOST_ADDRESSES_ENV_PATH: &str = "../../../solana-host-contracts/addresses/.env.host";

#[derive(Clone, Debug, Parser, PartialEq, Eq)]
#[command(
    name = "solana_host_listener_poller",
    about = "Poll finalized Solana host-program events and feed the coprocessor"
)]
pub struct PollerConfig {
    #[arg(long, env = "SOLANA_HOST_LISTENER_RPC_URL")]
    pub rpc_url: Option<String>,
    #[arg(long, env = "SOLANA_HOST_LISTENER_PROGRAM_ID")]
    pub program_id: Option<String>,
    #[arg(long, env = "SOLANA_HOST_LISTENER_HOST_CHAIN_ID")]
    pub host_chain_id: Option<u64>,
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: Option<String>,
    #[arg(
        long,
        env = "SOLANA_HOST_LISTENER_BATCH_SIZE_SLOTS",
        default_value_t = 128
    )]
    pub batch_size_slots: u64,
    #[arg(
        long,
        env = "SOLANA_HOST_LISTENER_POLL_INTERVAL_MS",
        default_value_t = 2_000
    )]
    pub poll_interval_ms: u64,
    #[arg(
        long,
        env = "SOLANA_HOST_LISTENER_RETRY_INTERVAL_MS",
        default_value_t = 5_000
    )]
    pub retry_interval_ms: u64,
    #[arg(long, env = "SOLANA_HOST_LISTENER_HEALTH_PORT", default_value_t = 8085)]
    pub health_port: u16,
    #[arg(long, env = "SOLANA_HOST_LISTENER_ONCE", default_value_t = false)]
    pub once: bool,
    #[arg(
        long,
        env = "SOLANA_HOST_LISTENER_COMMITMENT",
        default_value = "finalized"
    )]
    pub commitment: String,
    #[arg(
        long,
        env = "SOLANA_HOST_LISTENER_ADDRESSES_ENV",
        default_value = DEFAULT_HOST_ADDRESSES_ENV_PATH
    )]
    pub addresses_env: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedPollerConfig {
    pub rpc_url: String,
    pub program_id: String,
    pub host_chain_id: u64,
    pub database_url: String,
    pub batch_size_slots: u64,
    pub poll_interval_ms: u64,
    pub retry_interval_ms: u64,
    pub health_port: u16,
    pub once: bool,
    pub commitment: String,
    pub addresses_env: PathBuf,
}

impl PollerConfig {
    pub fn resolve(self) -> Result<ResolvedPollerConfig> {
        let host_env = load_addresses_env(&self.addresses_env).unwrap_or_default();

        Ok(ResolvedPollerConfig {
            rpc_url: self
                .rpc_url
                .or_else(|| host_env.get("SOLANA_HOST_RPC_URL").cloned())
                .or_else(|| host_env.get("SOLANA_RPC_URL").cloned())
                .context("missing rpc_url; pass --rpc-url or set SOLANA_HOST_RPC_URL in addresses/.env.host")?,
            program_id: self
                .program_id
                .or_else(|| host_env.get("SOLANA_HOST_PROGRAM_ID").cloned())
                .or_else(|| host_env.get("SOLANA_PROGRAM_ID").cloned())
                .context("missing program_id; pass --program-id or set SOLANA_HOST_PROGRAM_ID in addresses/.env.host")?,
            host_chain_id: self
                .host_chain_id
                .or_else(|| {
                    host_env
                        .get("SOLANA_HOST_CHAIN_ID")
                        .and_then(|value| value.parse::<u64>().ok())
                })
                .or_else(|| {
                    host_env
                        .get("SOLANA_HOST_CHAIN_ID")
                        .and_then(|value| value.parse::<u64>().ok())
                })
                .context(
                    "missing host_chain_id; pass --host-chain-id or set SOLANA_HOST_CHAIN_ID in addresses/.env.host",
                )?,
            database_url: self
                .database_url
                .context("missing database_url; pass --database-url or set DATABASE_URL")?,
            batch_size_slots: self.batch_size_slots,
            poll_interval_ms: self.poll_interval_ms,
            retry_interval_ms: self.retry_interval_ms,
            health_port: self.health_port,
            once: self.once,
            commitment: self.commitment,
            addresses_env: self.addresses_env,
        })
    }
}

pub fn load_addresses_env(path: &PathBuf) -> Result<HashMap<String, String>> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(parse_env_contents(&contents))
}

fn parse_env_contents(contents: &str) -> HashMap<String, String> {
    let mut values = HashMap::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        values.insert(
            key.trim().to_owned(),
            value.trim().trim_matches('"').to_owned(),
        );
    }

    values
}

#[cfg(test)]
mod tests {
    use super::{parse_env_contents, PollerConfig, ResolvedPollerConfig};
    use std::path::PathBuf;

    #[test]
    fn parses_env_file_contents() {
        let parsed = parse_env_contents(
            r#"
            # comment
            SOLANA_RPC_URL=http://127.0.0.1:8899
            SOLANA_PROGRAM_ID = test-program
            SOLANA_HOST_CHAIN_ID = 1001
            "#,
        );

        assert_eq!(
            parsed.get("SOLANA_RPC_URL"),
            Some(&"http://127.0.0.1:8899".to_owned())
        );
        assert_eq!(
            parsed.get("SOLANA_PROGRAM_ID"),
            Some(&"test-program".to_owned())
        );
        assert_eq!(parsed.get("SOLANA_HOST_CHAIN_ID"), Some(&"1001".to_owned()));
    }

    #[test]
    fn resolve_prefers_explicit_values() {
        let config = PollerConfig {
            rpc_url: Some("http://rpc".to_owned()),
            program_id: Some("program".to_owned()),
            host_chain_id: Some(77),
            database_url: Some("postgres://db".to_owned()),
            batch_size_slots: 16,
            poll_interval_ms: 500,
            retry_interval_ms: 1000,
            health_port: 9000,
            once: true,
            commitment: "finalized".to_owned(),
            addresses_env: PathBuf::from("/tmp/missing.env"),
        };

        let resolved = config.resolve().unwrap();
        assert_eq!(
            resolved,
            ResolvedPollerConfig {
                rpc_url: "http://rpc".to_owned(),
                program_id: "program".to_owned(),
                host_chain_id: 77,
                database_url: "postgres://db".to_owned(),
                batch_size_slots: 16,
                poll_interval_ms: 500,
                retry_interval_ms: 1000,
                health_port: 9000,
                once: true,
                commitment: "finalized".to_owned(),
                addresses_env: PathBuf::from("/tmp/missing.env"),
            }
        );
    }
}
