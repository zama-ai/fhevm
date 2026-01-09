//! V2: DISABLED - MultichainACL contract deleted in Gateway V2
//!
//! In V2, ACL is managed on Host Chain directly, not via Gateway MultichainACL.
//! The MultichainACL contract no longer exists.
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
pub struct AllowHandleOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    #[allow(dead_code)]
    multichain_acl_address: Address,
    #[allow(dead_code)]
    provider: NonceManagedProvider<P>,
    conf: crate::ConfigSettings,
    #[allow(dead_code)]
    gas: Option<u64>,
    #[allow(dead_code)]
    db_pool: Pool<Postgres>,
}

impl<P> AllowHandleOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    pub fn new(
        multichain_acl_address: Address,
        provider: NonceManagedProvider<P>,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
    ) -> Self {
        Self {
            multichain_acl_address,
            provider,
            conf,
            gas,
            db_pool,
        }
    }
}

#[async_trait]
impl<P> TransactionOperation<P> for AllowHandleOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        &self.conf.allow_handle_db_channel
    }

    async fn execute(&self) -> anyhow::Result<bool> {
        // V2: Disabled - MultichainACL contract deleted
        // ACL is now managed on Host Chain directly
        warn!("allow_handle tx-sender DISABLED in V2 - MultichainACL contract removed");
        Ok(false)
    }
}
