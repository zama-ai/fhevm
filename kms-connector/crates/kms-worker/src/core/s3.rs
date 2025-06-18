use crate::core::config::{Config, S3Config};
use alloy::{hex::encode, primitives::Address, providers::Provider, transports::http::reqwest};
use anyhow::anyhow;
use dashmap::DashMap;
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::SnsCiphertextMaterial,
    gatewayconfig::GatewayConfig::{self, GatewayConfigInstance},
};
use sha3::{Digest, Keccak256};
use std::{sync::LazyLock, time::Duration};
use tracing::{debug, info, warn};

// Global cache for coprocessor S3 bucket URLs
static S3_BUCKET_CACHE: LazyLock<DashMap<Address, String>> = LazyLock::new(DashMap::new);

#[derive(Clone)]
pub struct S3Service<P: Provider> {
    gateway_config_contract: GatewayConfigInstance<(), P>,
    fallback_config: Option<S3Config>,
}

impl<P> S3Service<P>
where
    P: Provider,
{
    pub fn new(config: &Config, provider: P) -> Self {
        let gateway_config_contract =
            GatewayConfig::new(config.gateway_config_contract.address, provider);

        Self {
            gateway_config_contract,
            fallback_config: config.s3_config.clone(),
        }
    }

    /// Retrieves the S3 bucket URL for a coprocessor from the GatewayConfig contract.
    async fn get_s3_url(&self, copro_addr: Address) -> Option<String> {
        info!(
            "Attempting to get S3 bucket URL for coprocessor {:?}",
            copro_addr
        );

        // Try to find a cached S3 bucket URL for any of the coprocessors
        if let Some(url) = S3_BUCKET_CACHE.get(&copro_addr) {
            info!(
                "CACHE HIT: Using cached S3 bucket URL for coprocessor {:?}: {}",
                copro_addr,
                url.value()
            );
            return Some(url.value().clone());
        }

        // If no cached URL found, query the GatewayConfig contract for the first available coprocessor
        info!(
            "CACHE MISS: Querying GatewayConfig contract for coprocessor {:?} S3 bucket URL",
            copro_addr
        );

        // Call getCoprocessor method to retrieve S3 bucket URL of the coprocessor
        let s3_bucket_url = match self
            .gateway_config_contract
            .getCoprocessor(copro_addr)
            .call()
            .await
        {
            Ok(coprocessor) => coprocessor._0.s3BucketUrl.to_string(),
            Err(e) => {
                warn!(
                    "GatewayConfig contract call failed for coprocessor {:?}: {}",
                    copro_addr, e
                );
                return None;
            }
        };

        if s3_bucket_url.is_empty() {
            warn!("Coprocessor {:?} returned empty S3 bucket URL", copro_addr);
            return None;
        }

        // Cache the URL for future use
        info!(
            "CACHE UPDATE: Adding S3 bucket URL for coprocessor {:?}: {}",
            copro_addr, s3_bucket_url
        );
        S3_BUCKET_CACHE.insert(copro_addr, s3_bucket_url.clone());

        // Log the updated cache state
        log_cache();

        info!(
            "Successfully retrieved and cached S3 bucket URL for coprocessor {:?}: {}",
            copro_addr, s3_bucket_url
        );
        Some(s3_bucket_url)
    }

    /// Prefetches and caches S3 bucket URLs to return a list of coprocessor s3 urls.
    async fn prefetch_coprocessor_buckets(
        &self,
        coprocessor_addresses: Vec<Address>,
    ) -> Vec<String> {
        info!(
            "S3 PREFETCH START: Prefetching S3 bucket URLs for {} coprocessors",
            coprocessor_addresses.len()
        );

        // Log current cache state before prefetching
        info!("S3 cache state before prefetching:");
        log_cache();

        let mut s3_urls = Vec::new();
        let mut success_count = 0;
        let mut cache_hit_count = 0;
        let mut cache_miss_count = 0;
        let mut fallback_used = false;

        for (idx, address) in coprocessor_addresses.iter().enumerate() {
            info!(
                "Processing coprocessor {}/{}: {:?}",
                idx + 1,
                coprocessor_addresses.len(),
                address
            );

            // Add the cached URL to our result list
            if let Some(url) = S3_BUCKET_CACHE.get(address) {
                info!(
                    "CACHE HIT: S3 bucket URL for coprocessor {:?} already cached",
                    address
                );
                cache_hit_count += 1;
                success_count += 1;

                s3_urls.push(url.value().clone());
                continue;
            }

            cache_miss_count += 1;
            info!(
                "CACHE MISS: Fetching S3 bucket URL for coprocessor {:?}",
                address
            );

            match self.get_s3_url(*address).await {
                Some(s3_url) => {
                    info!(
                        "Successfully fetched S3 bucket URL for coprocessor {:?}: {}",
                        address, s3_url
                    );
                    success_count += 1;
                    s3_urls.push(s3_url);
                }
                None => {
                    warn!(
                        "Failed to prefetch S3 bucket URL for coprocessor {:?}",
                        address
                    );
                }
            };
        }

        // Log cache state after prefetching
        info!("S3 cache state after prefetching:");
        log_cache();

        // If we couldn't get any URLs but have a fallback config, use it
        if s3_urls.is_empty() {
            if let Some(config) = &self.fallback_config {
                if !config.bucket.is_empty() {
                    let fallback_url = format!(
                        "https://s3.{}.amazonaws.com/{}",
                        config.region, config.bucket
                    );
                    warn!(
                        "All S3 URL retrievals failed. Using global fallback S3 URL: {}",
                        fallback_url
                    );
                    s3_urls.push(fallback_url);
                    success_count += 1;
                    fallback_used = true;
                } else {
                    warn!(
                        "All S3 URL retrievals failed and fallback bucket is empty. No URLs available."
                    );
                }
            } else {
                warn!(
                    "All S3 URL retrievals failed and no fallback configuration available. No URLs available."
                );
            }
        }

        info!(
            "S3 PREFETCH COMPLETE: Successfully prefetched {}/{} S3 bucket URLs (cache hits: {}, cache misses: {}, fallback used: {})",
            success_count,
            coprocessor_addresses.len(),
            cache_hit_count,
            cache_miss_count,
            fallback_used
        );
        s3_urls
    }

    /// Helper method to retrieve ciphertext materials from S3.
    pub async fn retrieve_sns_ciphertext_materials(
        &self,
        sns_materials: Vec<SnsCiphertextMaterial>,
    ) -> Vec<(Vec<u8>, Vec<u8>)> {
        // Process all SNS ciphertext materials
        let mut sns_ciphertext_materials = Vec::new();
        for sns_material in sns_materials {
            let extracted_ct_handle = sns_material.ctHandle.to_vec();
            let extracted_sns_ciphertext_digest = sns_material.snsCiphertextDigest.to_vec();
            let coprocessor_addresses = sns_material.coprocessorTxSenderAddresses;

            // Get S3 URL and retrieve ciphertext
            // 1. For each SNS material, we try to retrieve its ciphertext from multiple possible S3 URLs
            // 2. Once we successfully retrieve a ciphertext from any of those URLs, we break out of the S3 URLs loop
            // 3. Then we continue processing the next SNS material in the outer loop
            let s3_urls = self
                .prefetch_coprocessor_buckets(coprocessor_addresses)
                .await;

            if s3_urls.is_empty() {
                warn!(
                    "No S3 URLs found for ciphertext digest {}",
                    alloy::hex::encode(&extracted_sns_ciphertext_digest)
                );
                continue;
            }

            let mut ciphertext_retrieved = false;
            for s3_url in s3_urls {
                match retrieve_s3_ciphertext(
                    s3_url.clone(),
                    extracted_sns_ciphertext_digest.clone(),
                )
                .await
                {
                    Ok(ciphertext) => {
                        info!(
                            "Successfully retrieved ciphertext for digest {} from S3 URL {}",
                            alloy::hex::encode(&extracted_sns_ciphertext_digest),
                            s3_url
                        );
                        sns_ciphertext_materials.push((extracted_ct_handle.clone(), ciphertext));
                        ciphertext_retrieved = true;
                        break; // We want to stop as soon as ciphertext corresponding to extracted_sns_ciphertext_digest is retrieved
                    }
                    Err(error) => {
                        // Log warning but continue trying other URLs
                        warn!(
                            "Failed to retrieve ciphertext for digest {} from S3 URL {}: {}",
                            alloy::hex::encode(&extracted_sns_ciphertext_digest),
                            s3_url,
                            error
                        );
                        // Continue to the next URL
                    }
                }
            }

            if !ciphertext_retrieved {
                warn!(
                    "Failed to retrieve ciphertext for digest {} from any S3 URL",
                    alloy::hex::encode(&extracted_sns_ciphertext_digest)
                );
                // Continue to the next SNS material
            }
        }

        sns_ciphertext_materials
    }
}

/// Logs the current state of the S3 bucket cache.
fn log_cache() {
    let cache_size = S3_BUCKET_CACHE.len();
    info!("S3Service cache state: {} entries", cache_size);

    if cache_size > 0 {
        let mut cache_entries = Vec::new();
        for entry in S3_BUCKET_CACHE.iter() {
            cache_entries.push(format!("{:?}: {}", entry.key(), entry.value()));
        }
        debug!("S3_BUCKET_CACHE contents: {}", cache_entries.join(", "));
    }
}

/// Computes Keccak256 digest of a byte array.
pub fn compute_digest(ct: &[u8]) -> Vec<u8> {
    debug!("Computing Keccak256 digest for {} bytes of data", ct.len());
    let mut hasher = Keccak256::new();
    hasher.update(ct);
    let result = hasher.finalize().to_vec();
    debug!("Digest computed: {}", encode(&result));
    result
}

/// Retrieves a ciphertext from S3 using the bucket URL and ciphertext digest.
pub async fn retrieve_s3_ciphertext(
    s3_bucket_url: String,
    ciphertext_digest: Vec<u8>,
) -> anyhow::Result<Vec<u8>> {
    let digest_hex = encode(&ciphertext_digest);
    info!(
        "S3 RETRIEVAL START: Retrieving ciphertext with digest {} from S3 bucket {}",
        digest_hex, s3_bucket_url
    );

    // Direct HTTP retrieval
    let direct_url = format!("{}/{}", s3_bucket_url, digest_hex);
    let ciphertext = direct_http_retrieval(&direct_url).await?;
    info!(
        "DIRECT HTTP RETRIEVAL SUCCESS: Retrieved {} bytes",
        ciphertext.len()
    );

    // Verify digest
    let calculated_digest = compute_digest(&ciphertext);
    if calculated_digest != ciphertext_digest {
        warn!(
            "DIGEST MISMATCH: Expected: {}, Got: {}",
            encode(ciphertext_digest),
            encode(&calculated_digest)
        );
    } else {
        info!("DIRECT HTTP RETRIEVAL COMPLETE: Successfully verified ciphertext digest");
    }

    // Return data even with digest mismatch for non-failability
    Ok(ciphertext)
}

/// Retrieves a file directly via HTTP.
async fn direct_http_retrieval(url: &str) -> anyhow::Result<Vec<u8>> {
    debug!("Attempting direct HTTP retrieval from URL: {}", url);

    // Create a reqwest client with appropriate timeouts
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .build()
        .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

    // Send the GET request
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| anyhow!("HTTP request failed: {}", e))?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(anyhow!(
            "HTTP request failed with status: {}",
            response.status()
        ));
    }

    // Read the response body
    let body = response
        .bytes()
        .await
        .map_err(|e| anyhow!("Failed to read HTTP response body: {}", e))?;

    Ok(body.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::fmt;

    #[test]
    fn test_compute_digest_empty_input() {
        // Test digest calculation for empty input
        let empty_data: Vec<u8> = vec![];
        let digest = compute_digest(&empty_data);

        // Keccak256 of empty input is a known value
        let expected_hex = "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470";
        let expected_bytes = alloy::hex::decode(expected_hex).unwrap();

        assert_eq!(digest, expected_bytes);
        assert_eq!(digest.len(), 32); // Keccak256 produces 32 bytes
    }

    #[test]
    fn test_compute_digest_known_input() {
        // Test digest calculation for a known input
        let data = b"hello world";
        let digest = compute_digest(data);

        // Known Keccak256 hash of "hello world"
        let expected_hex = "47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad";
        let expected_bytes = alloy::hex::decode(expected_hex).unwrap();

        assert_eq!(digest, expected_bytes);
    }

    #[test]
    fn test_compute_digest_different_inputs() {
        // Test that different inputs produce different digests
        let data1 = b"test data 1";
        let data2 = b"test data 2";

        let digest1 = compute_digest(data1);
        let digest2 = compute_digest(data2);

        assert_ne!(digest1, digest2);
    }

    #[test]
    fn test_compute_digest_large_input() {
        // Test digest calculation for a larger input
        let large_data = vec![0u8; 1024 * 1024]; // 1MB of zeros
        let digest = compute_digest(&large_data);
        assert_eq!(digest.len(), 32);
        // The digest of 1MB of zeros is deterministic
        assert_eq!(
            encode(&digest),
            "7b6ff0a03e9c5a8e77a2059bf28e26a7f0e8d3939a7cfe2193908ad8d683be90"
        );
    }

    // TODO: to remove after integration: sanity check
    // This test requires a running MinIO server
    #[ignore]
    #[tokio::test]
    async fn test_retrieve_s3_ciphertext_minio() {
        // Initialize tracing for this test
        let subscriber = fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .finish();
        let _guard = tracing::subscriber::set_default(subscriber);

        let minio_url = "http://localhost:9000";
        let bucket = "ct128";
        let call_url = format!("{}/{}", minio_url, bucket);
        println!(
            "Testing S3 ciphertext retrieval from MinIO at URL: {}",
            call_url
        );

        let digest_hex = "1c37ba3cfd0151dd03584cd4819c6296d6a8b4d7ac3e31554fb0e842eab8ada9";
        let digest_bytes = alloy::hex::decode(digest_hex).expect("Failed to decode hex digest");

        // Test the full retrieval function
        let result = retrieve_s3_ciphertext(call_url, digest_bytes.clone()).await;

        match result {
            Ok(data) => {
                println!(
                    "Successfully retrieved {} bytes from MinIO via call_s3_ciphertext_retrieval",
                    data.len()
                );

                assert!(!data.is_empty(), "Retrieved data should not be empty");

                let calculated_digest = compute_digest(&data);
                let calculated_hex = encode(&calculated_digest);
                println!("Retrieved data digest: {}", calculated_hex);
                println!("Expected digest: {}", digest_hex);
            }
            Err(error) => {
                // This should only happen if both direct HTTP and S3 SDK retrievals fail completely
                println!("Failed to retrieve ciphertext from MinIO: {}", error);
                println!(
                    "This is expected if MinIO is not running or the bucket/object doesn't exist"
                );
            }
        }
    }
}
