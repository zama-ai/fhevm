//! Mock of the Coprocessors' S3 attestation surface (RFC 023).
//!
//! The relayer's readiness check probes each Coprocessor bucket with an unauthenticated HTTP
//! `HEAD` on `{bucket}/ct128/{hex handle}/{context id}` and reads the attestation out of the
//! `x-amz-meta-ct-attestation` response header. That is plain HTTP, not JSON-RPC and not real S3
//! — no SigV4, no XML, no bucket semantics — so it needs a listener of its own rather than a
//! MinIO container.
//!
//! Each Coprocessor gets its own [`wiremock::MockServer`], hence its own port, so the bucket URLs
//! the mocked `GatewayConfig` hands back are genuinely distinct origins. Attestations are signed
//! per-request from the path, since the signature binds the handle and tests mint handles at
//! runtime.

use alloy::primitives::{Address, B256, U256};
use alloy::signers::SignerSync;
use alloy_signer_local::PrivateKeySigner;
use ciphertext_attestation::{
    CiphertextAttestation, CiphertextAttestationPayload, CiphertextFormat, Version,
    S3_METADATA_ATTESTATION_HEADER,
};
use std::time::Duration;
use tracing::debug;
use wiremock::{
    matchers::{method, path_regex},
    Mock, MockServer, Request, Respond, ResponseTemplate,
};

/// Coprocessor context the attestations are signed under. Matches the relayer's
/// `COPROCESSOR_CONTEXT_ID`; a mismatch would surface as a signature failure.
const COPROCESSOR_CONTEXT_ID: U256 = U256::ONE;

/// Coprocessor count and threshold of the default topology, mirroring the e2e stack
/// (3 Coprocessors, majority 2) so tests exercise a real quorum rather than a single signer.
const DEFAULT_COPROCESSOR_COUNT: usize = 3;
const DEFAULT_MAJORITY_THRESHOLD: usize = 2;

/// Ciphertext digest every agreeing bucket attests over. Consensus groups on this value, so
/// sharing one constant is what makes separate buckets land in the same group.
const AGREED_CIPHERTEXT_DIGEST: B256 = B256::repeat_byte(0xBB);

/// How long a bucket with nothing to serve waits before answering, in
/// [`CtAttestationMock::serve_attestations_from_first`].
///
/// A 404 is otherwise far quicker than a signed attestation, so the failures reach the tracker
/// first and it returns its verdict before any vote is counted — which makes the state under test
/// the arrival order rather than the quorum arithmetic. Ordering the misses last means a partial
/// round is decided on the votes it actually received. Three orders of magnitude above the
/// signing cost, and far below any `head_timeout` a test configures.
const MISS_DELAY: Duration = Duration::from_millis(200);

/// One Coprocessor as the mocked `GatewayConfig` reports it.
#[derive(Clone, Debug)]
pub struct MockCoprocessor {
    pub tx_sender: Address,
    pub signer: Address,
    pub s3_bucket_url: String,
}

/// Signs attestations for whatever handle the request path names.
///
/// `ciphertext_digest` is what consensus groups on: give every Coprocessor the same digest and
/// they agree, vary it and they do not.
struct AttestationResponder {
    signer: PrivateKeySigner,
    ciphertext_digest: B256,
}

impl Respond for AttestationResponder {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        let Some(handle) = handle_from_path(request.url.path()) else {
            return ResponseTemplate::new(400);
        };

        let payload = CiphertextAttestationPayload::new(
            Version::V1,
            handle,
            U256::from(1),
            COPROCESSOR_CONTEXT_ID,
            self.ciphertext_digest,
            B256::repeat_byte(0xCC),
            CiphertextFormat::UncompressedOnCpu,
        );

        // Signing has to be synchronous here: `Respond::respond` is not async, so the
        // payload's own async `sign` is unusable and the attestation is assembled by hand.
        let signature = self
            .signer
            .sign_hash_sync(&payload.canonical_digest())
            .expect("mock attestation signing failed");

        let attestation = CiphertextAttestation {
            version: Version::V1,
            key_id: U256::from(1),
            ciphertext_digest: self.ciphertext_digest,
            sns_ciphertext_digest: B256::repeat_byte(0xCC),
            format: CiphertextFormat::UncompressedOnCpu,
            signer: self.signer.address(),
            signature: signature.as_bytes().to_vec(),
        };

        let header = serde_json::to_string(&attestation).expect("attestation serialization failed");
        ResponseTemplate::new(200).insert_header(S3_METADATA_ATTESTATION_HEADER, header.as_str())
    }
}

/// Extracts the handle from an RFC 023 object path: `/ct128/{hex handle}/{context id}`.
fn handle_from_path(path: &str) -> Option<B256> {
    let handle_hex = path.trim_start_matches('/').split('/').nth(1)?;
    let bytes = hex::decode(handle_hex).ok()?;
    (bytes.len() == 32).then(|| B256::from_slice(&bytes))
}

/// Matches any RFC 023 ciphertext object path, since tests mint handles at runtime.
fn ct_object_path() -> wiremock::matchers::PathRegexMatcher {
    path_regex(r"^/ct128/[0-9a-fA-F]{64}/\d+$")
}

/// A running Coprocessor S3 attestation surface.
///
/// Holds the [`MockServer`]s, so it must stay alive for as long as the relayer under test may
/// probe the buckets: dropping it shuts the listeners down and every `HEAD` starts failing.
pub struct CtAttestationMock {
    servers: Vec<MockServer>,
    signers: Vec<PrivateKeySigner>,
    coprocessors: Vec<MockCoprocessor>,
    majority_threshold: usize,
}

impl CtAttestationMock {
    /// Starts the default topology: 3 Coprocessors, majority threshold 2, serving nothing yet.
    pub async fn start() -> Self {
        Self::start_with(DEFAULT_COPROCESSOR_COUNT, DEFAULT_MAJORITY_THRESHOLD).await
    }

    /// Starts `count` Coprocessors with an explicit majority threshold.
    pub async fn start_with(count: usize, majority_threshold: usize) -> Self {
        let mut servers = Vec::with_capacity(count);
        let mut signers = Vec::with_capacity(count);
        let mut coprocessors = Vec::with_capacity(count);

        for index in 0..count {
            let server = MockServer::start().await;
            let signer = PrivateKeySigner::random();
            coprocessors.push(MockCoprocessor {
                // Deterministic, distinct from the signer: the registry maps tx senders to
                // buckets while consensus counts signers, and conflating them hides bugs.
                tx_sender: Address::repeat_byte(0xA0 + index as u8),
                signer: signer.address(),
                s3_bucket_url: server.uri(),
            });
            debug!(index, bucket = %server.uri(), "Started Coprocessor attestation bucket");
            servers.push(server);
            signers.push(signer);
        }

        Self {
            servers,
            signers,
            coprocessors,
            majority_threshold,
        }
    }

    /// The registry as the mocked `GatewayConfig` should report it.
    pub fn coprocessors(&self) -> &[MockCoprocessor] {
        &self.coprocessors
    }

    pub fn majority_threshold(&self) -> usize {
        self.majority_threshold
    }

    /// Every bucket serves a valid attestation over the same material: consensus succeeds.
    pub async fn serve_attestations(&self) {
        self.serve_attestations_from_first(self.servers.len()).await;
    }

    /// Only the first `count` buckets have published: they serve valid attestations over the same
    /// material, the remaining buckets 404.
    ///
    /// This is what separates the majority threshold from the registry size. At or above the
    /// threshold, consensus is reached without the remaining buckets; below it the round has merely
    /// missed and is therefore retriable, because agreement that lacks numbers is not disagreement.
    ///
    /// The remaining buckets answer last, by [`MISS_DELAY`], so the votes are counted before the
    /// failures and the verdict is not a race.
    pub async fn serve_attestations_from_first(&self, count: usize) {
        self.reset().await;
        for (index, (server, signer)) in self.servers.iter().zip(&self.signers).enumerate() {
            let mock = Mock::given(method("HEAD")).and(ct_object_path());
            if index < count {
                mock.respond_with(AttestationResponder {
                    signer: signer.clone(),
                    ciphertext_digest: AGREED_CIPHERTEXT_DIGEST,
                })
                .mount(server)
                .await;
            } else {
                mock.respond_with(ResponseTemplate::new(404).set_delay(MISS_DELAY))
                    .mount(server)
                    .await;
            }
        }
    }

    /// No bucket has the object yet: the fetch never reaches the threshold, which is the
    /// expected state while Coprocessors are still uploading.
    pub async fn serve_nothing(&self) {
        self.reset().await;
        for server in &self.servers {
            Mock::given(method("HEAD"))
                .and(ct_object_path())
                .respond_with(ResponseTemplate::new(404))
                .mount(server)
                .await;
        }
    }

    /// Each bucket 404s its first `misses` probes, then serves valid attestations.
    ///
    /// The relayer re-runs the whole fan-out per retry, so `misses` is also the number of
    /// readiness attempts that fail before one succeeds.
    pub async fn serve_after_n_misses(&self, misses: u64) {
        self.reset().await;
        for (server, signer) in self.servers.iter().zip(&self.signers) {
            Mock::given(method("HEAD"))
                .and(ct_object_path())
                .respond_with(ResponseTemplate::new(404))
                .up_to_n_times(misses)
                .with_priority(1)
                .mount(server)
                .await;
            Mock::given(method("HEAD"))
                .and(ct_object_path())
                .respond_with(AttestationResponder {
                    signer: signer.clone(),
                    ciphertext_digest: AGREED_CIPHERTEXT_DIGEST,
                })
                .with_priority(2)
                .mount(server)
                .await;
        }
    }

    /// Every bucket serves a validly signed attestation over *different* material, so the
    /// threshold is never met by any one group: consensus is reached-and-failed, the terminal
    /// case that must not be retried.
    pub async fn serve_divergent_attestations(&self) {
        self.reset().await;
        for (index, (server, signer)) in self.servers.iter().zip(&self.signers).enumerate() {
            Mock::given(method("HEAD"))
                .and(ct_object_path())
                .respond_with(AttestationResponder {
                    signer: signer.clone(),
                    ciphertext_digest: B256::repeat_byte(0xB0 + index as u8),
                })
                .mount(server)
                .await;
        }
    }

    /// Every bucket stalls past the caller's `head_timeout`.
    pub async fn serve_stalled(&self, delay: Duration) {
        self.reset().await;
        for server in &self.servers {
            Mock::given(method("HEAD"))
                .and(ct_object_path())
                .respond_with(ResponseTemplate::new(200).set_delay(delay))
                .mount(server)
                .await;
        }
    }

    /// Drops previously mounted expectations so a test can re-arm mid-run.
    async fn reset(&self) {
        for server in &self.servers {
            server.reset().await;
        }
    }
}
