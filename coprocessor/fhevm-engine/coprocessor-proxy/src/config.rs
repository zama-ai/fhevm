//! The coprocessor proxy reads its [`ProxyConfig`] and its own [`Config`] from the same sources:
//! a TOML file, then environment variables prefixed with [`ENV_PREFIX`], which take precedence.

use anyhow::Context;
use config::{Config as Sources, Environment, File, FileFormat};
use fhevm_sig_proxy::ProxyConfig;
use serde::{Deserialize, Deserializer};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
};
use tracing::Level;

/// The prefix of the environment variables, with `__` between nested keys, e.g.
/// `COPROCESSOR_PROXY_SIGNERS__PROTOCOL_CONFIG__RPC_URL`.
pub const ENV_PREFIX: &str = "COPROCESSOR_PROXY";

/// The settings of the binary around the proxy.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Config {
    #[serde(deserialize_with = "deserialize_level", default = "default_log_level")]
    pub log_level: Level,
    /// The port of `/healthz`, `/liveness` and `/version`.
    #[serde(default = "default_healthcheck_port")]
    pub healthcheck_port: u16,
    /// The address of the Prometheus `/metrics` endpoint.
    #[serde(default = "default_metrics_endpoint")]
    pub metrics_endpoint: SocketAddr,
}

/// Loads the binary's settings and the proxy's configuration from an optional TOML file and the
/// environment.
pub fn load(path: Option<&Path>) -> anyhow::Result<(Config, ProxyConfig)> {
    let mut builder = Sources::builder();
    if let Some(path) = path {
        let path = path.to_str().context("config path is not valid UTF-8")?;
        builder = builder.add_source(File::with_name(path).format(FileFormat::Toml));
    }
    let sources = builder
        .add_source(
            Environment::with_prefix(ENV_PREFIX)
                .prefix_separator("_")
                .separator("__")
                .try_parsing(true),
        )
        .build()?;
    let proxy_config = sources.clone().try_deserialize()?;
    Ok((sources.try_deserialize()?, proxy_config))
}

fn deserialize_level<'de, D>(d: D) -> Result<Level, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn default_log_level() -> Level {
    Level::INFO
}

fn default_healthcheck_port() -> u16 {
    8080
}

fn default_metrics_endpoint() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 9100)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fhevm_sig_proxy::{defaults, SignerSource};
    use http::Method;
    use std::{
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
        time::Duration,
    };

    /// What a deployment adds to the shipped configuration, as top-level TOML keys.
    const DEPLOYMENT: &str = r#"ethereum_chain_id = 11155111
protocol_config_address = "0x00000000000000000000000000000000000000c0"
audience = "0x00000000000000000000000000000000000000a1"
signers = { protocol_config = { rpc_url = "http://localhost:8545" } }
"#;

    fn shipped_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/proxy.toml")
    }

    /// Loads [`DEPLOYMENT`] followed by the shipped configuration, after replacing `from` with
    /// `to`.
    fn shipped_with(from: &str, to: &str) -> anyhow::Result<ProxyConfig> {
        let config = DEPLOYMENT.to_owned() + &std::fs::read_to_string(shipped_path()).unwrap();
        assert!(config.contains(from), "{from}");
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "coprocessor-proxy-{}-{}.toml",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, config.replace(from, to)).unwrap();
        let loaded = load(Some(&path));
        std::fs::remove_file(path).ok();
        let (_, config) = loaded?;
        config.validate()?;
        Ok(config)
    }

    #[test]
    fn the_shipped_config_is_a_prf_proxy() {
        let config = shipped_with("", "").unwrap();
        assert_eq!(config.routes.len(), 1);
        assert_eq!(config.routes[0].methods, [Method::GET, Method::HEAD]);
        assert_eq!(config.routes[0].upstream, "prf-service");
        assert_eq!(config.routes[0].max_body_bytes, 0);
        assert_eq!(
            config.routes[0].endpoint_response_timeout,
            Duration::from_secs(120)
        );
        assert_eq!(
            config.signers,
            SignerSource::ProtocolConfig {
                rpc_url: "http://localhost:8545".parse().unwrap(),
                refresh_interval: defaults::signer_refresh_interval(),
            }
        );
        assert!(config.forward_authorization);
        assert_eq!(
            config.upstreams[0].endpoint_addresses[0],
            "prf-service:8080"
        );
        assert_eq!(
            config.shutdown_grace_period,
            defaults::shutdown_grace_period()
        );
    }

    #[test]
    fn the_shipped_config_alone_lacks_the_deployment_values() {
        assert!(load(Some(&shipped_path())).is_err());
    }

    #[test]
    fn an_invalid_config_fails() {
        for (from, to) in [
            ("{prf}", "{prf:u16}"),
            ("upstream = \"prf-service\"", "upstream = \"other\""),
            ("[\"prf-service:8080\"]", "[]"),
            ("[\"prf-service:8080\"]", "[\"prf service\"]"),
            ("[\"GET\", \"HEAD\"]", "[\"G T\"]"),
            (
                "rpc_url = \"http://localhost:8545\"",
                "rpc_url = \"http://localhost:8545\", refresh_interval = \"0s\"",
            ),
            ("max_body_bytes = 0", "max_body_bytes = 0\nname = \"prf\""),
            ("ethereum_chain_id = 11155111\n", ""),
        ] {
            assert!(shipped_with(from, to).is_err(), "{to}");
        }
    }

    #[test]
    fn a_config_without_forward_authorization_removes_the_header() {
        let config = shipped_with("forward_authorization = true\n", "").unwrap();
        assert!(!config.forward_authorization);
    }
}
