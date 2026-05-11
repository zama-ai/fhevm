//! Prometheus metrics exporter bootstrap.
//!
//! Installs the `metrics-exporter-prometheus` recorder and starts an HTTP
//! server that serves the `/metrics` endpoint for Prometheus scraping.
//!
//! Application crates emit metrics via the `metrics` facade crate directly
//! (e.g., `metrics::counter!("broker_messages_published_total")`) — they do
//! not depend on this crate.

#![deny(clippy::correctness)]
#![warn(clippy::suspicious, clippy::style, clippy::complexity, clippy::perf)]

use std::net::SocketAddr;

use metrics_exporter_prometheus::PrometheusBuilder;
use thiserror::Error;

/// Configuration for the metrics HTTP endpoint.
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// Address to bind the metrics HTTP server. Default: `0.0.0.0:9090`
    pub listen_addr: SocketAddr,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            listen_addr: SocketAddr::from(([0, 0, 0, 0], 9090)),
        }
    }
}

/// Errors that can occur during metrics initialization.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum TelemetryError {
    /// Failed to install the Prometheus recorder.
    /// Happens if a global recorder is already installed (double-init)
    /// or if the HTTP server fails to bind.
    #[error("failed to install prometheus exporter: {0}")]
    Install(String),
}

/// Install the Prometheus exporter and start the HTTP server on `/metrics`.
///
/// Call once at startup, after loading config, before any subsystem that emits
/// metrics. The HTTP server runs until the tokio runtime shuts down.
///
/// # Errors
///
/// Returns [`TelemetryError::Install`] if:
/// - A global metrics recorder is already installed (call this once)
/// - The HTTP server fails to bind to the configured address
pub fn init_metrics(config: MetricsConfig) -> Result<(), TelemetryError> {
    PrometheusBuilder::new()
        .with_http_listener(config.listen_addr)
        .install()
        .map_err(|e| TelemetryError::Install(e.to_string()))?;

    tracing::info!(addr = %config.listen_addr, "Prometheus metrics endpoint started");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // AC-1.1
    #[test]
    fn metrics_config_default_binds_to_9090() {
        let config = MetricsConfig::default();
        assert_eq!(config.listen_addr, SocketAddr::from(([0, 0, 0, 0], 9090)));
    }
}
