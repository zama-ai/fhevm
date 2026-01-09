//! V2: DISABLED - MultichainACL contract deleted in Gateway V2
//!
//! In V2, user decryption delegation is handled differently.
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
pub struct DelegateUserDecryptOperation<P: Provider<Ethereum> + Clone + 'static> {
    #[allow(dead_code)]
    multichain_acl_address: Address,
    #[allow(dead_code)]
    gateway_provider: NonceManagedProvider<P>,
    #[allow(dead_code)]
    host_chain_provider: P,
    conf: crate::ConfigSettings,
    #[allow(dead_code)]
    gas: Option<u64>,
    #[allow(dead_code)]
    db_pool: Pool<Postgres>,
    #[allow(dead_code)]
    cancel_token: tokio_util::sync::CancellationToken,
}

impl<P: Provider<Ethereum> + Clone + 'static> DelegateUserDecryptOperation<P> {
    pub fn new(
        multichain_acl_address: Address,
        gateway_provider: NonceManagedProvider<P>,
        host_chain_provider: P,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
        cancel_token: tokio_util::sync::CancellationToken,
    ) -> Self {
        Self {
            multichain_acl_address,
            gateway_provider,
            host_chain_provider,
            conf,
            gas,
            db_pool,
            cancel_token,
        }
    }
}

#[async_trait]
impl<P> TransactionOperation<P> for DelegateUserDecryptOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        // host-listener/src/database/tfhe_event_propagate.rs
        "new_host_block"
    }

    async fn execute(&self) -> anyhow::Result<bool> {
        // V2: Disabled - MultichainACL contract deleted
        // User decryption delegation is handled via Host Chain directly
        warn!("delegate_user_decrypt tx-sender DISABLED in V2 - MultichainACL contract removed");
        Ok(false)
    }
}
