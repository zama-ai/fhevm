pub mod config;
pub mod http_server;
mod nonce_managed_provider;
mod ops;
mod transaction_sender;

use std::sync::Arc;
use std::time::Duration;

use alloy::network::TxSigner;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;
use alloy::providers::WsConnect;
use alloy::signers::Signature;
use alloy::signers::Signer;
use alloy::transports::http::reqwest::Url;
pub use config::ConfigSettings;
pub use nonce_managed_provider::FillersWithoutNonceManagement;
pub use nonce_managed_provider::NonceManagedProvider;
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

pub async fn get_chain_id(
    ws_url: Url,
    max_retries: u32,
    retry_interval: Duration,
) -> anyhow::Result<u64> {
    let provider = ProviderBuilder::new()
        .connect_ws(
            WsConnect::new(ws_url)
                .with_max_retries(max_retries)
                .with_retry_interval(retry_interval),
        )
        .await?;
    Ok(provider.get_chain_id().await?)
}
