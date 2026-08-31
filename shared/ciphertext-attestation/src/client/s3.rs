//! Interactions with the Coprocessors' S3 buckets: attestation fetching via `HEAD` requests.
//!
//! The attestation lives in an S3 metadata header of the ciphertext object (see
//! [`rfc023_ciphertext_url`]). Retrieving the ciphertext bytes themselves is out of scope for
//! this crate: that is KMS-only behavior that stays in `kms-connector`.

use crate::{
    AttestationError, CiphertextAttestation, S3_METADATA_ATTESTATION_HEADER, s3_ct128_key,
};
use alloy::{
    primitives::{B256, U256},
    transports::http::{Client, reqwest::header::HeaderMap},
};
use std::{
    collections::HashMap,
    num::NonZeroUsize,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::Semaphore;

/// An HTTP client with a ceiling on the `HEAD` probes it may have in flight against any one
/// Coprocessor bucket at once.
///
/// The ceiling is required at construction and this crate offers no default for it: each consumer
/// fans out differently, so a value that is right for one is wrong for the other.
#[derive(Clone)]
pub struct BoundedClient {
    client: Client,
    head_semaphores: Arc<Mutex<HashMap<String, Arc<Semaphore>>>>,
    max_concurrent_heads_per_bucket: NonZeroUsize,
}

impl BoundedClient {
    pub fn new(client: Client, max_concurrent_heads_per_bucket: NonZeroUsize) -> Self {
        Self {
            client,
            head_semaphores: Arc::new(Mutex::new(HashMap::new())),
            max_concurrent_heads_per_bucket,
        }
    }

    /// Fetches the attestation for a `handle` from the specified bucket, using a `HEAD` request.
    /// Waits for a ceiling permit first if `bucket` is already at its cap.
    pub(super) async fn fetch_single_attestation(
        &self,
        bucket: &str,
        handle: B256,
        head_timeout: Duration,
        context_id: U256,
    ) -> Result<CiphertextAttestation, BucketError> {
        let semaphore = self.bucket_semaphore(bucket);
        let _permit = semaphore.acquire().await.expect("HEAD semaphore closed");

        let url = rfc023_ciphertext_url(bucket, handle, context_id);
        let response = self
            .client
            .head(&url)
            .timeout(head_timeout)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    BucketError::Timeout
                } else {
                    BucketError::Http(e.to_string())
                }
            })?;

        if !response.status().is_success() {
            return Err(BucketError::Http(format!("status {}", response.status())));
        }

        attestation_from_http_headers(response.headers())
    }

    /// The `HEAD` ceiling of `bucket`, created on first use.
    fn bucket_semaphore(&self, bucket: &str) -> Arc<Semaphore> {
        let mut semaphores = self
            .head_semaphores
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        if let Some(semaphore) = semaphores.get(bucket) {
            Arc::clone(semaphore)
        } else {
            let semaphore = Arc::new(Semaphore::new(self.max_concurrent_heads_per_bucket.get()));
            semaphores.insert(bucket.to_string(), Arc::clone(&semaphore));
            semaphore
        }
    }
}

/// URL of a ciphertext object in a Coprocessor bucket (RFC 023 layout).
pub fn rfc023_ciphertext_url(bucket_url: &str, handle: B256, context_id: U256) -> String {
    format!(
        "{bucket_url}/{}",
        s3_ct128_key(handle.as_slice(), context_id)
    )
}

/// Every reason one bucket produced no usable attestation: fetch, parse, or validation.
#[derive(Debug, thiserror::Error)]
pub(super) enum BucketError {
    #[error("HEAD request timed out")]
    Timeout,
    #[error("HEAD request failed: {0}")]
    Http(String),
    #[error("malformed attestation header: {0}")]
    BadHeader(String),
    #[error("attestation header not found")]
    MissingHeader,
    #[error("invalid attestation: {0}")]
    Invalid(#[from] AttestationError),
}

/// Parses a [`CiphertextAttestation`] out of the S3 metadata header of an HTTP response.
pub(super) fn attestation_from_http_headers(
    headers: &HeaderMap,
) -> Result<CiphertextAttestation, BucketError> {
    let Some(header) = headers.get(S3_METADATA_ATTESTATION_HEADER) else {
        return Err(BucketError::MissingHeader);
    };

    serde_json::from_slice(header.as_bytes()).map_err(|e| BucketError::BadHeader(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attestation_from_http_headers_missing() {
        let err = attestation_from_http_headers(&HeaderMap::new()).unwrap_err();
        assert!(matches!(err, BucketError::MissingHeader));
    }
}
