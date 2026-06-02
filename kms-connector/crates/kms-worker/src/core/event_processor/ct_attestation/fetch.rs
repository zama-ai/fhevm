//! S3 HEAD fan-out: fetch one attestation per Coprocessor bucket for a handle.
//!
//! Best-effort — a single bucket failing must never short-circuit the others.

use super::{COPROCESSOR_CONTEXT_ID, error::AttestationError, registry::CoprocessorRegistry};
use crate::core::event_processor::s3::compute_keccak256_digest;
use alloy::{
    hex,
    primitives::{Address, B256},
    transports::http::Client,
};
use ciphertext_attestation::{CiphertextAttestation, S3_METADATA_ATTESTATION_HEADER};
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::warn;

/// Why a single bucket's HEAD attempt did not yield an attestation.
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("HEAD request timed out")]
    Timeout,
    #[error("HEAD request failed: {0}")]
    Http(String),
    #[error("malformed attestation header: {0}")]
    BadHeader(String),
    #[error("attestation header not found")]
    MissingHeader,
}

pub type BucketResult = Result<CiphertextAttestation, FetchError>;

/// Fans out parallel S3 HEAD requests to every registered Coprocessor bucket.
///
/// Returns the attestations that were successfully fetched, keyed by the Coprocessor's `txSender`
/// address. Failed fetches are logged and dropped.
pub async fn fetch_attestations(
    handle: B256,
    registry: &CoprocessorRegistry,
    client: &Client,
    head_timeout: Duration,
) -> Vec<(Address, CiphertextAttestation)> {
    let mut tasks = JoinSet::new();
    for (tx_sender, bucket) in registry.tx_sender_to_bucket.clone() {
        let client = client.clone();
        tasks.spawn(async move {
            let result = fetch_single_attestation(&client, &bucket, handle, head_timeout).await;
            (tx_sender, result)
        });
    }

    let mut valid_attestations = vec![];
    for (tx_sender, result) in tasks.join_all().await {
        match result {
            Ok(attestation) => valid_attestations.push((tx_sender, attestation)),
            Err(e) => warn!(%tx_sender, "Failed to fetch attestation: {e}"),
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
) -> BucketResult {
    let url = format!("{bucket}/{}/{COPROCESSOR_CONTEXT_ID}", hex::encode(handle));

    let response = tokio::time::timeout(head_timeout, client.head(&url).send())
        .await
        .map_err(|_| FetchError::Timeout)?
        .map_err(|e| FetchError::Http(e.to_string()))?;

    if !response.status().is_success() {
        return Err(FetchError::Http(format!("status {}", response.status())));
    }

    let Some(header) = response.headers().get(S3_METADATA_ATTESTATION_HEADER) else {
        return Err(FetchError::MissingHeader);
    };

    serde_json::from_slice(header.as_bytes()).map_err(|e| FetchError::BadHeader(e.to_string()))
}

/// Downloads the ciphertext for `handle` from the first reachable bucket and checks its digest.
pub async fn fetch_and_check_ciphertext(
    client: &Client,
    buckets: impl Iterator<Item = &str>,
    handle: B256,
    attested_sns_digest: B256,
    timeout: Duration,
) -> Result<Vec<u8>, AttestationError> {
    let mut buckets_attempted = 0;
    for bucket in buckets {
        buckets_attempted += 1;
        match download_ciphertext(client, bucket, handle, timeout).await {
            Ok(bytes) => {
                let computed_digest = B256::from_slice(&compute_keccak256_digest(&bytes));
                if computed_digest == attested_sns_digest {
                    return Ok(bytes);
                } else {
                    warn!(
                        bucket,
                        "Ciphertext digest mismatch: attested={attested_sns_digest} computed={computed_digest}"
                    );
                }
            }
            Err(e) => warn!(bucket, "Ciphertext download failed: {e}"),
        }
    }
    Err(AttestationError::CiphertextUnavailable { buckets_attempted })
}

/// Downloads the ciphertext for `handle` from the specified bucket.
async fn download_ciphertext(
    client: &Client,
    bucket: &str,
    handle: B256,
    timeout: Duration,
) -> Result<Vec<u8>, FetchError> {
    let url = format!("{bucket}/{}/{COPROCESSOR_CONTEXT_ID}", hex::encode(handle));

    let response = tokio::time::timeout(timeout, client.get(&url).send())
        .await
        .map_err(|_| FetchError::Timeout)?
        .map_err(|e| FetchError::Http(e.to_string()))?;

    if !response.status().is_success() {
        return Err(FetchError::Http(format!("status {}", response.status())));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| FetchError::Http(e.to_string()))?;
    Ok(bytes.to_vec())
}
