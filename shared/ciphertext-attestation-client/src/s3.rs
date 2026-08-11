//! Interactions with the Coprocessors' S3 buckets: attestation fetching via `HEAD` requests.
//!
//! The attestation lives in an S3 metadata header of the ciphertext object (see
//! [`rfc023_ciphertext_url`]). Bucket URLs are resolved from a
//! [`crate::registry::CoprocessorRegistrySnapshot`], the single source of on-chain Coprocessor
//! metadata.
//!
//! Retrieving and verifying the ciphertext bytes themselves is out of scope for this crate: that
//! is KMS-only behavior that stays in `kms-connector`.

use alloy::{
    hex,
    primitives::{B256, U256},
    transports::http::{Client, reqwest::header::HeaderMap},
};
use ciphertext_attestation::{CiphertextAttestation, S3_METADATA_ATTESTATION_HEADER};
use std::time::Duration;

/// URL of a ciphertext object in a Coprocessor bucket (RFC 023 layout).
pub fn rfc023_ciphertext_url(bucket_url: &str, handle: B256, context_id: U256) -> String {
    format!("{bucket_url}/{}/{context_id}", hex::encode(handle))
}

/// Why a single bucket's HEAD attempt did not yield an attestation.
#[derive(Debug, thiserror::Error)]
pub enum FetchAttestationError {
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
pub async fn fetch_single_attestation(
    client: &Client,
    bucket: &str,
    handle: B256,
    head_timeout: Duration,
    context_id: U256,
) -> Result<CiphertextAttestation, FetchAttestationError> {
    let url = rfc023_ciphertext_url(bucket, handle, context_id);

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

/// Parses a [`CiphertextAttestation`] out of the S3 metadata header of an HTTP response.
pub fn attestation_from_http_headers(
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
}
