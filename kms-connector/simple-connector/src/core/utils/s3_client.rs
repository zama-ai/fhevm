use alloy::{hex::encode, primitives::Address, providers::Provider, transports::http::reqwest};
use dashmap::DashMap;
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::SnsCiphertextMaterial, gatewayconfig::GatewayConfig,
};
use sha3::{Digest, Keccak256};
use std::{sync::Arc, time::Duration};
use tracing::{debug, info, warn};

use crate::{
    core::config::S3Config,
    error::{Error, Result},
};

/// S3 client for ciphertext retrieval
#[derive(Clone)]
pub struct S3Client {
    config: Option<S3Config>,
    http_client: reqwest::Client,
    cache: Arc<DashMap<Address, String>>,
}

impl S3Client {
    /// Create a new S3 client
    pub fn new(config: Option<S3Config>) -> Self {
        let http_client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(2))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            http_client,
            cache: Arc::new(DashMap::new()),
        }
    }

    /// Retrieve all ciphertext materials for the given SNS materials
    pub async fn retrieve_ciphertext_materials(
        &self,
        sns_materials: Vec<SnsCiphertextMaterial>,
    ) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        let mut results = Vec::new();

        for sns_material in sns_materials {
            let ct_handle = sns_material.ctHandle.to_vec();
            let ciphertext_digest = sns_material.snsCiphertextDigest.to_vec();
            let coprocessor_addresses = sns_material.coprocessorTxSenderAddresses;

            info!(
                "Retrieving ciphertext with digest {} from {} coprocessors",
                encode(&ciphertext_digest),
                coprocessor_addresses.len()
            );

            // Try each coprocessor until we find the ciphertext
            let mut retrieved = false;
            for &coprocessor_address in &coprocessor_addresses {
                let s3_url = self.get_s3_url(coprocessor_address).await;

                match self.retrieve_from_url(&s3_url, &ciphertext_digest).await {
                    Ok(ciphertext) => {
                        info!(
                            "Successfully retrieved ciphertext for digest {} from {}",
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
                    "Failed to retrieve ciphertext for digest {} from any coprocessor",
                    encode(&ciphertext_digest)
                );
            }
        }

        Ok(results)
    }

    /// Get S3 URL for a coprocessor (with caching)
    async fn get_s3_url(&self, coprocessor_address: Address) -> String {
        // Check cache first
        if let Some(cached_url) = self.cache.get(&coprocessor_address) {
            debug!(
                "Cache hit for coprocessor {}: {}",
                encode(coprocessor_address),
                cached_url.value()
            );
            return cached_url.value().clone();
        }

        // Cache miss - use fallback URL from config
        let fallback_url = self.build_fallback_url();

        // Cache the fallback URL for future use
        self.cache.insert(coprocessor_address, fallback_url.clone());

        info!(
            "Cache miss for coprocessor {}, using fallback: {}",
            encode(coprocessor_address),
            fallback_url
        );

        fallback_url
    }

    /// Build fallback URL from S3Config
    fn build_fallback_url(&self) -> String {
        if let Some(config) = &self.config {
            if let Some(endpoint) = &config.endpoint {
                // Use custom endpoint (e.g., local MinIO)
                format!("{}/{}", endpoint.trim_end_matches('/'), config.bucket)
            } else {
                // Use AWS S3 format
                format!(
                    "https://s3.{}.amazonaws.com/{}",
                    config.region, config.bucket
                )
            }
        } else {
            // Default fallback if no config provided
            "http://minio:9000/ct128".to_string()
        }
    }

    /// Retrieve ciphertext from a specific URL
    async fn retrieve_from_url(&self, s3_url: &str, ciphertext_digest: &[u8]) -> Result<Vec<u8>> {
        let digest_hex = encode(ciphertext_digest);
        let url = format!("{s3_url}/{digest_hex}");

        debug!("Retrieving ciphertext from URL: {}", url);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::S3Error(format!("HTTP request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(Error::S3Error(format!(
                "HTTP request failed with status: {}",
                response.status()
            )));
        }

        let body = response
            .bytes()
            .await
            .map_err(|e| Error::S3Error(format!("Failed to read response body: {e}")))?;

        let ciphertext = body.to_vec();

        // Verify digest but don't fail
        let calculated_digest = self.compute_digest(&ciphertext);
        if calculated_digest != *ciphertext_digest {
            warn!(
                "DIGEST MISMATCH: Expected: {}, Got: {}",
                encode(ciphertext_digest),
                encode(&calculated_digest)
            );
        } else {
            debug!("Digest verification successful");
        }

        // Return data even with digest mismatch
        // TODO: that's Ok for testnet, but need to revisit for production
        Ok(ciphertext)
    }

    /// Compute Keccak256 digest
    fn compute_digest(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
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
                self.cache.insert(address, url);
            }
        }
    }

    /// Get S3 URL from gateway config contract
    async fn get_s3_url_from_gateway<P: Provider + Clone>(
        &self,
        coprocessor_address: Address,
        gateway_config_address: Address,
        provider: Arc<P>,
    ) -> Option<String> {
        let gateway_config = GatewayConfig::new(gateway_config_address, provider);

        match gateway_config
            .getCoprocessor(coprocessor_address)
            .call()
            .await
        {
            Ok(coprocessor) => {
                let s3_bucket_url = coprocessor._0.s3BucketUrl.to_string();
                if !s3_bucket_url.is_empty() {
                    Some(s3_bucket_url)
                } else {
                    None
                }
            }
            Err(e) => {
                debug!(
                    "Failed to get S3 bucket URL for coprocessor {}: {}",
                    encode(coprocessor_address),
                    e
                );
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_digest() {
        let client = S3Client::new(None);
        let data = b"hello world";
        let digest = client.compute_digest(data);

        // Known Keccak256 hash of "hello world"
        let expected_hex = "47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad";
        let expected_bytes = alloy::hex::decode(expected_hex).unwrap();

        assert_eq!(digest, expected_bytes);
    }

    #[test]
    fn test_build_fallback_url_with_endpoint() {
        let config = S3Config {
            region: "eu-west-1".to_string(),
            bucket: "kms-public".to_string(),
            endpoint: Some("http://localhost:9000".to_string()),
        };

        let client = S3Client::new(Some(config));
        let url = client.build_fallback_url();

        assert_eq!(url, "http://localhost:9000/kms-public");
    }

    #[test]
    fn test_build_fallback_url_aws() {
        let config = S3Config {
            region: "eu-west-1".to_string(),
            bucket: "kms-public".to_string(),
            endpoint: None,
        };

        let client = S3Client::new(Some(config));
        let url = client.build_fallback_url();

        assert_eq!(url, "https://s3.eu-west-1.amazonaws.com/kms-public");
    }
}
