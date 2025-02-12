use alloy::network::Ethereum;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait TransactionOperation<P>: Send + Sync
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str;

    async fn execute(&self, db_pool: &Pool<Postgres>) -> anyhow::Result<bool>;
}

pub(crate) mod add_ciphertexts;
pub(crate) mod verify_proofs;
