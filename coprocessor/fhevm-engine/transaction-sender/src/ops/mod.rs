use alloy::network::Ethereum;
use async_trait::async_trait;
use fhevm_engine_common::versioning::{begin_write_guarded, GcsRollbackPolicy};
use sqlx::{PgPool, Postgres, Transaction};

async fn begin_live_write(pool: &PgPool) -> anyhow::Result<Option<Transaction<'static, Postgres>>> {
    // A claimed task must not be abandoned in SENDING during containment.
    // Park until recovery allows writes again; retirement still returns None
    // so the successor can recover the claim under exclusive ownership.
    loop {
        let tx = begin_write_guarded(pool, false, GcsRollbackPolicy::Continue)
            .await?
            .into_tx();
        if tx.is_some() || !fhevm_engine_common::chain_recovery::is_halted(pool).await? {
            return Ok(tx);
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

#[async_trait]
pub trait TransactionOperation<P>: Send + Sync
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str;

    async fn execute(&self) -> anyhow::Result<bool>;
}

pub(crate) mod add_ciphertext;
pub(crate) mod verify_proof;

mod common;
pub(crate) mod state;
