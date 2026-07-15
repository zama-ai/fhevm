//! Interactions with the Coprocessors' S3 buckets: attestation fetching (HEAD requests) and
//! ciphertext retrieval (GET requests).
//!
//! Both target the same object key (see [`rfc023_ciphertext_url`]): the attestation lives in
//! an S3 metadata header of the object whose body is the ciphertext itself. Bucket URLs are
//! resolved from a [`CoprocessorRegistrySnapshot`], the single source of on-chain
//! Coprocessor metadata.

use super::COPROCESSOR_CONTEXT_ID;
use crate::{
    core::event_processor::ProcessingError,
    monitoring::metrics::{S3_CIPHERTEXT_RETRIEVAL_COUNTER, S3_CIPHERTEXT_RETRIEVAL_ERRORS},
};
use alloy::{
    hex,
    primitives::{B256, FixedBytes},
    transports::http::{Client, reqwest::header::HeaderMap},
};
use anyhow::anyhow;
use ciphertext_attestation::{
    CiphertextAttestation, CiphertextFormat, S3_METADATA_ATTESTATION_HEADER,
    consensus::ConsensusMaterial,
};
use connector_utils::types::handle::extract_fhe_type_from_handle;
use kms_grpc::kms::v1::{CiphertextFormat as GrpcCiphertextFormat, TypedCiphertext};
use sha3::{
    Digest, Keccak256,
    digest::{consts::U32, generic_array::GenericArray},
};
use std::time::Duration;
use tracing::{debug, warn};

/// URL of a ciphertext object in a Coprocessor bucket (RFC 023 layout).
fn rfc023_ciphertext_url(bucket_url: &str, handle: B256) -> String {
    format!(
        "{bucket_url}/{}/{COPROCESSOR_CONTEXT_ID}",
        hex::encode(handle)
    )
}

/// Why a single bucket's HEAD attempt did not yield an attestation.
#[derive(Debug, thiserror::Error)]
pub(crate) enum FetchAttestationError {
    #[error("HEAD request timed out")]
    Timeout,
    #[error("HEAD request failed: {0}")]
    Http(String),
    #[error("malformed attestation header: {0}")]
    BadHeader(String),
    #[error("attestation header not found")]
    MissingHeader,
}

/// Fetches the attestation for a `handle` from the specified bucket, using a `HEAD` request.
pub(crate) async fn fetch_single_attestation(
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

/// Retrieves the SNS ciphertext of `handle` from a bucket in the winning consensus group and
/// verifies it against the attested digest (RFC 023, authoritative mode).
pub async fn retrieve_verified_ciphertext(
    client: &Client,
    handle: B256,
    material: &ConsensusMaterial,
    winning_buckets: &[String],
    retries: u8,
) -> Result<TypedCiphertext, ProcessingError> {
    // A handle that carries no valid FHE type is malformed; retrying cannot fix it.
    let fhe_type = extract_fhe_type_from_handle(handle.as_slice()).map_err(|e| {
        ProcessingError::Irrecoverable(anyhow!("cannot extract FHE type from handle {handle}: {e}"))
    })?;
    let ct_format = grpc_ciphertext_format(material.format);

    if winning_buckets.is_empty() {
        return Err(ProcessingError::Recoverable(anyhow!(
            "no winning-group bucket resolved for handle {handle}"
        )));
    }

    let mut last_error = "no retrieval attempt made".to_string();
    let mut digest_mismatch = false;
    for attempt in 1..=retries {
        for bucket in winning_buckets {
            let url = rfc023_ciphertext_url(bucket, handle);
            let body = match retrieve_ciphertext_via_http(client, &url).await {
                Ok(body) => body,
                Err(e) => {
                    S3_CIPHERTEXT_RETRIEVAL_ERRORS.inc();
                    last_error = format!("bucket {bucket}: {e}");
                    warn!(attempt, %handle, "Failed to retrieve ciphertext: {last_error}");
                    continue;
                }
            };

            let calculated_digest = compute_keccak256_digest(&body);
            if calculated_digest.as_slice() != material.sns_ciphertext_digest.as_slice() {
                S3_CIPHERTEXT_RETRIEVAL_ERRORS.inc();
                digest_mismatch = true;
                last_error = format!(
                    "bucket {bucket}: digest mismatch (expected {}, got {})",
                    material.sns_ciphertext_digest,
                    FixedBytes::<32>::from_slice(&calculated_digest),
                );
                warn!(attempt, %handle, "Ciphertext digest mismatch: {last_error}");
                continue;
            }

            S3_CIPHERTEXT_RETRIEVAL_COUNTER.inc();
            debug!(
                %handle,
                "Ciphertext retrieved and verified: format {}, length {}, FHE type {:?}",
                ct_format.as_str_name(),
                body.len(),
                fhe_type
            );
            return Ok(TypedCiphertext {
                ciphertext: body,
                external_handle: handle.to_vec(),
                fhe_type: fhe_type as i32,
                ciphertext_format: ct_format.into(),
            });
        }
    }

    if digest_mismatch {
        warn!(%handle, "All winning-group buckets failed ciphertext digest verification");
    }
    Err(ProcessingError::Recoverable(anyhow!(
        "ciphertext unavailable for handle {handle}: all retrieval attempts failed \
         (last: {last_error})"
    )))
}

/// Retrieves a ciphertext body directly via HTTP.
async fn retrieve_ciphertext_via_http(client: &Client, url: &str) -> anyhow::Result<Vec<u8>> {
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

    let body = response
        .bytes()
        .await
        .map_err(|e| anyhow!("Failed to read HTTP response body: {}", e))?;

    Ok(body.to_vec())
}

/// Maps the attested [`CiphertextFormat`] onto the KMS gRPC format.
fn grpc_ciphertext_format(format: CiphertextFormat) -> GrpcCiphertextFormat {
    match format {
        CiphertextFormat::CompressedOnCpu | CiphertextFormat::CompressedOnGpu => {
            GrpcCiphertextFormat::BigCompressed
        }
        CiphertextFormat::UncompressedOnCpu | CiphertextFormat::UncompressedOnGpu => {
            GrpcCiphertextFormat::BigExpanded
        }
    }
}

/// Computes Keccak256 digest of a byte array.
pub fn compute_keccak256_digest(ct: &[u8]) -> GenericArray<u8, U32> {
    let mut hasher = Keccak256::new();
    hasher.update(ct);
    hasher.finalize()
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

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(digest.as_slice(), expected_bytes.as_slice());
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
