use fhevm_engine_common::healthz_server::{
    default_get_version, HealthCheckService, HealthStatus, Version,
};

/// The health of the proxy, served on the healthcheck port.
pub struct State(fhevm_sig_proxy::health::State);

impl State {
    pub fn new(state: fhevm_sig_proxy::health::State) -> Self {
        Self(state)
    }
}

impl HealthCheckService for State {
    /// Healthy when the TLS listener answers, the KMS tx-senders have been read once and some
    /// upstream has a healthy endpoint.
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        let tls_listener = self.0.tls_listener_healthcheck().await;
        let signers_loaded = self.0.signers_loaded();
        let endpoints = self.0.endpoints_health();

        status.set_custom_check("tls_listener", tls_listener.is_ok(), true);
        status.set_custom_check("kms_tx_senders", signers_loaded, true);
        status.set_custom_check("endpoints", endpoints.any_upstream_healthy(), true);
        if let Err(e) = tls_listener {
            status.add_error_details(e);
        }
        if !signers_loaded {
            status.add_error_details("KMS tx-senders not read yet".to_owned());
        }
        for upstream in &endpoints.0 {
            if !upstream.unhealthy.is_empty() {
                status.add_error_details(format!(
                    "Unhealthy endpoints of {}: {}",
                    upstream.name,
                    upstream.unhealthy.join(", ")
                ));
            }
        }
        status
    }

    async fn is_alive(&self) -> bool {
        true
    }

    fn get_version(&self) -> Version {
        default_get_version()
    }
}
