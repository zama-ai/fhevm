use alloy::network::Ethereum;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionOperation<P>: Send + Sync
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str;

    async fn execute(&self) -> anyhow::Result<bool>;
}

pub(crate) mod add_ciphertext;
pub(crate) mod allow_handle;
pub(crate) mod delegate_user_decrypt;
pub(crate) mod verify_proof;

mod common;
