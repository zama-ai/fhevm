use crate::{
    core::{Config, SIGNER_RATE_INTERVAL},
    kms_tx_senders::Follower,
    monitoring::health,
};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use anyhow::{Context, anyhow};
use async_trait::async_trait;
use fhevm_sig_auth::{FhevmSigVerifier, RefreshableSigners};
use openssl::{asn1::Asn1Time, x509::X509};
use pingora::{
    listeners::tls::TlsSettings,
    prelude::Server,
    proxy::http_proxy_service_with_name,
    server::{
        RunArgs, ShutdownSignal, ShutdownSignalWatch, ShutdownWatch, UnixShutdownSignalWatch,
        configuration::ServerConf,
    },
    services::background::{BackgroundService, GenBackgroundService},
};
use pingora_limits::rate::Rate;
use prometheus::{IntGauge, register_int_gauge};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tracing::info;

/// Read at startup only: a renewed certificate is served, and reported, after a restart.
static TLS_CERTIFICATE_NOT_AFTER: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_proxy_tls_certificate_not_after_seconds",
        "Unix timestamp at which the served TLS certificate expires"
    )
    .unwrap()
});

/// The `Proxy` service: sender authentication in front of prf-service.
pub struct Proxy {
    pub(super) config: Config,
    pub(super) verifier: FhevmSigVerifier<RefreshableSigners>,
    /// [`Config::prf_service_address`], resolved.
    pub(super) prf_service: SocketAddr,
    pub(super) kms_tx_senders: Arc<Follower<DynProvider>>,
    /// The authenticated requests of each signer, per [`SIGNER_RATE_INTERVAL`].
    pub(super) signer_requests: Rate,
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

#[async_trait]
impl<P: Provider> BackgroundService for Follower<P> {
    async fn start(&self, mut shutdown: ShutdownWatch) {
        self.run(async move {
            let _ = shutdown.changed().await;
        })
        .await;
    }
}

/// How long in-flight connections are given to finish once the shutdown grace period has passed,
/// before Pingora forcibly closes them. The chart's `terminationGracePeriodSeconds`
/// (coprocessor-proxy-deployment.yaml) includes it.
const GRACEFUL_SHUTDOWN_TIMEOUT_SECONDS: u64 = 5;

impl Proxy {
    /// Creates the `Proxy` from its configuration, and the `health::State` used to monitor it.
    pub fn from_config(config: Config) -> anyhow::Result<(Self, health::State)> {
        let address = config.prf_service_address.as_str();
        let prf_service = address
            .to_socket_addrs()
            .ok()
            .and_then(|mut resolved| resolved.next())
            .with_context(|| format!("Failed to resolve the prf-service address {address}"))?;
        info!("Resolved prf-service: {prf_service}");

        let signers = RefreshableSigners::default();
        let follower = Follower::new(
            ProviderBuilder::new()
                .connect_http(config.ethereum_rpc_url.clone())
                .erased(),
            config.protocol_config_address,
            signers.clone(),
            config.kms_registry_refresh_interval,
        );
        let state = health::State::new(
            config.bind_address,
            follower.loaded(),
            config.healthcheck_timeout,
        )?;
        let domain =
            fhevm_sig_auth::domain(config.ethereum_chain_id, config.protocol_config_address);
        let verifier = FhevmSigVerifier::new(domain, config.coprocessor_signer_address, signers);
        let proxy = Self {
            verifier,
            config,
            prf_service,
            kms_tx_senders: Arc::new(follower),
            signer_requests: Rate::new(SIGNER_RATE_INTERVAL),
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
        // The grace lets a request held for the whole response timeout complete.
        let grace = self
            .config
            .prf_service_response_timeout
            .as_millis()
            .div_ceil(1000);
        let server_conf = ServerConf {
            threads: self.config.threads.get(),
            grace_period_seconds: Some(grace as u64),
            graceful_shutdown_timeout_seconds: Some(GRACEFUL_SHUTDOWN_TIMEOUT_SECONDS),
            ..ServerConf::default()
        };
        let mut server = Server::new_with_opt_and_conf(None, server_conf);
        server.bootstrap();

        let listen_addr = self.config.bind_address.to_string();
        let cert_path = path_to_str(&self.config.tls_cert_path)?;
        let key_path = path_to_str(&self.config.tls_key_path)?;
        let tls = TlsSettings::intermediate(cert_path, key_path)
            .map_err(|e| anyhow!("Failed to configure TLS: {e}"))?;
        // OpenSSL silently drops a private key that does not match the certificate loaded after
        // it, so check their consistency explicitly to fail fast.
        tls.check_private_key()
            .map_err(|e| anyhow!("Failed to configure TLS: {e}"))?;
        TLS_CERTIFICATE_NOT_AFTER.set(certificate_not_after(cert_path)?);

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

/// The Unix time at which the served certificate, the first of the chain in `cert_path`, expires.
fn certificate_not_after(cert_path: &str) -> anyhow::Result<i64> {
    let pem = std::fs::read(cert_path).with_context(|| format!("Failed to read {cert_path}"))?;
    let cert = X509::from_pem(&pem).with_context(|| format!("Failed to parse {cert_path}"))?;
    let until_expiry = Asn1Time::from_unix(0)?.diff(cert.not_after())?;
    Ok(i64::from(until_expiry.days) * 86_400 + i64::from(until_expiry.secs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use rstest::rstest;
    use serial_test::serial;
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

    // Parsing reads the environment, which config.rs's `test_load_from_env` sets.
    #[rstest]
    #[serial(config_tests)]
    #[case::garbage_cert("tls.crt", "not a certificate".to_string())]
    #[case::garbage_key("tls.key", "not a private key".to_string())]
    #[case::mismatched_key("tls.key", key_pem())]
    fn build_server_rejects_invalid_tls_material(#[case] overwrite: &str, #[case] content: String) {
        let dir = tls_dir(overwrite, &content);
        let config = Config::parse_from([
            "coprocessor-proxy".as_ref(),
            "--tls-cert-path".as_ref(),
            dir.join("tls.crt").as_os_str(),
            "--tls-key-path".as_ref(),
            dir.join("tls.key").as_os_str(),
            "--ethereum-chain-id=1".as_ref(),
            "--protocol-config-address=0x0000000000000000000000000000000000000000".as_ref(),
            "--ethereum-rpc-url=http://localhost:8545".as_ref(),
            "--coprocessor-signer-address=0x0000000000000000000000000000000000000000".as_ref(),
            "--prf-service-address=127.0.0.1:1".as_ref(),
        ]);

        let (proxy, _state) = Proxy::from_config(config).unwrap();
        let error = match proxy.build_server() {
            Ok(_) => panic!("invalid TLS material was accepted"),
            Err(e) => e.to_string(),
        };
        assert!(error.contains("Failed to configure TLS"), "{error}");
    }

    #[test]
    fn certificate_not_after_reads_the_served_certificate() {
        // rcgen's certificates expire on 4096-01-01T00:00:00Z.
        let dir = tls_dir("unused", "");
        let not_after = certificate_not_after(path_to_str(&dir.join("tls.crt")).unwrap());
        assert_eq!(not_after.unwrap(), 67_090_118_400);
    }
}
