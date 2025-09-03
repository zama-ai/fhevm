use crate::{
    core::config::{Config, S3Config},
    monitoring::metrics::{S3_CIPHERTEXT_RETRIEVAL_COUNTER, S3_CIPHERTEXT_RETRIEVAL_ERRORS},
};
use alloy::{hex, transports::http::reqwest};
use anyhow::anyhow;
use connector_utils::types::fhe::extract_fhe_type_from_handle;
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;
use kms_grpc::kms::v1::{CiphertextFormat, TypedCiphertext};
use sha3::{Digest, Keccak256};
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// The header used to retrieve the ciphertext format from the S3 HTTP response.
const CT_FORMAT_HEADER: &str = "x-amz-meta-Ct-Format";

/// Struct used to fetch ciphertext from S3 buckets.
#[derive(Clone)]
pub struct S3Service {
    /// An optional S3 bucket fallback configuration.
    fallback_config: Option<S3Config>,

    /// Number of retries for S3 ciphertext retrieval.
    s3_ciphertext_retrieval_retries: u8,

    /// Timeout for S3 ciphertext retrieval in seconds.
    s3_connect_timeout: Duration,
}

impl S3Service {
    pub fn new(config: &Config) -> Self {
        Self {
            fallback_config: config.s3_config.clone(),
            s3_ciphertext_retrieval_retries: config.s3_ciphertext_retrieval_retries,
            s3_connect_timeout: config.s3_connect_timeout,
        }
    }

    /// Helper method to retrieve ciphertext materials from S3.
    pub async fn retrieve_sns_ciphertext_materials(
        &self,
        sns_materials: Vec<SnsCiphertextMaterial>,
        s3_urls_metrix: Vec<Vec<String>>,
    ) -> Vec<TypedCiphertext> {
        let mut sns_ciphertext_materials = Vec::new();

        for (sns_material, mut s3_urls) in sns_materials.into_iter().zip(s3_urls_metrix) {
            // 1. For each SNS material, we try to retrieve its ciphertext from multiple possible S3 URLs
            //    1.1. We try to fetch the ciphertext for `self.s3_ct_retrieval_retries` times for each S3 URL
            // 2. Once we successfully retrieve a ciphertext from any of those URLs, we break out of the S3 URLs loop
            // 3. Then we continue processing the next SNS material in the outer loop

            let handle = sns_material.ctHandle.to_vec();
            let ct_digest = sns_material.snsCiphertextDigest.as_slice();
            let ct_digest_hex = hex::encode(ct_digest);
            if s3_urls.is_empty() {
                if let Some(fallback_s3_config) = &self.fallback_config {
                    warn!(
                        "No S3 URLs found for ciphertext digest {ct_digest_hex}, using S3 fallback config"
                    );
                    s3_urls.push(extract_fallback_url(fallback_s3_config));
                } else {
                    warn!("No S3 URLs found for ciphertext digest {ct_digest_hex}");
                    continue;
                }
            }

            match self
                .retrieve_s3_ciphertext_with_retry(s3_urls, &handle, ct_digest, &ct_digest_hex)
                .await
            {
                Some(ciphertext) => sns_ciphertext_materials.push(ciphertext),
                None => error!(
                    "Failed to retrieve ciphertext for digest {ct_digest_hex} from any S3 URL"
                ),
            }
        }

        sns_ciphertext_materials
    }

    /// Retrieves a ciphertext from S3 with `self.s3_ct_retrieval_retries` retries.
    pub async fn retrieve_s3_ciphertext_with_retry(
        &self,
        s3_urls: Vec<String>,
        handle: &[u8],
        ciphertext_digest: &[u8],
        digest_hex: &str,
    ) -> Option<TypedCiphertext> {
        for i in 1..=self.s3_ciphertext_retrieval_retries {
            for s3_url in s3_urls.iter() {
                match self
                    .retrieve_s3_ciphertext(s3_url, handle, ciphertext_digest, digest_hex)
                    .await
                {
                    Ok(ciphertext) => {
                        S3_CIPHERTEXT_RETRIEVAL_COUNTER.inc();
                        info!(
                            attempt = i,
                            "Successfully retrieved ciphertext for digest {digest_hex} from S3 URL {s3_url}"
                        );
                        return Some(ciphertext);
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
        None
    }

    /// Retrieves a ciphertext from S3 using the bucket URLs and ciphertext digest.
    pub async fn retrieve_s3_ciphertext(
        &self,
        s3_bucket_url: &str,
        handle: &[u8],
        ciphertext_digest: &[u8],
        digest_hex: &str,
    ) -> anyhow::Result<TypedCiphertext> {
        info!("S3 CIPHERTEXT RETRIEVAL START: from bucket {s3_bucket_url}, digest {digest_hex}");

        // Direct HTTP retrieval
        let direct_url = format!("{s3_bucket_url}/{digest_hex}");
        let (ciphertext, ct_format) = self.direct_http_retrieval(&direct_url).await?;

        // TODO: once tfhe-rs is upgraded to 1.3, replace map().unwrap_or() by ?
        // Right now it fails test when facing unknown types
        let fhe_type = extract_fhe_type_from_handle(handle)
            .map(|t| t as i32)
            .unwrap_or(0);

        info!(
            handle = hex::encode(handle),
            "S3 CIPHERTEXT RETRIEVAL SUCCESS: format: {}, length: {}, FHE Type: {:?}",
            ct_format.as_str_name(),
            ciphertext.len(),
            fhe_type
        );

        // Verify digest
        let calculated_digest = compute_digest(&ciphertext);
        if calculated_digest != ciphertext_digest {
            let calculated_digest_hex = hex::encode(&calculated_digest);
            return Err(anyhow!(
                "DIGEST MISMATCH: Expected: {digest_hex}, Got: {calculated_digest_hex}",
            ));
        }
        info!("S3 CIPHERTEXT RETRIEVAL COMPLETE: Successfully verified ciphertext digest");

        Ok(TypedCiphertext {
            ciphertext,
            external_handle: handle.to_vec(),
            fhe_type,
            ciphertext_format: ct_format.into(),
        })
    }

    /// Retrieves a file directly via HTTP.
    async fn direct_http_retrieval(
        &self,
        url: &str,
    ) -> anyhow::Result<(Vec<u8>, CiphertextFormat)> {
        debug!("Attempting direct HTTP retrieval from URL: {}", url);

        // Create a reqwest client with appropriate timeouts
        let client = reqwest::Client::builder()
            .connect_timeout(self.s3_connect_timeout)
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

        // Read the ciphertext format from AWS metadata header
        let ct_format = match response.headers().get(CT_FORMAT_HEADER).map(AsRef::as_ref) {
            Some(b"compressed_on_cpu") | Some(b"compressed_on_gpu") => {
                CiphertextFormat::BigCompressed
            }
            _ => CiphertextFormat::BigExpanded,
        };

        // Read the response body
        let body = response
            .bytes()
            .await
            .map_err(|e| anyhow!("Failed to read HTTP response body: {}", e))?;

        Ok((body.to_vec(), ct_format))
    }
}

/// Computes Keccak256 digest of a byte array.
pub fn compute_digest(ct: &[u8]) -> Vec<u8> {
    debug!("Computing Keccak256 digest for {} bytes of data", ct.len());
    let mut hasher = Keccak256::new();
    hasher.update(ct);
    let result = hasher.finalize().to_vec();
    debug!("Digest computed: {}", hex::encode(&result));
    result
}

pub fn extract_fallback_url(s3_config: &S3Config) -> String {
    if let Some(fallback_s3_endpoint) = s3_config.endpoint.clone() {
        fallback_s3_endpoint
    } else {
        format!(
            "https://s3.{}.amazonaws.com/{}",
            s3_config.region, s3_config.bucket
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            hex::encode(&digest),
            "7b6ff0a03e9c5a8e77a2059bf28e26a7f0e8d3939a7cfe2193908ad8d683be90"
        );
    }
}
