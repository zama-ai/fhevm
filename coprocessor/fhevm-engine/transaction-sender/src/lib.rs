pub mod config;
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
use anyhow::Error;
pub use config::ConfigSettings;
pub use fhevm_engine_common::gateway_http::gateway_http_client;
use fhevm_engine_common::gateway_http::is_gateway_provider_exhausted_transport_error;
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

// Gets the chain ID from the given Gateway HTTP RPC URL.
// This is a utility function that will try to connect until it succeeds.
pub async fn get_chain_id(gateway_url: Url, retry_interval: Duration) -> u64 {
    loop {
        let provider = ProviderBuilder::new().connect_client(gateway_http_client(
            &gateway_url,
            1,
            retry_interval,
        ));

        match provider.get_chain_id().await {
            Ok(chain_id) => {
                tracing::info!(chain_id = chain_id, "Found chain ID");
                return chain_id;
            }
            Err(e) => {
                error!(
                    gateway_url = %gateway_url,
                    error = %e,
                    retry_interval = ?retry_interval,
                    "Failed to get chain ID from Gateway, retrying"
                );
                tokio::time::sleep(retry_interval).await;
            }
        }
    }
}

pub fn is_gateway_provider_exhausted(err: &Error) -> bool {
    err.chain().any(|cause| {
        if let Some(t) = cause.downcast_ref::<TransportError>() {
            matches!(t, TransportError::Transport(inner) if is_gateway_provider_exhausted_transport_error(inner))
        } else {
            false
        }
    })
}
