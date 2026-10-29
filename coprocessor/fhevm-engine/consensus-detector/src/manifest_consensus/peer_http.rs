//! What the two HTTP peer readers share.
//!
//! Manifests and ct64 objects live in the same peer store and arrive over the
//! same channel, so the URL shape, the reader choice and the response rules are
//! one set of answers, held here rather than once per reader.

use crate::manifest_consensus::verification::peer_manifest_source::s3_bucket_location;
use crate::manifest_consensus::ExecutionError;

/// An object's URL under a peer's registered base.
pub(crate) fn object_url(base_url: &str, object_key: &str) -> String {
    format!("{}/{object_key}", base_url.trim_end_matches('/'))
}

/// Whether a registered URL names a bucket, and so keeps the S3 reader. A URL
/// that names only a host goes over HTTP.
pub(crate) fn reads_s3(bucket_url: &str) -> bool {
    s3_bucket_location(bucket_url).is_ok()
}

/// A response that carries the object, or the reason it does not.
pub(crate) fn check_status(
    response: reqwest::Response,
    object: &str,
) -> Result<reqwest::Response, ExecutionError> {
    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(ExecutionError::S3ObjectNotFound(object.to_owned()));
    }
    if !status.is_success() {
        return Err(ExecutionError::S3TransientError(format!(
            "{object} returned {status}"
        )));
    }
    Ok(response)
}

/// The body, refused once it passes `cap`. The length header is checked first,
/// so an oversize object costs no transfer, and the running total is checked
/// again in case that header lies.
pub(crate) async fn read_capped(
    mut response: reqwest::Response,
    cap: usize,
    object: &str,
    too_large: impl Fn() -> ExecutionError,
) -> Result<Vec<u8>, ExecutionError> {
    if response
        .content_length()
        .is_some_and(|length| length > cap as u64)
    {
        return Err(too_large());
    }
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|err| {
        ExecutionError::S3TransientError(format!("failed to read {object}: {err}"))
    })? {
        if body.len().saturating_add(chunk.len()) > cap {
            return Err(too_large());
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}
