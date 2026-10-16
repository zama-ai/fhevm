use actix_web::http::StatusCode;
use connector_utils::monitoring::health::Healthcheck;
use serde::{Deserialize, Serialize};

/// The struct used to monitor the health of the `Proxy`.
#[derive(Clone)]
pub struct State(fhevm_sig_proxy::health::State);

impl State {
    pub fn new(state: fhevm_sig_proxy::health::State) -> Self {
        Self(state)
    }
}

impl Healthcheck for State {
    async fn healthcheck(&self) -> actix_web::HttpResponse {
        let tls_listener_result = self.0.tls_listener_healthcheck().await;
        let endpoints_health = self.0.endpoints_health();
        let endpoints_serving = endpoints_health.any_upstream_healthy();
        let healthy_endpoints = endpoints_health.healthy_endpoints();
        let unhealthy_endpoints = endpoints_health.unhealthy_endpoints();

        let mut errors = vec![];
        let tls_listener_reachable = tls_listener_result.map_err(|e| errors.push(e)).is_ok();
        if !unhealthy_endpoints.is_empty() {
            errors.push(format!(
                "Unhealthy endpoints: {}",
                unhealthy_endpoints.join(", ")
            ));
        }
        if !endpoints_serving {
            errors.push("No healthy endpoint available".to_string());
        }

        let healthy = tls_listener_reachable && endpoints_serving;
        let status_code = if healthy {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        };

        let status = HealthStatus {
            healthy,
            tls_listener_reachable,
            healthy_endpoints,
            unhealthy_endpoints,
            details: errors.join("; "),
        };

        actix_web::HttpResponse::build(status_code).json(status)
    }

    fn service_name() -> &'static str {
        "kms-connector-proxy"
    }
}

/// Serializable representation of `Proxy`'s health status.
#[derive(Debug, Deserialize, Serialize)]
pub struct HealthStatus {
    /// Overall health of the service: the TLS listener answers and at least one endpoint is
    /// healthy.
    pub healthy: bool,
    /// Whether the public TLS listener accepts connections and answers requests.
    pub tls_listener_reachable: bool,
    /// The endpoints currently receiving requests.
    pub healthy_endpoints: Vec<String>,
    /// The endpoints that failed their healthcheck. Non-empty is degraded, but not unhealthy.
    pub unhealthy_endpoints: Vec<String>,
    /// Details about any issues encountered during healthcheck.
    pub details: String,
}
