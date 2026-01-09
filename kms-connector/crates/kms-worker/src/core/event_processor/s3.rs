use crate::core::config::Config;
use alloy::{hex, providers::Provider, transports::http::Client};
use anyhow::anyhow;
use alloy::primitives::FixedBytes;
use fhevm_gateway_bindings::gateway_config::GatewayConfig::{self, GatewayConfigInstance};
use kms_grpc::kms::v1::{CiphertextFormat, TypedCiphertext};
use sha3::{Digest, Keccak256};
use tracing::{debug, trace, warn};

/// The header used to retrieve the ciphertext format from the S3 HTTP response.
const CT_FORMAT_HEADER: &str = "x-amz-meta-Ct-Format";

/// Struct used to fetch ciphertext from S3 buckets (V2: primarily via HTTP API).
#[derive(Clone)]
pub struct S3Service<P: Provider> {
    #[allow(dead_code)]
    gateway_config_contract: GatewayConfigInstance<P>,

    client: Client,

    #[allow(dead_code)]
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

    /// V2: Retrieves ciphertext materials from S3 using handles.
    /// In V2, ciphertext data is fetched via HTTP API from the coprocessor.
    pub async fn retrieve_ciphertext_materials_v2(
        &self,
        handles: &[FixedBytes<32>],
    ) -> anyhow::Result<Vec<TypedCiphertext>> {
        warn!(
            "V2: retrieve_ciphertext_materials_v2 called with {} handles - HTTP API fetching not yet implemented",
            handles.len()
        );
        // V2: Would fetch ciphertext data from coprocessor HTTP API using handles
        Ok(vec![])
    }

    /// Retrieves a ciphertext directly via HTTP (kept for V2 HTTP API implementation).
    #[allow(dead_code)]
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
