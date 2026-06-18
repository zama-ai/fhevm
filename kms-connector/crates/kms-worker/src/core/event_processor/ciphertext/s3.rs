//! Interactions with the Coprocessors' S3 buckets: attestation fetching (HEAD requests) and
//! ciphertext retrieval (GET requests).
//!
//! Both target the same object key (see [`ciphertext_object_url`]): the attestation lives in
//! an S3 metadata header of the object whose body is the ciphertext itself. Bucket URLs are
//! resolved from a [`CoprocessorRegistrySnapshot`], the single source of on-chain
//! Coprocessor metadata.

use super::{COPROCESSOR_CONTEXT_ID, registry::CoprocessorRegistrySnapshot};
use crate::monitoring::metrics::{S3_CIPHERTEXT_RETRIEVAL_COUNTER, S3_CIPHERTEXT_RETRIEVAL_ERRORS};
use alloy::{
    hex,
    primitives::{Address, B256},
    transports::http::{Client, reqwest::header::HeaderMap},
};
use anyhow::anyhow;
use ciphertext_attestation::{
    CiphertextAttestation, CiphertextFormat, S3_METADATA_ATTESTATION_HEADER,
};
use connector_utils::types::handle::extract_fhe_type_from_handle;
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;
use futures::future::try_join_all;
use kms_grpc::kms::v1::{CiphertextFormat as GrpcCiphertextFormat, TypedCiphertext};
use sha3::{Digest, Keccak256};
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::{debug, info, trace, warn};

/// The header used to retrieve the ciphertext format from the S3 HTTP response (pre RFC-023).
const OLD_CT_FORMAT_HEADER: &str = "x-amz-meta-Ct-Format";

/// URL of a ciphertext object in a Coprocessor bucket (RFC 023 layout).
fn rfc023_ciphertext_url(bucket_url: &str, handle: B256) -> String {
    format!(
        "{bucket_url}/{}/{COPROCESSOR_CONTEXT_ID}",
        hex::encode(handle)
    )
}

/// Why a single bucket's HEAD attempt did not yield an attestation.
#[derive(Debug, thiserror::Error)]
enum FetchAttestationError {
    #[error("HEAD request timed out")]
    Timeout,
    #[error("HEAD request failed: {0}")]
    Http(String),
    #[error("malformed attestation header: {0}")]
    BadHeader(String),
    #[error("attestation header not found")]
    MissingHeader,
}

/// Fans out parallel S3 HEAD requests to every registered Coprocessor bucket.
///
/// Returns the attestations that were successfully fetched, keyed by the Coprocessor's `txSender`
/// address. Failed fetches are logged and dropped.
pub async fn fetch_attestations(
    handle: B256,
    registry: &CoprocessorRegistrySnapshot,
    client: &Client,
    head_timeout: Duration,
) -> Vec<(Address, CiphertextAttestation)> {
    let mut tasks = JoinSet::new();
    for (tx_sender, bucket) in registry.tx_sender_to_bucket.iter() {
        let (client, bucket, tx_sender) = (client.clone(), bucket.clone(), *tx_sender);
        tasks.spawn(async move {
            let result = fetch_single_attestation(&client, &bucket, handle, head_timeout).await;
            (tx_sender, result)
        });
    }

    let mut valid_attestations = vec![];
    for (tx_sender, result) in tasks.join_all().await {
        match result {
            Ok(attestation) => valid_attestations.push((tx_sender, attestation)),
            Err(e) => warn!(
                %tx_sender,
                handle = hex::encode(handle),
                "Failed to fetch attestation: {e}"
            ),
        }
    }
    valid_attestations
}

/// Fetches the attestation for a `handle` from the specified bucket, using a `HEAD` request.
async fn fetch_single_attestation(
    client: &Client,
    bucket: &str,
    handle: B256,
    head_timeout: Duration,
) -> Result<CiphertextAttestation, FetchAttestationError> {
    let url = rfc023_ciphertext_url(bucket, handle);

    let response = client
        .head(&url)
        .timeout(head_timeout)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                FetchAttestationError::Timeout
            } else {
                FetchAttestationError::Http(e.to_string())
            }
        })?;

    if !response.status().is_success() {
        return Err(FetchAttestationError::Http(format!(
            "status {}",
            response.status()
        )));
    }

    attestation_from_http_headers(response.headers())
}

/// Retrieves the ciphertext materials from S3 concurrently.
pub async fn retrieve_sns_ciphertext_materials(
    client: &Client,
    copro_registry: &CoprocessorRegistrySnapshot,
    sns_materials: &[SnsCiphertextMaterial],
    retries: u8,
) -> anyhow::Result<Vec<TypedCiphertext>> {
    let fetch_futures = sns_materials.iter().map(|sns_material| {
        retrieve_s3_ciphertext_with_retry(client, copro_registry, sns_material, retries)
    });

    try_join_all(fetch_futures).await
}

/// Retrieves a ciphertext from S3 with retries.
async fn retrieve_s3_ciphertext_with_retry(
    client: &Client,
    copro_registry: &CoprocessorRegistrySnapshot,
    sns_material: &SnsCiphertextMaterial,
    retries: u8,
) -> anyhow::Result<TypedCiphertext> {
    let digest_hex = hex::encode(sns_material.snsCiphertextDigest);
    let s3_urls = coprocessors_s3_urls(copro_registry, &sns_material.coprocessorTxSenderAddresses);
    if s3_urls.is_empty() {
        return Err(anyhow!(
            "No S3 bucket URL found in the Coprocessor registry for ciphertext digest {digest_hex}"
        ));
    }

    info!("S3 CIPHERTEXT RETRIEVAL START: digest {digest_hex}");
    for i in 1..=retries {
        for s3_url in s3_urls.iter() {
            match retrieve_s3_ciphertext(client, s3_url, sns_material, &digest_hex).await {
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

/// Resolves the S3 bucket URLs of the given Coprocessors from the registry snapshot.
fn coprocessors_s3_urls<'a>(
    copro_registry: &'a CoprocessorRegistrySnapshot,
    coprocessor_tx_senders: &[Address],
) -> Vec<&'a str> {
    coprocessor_tx_senders
        .iter()
        .filter_map(|addr| match copro_registry.tx_sender_to_bucket.get(addr) {
            Some(url) if !url.is_empty() => Some(url.as_str()),
            _ => {
                warn!("No S3 bucket URL in the Coprocessor registry for Coprocessor {addr}");
                None
            }
        })
        .collect()
}

/// Retrieves a ciphertext from S3 using the bucket URL and ciphertext digest.
pub async fn retrieve_s3_ciphertext(
    client: &Client,
    s3_bucket_url: &str,
    sns_material: &SnsCiphertextMaterial,
    digest_hex: &str,
) -> anyhow::Result<TypedCiphertext> {
    let fhe_type = extract_fhe_type_from_handle(sns_material.ctHandle.as_slice())?;

    let old_url = format!("{s3_bucket_url}/{digest_hex}");
    let new_url = rfc023_ciphertext_url(s3_bucket_url, sns_material.ctHandle);
    let (ciphertext, ct_format) = match retrieve_ciphertext_via_http(client, &new_url).await {
        Ok((ciphertext, ct_format)) => (ciphertext, ct_format),
        Err(e) => {
            warn!("Fetching via RFC-023 URL format failed: {e}. Falling back to old URL format");
            retrieve_ciphertext_via_http(client, &old_url).await?
        }
    };

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
    client: &Client,
    url: &str,
) -> anyhow::Result<(Vec<u8>, GrpcCiphertextFormat)> {
    debug!("Attempting direct HTTP retrieval from URL: {url}");

    let response = client
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

    let ct_format = ct_format_from_http_headers(response.headers());
    let body = response
        .bytes()
        .await
        .map_err(|e| anyhow!("Failed to read HTTP response body: {}", e))?;

    Ok((body.to_vec(), ct_format))
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

fn attestation_from_http_headers(
    headers: &HeaderMap,
) -> Result<CiphertextAttestation, FetchAttestationError> {
    let Some(header) = headers.get(S3_METADATA_ATTESTATION_HEADER) else {
        return Err(FetchAttestationError::MissingHeader);
    };

    serde_json::from_slice(header.as_bytes())
        .map_err(|e| FetchAttestationError::BadHeader(e.to_string()))
}

fn ct_format_from_http_headers(headers: &HeaderMap) -> GrpcCiphertextFormat {
    match attestation_from_http_headers(headers) {
        // Get format from attestation (RFC-023)
        Ok(attestation) => match attestation.format {
            CiphertextFormat::CompressedOnCpu | CiphertextFormat::CompressedOnGpu => {
                GrpcCiphertextFormat::BigCompressed
            }
            CiphertextFormat::UncompressedOnCpu | CiphertextFormat::UncompressedOnGpu => {
                GrpcCiphertextFormat::BigExpanded
            }
        },

        // Fallback to old format header if attestation is not available
        Err(e) => {
            warn!("attestation fetch error: {e}. Falling back to {OLD_CT_FORMAT_HEADER} header");
            match headers.get(OLD_CT_FORMAT_HEADER).map(AsRef::as_ref) {
                Some(b"compressed_on_cpu") | Some(b"compressed_on_gpu") => {
                    GrpcCiphertextFormat::BigCompressed
                }
                _ => GrpcCiphertextFormat::BigExpanded,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{B256, U256},
        transports::http::reqwest::header::{HeaderName, HeaderValue},
    };
    use ciphertext_attestation::Version;

    fn attestation_headers(format: CiphertextFormat) -> HeaderMap {
        let attestation = CiphertextAttestation {
            version: Version::V1,
            key_id: U256::ZERO,
            ciphertext_digest: B256::ZERO,
            sns_ciphertext_digest: B256::ZERO,
            format,
            signer: Address::ZERO,
            signature: vec![0xab; 65],
        };
        let mut headers = HeaderMap::new();
        headers.insert(
            S3_METADATA_ATTESTATION_HEADER,
            HeaderValue::from_str(&serde_json::to_string(&attestation).unwrap()).unwrap(),
        );
        headers
    }

    fn old_format_headers(format: &'static str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            OLD_CT_FORMAT_HEADER.parse::<HeaderName>().unwrap(),
            HeaderValue::from_static(format),
        );
        headers
    }

    #[test]
    fn test_ct_format_from_rfc023_attestation_header() {
        let headers = attestation_headers(CiphertextFormat::CompressedOnCpu);
        assert_eq!(
            ct_format_from_http_headers(&headers),
            GrpcCiphertextFormat::BigCompressed
        );

        let headers = attestation_headers(CiphertextFormat::UncompressedOnCpu);
        assert_eq!(
            ct_format_from_http_headers(&headers),
            GrpcCiphertextFormat::BigExpanded
        );
    }

    #[test]
    fn test_ct_format_falls_back_to_old_header() {
        let headers = old_format_headers("compressed_on_cpu");
        assert_eq!(
            ct_format_from_http_headers(&headers),
            GrpcCiphertextFormat::BigCompressed
        );

        let headers = old_format_headers("uncompressed_on_cpu");
        assert_eq!(
            ct_format_from_http_headers(&headers),
            GrpcCiphertextFormat::BigExpanded
        );
    }

    #[test]
    fn test_ct_format_defaults_to_big_expanded_without_headers() {
        assert_eq!(
            ct_format_from_http_headers(&HeaderMap::new()),
            GrpcCiphertextFormat::BigExpanded
        );
    }

    #[test]
    fn test_ct_format_falls_back_on_malformed_attestation_header() {
        let mut headers = old_format_headers("compressed_on_gpu");
        headers.insert(
            S3_METADATA_ATTESTATION_HEADER,
            HeaderValue::from_static("not-a-json-attestation"),
        );
        assert_eq!(
            ct_format_from_http_headers(&headers),
            GrpcCiphertextFormat::BigCompressed
        );
    }

    #[test]
    fn test_attestation_from_http_headers_missing() {
        let err = attestation_from_http_headers(&HeaderMap::new()).unwrap_err();
        assert!(matches!(err, FetchAttestationError::MissingHeader));
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
}
