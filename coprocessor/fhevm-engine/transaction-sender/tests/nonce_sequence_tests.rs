//! Regression tests for the nonce sequence used by `NonceManagedProvider`.
//!
//! Both cases below fail on the pre-hotfix implementation, which used alloy's
//! `CachedNonceManager` (seeded from `latest`, so blind to the mempool) and
//! reset that manager on *any* send error.

mod common;

use alloy::consensus::Transaction;
use alloy::network::TransactionBuilder;
use alloy::primitives::U256;
use alloy::providers::ext::AnvilApi;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::TransactionRequest;
use common::{SignerType, TestEnvironment};
use serial_test::serial;
use std::time::Duration;
use transaction_sender::{ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider};

/// With mining paused, every attempt exceeds its receipt deadline even though the
/// transaction did reach the mempool. The next send must therefore continue the
/// sequence rather than reuse the pending nonce: submission was acknowledged, so
/// the nonce is spent regardless of what inclusion does.
///
/// Pre-hotfix behaviour: the failed send reset the nonce manager, which re-seeded
/// from `latest` (mined only) and handed out the *same* nonce again, producing
/// `already known` / `replacement transaction underpriced` / `nonce too low`.
#[tokio::test]
#[serial(db)]
async fn nonce_sequence_does_not_reuse_after_send_timeout() -> anyhow::Result<()> {
    let env = TestEnvironment::new_with_config_and_anvil_args(
        SignerType::PrivateKey,
        ConfigSettings::default(),
        false,
        &["--no-mining"],
    )
    .await?;

    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.signer_address()),
    );

    let start_nonce = provider.get_transaction_count(env.signer_address()).await?;

    // Two sends that both time out because nothing is being mined.
    for _ in 0..2 {
        let txn = TransactionRequest::default()
            .with_to(env.user_address)
            .with_value(U256::from(1))
            .with_gas_limit(21_000);
        let err = provider
            .send_transaction_sync(txn, Duration::from_secs(2), Duration::from_secs(2))
            .await
            .expect_err("the receipt must time out while mining is paused");
        let err = err.to_string();
        assert!(
            !err.contains("already known")
                && !err.contains("underpriced")
                && !err.contains("nonce too low"),
            "second send reused a pending nonce: {err}"
        );
    }

    // Both transactions must be in the pool on consecutive nonces, with no hole.
    provider.inner().anvil_mine(Some(1), None).await?;
    let mut nonces = Vec::new();
    let head = provider.inner().get_block_number().await?;
    for number in 0..=head {
        if let Some(block) = provider
            .inner()
            .get_block_by_number(number.into())
            .full()
            .await?
        {
            for txn in block.transactions.txns() {
                if txn.inner.signer() == env.signer_address() {
                    nonces.push(txn.inner.nonce());
                }
            }
        }
    }
    nonces.sort_unstable();
    assert_eq!(
        nonces,
        vec![start_nonce, start_nonce + 1],
        "expected two transactions on consecutive nonces, got {nonces:?}"
    );

    Ok(())
}

/// A send rejected before it reaches the mempool must not consume a nonce, so the
/// sequence continues from the same value and leaves no gap for the next send.
/// A gap would be rejected as `nonce too high` by any node that does not queue
/// gapped transactions.
///
/// NOTE: unlike the test above, this one also passes on the pre-hotfix
/// implementation — resetting the manager and re-seeding from `latest` happened
/// to yield the same nonce in this single-send scenario. It is a guard against
/// a future change that *rewinds* the sequence instead of invalidating it, not a
/// reproduction of the original defect. Reproducing the gap itself needs a
/// concurrent wave, which the serialisation in `NonceManagedProvider` now makes
/// unreachable by construction; see the doc comment on `nonce_seq`.
#[tokio::test]
#[serial(db)]
async fn nonce_sequence_leaves_no_gap_after_rejected_send() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;

    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.signer_address()),
    );

    let start_nonce = provider.get_transaction_count(env.signer_address()).await?;

    // Rejected at submission: the gas limit exceeds anything the block can hold,
    // so this never enters the mempool and must not consume a nonce.
    let doomed = TransactionRequest::default()
        .with_to(env.user_address)
        .with_value(U256::from(1))
        .with_gas_limit(u64::MAX / 2);
    provider
        .send_transaction_sync(doomed, Duration::from_secs(4), Duration::from_secs(4))
        .await
        .expect_err("transaction over the block gas limit must be rejected");

    // The next send has to take the nonce the rejected one did not use.
    let good = TransactionRequest::default()
        .with_to(env.user_address)
        .with_value(U256::from(1))
        .with_gas_limit(21_000);
    let receipt = provider
        .send_transaction_sync(good, Duration::from_secs(4), Duration::from_secs(30))
        .await?;
    assert!(receipt.status(), "follow-up transaction should succeed");

    let txn = provider
        .inner()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .expect("mined transaction is retrievable");
    assert_eq!(
        txn.inner.nonce(),
        start_nonce,
        "rejected send must not consume a nonce or leave a gap"
    );

    Ok(())
}
