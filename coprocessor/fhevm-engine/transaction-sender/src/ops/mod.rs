use alloy::network::Ethereum;
use async_trait::async_trait;
use fhevm_engine_common::versioning::{begin_write_guarded, GcsRollbackPolicy};
use sqlx::{PgPool, Postgres, Transaction};

async fn begin_live_write(pool: &PgPool) -> anyhow::Result<Option<Transaction<'static, Postgres>>> {
    // The sender writes only after cutover.
    Ok(
        begin_write_guarded(pool, false, GcsRollbackPolicy::Continue)
            .await?
            .into_tx(),
    )
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
