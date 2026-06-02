//! TTL'd snapshot of the on-chain Coprocessor registry used by the verifier.
//!
//! The verifier needs three things from `GatewayConfig` to evaluate an
//! attestation set: the set of authorized signer addresses, the per-Coprocessor
//! S3 bucket URLs to fan HEAD requests at, and the majority threshold. Querying
//! these on every decryption request would trigger N+1 RPC calls, so we cache a
//! whole snapshot behind a short TTL (see [`super::AttestationVerifier`]) and
//! tolerate registration changes within one refresh window.

use alloy::{primitives::Address, providers::Provider};
use fhevm_gateway_bindings::gateway_config::GatewayConfig::GatewayConfigInstance;
use futures::future::try_join_all;
use std::collections::{HashMap, HashSet};
use tracing::warn;

/// An immutable snapshot of the Coprocessor registry at one point in time.
///
/// Held behind an `Arc` so reads are snapshot-and-release: a reader clones the
/// `Arc` (one atomic increment), drops the lock guard, and runs the HEAD fan-out
/// while holding no lock.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CoprocessorRegistry {
    /// Addresses an attestation signature must recover to
    /// (`getCoprocessorSigners`).
    pub signers: HashSet<Address>,
    /// `txSender -> s3BucketUrl` for every registered Coprocessor
    /// (`getCoprocessorTxSenders` + `getCoprocessor(addr).s3BucketUrl`).
    pub tx_sender_to_bucket: HashMap<Address, String>,
    /// Number of agreeing signers required for consensus (`getCoprocessorMajorityThreshold`).
    pub threshold: usize,
}

impl CoprocessorRegistry {
    pub fn new(
        signers: HashSet<Address>,
        tx_sender_to_bucket: HashMap<Address, String>,
        threshold: usize,
    ) -> Self {
        Self {
            signers,
            tx_sender_to_bucket,
            threshold,
        }
    }

    /// Loads a fresh snapshot from the `GatewayConfig` contract.
    pub async fn load<P: Provider>(contract: &GatewayConfigInstance<P>) -> anyhow::Result<Self> {
        let get_copro_signers = contract.getCoprocessorSigners();
        let get_copro_tx_senders = contract.getCoprocessorTxSenders();
        let get_copro_threshold = contract.getCoprocessorMajorityThreshold();

        let (signers, tx_senders, threshold_u256) = tokio::try_join!(
            biased;
            get_copro_signers.call(),
            get_copro_tx_senders.call(),
            get_copro_threshold.call()
        )?;
        let threshold = threshold_u256.saturating_to::<usize>();

        let tx_sender_to_bucket = try_join_all(
            tx_senders
                .into_iter()
                .map(|tx_sender| async move { get_copro_bucket(contract, tx_sender).await }),
        )
        .await?
        .into_iter()
        .collect();

        Ok(Self::new(
            signers.into_iter().collect(),
            tx_sender_to_bucket,
            threshold,
        ))
    }
}

async fn get_copro_bucket<P: Provider>(
    contract: &GatewayConfigInstance<P>,
    copro_tx_sender_addr: Address,
) -> anyhow::Result<(Address, String)> {
    let copro = contract.getCoprocessor(copro_tx_sender_addr).call().await?;
    if copro.s3BucketUrl.is_empty() {
        warn!("No S3 bucket URL registered for Coprocessor {copro_tx_sender_addr}");
    }
    Ok((copro_tx_sender_addr, copro.s3BucketUrl))
}
