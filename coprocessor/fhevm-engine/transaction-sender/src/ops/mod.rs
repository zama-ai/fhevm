use alloy::network::Ethereum;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionOperation<P>: Send + Sync
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str;

    async fn execute(&self) -> anyhow::Result<bool>;

    /// Get a reference to the provider
    fn provider(&self) -> &P;

    /// Check if the provider connection is healthy
    async fn check_provider_connection(&self) -> anyhow::Result<()> {
        // Default implementation for checking provider connection
        let _ = self.provider().get_block_number().await?;
        Ok(())
    }
}

pub(crate) mod add_ciphertext;
pub(crate) mod allow_handle;
pub(crate) mod verify_proof;

mod common;
