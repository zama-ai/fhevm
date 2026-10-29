use std::sync::Arc;
use std::time::Duration;

use alloy_primitives::{Address, B256, U256};
use aws_sdk_s3::Client;
use block_manifest::{LEGACY_CONSENSUS_EPOCH, S3_METADATA_CONSENSUS_EPOCH_KEY};
use ciphertext_attestation::{s3_ct64_key, CiphertextAttestation, S3_METADATA_ATTESTATION_KEY};
use tokio::time::timeout;

use crate::manifest_consensus::peer_http;
use crate::manifest_consensus::verification::peer_manifest_source::s3_bucket_location;
use crate::manifest_consensus::ExecutionError;

const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_CT64_BYTES: usize = 16 * 1024 * 1024;

/// A peer's attested ct64 digest and the epoch that computed its bytes.
pub(super) struct AttestedCt64 {
    pub(super) digest: B256,
    pub(super) consensus_epoch: String,
}

impl AttestedCt64 {
    /// An object without the epoch tag comes from the `legacy` epoch.
    pub(super) fn new(digest: B256, consensus_epoch: Option<&str>) -> Self {
        Self {
            digest,
            consensus_epoch: tagged_epoch(consensus_epoch),
        }
    }
}

/// A peer's ct64 bytes and the epoch that computed them.
pub(super) struct DownloadedCt64 {
    pub(super) bytes: Vec<u8>,
    pub(super) consensus_epoch: String,
}

impl DownloadedCt64 {
    /// An object without the epoch tag comes from the `legacy` epoch.
    pub(super) fn new(bytes: Vec<u8>, consensus_epoch: Option<&str>) -> Self {
        Self {
            bytes,
            consensus_epoch: tagged_epoch(consensus_epoch),
        }
    }
}

fn tagged_epoch(consensus_epoch: Option<&str>) -> String {
    consensus_epoch.unwrap_or(LEGACY_CONSENSUS_EPOCH).to_owned()
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

#[derive(Clone)]
pub(super) struct S3Ct64Source {
    client: Arc<Client>,
}

impl S3Ct64Source {
    pub(super) fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

impl Ct64Source for S3Ct64Source {
    async fn get_ct64(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
    ) -> Result<DownloadedCt64, ExecutionError> {
        let location = s3_bucket_location(bucket_url)?;
        let key = location.object_key(&s3_ct64_key(handle, coprocessor_context_id));
        let response = timeout(
            DOWNLOAD_TIMEOUT,
            self.client
                .get_object()
                .bucket(&location.bucket)
                .key(&key)
                .send(),
        )
        .await
        .map_err(|_| {
            ExecutionError::S3TransientError(format!(
                "timed out downloading ct64 {key} from {}",
                location.bucket
            ))
        })?
        .map_err(|err| {
            if err
                .as_service_error()
                .is_some_and(|error| error.is_no_such_key())
            {
                ExecutionError::S3ObjectNotFound(format!("ct64 {key} from {}", location.bucket))
            } else {
                ExecutionError::S3TransientError(format!(
                    "failed to download ct64 {key} from {}: {err}",
                    location.bucket
                ))
            }
        })?;
        if response
            .content_length()
            .is_some_and(|length| length < 0 || length as usize > MAX_CT64_BYTES)
        {
            return Err(ExecutionError::InternalError(format!(
                "ct64 {key} from {} exceeds {MAX_CT64_BYTES} bytes",
                location.bucket
            )));
        }
        let consensus_epoch = response
            .metadata()
            .and_then(|meta| meta.get(S3_METADATA_CONSENSUS_EPOCH_KEY))
            .cloned();
        let collected = timeout(DOWNLOAD_TIMEOUT, response.body.collect())
            .await
            .map_err(|_| {
                ExecutionError::S3TransientError(format!(
                    "timed out reading ct64 {key} from {}",
                    location.bucket
                ))
            })?
            .map_err(|err| {
                ExecutionError::S3TransientError(format!(
                    "failed to read ct64 {key} from {}: {err}",
                    location.bucket
                ))
            })?;
        let body = collected.into_bytes();
        if body.len() > MAX_CT64_BYTES {
            return Err(ExecutionError::InternalError(format!(
                "ct64 {key} from {} exceeds {MAX_CT64_BYTES} bytes",
                location.bucket
            )));
        }
        Ok(DownloadedCt64::new(
            body.to_vec(),
            consensus_epoch.as_deref(),
        ))
    }

    async fn head_ct64_digest(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
        expected_signer: Option<Address>,
    ) -> Result<AttestedCt64, ExecutionError> {
        let location = s3_bucket_location(bucket_url)?;
        let key = location.object_key(&s3_ct64_key(handle, coprocessor_context_id));
        let response = timeout(
            DOWNLOAD_TIMEOUT,
            self.client
                .head_object()
                .bucket(&location.bucket)
                .key(&key)
                .send(),
        )
        .await
        .map_err(|_| {
            ExecutionError::S3TransientError(format!(
                "timed out heading ct64 {key} from {}",
                location.bucket
            ))
        })?
        .map_err(|err| {
            if err
                .as_service_error()
                .is_some_and(|error| error.is_not_found())
            {
                ExecutionError::S3ObjectNotFound(format!("ct64 {key} from {}", location.bucket))
            } else {
                ExecutionError::S3TransientError(format!(
                    "failed to head ct64 {key} from {}: {err}",
                    location.bucket
                ))
            }
        })?;
        let json = response
            .metadata()
            .and_then(|meta| {
                meta.get(S3_METADATA_ATTESTATION_KEY)
                    .or_else(|| meta.get("ct-attestation"))
            })
            .ok_or_else(|| {
                ExecutionError::DeserializationError(format!(
                    "ct64 {key} from {} has no attestation metadata",
                    location.bucket
                ))
            })?;
        let attestation: CiphertextAttestation = serde_json::from_str(json).map_err(|err| {
            ExecutionError::DeserializationError(format!(
                "ct64 attestation on {key} from {}: {err}",
                location.bucket
            ))
        })?;
        let handle: B256 = handle.try_into().map_err(|_| {
            ExecutionError::InternalError(format!("handle must be 32 bytes, got {}", handle.len()))
        })?;
        attestation
            .verify(
                handle,
                coprocessor_context_id,
                expected_signer.unwrap_or(attestation.signer),
            )
            .map_err(|err| {
                ExecutionError::DeserializationError(format!(
                    "ct64 attestation on {key} from {} failed verify: {err}",
                    location.bucket
                ))
            })?;
        let consensus_epoch = response
            .metadata()
            .and_then(|meta| meta.get(S3_METADATA_CONSENSUS_EPOCH_KEY))
            .map(String::as_str);
        Ok(AttestedCt64::new(
            attestation.ciphertext_digest,
            consensus_epoch,
        ))
    }
}

/// The object this reader names in every message, and fetches from.
fn ct64_object(bucket_url: &str, handle: &[u8], coprocessor_context_id: U256) -> String {
    peer_http::object_url(bucket_url, &s3_ct64_key(handle, coprocessor_context_id))
}

/// Reads ct64 objects over HTTP, for a peer whose registered URL names no
/// bucket.
///
/// S3 serves its user metadata as `x-amz-meta-*` response headers. The epoch
/// header is not signed, and an absent one cannot be told from one a proxy
/// removed, so this reader refuses the object rather than install it under the
/// `legacy` epoch that the S3 reader assumes.
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

    fn consensus_epoch<'a>(
        response: &'a reqwest::Response,
        object: &str,
    ) -> Result<&'a str, ExecutionError> {
        Self::metadata(response, S3_METADATA_CONSENSUS_EPOCH_KEY).ok_or_else(|| {
            ExecutionError::DeserializationError(format!("{object} has no epoch metadata"))
        })
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
        let consensus_epoch = Self::consensus_epoch(&response, &object)?.to_owned();
        let body = peer_http::read_capped(response, MAX_CT64_BYTES, &object, || {
            ExecutionError::InternalError(format!("{object} exceeds {MAX_CT64_BYTES} bytes"))
        })
        .await?;
        Ok(DownloadedCt64::new(body, Some(&consensus_epoch)))
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
                handle,
                coprocessor_context_id,
                expected_signer.unwrap_or(attestation.signer),
            )
            .map_err(|err| {
                ExecutionError::DeserializationError(format!(
                    "attestation on {object} failed verify: {err}"
                ))
            })?;
        let consensus_epoch = Self::consensus_epoch(&response, &object)?;
        Ok(AttestedCt64::new(
            attestation.ciphertext_digest,
            Some(consensus_epoch),
        ))
    }
}

/// Picks the reader each registered URL supports, as the manifest reader does.
#[derive(Clone)]
pub(super) struct Ct64Sources {
    s3: S3Ct64Source,
    http: HttpCt64Source,
}

impl Ct64Sources {
    pub(super) fn new(client: Arc<Client>, http: reqwest::Client) -> Self {
        Self {
            s3: S3Ct64Source::new(client),
            http: HttpCt64Source::new(http),
        }
    }
}

impl Ct64Source for Ct64Sources {
    async fn get_ct64(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
    ) -> Result<DownloadedCt64, ExecutionError> {
        if peer_http::reads_s3(bucket_url) {
            self.s3
                .get_ct64(bucket_url, handle, coprocessor_context_id)
                .await
        } else {
            self.http
                .get_ct64(bucket_url, handle, coprocessor_context_id)
                .await
        }
    }

    async fn head_ct64_digest(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
        expected_signer: Option<Address>,
    ) -> Result<AttestedCt64, ExecutionError> {
        if peer_http::reads_s3(bucket_url) {
            self.s3
                .head_ct64_digest(bucket_url, handle, coprocessor_context_id, expected_signer)
                .await
        } else {
            self.http
                .head_ct64_digest(bucket_url, handle, coprocessor_context_id, expected_signer)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use axum::http::{HeaderMap, StatusCode};
    use axum::Router;
    use tokio::net::TcpListener;

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
    async fn an_absent_epoch_header_refuses_the_object() {
        // A proxy that drops the metadata is not the same as an untagged
        // object, and installing under the legacy epoch cannot be undone.
        let base_url = origin(HeaderMap::new()).await;
        let source = HttpCt64Source::new(reqwest::Client::new());

        let Err(error) = source.get_ct64(&base_url, &[0u8; 32], U256::from(1)).await else {
            panic!("a stripped epoch must not read as the legacy epoch");
        };

        assert!(
            matches!(error, ExecutionError::DeserializationError(_)),
            "{error}"
        );
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
