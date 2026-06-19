//! S3 bucket URL resolver, modeled on kms-worker's `S3Service`
//! (`fhevm/kms-connector/crates/kms-worker/src/core/event_processor/s3.rs`).
//!
//! Wraps a `GatewayConfigInstance` and a `DashMap` cache keyed by coprocessor
//! address. Cache hits avoid the on-chain round-trip. The cache is scoped to
//! a single `S3Service` instance (kms-worker uses a global `LazyLock`; we keep
//! it on the instance because consensus-detector runs one service per process).

use alloy::{network::Ethereum, primitives::Address, providers::Provider};
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

    /// Resolve `s3BucketUrl` for one coprocessor, populating the cache on miss.
    ///
    /// Returns `Err` for RPC failures. Returns `Ok("")` (and warns) when the
    /// contract has no S3 URL registered for `copro_addr` — matches the
    /// kms-worker behaviour so the bulk caller can decide what to do.
    pub async fn get_coprocessor_s3_url(
        &self,
        copro_addr: Address,
    ) -> anyhow::Result<String> {
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

    /// Resolve S3 URLs for the supplied address list. Per-address failures and
    /// empty URLs are logged and skipped — the returned vec may be shorter
    /// than `coprocessor_addresses`. Mirrors kms-worker's
    /// `get_all_coprocessors_s3_urls`.
    pub async fn get_all_coprocessors_s3_urls(
        &self,
        coprocessor_addresses: &[Address],
    ) -> Vec<String> {
        info!(
            count = coprocessor_addresses.len(),
            "COPRO S3 URL FETCH START"
        );

        let mut s3_urls = Vec::with_capacity(coprocessor_addresses.len());
        for address in coprocessor_addresses.iter() {
            match self.get_coprocessor_s3_url(*address).await {
                Ok(url) if url.is_empty() => {
                    // Empty already warned in get_coprocessor_s3_url.
                }
                Ok(url) => s3_urls.push(url),
                Err(e) => {
                    warn!(copro = %address, error = %e, "failed to fetch S3 bucket URL");
                }
            }
        }
        s3_urls
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
        Ok(self.get_all_coprocessors_s3_urls(&signers).await)
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
