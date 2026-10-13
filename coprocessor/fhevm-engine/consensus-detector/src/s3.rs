//! S3 bucket URL resolver, modeled on kms-worker's `S3Service`
//! (`fhevm/kms-connector/crates/kms-worker/src/core/event_processor/s3.rs`).
//!
//! Wraps a `GatewayConfigInstance` and a `DashMap` cache keyed by coprocessor
//! address. Cache hits avoid the on-chain round-trip. The cache is scoped to
//! a single `S3Service` instance (kms-worker uses a global `LazyLock`; we keep
//! it on the instance because consensus-detector runs one service per process).

use alloy::{eips::BlockId, network::Ethereum, primitives::Address, providers::Provider};
use anyhow::anyhow;
use dashmap::DashMap;
use fhevm_gateway_bindings::gateway_config::GatewayConfig::{self, GatewayConfigInstance};
use tracing::{debug, info, warn};

/// Resolver for coprocessor S3 bucket URLs via the on-chain `GatewayConfig`
/// contract. Mirrors the cache + bulk-fetch shape used by
/// `kms-worker::S3Service`.
#[derive(Clone)]
pub struct S3Service<P: Provider<Ethereum>> {
    gateway_config_contract: GatewayConfigInstance<P>,
    cache: DashMap<Address, String>,
}

impl<P> S3Service<P>
where
    P: Provider<Ethereum>,
{
    pub fn new(provider: P, gateway_config_address: Address) -> Self {
        let gateway_config_contract = GatewayConfig::new(gateway_config_address, provider);
        Self {
            gateway_config_contract,
            cache: DashMap::new(),
        }
    }

    /// Fetch the current coprocessor signer set from `GatewayConfig`.
    pub async fn get_coprocessor_signers(&self) -> anyhow::Result<Vec<Address>> {
        let signers = self
            .gateway_config_contract
            .getCoprocessorSigners()
            .call()
            .await
            .map_err(|e| anyhow!("getCoprocessorSigners failed: {e}"))?;
        Ok(signers)
    }

    /// The members as of one block: each coprocessor's signer with the bucket it
    /// registered. `GatewayConfig` keys `coprocessors` by tx-sender address, so
    /// this walks the tx senders and takes the signer from each registration.
    /// Both reads name the one block, so every operator resolves one set, and a
    /// coprocessor dropped later keeps the bucket it used.
    pub async fn get_members_at(&self, block: u64) -> anyhow::Result<Vec<(Address, String)>> {
        let at = BlockId::number(block);
        let tx_senders = self
            .gateway_config_contract
            .getCoprocessorTxSenders()
            .block(at)
            .call()
            .await
            .map_err(|e| anyhow!("getCoprocessorTxSenders at block {block} failed: {e}"))?;

        let mut members = Vec::with_capacity(tx_senders.len());
        for tx_sender in tx_senders {
            let coprocessor = self
                .gateway_config_contract
                .getCoprocessor(tx_sender)
                .block(at)
                .call()
                .await
                .map_err(|e| anyhow!("getCoprocessor({tx_sender}) at block {block} failed: {e}"))?;
            if coprocessor.s3BucketUrl.is_empty() {
                return Err(anyhow!(
                    "no S3 bucket URL for coprocessor {tx_sender} at block {block}"
                ));
            }
            members.push((coprocessor.signerAddress, coprocessor.s3BucketUrl));
        }
        Ok(members)
    }
    /// Resolve `s3BucketUrl` for one coprocessor, populating the cache on miss.
    ///
    /// Returns `Err` for RPC failures and when the contract has no S3 URL
    /// registered for `copro_addr`.
    pub async fn get_coprocessor_s3_url(&self, copro_addr: Address) -> anyhow::Result<String> {
        log_cache(&self.cache, "S3 cache state before S3 URL fetching");
        if let Some(url) = self.cache.get(&copro_addr) {
            info!(
                copro = %copro_addr,
                url = %url.value(),
                "CACHE HIT: using cached S3 bucket URL"
            );
            return Ok(url.value().clone());
        }

        info!(
            copro = %copro_addr,
            "CACHE MISS: querying GatewayConfig for S3 bucket URL"
        );
        let s3_bucket_url = self
            .gateway_config_contract
            .getCoprocessor(copro_addr)
            .call()
            .await
            .map_err(|e| anyhow!("getCoprocessor({copro_addr}) failed: {e}"))?
            .s3BucketUrl;

        if s3_bucket_url.is_empty() {
            warn!(copro = %copro_addr, "no S3 bucket URL registered for coprocessor");
            return Err(anyhow!(
                "no S3 bucket URL registered for coprocessor {copro_addr}"
            ));
        }

        self.cache.insert(copro_addr, s3_bucket_url.clone());
        log_cache(&self.cache, "S3 cache state after insert");
        info!(
            copro = %copro_addr,
            url = %s3_bucket_url,
            "S3 bucket URL retrieved and cached"
        );
        Ok(s3_bucket_url)
    }

    /// Resolve S3 URLs for the supplied address list. Fails if any address
    /// cannot be resolved (RPC failure or no registered URL). Mirrors
    /// kms-worker's `get_all_coprocessors_s3_urls`.
    pub async fn get_all_coprocessors_s3_urls(
        &self,
        coprocessor_addresses: &[Address],
    ) -> anyhow::Result<Vec<String>> {
        info!(
            count = coprocessor_addresses.len(),
            "COPRO S3 URL FETCH START"
        );

        let mut s3_urls = Vec::with_capacity(coprocessor_addresses.len());
        for address in coprocessor_addresses.iter() {
            let url = self.get_coprocessor_s3_url(*address).await?;
            s3_urls.push(url);
        }
        Ok(s3_urls)
    }

    /// Convenience: fetch the signer set, then resolve every signer's URL.
    ///
    /// Note: this follows the documented pseudo-code exactly — `getCoprocessor`
    /// is called with the signer address. The Solidity definition keys the
    /// `coprocessors` mapping on the *tx-sender* address, so a coprocessor
    /// whose signer ≠ tx-sender will miss here. Adjust this call (and the
    /// `S3_BUCKET_CACHE` semantics) once the on-chain mapping is confirmed.
    pub async fn refresh_signer_urls(&self) -> anyhow::Result<Vec<String>> {
        let signers = self.get_coprocessor_signers().await?;
        self.get_all_coprocessors_s3_urls(&signers).await
    }
}

fn log_cache(cache: &DashMap<Address, String>, prefix: &str) {
    if tracing::enabled!(tracing::Level::DEBUG) {
        let size = cache.len();
        debug!("{prefix}: {size} entries");
        if size > 0 {
            let entries: Vec<String> = cache
                .iter()
                .map(|e| format!("{}: {}", e.key(), e.value()))
                .collect();
            debug!("S3 cache contents: {}", entries.join(", "));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Bytes;
    use alloy::providers::{mock::Asserter, ProviderBuilder};
    use alloy::sol_types::SolCall;
    use fhevm_gateway_bindings::gateway_config::GatewayConfig::{
        getCoprocessorCall, getCoprocessorTxSendersCall, Coprocessor,
    };

    /// `GatewayConfig` keys `coprocessors` by tx sender, and each registration
    /// names the signer. A member read by signer address resolves to nothing,
    /// and this operator then waits for a member it cannot reach.
    #[tokio::test]
    async fn members_come_from_the_tx_sender_registration() {
        let tx_sender = Address::from([0x01; 20]);
        let signer = Address::from([0x02; 20]);
        let asserter = Asserter::new();
        asserter.push_success(&Bytes::from(
            getCoprocessorTxSendersCall::abi_encode_returns(&vec![tx_sender]),
        ));
        asserter.push_success(&Bytes::from(getCoprocessorCall::abi_encode_returns(
            &Coprocessor {
                txSenderAddress: tx_sender,
                signerAddress: signer,
                s3BucketUrl: "http://bucket".to_owned(),
            },
        )));

        let service = S3Service::new(
            ProviderBuilder::default().connect_mocked_client(asserter),
            Address::ZERO,
        );

        assert_eq!(
            service.get_members_at(7).await.expect("members"),
            vec![(signer, "http://bucket".to_owned())]
        );
    }
}
