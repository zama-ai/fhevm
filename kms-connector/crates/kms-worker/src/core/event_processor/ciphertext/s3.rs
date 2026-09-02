//! Interactions with the Coprocessors' S3 buckets: attestation fetching (HEAD requests) and
//! ciphertext retrieval (GET requests).
//!
//! Both target the same object key (see [`rfc023_ciphertext_url`]): the attestation lives in
//! an S3 metadata header of the object whose body is the ciphertext itself. Bucket URLs are
//! resolved from a [`CoprocessorRegistrySnapshot`], the single source of on-chain
//! Coprocessor metadata.

use super::COPROCESSOR_CONTEXT_ID;
use crate::{
    core::{config::Config, event_processor::ProcessingError},
    monitoring::metrics::{S3_CIPHERTEXT_RETRIEVAL_COUNTER, S3_CIPHERTEXT_RETRIEVAL_ERRORS},
};
use alloy::{
    primitives::{B256, FixedBytes},
    transports::http::{
        Client,
        reqwest::{self, header::HeaderMap},
    },
};
use anyhow::anyhow;
use ciphertext_attestation::{
    CiphertextAttestation, CiphertextFormat, S3_METADATA_ATTESTATION_HEADER,
    consensus::ConsensusMaterial, s3_ct128_key,
};
use connector_utils::types::handle::extract_fhe_type_from_handle;
use kms_grpc::kms::v1::{CiphertextFormat as GrpcCiphertextFormat, TypedCiphertext};
use sha3::{
    Digest, Keccak256,
    digest::{consts::U32, generic_array::GenericArray},
};
use std::{
    collections::HashMap,
    num::NonZeroUsize,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::Semaphore;
use tracing::{debug, warn};

/// An HTTP client with a ceiling on the requests it may have in flight.
#[derive(Clone)]
pub struct BoundedClient {
    /// The inner HTTP client.
    client: Client,

    /// The per bucket HEAD request ceiling.
    head_semaphores: Arc<Mutex<HashMap<String, Arc<Semaphore>>>>,

    /// The maximum number of concurrent HEAD requests per bucket.
    max_concurrent_heads_per_bucket: NonZeroUsize,

    /// The global `GET` ceiling, as we query a single winning bucket to fetch ciphertexts.
    /// Kept low: SNS ciphertexts are big, so too many buffered at once would OOM the worker.
    get_semaphore: Arc<Semaphore>,

    /// Ceiling on the size of a single ciphertext body, in bytes.
    max_ciphertext_size: NonZeroUsize,

    /// Deadline of a single attestation `HEAD`.
    head_timeout: Duration,

    /// Number of attempts of a ciphertext retrieval, per winning-group bucket.
    retrieval_attempts: u8,
}

impl BoundedClient {
    /// Builds the S3 client with the bounds of `config`.
    pub fn from_config(config: &Config) -> anyhow::Result<Self> {
        let client = Client::builder()
            .connect_timeout(config.s3_connect_timeout)
            .timeout(config.s3_get_timeout)
            .build()
            .map_err(|e| anyhow!("Failed to create S3 HTTP client: {e}"))?;

        Ok(Self {
            client,
            head_semaphores: Arc::new(Mutex::new(HashMap::new())),
            max_concurrent_heads_per_bucket: config.s3_max_concurrent_heads_per_bucket,
            get_semaphore: Arc::new(Semaphore::new(config.s3_max_concurrent_gets.get())),
            max_ciphertext_size: config.s3_max_ciphertext_size,
            head_timeout: config.s3_head_timeout,
            retrieval_attempts: config.s3_ciphertext_retrieval_attempts,
        })
    }

    /// Fetches the attestation for a `handle` from the specified bucket, using a `HEAD` request.
    pub async fn fetch_single_attestation(
        &self,
        bucket: &str,
        handle: B256,
    ) -> Result<CiphertextAttestation, FetchAttestationError> {
        let bucket_semaphore = self.bucket_semaphore(bucket);
        let _permit = bucket_semaphore
            .acquire()
            .await
            .expect("S3 HEAD semaphore closed");

        let url = rfc023_ciphertext_url(bucket, handle);
        let response = self
            .client
            .head(&url)
            .timeout(self.head_timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FetchAttestationError::Http(format!(
                "status {}",
                response.status()
            )));
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

    /// Retrieves the SNS ciphertext of `handle` from a bucket in the winning consensus group and
    /// verifies it against the attested digest (RFC 023, authoritative mode).
    pub async fn retrieve_verified_ciphertext(
        &self,
        handle: B256,
        material: &ConsensusMaterial,
        winning_buckets: &[String],
    ) -> Result<TypedCiphertext, ProcessingError> {
        // A handle that carries no valid FHE type is malformed; retrying cannot fix it.
        let fhe_type = extract_fhe_type_from_handle(handle.as_slice()).map_err(|e| {
            ProcessingError::Irrecoverable(anyhow!(
                "cannot extract FHE type from handle {handle}: {e}"
            ))
        })?;
        let ct_format = grpc_ciphertext_format(material.format);

        if winning_buckets.is_empty() {
            return Err(ProcessingError::Recoverable(anyhow!(
                "no winning-group bucket resolved for handle {handle}"
            )));
        }

        let mut last_error = "no retrieval attempt made".to_string();
        let mut digest_mismatch = false;
        for attempt in 1..=self.retrieval_attempts {
            for bucket in winning_buckets {
                let url = rfc023_ciphertext_url(bucket, handle);

                let _permit = self
                    .get_semaphore
                    .acquire()
                    .await
                    .expect("S3 GET semaphore closed");
                let body = match self.retrieve_ciphertext_via_http(&url).await {
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

    /// Retrieves a ciphertext body directly via HTTP, holding at most `max_ciphertext_size` of it.
    async fn retrieve_ciphertext_via_http(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        debug!("Attempting direct HTTP retrieval from URL: {url}");
        let max_size = self.max_ciphertext_size.get();

        let mut response = self
            .client
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

        let expected_size = response.content_length().unwrap_or(0);
        if expected_size > max_size as u64 {
            return Err(anyhow!(
                "announced body of {expected_size} bytes exceeds the ceiling of {max_size}"
            ));
        }

        let mut body = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| anyhow!("Failed to read HTTP response body: {e}"))?
        {
            if body.len() + chunk.len() > max_size {
                return Err(anyhow!("body exceeds the ceiling of {max_size} bytes"));
            }
            body.extend_from_slice(&chunk);
        }

        Ok(body)
    }
}

/// URL of a ciphertext object in a Coprocessor bucket (RFC 023 layout).
fn rfc023_ciphertext_url(bucket_url: &str, handle: B256) -> String {
    format!(
        "{bucket_url}/{}",
        s3_ct128_key(handle.as_slice(), COPROCESSOR_CONTEXT_ID)
    )
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

impl From<reqwest::Error> for FetchAttestationError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            FetchAttestationError::Timeout
        } else {
            FetchAttestationError::Http(err.to_string())
        }
    }
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
impl BoundedClient {
    /// Takes a permit out of `bucket`'s ceiling, to check that its `HEAD`s wait for one.
    pub(super) async fn acquire_head_for_test(
        &self,
        bucket: &str,
    ) -> tokio::sync::OwnedSemaphorePermit {
        self.bucket_semaphore(bucket).acquire_owned().await.unwrap()
    }

    /// Takes a permit out of the global `GET` ceiling, to check that retrievals wait for one.
    pub(super) async fn acquire_get_for_test(&self) -> tokio::sync::OwnedSemaphorePermit {
        Arc::clone(&self.get_semaphore)
            .acquire_owned()
            .await
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U256;
    use connector_utils::tests::net::black_hole_server;
    use tokio::{io::AsyncWriteExt, task::JoinHandle};

    /// Two addresses nothing can be listening on: a request to either fails at connection, fast.
    const UNREACHABLE_BUCKET: &str = "http://127.0.0.1:1";
    const OTHER_UNREACHABLE_BUCKET: &str = "http://127.0.0.1:2";

    /// A single-permit-per-ceiling client, to make permit starvation observable.
    fn single_permit_client() -> BoundedClient {
        BoundedClient::from_config(&Config {
            s3_max_concurrent_heads_per_bucket: NonZeroUsize::MIN,
            s3_max_concurrent_gets: NonZeroUsize::MIN,
            ..Default::default()
        })
        .unwrap()
    }

    fn material() -> ConsensusMaterial {
        ConsensusMaterial {
            key_id: U256::ONE,
            ciphertext_digest: B256::ZERO,
            sns_ciphertext_digest: B256::ZERO,
            format: CiphertextFormat::CompressedOnCpu,
        }
    }

    /// `HEAD` ceilings are per bucket: a bucket with no permit left must not gate the others,
    /// otherwise one unresponsive Coprocessor would stall the attestation fan-out of every bucket.
    #[tokio::test]
    async fn head_ceilings_are_per_bucket() {
        let client = single_permit_client();
        let _saturating = client.acquire_head_for_test(UNREACHABLE_BUCKET).await;

        // The other bucket draws from its own ceiling: its `HEAD` is issued — and fails on the
        // unreachable address — instead of waiting for the saturated bucket's permit.
        let issued = tokio::time::timeout(
            Duration::from_secs(5),
            client.fetch_single_attestation(OTHER_UNREACHABLE_BUCKET, B256::ZERO),
        )
        .await
        .expect("a HEAD on another bucket should not wait for the saturated one");
        assert!(matches!(issued, Err(FetchAttestationError::Http(_))));

        // While the saturated bucket does gate its own `HEAD`s.
        let gated = tokio::time::timeout(
            Duration::from_millis(200),
            client.fetch_single_attestation(UNREACHABLE_BUCKET, B256::ZERO),
        )
        .await;
        assert!(gated.is_err(), "the saturated bucket should gate its HEADs");
    }

    /// The `GET` ceiling is global: one permit gates the retrievals of every handle, rather than
    /// each handle getting a ceiling of its own.
    #[tokio::test]
    async fn get_ceiling_is_shared_across_handles() {
        let client = single_permit_client();
        let (material, buckets) = (material(), [UNREACHABLE_BUCKET.to_string()]);

        let permit = client.acquire_get_for_test().await;

        // The only permit is held, so no handle can start retrieving.
        for handle in [B256::ZERO, B256::repeat_byte(0x02)] {
            let gated = tokio::time::timeout(
                Duration::from_millis(200),
                client.retrieve_verified_ciphertext(handle, &material, &buckets),
            )
            .await;
            assert!(
                gated.is_err(),
                "handle {handle} should wait for a GET permit"
            );
        }

        drop(permit);

        // The bucket is unreachable, so the retrieval fails: reaching that error is the proof that
        // the `GET` was issued once the permit freed.
        let result = tokio::time::timeout(
            Duration::from_secs(5),
            client.retrieve_verified_ciphertext(B256::ZERO, &material, &buckets),
        )
        .await
        .expect("retrieval should have resumed");
        assert!(matches!(result, Err(ProcessingError::Recoverable(_))));
    }

    /// Every request the client issues carries a deadline — the `HEAD` its own, the `GET` the
    /// client-wide one: a bucket that accepts the connection then goes silent can hang neither.
    #[tokio::test]
    async fn requests_are_bounded_by_their_own_deadline() {
        let (bucket, accept_loop) = black_hole_server().await;
        let client = BoundedClient::from_config(&Config {
            s3_head_timeout: Duration::from_millis(100),
            s3_get_timeout: Duration::from_millis(300),
            s3_ciphertext_retrieval_attempts: 1,
            ..Default::default()
        })
        .unwrap();

        let head = tokio::time::timeout(
            Duration::from_secs(5),
            client.fetch_single_attestation(&bucket, B256::ZERO),
        )
        .await
        .expect("the HEAD should have ended on its own deadline");
        assert!(matches!(head, Err(FetchAttestationError::Timeout)));

        // The client-wide deadline covers the whole response body, so the silent bucket trips it.
        let get = tokio::time::timeout(
            Duration::from_secs(5),
            client.retrieve_verified_ciphertext(B256::ZERO, &material(), &[bucket]),
        )
        .await
        .expect("the GET should have ended on its own deadline");
        assert!(matches!(get, Err(ProcessingError::Recoverable(_))));

        accept_loop.abort();
    }

    /// A bucket serving `body` on any path, its length announced in a `Content-Length` header or
    /// left for the connection close to frame. Returns its URL, and its accept loop to abort.
    async fn ciphertext_bucket(body: Vec<u8>, announce_length: bool) -> (String, JoinHandle<()>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let accept_loop = tokio::spawn(async move {
            while let Ok((mut stream, _)) = listener.accept().await {
                let framing = if announce_length {
                    format!("Content-Length: {}", body.len())
                } else {
                    "Connection: close".to_string()
                };
                let head = format!("HTTP/1.1 200 OK\r\n{framing}\r\n\r\n");
                let _ = stream.write_all(head.as_bytes()).await;
                let _ = stream.write_all(&body).await;
                let _ = stream.shutdown().await;
            }
        });
        (url, accept_loop)
    }

    /// A body above the ceiling is turned down, announced or not. Every served body here verifies
    /// against the attested digest, so the ceiling is the only thing that can reject one — and the
    /// body that fits proves as much.
    #[tokio::test]
    async fn oversized_ciphertext_bodies_are_rejected() {
        const CEILING: usize = 1024;

        for (body_len, announce_length, expected_ok) in [
            (CEILING, true, true),
            (4 * CEILING, true, false),
            (4 * CEILING, false, false),
        ] {
            let body = vec![0u8; body_len];
            let material = ConsensusMaterial {
                sns_ciphertext_digest: B256::from_slice(&compute_keccak256_digest(&body)),
                ..material()
            };
            let (bucket, accept_loop) = ciphertext_bucket(body, announce_length).await;

            let client = BoundedClient::from_config(&Config {
                s3_max_ciphertext_size: NonZeroUsize::new(CEILING).unwrap(),
                s3_ciphertext_retrieval_attempts: 1,
                ..Default::default()
            })
            .unwrap();
            let result = client
                .retrieve_verified_ciphertext(B256::ZERO, &material, &[bucket])
                .await;

            assert_eq!(
                result.is_ok(),
                expected_ok,
                "{body_len} bytes announced as {announce_length}: {result:?}"
            );

            accept_loop.abort();
        }
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
