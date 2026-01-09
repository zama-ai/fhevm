//! V2: DISABLED - verify_proof responses now served via Coprocessor HTTP API
//!
//! This module is disabled in Gateway V2. Input verification responses are
//! served via the HTTP API (`GET /v1/ciphertext/:handle`) instead of on-chain
//! transactions.
//!
//! See docs/gateway-v2-implementation/RESTRUCTURED_PLAN.md Phase 2 for details.
//!
//! Original implementation is preserved below (commented out) for reference
//! and potential rollback needs.

use super::TransactionOperation;
use crate::nonce_managed_provider::NonceManagedProvider;
use crate::AbstractSigner;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::network::Ethereum;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use tracing::warn;

#[derive(Clone)]
pub(crate) struct VerifyProofOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    #[allow(dead_code)]
    input_verification_address: Address,
    #[allow(dead_code)]
    provider: NonceManagedProvider<P>,
    #[allow(dead_code)]
    signer: AbstractSigner,
    conf: crate::ConfigSettings,
    #[allow(dead_code)]
    gas: Option<u64>,
    #[allow(dead_code)]
    gw_chain_id: u64,
    #[allow(dead_code)]
    db_pool: Pool<Postgres>,
}

impl<P> VerifyProofOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    pub(crate) async fn new(
        input_verification_address: Address,
        provider: NonceManagedProvider<P>,
        signer: AbstractSigner,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
    ) -> anyhow::Result<Self> {
        let gw_chain_id = provider.get_chain_id().await?;
        Ok(Self {
            input_verification_address,
            provider,
            signer,
            conf,
            gas,
            gw_chain_id,
            db_pool,
        })
    }
}

#[async_trait]
impl<P> TransactionOperation<P> for VerifyProofOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        &self.conf.verify_proof_resp_db_channel
    }

    async fn execute(&self) -> anyhow::Result<bool> {
        // V2: Disabled - responses now served via Coprocessor HTTP API
        // See docs/gateway-v2-implementation/RESTRUCTURED_PLAN.md Phase 2
        warn!("verify_proof tx-sender DISABLED in V2 - responses served via Coprocessor HTTP API");

        // Return false to indicate no work was done (no more work to process)
        Ok(false)
    }
}

// =============================================================================
// ORIGINAL V1 IMPLEMENTATION (preserved for reference)
// =============================================================================
//
// The original implementation handled:
// 1. Querying `verify_proofs` table for pending proofs
// 2. Building `verifyProofResponse` or `rejectProofResponse` transactions
// 3. Signing with EIP-712 domain
// 4. Sending transactions to Gateway InputVerification contract
// 5. Handling CoprocessorAlreadyVerified/Rejected errors
// 6. Retry logic with exponential backoff
//
// Key dependencies (now removed):
// - fhevm_gateway_bindings::InputVerification (deleted in V2)
// - fhevm_gateway_bindings::InputVerification::CiphertextVerification
// - fhevm_gateway_bindings::InputVerification::InputVerificationErrors
//
// Original imports:
// use alloy::primitives::{FixedBytes, U256};
// use alloy::rpc::types::TransactionRequest;
// use fhevm_gateway_bindings::InputVerification::{
//     self, CiphertextVerification, InputVerificationErrors,
// };
// use std::time::Duration;
// use telemetry::VERIFY_PROOF_FAIL_COUNTER;
// use telemetry::VERIFY_PROOF_SUCCESS_COUNTER;
// use tokio::task::JoinSet;
// use tracing::{debug, error, info};
//
// Original execute() implementation:
// async fn execute(&self) -> anyhow::Result<bool> {
//     let input_verification =
//         InputVerification::new(self.input_verification_address, self.provider.inner());
//     if self.conf.verify_proof_remove_after_max_retries {
//         self.remove_proofs_by_retry_count().await?;
//     }
//     let rows = sqlx::query!(
//         "SELECT zk_proof_id, chain_id, contract_address, user_address, handles, verified, retry_count, extra_data, transaction_id
//          FROM verify_proofs
//          WHERE verified IS NOT NULL AND retry_count < $1
//          ORDER BY zk_proof_id
//          LIMIT $2",
//         self.conf.verify_proof_resp_max_retries as i64,
//         self.conf.verify_proof_resp_batch_limit as i64
//     )
//     .fetch_all(&self.db_pool)
//     .await?;
//     // ... rest of implementation
// }
