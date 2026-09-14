//! Zama relayer over HTTP. Reading order: `kms_aggregator/docs.md`, then `kms_aggregator/aggregator.rs`.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic
    )
)]

pub mod endpoint;
pub mod kms_aggregator;
pub mod logging;
pub mod settings;

use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use kms_aggregator::{Aggregator, Caller, ConfigError, HttpClient, PublicDecrypt, UserDecrypt};
use settings::{HttpConfig, Settings};

/// Everything a request handler needs: the one shared state of the process. Built once in `main`, cloned per
/// request (Arcs only). Both aggregators share one `Caller`: one HTTP client, one call semaphore.
#[derive(Clone)]
pub struct App {
    pub user_decrypt: Arc<Aggregator<UserDecrypt>>,
    pub public_decrypt: Arc<Aggregator<PublicDecrypt>>,
    /// Bind address, body limit, supported chain ids.
    pub http: Arc<HttpConfig>,
    /// Cancelled on SIGINT/SIGTERM: `/healthz` answers 503 and every running aggregation ends.
    pub shutdown: CancellationToken,
}

impl App {
    /// Resolves the API keys from the environment, builds the shared HTTP client, semaphore and both aggregators.
    pub fn new(settings: &Settings, shutdown: CancellationToken) -> Result<Self, ConfigError> {
        let cfg = &settings.kms_aggregator;
        let client = Arc::new(HttpClient::new(cfg)?);
        let caller = Arc::new(Caller::new(cfg, client)?);
        Ok(Self {
            user_decrypt: Arc::new(Aggregator::new(
                caller.clone(),
                cfg.user_decrypt.threshold,
                cfg.user_decrypt.checks,
                shutdown.clone(),
            )),
            public_decrypt: Arc::new(Aggregator::new(
                caller,
                cfg.public_decrypt.threshold,
                (),
                shutdown.clone(),
            )),
            http: Arc::new(settings.http.clone()),
            shutdown,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kms_aggregator::config::AuthConfig;

    const EXAMPLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../config/config.yaml");

    #[test]
    fn app_needs_every_api_key() {
        let settings = Settings::load(EXAMPLE).unwrap();
        let e = App::new(&settings, CancellationToken::new()).err().unwrap();
        assert!(e.0.contains("KMS_00_API_KEY is not set"), "{e}");
    }

    #[test]
    fn app_builds_without_authentication() {
        let mut settings = Settings::load(EXAMPLE).unwrap();
        settings.kms_aggregator.allow_insecure_http = true;
        for endpoint in &mut settings.kms_aggregator.endpoints {
            endpoint.auth = AuthConfig::None;
        }
        let app = App::new(&settings, CancellationToken::new()).unwrap();
        let _shared = app.clone();
    }
}
