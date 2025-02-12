use super::TransactionOperation;
use alloy::{network::Ethereum, primitives::Address, providers::Provider, sol};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

sol!(
    #[sol(rpc)]
    CiphertextStorage,
    "artifacts/CiphertextStorage.sol/CiphertextStorage.json"
);

#[derive(Clone)]
pub struct AddCiphertextsOperation<P: Provider<Ethereum> + Clone + 'static> {
    ciphertext_storage_address: Address,
    provider: Arc<P>,
}

impl<P: Provider<Ethereum> + Clone + 'static> AddCiphertextsOperation<P> {
    pub fn new(ciphertext_storage_address: Address, provider: Arc<P>) -> Self {
        Self {
            ciphertext_storage_address,
            provider,
        }
    }
}

#[async_trait]
impl<P> TransactionOperation<P> for AddCiphertextsOperation<P>
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        "add_ciphertexts"
    }

    async fn execute(&self, _db_pool: &Pool<Postgres>) -> anyhow::Result<bool> {
        let _ciphertext_storage =
            CiphertextStorage::new(self.ciphertext_storage_address, &self.provider);
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        Ok(false)
    }
}
