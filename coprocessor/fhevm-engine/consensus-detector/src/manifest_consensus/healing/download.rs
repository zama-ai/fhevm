use std::time::Duration;

use alloy_primitives::{Address, B256, U256};
use block_manifest::{LEGACY_CONSENSUS_EPOCH, S3_METADATA_CONSENSUS_EPOCH_KEY};
use ciphertext_attestation::{
    s3_ct64_key, Attestation, CiphertextAttestation, CiphertextRef, S3_METADATA_ATTESTATION_KEY,
};

use crate::manifest_consensus::peer_http;
use crate::manifest_consensus::ExecutionError;

const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_CT64_BYTES: usize = 16 * 1024 * 1024;

/// A peer's attested ct64 digest and the epoch that computed its bytes.
pub(super) struct AttestedCt64 {
    pub(super) digest: B256,
    pub(super) consensus_epoch: String,
}

impl AttestedCt64 {
    pub(super) fn new(digest: B256, consensus_epoch: &str) -> Self {
        Self {
            digest,
            consensus_epoch: consensus_epoch.to_owned(),
        }
    }
}

/// A peer's ct64 bytes and the epoch that computed them.
pub(super) struct DownloadedCt64 {
    pub(super) bytes: Vec<u8>,
    pub(super) consensus_epoch: String,
}

impl DownloadedCt64 {
    pub(super) fn new(bytes: Vec<u8>, consensus_epoch: &str) -> Self {
        Self {
            bytes,
            consensus_epoch: consensus_epoch.to_owned(),
        }
    }
}

pub(super) trait Ct64Source: Clone + Send + Sync + 'static {
    async fn get_ct64(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
    ) -> Result<DownloadedCt64, ExecutionError>;

    /// Attested ct64 digest and computing epoch from object metadata (HEAD).
    /// `expected_signer` is the registry signer for this bucket when known.
    async fn head_ct64_digest(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
        expected_signer: Option<Address>,
    ) -> Result<AttestedCt64, ExecutionError>;
}

/// The object this reader names in every message, and fetches from.
fn ct64_object(bucket_url: &str, handle: &[u8], coprocessor_context_id: U256) -> String {
    peer_http::object_url(bucket_url, &s3_ct64_key(handle, coprocessor_context_id))
}

/// Reads ct64 objects from a peer's registered base URL.
///
/// S3 serves its user metadata as `x-amz-meta-*` response headers. The
/// migration path writes ct64 objects with no epoch tag, so an absent header
/// reads as `legacy`. A proxy that drops metadata fails the attestation first.
#[derive(Clone)]
pub(super) struct HttpCt64Source {
    http: reqwest::Client,
}

impl HttpCt64Source {
    pub(super) fn new(http: reqwest::Client) -> Self {
        Self { http }
    }

    /// S3 user metadata, as the header that carries it over HTTP. The SDK
    /// returns these keys without the prefix, so only this reader adds it.
    fn metadata<'a>(response: &'a reqwest::Response, key: &str) -> Option<&'a str> {
        response
            .headers()
            .get(format!("x-amz-meta-{key}"))
            .and_then(|value| value.to_str().ok())
    }

    async fn send(
        &self,
        request: reqwest::RequestBuilder,
        object: &str,
    ) -> Result<reqwest::Response, ExecutionError> {
        let response = request
            .timeout(DOWNLOAD_TIMEOUT)
            .send()
            .await
            .map_err(|err| {
                ExecutionError::S3TransientError(format!("failed to request {object}: {err}"))
            })?;
        peer_http::check_status(response, object)
    }

    fn consensus_epoch(response: &reqwest::Response) -> &str {
        Self::metadata(response, S3_METADATA_CONSENSUS_EPOCH_KEY).unwrap_or(LEGACY_CONSENSUS_EPOCH)
    }
}

impl Ct64Source for HttpCt64Source {
    async fn get_ct64(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
    ) -> Result<DownloadedCt64, ExecutionError> {
        let object = ct64_object(bucket_url, handle, coprocessor_context_id);
        let response = self.send(self.http.get(&object), &object).await?;
        let consensus_epoch = Self::consensus_epoch(&response).to_owned();
        let body = peer_http::read_capped(response, MAX_CT64_BYTES, &object, || {
            ExecutionError::InternalError(format!("{object} exceeds {MAX_CT64_BYTES} bytes"))
        })
        .await?;
        Ok(DownloadedCt64::new(body, &consensus_epoch))
    }

    async fn head_ct64_digest(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
        expected_signer: Option<Address>,
    ) -> Result<AttestedCt64, ExecutionError> {
        let object = ct64_object(bucket_url, handle, coprocessor_context_id);
        let response = self.send(self.http.head(&object), &object).await?;
        let json = Self::metadata(&response, S3_METADATA_ATTESTATION_KEY).ok_or_else(|| {
            ExecutionError::DeserializationError(format!("{object} has no attestation metadata"))
        })?;
        let attestation: CiphertextAttestation = serde_json::from_str(json).map_err(|err| {
            ExecutionError::DeserializationError(format!("attestation on {object}: {err}"))
        })?;
        let handle: B256 = handle.try_into().map_err(|_| {
            ExecutionError::InternalError(format!("handle must be 32 bytes, got {}", handle.len()))
        })?;
        attestation
            .verify(
                &CiphertextRef::new(handle, coprocessor_context_id),
                expected_signer.unwrap_or(attestation.signer),
            )
            .map_err(|err| {
                ExecutionError::DeserializationError(format!(
                    "attestation on {object} failed verify: {err}"
                ))
            })?;
        let consensus_epoch = Self::consensus_epoch(&response);
        Ok(AttestedCt64::new(
            attestation.ciphertext_digest,
            consensus_epoch,
        ))
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use axum::http::{HeaderMap, StatusCode};
    use axum::Router;
    use tokio::net::TcpListener;

    use block_manifest::LEGACY_CONSENSUS_EPOCH;

    use super::{Ct64Source, HttpCt64Source};
    use crate::manifest_consensus::ExecutionError;

    const CT64_BODY: &[u8] = b"ct64-bytes";

    /// A peer origin that serves one ct64 object, with the metadata headers it
    /// is told to send.
    async fn origin(headers: HeaderMap) -> String {
        let app = Router::new().fallback(move || {
            let headers = headers.clone();
            async move { (StatusCode::OK, headers, CT64_BODY.to_vec()) }
        });
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        base_url
    }

    fn metadata(key: &str, value: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            format!("x-amz-meta-{key}")
                .parse::<axum::http::HeaderName>()
                .unwrap(),
            value.parse().unwrap(),
        );
        headers
    }

    #[tokio::test]
    async fn the_epoch_comes_from_the_metadata_header() {
        let base_url = origin(metadata("consensus-epoch", "2/block_7")).await;
        let source = HttpCt64Source::new(reqwest::Client::new());

        let ct64 = source
            .get_ct64(&base_url, &[0u8; 32], U256::from(1))
            .await
            .unwrap();

        assert_eq!(ct64.bytes, CT64_BODY);
        assert_eq!(ct64.consensus_epoch, "2/block_7");
    }

    #[tokio::test]
    async fn an_absent_epoch_header_reads_as_the_legacy_epoch() {
        // The migration path writes ct64 objects with no epoch tag. Refusing
        // them leaves every legacy finding without a vote.
        let base_url = origin(HeaderMap::new()).await;
        let source = HttpCt64Source::new(reqwest::Client::new());

        let ct64 = source
            .get_ct64(&base_url, &[0u8; 32], U256::from(1))
            .await
            .unwrap();

        assert_eq!(ct64.bytes, CT64_BODY);
        assert_eq!(ct64.consensus_epoch, LEGACY_CONSENSUS_EPOCH);
    }

    #[tokio::test]
    async fn an_absent_attestation_header_refuses_the_digest() {
        let base_url = origin(metadata("consensus-epoch", "2/block_7")).await;
        let source = HttpCt64Source::new(reqwest::Client::new());

        let Err(error) = source
            .head_ct64_digest(&base_url, &[0u8; 32], U256::from(1), None)
            .await
        else {
            panic!("a ct64 without an attestation is not a vote");
        };

        assert!(
            matches!(error, ExecutionError::DeserializationError(_)),
            "{error}"
        );
    }
}
