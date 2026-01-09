//! V2: DISABLED - CiphertextCommits contract deleted in Gateway V2
//!
//! In V2, ciphertext material is served via HTTP API instead of being
//! committed on-chain. The CiphertextCommits contract no longer exists.
//!
//! See docs/gateway-v2-implementation/RESTRUCTURED_PLAN.md for details.

use super::TransactionOperation;
use crate::nonce_managed_provider::NonceManagedProvider;
use alloy::network::Ethereum;
use alloy::primitives::Address;
use alloy::providers::Provider;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use tracing::warn;

#[derive(Clone)]
pub struct AddCiphertextOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    #[allow(dead_code)]
    ciphertext_commits_address: Address,
    #[allow(dead_code)]
    provider: NonceManagedProvider<P>,
    conf: crate::ConfigSettings,
    #[allow(dead_code)]
    gas: Option<u64>,
    #[allow(dead_code)]
    db_pool: Pool<Postgres>,
}

impl<P> AddCiphertextOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    pub fn new(
        ciphertext_commits_address: Address,
        provider: NonceManagedProvider<P>,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
    ) -> Self {
        Self {
            db_pool,
            ciphertext_commits_address,
            provider,
            conf,
            gas,
        }
    }
}

#[async_trait]
impl<P> TransactionOperation<P> for AddCiphertextOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        &self.conf.add_ciphertexts_db_channel
    }

    async fn execute(&self) -> anyhow::Result<bool> {
        // V2: Disabled - CiphertextCommits contract deleted
        // Ciphertext material is now served via Coprocessor HTTP API
        warn!("add_ciphertext tx-sender DISABLED in V2 - CiphertextCommits contract removed");
        Ok(false)
    }
}
