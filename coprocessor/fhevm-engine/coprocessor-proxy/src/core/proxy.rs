#[cfg(test)]
use crate::core::TlsConfig;
use crate::{
    core::Config,
    monitoring::health::{self, endpoint_health_check},
};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use anyhow::{Context, anyhow};
use async_trait::async_trait;
use fhevm_engine_common::kms_tx_senders::Follower;
use fhevm_sig_auth::{FhevmSigVerifier, RefreshableSigners};
use http::uri::Authority;
use pingora::{
    lb::{LoadBalancer, selection::RoundRobin},
    listeners::tls::TlsSettings,
    prelude::Server,
    proxy::http_proxy_service_with_name,
    server::{
        RunArgs, ShutdownSignal, ShutdownSignalWatch, ShutdownWatch, UnixShutdownSignalWatch,
        configuration::ServerConf,
    },
    services::background::{BackgroundService, GenBackgroundService},
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tracing::info;

/// The `Proxy` service: sender authentication and load-balancing in front of the endpoints.
pub struct Proxy {
    pub(super) config: Config,
    pub(super) verifier: FhevmSigVerifier<RefreshableSigners>,
    pub(super) endpoint_balancer: Arc<LoadBalancer<RoundRobin>>,
    pub(super) kms_tx_senders: Arc<KmsTxSenders>,
    /// Set during the shutdown delay: each client connection closes after its next response.
    pub(super) draining: Arc<AtomicBool>,
}

/// Watches the Unix signals like [`UnixShutdownSignalWatch`], but holds `SIGTERM` back for
/// [`Config::shutdown_delay`]. Kubernetes sends `SIGTERM` when it starts removing the pod from its
/// Service, and nodes keep routing new connections to the pod until that removal reaches them.
/// Pingora stops accepting connections as soon as it handles `SIGTERM`.
struct DelayedShutdown {
    delay: Duration,
    draining: Arc<AtomicBool>,
}

#[async_trait]
impl ShutdownSignalWatch for DelayedShutdown {
    async fn recv(&self) -> ShutdownSignal {
        let signal = UnixShutdownSignalWatch.recv().await;
        if matches!(signal, ShutdownSignal::GracefulTerminate) {
            info!(
                "SIGTERM received, closing client connections after their next response for {:?}",
                self.delay
            );
            self.draining.store(true, Ordering::Relaxed);
            tokio::time::sleep(self.delay).await;
        }
        signal
    }
}

/// Runs the [`Follower`] that keeps the verifier's signers equal to the KMS tx-senders.
pub(super) struct KmsTxSenders(pub(super) Follower<DynProvider>);

#[async_trait]
impl BackgroundService for KmsTxSenders {
    async fn start(&self, mut shutdown: ShutdownWatch) {
        self.0
            .run(async move {
                let _ = shutdown.changed().await;
            })
            .await;
    }
}

/// How long in-flight connections are given to finish once the graceful shutdown deadline (
/// [`Config::shutdown_grace_period`]) has passed, before Pingora forcibly closes them.
const GRACEFUL_SHUTDOWN_TIMEOUT_SECONDS: u64 = 5;

impl Proxy {
    /// Creates the `Proxy` from its configuration, and the `health::State` used to monitor it.
    pub fn from_config(config: Config) -> anyhow::Result<(Self, health::State)> {
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

        let signers = RefreshableSigners::default();
        let follower = Follower::new(
            ProviderBuilder::new()
                .connect_http(config.ethereum_rpc_url.clone())
                .erased(),
            config.protocol_config_address,
            signers.clone(),
            config.kms_registry_poll_interval,
        );
        let state = health::State::new(
            config.bind_address,
            Arc::clone(&endpoint_balancer),
            follower.loaded(),
            config.healthcheck_timeout,
        )?;
        let domain =
            fhevm_sig_auth::domain(config.ethereum_chain_id, config.protocol_config_address);
        let verifier = FhevmSigVerifier::new(domain, config.coprocessor_signer_address, signers);
        let proxy = Self {
            verifier,
            config,
            endpoint_balancer,
            kms_tx_senders: Arc::new(KmsTxSenders(follower)),
            draining: Arc::default(),
        };
        Ok((proxy, state))
    }

    /// Runs the proxy until it receives `SIGTERM`, `SIGINT` or `SIGQUIT`.
    pub fn run(self) -> anyhow::Result<()> {
        info!("Starting Proxy");
        let shutdown_signal = DelayedShutdown {
            delay: self.config.shutdown_delay,
            draining: Arc::clone(&self.draining),
        };
        let server = self.build_server()?;
        server.run(RunArgs {
            shutdown_signal: Box::new(shutdown_signal),
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
        server.add_service(GenBackgroundService::new(
            "BG KMS tx-senders".to_string(),
            Arc::clone(&self.kms_tx_senders),
        ));

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

        let (proxy, _state) = Proxy::from_config(config).unwrap();
        let error = match proxy.build_server() {
            Ok(_) => panic!("invalid TLS material was accepted"),
            Err(e) => e.to_string(),
        };
        assert!(error.contains("Failed to configure TLS"), "{error}");
    }
}
