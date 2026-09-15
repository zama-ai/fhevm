#[cfg(test)]
use crate::core::TlsConfig;
use crate::core::{ApiKeyVerifier, Config};
use anyhow::{Context, anyhow};
use http::uri::Authority;
use pingora::{
    lb::{LoadBalancer, selection::RoundRobin},
    listeners::tls::TlsSettings,
    prelude::Server,
    proxy::http_proxy_service_with_name,
    server::{RunArgs, UnixShutdownSignalWatch, configuration::ServerConf},
};
use tracing::info;

/// The `Proxy` service: sender authentication and load-balancing in front of the endpoints.
pub struct Proxy {
    pub(super) config: Config,
    pub(super) verifier: ApiKeyVerifier,
    pub(super) endpoint_balancer: LoadBalancer<RoundRobin>,
}

/// How long in-flight connections are given to finish once the graceful shutdown deadline (
/// [`Config::shutdown_grace_period`]) has passed, before Pingora forcibly closes them.
const GRACEFUL_SHUTDOWN_TIMEOUT_SECONDS: u64 = 5;

impl Proxy {
    pub fn from_config(config: Config) -> anyhow::Result<Self> {
        let endpoint_balancer =
            LoadBalancer::try_from_iter(config.endpoint_addresses.iter().map(Authority::as_str))
                .context("Failed to resolve endpoint addresses")?;
        info!(
            "Resolved endpoints: {:?}",
            endpoint_balancer
                .backends()
                .get_backend()
                .iter()
                .map(|b| b.addr.to_string())
                .collect::<Vec<_>>()
        );

        Ok(Self {
            verifier: ApiKeyVerifier::new(config.api_key_digest),
            config,
            endpoint_balancer,
        })
    }

    /// Runs the proxy until it receives `SIGTERM`, `SIGINT` or `SIGQUIT`.
    pub fn run(self) -> anyhow::Result<()> {
        info!("Starting Proxy");
        let server = self.build_server()?;
        server.run(RunArgs {
            shutdown_signal: Box::new(UnixShutdownSignalWatch),
        });
        info!("Proxy stopped");
        Ok(())
    }

    /// Builds the Pingora server.
    fn build_server(self) -> anyhow::Result<Server> {
        let server_conf = ServerConf {
            grace_period_seconds: Some(self.config.shutdown_grace_period.as_secs()),
            graceful_shutdown_timeout_seconds: Some(GRACEFUL_SHUTDOWN_TIMEOUT_SECONDS),
            ..ServerConf::default()
        };
        let mut server = Server::new_with_opt_and_conf(None, server_conf);
        server.bootstrap();

        let listen_addr = self.config.bind_address.to_string();
        let tls_config = &self.config.tls_config;
        let cert_path = path_to_str(&tls_config.cert_path)?;
        let key_path = path_to_str(&tls_config.key_path)?;
        let tls = TlsSettings::intermediate(cert_path, key_path)
            .map_err(|e| anyhow!("Failed to configure TLS: {e}"))?;
        // OpenSSL silently drops a private key that does not match the certificate loaded after
        // it, so check their consistency explicitly to fail fast.
        tls.check_private_key()
            .map_err(|e| anyhow!("Failed to configure TLS: {e}"))?;

        let service_name = self.config.service_name.clone();
        let mut proxy_service =
            http_proxy_service_with_name(&server.configuration, self, &service_name);
        proxy_service.add_tls_with_settings(&listen_addr, None, tls);
        server.add_service(proxy_service);
        info!("Proxy will listen at: https://{listen_addr}");

        Ok(server)
    }
}

fn path_to_str(path: &std::path::Path) -> anyhow::Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("Path {} is not valid UTF-8", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::{fs, path::PathBuf};

    fn key_pem() -> String {
        rcgen::generate_simple_self_signed(vec!["localhost".to_string()])
            .unwrap()
            .signing_key
            .serialize_pem()
    }

    /// Writes a self-signed certificate and key in a temporary directory, then overwrites either
    /// `tls.crt` or `tls.key` with `content`.
    fn tls_dir(overwrite: &str, content: &str) -> PathBuf {
        let certified_key =
            rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        let dir = std::env::temp_dir().join(format!(
            "proxy-invalid-tls-{}-{overwrite}-{}",
            std::process::id(),
            content.len()
        ));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("tls.crt"), certified_key.cert.pem()).unwrap();
        fs::write(
            dir.join("tls.key"),
            certified_key.signing_key.serialize_pem(),
        )
        .unwrap();
        fs::write(dir.join(overwrite), content).unwrap();
        dir
    }

    #[rstest]
    #[case::garbage_cert("tls.crt", "not a certificate".to_string())]
    #[case::garbage_key("tls.key", "not a private key".to_string())]
    #[case::mismatched_key("tls.key", key_pem())]
    fn build_server_rejects_invalid_tls_material(#[case] overwrite: &str, #[case] content: String) {
        let dir = tls_dir(overwrite, &content);
        let config = Config {
            tls_config: TlsConfig {
                cert_path: dir.join("tls.crt"),
                key_path: dir.join("tls.key"),
            },
            endpoint_addresses: vec!["127.0.0.1:1".parse().unwrap()],
            ..Config::default()
        };

        let error = match Proxy::from_config(config).unwrap().build_server() {
            Ok(_) => panic!("invalid TLS material was accepted"),
            Err(e) => e.to_string(),
        };
        assert!(error.contains("Failed to configure TLS"), "{error}");
    }
}
