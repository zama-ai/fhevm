//! Interactions with the Coprocessors' S3 buckets: attestation fetching via `HEAD` requests and
//! ciphertext retrieval via `GET`.
//!
//! Both target the same object under the RFC-023 key layout: the attestation lives in an S3
//! metadata header of the object whose body is the ciphertext. Retrieval here is one bucket's raw
//! bytes; wrapping them in a KMS type and extracting the handle's FHE type stay in
//! `kms-connector`.

use crate::{
    CiphertextAttestation, S3_METADATA_ATTESTATION_HEADER, s3_ct128_key, sign::keccak_b256,
};
use alloy::{
    primitives::{B256, U256},
    transports::http::{
        Client,
        reqwest::{self, header::HeaderMap},
    },
};
use std::{
    collections::HashMap,
    num::NonZeroUsize,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::Semaphore;

/// An HTTP client with a ceiling on the requests it may have in flight.
///
/// Every bound is required at construction, with no default: each consumer fans out differently.
#[derive(Clone)]
pub struct BoundedClient {
    client: Client,
    head_semaphores: Arc<Mutex<HashMap<String, Arc<Semaphore>>>>,
    max_concurrent_heads_per_bucket: NonZeroUsize,
    /// Deadline of a single attestation `HEAD`.
    head_timeout: Duration,
    /// Coprocessor context whose object keys are probed (RFC 023).
    context_id: U256,
    /// The `GET` ceiling, global rather than per bucket: ciphertexts are big, so too many
    /// buffered at once would exhaust memory whichever buckets they came from.
    get_semaphore: Option<Arc<Semaphore>>,
    /// Ceiling on the size of a single ciphertext body, in bytes.
    max_ciphertext_size: Option<NonZeroUsize>,
}

impl BoundedClient {
    /// HEAD probes only. Cannot retrieve ciphertext bodies.
    pub fn for_attestations_only(
        client: Client,
        max_concurrent_heads_per_bucket: NonZeroUsize,
        head_timeout: Duration,
        context_id: U256,
    ) -> Self {
        Self {
            client,
            head_semaphores: Arc::new(Mutex::new(HashMap::new())),
            max_concurrent_heads_per_bucket,
            head_timeout,
            context_id,
            get_semaphore: None,
            max_ciphertext_size: None,
        }
    }

    /// HEAD probes and ciphertext GET retrieval.
    ///
    /// A `HEAD` carries `head_timeout`; a `GET` carries whatever deadline `client` was built
    /// with, so a `Client` with none lets a silent bucket hold a retrieval open indefinitely.
    pub fn for_attestations_and_ciphertexts(
        client: Client,
        max_concurrent_heads_per_bucket: NonZeroUsize,
        head_timeout: Duration,
        context_id: U256,
        max_concurrent_gets: NonZeroUsize,
        max_ciphertext_size: NonZeroUsize,
    ) -> Self {
        Self {
            get_semaphore: Some(Arc::new(Semaphore::new(max_concurrent_gets.get()))),
            max_ciphertext_size: Some(max_ciphertext_size),
            ..Self::for_attestations_only(
                client,
                max_concurrent_heads_per_bucket,
                head_timeout,
                context_id,
            )
        }
    }

    pub(super) fn context_id(&self) -> U256 {
        self.context_id
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

        let url = rfc023_ciphertext_url(bucket, handle, self.context_id);
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

    /// Retrieves the ciphertext body of `handle` from `bucket` and verifies it against the
    /// attested digest.
    pub async fn fetch_ciphertext(
        &self,
        bucket: &str,
        handle: B256,
        expected_digest: B256,
    ) -> Result<Vec<u8>, FetchCiphertextError> {
        let (Some(get_semaphore), Some(max_ciphertext_size)) =
            (&self.get_semaphore, self.max_ciphertext_size)
        else {
            return Err(FetchCiphertextError::NotConfigured);
        };
        let _permit = get_semaphore
            .acquire()
            .await
            .expect("S3 GET semaphore closed");
        let ceiling = max_ciphertext_size.get();

        let url = rfc023_ciphertext_url(bucket, handle, self.context_id);
        let mut response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| FetchCiphertextError::Http(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FetchCiphertextError::Http(format!(
                "status {}",
                response.status()
            )));
        }

        if response.content_length().unwrap_or(0) > ceiling as u64 {
            return Err(FetchCiphertextError::TooLarge { ceiling });
        }

        let mut body = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| FetchCiphertextError::Http(e.to_string()))?
        {
            // An unannounced body is only caught here, once it has actually overrun the ceiling.
            if body.len() + chunk.len() > ceiling {
                return Err(FetchCiphertextError::TooLarge { ceiling });
            }
            body.extend_from_slice(&chunk);
        }

        let actual = keccak_b256(&body);
        if actual != expected_digest {
            return Err(FetchCiphertextError::DigestMismatch {
                expected: expected_digest,
                actual,
            });
        }

        Ok(body)
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
fn rfc023_ciphertext_url(bucket_url: &str, handle: B256, context_id: U256) -> String {
    format!(
        "{bucket_url}/{}",
        s3_ct128_key(handle.as_slice(), context_id)
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

/// Why a bucket yielded no verified ciphertext body.
#[derive(Debug, thiserror::Error)]
pub enum FetchCiphertextError {
    /// The client was built by [`BoundedClient::for_attestations_only`], so it has no GET capacity.
    #[error("this client has no GET capacity")]
    NotConfigured,
    #[error("GET request failed: {0}")]
    Http(String),
    #[error("ciphertext body exceeds the ceiling of {ceiling} bytes")]
    TooLarge { ceiling: usize },
    #[error("ciphertext digest mismatch: expected {expected}, got {actual}")]
    DigestMismatch { expected: B256, actual: B256 },
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
        Arc::clone(self.get_semaphore.as_ref().expect("no GET ceiling"))
            .acquire_owned()
            .await
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{io::AsyncWriteExt, net::TcpListener, task::JoinHandle};

    /// Two addresses nothing can be listening on: a request to either fails at connection, fast.
    const UNREACHABLE_BUCKET: &str = "http://127.0.0.1:1";
    const OTHER_UNREACHABLE_BUCKET: &str = "http://127.0.0.1:2";

    const CONTEXT_ID: U256 = U256::ONE;

    /// A single-permit-per-bucket client, to make permit starvation observable.
    fn single_permit_client() -> BoundedClient {
        BoundedClient::for_attestations_only(
            Client::new(),
            NonZeroUsize::MIN,
            Duration::from_secs(5),
            CONTEXT_ID,
        )
    }

    /// The same, with a single `GET` permit and the given body ceiling.
    fn ciphertext_client(max_ciphertext_size: usize) -> BoundedClient {
        BoundedClient::for_attestations_and_ciphertexts(
            Client::new(),
            NonZeroUsize::MIN,
            Duration::from_secs(5),
            CONTEXT_ID,
            NonZeroUsize::MIN,
            NonZeroUsize::new(max_ciphertext_size).unwrap(),
        )
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

    /// A bucket that accepts connections and never answers. The accepted streams are held open so
    /// the client waits on a live connection rather than seeing it closed. Returns its URL, and
    /// its accept loop to abort.
    async fn black_hole_server() -> (String, JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let accept_loop = tokio::spawn(async move {
            let mut accepted = Vec::new();
            while let Ok((stream, _)) = listener.accept().await {
                accepted.push(stream);
            }
        });
        (url, accept_loop)
    }

    /// A `HEAD` carries its own deadline: a bucket that accepts the connection then goes silent
    /// cannot hang it.
    #[tokio::test]
    async fn a_head_is_bounded_by_its_own_deadline() {
        let (bucket, accept_loop) = black_hole_server().await;
        let client = BoundedClient::for_attestations_only(
            Client::new(),
            NonZeroUsize::MIN,
            Duration::from_millis(100),
            CONTEXT_ID,
        );

        let head = tokio::time::timeout(
            Duration::from_secs(5),
            client.fetch_single_attestation(&bucket, B256::ZERO),
        )
        .await
        .expect("the HEAD should have ended on its own deadline");
        assert!(matches!(head, Err(FetchAttestationError::Timeout)));

        accept_loop.abort();
    }

    /// The `GET` ceiling is global: one permit gates the retrievals of every handle, rather than
    /// each handle getting a ceiling of its own.
    #[tokio::test]
    async fn get_ceiling_is_shared_across_handles() {
        let client = ciphertext_client(1024);
        let permit = client.acquire_get_for_test().await;

        // The only permit is held, so no handle can start retrieving.
        for handle in [B256::ZERO, B256::repeat_byte(0x02)] {
            let gated = tokio::time::timeout(
                Duration::from_millis(200),
                client.fetch_ciphertext(UNREACHABLE_BUCKET, handle, B256::ZERO),
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
            client.fetch_ciphertext(UNREACHABLE_BUCKET, B256::ZERO, B256::ZERO),
        )
        .await
        .expect("retrieval should have resumed");
        assert!(matches!(result, Err(FetchCiphertextError::Http(_))));
    }

    /// A bucket serving `body` on any path, its length announced in a `Content-Length` header or
    /// left for the connection close to frame. Returns its URL, and its accept loop to abort.
    async fn ciphertext_bucket(body: Vec<u8>, announce_length: bool) -> (String, JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
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

    /// A body above the ceiling is turned down, announced or not. Every served body here hashes to
    /// the digest asked for, so the ceiling is the only thing that can reject one — and the body
    /// that fits proves as much.
    #[tokio::test]
    async fn oversized_ciphertext_bodies_are_rejected() {
        const CEILING: usize = 1024;

        for (body_len, announce_length, expected_ok) in [
            (CEILING, true, true),
            (4 * CEILING, true, false),
            (4 * CEILING, false, false),
        ] {
            let body = vec![0u8; body_len];
            let digest = keccak_b256(&body);
            let (bucket, accept_loop) = ciphertext_bucket(body, announce_length).await;

            let result = ciphertext_client(CEILING)
                .fetch_ciphertext(&bucket, B256::ZERO, digest)
                .await;

            match (expected_ok, &result) {
                (true, Ok(_)) | (false, Err(FetchCiphertextError::TooLarge { .. })) => {}
                _ => panic!("{body_len} bytes announced as {announce_length}: {result:?}"),
            }

            accept_loop.abort();
        }
    }

    /// A body that fits the ceiling but does not hash to the attested digest is not the ciphertext
    /// consensus vouched for.
    #[tokio::test]
    async fn a_body_failing_the_digest_check_is_rejected() {
        let (bucket, accept_loop) = ciphertext_bucket(vec![0u8; 32], true).await;

        let result = ciphertext_client(1024)
            .fetch_ciphertext(&bucket, B256::ZERO, keccak_b256(b"another ciphertext"))
            .await;
        assert!(matches!(
            result,
            Err(FetchCiphertextError::DigestMismatch { .. })
        ));

        accept_loop.abort();
    }

    /// A HEAD-only client says it has no GET capacity rather than issuing a request without one.
    #[tokio::test]
    async fn a_head_only_client_cannot_fetch_ciphertexts() {
        let result = single_permit_client()
            .fetch_ciphertext(UNREACHABLE_BUCKET, B256::ZERO, B256::ZERO)
            .await;
        assert!(matches!(result, Err(FetchCiphertextError::NotConfigured)));
    }

    #[test]
    fn test_attestation_from_http_headers_missing() {
        let err = attestation_from_http_headers(&HeaderMap::new()).unwrap_err();
        assert!(matches!(err, FetchAttestationError::MissingHeader));
    }
}
