//! S3 HEAD fan-out: fetch one attestation per Coprocessor bucket for a handle.
//!
//! Best-effort by design — a single bucket failing (timeout, HTTP error, bad
//! header) must never short-circuit the others. Each per-bucket result is
//! returned verbatim so the consensus stage can count valid signers and the
//! logging boundary can `warn!` on the failures without them surfacing as a
//! per-handle error.

use super::registry::CoprocessorRegistry;
use alloy::{hex, primitives::B256, transports::http::Client};
use ciphertext_attestation::{CiphertextAttestation, S3_METADATA_ATTESTATION_HEADER};
use std::time::Duration;
use tokio::task::JoinSet;

/// Why a single bucket's HEAD attempt did not yield an attestation.
///
/// These are per-bucket failures, distinct from the per-handle
/// [`super::error::AttestationError`] verdict: they are logged at `warn!` and
/// simply reduce the valid-signer count.
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    /// The HEAD request did not complete within the configured timeout.
    #[error("HEAD request timed out")]
    Timeout,
    /// The HEAD request failed to send, or returned a non-success status.
    #[error("HEAD request failed: {0}")]
    Http(String),
    /// The attestation header was present but not valid UTF-8 / JSON.
    #[error("malformed attestation header: {0}")]
    BadHeader(String),
}

/// Per-bucket outcome: `Ok(Some)` = attestation present, `Ok(None)` = header
/// absent (contributes to the silent no-attestations path), `Err` = the bucket
/// failed and is skipped.
pub type BucketResult = Result<Option<CiphertextAttestation>, FetchError>;

/// Fans out parallel S3 HEAD requests to every registered Coprocessor bucket and
/// collects one [`BucketResult`] per bucket, keyed by the Coprocessor's
/// `txSender` address. Never short-circuits: every bucket is drained.
pub async fn fetch_attestations(
    handle: B256,
    registry: &CoprocessorRegistry,
    client: &Client,
    head_timeout: Duration,
) -> Vec<(alloy::primitives::Address, BucketResult)> {
    let mut tasks = JoinSet::new();
    for (tx_sender, bucket) in registry.tx_sender_to_bucket.iter() {
        // Skip Coprocessors with no registered bucket: nothing to HEAD against.
        if bucket.is_empty() {
            continue;
        }
        let tx_sender = *tx_sender;
        let bucket = bucket.clone();
        let client = client.clone();
        tasks.spawn(async move {
            let result = fetch_one(&client, &bucket, handle, head_timeout).await;
            (tx_sender, result)
        });
    }
    tasks.join_all().await
}

/// Issues a single timed HEAD to `<bucket>/<hex(handle)>/0` and parses the
/// attestation metadata header. `coprocessor_context_id` is hardcoded to `0`
/// per RFC 023, hence the trailing `/0` path segment.
async fn fetch_one(
    client: &Client,
    bucket: &str,
    handle: B256,
    head_timeout: Duration,
) -> BucketResult {
    let url = format!("{bucket}/{}/0", hex::encode(handle));

    let response = tokio::time::timeout(head_timeout, client.head(&url).send())
        .await
        .map_err(|_| FetchError::Timeout)?
        .map_err(|e| FetchError::Http(e.to_string()))?;

    if !response.status().is_success() {
        return Err(FetchError::Http(format!("status {}", response.status())));
    }

    // `HeaderMap::get` is case-insensitive. A missing header is the no-attestation
    // path, not an error.
    let Some(raw) = response.headers().get(S3_METADATA_ATTESTATION_HEADER) else {
        return Ok(None);
    };

    let json = raw
        .to_str()
        .map_err(|e| FetchError::BadHeader(e.to_string()))?;
    let attestation =
        serde_json::from_str(json).map_err(|e| FetchError::BadHeader(e.to_string()))?;
    Ok(Some(attestation))
}
