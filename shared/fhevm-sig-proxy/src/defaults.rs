//! The defaults of the [`crate::ProxyConfig`] settings every deployment exposes, for their
//! configuration loaders, e.g. `#[serde(default = "fhevm_sig_proxy::defaults::request_read_timeout")]`.

use std::time::Duration;

pub fn endpoint_connect_timeout() -> Duration {
    Duration::from_secs(3)
}

pub fn endpoint_healthcheck_frequency() -> Duration {
    Duration::from_secs(3)
}

pub fn endpoint_idle_timeout() -> Duration {
    // Short on purpose: a Kubernetes ClusterIP Service picks the target pod once per TCP
    // connection, not per request, so all requests sent over a long-lived pooled connection
    // keep landing on the same pod. A short idle timeout forces the pool to reconnect
    // regularly, letting the Service's balancing reach newly scaled-up or restarted pods.
    Duration::from_secs(10)
}

pub fn request_read_timeout() -> Duration {
    Duration::from_secs(30)
}

pub fn shutdown_grace_period() -> Duration {
    Duration::from_secs(60)
}
