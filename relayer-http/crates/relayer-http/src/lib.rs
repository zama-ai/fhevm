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

pub mod config;
pub mod endpoint;
pub mod kms_aggregator;
pub mod logging;

use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use crate::config::{HttpConfig, RelayerConfig};
use kms_aggregator::{
    Aggregator, Caller, ConfigError, HttpClient, PublicDecrypt, RequestSigner, UserDecrypt,
};

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
    /// Loads the signing key (env var or AWS KMS), builds the shared HTTP client, semaphore and both aggregators.
    pub async fn new(
        config: &RelayerConfig,
        shutdown: CancellationToken,
    ) -> Result<Self, ConfigError> {
        let kms = &config.kms_aggregator;
        let client = Arc::new(HttpClient::new(kms)?);
        let signer = Arc::new(RequestSigner::from_config(&kms.auth).await?);
        tracing::info!(address = %signer.address(), "FhevmSig signer loaded");
        let caller = Arc::new(Caller::new(kms, client, signer)?);
        Ok(Self {
            user_decrypt: Arc::new(Aggregator::new(
                caller.clone(),
                kms.user_decrypt.threshold,
                kms.user_decrypt.checks,
                shutdown.clone(),
            )),
            public_decrypt: Arc::new(Aggregator::new(
                caller,
                kms.public_decrypt.threshold,
                (),
                shutdown.clone(),
            )),
            http: Arc::new(config.http.clone()),
            shutdown,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kms_aggregator::config::SignerConfig;

    const EXAMPLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../config/config.yaml");

    #[tokio::test]
    async fn app_needs_the_signing_key() {
        let mut config = RelayerConfig::load(EXAMPLE).unwrap();
        config.kms_aggregator.auth.signer = SignerConfig::PrivateKey {
            value_env: "RELAYER_HTTP_TEST_UNSET_KEY".to_owned(),
        };
        let e = App::new(&config, CancellationToken::new())
            .await
            .err()
            .unwrap();
        assert!(
            e.0.contains("RELAYER_HTTP_TEST_UNSET_KEY is not set"),
            "{e}"
        );
    }
}
