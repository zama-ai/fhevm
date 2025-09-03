use async_trait::async_trait;

#[async_trait]
pub trait TransactionOperation: Send + Sync {
    fn channel(&self) -> &str;

    async fn execute(&self) -> anyhow::Result<bool>;
}

pub(crate) mod add_ciphertext;
pub(crate) mod allow_handle;
pub(crate) mod verify_proof;

mod common;
