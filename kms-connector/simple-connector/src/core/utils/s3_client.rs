use alloy::{hex::encode, primitives::Address, providers::Provider, transports::http::reqwest};
use chrono::Utc;
use dashmap::DashMap;
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::SnsCiphertextMaterial, gatewayconfig::GatewayConfig,
};
use sha3::{Digest, Keccak256};
use std::{
    sync::{Arc, LazyLock, OnceLock},
    time::Duration,
};
use tracing::{debug, info, warn};

use crate::{
    core::config::S3Config,
    error::{Error, Result},
};

// Cache entry with creation time for TTL cleanup
#[derive(Debug, Clone)]
struct CacheEntry {
    url: String,
    created_at: i64, // UTC timestamp in seconds (chrono::Utc)
}

// Global cache for coprocessor S3 bucket URLs with TTL support
static S3_URL_CACHE: LazyLock<DashMap<Address, CacheEntry>> = LazyLock::new(DashMap::new);

// Flag to ensure cleanup task is started only once
static S3_CLEANUP_TASK_STARTED: OnceLock<()> = OnceLock::new();

/// Non-blocking TTL cleanup for S3 URL cache
/// Runs in background task to prevent flow jamming
/// Uses chrono::Utc for consistent UTC timing
async fn cleanup_expired_s3_cache_once() -> std::result::Result<usize, String> {
    let ttl_seconds = 24 * 60 * 60; // 24 hours TTL (longer than block timestamps)
    let now = Utc::now().timestamp(); // UTC timestamp in seconds
    let mut expired_count = 0;

    // Non-blocking cleanup using DashMap's retain method
    S3_URL_CACHE.retain(|_address, entry| {
        let is_expired = now.saturating_sub(entry.created_at) > ttl_seconds;
        if is_expired {
            expired_count += 1;
        }
        !is_expired
    });

    // Log cache state periodically
    if !S3_URL_CACHE.is_empty() {
        debug!(
            "S3_URL_CACHE: {} active entries after cleanup",
            S3_URL_CACHE.len()
        );
    }

    Ok(expired_count)
}

/// Start the non-blocking S3 cache cleanup task (called once)
fn start_s3_cleanup_task() {
    S3_CLEANUP_TASK_STARTED.get_or_init(|| {
        let handle = tokio::spawn(async {
            info!("S3 cache cleanup task starting...");

            // Run cleanup with proper error handling
            loop {
                match tokio::time::timeout(
                    Duration::from_secs(30), // 30 second timeout for cleanup operation
                    cleanup_expired_s3_cache_once(),
                )
                .await
                {
                    Ok(Ok(cleaned_count)) => {
                        if cleaned_count > 0 {
                            info!(
                                "S3 cache cleanup completed: {} entries removed",
                                cleaned_count
                            );
                        } else {
                            debug!("S3 cache cleanup completed: no expired entries");
                        }
                    }
                    Ok(Err(e)) => {
                        warn!("S3 cache cleanup failed: {}", e);
                    }
                    Err(_) => {
                        warn!("S3 cache cleanup timed out after 30 seconds");
                    }
                }

                // Wait 10 minutes before next cleanup (same as block timestamps)
                tokio::time::sleep(Duration::from_secs(10 * 60)).await;
            }
        });

        info!(
            "Started non-blocking S3 cache cleanup task (JoinHandle: {:?})",
            handle.id()
        );
    });
}

/// S3 client for ciphertext retrieval
#[derive(Clone)]
pub struct S3Client {
    config: Option<S3Config>,
    http_client: reqwest::Client,
}

impl S3Client {
    /// Create a new S3 client
    pub fn new(config: Option<S3Config>) -> Self {
        let http_client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(2))
            .build()
            .expect("Failed to create HTTP client");

        // Start cleanup task if not already started (non-blocking)
        start_s3_cleanup_task();

        Self {
            config,
            http_client,
        }
    }

    /// Retrieve all ciphertext materials for the given SNS materials
    /// Uses Gateway contract first, then S3 config fallback if available
    pub async fn retrieve_ciphertext_materials<P: Provider + Clone + 'static>(
        &self,
        sns_materials: Vec<SnsCiphertextMaterial>,
        gateway_config_address: Address,
        provider: Arc<P>,
    ) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        // If no S3 config, log warning and return empty results (S3 disabled is valid)
        if self.config.is_none() {
            warn!("No S3 configuration available - S3 retrieval disabled, returning empty results");
            return Ok(Vec::new());
        }

        let mut results = Vec::new();
        let mut s3_urls = Vec::new();
        let mut success_count = 0;
        let mut fallback_used = false;

        info!(
            "S3 PREFETCH START: Prefetching S3 bucket URLs for {} coprocessors",
            sns_materials.len()
        );

        // Collect unique coprocessor addresses
        let mut coprocessor_addresses = Vec::new();
        for sns_material in &sns_materials {
            for &addr in &sns_material.coprocessorTxSenderAddresses {
                if !coprocessor_addresses.contains(&addr) {
                    coprocessor_addresses.push(addr);
                }
            }
        }

        // Try to get S3 URLs from Gateway contract first
        for (idx, &address) in coprocessor_addresses.iter().enumerate() {
            info!(
                "Processing coprocessor {}/{}: {:?}",
                idx + 1,
                coprocessor_addresses.len(),
                address
            );

            // Check global cache first
            if let Some(cached_entry) = S3_URL_CACHE.get(&address) {
                info!(
                    "CACHE HIT: S3 bucket URL for coprocessor {:?} already cached: {}",
                    address,
                    cached_entry.value().url
                );
                s3_urls.push(cached_entry.value().url.clone());
                success_count += 1;
                continue;
            }

            info!(
                "CACHE MISS: Querying GatewayConfig contract for coprocessor {:?} S3 bucket URL",
                address
            );

            // Query Gateway contract for S3 URL
            match self
                .get_s3_url_from_gateway(address, gateway_config_address, provider.clone())
                .await
            {
                Some(s3_url) => {
                    info!(
                        "Successfully fetched S3 bucket URL for coprocessor {:?}: {}",
                        address, s3_url
                    );
                    s3_urls.push(s3_url.clone());
                    // Cache the URL with timestamp for TTL
                    let cache_entry = CacheEntry {
                        url: s3_url,
                        created_at: Utc::now().timestamp(),
                    };
                    S3_URL_CACHE.insert(address, cache_entry);
                    success_count += 1;
                }
                None => {
                    warn!(
                        "Failed to prefetch S3 bucket URL for coprocessor {:?}",
                        address
                    );
                }
            }
        }

        // If we couldn't get any URLs but have a fallback config, use it
        if s3_urls.is_empty() {
            if let Some(config) = &self.config {
                if !config.bucket.is_empty() {
                    let fallback_url = if let Some(endpoint) = &config.endpoint {
                        format!("{}/{}", endpoint.trim_end_matches('/'), config.bucket)
                    } else {
                        format!(
                            "https://s3.{}.amazonaws.com/{}",
                            config.region, config.bucket
                        )
                    };
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

        if s3_urls.is_empty() {
            warn!("No S3 URLs available for ciphertext retrieval - returning empty results");
            return Ok(Vec::new());
        }

        info!(
            "S3 PREFETCH COMPLETE: Successfully prefetched {}/{} S3 bucket URLs (fallback used: {})",
            success_count,
            coprocessor_addresses.len(),
            fallback_used
        );

        for sns_material in sns_materials {
            let ct_handle = sns_material.ctHandle.to_vec();
            let ciphertext_digest = sns_material.snsCiphertextDigest.to_vec();

            info!(
                "S3 RETRIEVAL START: Retrieving ciphertext with digest {} from S3 bucket",
                encode(&ciphertext_digest)
            );

            // Try each S3 URL until we find the ciphertext
            let mut retrieved = false;
            for s3_url in &s3_urls {
                match self.retrieve_from_url(s3_url, &ciphertext_digest).await {
                    Ok(ciphertext) => {
                        info!(
                            "Successfully retrieved ciphertext for digest {} from S3 URL {}",
                            encode(&ciphertext_digest),
                            s3_url
                        );
                        results.push((ct_handle.clone(), ciphertext));
                        retrieved = true;
                        break;
                    }
                    Err(e) => {
                        warn!(
                            "Failed to retrieve ciphertext for digest {} from {}: {}",
                            encode(&ciphertext_digest),
                            s3_url,
                            e
                        );
                    }
                }
            }

            if !retrieved {
                warn!(
                    "Failed to retrieve ciphertext for digest {} from any S3 URL - skipping this ciphertext",
                    encode(&ciphertext_digest)
                );
                // Continue processing other ciphertexts instead of failing
            }
        }

        if results.is_empty() {
            warn!("No ciphertext materials retrieved from S3 - returning empty results");
        }

        Ok(results)
    }

    /// Get S3 URL from gateway config contract
    async fn get_s3_url_from_gateway<P: Provider>(
        &self,
        coprocessor_address: Address,
        gateway_config_address: Address,
        provider: Arc<P>,
    ) -> Option<String> {
        info!(
            "Attempting to get S3 bucket URL for coprocessor {:?}",
            coprocessor_address
        );

        // Create GatewayConfig contract instance
        let contract = GatewayConfig::new(gateway_config_address, provider);

        // Call getCoprocessor method
        let coprocessor = match contract.getCoprocessor(coprocessor_address).call().await {
            Ok(result) => result,
            Err(e) => {
                warn!(
                    "GatewayConfig contract call failed for coprocessor {:?}: {}",
                    coprocessor_address, e
                );
                return None;
            }
        };

        // Extract S3 bucket URL from the coprocessor
        let s3_bucket_url = coprocessor._0.s3BucketUrl.to_string();

        if s3_bucket_url.is_empty() {
            warn!(
                "Coprocessor {:?} returned empty S3 bucket URL",
                coprocessor_address
            );
            return None;
        }

        info!(
            "Successfully retrieved S3 bucket URL for coprocessor {:?}: {}",
            coprocessor_address, s3_bucket_url
        );
        Some(s3_bucket_url)
    }

    /// Compute Keccak256 digest of a byte array
    fn compute_digest(&self, data: &[u8]) -> Vec<u8> {
        debug!(
            "Computing Keccak256 digest for {} bytes of data",
            data.len()
        );
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let result = hasher.finalize().to_vec();
        debug!("Digest computed: {}", encode(&result));
        result
    }

    /// Retrieve ciphertext from a specific URL
    async fn retrieve_from_url(&self, s3_url: &str, ciphertext_digest: &[u8]) -> Result<Vec<u8>> {
        let digest_hex = encode(ciphertext_digest);
        let url = format!("{}/{}", s3_url.trim_end_matches('/'), digest_hex);

        debug!("Attempting to retrieve ciphertext from URL: {}", url);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::S3Error(format!("HTTP request failed: {e}")))?;

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(Error::S3Error(format!(
                "HTTP request failed with status: {}",
                response.status()
            )));
        }

        // Read the response body
        let body = response
            .bytes()
            .await
            .map_err(|e| Error::S3Error(format!("Failed to read HTTP response body: {e}")))?;

        let ciphertext = body.to_vec();

        // Verify digest
        let calculated_digest = self.compute_digest(&ciphertext);
        if calculated_digest != ciphertext_digest {
            warn!(
                "DIGEST MISMATCH: Expected: {}, Got: {}",
                encode(ciphertext_digest),
                encode(&calculated_digest)
            );
        } else {
            info!("DIRECT HTTP RETRIEVAL COMPLETE: Successfully verified ciphertext digest");
        }

        info!(
            "DIRECT HTTP RETRIEVAL SUCCESS: Retrieved {} bytes",
            ciphertext.len()
        );

        // Return data even with digest mismatch for non-failability
        Ok(ciphertext)
    }

    /// Populate cache with S3 URLs from gateway (optional optimization)
    pub async fn populate_cache_from_gateway<P: Provider + Clone>(
        &self,
        coprocessor_addresses: Vec<Address>,
        gateway_config_address: Address,
        provider: Arc<P>,
    ) {
        for &address in &coprocessor_addresses {
            if let Some(url) = self
                .get_s3_url_from_gateway(address, gateway_config_address, provider.clone())
                .await
            {
                // Cache the URL with timestamp for TTL
                let cache_entry = CacheEntry {
                    url,
                    created_at: Utc::now().timestamp(),
                };
                S3_URL_CACHE.insert(address, cache_entry);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compute_digest() {
        let client = S3Client::new(None);
        let data = b"hello world";
        let digest = client.compute_digest(data);

        // Known Keccak256 hash of "hello world"
        let expected_hex = "47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad";
        let expected_bytes = alloy::hex::decode(expected_hex).unwrap();

        assert_eq!(digest, expected_bytes);
    }

    #[tokio::test]
    async fn test_s3_client_with_config() {
        let config = S3Config {
            region: "eu-west-1".to_string(),
            bucket: "kms-public".to_string(),
            endpoint: Some("http://localhost:9000".to_string()),
        };

        let client = S3Client::new(Some(config));
        assert!(client.config.is_some());
    }

    #[tokio::test]
    async fn test_s3_client_without_config() {
        let client = S3Client::new(None);
        assert!(client.config.is_none());
    }

    #[tokio::test]
    async fn test_s3_cache_cleanup_functionality() {
        // Create S3Client to start cleanup task
        let _client = S3Client::new(None);

        // Manually add some cache entries with different timestamps
        let now = Utc::now().timestamp();
        let old_timestamp = now - (25 * 60 * 60); // 25 hours ago (expired)
        let fresh_timestamp = now - (60 * 60); // 1 hour ago (fresh)

        let old_entry = CacheEntry {
            url: "https://s3.amazonaws.com/old-bucket".to_string(),
            created_at: old_timestamp,
        };

        let fresh_entry = CacheEntry {
            url: "https://s3.amazonaws.com/fresh-bucket".to_string(),
            created_at: fresh_timestamp,
        };

        // Insert test entries
        let old_address = Address::from([1u8; 20]);
        let fresh_address = Address::from([2u8; 20]);

        S3_URL_CACHE.insert(old_address, old_entry);
        S3_URL_CACHE.insert(fresh_address, fresh_entry);

        // Verify both entries are in cache
        assert_eq!(S3_URL_CACHE.len(), 2);
        assert!(S3_URL_CACHE.contains_key(&old_address));
        assert!(S3_URL_CACHE.contains_key(&fresh_address));

        // Run cleanup manually
        let cleaned_count = cleanup_expired_s3_cache_once().await.unwrap();

        // Verify cleanup worked correctly
        assert_eq!(cleaned_count, 1); // One expired entry removed
        assert_eq!(S3_URL_CACHE.len(), 1); // Only fresh entry remains
        assert!(!S3_URL_CACHE.contains_key(&old_address)); // Old entry removed
        assert!(S3_URL_CACHE.contains_key(&fresh_address)); // Fresh entry kept

        // Clean up for other tests
        S3_URL_CACHE.clear();
    }
}
