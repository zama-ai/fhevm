pub mod config;
pub mod diagnostics;
pub mod http_server;
pub mod metrics;
mod nonce_managed_provider;
mod ops;
mod transaction_sender;

use std::sync::Arc;
use std::time::Duration;

use alloy::network::TxSigner;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;
use alloy::signers::Signature;
use alloy::signers::Signer;
use alloy::transports::http::reqwest::Url;
use alloy::transports::TransportError;
use alloy::transports::TransportErrorKind;
use anyhow::Error;
pub use config::ConfigSettings;
pub use nonce_managed_provider::FillersWithoutNonceManagement;
pub use nonce_managed_provider::NonceManagedProvider;
use tracing::error;
pub use transaction_sender::TransactionSender;

pub const REVIEW: &str = "review";

// A signer that can both sign transactions and messages. Only needed for `AbstractSigner` (see below).
pub trait CombinedSigner: TxSigner<Signature> + Signer<Signature> {}
impl<T: TxSigner<Signature> + Signer<Signature>> CombinedSigner for T {}

// A thread-safe abstract signer that can sign both transactions and messages.
pub type AbstractSigner = Arc<dyn CombinedSigner + Send + Sync>;

pub fn make_abstract_signer<S>(signer: S) -> AbstractSigner
where
    S: CombinedSigner + Send + Sync + 'static,
{
    Arc::new(signer)
}

/// Represents the health status of the transaction sender service
#[derive(Debug)]
pub struct HealthStatus {
    /// Overall health of the service
    pub healthy: bool,
    /// Database connection status
    pub database_connected: bool,
    /// Blockchain provider connection status
    pub blockchain_connected: bool,
    /// Details about any issues encountered during health check
    pub details: Option<String>,
}

impl HealthStatus {
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            database_connected: true,
            blockchain_connected: true,
            details: None,
        }
    }

    pub fn unhealthy(
        database_connected: bool,
        blockchain_connected: bool,
        details: String,
    ) -> Self {
        Self {
            healthy: false,
            database_connected,
            blockchain_connected,
            details: Some(details),
        }
    }
}

/// Pooled HTTP client used for the Gateway RPCs.
///
/// The Gateway moved from a WebSocket endpoint to an HTTP one. Everything else
/// about the sender is unchanged, so the policy here is deliberately plain:
/// bounded connect and request times, no redirects, and no transport-level
/// retry. A retried submission is indistinguishable from a duplicate one, and
/// recovery already belongs to the operation retry loop.
pub fn gateway_http_client(url: &Url) -> anyhow::Result<alloy::transports::http::reqwest::Client> {
    anyhow::ensure!(
        matches!(url.scheme(), "http" | "https"),
        "transaction sender requires an http:// or https:// Gateway URL"
    );
    Ok(alloy::transports::http::reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(4))
        .timeout(Duration::from_secs(30))
        .redirect(alloy::transports::http::reqwest::redirect::Policy::none())
        .retry(alloy::transports::http::reqwest::retry::never())
        .build()?)
}

// Gets the chain ID from the given Gateway URL.
// This is a utility function that will try to connect until it succeeds.
pub async fn get_chain_id(url: Url, retry_interval: Duration) -> anyhow::Result<u64> {
    // A bad scheme is returned, not retried: it will never become good, and
    // retrying one forever is the failure mode this replaces.
    let provider = ProviderBuilder::new().connect_reqwest(gateway_http_client(&url)?, url);
    loop {
        match provider.get_chain_id().await {
            Ok(chain_id) => {
                tracing::info!(chain_id = chain_id, "Found chain ID");
                return Ok(chain_id);
            }
            Err(e) => {
                error!(
                    error = %diagnostics::safe_rpc_error(&e),
                    retry_interval = ?retry_interval,
                    "Failed to get chain ID from Gateway, retrying"
                );
                tokio::time::sleep(retry_interval).await;
            }
        }
    }
}

pub fn is_backend_gone(err: &Error) -> bool {
    err.chain().any(|cause| {
        if let Some(t) = cause.downcast_ref::<TransportError>() {
            matches!(
                t,
                TransportError::Transport(TransportErrorKind::BackendGone)
            )
        } else {
            false
        }
    })
}
