use connector_utils::{
    config::DeserializeConfig,
    monitoring::{health::default_healthcheck_timeout, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;
use std::{net::SocketAddr, time::Duration};

/// Configuration of the `Proxy` service.
// TODO: this is a dummy configuration. Update it once the `Proxy` service is implemented.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(Serialize))]
pub struct Config {
    /// The service name used for tracing.
    #[serde(default = "default_service_name")]
    pub service_name: String,
    /// The maximum number of tasks that can be executed concurrently.
    #[serde(default = "default_task_limit")]
    pub task_limit: usize,
    /// The monitoring server endpoint of the `Proxy` service.
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: SocketAddr,
    /// The timeout to perform each external service connection healthcheck.
    #[serde(with = "humantime_serde", default = "default_healthcheck_timeout")]
    pub healthcheck_timeout: Duration,
}

impl DeserializeConfig for Config {}

fn default_service_name() -> String {
    "kms-connector-proxy".to_string()
}

// Default implementation for testing purpose
impl Default for Config {
    fn default() -> Self {
        Self {
            service_name: default_service_name(),
            task_limit: default_task_limit(),
            monitoring_endpoint: default_monitoring_endpoint(),
            healthcheck_timeout: default_healthcheck_timeout(),
        }
    }
}
