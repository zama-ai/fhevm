use crate::{
    core::config::Config,
    monitoring::metrics::{S3_CIPHERTEXT_RETRIEVAL_COUNTER, S3_CIPHERTEXT_RETRIEVAL_ERRORS},
};
use alloy::{hex, primitives::Address, providers::Provider, transports::http::Client};
use anyhow::anyhow;
use connector_utils::types::fhe::extract_fhe_type_from_handle;
use dashmap::DashMap;
use fhevm_gateway_bindings::{
    decryption::Decryption::SnsCiphertextMaterial,
    gateway_config::GatewayConfig::{self, GatewayConfigInstance},
};
use kms_grpc::kms::v1::{CiphertextFormat, TypedCiphertext};
use sha3::{Digest, Keccak256};
use std::sync::LazyLock;
use tracing::{debug, info, trace, warn};

/// Global cache for coprocessor S3 bucket URLs.
static S3_BUCKET_CACHE: LazyLock<DashMap<Address, String>> = LazyLock::new(DashMap::new);

/// The header used to retrieve the ciphertext format from the S3 HTTP response.
const CT_FORMAT_HEADER: &str = "x-amz-meta-Ct-Format";

/// Struct used to fetch ciphertext from S3 buckets.
#[derive(Clone)]
pub struct S3Service<P: Provider> {
    /// The instance of the `GatewayConfig` contract.
    gateway_config_contract: GatewayConfigInstance<P>,

    /// The HTTP client used to fetch ciphertext from S3 buckets.
    client: Client,

    /// Number of retries for S3 ciphertext retrieval.
    s3_ciphertext_retrieval_retries: u8,
}

impl<P> S3Service<P>
where
    P: Provider,
{
    pub fn new(config: &Config, provider: P, client: Client) -> Self {
        let gateway_config_contract =
            GatewayConfig::new(config.gateway_config_contract.address, provider);

        Self {
            gateway_config_contract,
            client,
            s3_ciphertext_retrieval_retries: config.s3_ciphertext_retrieval_retries,
        }
    }

    /// Helper method to retrieve ciphertext materials from S3.
    pub async fn retrieve_sns_ciphertext_materials(
        &self,
        sns_materials: Vec<SnsCiphertextMaterial>,
    ) -> anyhow::Result<Vec<TypedCiphertext>> {
        let mut sns_ciphertext_materials = Vec::new();
        for sns_material in sns_materials {
            let ciphertext = self.retrieve_s3_ciphertext_with_retry(sns_material).await?;
            sns_ciphertext_materials.push(ciphertext);
        }
        Ok(sns_ciphertext_materials)
    }

    /// Retrieves a ciphertext from S3 with `self.s3_ct_retrieval_retries` retries.
    pub async fn retrieve_s3_ciphertext_with_retry(
        &self,
        sns_material: SnsCiphertextMaterial,
    ) -> anyhow::Result<TypedCiphertext> {
        let s3_urls = self
            .get_all_coprocessors_s3_urls(&sns_material.coprocessorTxSenderAddresses)
            .await;

        let digest_hex = hex::encode(sns_material.snsCiphertextDigest);
        info!("S3 CIPHERTEXT RETRIEVAL START: digest {digest_hex}");

        for i in 1..=self.s3_ciphertext_retrieval_retries {
            if s3_urls.is_empty() {
                warn!("No S3 URLs found for ciphertext digest {digest_hex}",);
                continue;
            }

            for s3_url in s3_urls.iter() {
                match self
                    .retrieve_s3_ciphertext(s3_url, &sns_material, &digest_hex)
                    .await
                {
                    Ok(ciphertext) => {
                        S3_CIPHERTEXT_RETRIEVAL_COUNTER.inc();
                        return Ok(ciphertext);
                    }
                    Err(e) => {
                        S3_CIPHERTEXT_RETRIEVAL_ERRORS.inc();
                        warn!(
                            attempt = i,
                            "Failed to retrieve ciphertext for digest {digest_hex} from S3 URL {s3_url}: {e}"
                        )
                    }
                }
            }
        }
        Err(anyhow!("All S3 retrieval attempts failed"))
    }

    /// Fetches and caches S3 bucket URLs to return a list of coprocessor s3 urls.
    async fn get_all_coprocessors_s3_urls(&self, coprocessor_addresses: &[Address]) -> Vec<String> {
        info!(
            "COPRO S3 URL FETCH START: Fetching S3 bucket URLs for {} coprocessors",
            coprocessor_addresses.len()
        );

        let mut s3_urls = Vec::new();
        for address in coprocessor_addresses.iter() {
            match self.get_coprocessor_s3_urls(*address).await {
                Ok(s3_url) => {
                    info!("Successfully fetched S3 bucket URL for coprocessor {address}: {s3_url}");
                    s3_urls.push(s3_url);
                }
                Err(e) => {
                    warn!("Failed to prefetch S3 bucket URL for coprocessor {address}: {e}");
                }
            };
        }

        s3_urls
    }

    /// Retrieves the S3 bucket URL for a coprocessor from the GatewayConfig contract.
    async fn get_coprocessor_s3_urls(&self, copro_addr: Address) -> anyhow::Result<String> {
        // Try to find a cached S3 bucket URL for any of the coprocessors
        log_cache("S3 cache state before S3 URL fetching");
        if let Some(url) = S3_BUCKET_CACHE.get(&copro_addr) {
            info!(
                "CACHE HIT: Using cached S3 bucket URL for coprocessor {}: {}",
                copro_addr,
                url.value()
            );
            return Ok(url.value().clone());
        }

        info!(
            "CACHE MISS: Querying GatewayConfig contract for coprocessor {copro_addr} S3 bucket URL"
        );
        let s3_bucket_url = self
            .gateway_config_contract
            .getCoprocessor(copro_addr)
            .call()
            .await?
            .s3BucketUrl
            .to_string();

        if s3_bucket_url.is_empty() {
            warn!("No S3 bucket URL registered for coprocessor {copro_addr}");
        }

        S3_BUCKET_CACHE.insert(copro_addr, s3_bucket_url.clone());
        log_cache("S3 cache state after insert");
        info!(
            "Successfully retrieved and cached S3 bucket URL for coprocessor {copro_addr}: {s3_bucket_url}"
        );
        Ok(s3_bucket_url)
    }

    /// Retrieves a ciphertext from S3 using the bucket URLs and ciphertext digest.
    pub async fn retrieve_s3_ciphertext(
        &self,
        s3_bucket_url: &str,
        sns_material: &SnsCiphertextMaterial,
        digest_hex: &str,
    ) -> anyhow::Result<TypedCiphertext> {
        let fhe_type = extract_fhe_type_from_handle(sns_material.ctHandle.as_slice())?;
        let direct_url = format!("{s3_bucket_url}/{digest_hex}");
        let (ciphertext, ct_format) = self.retrieve_ciphertext_via_http(&direct_url).await?;

        info!(
            handle = hex::encode(sns_material.ctHandle),
            "S3 CIPHERTEXT RETRIEVAL SUCCESS: format: {}, length: {}, FHE Type: {:?}",
            ct_format.as_str_name(),
            ciphertext.len(),
            fhe_type
        );

        // Verify digest
        let calculated_digest = compute_keccak256_digest(&ciphertext);
        if calculated_digest != sns_material.snsCiphertextDigest.as_slice() {
            let calculated_digest_hex = hex::encode(&calculated_digest);
            return Err(anyhow!(
                "DIGEST MISMATCH: Expected: {digest_hex}, Got: {calculated_digest_hex}",
            ));
        }
        info!("S3 CIPHERTEXT RETRIEVAL COMPLETE: Successfully verified ciphertext digest");

        Ok(TypedCiphertext {
            ciphertext,
            external_handle: sns_material.ctHandle.to_vec(),
            fhe_type: fhe_type as i32,
            ciphertext_format: ct_format.into(),
        })
    }

    /// Retrieves a ciphertext directly via HTTP.
    async fn retrieve_ciphertext_via_http(
        &self,
        url: &str,
    ) -> anyhow::Result<(Vec<u8>, CiphertextFormat)> {
        debug!("Attempting direct HTTP retrieval from URL: {url}");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "HTTP request failed with status: {}",
                response.status()
            ));
        }

        // Read the ciphertext format from AWS metadata header
        let ct_format = match response.headers().get(CT_FORMAT_HEADER).map(AsRef::as_ref) {
            Some(b"compressed_on_cpu") | Some(b"compressed_on_gpu") => {
                CiphertextFormat::BigCompressed
            }
            _ => CiphertextFormat::BigExpanded,
        };

        let body = response
            .bytes()
            .await
            .map_err(|e| anyhow!("Failed to read HTTP response body: {}", e))?;

        Ok((body.to_vec(), ct_format))
    }
}

/// Logs the current state of the S3 bucket cache (only if log level is set to debug).
fn log_cache(prefix: &str) {
    if tracing::enabled!(tracing::Level::DEBUG) {
        let cache_size = S3_BUCKET_CACHE.len();
        debug!("{prefix}: {cache_size} entries");

        if cache_size > 0 {
            let mut cache_entries = Vec::new();
            for entry in S3_BUCKET_CACHE.iter() {
                cache_entries.push(format!("{}: {}", entry.key(), entry.value()));
            }
            debug!("S3_BUCKET_CACHE contents: {}", cache_entries.join(", "));
        }
    }
}

/// Computes Keccak256 digest of a byte array.
pub fn compute_keccak256_digest(ct: &[u8]) -> Vec<u8> {
    trace!("Computing Keccak256 digest for {} bytes of data", ct.len());
    let mut hasher = Keccak256::new();
    hasher.update(ct);
    let result = hasher.finalize().to_vec();
    trace!("Digest computed: {}", hex::encode(&result));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_digest_empty_input() {
        // Test digest calculation for empty input
        let empty_data: Vec<u8> = vec![];
        let digest = compute_keccak256_digest(&empty_data);

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
        let digest = compute_keccak256_digest(data);

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

        let digest1 = compute_keccak256_digest(data1);
        let digest2 = compute_keccak256_digest(data2);

        assert_ne!(digest1, digest2);
    }

    #[test]
    fn test_compute_digest_large_input() {
        // Test digest calculation for a larger input
        let large_data = vec![0u8; 1024 * 1024]; // 1MB of zeros
        let digest = compute_keccak256_digest(&large_data);
        assert_eq!(digest.len(), 32);
        // The digest of 1MB of zeros is deterministic
        assert_eq!(
            hex::encode(&digest),
            "7b6ff0a03e9c5a8e77a2059bf28e26a7f0e8d3939a7cfe2193908ad8d683be90"
        );
    }
}
