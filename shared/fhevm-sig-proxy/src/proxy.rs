use crate::{
    ProxyConfig,
    health::{self, endpoint_health_check},
};
use anyhow::{Context, anyhow};
use fhevm_sig_auth::FhevmSigVerifier;
use http::uri::Authority;
use pingora::{
    lb::{LoadBalancer, selection::RoundRobin},
    listeners::tls::TlsSettings,
    prelude::Server,
    proxy::http_proxy_service_with_name,
    server::{RunArgs, UnixShutdownSignalWatch, configuration::ServerConf},
    services::background::GenBackgroundService,
};
use std::sync::Arc;
use tracing::info;

/// The `Proxy` service: sender authentication and load-balancing in front of the endpoints.
pub struct Proxy {
    pub(crate) config: ProxyConfig,
    pub(crate) verifier: FhevmSigVerifier,
    pub(crate) endpoint_balancer: Arc<LoadBalancer<RoundRobin>>,
}

/// How long in-flight connections are given to finish once the graceful shutdown deadline (
/// [`ProxyConfig::shutdown_grace_period`]) has passed, before Pingora forcibly closes them.
const GRACEFUL_SHUTDOWN_TIMEOUT_SECONDS: u64 = 5;

impl Proxy {
    /// Creates the `Proxy` from its configuration, and the `health::State` used to monitor it.
    pub fn from_config(config: ProxyConfig) -> anyhow::Result<(Self, health::State)> {
        let mut endpoint_balancer =
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
        endpoint_balancer.set_health_check(Box::new(endpoint_health_check(&config)?));
        endpoint_balancer.health_check_frequency = Some(config.endpoint_healthcheck_frequency);
        endpoint_balancer.parallel_health_check = true;
        let endpoint_balancer = Arc::new(endpoint_balancer);

        let state = health::State::new(
            config.bind_address,
            &config.tls_self_check_path,
            Arc::clone(&endpoint_balancer),
            config.healthcheck_timeout,
        )?;
        let domain =
            fhevm_sig_auth::domain(config.ethereum_chain_id, config.protocol_config_address);
        let verifier =
            FhevmSigVerifier::new(domain, config.audience, config.allowed_senders.clone());
        let proxy = Self {
            verifier,
            config,
            endpoint_balancer,
        };
        Ok((proxy, state))
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

        let endpoint_health_service = GenBackgroundService::new(
            "BG endpoint health check".to_string(),
            Arc::clone(&self.endpoint_balancer),
        );
        server.add_service(endpoint_health_service);

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
    use crate::{Route, TlsConfig};
    use alloy::primitives::Address;
    use rstest::rstest;
    use std::{collections::HashSet, time::Duration};
    use std::{fs, path::PathBuf};

    pub(crate) fn test_config() -> ProxyConfig {
        ProxyConfig {
            bind_address: "127.0.0.1:0".parse().unwrap(),
            tls_config: TlsConfig {
                cert_path: "tls.crt".into(),
                key_path: "tls.key".into(),
            },
            ethereum_chain_id: 1,
            protocol_config_address: Address::ZERO,
            allowed_senders: HashSet::from([Address::ZERO]),
            audience: Address::ZERO,
            error_body: |_| crate::ErrorBody {
                status: http::StatusCode::INTERNAL_SERVER_ERROR,
                json: vec![],
            },
            routes: vec![Route {
                method: http::Method::GET,
                path: "/version".to_owned(),
            }],
            tls_self_check_path: "/version".to_owned(),
            endpoint_addresses: vec!["127.0.0.1:1".parse().unwrap()],
            endpoint_healthcheck_path: "/version".to_owned(),
            endpoint_connect_timeout: Duration::from_secs(3),
            endpoint_healthcheck_frequency: Duration::from_secs(3),
            endpoint_response_timeout: Duration::from_secs(60),
            endpoint_idle_timeout: Duration::from_secs(10),
            max_body_bytes: 1024,
            request_read_timeout: Duration::from_secs(30),
            shutdown_grace_period: Duration::from_secs(60),
            service_name: "test-proxy".to_owned(),
            healthcheck_timeout: Duration::from_secs(3),
        }
    }

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
        let config = ProxyConfig {
            tls_config: TlsConfig {
                cert_path: dir.join("tls.crt"),
                key_path: dir.join("tls.key"),
            },
            ..test_config()
        };

        let (proxy, _state) = Proxy::from_config(config).unwrap();
        let error = match proxy.build_server() {
            Ok(_) => panic!("invalid TLS material was accepted"),
            Err(e) => e.to_string(),
        };
        assert!(error.contains("Failed to configure TLS"), "{error}");
    }
}
